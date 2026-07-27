use cal04b_executor::{arg_value, days_in_year, read_configs, sha256};
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

const SYNTHETIC_MAGIC: &[u8; 8] = b"SYN04B02";
const CANDIDATES: [&str; 3] = ["GSI-0001", "GSI-5557", "GSI-9261"];
const HIDDEN: &str = "GSI-5557";

fn identity(path: &Path) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut fields = HashMap::new();
    for (number, line) in fs::read_to_string(path)?.lines().enumerate() {
        if number == 0 {
            continue;
        }
        let (key, value) = line
            .split_once(',')
            .ok_or_else(|| format!("malformed identity row {}", number + 1))?;
        fields.insert(key.to_string(), value.to_string());
    }
    Ok(fields)
}

fn first_crossings(values: &[f64]) -> Result<BTreeMap<i32, u16>, Box<dyn Error>> {
    let mut result = BTreeMap::new();
    let mut offset = 0_usize;
    let mut previous: Option<f64> = None;
    for year in 2001..=2003 {
        for day in 1..=days_in_year(year) {
            let current = values[offset];
            if !current.is_finite() || !(0.0..=1.0).contains(&current) {
                return Err(format!("invalid GSI at {year}-{day}").into());
            }
            if !result.contains_key(&year)
                && previous.is_some_and(|prior| prior < 0.5)
                && current >= 0.5
            {
                result.insert(year, day);
            }
            previous = Some(current);
            offset += 1;
        }
    }
    if offset != values.len() {
        return Err("daily trace length differs from calendar".into());
    }
    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let trace_path = PathBuf::from(arg_value(&args, "--trace")?);
    let identity_path = PathBuf::from(arg_value(&args, "--identity")?);
    let configs_path = PathBuf::from(arg_value(&args, "--configs")?);
    let output = PathBuf::from(arg_value(&args, "--out")?);
    if output.exists() {
        return Err("synthetic primary output already exists".into());
    }
    fs::create_dir_all(&output)?;
    let metadata = identity(&identity_path)?;
    if metadata.get("schema").map(String::as_str) != Some("SYN04B02")
        || metadata.get("hidden_candidate").map(String::as_str) != Some(HIDDEN)
        || metadata.get("candidate_ids").map(String::as_str) != Some(&CANDIDATES.join("|"))
        || metadata.get("trace_sha256") != Some(&sha256(&trace_path)?)
        || metadata.get("config_sha256") != Some(&sha256(&configs_path)?)
    {
        return Err("synthetic identity mismatch".into());
    }
    let configs = read_configs(&configs_path)?;
    for id in CANDIDATES {
        if !configs.iter().any(|row| row.id == id) {
            return Err(format!("missing synthetic config {id}").into());
        }
    }
    let expected_days: usize = (2001..=2003)
        .map(|year| usize::from(days_in_year(year)))
        .sum();
    let mut reader = BufReader::new(File::open(&trace_path)?);
    let mut magic = [0_u8; 8];
    reader.read_exact(&mut magic)?;
    let mut four = [0_u8; 4];
    reader.read_exact(&mut four)?;
    let candidate_count = u32::from_le_bytes(four) as usize;
    reader.read_exact(&mut four)?;
    let start_year = i32::from_le_bytes(four);
    reader.read_exact(&mut four)?;
    let day_count = u32::from_le_bytes(four) as usize;
    if &magic != SYNTHETIC_MAGIC
        || candidate_count != CANDIDATES.len()
        || start_year != 2001
        || day_count != expected_days
    {
        return Err("synthetic trace header mismatch".into());
    }
    let mut crossings = BTreeMap::new();
    let mut bytes = [0_u8; 8];
    for id in CANDIDATES {
        let mut values = Vec::with_capacity(day_count);
        for _ in 0..day_count {
            reader.read_exact(&mut bytes)?;
            values.push(f64::from_le_bytes(bytes));
        }
        crossings.insert(id, first_crossings(&values)?);
    }
    let mut extra = [0_u8; 1];
    if reader.read(&mut extra)? != 0 {
        return Err("synthetic trace has trailing bytes".into());
    }
    let hidden = crossings.get(HIDDEN).ok_or("hidden crossings absent")?;
    if hidden.len() != 3 {
        return Err("hidden candidate lacks one crossing per year".into());
    }

    let components_path = output.join("candidate-observation-components.csv");
    let annual_path = output.join("candidate-annual-components.csv");
    let candidate_path = output.join("candidate-ledger.csv");
    let accepted_path = output.join("accepted-synthetic-ensemble.csv");
    let mut components = BufWriter::new(File::create(&components_path)?);
    let mut annual = BufWriter::new(File::create(&annual_path)?);
    writeln!(components, "candidate_id,record_id,year,crossing_doy,lower_doy,upper_doy,distance_days,squared_distance")?;
    writeln!(
        annual,
        "candidate_id,year,crossing_doy,observation_count,annual_mse,annual_rmse"
    )?;
    let mut objectives = BTreeMap::new();
    for id in CANDIDATES {
        let candidate_crossings = crossings.get(id).ok_or("candidate crossings absent")?;
        let mut annual_mse = Vec::new();
        let mut complete = true;
        for year in 2001..=2003 {
            let hidden_crossing = i32::from(*hidden.get(&year).ok_or("hidden crossing absent")?);
            let lower = hidden_crossing - 2;
            let upper = hidden_crossing + 2;
            if let Some(crossing) = candidate_crossings.get(&year).copied() {
                let crossing = i32::from(crossing);
                let distance = if crossing < lower {
                    lower - crossing
                } else if crossing > upper {
                    crossing - upper
                } else {
                    0
                };
                let square = f64::from(distance * distance);
                writeln!(
                    components,
                    "{id},SYN-{year},{year},{crossing},{lower},{upper},{distance},{square:.12}"
                )?;
                writeln!(
                    annual,
                    "{id},{year},{crossing},1,{square:.12},{:.12}",
                    square.sqrt()
                )?;
                annual_mse.push(square);
            } else {
                complete = false;
                writeln!(
                    components,
                    "{id},SYN-{year},{year},,{lower},{upper},+infinity,+infinity"
                )?;
                writeln!(annual, "{id},{year},,1,+infinity,+infinity")?;
            }
        }
        let objective = if complete {
            (annual_mse.iter().sum::<f64>() / annual_mse.len() as f64).sqrt()
        } else {
            f64::INFINITY
        };
        objectives.insert(id, objective);
    }
    components.flush()?;
    annual.flush()?;
    let minimum = objectives
        .values()
        .copied()
        .min_by(f64::total_cmp)
        .ok_or("no synthetic objectives")?;
    let mut candidate_writer = BufWriter::new(File::create(&candidate_path)?);
    let mut accepted_writer = BufWriter::new(File::create(&accepted_path)?);
    writeln!(
        candidate_writer,
        "candidate_id,state,objective,minimum_objective,member"
    )?;
    writeln!(accepted_writer, "candidate_id,objective,state")?;
    let mut members = Vec::new();
    for id in CANDIDATES {
        let objective = objectives[id];
        let member = objective.is_finite() && objective == minimum;
        let objective_text = if objective.is_finite() {
            format!("{objective:.12}")
        } else {
            "+infinity".to_string()
        };
        writeln!(
            candidate_writer,
            "{id},{},{objective_text},{minimum:.12},{}",
            if objective.is_finite() {
                "FINITE"
            } else {
                "FAILED_REQUIRED_CROSSING"
            },
            if member { "TRUE" } else { "FALSE" }
        )?;
        if member {
            members.push(id);
            writeln!(accepted_writer, "{id},{objective:.12},RECOVERED_MINIMUM")?;
        }
    }
    candidate_writer.flush()?;
    accepted_writer.flush()?;
    if objectives[HIDDEN] != 0.0
        || !members.contains(&HIDDEN)
        || !objectives
            .iter()
            .any(|(id, objective)| *id != HIDDEN && *objective > 0.0)
    {
        return Err("synthetic exact recovery or non-vacuity failed".into());
    }
    let receipt = output.join("primary-reconstruction-receipt.csv");
    fs::write(
        &receipt,
        format!(
            "field,value\nstate,PASS\ncase_id,SYN-GSI-01\ntrace_sha256,{}\nhidden_candidate,GSI-5557\nhidden_objective,0.000000000000\nrecovered_set,{}\nnonvacuous_competitor,TRUE\ncomponents_sha256,{}\nannual_sha256,{}\ncandidate_ledger_sha256,{}\naccepted_ensemble_sha256,{}\n",
            sha256(&trace_path)?,
            members.join("|"),
            sha256(&components_path)?,
            sha256(&annual_path)?,
            sha256(&candidate_path)?,
            sha256(&accepted_path)?,
        ),
    )?;
    println!(
        "PASS synthetic primary recovered={} receipt={}",
        members.join("|"),
        receipt.display()
    );
    Ok(())
}
