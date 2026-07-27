use cal04b_executor::{arg_value, days_in_year, read_configs, sha256};
use openwepp_plant_phenology::{GsiDailyForcing, GsiDate, GsiParameters, GsiState};
use std::env;
use std::error::Error;
use std::f64::consts::PI;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

const SYNTHETIC_MAGIC: &[u8; 8] = b"SYN04B02";
const SELECTED: [&str; 3] = ["GSI-0001", "GSI-5557", "GSI-9261"];

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let configs_path = PathBuf::from(arg_value(&args, "--configs")?);
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
        let mut state = GsiState::new();
        let mut emitted = 0_usize;
        for year in 2001..=2003 {
            for ordinal_day in 1..=days_in_year(year) {
                let temperature_angle = 2.0 * PI * (f64::from(ordinal_day) - 80.0) / 365.0;
                let vpd_angle = 2.0 * PI * (f64::from(ordinal_day) - 100.0) / 365.0;
                let result = state.advance(
                    parameters,
                    GsiDailyForcing {
                        minimum_temperature_c: -2.0 + 12.0 * temperature_angle.sin(),
                        vapor_pressure_deficit_pa: 600.0 + 350.0 * vpd_angle.sin(),
                        latitude_degrees: 44.27,
                        date: GsiDate { year, ordinal_day },
                    },
                )?;
                writer.write_all(&result.growing_season_index.to_le_bytes())?;
                emitted += 1;
            }
        }
        if emitted != day_count {
            return Err(format!("{id} emitted {emitted} != {day_count}").into());
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
        "field,value\nschema,SYN04B02\ncase_id,SYN-GSI-01\nhidden_candidate,GSI-5557\ncandidate_ids,GSI-0001|GSI-5557|GSI-9261\ncandidate_count,3\nstart_date,2001-01-01\nend_date,2003-12-31\nday_count,{day_count}\nlatitude_degrees,44.27\ntmin_formula_c,-2.0+12.0*sin(2*pi*(ordinal_day-80)/365)\nvpd_formula_pa,600.0+350.0*sin(2*pi*(ordinal_day-100)/365)\ntrace_path,{}\ntrace_bytes,{observed_bytes}\ntrace_sha256,{}\nconfig_path,{}\nconfig_sha256,{}\nproducer_binary,{}\nproducer_binary_sha256,{}\n",
        trace_path.display(),
        sha256(&trace_path)?,
        configs_path.display(),
        sha256(&configs_path)?,
        executable.display(),
        sha256(&executable)?,
    );
    fs::write(identity_path, identity)?;
    println!("PASS synthetic producer candidates=3 days={day_count}");
    Ok(())
}
