use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use openwepp_meteorology::phase::{PhaseTimescale, harder_pomeroy_phase_from_relative_humidity};
use openwepp_unit_boundary::{FractionUnitInterval, TemperatureCelsius};
use serde::Serialize;

use super::SnowbenchError;

#[derive(Debug, Clone)]
pub struct JenningsPhaseValidationRequest {
    pub observations_path: PathBuf,
    pub thresholds_path: PathBuf,
    pub output_dir: PathBuf,
    pub max_rows: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct JenningsPhaseValidationReport {
    pub observations_path: String,
    pub thresholds_path: String,
    pub output_dir: String,
    pub rows_read: usize,
    pub rows_scored: usize,
    pub rows_skipped: usize,
    pub stations_scored: usize,
    pub rh_normalized_to_saturation: usize,
    pub harder_pomeroy_hourly: JenningsMethodScore,
    pub legacy_rst_0c: JenningsMethodScore,
    pub threshold_summary: JenningsThresholdSummary,
    pub humidity_threshold_contrast: JenningsHumidityContrast,
    pub report_json_path: String,
    pub report_markdown_path: String,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct JenningsMethodScore {
    pub rows_scored: usize,
    pub accuracy: f64,
    pub observed_rain_predicted_rain: usize,
    pub observed_rain_predicted_snow: usize,
    pub observed_snow_predicted_rain: usize,
    pub observed_snow_predicted_snow: usize,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct JenningsThresholdSummary {
    pub station_count: usize,
    pub mean_predicted_temp50_c: f64,
    pub mean_observed_temp50_c: f64,
    pub mean_bias_c: f64,
    pub mean_absolute_error_c: f64,
    pub max_absolute_error_c: f64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct JenningsHumidityContrast {
    pub low_rh_station_count: usize,
    pub high_rh_station_count: usize,
    pub low_rh_mean_observed_temp50_c: f64,
    pub high_rh_mean_observed_temp50_c: f64,
    pub low_rh_mean_predicted_temp50_c: f64,
    pub high_rh_mean_predicted_temp50_c: f64,
    pub observed_high_minus_low_c: f64,
    pub predicted_high_minus_low_c: f64,
}

#[derive(Debug, Clone)]
struct StationStats {
    observed_temp50_c: f64,
    best_delta_from_half: f64,
    predicted_temp50_c: Option<f64>,
    rh_sum: f64,
    rh_count: usize,
}

#[derive(Debug, Clone, Copy)]
struct MethodAccumulator {
    rows_scored: usize,
    observed_rain_predicted_rain: usize,
    observed_rain_predicted_snow: usize,
    observed_snow_predicted_rain: usize,
    observed_snow_predicted_snow: usize,
}

impl MethodAccumulator {
    const fn new() -> Self {
        Self {
            rows_scored: 0,
            observed_rain_predicted_rain: 0,
            observed_rain_predicted_snow: 0,
            observed_snow_predicted_rain: 0,
            observed_snow_predicted_snow: 0,
        }
    }

    fn score(&mut self, observed_rain: bool, predicted_rain: bool) {
        self.rows_scored += 1;
        match (observed_rain, predicted_rain) {
            (true, true) => self.observed_rain_predicted_rain += 1,
            (true, false) => self.observed_rain_predicted_snow += 1,
            (false, true) => self.observed_snow_predicted_rain += 1,
            (false, false) => self.observed_snow_predicted_snow += 1,
        }
    }

    fn finish(self) -> JenningsMethodScore {
        let correct = self.observed_rain_predicted_rain + self.observed_snow_predicted_snow;
        let accuracy = if self.rows_scored == 0 {
            0.0
        } else {
            count_to_f64(correct) / count_to_f64(self.rows_scored)
        };
        JenningsMethodScore {
            rows_scored: self.rows_scored,
            accuracy,
            observed_rain_predicted_rain: self.observed_rain_predicted_rain,
            observed_rain_predicted_snow: self.observed_rain_predicted_snow,
            observed_snow_predicted_rain: self.observed_snow_predicted_rain,
            observed_snow_predicted_snow: self.observed_snow_predicted_snow,
        }
    }
}

#[allow(clippy::too_many_lines)]
pub fn run_jennings_phase_validation(
    request: &JenningsPhaseValidationRequest,
) -> Result<JenningsPhaseValidationReport, SnowbenchError> {
    fs::create_dir_all(&request.output_dir).map_err(|source| SnowbenchError::Io {
        path: request.output_dir.clone(),
        source,
    })?;
    let mut stations = read_thresholds(&request.thresholds_path)?;
    let observation_file =
        fs::File::open(&request.observations_path).map_err(|source| SnowbenchError::Io {
            path: request.observations_path.clone(),
            source,
        })?;
    let reader = BufReader::new(observation_file);

    let mut rows_read = 0usize;
    let mut rows_scored = 0usize;
    let mut rows_skipped = 0usize;
    let mut rh_normalized_to_saturation = 0usize;
    let mut hp = MethodAccumulator::new();
    let mut legacy = MethodAccumulator::new();

    for (line_index, line) in reader.lines().enumerate() {
        let line = line.map_err(|source| SnowbenchError::Io {
            path: request.observations_path.clone(),
            source,
        })?;
        if line_index == 0 {
            validate_observation_header(&line)?;
            continue;
        }
        if let Some(max_rows) = request.max_rows
            && rows_read >= max_rows
        {
            break;
        }
        rows_read += 1;

        let Some(row) = parse_observation_row(&line) else {
            rows_skipped += 1;
            continue;
        };
        let Some(station) = stations.get_mut(row.station_id) else {
            rows_skipped += 1;
            continue;
        };
        let Some(observed_rain) = observed_phase(row.rain_phase, row.snow_phase) else {
            rows_skipped += 1;
            continue;
        };
        let Some((relative_humidity, normalized)) = unit_relative_humidity(row.rh_percent) else {
            rows_skipped += 1;
            continue;
        };
        if normalized {
            rh_normalized_to_saturation += 1;
        }
        let Ok(air_temperature) = TemperatureCelsius::try_new(row.air_temp_c) else {
            rows_skipped += 1;
            continue;
        };
        let Ok(estimate) = harder_pomeroy_phase_from_relative_humidity(
            air_temperature,
            relative_humidity,
            PhaseTimescale::Hourly,
        ) else {
            rows_skipped += 1;
            continue;
        };

        let rain_fraction = estimate.fractions.rain_fraction.as_fraction();
        let predicted_rain_hp = rain_fraction >= 0.5;
        let predicted_rain_legacy = row.air_temp_c > 0.0;
        hp.score(observed_rain, predicted_rain_hp);
        legacy.score(observed_rain, predicted_rain_legacy);
        rows_scored += 1;

        station.rh_sum += relative_humidity.as_fraction();
        station.rh_count += 1;
        let delta_from_half = (rain_fraction - 0.5).abs();
        if delta_from_half < station.best_delta_from_half {
            station.best_delta_from_half = delta_from_half;
            station.predicted_temp50_c = Some(row.air_temp_c);
        }
    }

    let station_summaries = station_summaries(&stations);
    let threshold_summary = summarize_thresholds(&station_summaries);
    let humidity_threshold_contrast = summarize_humidity_contrast(&station_summaries);
    let report_json_path = request.output_dir.join("jennings-validation-report.json");
    let report_markdown_path = request.output_dir.join("jennings-validation-report.md");
    let report = JenningsPhaseValidationReport {
        observations_path: request.observations_path.display().to_string(),
        thresholds_path: request.thresholds_path.display().to_string(),
        output_dir: request.output_dir.display().to_string(),
        rows_read,
        rows_scored,
        rows_skipped,
        stations_scored: station_summaries.len(),
        rh_normalized_to_saturation,
        harder_pomeroy_hourly: hp.finish(),
        legacy_rst_0c: legacy.finish(),
        threshold_summary,
        humidity_threshold_contrast,
        report_json_path: report_json_path.display().to_string(),
        report_markdown_path: report_markdown_path.display().to_string(),
    };
    write_json(&report_json_path, &report)?;
    write_markdown(&report_markdown_path, &report)?;
    Ok(report)
}

fn read_thresholds(path: &Path) -> Result<HashMap<String, StationStats>, SnowbenchError> {
    let file = fs::File::open(path).map_err(|source| SnowbenchError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let mut rows = HashMap::new();
    for (line_index, line) in BufReader::new(file).lines().enumerate() {
        let line = line.map_err(|source| SnowbenchError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        if line_index == 0 {
            if line.trim() != "Station_ID,temp50" {
                return Err(SnowbenchError::InvalidInput {
                    detail: format!("unexpected Jennings file3 header: {line}"),
                });
            }
            continue;
        }
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() != 2 {
            continue;
        }
        let Ok(observed_temp50_c) = fields[1].parse::<f64>() else {
            continue;
        };
        rows.insert(
            fields[0].to_string(),
            StationStats {
                observed_temp50_c,
                best_delta_from_half: f64::INFINITY,
                predicted_temp50_c: None,
                rh_sum: 0.0,
                rh_count: 0,
            },
        );
    }
    Ok(rows)
}

fn validate_observation_header(line: &str) -> Result<(), SnowbenchError> {
    let expected = "Station_ID,Date,Hour,Air_Temp,Dewpoint,RH,gridded_data_pres,Prec_Type,Snow_Phase,Rain_Phase";
    if line.trim() == expected {
        Ok(())
    } else {
        Err(SnowbenchError::InvalidInput {
            detail: format!("unexpected Jennings file2 header: {line}"),
        })
    }
}

struct ObservationRow<'a> {
    station_id: &'a str,
    air_temp_c: f64,
    rh_percent: f64,
    snow_phase: i32,
    rain_phase: i32,
}

fn parse_observation_row(line: &str) -> Option<ObservationRow<'_>> {
    let fields: Vec<&str> = line.split(',').collect();
    if fields.len() != 10 {
        return None;
    }
    Some(ObservationRow {
        station_id: fields[0],
        air_temp_c: fields[3].parse().ok()?,
        rh_percent: fields[5].parse().ok()?,
        snow_phase: fields[8].parse().ok()?,
        rain_phase: fields[9].parse().ok()?,
    })
}

fn observed_phase(rain_phase: i32, snow_phase: i32) -> Option<bool> {
    match (rain_phase, snow_phase) {
        (1, 0) => Some(true),
        (0, 1) => Some(false),
        _ => None,
    }
}

fn unit_relative_humidity(rh_percent: f64) -> Option<(FractionUnitInterval, bool)> {
    if !rh_percent.is_finite() || rh_percent < 0.0 {
        return None;
    }
    let ratio = rh_percent / 100.0;
    let normalized = ratio > 1.0;
    let bounded = ratio.min(1.0);
    FractionUnitInterval::try_new(bounded)
        .ok()
        .map(|relative_humidity| (relative_humidity, normalized))
}

#[derive(Debug, Clone, Copy)]
struct StationSummary {
    observed_temp50_c: f64,
    predicted_temp50_c: f64,
    mean_relative_humidity: f64,
}

fn station_summaries(stations: &HashMap<String, StationStats>) -> Vec<StationSummary> {
    stations
        .values()
        .filter_map(|station| {
            let predicted_temp50_c = station.predicted_temp50_c?;
            if station.rh_count == 0 {
                return None;
            }
            Some(StationSummary {
                observed_temp50_c: station.observed_temp50_c,
                predicted_temp50_c,
                mean_relative_humidity: station.rh_sum / count_to_f64(station.rh_count),
            })
        })
        .collect()
}

fn summarize_thresholds(stations: &[StationSummary]) -> JenningsThresholdSummary {
    if stations.is_empty() {
        return JenningsThresholdSummary {
            station_count: 0,
            mean_predicted_temp50_c: 0.0,
            mean_observed_temp50_c: 0.0,
            mean_bias_c: 0.0,
            mean_absolute_error_c: 0.0,
            max_absolute_error_c: 0.0,
        };
    }
    let mut predicted_sum = 0.0;
    let mut observed_sum = 0.0;
    let mut bias_sum = 0.0;
    let mut abs_sum = 0.0;
    let mut max_abs = 0.0;
    for station in stations {
        let bias = station.predicted_temp50_c - station.observed_temp50_c;
        predicted_sum += station.predicted_temp50_c;
        observed_sum += station.observed_temp50_c;
        bias_sum += bias;
        abs_sum += bias.abs();
        if bias.abs() > max_abs {
            max_abs = bias.abs();
        }
    }
    let count = count_to_f64(stations.len());
    JenningsThresholdSummary {
        station_count: stations.len(),
        mean_predicted_temp50_c: predicted_sum / count,
        mean_observed_temp50_c: observed_sum / count,
        mean_bias_c: bias_sum / count,
        mean_absolute_error_c: abs_sum / count,
        max_absolute_error_c: max_abs,
    }
}

fn summarize_humidity_contrast(stations: &[StationSummary]) -> JenningsHumidityContrast {
    if stations.is_empty() {
        return JenningsHumidityContrast {
            low_rh_station_count: 0,
            high_rh_station_count: 0,
            low_rh_mean_observed_temp50_c: 0.0,
            high_rh_mean_observed_temp50_c: 0.0,
            low_rh_mean_predicted_temp50_c: 0.0,
            high_rh_mean_predicted_temp50_c: 0.0,
            observed_high_minus_low_c: 0.0,
            predicted_high_minus_low_c: 0.0,
        };
    }
    let mut sorted = stations.to_vec();
    sorted.sort_by(|a, b| {
        a.mean_relative_humidity
            .partial_cmp(&b.mean_relative_humidity)
            .unwrap_or(Ordering::Equal)
    });
    let decile_count = (sorted.len() / 10).max(1);
    let low = &sorted[..decile_count];
    let high = &sorted[sorted.len() - decile_count..];
    let low_observed = mean_by(low, |station| station.observed_temp50_c);
    let high_observed = mean_by(high, |station| station.observed_temp50_c);
    let low_predicted = mean_by(low, |station| station.predicted_temp50_c);
    let high_predicted = mean_by(high, |station| station.predicted_temp50_c);
    JenningsHumidityContrast {
        low_rh_station_count: low.len(),
        high_rh_station_count: high.len(),
        low_rh_mean_observed_temp50_c: low_observed,
        high_rh_mean_observed_temp50_c: high_observed,
        low_rh_mean_predicted_temp50_c: low_predicted,
        high_rh_mean_predicted_temp50_c: high_predicted,
        observed_high_minus_low_c: high_observed - low_observed,
        predicted_high_minus_low_c: high_predicted - low_predicted,
    }
}

fn mean_by(stations: &[StationSummary], field: fn(&StationSummary) -> f64) -> f64 {
    stations.iter().map(field).sum::<f64>() / count_to_f64(stations.len())
}

fn count_to_f64(value: usize) -> f64 {
    f64::from(u32::try_from(value).unwrap_or(u32::MAX))
}

fn write_json(path: &Path, report: &JenningsPhaseValidationReport) -> Result<(), SnowbenchError> {
    let text = serde_json::to_string_pretty(report).map_err(|source| SnowbenchError::Json {
        path: path.to_path_buf(),
        source,
    })?;
    write_text(path, &text)
}

#[allow(clippy::format_push_string)]
fn write_markdown(
    path: &Path,
    report: &JenningsPhaseValidationReport,
) -> Result<(), SnowbenchError> {
    let mut text = String::new();
    text.push_str("# Jennings Phase Validation Report\n\n");
    text.push_str("Status: complete\n");
    text.push_str("Evidence mode: Ran\n\n");
    text.push_str("## Inputs\n\n");
    text.push_str(&format!("- Observations: `{}`\n", report.observations_path));
    text.push_str(&format!("- Thresholds: `{}`\n", report.thresholds_path));
    text.push_str(&format!("- Rows read: `{}`\n", report.rows_read));
    text.push_str(&format!("- Rows scored: `{}`\n", report.rows_scored));
    text.push_str(&format!("- Rows skipped: `{}`\n", report.rows_skipped));
    text.push_str(&format!(
        "- Stations scored: `{}`\n",
        report.stations_scored
    ));
    text.push_str(&format!(
        "- RH values normalized to saturation: `{}`\n\n",
        report.rh_normalized_to_saturation
    ));
    text.push_str("## Scores\n\n");
    text.push_str("| Model | Accuracy | RR | RS | SR | SS |\n");
    text.push_str("|---|---:|---:|---:|---:|---:|\n");
    append_score_row(
        &mut text,
        "harder_pomeroy_hourly",
        report.harder_pomeroy_hourly,
    );
    append_score_row(&mut text, "legacy_rst_0c", report.legacy_rst_0c);
    text.push_str("\n## Threshold Summary\n\n");
    text.push_str(&format!(
        "- Station count: `{}`\n- Mean predicted temp50 C: `{:.6}`\n- Mean observed temp50 C: `{:.6}`\n- Mean bias C: `{:.6}`\n- Mean absolute error C: `{:.6}`\n- Max absolute error C: `{:.6}`\n\n",
        report.threshold_summary.station_count,
        report.threshold_summary.mean_predicted_temp50_c,
        report.threshold_summary.mean_observed_temp50_c,
        report.threshold_summary.mean_bias_c,
        report.threshold_summary.mean_absolute_error_c,
        report.threshold_summary.max_absolute_error_c,
    ));
    text.push_str("## Humidity Contrast\n\n");
    text.push_str(&format!(
        "- Low-RH station count: `{}`\n- High-RH station count: `{}`\n- Observed high-minus-low temp50 C: `{:.6}`\n- Predicted high-minus-low temp50 C: `{:.6}`\n",
        report.humidity_threshold_contrast.low_rh_station_count,
        report.humidity_threshold_contrast.high_rh_station_count,
        report.humidity_threshold_contrast.observed_high_minus_low_c,
        report.humidity_threshold_contrast.predicted_high_minus_low_c,
    ));
    write_text(path, &text)
}

#[allow(clippy::format_push_string)]
fn append_score_row(text: &mut String, model: &str, score: JenningsMethodScore) {
    text.push_str(&format!(
        "| `{}` | {:.6} | {} | {} | {} | {} |\n",
        model,
        score.accuracy,
        score.observed_rain_predicted_rain,
        score.observed_rain_predicted_snow,
        score.observed_snow_predicted_rain,
        score.observed_snow_predicted_snow
    ));
}

fn write_text(path: &Path, text: &str) -> Result<(), SnowbenchError> {
    let mut file = fs::File::create(path).map_err(|source| SnowbenchError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    file.write_all(text.as_bytes())
        .map_err(|source| SnowbenchError::Io {
            path: path.to_path_buf(),
            source,
        })
}
