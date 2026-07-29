use cal04b_executor::{arg_value, days_in_year, read_configs, sha256};
use openwepp_plant_phenology::{GsiDailyForcing, GsiDate, GsiParameters, GsiState};
use std::env;
use std::error::Error;
use std::f64::consts::PI;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

const SYNTHETIC_MAGIC: &[u8; 8] = b"SYN04B03";
const SELECTED: [&str; 4] = ["GSI-0001", "GSI-0064", "GSI-5557", "GSI-9261"];
const STATE_RULE: &str = "each candidate-year is a separate native GsiState cold start; admit ordinal days 1-365 in order; no synthetic prefill or cross-year carry";
const CROSSING_RULE: &str = "count every upward previous<0.5 and current>=0.5 crossing only on ordinal days 60-180; hidden truth requires exactly one eligible crossing per year; competitors use their first eligible crossing and retain the full count";
const CHALLENGER_RULE: &str = "lexicographically first non-hidden grid candidate with exactly one eligible crossing per year outside the hidden +/-2-day interval under the frozen corrected forcing";
const OBSERVATION_OPERATOR: &str = "for each of 3 complete calendar years: closed interval [hidden sole eligible spring crossing-2 days;hidden sole eligible spring crossing+2 days]";
const REQUIRED_RESULT: &str = "both independent reconstructors emit 4 candidate x 3 year completeness and crossing counts; identical components/counts/minimum set; GSI-5557 has exactly one eligible crossing per year, objective=0, and is included; GSI-0064 has exactly one eligible crossing per year and finite objective>0; boundary competitors retain missing-crossing failures";

fn new_year_state() -> GsiState {
    GsiState::new()
}

fn candidate_values(
    parameters: GsiParameters,
    forcing: impl Fn(i32, u16) -> GsiDailyForcing,
) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut values = Vec::new();
    for year in 2001..=2003 {
        let mut state = new_year_state();
        for ordinal_day in 1..=days_in_year(year) {
            values.push(
                state
                    .advance(parameters, forcing(year, ordinal_day))?
                    .growing_season_index,
            );
        }
    }
    Ok(values)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let configs_path = PathBuf::from(arg_value(&args, "--configs")?);
    let design_path = PathBuf::from(arg_value(&args, "--design")?);
    let trace_path = PathBuf::from(arg_value(&args, "--trace")?);
    let identity_path = PathBuf::from(arg_value(&args, "--identity")?);
    if trace_path.exists() || identity_path.exists() {
        return Err("synthetic trace or identity output already exists".into());
    }
    if let Some(parent) = trace_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let configs = read_configs(&configs_path)?;
    let day_count: usize = (2001..=2003)
        .map(|year| usize::from(days_in_year(year)))
        .sum();
    let file = File::create(&trace_path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(SYNTHETIC_MAGIC)?;
    writer.write_all(&(SELECTED.len() as u32).to_le_bytes())?;
    writer.write_all(&2001_i32.to_le_bytes())?;
    writer.write_all(&(day_count as u32).to_le_bytes())?;

    for id in SELECTED {
        let config = configs
            .iter()
            .find(|row| row.id == id)
            .ok_or_else(|| format!("synthetic candidate missing: {id}"))?;
        let values = config.values;
        let parameters = GsiParameters {
            minimum_temperature_inactive_c: values[0],
            minimum_temperature_unconstrained_c: values[1],
            vapor_pressure_deficit_unconstrained_pa: values[2],
            vapor_pressure_deficit_inactive_pa: values[3],
            photoperiod_inactive_hours: values[4],
            photoperiod_unconstrained_hours: values[5],
        };
        let emitted = candidate_values(parameters, |year, ordinal_day| {
            let temperature_angle = 2.0 * PI * (f64::from(ordinal_day) - 80.0) / 365.0;
            let vpd_angle = 2.0 * PI * (f64::from(ordinal_day) - 100.0) / 365.0;
            GsiDailyForcing {
                minimum_temperature_c: -2.0 + 12.0 * temperature_angle.sin(),
                vapor_pressure_deficit_pa: 600.0 - 350.0 * vpd_angle.sin(),
                latitude_degrees: 44.27,
                date: GsiDate { year, ordinal_day },
            }
        })?;
        for value in &emitted {
            writer.write_all(&value.to_le_bytes())?;
        }
        if emitted.len() != day_count {
            return Err(format!("{id} emitted {} != {day_count}", emitted.len()).into());
        }
    }
    writer.flush()?;
    let expected_bytes = 20_u64 + SELECTED.len() as u64 * day_count as u64 * 8;
    let observed_bytes = fs::metadata(&trace_path)?.len();
    if observed_bytes != expected_bytes {
        return Err(format!("trace bytes {observed_bytes} != {expected_bytes}").into());
    }
    let executable = env::current_exe()?;
    let identity = format!(
        "field,value\nschema,SYN04B03\ncase_id,SYN-GSI-01\ndesign_class,ASSUMED_FOR_EXECUTION\ndesign_path,{}\ndesign_sha256,{}\nhidden_candidate,GSI-5557\nfinite_challenger,GSI-0064\nfinite_challenger_rule,{CHALLENGER_RULE}\ncandidate_ids,GSI-0001|GSI-0064|GSI-5557|GSI-9261\ncandidate_count,4\nstart_date,2001-01-01\nend_date,2003-12-31\nday_count,{day_count}\nlatitude_degrees,44.27\ntmin_formula_c,-2.0+12.0*sin(2*pi*(ordinal_day-80)/365)\nvpd_formula_pa,600.0-350.0*sin(2*pi*(ordinal_day-100)/365)\nstate_calendar_rule,{STATE_RULE}\ncrossing_rule,{CROSSING_RULE}\nobservation_operator,{OBSERVATION_OPERATOR}\nacceptance,minimum equal-year interval RMSE set\nrequired_result,{REQUIRED_RESULT}\ntrace_path,{}\ntrace_bytes,{observed_bytes}\ntrace_sha256,{}\nconfig_path,{}\nconfig_sha256,{}\nproducer_binary,{}\nproducer_binary_sha256,{}\n",
        design_path.display(),
        sha256(&design_path)?,
        trace_path.display(),
        sha256(&trace_path)?,
        configs_path.display(),
        sha256(&configs_path)?,
        executable.display(),
        sha256(&executable)?,
    );
    fs::write(identity_path, identity)?;
    println!(
        "PASS synthetic producer candidates={} days={day_count}",
        SELECTED.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn forcing(year: i32, ordinal_day: u16, minimum_temperature_c: f64) -> GsiDailyForcing {
        GsiDailyForcing {
            minimum_temperature_c,
            vapor_pressure_deficit_pa: 0.0,
            latitude_degrees: 0.0,
            date: GsiDate { year, ordinal_day },
        }
    }

    #[test]
    fn producer_generation_cold_starts_each_year() {
        let parameters = GsiParameters::generalized();
        let values = candidate_values(parameters, |year, ordinal_day| {
            forcing(
                year,
                ordinal_day,
                if ordinal_day == 1 { -10.0 } else { 10.0 },
            )
        })
        .expect("test generation must pass");
        let year_two_offset = 365;
        assert_eq!(values[year_two_offset].to_bits(), 0.0_f64.to_bits());
        assert!(values[year_two_offset + 1] > 0.0);
    }
}
