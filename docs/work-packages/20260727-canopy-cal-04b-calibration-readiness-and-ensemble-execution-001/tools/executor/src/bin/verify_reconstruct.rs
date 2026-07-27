use cal04b_executor::{
    arg_value, read_configs, sha256, TraceHeader, CALIBRATION_DAYS_PER_LANE,
    CALIBRATION_DAYS_PER_YEAR, DAYMET_LANE_COUNT,
};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Clone)]
struct Window {
    record: String,
    year: i32,
    species: String,
    plot: String,
    start: i32,
    end: i32,
}

#[derive(Clone)]
struct Lane {
    index: usize,
    plot: String,
}

fn windows(path: &Path) -> Result<Vec<Window>, Box<dyn Error>> {
    let mut result = Vec::new();
    for (number, line) in fs::read_to_string(path)?.lines().enumerate() {
        if number == 0 || line.trim().is_empty() {
            continue;
        }
        let columns: Vec<_> = line.split(',').collect();
        if columns.len() != 18 {
            return Err(format!("bad timing row {}", number + 1).into());
        }
        result.push(Window {
            record: columns[0].into(),
            year: columns[1].parse()?,
            species: columns[2].into(),
            plot: columns[3].into(),
            start: columns[5].parse()?,
            end: columns[6].parse()?,
        });
    }
    if result.len() != 932 {
        return Err(format!("timing count {}", result.len()).into());
    }
    Ok(result)
}

fn identity(path: &Path) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut rows = HashMap::new();
    for (number, line) in fs::read_to_string(path)?.lines().enumerate() {
        if number == 0 {
            continue;
        }
        let (key, value) = line.split_once(',').ok_or("bad identity row")?;
        if rows.insert(key.into(), value.into()).is_some() {
            return Err(format!("duplicate identity {key}").into());
        }
    }
    Ok(rows)
}

fn meta<'a>(values: &'a HashMap<String, String>, key: &str) -> Result<&'a str, Box<dyn Error>> {
    values
        .get(key)
        .map(String::as_str)
        .ok_or_else(|| format!("missing trace identity {key}").into())
}

fn authenticate(
    values: &HashMap<String, String>,
    trace: &Path,
    configs: &Path,
) -> Result<(), Box<dyn Error>> {
    for (key, expected) in [
        ("schema", "CAL04B03"),
        ("site_id", "hubbard_brook"),
        ("arm_id", "deciduous"),
        ("candidate_count", "9261"),
        ("lane_count", "9"),
        ("days_per_lane", "6480"),
        ("source_days_per_plot_year", "365"),
        ("retained_days_per_plot_year", "180"),
        ("first_year", "1989"),
        ("last_year", "2024"),
        (
            "state_initialization",
            "FRESH_GSI_STATE_EACH_CANDIDATE_PLOT_YEAR",
        ),
        ("crossing_eligibility_yday", "60-180"),
        ("trace_order", "candidate_lane_year_yday"),
    ] {
        if meta(values, key)? != expected {
            return Err(format!("trace identity differs for {key}").into());
        }
    }
    if Path::new(meta(values, "trace_path")?) != trace
        || Path::new(meta(values, "config_path")?) != configs
        || fs::metadata(trace)?.len().to_string() != meta(values, "trace_bytes")?
    {
        return Err("trace path/config/byte identity differs".into());
    }
    for (path_key, digest_key) in [
        ("trace_path", "trace_sha256"),
        ("calendar_path", "calendar_sha256"),
        ("lane_manifest_path", "lane_manifest_sha256"),
        ("config_path", "config_sha256"),
        ("forcing_path", "forcing_sha256"),
        ("geometry_path", "geometry_sha256"),
        ("source_manifest_path", "source_manifest_sha256"),
        ("authority_manifest_path", "authority_manifest_sha256"),
        (
            "forcing_authority_resolution_path",
            "forcing_authority_resolution_sha256",
        ),
        ("producer_source", "producer_source_sha256"),
        ("producer_binary", "producer_binary_sha256"),
        ("failure_ledger", "failure_ledger_sha256"),
    ] {
        if sha256(Path::new(meta(values, path_key)?))? != meta(values, digest_key)? {
            return Err(format!("trace identity hash differs for {path_key}").into());
        }
    }
    if meta(values, "exact_command")?.is_empty() {
        return Err("trace command is empty".into());
    }
    Ok(())
}

fn lanes(path: &Path) -> Result<Vec<Lane>, Box<dyn Error>> {
    let mut result = Vec::new();
    for (number, line) in fs::read_to_string(path)?.lines().enumerate() {
        if number == 0 {
            continue;
        }
        let columns: Vec<_> = line.split(',').collect();
        if columns.len() != 12 {
            return Err(format!("bad lane row {}", number + 1).into());
        }
        let lane = Lane {
            index: columns[0].parse()?,
            plot: columns[1].into(),
        };
        if lane.index != result.len()
            || columns[4] != "1989"
            || columns[5] != "2024"
            || columns[6] != "365"
            || columns[7] != "13140"
            || columns[8] != "180"
            || columns[9] != "6480"
        {
            return Err(format!("lane identity differs for {}", lane.plot).into());
        }
        result.push(lane);
    }
    if result.len() != DAYMET_LANE_COUNT {
        return Err("lane count differs".into());
    }
    Ok(result)
}

fn verify_calendar(path: &Path, lanes: &[Lane]) -> Result<(), Box<dyn Error>> {
    let mut rows = BufReader::new(File::open(path)?).lines();
    if rows.next().transpose()?.as_deref() != Some("lane_index,plot_id,year,yday") {
        return Err("calendar header differs".into());
    }
    for lane in lanes {
        for year in 1989..=2024 {
            for yday in 1..=180 {
                let actual = rows.next().transpose()?.ok_or("calendar ended early")?;
                if actual != format!("{},{},{year},{yday}", lane.index, lane.plot) {
                    return Err("calendar lane/plot/year/yday order differs".into());
                }
            }
        }
    }
    if rows.next().transpose()?.is_some() {
        return Err("calendar has extra rows".into());
    }
    Ok(())
}

fn crossing(values: &[f64]) -> Option<u16> {
    values
        .windows(2)
        .enumerate()
        .find_map(|(pair_index, pair)| {
            let ending_day = pair_index as u16 + 2;
            ((60..=180).contains(&ending_day) && pair[0] < 0.5 && pair[1] >= 0.5)
                .then_some(ending_day)
        })
}

fn median(values: &mut [f64]) -> f64 {
    values.sort_by(|left, right| left.total_cmp(right));
    let half = values.len() / 2;
    if values.len().is_multiple_of(2) {
        (values[half - 1] + values[half]) / 2.0
    } else {
        values[half]
    }
}

fn objective_from_year_squares(values: &BTreeMap<i32, Vec<f64>>) -> Option<f64> {
    if values.is_empty() || values.values().any(Vec::is_empty) {
        return None;
    }
    let sum_of_year_mses = values
        .values()
        .map(|squares| squares.iter().copied().sum::<f64>() / squares.len() as f64)
        .sum::<f64>();
    Some((sum_of_year_mses / values.len() as f64).sqrt())
}

fn absent_canonical_crossings(
    lanes: &[Lane],
    crossings: &BTreeMap<(String, i32), Option<u16>>,
) -> Vec<(String, i32)> {
    let mut absent = Vec::new();
    for lane in lanes {
        for year in 1989..=2024 {
            if crossings
                .get(&(lane.plot.clone(), year))
                .is_none_or(Option::is_none)
            {
                absent.push((lane.plot.clone(), year));
            }
        }
    }
    absent
}

fn exact(left: &Path, right: &Path, label: &str) -> Result<(), Box<dyn Error>> {
    let left_hash = sha256(left)?;
    let right_hash = sha256(right)?;
    if left_hash != right_hash {
        return Err(format!("{label} differs: {left_hash} != {right_hash}").into());
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let argv: Vec<String> = env::args().collect();
    let trace_path = PathBuf::from(arg_value(&argv, "--trace")?);
    let identity_path = PathBuf::from(arg_value(&argv, "--identity")?);
    let config_path = PathBuf::from(arg_value(&argv, "--configs")?);
    let timing_path = PathBuf::from(arg_value(&argv, "--observations")?);
    let primary_components = PathBuf::from(arg_value(&argv, "--primary-components")?);
    let primary_ledgers = PathBuf::from(arg_value(&argv, "--primary-ledgers")?);
    let output = PathBuf::from(arg_value(&argv, "--out")?);
    fs::create_dir_all(&output)?;

    let identity = identity(&identity_path)?;
    authenticate(&identity, &trace_path, &config_path)?;
    let lanes = lanes(Path::new(meta(&identity, "lane_manifest_path")?))?;
    verify_calendar(Path::new(meta(&identity, "calendar_path")?), &lanes)?;
    let lane_by_plot = lanes
        .iter()
        .map(|lane| (lane.plot.clone(), lane.index))
        .collect::<HashMap<_, _>>();
    let candidates = read_configs(&config_path)?;
    let windows = windows(&timing_path)?;
    if windows
        .iter()
        .any(|window| !lane_by_plot.contains_key(&window.plot))
    {
        return Err("timing contains unknown plot".into());
    }
    let observed_groups = windows
        .iter()
        .map(|window| (window.plot.clone(), window.year))
        .collect::<BTreeSet<_>>();

    let mut trace = BufReader::with_capacity(8 * 1024 * 1024, File::open(&trace_path)?);
    let header = TraceHeader::read(&mut trace)?;
    if header
        != (TraceHeader {
            candidate_count: candidates.len(),
            lane_count: lanes.len(),
            days_per_lane: CALIBRATION_DAYS_PER_LANE,
        })
        || fs::metadata(&trace_path)?.len() != header.expected_bytes()?
    {
        return Err("trace header differs".into());
    }

    let observations_out = output.join("candidate-observation-components.csv");
    let crossings_out = output.join("candidate-crossing-components.csv");
    let years_out = output.join("candidate-annual-components.csv");
    let diagnostics_out = output.join("candidate-diagnostics.csv");
    let mut obs_writer = BufWriter::new(File::create(&observations_out)?);
    let mut crossing_writer = BufWriter::new(File::create(&crossings_out)?);
    let mut year_writer = BufWriter::new(File::create(&years_out)?);
    let mut diagnostics_writer = BufWriter::new(File::create(&diagnostics_out)?);
    writeln!(obs_writer, "candidate_id,plot_id,lane_index,record_id,year,species,crossing_doy,lower_doy,upper_doy,distance_days,squared_distance")?;
    writeln!(
        crossing_writer,
        "candidate_id,plot_id,lane_index,year,crossing_doy,eligibility_start_yday,eligibility_end_yday,state"
    )?;
    writeln!(
        year_writer,
        "candidate_id,year,observation_count,annual_mse,annual_rmse"
    )?;
    writeln!(diagnostics_writer, "candidate_id,species_rmse,observation_median_absolute_distance,year_median_absolute_distance,interval_coverage_fraction,failed_records,failed_years")?;

    let mut objectives = Vec::with_capacity(candidates.len());
    let mut failures = Vec::with_capacity(candidates.len());
    let mut raw = [0_u8; 8];
    for candidate in &candidates {
        let mut crossings = BTreeMap::new();
        for lane in &lanes {
            for year in 1989..=2024 {
                let mut daily = [0.0_f64; CALIBRATION_DAYS_PER_YEAR];
                for value in &mut daily {
                    trace.read_exact(&mut raw)?;
                    *value = f64::from_le_bytes(raw);
                    if !value.is_finite() || !(0.0..=1.0).contains(value) {
                        return Err(format!(
                            "invalid trace value for {}/{}/{year}",
                            candidate.id, lane.plot
                        )
                        .into());
                    }
                }
                let modeled_crossing = crossing(&daily);
                writeln!(
                    crossing_writer,
                    "{},{},{},{year},{},60,180,{}",
                    candidate.id,
                    lane.plot,
                    lane.index,
                    modeled_crossing.map_or_else(String::new, |day| day.to_string()),
                    if modeled_crossing.is_some() {
                        "FOUND"
                    } else {
                        "MISSING"
                    }
                )?;
                crossings.insert((lane.plot.clone(), year), modeled_crossing);
            }
        }

        let mut squares_by_year: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
        let mut squares_by_group: BTreeMap<(String, i32), Vec<f64>> = BTreeMap::new();
        let mut species_squares: BTreeMap<String, Vec<f64>> = BTreeMap::new();
        let mut all_distances = Vec::new();
        let mut coverage = 0_usize;
        let mut failed_records = 0_usize;
        for window in &windows {
            let key = (window.plot.clone(), window.year);
            let modeled = crossings[&key];
            let (distance, square, modeled_text) = modeled.map_or_else(
                || {
                    failed_records += 1;
                    (f64::INFINITY, f64::INFINITY, String::new())
                },
                |day| {
                    let day = i32::from(day);
                    let distance = (window.start - day).max(0) + (day - window.end).max(0);
                    if distance == 0 {
                        coverage += 1;
                    }
                    (
                        f64::from(distance),
                        f64::from(distance * distance),
                        day.to_string(),
                    )
                },
            );
            writeln!(
                obs_writer,
                "{},{},{},{},{},{},{},{},{},{},{}",
                candidate.id,
                window.plot,
                lane_by_plot[&window.plot],
                window.record,
                window.year,
                window.species,
                modeled_text,
                window.start,
                window.end,
                distance,
                square
            )?;
            if square.is_finite() {
                squares_by_year.entry(window.year).or_default().push(square);
                squares_by_group.entry(key).or_default().push(square);
                species_squares
                    .entry(window.species.clone())
                    .or_default()
                    .push(square);
                all_distances.push(distance);
            }
        }

        let failed_groups = absent_canonical_crossings(&lanes, &crossings);
        for (plot, year) in &observed_groups {
            if crossings[&(plot.clone(), *year)].is_some()
                && !squares_by_group.contains_key(&(plot.clone(), *year))
            {
                return Err("finite observed crossing lacks group components".into());
            }
        }
        let mut annual_mses = Vec::new();
        let mut annual_median_distances = Vec::new();
        for year in 1989..=2024 {
            let expected = windows.iter().filter(|window| window.year == year).count();
            if expected == 0 {
                continue;
            }
            if failed_groups
                .iter()
                .any(|group| group.1 == year && observed_groups.contains(group))
            {
                writeln!(
                    year_writer,
                    "{},{year},{expected},+infinity,+infinity",
                    candidate.id
                )?;
            } else {
                let squares = squares_by_year
                    .get(&year)
                    .ok_or("finite year lacks components")?;
                if squares.len() != expected {
                    return Err("annual component count differs".into());
                }
                let mse = squares.iter().copied().sum::<f64>() / squares.len() as f64;
                writeln!(
                    year_writer,
                    "{},{year},{},{mse},{}",
                    candidate.id,
                    squares.len(),
                    mse.sqrt()
                )?;
                annual_mses.push(mse);
                let mut distances = squares
                    .iter()
                    .map(|square| square.sqrt())
                    .collect::<Vec<_>>();
                annual_median_distances.push(median(&mut distances));
            }
        }
        let species = species_squares
            .iter()
            .map(|(species, values)| {
                format!(
                    "{species}:{:.9}",
                    (values.iter().copied().sum::<f64>() / values.len() as f64).sqrt()
                )
            })
            .collect::<Vec<_>>()
            .join(";");
        let failed_years = failed_groups
            .iter()
            .map(|(_, year)| *year)
            .collect::<BTreeSet<_>>();
        writeln!(
            diagnostics_writer,
            "{},{},{},{},{},{},{}",
            candidate.id,
            species,
            if all_distances.is_empty() {
                f64::INFINITY
            } else {
                median(&mut all_distances)
            },
            if annual_median_distances.is_empty() {
                f64::INFINITY
            } else {
                median(&mut annual_median_distances)
            },
            coverage as f64 / windows.len() as f64,
            failed_records,
            failed_years.len()
        )?;
        objectives.push(if failed_groups.is_empty() {
            let rebuilt = objective_from_year_squares(&squares_by_year)
                .ok_or("finite candidate lacks year squares")?;
            let emitted =
                (annual_mses.iter().copied().sum::<f64>() / annual_mses.len() as f64).sqrt();
            if rebuilt.to_bits() != emitted.to_bits() {
                return Err("independent equal-year objective differs".into());
            }
            Some(emitted)
        } else {
            None
        });
        failures.push(failed_groups);
    }
    let mut extra = [0_u8; 1];
    if trace.read(&mut extra)? != 0 {
        return Err("trace has extra bytes".into());
    }
    obs_writer.flush()?;
    crossing_writer.flush()?;
    year_writer.flush()?;
    diagnostics_writer.flush()?;

    let minimum = objectives
        .iter()
        .flatten()
        .copied()
        .min_by(f64::total_cmp)
        .ok_or("no finite objective")?;
    let threshold = minimum + 1.0;
    let candidate_out = output.join("candidate-ledger.csv");
    let accepted_out = output.join("accepted-calibration-ensemble.csv");
    let failure_out = output.join("failure-ledger.csv");
    let mut candidate_writer = BufWriter::new(File::create(&candidate_out)?);
    let mut accepted_writer = BufWriter::new(File::create(&accepted_out)?);
    let mut failure_writer = BufWriter::new(File::create(&failure_out)?);
    writeln!(
        candidate_writer,
        "candidate_id,configuration_id,state,objective,boundary_flags,saturation_flags,evidence"
    )?;
    writeln!(
        accepted_writer,
        "candidate_id,objective,acceptance_threshold,boundary_flags,saturation_flags,state"
    )?;
    writeln!(failure_writer, "failure_id,candidate_id,plot_id,lane_index,year,stage,failure_class,attempt,typed_error,evidence")?;
    let mut failure_serial = 0;
    for ((candidate, objective), failed_groups) in candidates.iter().zip(&objectives).zip(&failures)
    {
        let rendered = objective.map_or_else(|| "+infinity".into(), |value| format!("{value:.17}"));
        writeln!(
            candidate_writer,
            "{},{},{},{},{},{},{}",
            candidate.id,
            candidate.configuration_id,
            if objective.is_some() {
                "FINITE"
            } else {
                "FAILED_REQUIRED_PLOT_YEAR_CROSSING"
            },
            rendered,
            candidate.boundary,
            candidate.saturation,
            primary_components
                .join("candidate-observation-components.csv")
                .display()
        )?;
        if objective.is_some_and(|value| value <= threshold) {
            writeln!(
                accepted_writer,
                "{},{:.17},{threshold:.17},{},{},ACCEPTED_FROZEN",
                candidate.id,
                objective.unwrap_or_default(),
                candidate.boundary,
                candidate.saturation
            )?;
        }
        for (plot, year) in failed_groups {
            failure_serial += 1;
            writeln!(
                failure_writer,
                "FAIL-{failure_serial:06},{},{plot},{},{year},gsi_timing,MISSING_REQUIRED_PLOT_YEAR_CROSSING,1,objective_positive_infinity,eligibility_yday_60_180",
                candidate.id, lane_by_plot[plot]
            )?;
        }
    }
    candidate_writer.flush()?;
    accepted_writer.flush()?;
    failure_writer.flush()?;

    for (verification, primary_path, label) in [
        (
            &crossings_out,
            primary_components.join("candidate-crossing-components.csv"),
            "crossing components",
        ),
        (
            &observations_out,
            primary_components.join("candidate-observation-components.csv"),
            "observation components",
        ),
        (
            &years_out,
            primary_components.join("candidate-annual-components.csv"),
            "annual components",
        ),
        (
            &diagnostics_out,
            primary_components.join("candidate-diagnostics.csv"),
            "candidate diagnostics",
        ),
        (
            &candidate_out,
            primary_ledgers.join("candidate-ledger.csv"),
            "candidate ledger",
        ),
        (
            &accepted_out,
            primary_ledgers.join("accepted-calibration-ensemble.csv"),
            "accepted ensemble",
        ),
        (
            &failure_out,
            primary_ledgers.join("failure-ledger.csv"),
            "failure ledger",
        ),
    ] {
        exact(verification, &primary_path, label)?;
    }
    let executable = env::current_exe()?;
    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/bin/verify_reconstruct.rs");
    fs::write(
        output.join("verification-receipt.csv"),
        format!(
            "field,value\nstate,PASS\nexact_command,{}\nsource_path,{}\nsource_sha256,{}\nbinary_path,{}\nbinary_sha256,{}\ntrace_sha256,{}\nidentity_sha256,{}\nconfig_sha256,{}\nobservation_sha256,{}\nforcing_authority_resolution_sha256,{}\nobjective_grouping,equal_year_mean_of_all_admitted_record_squared_distances\ncrossing_eligibility_yday,60-180\nstate_initialization,FRESH_GSI_STATE_EACH_CANDIDATE_PLOT_YEAR\ncrossing_components_sha256,{}\nobservation_components_sha256,{}\nannual_components_sha256,{}\ndiagnostics_sha256,{}\ncandidate_ledger_sha256,{}\naccepted_ensemble_sha256,{}\nfailure_ledger_sha256,{}\n",
            argv.join(" "),
            source.display(),
            sha256(&source)?,
            executable.display(),
            sha256(&executable)?,
            sha256(&trace_path)?,
            sha256(&identity_path)?,
            sha256(&config_path)?,
            sha256(&timing_path)?,
            meta(&identity, "forcing_authority_resolution_sha256")?,
            sha256(&crossings_out)?,
            sha256(&observations_out)?,
            sha256(&years_out)?,
            sha256(&diagnostics_out)?,
            sha256(&candidate_out)?,
            sha256(&accepted_out)?,
            sha256(&failure_out)?
        ),
    )?;
    println!(
        "PASS independent reconstruction candidates={} plots={}",
        candidates.len(),
        lanes.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{absent_canonical_crossings, crossing, objective_from_year_squares, Lane};
    use std::collections::BTreeMap;

    #[test]
    fn independent_crossing_enforces_warmup_and_upper_boundary() {
        let mut values = [0.0; 180];
        values[58] = 1.0;
        assert_eq!(crossing(&values), None);
        values.fill(0.0);
        values[59] = 1.0;
        assert_eq!(crossing(&values), Some(60));
        values.fill(0.0);
        values[179] = 1.0;
        assert_eq!(crossing(&values), Some(180));
    }

    #[test]
    fn independent_crossing_has_no_external_prior_state() {
        let prior_year = [1.0; 180];
        let mut new_year = [1.0; 180];
        new_year[58] = 0.0;
        new_year[59] = 1.0;
        assert_eq!(crossing(&prior_year), None);
        assert_eq!(crossing(&new_year), Some(60));
    }

    #[test]
    fn independent_objective_weights_complete_years_equally() {
        let squares = BTreeMap::from([(2001, vec![0.0]), (2002, vec![4.0, 4.0, 4.0])]);
        assert_eq!(objective_from_year_squares(&squares), Some(2.0_f64.sqrt()));
    }

    #[test]
    fn independent_inventory_rejects_unobserved_missing_plot_year() {
        let lanes = vec![Lane {
            index: 0,
            plot: "7T".to_string(),
        }];
        let crossings = (1989..=2024)
            .map(|year| {
                (
                    ("7T".to_string(), year),
                    if year == 1989 { None } else { Some(101) },
                )
            })
            .collect();
        assert_eq!(
            absent_canonical_crossings(&lanes, &crossings),
            vec![("7T".to_string(), 1989)]
        );
    }
}
