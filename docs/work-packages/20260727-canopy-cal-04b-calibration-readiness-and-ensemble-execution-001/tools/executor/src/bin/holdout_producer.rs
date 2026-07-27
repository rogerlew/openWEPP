use cal04b_executor::{
    arg_value, read_climate, read_configs, require_calendar_extent, sha256, write_calendar,
    HOLDOUT_MAGIC,
};
use openwepp_plant_phenology::{GsiDailyForcing, GsiDate, GsiParameters, GsiState};
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

fn accepted(path: &PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    let mut ids = Vec::new();
    let mut unique = HashSet::new();
    for (index, line) in BufReader::new(File::open(path)?).lines().enumerate() {
        if index == 0 {
            continue;
        }
        let line = line?;
        let id = line
            .split(',')
            .next()
            .ok_or("bad accepted row")?
            .to_string();
        if !unique.insert(id.clone()) {
            return Err(format!("duplicate accepted candidate {id}").into());
        }
        ids.push(id);
    }
    if ids.is_empty() {
        return Err("empty accepted set".into());
    }
    if !ids.windows(2).all(|pair| pair[0] < pair[1]) {
        return Err("accepted candidate lanes are not in canonical ascending order".into());
    }
    Ok(ids)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let configs_path = PathBuf::from(arg_value(&args, "--configs")?);
    let accepted_path = PathBuf::from(arg_value(&args, "--accepted")?);
    let climate_path = PathBuf::from(arg_value(&args, "--climate")?);
    let trace_path = PathBuf::from(arg_value(&args, "--trace")?);
    let identity_path = PathBuf::from(arg_value(&args, "--identity")?);
    let calendar_path = trace_path.with_extension("calendar.csv");
    let partial_trace_path = trace_path.with_extension("partial");
    if trace_path.exists()
        || identity_path.exists()
        || calendar_path.exists()
        || partial_trace_path.exists()
    {
        return Err("holdout producer output already exists".into());
    }
    let configs = read_configs(&configs_path)?;
    let accepted = accepted(&accepted_path)?;
    let accepted_set: HashSet<_> = accepted.iter().cloned().collect();
    let selected: Vec<_> = configs
        .iter()
        .filter(|config| accepted_set.contains(&config.id))
        .collect();
    if selected.len() != accepted.len() {
        return Err("accepted configuration join incomplete".into());
    }
    if selected
        .iter()
        .map(|config| config.id.as_str())
        .ne(accepted.iter().map(String::as_str))
    {
        return Err("accepted lane order differs from canonical configuration order".into());
    }
    let (latitude, climate) = read_climate(&climate_path)?;
    let retained: Vec<_> = climate
        .iter()
        .copied()
        .filter(|row| (1991..=2023).contains(&row.year))
        .collect();
    require_calendar_extent(&retained, (1991, 1), (2023, 365))?;
    let warmup = climate
        .iter()
        .position(|row| row.year == 1990 && row.ordinal == 335)
        .ok_or("missing holdout warmup")?;
    let end = climate
        .iter()
        .position(|row| row.year == 2024 && row.ordinal == 1)
        .unwrap_or(climate.len());
    write_calendar(&calendar_path, &retained)?;
    let mut writer = BufWriter::with_capacity(8 * 1024 * 1024, File::create(&partial_trace_path)?);
    writer.write_all(HOLDOUT_MAGIC)?;
    writer.write_all(&(selected.len() as u32).to_le_bytes())?;
    writer.write_all(&1991_i32.to_le_bytes())?;
    writer.write_all(&(retained.len() as u32).to_le_bytes())?;
    for config in &selected {
        let v = config.values;
        let parameters = GsiParameters {
            minimum_temperature_inactive_c: v[0],
            minimum_temperature_unconstrained_c: v[1],
            vapor_pressure_deficit_unconstrained_pa: v[2],
            vapor_pressure_deficit_inactive_pa: v[3],
            photoperiod_inactive_hours: v[4],
            photoperiod_unconstrained_hours: v[5],
        };
        let mut state = GsiState::new();
        let mut emitted = 0;
        for row in &climate[warmup..end] {
            let daily = state.advance(
                parameters,
                GsiDailyForcing {
                    minimum_temperature_c: row.tmin,
                    vapor_pressure_deficit_pa: row.vpd,
                    latitude_degrees: latitude,
                    date: GsiDate {
                        year: row.year,
                        ordinal_day: row.ordinal,
                    },
                },
            )?;
            if row.year >= 1991 {
                writer.write_all(&daily.growing_season_index.to_le_bytes())?;
                emitted += 1;
            }
        }
        if emitted != retained.len() {
            return Err("holdout retained count mismatch".into());
        }
    }
    writer.flush()?;
    drop(writer);
    let expected_bytes = 20_u64 + selected.len() as u64 * retained.len() as u64 * 8;
    let trace_bytes = fs::metadata(&partial_trace_path)?.len();
    if trace_bytes != expected_bytes {
        return Err(format!("holdout trace bytes {trace_bytes} != {expected_bytes}").into());
    }
    fs::rename(&partial_trace_path, &trace_path)?;
    let executable = env::current_exe()?;
    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/bin/holdout_producer.rs");
    let identity = format!(
        "field,value\nschema,CAL04B02\nsite_id,harvard\narm_id,deciduous\nlane_count,1\nlane_index,0\ncandidate_count,{}\nretained_start,1991-001\nretained_end,2023-365\nretained_day_count,{}\nlatitude_degrees,{latitude:.17}\ntrace_path,{}\ntrace_bytes,{trace_bytes}\ntrace_sha256,{}\ncalendar_path,{}\ncalendar_sha256,{}\nconfig_path,{}\nconfig_sha256,{}\naccepted_path,{}\naccepted_sha256,{}\nclimate_path,{}\nclimate_sha256,{}\nproducer_source,{}\nproducer_source_sha256,{}\nproducer_binary,{}\nproducer_binary_sha256,{}\nexact_command,{}\n",
        accepted.len(), retained.len(), trace_path.display(), sha256(&trace_path)?,
        calendar_path.display(), sha256(&calendar_path)?, configs_path.display(),
        sha256(&configs_path)?, accepted_path.display(), sha256(&accepted_path)?,
        climate_path.display(), sha256(&climate_path)?, source.display(), sha256(&source)?,
        executable.display(), sha256(&executable)?, args.join(" "),
    );
    fs::write(identity_path, identity)?;
    println!(
        "PASS holdout candidates={} days={}",
        accepted.len(),
        retained.len()
    );
    Ok(())
}
