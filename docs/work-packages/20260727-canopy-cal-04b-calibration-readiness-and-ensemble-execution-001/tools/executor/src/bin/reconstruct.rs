use cal04b_executor::{
    arg_value, read_configs, sha256, TraceHeader, CALIBRATION_DAYS_PER_LANE,
    CALIBRATION_DAYS_PER_YEAR, DAYMET_LANE_COUNT,
};
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Clone)]
struct Observation {
    id: String,
    year: i32,
    species: String,
    plot_id: String,
    lower: i32,
    upper: i32,
}

#[derive(Clone)]
struct Lane {
    index: usize,
    plot_id: String,
}

fn read_observations(path: &Path) -> Result<Vec<Observation>, Box<dyn Error>> {
    let mut rows = Vec::new();
    for (index, line) in fs::read_to_string(path)?.lines().enumerate() {
        if index == 0 || line.trim().is_empty() {
            continue;
        }
        let fields: Vec<_> = line.split(',').collect();
        if fields.len() != 18 {
            return Err(format!("observation line {} malformed", index + 1).into());
        }
        rows.push(Observation {
            id: fields[0].to_string(),
            year: fields[1].parse()?,
            species: fields[2].to_string(),
            plot_id: fields[3].to_string(),
            lower: fields[5].parse()?,
            upper: fields[6].parse()?,
        });
    }
    if rows.len() != 932 {
        return Err(format!("expected 932 observations, observed {}", rows.len()).into());
    }
    Ok(rows)
}

fn read_identity(path: &Path) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut values = HashMap::new();
    for (index, line) in fs::read_to_string(path)?.lines().enumerate() {
        if index == 0 {
            continue;
        }
        let (key, value) = line.split_once(',').ok_or("identity row malformed")?;
        if values.insert(key.to_string(), value.to_string()).is_some() {
            return Err(format!("duplicate identity field {key}").into());
        }
    }
    Ok(values)
}

fn field<'a>(identity: &'a HashMap<String, String>, key: &str) -> Result<&'a str, Box<dyn Error>> {
    identity
        .get(key)
        .map(String::as_str)
        .ok_or_else(|| format!("identity missing {key}").into())
}

fn authenticate(
    identity: &HashMap<String, String>,
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
        if field(identity, key)? != expected {
            return Err(format!("identity {key} differs").into());
        }
    }
    if Path::new(field(identity, "trace_path")?) != trace
        || Path::new(field(identity, "config_path")?) != configs
        || fs::metadata(trace)?.len().to_string() != field(identity, "trace_bytes")?
    {
        return Err("trace/config/path identity mismatch".into());
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
        if sha256(Path::new(field(identity, path_key)?))? != field(identity, digest_key)? {
            return Err(format!("identity digest differs for {path_key}").into());
        }
    }
    if field(identity, "exact_command")?.is_empty() {
        return Err("producer command identity is empty".into());
    }
    Ok(())
}

fn read_lanes(path: &Path) -> Result<Vec<Lane>, Box<dyn Error>> {
    let mut lanes = Vec::new();
    for (index, line) in fs::read_to_string(path)?.lines().enumerate() {
        if index == 0 {
            continue;
        }
        let fields: Vec<_> = line.split(',').collect();
        if fields.len() != 12 {
            return Err(format!("lane manifest row {} malformed", index + 1).into());
        }
        let lane = Lane {
            index: fields[0].parse()?,
            plot_id: fields[1].to_string(),
        };
        if lane.index != lanes.len()
            || fields[4] != "1989"
            || fields[5] != "2024"
            || fields[6] != "365"
            || fields[7] != "13140"
            || fields[8] != "180"
            || fields[9] != "6480"
        {
            return Err(format!("lane manifest identity mismatch at {}", lane.plot_id).into());
        }
        lanes.push(lane);
    }
    if lanes.len() != DAYMET_LANE_COUNT {
        return Err("lane manifest count mismatch".into());
    }
    Ok(lanes)
}

fn authenticate_calendar(path: &Path, lanes: &[Lane]) -> Result<(), Box<dyn Error>> {
    let mut lines = BufReader::new(File::open(path)?).lines();
    if lines.next().transpose()?.as_deref() != Some("lane_index,plot_id,year,yday") {
        return Err("trace calendar header mismatch".into());
    }
    for lane in lanes {
        for year in 1989..=2024 {
            for day in 1..=180_u16 {
                let line = lines
                    .next()
                    .transpose()?
                    .ok_or("trace calendar ended early")?;
                let expected = format!("{},{},{year},{day}", lane.index, lane.plot_id);
                if line != expected {
                    return Err(format!("trace calendar mismatch: {line} != {expected}").into());
                }
            }
        }
    }
    if lines.next().transpose()?.is_some() {
        return Err("trace calendar has extra rows".into());
    }
    Ok(())
}

fn eligible_crossing(values: &[f64]) -> Option<u16> {
    let mut previous = None;
    for (index, &current) in values.iter().enumerate() {
        let day = index as u16 + 1;
        if (60..=180).contains(&day) && previous.is_some_and(|prior| prior < 0.5) && current >= 0.5
        {
            return Some(day);
        }
        previous = Some(current);
    }
    None
}

fn median(values: &mut [f64]) -> f64 {
    values.sort_by(f64::total_cmp);
    let middle = values.len() / 2;
    if values.len().is_multiple_of(2) {
        0.5 * (values[middle - 1] + values[middle])
    } else {
        values[middle]
    }
}

fn equal_year_objective(squares_by_year: &BTreeMap<i32, Vec<f64>>) -> Option<f64> {
    if squares_by_year.is_empty() || squares_by_year.values().any(Vec::is_empty) {
        return None;
    }
    let year_mses = squares_by_year
        .values()
        .map(|squares| squares.iter().sum::<f64>() / squares.len() as f64)
        .collect::<Vec<_>>();
    Some((year_mses.iter().sum::<f64>() / year_mses.len() as f64).sqrt())
}

fn missing_canonical_groups(
    lanes: &[Lane],
    crossings: &BTreeMap<(String, i32), Option<u16>>,
) -> Vec<(String, i32)> {
    lanes
        .iter()
        .flat_map(|lane| {
            (1989..=2024)
                .filter(|year| {
                    crossings
                        .get(&(lane.plot_id.clone(), *year))
                        .is_none_or(Option::is_none)
                })
                .map(|year| (lane.plot_id.clone(), year))
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let trace_path = PathBuf::from(arg_value(&args, "--trace")?);
    let identity_path = PathBuf::from(arg_value(&args, "--identity")?);
    let configs_path = PathBuf::from(arg_value(&args, "--configs")?);
    let observations_path = PathBuf::from(arg_value(&args, "--observations")?);
    let out = PathBuf::from(arg_value(&args, "--out")?);
    let object_root = PathBuf::from("/home/workdir/cal04b-objects/primary");
    fs::create_dir_all(&object_root)?;
    fs::create_dir_all(&out)?;

    let identity = read_identity(&identity_path)?;
    authenticate(&identity, &trace_path, &configs_path)?;
    let lanes = read_lanes(Path::new(field(&identity, "lane_manifest_path")?))?;
    authenticate_calendar(Path::new(field(&identity, "calendar_path")?), &lanes)?;
    let lane_by_plot = lanes
        .iter()
        .map(|lane| (lane.plot_id.clone(), lane.index))
        .collect::<HashMap<_, _>>();
    let configs = read_configs(&configs_path)?;
    let observations = read_observations(&observations_path)?;
    if observations
        .iter()
        .any(|observation| !lane_by_plot.contains_key(&observation.plot_id))
    {
        return Err("observation contains unauthenticated plot".into());
    }

    let mut reader = BufReader::with_capacity(8 * 1024 * 1024, File::open(&trace_path)?);
    let header = TraceHeader::read(&mut reader)?;
    if header
        != (TraceHeader {
            candidate_count: configs.len(),
            lane_count: lanes.len(),
            days_per_lane: CALIBRATION_DAYS_PER_LANE,
        })
        || fs::metadata(&trace_path)?.len() != header.expected_bytes()?
    {
        return Err("trace header/size mismatch".into());
    }

    let observation_path = object_root.join("candidate-observation-components.csv");
    let crossing_path = object_root.join("candidate-crossing-components.csv");
    let annual_path = object_root.join("candidate-annual-components.csv");
    let diagnostics_path = object_root.join("candidate-diagnostics.csv");
    let mut observation_writer = BufWriter::new(File::create(&observation_path)?);
    let mut crossing_writer = BufWriter::new(File::create(&crossing_path)?);
    let mut annual_writer = BufWriter::new(File::create(&annual_path)?);
    let mut diagnostics_writer = BufWriter::new(File::create(&diagnostics_path)?);
    writeln!(observation_writer, "candidate_id,plot_id,lane_index,record_id,year,species,crossing_doy,lower_doy,upper_doy,distance_days,squared_distance")?;
    writeln!(
        crossing_writer,
        "candidate_id,plot_id,lane_index,year,crossing_doy,eligibility_start_yday,eligibility_end_yday,state"
    )?;
    writeln!(
        annual_writer,
        "candidate_id,year,observation_count,annual_mse,annual_rmse"
    )?;
    writeln!(diagnostics_writer, "candidate_id,species_rmse,observation_median_absolute_distance,year_median_absolute_distance,interval_coverage_fraction,failed_records,failed_years")?;

    let mut objectives = Vec::with_capacity(configs.len());
    let mut failures_by_candidate = Vec::with_capacity(configs.len());
    let mut value_bytes = [0_u8; 8];
    for config in &configs {
        let mut crossings: BTreeMap<(String, i32), Option<u16>> = BTreeMap::new();
        for lane in &lanes {
            for year in 1989..=2024 {
                let mut year_values = Vec::with_capacity(CALIBRATION_DAYS_PER_YEAR);
                for _ in 0..CALIBRATION_DAYS_PER_YEAR {
                    reader.read_exact(&mut value_bytes)?;
                    let value = f64::from_le_bytes(value_bytes);
                    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
                        return Err(format!(
                            "{} plot {} year {year} has invalid GSI",
                            config.id, lane.plot_id
                        )
                        .into());
                    }
                    year_values.push(value);
                }
                let crossing = eligible_crossing(&year_values);
                writeln!(
                    crossing_writer,
                    "{},{},{},{year},{},60,180,{}",
                    config.id,
                    lane.plot_id,
                    lane.index,
                    crossing.map_or_else(String::new, |day| day.to_string()),
                    if crossing.is_some() {
                        "FOUND"
                    } else {
                        "MISSING"
                    }
                )?;
                crossings.insert((lane.plot_id.clone(), year), crossing);
            }
        }

        let mut squares_by_plot_year: BTreeMap<(String, i32), Vec<f64>> = BTreeMap::new();
        let mut squares_by_year: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
        let mut squares_by_species: BTreeMap<String, Vec<f64>> = BTreeMap::new();
        let mut absolute_distances = Vec::new();
        let mut coverage = 0_usize;
        let mut failed_records = 0_usize;
        for observation in &observations {
            let key = (observation.plot_id.clone(), observation.year);
            let crossing = crossings[&key];
            let (distance, square, crossing_text) = if let Some(day) = crossing {
                let day_i32 = i32::from(day);
                let distance = if day_i32 < observation.lower {
                    observation.lower - day_i32
                } else if day_i32 > observation.upper {
                    day_i32 - observation.upper
                } else {
                    0
                };
                if distance == 0 {
                    coverage += 1;
                }
                (
                    f64::from(distance),
                    f64::from(distance * distance),
                    day.to_string(),
                )
            } else {
                failed_records += 1;
                (f64::INFINITY, f64::INFINITY, String::new())
            };
            writeln!(
                observation_writer,
                "{},{},{},{},{},{},{},{},{},{},{}",
                config.id,
                observation.plot_id,
                lane_by_plot[&observation.plot_id],
                observation.id,
                observation.year,
                observation.species,
                crossing_text,
                observation.lower,
                observation.upper,
                distance,
                square
            )?;
            if square.is_finite() {
                squares_by_plot_year.entry(key).or_default().push(square);
                squares_by_year
                    .entry(observation.year)
                    .or_default()
                    .push(square);
                squares_by_species
                    .entry(observation.species.clone())
                    .or_default()
                    .push(square);
                absolute_distances.push(distance);
            }
        }

        let observed_groups = observations
            .iter()
            .map(|row| (row.plot_id.clone(), row.year))
            .collect::<std::collections::BTreeSet<_>>();
        let mut annual_mses = Vec::new();
        let mut year_median_distances = Vec::new();
        let failed_plot_years = missing_canonical_groups(&lanes, &crossings);
        for (plot_id, year) in &observed_groups {
            if crossings[&(plot_id.clone(), *year)].is_some()
                && !squares_by_plot_year.contains_key(&(plot_id.clone(), *year))
            {
                return Err(format!(
                    "{} observed group {plot_id}/{year} lacks components",
                    config.id
                )
                .into());
            }
        }
        for year in 1989..=2024 {
            let expected = observations.iter().filter(|row| row.year == year).count();
            if expected == 0 {
                continue;
            }
            let year_failed = failed_plot_years
                .iter()
                .any(|group| group.1 == year && observed_groups.contains(group));
            if !year_failed {
                let squares = squares_by_year
                    .get(&year)
                    .ok_or("finite year lacks squared components")?;
                if squares.len() != expected {
                    return Err(format!(
                        "{} year {year} has {} components, expected {expected}",
                        config.id,
                        squares.len()
                    )
                    .into());
                }
                let mse = squares.iter().sum::<f64>() / squares.len() as f64;
                writeln!(
                    annual_writer,
                    "{},{year},{},{mse},{}",
                    config.id,
                    squares.len(),
                    mse.sqrt()
                )?;
                annual_mses.push(mse);
                let mut distances = squares
                    .iter()
                    .map(|square| square.sqrt())
                    .collect::<Vec<_>>();
                year_median_distances.push(median(&mut distances));
            } else {
                writeln!(
                    annual_writer,
                    "{},{year},{expected},+infinity,+infinity",
                    config.id
                )?;
            }
        }
        let species = squares_by_species
            .iter()
            .map(|(species, values)| {
                format!(
                    "{species}:{:.9}",
                    (values.iter().sum::<f64>() / values.len() as f64).sqrt()
                )
            })
            .collect::<Vec<_>>()
            .join(";");
        let failed_years = failed_plot_years
            .iter()
            .map(|(_, year)| *year)
            .collect::<std::collections::BTreeSet<_>>();
        writeln!(
            diagnostics_writer,
            "{},{},{},{},{},{},{}",
            config.id,
            species,
            if absolute_distances.is_empty() {
                f64::INFINITY
            } else {
                median(&mut absolute_distances)
            },
            if year_median_distances.is_empty() {
                f64::INFINITY
            } else {
                median(&mut year_median_distances)
            },
            coverage as f64 / observations.len() as f64,
            failed_records,
            failed_years.len()
        )?;
        let objective = if failed_plot_years.is_empty() {
            let independently_rebuilt = equal_year_objective(&squares_by_year)
                .ok_or("finite candidate lacks annual components")?;
            let written_objective =
                (annual_mses.iter().sum::<f64>() / annual_mses.len() as f64).sqrt();
            if independently_rebuilt.to_bits() != written_objective.to_bits() {
                return Err("equal-year objective reconstruction differs".into());
            }
            Some(written_objective)
        } else {
            None
        };
        objectives.push(objective);
        failures_by_candidate.push(failed_plot_years);
    }
    let mut extra = [0_u8; 1];
    if reader.read(&mut extra)? != 0 {
        return Err("trace has extra bytes".into());
    }
    observation_writer.flush()?;
    crossing_writer.flush()?;
    annual_writer.flush()?;
    diagnostics_writer.flush()?;

    let finite_minimum = objectives
        .iter()
        .flatten()
        .copied()
        .min_by(f64::total_cmp)
        .ok_or("no finite objective")?;
    let threshold = finite_minimum + 1.0;
    let mut candidate_writer = BufWriter::new(File::create(out.join("candidate-ledger.csv"))?);
    let mut accepted_writer =
        BufWriter::new(File::create(out.join("accepted-calibration-ensemble.csv"))?);
    let mut failure_writer = BufWriter::new(File::create(out.join("failure-ledger.csv"))?);
    writeln!(
        candidate_writer,
        "candidate_id,configuration_id,state,objective,boundary_flags,saturation_flags,evidence"
    )?;
    writeln!(
        accepted_writer,
        "candidate_id,objective,acceptance_threshold,boundary_flags,saturation_flags,state"
    )?;
    writeln!(failure_writer, "failure_id,candidate_id,plot_id,lane_index,year,stage,failure_class,attempt,typed_error,evidence")?;
    let mut accepted = 0_usize;
    let mut failure_id = 0_usize;
    for ((config, objective), failures) in
        configs.iter().zip(&objectives).zip(&failures_by_candidate)
    {
        let objective_text =
            objective.map_or_else(|| "+infinity".to_string(), |value| format!("{value:.17}"));
        writeln!(
            candidate_writer,
            "{},{},{},{},{},{},{}",
            config.id,
            config.configuration_id,
            if objective.is_some() {
                "FINITE"
            } else {
                "FAILED_REQUIRED_PLOT_YEAR_CROSSING"
            },
            objective_text,
            config.boundary,
            config.saturation,
            observation_path.display()
        )?;
        if objective.is_some_and(|value| value <= threshold) {
            accepted += 1;
            writeln!(
                accepted_writer,
                "{},{:.17},{threshold:.17},{},{},ACCEPTED_FROZEN",
                config.id,
                objective.unwrap_or_default(),
                config.boundary,
                config.saturation
            )?;
        }
        for (plot_id, year) in failures {
            failure_id += 1;
            writeln!(
                failure_writer,
                "FAIL-{failure_id:06},{},{plot_id},{},{year},gsi_timing,MISSING_REQUIRED_PLOT_YEAR_CROSSING,1,objective_positive_infinity,eligibility_yday_60_180",
                config.id, lane_by_plot[plot_id]
            )?;
        }
    }
    candidate_writer.flush()?;
    accepted_writer.flush()?;
    failure_writer.flush()?;

    let executable = env::current_exe()?;
    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/bin/reconstruct.rs");
    fs::write(
        object_root.join("reconstruction-receipt.csv"),
        format!(
            "field,value\nstate,PASS\nexact_command,{}\nsource_path,{}\nsource_sha256,{}\nbinary_path,{}\nbinary_sha256,{}\ntrace_sha256,{}\nidentity_sha256,{}\nconfig_sha256,{}\nobservation_sha256,{}\nforcing_authority_resolution_sha256,{}\ncandidate_count,{}\nplot_count,{}\nobservation_count,{}\nobjective_unit,day\nobjective_grouping,equal_year_mean_of_all_admitted_record_squared_distances\ncrossing_eligibility_yday,60-180\nstate_initialization,FRESH_GSI_STATE_EACH_CANDIDATE_PLOT_YEAR\nfinite_minimum,{finite_minimum:.17}\nacceptance_threshold,{threshold:.17}\naccepted_count,{accepted}\ncrossing_components_sha256,{}\nobservation_components_sha256,{}\nannual_components_sha256,{}\ndiagnostics_sha256,{}\n",
            args.join(" "),
            source.display(),
            sha256(&source)?,
            executable.display(),
            sha256(&executable)?,
            sha256(&trace_path)?,
            sha256(&identity_path)?,
            sha256(&configs_path)?,
            sha256(&observations_path)?,
            field(&identity, "forcing_authority_resolution_sha256")?,
            configs.len(),
            lanes.len(),
            observations.len(),
            sha256(&crossing_path)?,
            sha256(&observation_path)?,
            sha256(&annual_path)?,
            sha256(&diagnostics_path)?
        ),
    )?;
    println!(
        "PASS candidates={} plots={} accepted={accepted} minimum={finite_minimum:.9}",
        configs.len(),
        lanes.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{eligible_crossing, equal_year_objective, missing_canonical_groups, Lane};
    use std::collections::BTreeMap;

    #[test]
    fn crossing_eligibility_excludes_warmup_and_stops_at_180() {
        let mut before = vec![0.0; 365];
        before[58] = 1.0;
        assert_eq!(eligible_crossing(&before), None);

        let mut lower_boundary = vec![0.0; 365];
        lower_boundary[59] = 1.0;
        assert_eq!(eligible_crossing(&lower_boundary), Some(60));

        let mut after = vec![0.0; 365];
        after[180] = 1.0;
        assert_eq!(eligible_crossing(&after), None);

        let mut upper_boundary = vec![0.0; 180];
        upper_boundary[179] = 1.0;
        assert_eq!(eligible_crossing(&upper_boundary), Some(180));
    }

    #[test]
    fn crossing_search_has_no_prior_year_carry() {
        let prior_year = vec![1.0; 365];
        let mut next_year = vec![1.0; 365];
        next_year[59] = 0.0;
        next_year[60] = 1.0;
        assert_eq!(eligible_crossing(&prior_year), None);
        assert_eq!(eligible_crossing(&next_year), Some(61));
    }

    #[test]
    fn objective_weights_years_equally_not_plot_years_or_records() {
        let squares = BTreeMap::from([(2001, vec![0.0]), (2002, vec![4.0, 4.0, 4.0])]);
        assert_eq!(equal_year_objective(&squares), Some(2.0_f64.sqrt()));
    }

    #[test]
    fn unobserved_plot_year_missing_crossing_invalidates_candidate() {
        let lanes = vec![Lane {
            index: 0,
            plot_id: "HQ".to_string(),
        }];
        let crossings = (1989..=2024)
            .map(|year| {
                (
                    ("HQ".to_string(), year),
                    if year == 2024 { None } else { Some(100) },
                )
            })
            .collect();
        assert_eq!(
            missing_canonical_groups(&lanes, &crossings),
            vec![("HQ".to_string(), 2024)]
        );
    }
}
