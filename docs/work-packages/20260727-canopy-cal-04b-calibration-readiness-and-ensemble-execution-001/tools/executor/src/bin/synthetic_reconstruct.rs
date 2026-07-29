use cal04b_executor::{arg_value, days_in_year, read_configs, sha256};
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

const SYNTHETIC_MAGIC: &[u8; 8] = b"SYN04B03";
const CANDIDATES: [&str; 4] = ["GSI-0001", "GSI-0064", "GSI-5557", "GSI-9261"];
const HIDDEN: &str = "GSI-5557";
const FINITE_CHALLENGER: &str = "GSI-0064";
const CHALLENGER_RULE: &str = "lexicographically first non-hidden grid candidate with exactly one eligible crossing per year outside the hidden +/-2-day interval under the frozen corrected forcing";
const STATE_RULE: &str = "each candidate-year is a separate native GsiState cold start; admit ordinal days 1-365 in order; no synthetic prefill or cross-year carry";
const CROSSING_RULE: &str = "count every upward previous<0.5 and current>=0.5 crossing only on ordinal days 60-180; hidden truth requires exactly one eligible crossing per year; competitors use their first eligible crossing and retain the full count";
const OBSERVATION_OPERATOR: &str = "for each of 3 complete calendar years: closed interval [hidden sole eligible spring crossing-2 days;hidden sole eligible spring crossing+2 days]";
const REQUIRED_RESULT: &str = "both independent reconstructors emit 4 candidate x 3 year completeness and crossing counts; identical components/counts/minimum set; GSI-5557 has exactly one eligible crossing per year, objective=0, and is included; GSI-0064 has exactly one eligible crossing per year and finite objective>0; boundary competitors retain missing-crossing failures";

fn identity(path: &Path) -> Result<HashMap<String, String>, Box<dyn Error>> {
    identity_from_text(&fs::read_to_string(path)?)
}

fn identity_from_text(text: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut fields = HashMap::new();
    let mut lines = text.lines();
    if lines.next() != Some("field,value") {
        return Err("synthetic identity header mismatch".into());
    }
    for (number, line) in lines.enumerate() {
        let (key, value) = line
            .split_once(',')
            .ok_or_else(|| format!("malformed identity row {}", number + 2))?;
        if key.is_empty() || fields.insert(key.to_string(), value.to_string()).is_some() {
            return Err(format!("empty or duplicate identity field at row {}", number + 2).into());
        }
    }
    Ok(fields)
}

fn static_identity_matches(metadata: &HashMap<String, String>) -> bool {
    metadata.get("schema").map(String::as_str) == Some("SYN04B03")
        && metadata.get("case_id").map(String::as_str) == Some("SYN-GSI-01")
        && metadata.get("design_class").map(String::as_str) == Some("ASSUMED_FOR_EXECUTION")
        && metadata.get("hidden_candidate").map(String::as_str) == Some(HIDDEN)
        && metadata.get("finite_challenger").map(String::as_str) == Some(FINITE_CHALLENGER)
        && metadata.get("finite_challenger_rule").map(String::as_str) == Some(CHALLENGER_RULE)
        && metadata.get("candidate_ids").map(String::as_str) == Some(&CANDIDATES.join("|"))
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

fn crossings_in_window(
    values: &[f64],
    first_eligible: u16,
    last_eligible: u16,
) -> Result<BTreeMap<i32, Vec<u16>>, Box<dyn Error>> {
    let mut result = BTreeMap::new();
    let mut offset = 0_usize;
    for year in 2001..=2003 {
        let mut previous: Option<f64> = None;
        let mut year_crossings = Vec::new();
        for day in 1..=days_in_year(year) {
            let current = values[offset];
            if !current.is_finite() || !(0.0..=1.0).contains(&current) {
                return Err(format!("invalid GSI at {year}-{day}").into());
            }
            if (first_eligible..=last_eligible).contains(&day)
                && previous.is_some_and(|prior| prior < 0.5)
                && current >= 0.5
            {
                year_crossings.push(day);
            }
            previous = Some(current);
            offset += 1;
        }
        result.insert(year, year_crossings);
    }
    if offset != values.len() {
        return Err("daily trace length differs from calendar".into());
    }
    Ok(result)
}

fn eligible_crossings(values: &[f64]) -> Result<BTreeMap<i32, Vec<u16>>, Box<dyn Error>> {
    crossings_in_window(values, 60, 180)
}

fn validate_trace_header(
    magic: &[u8; 8],
    candidate_count: usize,
    start_year: i32,
    day_count: usize,
    expected_days: usize,
) -> Result<(), Box<dyn Error>> {
    if magic != SYNTHETIC_MAGIC
        || candidate_count != CANDIDATES.len()
        || start_year != 2001
        || day_count != expected_days
    {
        return Err("synthetic trace header mismatch".into());
    }
    Ok(())
}

fn reject_trailing(reader: &mut impl Read) -> Result<(), Box<dyn Error>> {
    let mut extra = [0_u8; 1];
    if reader.read(&mut extra)? != 0 {
        return Err("synthetic trace has trailing bytes".into());
    }
    Ok(())
}

fn validate_crossing_shape(
    crossings: &BTreeMap<&str, BTreeMap<i32, Vec<u16>>>,
) -> Result<(), Box<dyn Error>> {
    for id in [HIDDEN, FINITE_CHALLENGER] {
        let candidate = crossings
            .get(id)
            .ok_or("required crossing candidate absent")?;
        if candidate.len() != 3 || candidate.values().any(|year| year.len() != 1) {
            return Err(
                format!("{id} does not have exactly one eligible crossing per year").into(),
            );
        }
    }
    for id in ["GSI-0001", "GSI-9261"] {
        let candidate = crossings
            .get(id)
            .ok_or("boundary crossing candidate absent")?;
        if candidate.len() != 3 || candidate.values().any(|year| !year.is_empty()) {
            return Err(format!("{id} does not retain missing-crossing boundary behavior").into());
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let trace_path = PathBuf::from(arg_value(&args, "--trace")?);
    let identity_path = PathBuf::from(arg_value(&args, "--identity")?);
    let configs_path = PathBuf::from(arg_value(&args, "--configs")?);
    let design_path = PathBuf::from(arg_value(&args, "--design")?);
    let output = PathBuf::from(arg_value(&args, "--out")?);
    if output.exists() {
        return Err("synthetic primary output already exists".into());
    }
    fs::create_dir_all(&output)?;
    let metadata = identity(&identity_path)?;
    if !static_identity_matches(&metadata)
        || metadata.get("design_sha256") != Some(&sha256(&design_path)?)
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
    validate_trace_header(
        &magic,
        candidate_count,
        start_year,
        day_count,
        expected_days,
    )?;
    let mut crossings = BTreeMap::new();
    let mut bytes = [0_u8; 8];
    for id in CANDIDATES {
        let mut values = Vec::with_capacity(day_count);
        for _ in 0..day_count {
            reader.read_exact(&mut bytes)?;
            values.push(f64::from_le_bytes(bytes));
        }
        crossings.insert(id, eligible_crossings(&values)?);
    }
    reject_trailing(&mut reader)?;
    validate_crossing_shape(&crossings)?;
    let hidden = crossings.get(HIDDEN).ok_or("hidden crossings absent")?;

    let components_path = output.join("candidate-observation-components.csv");
    let annual_path = output.join("candidate-annual-components.csv");
    let crossing_counts_path = output.join("candidate-year-crossing-counts.csv");
    let candidate_path = output.join("candidate-ledger.csv");
    let accepted_path = output.join("accepted-synthetic-ensemble.csv");
    let mut components = BufWriter::new(File::create(&components_path)?);
    let mut annual = BufWriter::new(File::create(&annual_path)?);
    let mut crossing_counts = BufWriter::new(File::create(&crossing_counts_path)?);
    writeln!(components, "candidate_id,record_id,year,crossing_doy,lower_doy,upper_doy,distance_days,squared_distance")?;
    writeln!(
        annual,
        "candidate_id,year,crossing_doy,observation_count,annual_mse,annual_rmse"
    )?;
    writeln!(
        crossing_counts,
        "candidate_id,year,eligible_crossing_count,first_crossing_doy"
    )?;
    let mut objectives = BTreeMap::new();
    for id in CANDIDATES {
        let candidate_crossings = crossings.get(id).ok_or("candidate crossings absent")?;
        let mut annual_mse = Vec::new();
        let mut complete = true;
        for year in 2001..=2003 {
            let hidden_year = hidden.get(&year).ok_or("hidden crossing year absent")?;
            let hidden_crossing = i32::from(hidden_year[0]);
            let lower = hidden_crossing - 2;
            let upper = hidden_crossing + 2;
            let candidate_year = candidate_crossings
                .get(&year)
                .ok_or("candidate crossing year absent")?;
            writeln!(
                crossing_counts,
                "{id},{year},{},{}",
                candidate_year.len(),
                candidate_year
                    .first()
                    .map(u16::to_string)
                    .unwrap_or_default()
            )?;
            if let Some(crossing) = candidate_year.first().copied() {
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
    crossing_counts.flush()?;
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
        || !objectives[FINITE_CHALLENGER].is_finite()
        || objectives[FINITE_CHALLENGER] <= 0.0
    {
        return Err("synthetic exact recovery or non-vacuity failed".into());
    }
    let receipt = output.join("primary-reconstruction-receipt.csv");
    fs::write(
        &receipt,
        format!(
            "field,value\nstate,PASS\nschema,SYN04B03\ncase_id,SYN-GSI-01\ndesign_sha256,{}\ntrace_sha256,{}\nhidden_candidate,GSI-5557\nhidden_objective,0.000000000000\nfinite_challenger,GSI-0064\nfinite_challenger_objective,{:.12}\nrecovered_set,{}\nnonvacuous_competitor,TRUE\ncrossing_counts_sha256,{}\ncomponents_sha256,{}\nannual_sha256,{}\ncandidate_ledger_sha256,{}\naccepted_ensemble_sha256,{}\n",
            sha256(&design_path)?,
            sha256(&trace_path)?,
            objectives[FINITE_CHALLENGER],
            members.join("|"),
            sha256(&crossing_counts_path)?,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_rejects_bad_header_and_duplicate_fields() {
        assert!(identity_from_text("wrong\nschema,SYN04B03\n").is_err());
        assert!(identity_from_text("field,value\nschema,SYN04B03\nschema,SYN04B02\n").is_err());
    }

    #[test]
    fn old_schema_is_not_current_semantic_identity() {
        let fields = identity_from_text("field,value\nschema,SYN04B02\n")
            .expect("identity syntax must parse");
        assert!(!static_identity_matches(&fields));
    }

    #[test]
    fn crossings_reset_per_year_and_count_every_eligible_transition() {
        let days: usize = (2001..=2003)
            .map(|year| usize::from(days_in_year(year)))
            .sum();
        let mut values = vec![0.0; days];
        let mut offset = 0;
        for year in 2001..=2003 {
            values[offset + 59] = 1.0;
            values[offset + 60] = 0.0;
            values[offset + 61] = 1.0;
            offset += usize::from(days_in_year(year));
        }
        let crossings = eligible_crossings(&values).expect("crossing scan must pass");
        assert!(crossings.values().all(|year| year == &[60, 62]));
    }

    #[test]
    fn crossing_memory_does_not_cross_year_boundary() {
        let days: usize = (2001..=2003)
            .map(|year| usize::from(days_in_year(year)))
            .sum();
        let mut values = vec![1.0; days];
        values[364] = 0.0;
        values[365] = 1.0;
        let crossings = crossings_in_window(&values, 1, 180).expect("crossing scan must pass");
        assert!(!crossings[&2002].contains(&1));
    }

    #[test]
    fn header_and_trailing_guards_reject_old_or_extra_data() {
        assert!(validate_trace_header(b"SYN04B02", 4, 2001, 1095, 1095).is_err());
        assert!(validate_trace_header(SYNTHETIC_MAGIC, 3, 2001, 1095, 1095).is_err());
        let mut clean = std::io::Cursor::new(Vec::<u8>::new());
        reject_trailing(&mut clean).expect("empty remainder");
        let mut dirty = std::io::Cursor::new(vec![1_u8]);
        assert!(reject_trailing(&mut dirty).is_err());
    }

    #[test]
    fn fixture_shape_rejects_challenger_and_boundary_mutations() {
        let mut crossings = BTreeMap::new();
        for id in CANDIDATES {
            crossings.insert(
                id,
                (2001..=2003)
                    .map(|year| {
                        (
                            year,
                            if id == HIDDEN || id == FINITE_CHALLENGER {
                                vec![146]
                            } else {
                                Vec::new()
                            },
                        )
                    })
                    .collect(),
            );
        }
        validate_crossing_shape(&crossings).expect("valid fixture shape");
        crossings
            .get_mut(FINITE_CHALLENGER)
            .expect("challenger")
            .get_mut(&2002)
            .expect("year")
            .push(170);
        assert!(validate_crossing_shape(&crossings).is_err());
        crossings
            .get_mut(FINITE_CHALLENGER)
            .expect("challenger")
            .get_mut(&2002)
            .expect("year")
            .pop();
        crossings
            .get_mut("GSI-0001")
            .expect("boundary")
            .get_mut(&2001)
            .expect("year")
            .push(150);
        assert!(validate_crossing_shape(&crossings).is_err());
        crossings
            .get_mut("GSI-0001")
            .expect("boundary")
            .get_mut(&2001)
            .expect("year")
            .pop();
        crossings
            .get_mut(HIDDEN)
            .expect("hidden")
            .get_mut(&2003)
            .expect("year")
            .push(175);
        assert!(validate_crossing_shape(&crossings).is_err());
    }
}
