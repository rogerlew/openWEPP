use cal04b_executor::{arg_value, days_in_year, read_configs, sha256};
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

const MAGIC: [u8; 8] = *b"SYN04B02";
const IDS: [&str; 3] = ["GSI-0001", "GSI-5557", "GSI-9261"];
const TRUTH: &str = "GSI-5557";

fn fields(path: &Path) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let mut result = HashMap::new();
    for line in text.lines().skip(1) {
        let comma = line.find(',').ok_or("identity delimiter absent")?;
        result.insert(line[..comma].to_owned(), line[comma + 1..].to_owned());
    }
    Ok(result)
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
    let configs_path = PathBuf::from(arg_value(&argv, "--configs")?);
    let primary = PathBuf::from(arg_value(&argv, "--primary")?);
    let output = PathBuf::from(arg_value(&argv, "--out")?);
    if output.exists() {
        return Err("synthetic verification output already exists".into());
    }
    fs::create_dir_all(&output)?;
    let metadata = fields(&identity_path)?;
    if metadata.get("case_id").map(String::as_str) != Some("SYN-GSI-01")
        || metadata.get("hidden_candidate").map(String::as_str) != Some(TRUTH)
        || metadata.get("candidate_ids").map(String::as_str) != Some("GSI-0001|GSI-5557|GSI-9261")
        || metadata.get("trace_sha256") != Some(&sha256(&trace_path)?)
        || metadata.get("config_sha256") != Some(&sha256(&configs_path)?)
    {
        return Err("synthetic verification identity mismatch".into());
    }
    let configs = read_configs(&configs_path)?;
    if IDS
        .iter()
        .any(|id| configs.iter().all(|config| config.id != *id))
    {
        return Err("synthetic verification config missing".into());
    }
    let raw = fs::read(&trace_path)?;
    if raw.len() < 20 || raw[..8] != MAGIC {
        return Err("synthetic verification trace prefix invalid".into());
    }
    let count = u32::from_le_bytes(raw[8..12].try_into()?) as usize;
    let start = i32::from_le_bytes(raw[12..16].try_into()?);
    let days = u32::from_le_bytes(raw[16..20].try_into()?) as usize;
    let expected_days: usize = (2001..=2003)
        .map(|year| usize::from(days_in_year(year)))
        .sum();
    if count != IDS.len()
        || start != 2001
        || days != expected_days
        || raw.len() != 20 + count * days * 8
    {
        return Err("synthetic verification trace dimensions invalid".into());
    }

    let mut crossings: HashMap<&str, HashMap<i32, u16>> = HashMap::new();
    for (candidate_index, id) in IDS.iter().enumerate() {
        let base = 20 + candidate_index * days * 8;
        let mut year_crossings = HashMap::new();
        let mut previous: Option<f64> = None;
        let mut day_offset = 0_usize;
        for year in 2001..=2003 {
            for ordinal in 1..=days_in_year(year) {
                let byte_offset = base + day_offset * 8;
                let current = f64::from_le_bytes(raw[byte_offset..byte_offset + 8].try_into()?);
                if !current.is_finite() || !(0.0..=1.0).contains(&current) {
                    return Err(format!("invalid verification value for {id}").into());
                }
                if !year_crossings.contains_key(&year)
                    && matches!(previous, Some(value) if value < 0.5)
                    && current >= 0.5
                {
                    year_crossings.insert(year, ordinal);
                }
                previous = Some(current);
                day_offset += 1;
            }
        }
        crossings.insert(id, year_crossings);
    }
    let truth_crossings = crossings.get(TRUTH).ok_or("verification truth absent")?;
    if truth_crossings.len() != 3 {
        return Err("verification truth crossing incomplete".into());
    }
    let component_path = output.join("candidate-observation-components.csv");
    let annual_path = output.join("candidate-annual-components.csv");
    let ledger_path = output.join("candidate-ledger.csv");
    let membership_path = output.join("accepted-synthetic-ensemble.csv");
    let mut component_writer = BufWriter::new(File::create(&component_path)?);
    let mut annual_writer = BufWriter::new(File::create(&annual_path)?);
    writeln!(component_writer, "candidate_id,record_id,year,crossing_doy,lower_doy,upper_doy,distance_days,squared_distance")?;
    writeln!(
        annual_writer,
        "candidate_id,year,crossing_doy,observation_count,annual_mse,annual_rmse"
    )?;
    let mut objective_by_id = BTreeMap::new();
    for id in IDS {
        let candidate = crossings.get(id).ok_or("verification candidate absent")?;
        let mut sum = 0.0_f64;
        let mut complete = true;
        for year in [2001, 2002, 2003] {
            let truth_day = i32::from(truth_crossings[&year]);
            let low = truth_day - 2;
            let high = truth_day + 2;
            if let Some(crossing) = candidate.get(&year).copied() {
                let crossing = i32::from(crossing);
                let distance = (low - crossing).max(0) + (crossing - high).max(0);
                let squared = f64::from(distance).powi(2);
                writeln!(
                    component_writer,
                    "{id},SYN-{year},{year},{crossing},{low},{high},{distance},{squared:.12}"
                )?;
                writeln!(
                    annual_writer,
                    "{id},{year},{crossing},1,{squared:.12},{:.12}",
                    squared.sqrt()
                )?;
                sum += squared;
            } else {
                complete = false;
                writeln!(
                    component_writer,
                    "{id},SYN-{year},{year},,{low},{high},+infinity,+infinity"
                )?;
                writeln!(annual_writer, "{id},{year},,1,+infinity,+infinity")?;
            }
        }
        objective_by_id.insert(
            id,
            if complete {
                (sum / 3.0).sqrt()
            } else {
                f64::INFINITY
            },
        );
    }
    component_writer.flush()?;
    annual_writer.flush()?;
    let minimum = objective_by_id
        .values()
        .copied()
        .fold(f64::INFINITY, f64::min);
    let mut ledger_writer = BufWriter::new(File::create(&ledger_path)?);
    let mut membership_writer = BufWriter::new(File::create(&membership_path)?);
    writeln!(
        ledger_writer,
        "candidate_id,state,objective,minimum_objective,member"
    )?;
    writeln!(membership_writer, "candidate_id,objective,state")?;
    let mut recovered = Vec::new();
    for id in IDS {
        let objective = objective_by_id[id];
        let selected = objective.is_finite() && objective.total_cmp(&minimum).is_eq();
        let objective_text = if objective.is_finite() {
            format!("{objective:.12}")
        } else {
            "+infinity".to_owned()
        };
        writeln!(
            ledger_writer,
            "{id},{},{objective_text},{minimum:.12},{}",
            if objective.is_finite() {
                "FINITE"
            } else {
                "FAILED_REQUIRED_CROSSING"
            },
            if selected { "TRUE" } else { "FALSE" }
        )?;
        if selected {
            recovered.push(id);
            writeln!(membership_writer, "{id},{objective:.12},RECOVERED_MINIMUM")?;
        }
    }
    ledger_writer.flush()?;
    membership_writer.flush()?;
    if objective_by_id[TRUTH] != 0.0
        || !recovered.contains(&TRUTH)
        || !objective_by_id
            .iter()
            .any(|(id, value)| *id != TRUTH && *value > 0.0)
    {
        return Err("verification recovery or competitor evidence failed".into());
    }
    exact(
        &component_path,
        &primary.join("candidate-observation-components.csv"),
        "synthetic components",
    )?;
    exact(
        &annual_path,
        &primary.join("candidate-annual-components.csv"),
        "synthetic annual components",
    )?;
    exact(
        &ledger_path,
        &primary.join("candidate-ledger.csv"),
        "synthetic candidate ledger",
    )?;
    exact(
        &membership_path,
        &primary.join("accepted-synthetic-ensemble.csv"),
        "synthetic membership",
    )?;
    let receipt_path = output.join("verification-reconstruction-receipt.csv");
    fs::write(
        &receipt_path,
        format!(
            "field,value\nstate,PASS\ncase_id,SYN-GSI-01\ntrace_sha256,{}\nhidden_candidate,GSI-5557\nhidden_objective,0.000000000000\nrecovered_set,{}\nnonvacuous_competitor,TRUE\nexact_primary_match,TRUE\ncomponents_sha256,{}\nannual_sha256,{}\ncandidate_ledger_sha256,{}\naccepted_ensemble_sha256,{}\n",
            sha256(&trace_path)?,
            recovered.join("|"),
            sha256(&component_path)?,
            sha256(&annual_path)?,
            sha256(&ledger_path)?,
            sha256(&membership_path)?,
        ),
    )?;
    println!(
        "PASS synthetic verification recovered={} receipt={}",
        recovered.join("|"),
        receipt_path.display()
    );
    Ok(())
}
