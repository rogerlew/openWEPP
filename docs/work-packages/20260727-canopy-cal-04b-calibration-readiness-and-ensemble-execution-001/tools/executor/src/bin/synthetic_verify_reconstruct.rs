use cal04b_executor::{arg_value, days_in_year, read_configs, sha256};
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

const MAGIC: [u8; 8] = *b"SYN04B03";
const IDS: [&str; 4] = ["GSI-0001", "GSI-0064", "GSI-5557", "GSI-9261"];
const TRUTH: &str = "GSI-5557";
const FINITE_CHALLENGER: &str = "GSI-0064";
const CHALLENGER_RULE: &str = "lexicographically first non-hidden grid candidate with exactly one eligible crossing per year outside the hidden +/-2-day interval under the frozen corrected forcing";
const STATE_RULE: &str = "each candidate-year is a separate native GsiState cold start; admit ordinal days 1-365 in order; no synthetic prefill or cross-year carry";
const CROSSING_RULE: &str = "count every upward previous<0.5 and current>=0.5 crossing only on ordinal days 60-180; hidden truth requires exactly one eligible crossing per year; competitors use their first eligible crossing and retain the full count";
const OBSERVATION_OPERATOR: &str = "for each of 3 complete calendar years: closed interval [hidden sole eligible spring crossing-2 days;hidden sole eligible spring crossing+2 days]";
const REQUIRED_RESULT: &str = "both independent reconstructors emit 4 candidate x 3 year completeness and crossing counts; identical components/counts/minimum set; GSI-5557 has exactly one eligible crossing per year, objective=0, and is included; GSI-0064 has exactly one eligible crossing per year and finite objective>0; boundary competitors retain missing-crossing failures";

fn fields(path: &Path) -> Result<HashMap<String, String>, Box<dyn Error>> {
    fields_from_text(&fs::read_to_string(path)?)
}

fn fields_from_text(text: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut lines = text.lines();
    if lines.next() != Some("field,value") {
        return Err("verification identity header mismatch".into());
    }
    let mut result = HashMap::new();
    for (index, line) in lines.enumerate() {
        let comma = line.find(',').ok_or("identity delimiter absent")?;
        let key = &line[..comma];
        if key.is_empty()
            || result
                .insert(key.to_owned(), line[comma + 1..].to_owned())
                .is_some()
        {
            return Err(
                format!("empty or duplicate verification field at row {}", index + 2).into(),
            );
        }
    }
    Ok(result)
}

fn static_fields_match(metadata: &HashMap<String, String>) -> bool {
    metadata.get("schema").map(String::as_str) == Some("SYN04B03")
        && metadata.get("case_id").map(String::as_str) == Some("SYN-GSI-01")
        && metadata.get("design_class").map(String::as_str) == Some("ASSUMED_FOR_EXECUTION")
        && metadata.get("hidden_candidate").map(String::as_str) == Some(TRUTH)
        && metadata.get("finite_challenger").map(String::as_str) == Some(FINITE_CHALLENGER)
        && metadata.get("finite_challenger_rule").map(String::as_str) == Some(CHALLENGER_RULE)
        && metadata.get("candidate_ids").map(String::as_str)
            == Some("GSI-0001|GSI-0064|GSI-5557|GSI-9261")
        && metadata.get("tmin_formula_c").map(String::as_str)
            == Some("-2.0+12.0*sin(2*pi*(ordinal_day-80)/365)")
        && metadata.get("vpd_formula_pa").map(String::as_str)
            == Some("600.0-350.0*sin(2*pi*(ordinal_day-100)/365)")
        && metadata.get("state_calendar_rule").map(String::as_str) == Some(STATE_RULE)
        && metadata.get("crossing_rule").map(String::as_str) == Some(CROSSING_RULE)
        && metadata.get("observation_operator").map(String::as_str) == Some(OBSERVATION_OPERATOR)
        && metadata.get("acceptance").map(String::as_str)
            == Some("minimum equal-year interval RMSE set")
        && metadata.get("required_result").map(String::as_str) == Some(REQUIRED_RESULT)
}

fn validate_crossing_shape(
    crossings: &HashMap<&str, HashMap<i32, Vec<u16>>>,
) -> Result<(), Box<dyn Error>> {
    for id in [TRUTH, FINITE_CHALLENGER] {
        let candidate = crossings
            .get(id)
            .ok_or("verification required candidate absent")?;
        if candidate.len() != 3 || candidate.values().any(|year| year.len() != 1) {
            return Err(format!("{id} verification crossing count invalid").into());
        }
    }
    for id in ["GSI-0001", "GSI-9261"] {
        let candidate = crossings
            .get(id)
            .ok_or("verification boundary candidate absent")?;
        if candidate.len() != 3 || candidate.values().any(|year| !year.is_empty()) {
            return Err(format!("{id} verification boundary behavior invalid").into());
        }
    }
    Ok(())
}

fn verification_crossings(
    values: &[f64],
    first_eligible: u16,
    last_eligible: u16,
) -> Result<HashMap<i32, Vec<u16>>, Box<dyn Error>> {
    let mut result = HashMap::new();
    let mut offset = 0_usize;
    for year in 2001..=2003 {
        let mut previous: Option<f64> = None;
        let mut eligible = Vec::new();
        for ordinal in 1..=days_in_year(year) {
            let current = values[offset];
            if !current.is_finite() || !(0.0..=1.0).contains(&current) {
                return Err("invalid verification GSI value".into());
            }
            if (first_eligible..=last_eligible).contains(&ordinal)
                && matches!(previous, Some(value) if value < 0.5)
                && current >= 0.5
            {
                eligible.push(ordinal);
            }
            previous = Some(current);
            offset += 1;
        }
        result.insert(year, eligible);
    }
    if offset != values.len() {
        return Err("verification daily trace length differs from calendar".into());
    }
    Ok(result)
}

fn validate_raw_shape(raw: &[u8]) -> Result<(usize, usize), Box<dyn Error>> {
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
    Ok((count, days))
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
    let design_path = PathBuf::from(arg_value(&argv, "--design")?);
    let primary = PathBuf::from(arg_value(&argv, "--primary")?);
    let output = PathBuf::from(arg_value(&argv, "--out")?);
    if output.exists() {
        return Err("synthetic verification output already exists".into());
    }
    fs::create_dir_all(&output)?;
    let metadata = fields(&identity_path)?;
    if !static_fields_match(&metadata)
        || metadata.get("design_sha256") != Some(&sha256(&design_path)?)
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
    let (_, days) = validate_raw_shape(&raw)?;

    let mut crossings: HashMap<&str, HashMap<i32, Vec<u16>>> = HashMap::new();
    for (candidate_index, id) in IDS.iter().enumerate() {
        let base = 20 + candidate_index * days * 8;
        let mut values = Vec::with_capacity(days);
        for day_offset in 0..days {
            let byte_offset = base + day_offset * 8;
            values.push(f64::from_le_bytes(
                raw[byte_offset..byte_offset + 8].try_into()?,
            ));
        }
        crossings.insert(id, verification_crossings(&values, 60, 180)?);
    }
    validate_crossing_shape(&crossings)?;
    let truth_crossings = crossings.get(TRUTH).ok_or("verification truth absent")?;
    let component_path = output.join("candidate-observation-components.csv");
    let annual_path = output.join("candidate-annual-components.csv");
    let crossing_counts_path = output.join("candidate-year-crossing-counts.csv");
    let ledger_path = output.join("candidate-ledger.csv");
    let membership_path = output.join("accepted-synthetic-ensemble.csv");
    let mut component_writer = BufWriter::new(File::create(&component_path)?);
    let mut annual_writer = BufWriter::new(File::create(&annual_path)?);
    let mut crossing_counts_writer = BufWriter::new(File::create(&crossing_counts_path)?);
    writeln!(component_writer, "candidate_id,record_id,year,crossing_doy,lower_doy,upper_doy,distance_days,squared_distance")?;
    writeln!(
        annual_writer,
        "candidate_id,year,crossing_doy,observation_count,annual_mse,annual_rmse"
    )?;
    writeln!(
        crossing_counts_writer,
        "candidate_id,year,eligible_crossing_count,first_crossing_doy"
    )?;
    let mut objective_by_id = BTreeMap::new();
    for id in IDS {
        let candidate = crossings.get(id).ok_or("verification candidate absent")?;
        let mut sum = 0.0_f64;
        let mut complete = true;
        for year in [2001, 2002, 2003] {
            let truth_day = i32::from(truth_crossings[&year][0]);
            let low = truth_day - 2;
            let high = truth_day + 2;
            let candidate_year = &candidate[&year];
            writeln!(
                crossing_counts_writer,
                "{id},{year},{},{}",
                candidate_year.len(),
                candidate_year
                    .first()
                    .map(u16::to_string)
                    .unwrap_or_default()
            )?;
            if let Some(crossing) = candidate_year.first().copied() {
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
    crossing_counts_writer.flush()?;
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
        || !objective_by_id[FINITE_CHALLENGER].is_finite()
        || objective_by_id[FINITE_CHALLENGER] <= 0.0
    {
        return Err("verification recovery or competitor evidence failed".into());
    }
    exact(
        &crossing_counts_path,
        &primary.join("candidate-year-crossing-counts.csv"),
        "synthetic crossing counts",
    )?;
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
            "field,value\nstate,PASS\nschema,SYN04B03\ncase_id,SYN-GSI-01\ndesign_sha256,{}\ntrace_sha256,{}\nhidden_candidate,GSI-5557\nhidden_objective,0.000000000000\nfinite_challenger,GSI-0064\nfinite_challenger_objective,{:.12}\nrecovered_set,{}\nnonvacuous_competitor,TRUE\nexact_primary_match,TRUE\ncrossing_counts_sha256,{}\ncomponents_sha256,{}\nannual_sha256,{}\ncandidate_ledger_sha256,{}\naccepted_ensemble_sha256,{}\n",
            sha256(&design_path)?,
            sha256(&trace_path)?,
            objective_by_id[FINITE_CHALLENGER],
            recovered.join("|"),
            sha256(&crossing_counts_path)?,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verification_identity_rejects_header_duplicates_and_old_schema() {
        assert!(fields_from_text("bad\nschema,SYN04B03\n").is_err());
        assert!(fields_from_text("field,value\nschema,SYN04B03\nschema,SYN04B02\n").is_err());
        let old = fields_from_text("field,value\nschema,SYN04B02\n")
            .expect("old identity syntax must parse");
        assert!(!static_fields_match(&old));
    }

    #[test]
    fn verification_shape_rejects_multiple_and_boundary_crossings() {
        let mut crossings = HashMap::new();
        for id in IDS {
            crossings.insert(
                id,
                (2001..=2003)
                    .map(|year| {
                        (
                            year,
                            if id == TRUTH || id == FINITE_CHALLENGER {
                                vec![146]
                            } else {
                                Vec::new()
                            },
                        )
                    })
                    .collect(),
            );
        }
        validate_crossing_shape(&crossings).expect("valid verification shape");
        crossings
            .get_mut(FINITE_CHALLENGER)
            .expect("challenger")
            .get_mut(&2001)
            .expect("year")
            .push(170);
        assert!(validate_crossing_shape(&crossings).is_err());
        crossings
            .get_mut(FINITE_CHALLENGER)
            .expect("challenger")
            .get_mut(&2001)
            .expect("year")
            .pop();
        crossings
            .get_mut("GSI-9261")
            .expect("boundary")
            .get_mut(&2003)
            .expect("year")
            .push(150);
        assert!(validate_crossing_shape(&crossings).is_err());
        crossings
            .get_mut("GSI-9261")
            .expect("boundary")
            .get_mut(&2003)
            .expect("year")
            .pop();
        crossings
            .get_mut(TRUTH)
            .expect("truth")
            .get_mut(&2002)
            .expect("year")
            .push(175);
        assert!(validate_crossing_shape(&crossings).is_err());
    }

    #[test]
    fn verification_crossing_memory_resets_at_year_boundary() {
        let days: usize = (2001..=2003)
            .map(|year| usize::from(days_in_year(year)))
            .sum();
        let mut values = vec![1.0; days];
        values[364] = 0.0;
        values[365] = 1.0;
        let crossings =
            verification_crossings(&values, 1, 180).expect("verification crossing scan");
        assert!(!crossings[&2002].contains(&1));
    }

    #[test]
    fn verification_raw_shape_rejects_old_schema_dimensions_and_trailing_bytes() {
        let days = 1095_usize;
        let mut raw = Vec::new();
        raw.extend_from_slice(&MAGIC);
        raw.extend_from_slice(&(IDS.len() as u32).to_le_bytes());
        raw.extend_from_slice(&2001_i32.to_le_bytes());
        raw.extend_from_slice(&(days as u32).to_le_bytes());
        raw.resize(20 + IDS.len() * days * 8, 0);
        validate_raw_shape(&raw).expect("valid raw dimensions");
        let mut old = raw.clone();
        old[..8].copy_from_slice(b"SYN04B02");
        assert!(validate_raw_shape(&old).is_err());
        let mut wrong_count = raw.clone();
        wrong_count[8..12].copy_from_slice(&3_u32.to_le_bytes());
        assert!(validate_raw_shape(&wrong_count).is_err());
        raw.push(0);
        assert!(validate_raw_shape(&raw).is_err());
    }
}
