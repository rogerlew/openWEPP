use cal04b_executor::{arg_value, days_in_year, sha256, HOLDOUT_MAGIC};
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
    lower: i32,
    upper: i32,
}

fn identity(path: &Path) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut result = HashMap::new();
    for (index, line) in fs::read_to_string(path)?.lines().enumerate() {
        if index == 0 {
            continue;
        }
        let (key, value) = line.split_once(',').ok_or("bad holdout identity row")?;
        result.insert(key.to_string(), value.to_string());
    }
    Ok(result)
}

fn field<'a>(identity: &'a HashMap<String, String>, key: &str) -> Result<&'a str, Box<dyn Error>> {
    identity
        .get(key)
        .map(String::as_str)
        .ok_or_else(|| format!("missing holdout identity {key}").into())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let trace_path = PathBuf::from(arg_value(&args, "--trace")?);
    let identity_path = PathBuf::from(arg_value(&args, "--identity")?);
    let calendar_path = PathBuf::from(arg_value(&args, "--calendar")?);
    let accepted_path = PathBuf::from(arg_value(&args, "--accepted")?);
    let timing_path = PathBuf::from(arg_value(&args, "--observations")?);
    let observation_out = PathBuf::from(arg_value(&args, "--observation-out")?);
    let annual_out = PathBuf::from(arg_value(&args, "--annual-out")?);
    let result_out = PathBuf::from(arg_value(&args, "--result-out")?);
    let accepted: Vec<String> = BufReader::new(File::open(&accepted_path)?)
        .lines()
        .skip(1)
        .map(|line| line.map(|value| value.split(',').next().unwrap_or_default().to_string()))
        .collect::<Result<_, _>>()?;
    if accepted.is_empty() {
        return Err("empty accepted holdout membership".into());
    }
    let calendar: Vec<(i32, u16)> = BufReader::new(File::open(&calendar_path)?)
        .lines()
        .skip(1)
        .map(|line| {
            let line = line?;
            let (year, day) = line
                .split_once(',')
                .ok_or_else(|| std::io::Error::other("bad calendar"))?;
            Ok((
                year.parse().map_err(std::io::Error::other)?,
                day.parse().map_err(std::io::Error::other)?,
            ))
        })
        .collect::<Result<_, std::io::Error>>()?;
    if calendar.first() != Some(&(1991, 1))
        || calendar.last() != Some(&(2023, 365))
        || calendar.len() != 12_053
    {
        return Err("holdout calendar extent/count mismatch".into());
    }
    for pair in calendar.windows(2) {
        let expected = if pair[0].1 == days_in_year(pair[0].0) {
            (pair[0].0 + 1, 1)
        } else {
            (pair[0].0, pair[0].1 + 1)
        };
        if pair[1] != expected {
            return Err("holdout calendar is not consecutive".into());
        }
    }
    let meta = identity(&identity_path)?;
    for (key, expected) in [
        ("schema", "CAL04B02"),
        ("site_id", "harvard"),
        ("arm_id", "deciduous"),
        ("lane_count", "1"),
        ("lane_index", "0"),
        ("retained_start", "1991-001"),
        ("retained_end", "2023-365"),
        ("retained_day_count", "12053"),
    ] {
        if field(&meta, key)? != expected {
            return Err(format!("holdout identity differs for {key}").into());
        }
    }
    if field(&meta, "candidate_count")?.parse::<usize>()? != accepted.len()
        || Path::new(field(&meta, "trace_path")?) != trace_path
        || Path::new(field(&meta, "calendar_path")?) != calendar_path
        || Path::new(field(&meta, "accepted_path")?) != accepted_path
        || fs::metadata(&trace_path)?.len().to_string() != field(&meta, "trace_bytes")?
    {
        return Err("holdout identity path/count/bytes mismatch".into());
    }
    for (path_key, digest_key) in [
        ("trace_path", "trace_sha256"),
        ("calendar_path", "calendar_sha256"),
        ("config_path", "config_sha256"),
        ("accepted_path", "accepted_sha256"),
        ("climate_path", "climate_sha256"),
        ("producer_source", "producer_source_sha256"),
        ("producer_binary", "producer_binary_sha256"),
    ] {
        if sha256(Path::new(field(&meta, path_key)?))? != field(&meta, digest_key)? {
            return Err(format!("holdout identity digest differs for {path_key}").into());
        }
    }
    let latitude: f64 = field(&meta, "latitude_degrees")?.parse()?;
    if !latitude.is_finite() || !(-90.0..=90.0).contains(&latitude) {
        return Err("holdout latitude identity invalid".into());
    }
    if field(&meta, "exact_command")?.is_empty() {
        return Err("holdout producer command identity empty".into());
    }
    let mut observations = Vec::new();
    for (index, line) in fs::read_to_string(timing_path)?.lines().enumerate() {
        if index == 0 {
            continue;
        }
        let fields: Vec<_> = line.split(',').collect();
        if fields.len() != 13 {
            return Err("bad timing row".into());
        }
        if fields[11] == "INDEPENDENT_HOLDOUT" {
            observations.push(Observation {
                id: fields[0].into(),
                year: fields[1].parse()?,
                species: fields[3].into(),
                lower: fields[8].parse()?,
                upper: fields[10].parse()?,
            });
        }
    }
    if observations.len() != 319 {
        return Err(format!("holdout observations {}", observations.len()).into());
    }
    let mut trace = BufReader::with_capacity(8 * 1024 * 1024, File::open(trace_path)?);
    let mut magic = [0; 8];
    trace.read_exact(&mut magic)?;
    if &magic != HOLDOUT_MAGIC {
        return Err("holdout magic mismatch".into());
    }
    let mut four = [0; 4];
    trace.read_exact(&mut four)?;
    let count = u32::from_le_bytes(four) as usize;
    trace.read_exact(&mut four)?;
    let year = i32::from_le_bytes(four);
    trace.read_exact(&mut four)?;
    let days = u32::from_le_bytes(four) as usize;
    if count != accepted.len() || year != 1991 || days != calendar.len() {
        return Err("holdout header mismatch".into());
    }
    let mut obs_writer = BufWriter::new(File::create(observation_out)?);
    let mut annual_writer = BufWriter::new(File::create(annual_out)?);
    let mut result_writer = BufWriter::new(File::create(result_out)?);
    writeln!(obs_writer,"candidate_id,record_id,year,species,crossing_doy,lower_doy,upper_doy,distance_days,squared_distance")?;
    writeln!(
        annual_writer,
        "candidate_id,year,crossing_doy,observation_count,annual_mse,annual_rmse"
    )?;
    writeln!(result_writer,"candidate_id,aggregate_score,species_rmse,observation_median_absolute_distance,year_median_absolute_distance,interval_coverage_fraction,failed_records,failed_years,state")?;
    let mut bytes = [0; 8];
    for candidate in accepted {
        let mut crossings: HashMap<i32, u16> = HashMap::new();
        let mut previous = None;
        for &(year, day) in &calendar {
            trace.read_exact(&mut bytes)?;
            let current = f64::from_le_bytes(bytes);
            if !current.is_finite() || !(0.0..=1.0).contains(&current) {
                return Err(format!("invalid holdout daily GSI for {candidate}").into());
            }
            if !crossings.contains_key(&year)
                && previous.is_some_and(|prior| prior > 0.5)
                && current <= 0.5
            {
                crossings.insert(year, day);
            }
            previous = Some(current);
        }
        let mut annual: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
        let mut species: BTreeMap<String, Vec<f64>> = BTreeMap::new();
        let mut abs = Vec::new();
        let mut coverage = 0;
        let mut failed_records = 0;
        for observation in &observations {
            if let Some(&crossing) = crossings.get(&observation.year) {
                let day = i32::from(crossing);
                let distance = (observation.lower - day).max(0) + (day - observation.upper).max(0);
                let square = f64::from(distance * distance);
                writeln!(
                    obs_writer,
                    "{candidate},{},{},{},{crossing},{},{},{distance},{square}",
                    observation.id,
                    observation.year,
                    observation.species,
                    observation.lower,
                    observation.upper
                )?;
                annual.entry(observation.year).or_default().push(square);
                species
                    .entry(observation.species.clone())
                    .or_default()
                    .push(square);
                abs.push(f64::from(distance));
                if distance == 0 {
                    coverage += 1;
                }
            } else {
                failed_records += 1;
                writeln!(
                    obs_writer,
                    "{candidate},{},{},{},,{},{},inf,inf",
                    observation.id,
                    observation.year,
                    observation.species,
                    observation.lower,
                    observation.upper
                )?;
            }
        }
        let mut mses = Vec::new();
        let mut year_median_abs = Vec::new();
        let mut failed_years = Vec::new();
        for year in 1991..=2023 {
            let expected = observations.iter().filter(|row| row.year == year).count();
            if expected == 0 {
                continue;
            }
            if let (Some(&crossing), Some(values)) = (crossings.get(&year), annual.get(&year)) {
                let mse = values.iter().sum::<f64>() / values.len() as f64;
                writeln!(
                    annual_writer,
                    "{candidate},{year},{crossing},{},{mse},{}",
                    values.len(),
                    mse.sqrt()
                )?;
                mses.push(mse);
                let mut distances = values
                    .iter()
                    .map(|square| square.sqrt())
                    .collect::<Vec<_>>();
                distances.sort_by(f64::total_cmp);
                year_median_abs.push(if distances.len().is_multiple_of(2) {
                    0.5 * (distances[distances.len() / 2 - 1] + distances[distances.len() / 2])
                } else {
                    distances[distances.len() / 2]
                });
            } else {
                failed_years.push(year);
                writeln!(
                    annual_writer,
                    "{candidate},{year},,{expected},+infinity,+infinity"
                )?;
            }
        }
        abs.sort_by(f64::total_cmp);
        year_median_abs.sort_by(f64::total_cmp);
        let median = |values: &Vec<f64>| {
            if values.is_empty() {
                f64::INFINITY
            } else if values.len().is_multiple_of(2) {
                0.5 * (values[values.len() / 2 - 1] + values[values.len() / 2])
            } else {
                values[values.len() / 2]
            }
        };
        let aggregate = if failed_records == 0 && failed_years.is_empty() {
            (mses.iter().sum::<f64>() / mses.len() as f64).sqrt()
        } else {
            f64::INFINITY
        };
        let species_text = species
            .iter()
            .map(|(key, values)| {
                format!(
                    "{key}:{:.9}",
                    (values.iter().sum::<f64>() / values.len() as f64).sqrt()
                )
            })
            .collect::<Vec<_>>()
            .join(";");
        let failed_text = failed_years
            .iter()
            .map(i32::to_string)
            .collect::<Vec<_>>()
            .join(";");
        writeln!(
            result_writer,
            "{candidate},{aggregate},{species_text},{},{},{},{failed_records},{failed_text},{}",
            median(&abs),
            median(&year_median_abs),
            coverage as f64 / observations.len() as f64,
            if aggregate.is_finite() {
                "SCORED_NO_REFIT"
            } else {
                "RETAINED_VALIDATION_FAILURE"
            }
        )?;
    }
    let mut extra = [0_u8; 1];
    if trace.read(&mut extra)? != 0 {
        return Err("holdout trace has extra bytes".into());
    }
    obs_writer.flush()?;
    annual_writer.flush()?;
    result_writer.flush()?;
    println!("PASS holdout reconstructed");
    Ok(())
}
