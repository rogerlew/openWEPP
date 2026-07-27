use cal04b_executor::{
    arg_value, read_authenticated_daymet, read_configs, sha256, Climate, TraceHeader,
    CALIBRATION_DAYS_PER_LANE, CALIBRATION_DAYS_PER_YEAR, DAYMET_LANE_COUNT,
    DAYMET_SOURCE_DAYS_PER_LANE, DAYMET_SOURCE_DAYS_PER_YEAR,
};
use openwepp_plant_phenology::{GsiDailyForcing, GsiDate, GsiParameters, GsiState};
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

fn write_plot_year(
    parameters: GsiParameters,
    latitude_degrees: f64,
    rows: &[Climate],
    writer: &mut impl Write,
) -> Result<usize, Box<dyn Error>> {
    if rows.len() != CALIBRATION_DAYS_PER_YEAR {
        return Err("plot-year forcing does not contain yday 1 through 180".into());
    }
    let year = rows[0].year;
    let mut state = GsiState::new();
    for (index, row) in rows.iter().enumerate() {
        if row.year != year || row.ordinal != index as u16 + 1 {
            return Err("plot-year forcing order differs".into());
        }
        let result = state.advance(
            parameters,
            GsiDailyForcing {
                minimum_temperature_c: row.tmin,
                vapor_pressure_deficit_pa: row.vpd,
                latitude_degrees,
                date: GsiDate {
                    year: row.year,
                    ordinal_day: row.ordinal,
                },
            },
        )?;
        writer.write_all(&result.growing_season_index.to_le_bytes())?;
    }
    Ok(rows.len())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let configs_path = PathBuf::from(arg_value(&args, "--configs")?);
    let forcing_path = PathBuf::from(arg_value(&args, "--forcing")?);
    let geometry_path = PathBuf::from(arg_value(&args, "--geometry")?);
    let source_manifest_path = PathBuf::from(arg_value(&args, "--source-manifest")?);
    let authority_path = PathBuf::from(arg_value(&args, "--authority-manifest")?);
    let authority_resolution_path =
        PathBuf::from(arg_value(&args, "--forcing-authority-resolution")?);
    let trace_path = PathBuf::from(arg_value(&args, "--trace")?);
    let identity_path = PathBuf::from(arg_value(&args, "--identity")?);
    let failures_path = PathBuf::from(arg_value(&args, "--failures")?);
    let partial_trace_path = trace_path.with_extension("partial");
    let calendar_path = trace_path.with_extension("calendar.csv");
    let lane_manifest_path = trace_path.with_extension("lanes.csv");
    if let Some(parent) = trace_path.parent() {
        fs::create_dir_all(parent)?;
    }
    for path in [
        &trace_path,
        &identity_path,
        &failures_path,
        &partial_trace_path,
        &calendar_path,
        &lane_manifest_path,
    ] {
        if path.exists() {
            return Err(format!("producer output already exists: {}", path.display()).into());
        }
    }

    let configs = read_configs(&configs_path)?;
    let lanes = read_authenticated_daymet(
        &forcing_path,
        &geometry_path,
        &source_manifest_path,
        &authority_path,
        &authority_resolution_path,
    )?;
    if lanes.len() != DAYMET_LANE_COUNT {
        return Err("authenticated lane count mismatch".into());
    }
    let header = TraceHeader {
        candidate_count: configs.len(),
        lane_count: lanes.len(),
        days_per_lane: CALIBRATION_DAYS_PER_LANE,
    };

    let mut calendar = BufWriter::new(File::create(&calendar_path)?);
    writeln!(calendar, "lane_index,plot_id,year,yday")?;
    for lane in &lanes {
        for year_rows in lane.forcing.chunks_exact(DAYMET_SOURCE_DAYS_PER_YEAR) {
            for row in &year_rows[..CALIBRATION_DAYS_PER_YEAR] {
                writeln!(
                    calendar,
                    "{},{},{},{}",
                    lane.lane_index, lane.plot_id, row.year, row.ordinal
                )?;
            }
        }
    }
    calendar.flush()?;

    let mut lane_manifest = BufWriter::new(File::create(&lane_manifest_path)?);
    writeln!(
        lane_manifest,
        "lane_index,plot_id,latitude_degrees,longitude_degrees,first_year,last_year,source_days_per_year,source_day_count,retained_days_per_year,retained_day_count,forcing_source_path,forcing_source_sha256"
    )?;
    let forcing_hash = sha256(&forcing_path)?;
    for lane in &lanes {
        writeln!(
            lane_manifest,
            "{},{},{:.17},{:.17},1989,2024,{DAYMET_SOURCE_DAYS_PER_YEAR},{DAYMET_SOURCE_DAYS_PER_LANE},{CALIBRATION_DAYS_PER_YEAR},{CALIBRATION_DAYS_PER_LANE},{},{}",
            lane.lane_index,
            lane.plot_id,
            lane.latitude_degrees,
            lane.longitude_degrees,
            forcing_path.display(),
            forcing_hash
        )?;
    }
    lane_manifest.flush()?;

    let mut failure_writer = BufWriter::new(File::create(&failures_path)?);
    writeln!(
        failure_writer,
        "failure_id,candidate_id,lane_index,plot_id,year,failure_class,typed_error"
    )?;
    let mut writer = BufWriter::with_capacity(8 * 1024 * 1024, File::create(&partial_trace_path)?);
    header.write(&mut writer)?;
    let mut failure_serial = 0_u64;
    for config in &configs {
        let values = config.values;
        let parameters = GsiParameters {
            minimum_temperature_inactive_c: values[0],
            minimum_temperature_unconstrained_c: values[1],
            vapor_pressure_deficit_unconstrained_pa: values[2],
            vapor_pressure_deficit_inactive_pa: values[3],
            photoperiod_inactive_hours: values[4],
            photoperiod_unconstrained_hours: values[5],
        };
        for lane in &lanes {
            let mut emitted = 0_usize;
            for year_rows in lane.forcing.chunks_exact(DAYMET_SOURCE_DAYS_PER_YEAR) {
                let year = year_rows[0].year;
                match write_plot_year(
                    parameters,
                    lane.latitude_degrees,
                    &year_rows[..CALIBRATION_DAYS_PER_YEAR],
                    &mut writer,
                ) {
                    Ok(count) => emitted += count,
                    Err(error) => {
                        failure_serial += 1;
                        writeln!(
                            failure_writer,
                            "PRODUCER-FAIL-{failure_serial:06},{},{},{},{year},GSI_ADVANCE,{error}",
                            config.id, lane.lane_index, lane.plot_id
                        )?;
                        failure_writer.flush()?;
                        writer.flush()?;
                        drop(writer);
                        fs::remove_file(&partial_trace_path)?;
                        return Err(
                            format!("{}/{}/{year}: {error}", config.id, lane.plot_id).into()
                        );
                    }
                }
            }
            if emitted != CALIBRATION_DAYS_PER_LANE {
                return Err(format!(
                    "{} plot {} emitted {emitted} != {CALIBRATION_DAYS_PER_LANE}",
                    config.id, lane.plot_id
                )
                .into());
            }
        }
    }
    writer.flush()?;
    drop(writer);
    failure_writer.flush()?;
    let observed_bytes = fs::metadata(&partial_trace_path)?.len();
    let expected_bytes = header.expected_bytes()?;
    if observed_bytes != expected_bytes {
        return Err(format!("trace bytes {observed_bytes} != {expected_bytes}").into());
    }
    fs::rename(&partial_trace_path, &trace_path)?;

    let executable = env::current_exe()?;
    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/bin/native_producer.rs");
    let identity = format!(
        "field,value\nschema,CAL04B03\nsite_id,hubbard_brook\narm_id,deciduous\ncandidate_count,{}\nlane_count,{}\ndays_per_lane,{CALIBRATION_DAYS_PER_LANE}\nsource_days_per_plot_year,{DAYMET_SOURCE_DAYS_PER_YEAR}\nretained_days_per_plot_year,{CALIBRATION_DAYS_PER_YEAR}\nfirst_year,1989\nlast_year,2024\nstate_initialization,FRESH_GSI_STATE_EACH_CANDIDATE_PLOT_YEAR\ncrossing_eligibility_yday,60-180\ntrace_order,candidate_lane_year_yday\ntrace_path,{}\ntrace_bytes,{observed_bytes}\ntrace_sha256,{}\ncalendar_path,{}\ncalendar_sha256,{}\nlane_manifest_path,{}\nlane_manifest_sha256,{}\nconfig_path,{}\nconfig_sha256,{}\nforcing_path,{}\nforcing_sha256,{}\ngeometry_path,{}\ngeometry_sha256,{}\nsource_manifest_path,{}\nsource_manifest_sha256,{}\nauthority_manifest_path,{}\nauthority_manifest_sha256,{}\nforcing_authority_resolution_path,{}\nforcing_authority_resolution_sha256,{}\nproducer_source,{}\nproducer_source_sha256,{}\nproducer_binary,{}\nproducer_binary_sha256,{}\nexact_command,{}\nfailure_ledger,{}\nfailure_ledger_sha256,{}\n",
        configs.len(),
        lanes.len(),
        trace_path.display(),
        sha256(&trace_path)?,
        calendar_path.display(),
        sha256(&calendar_path)?,
        lane_manifest_path.display(),
        sha256(&lane_manifest_path)?,
        configs_path.display(),
        sha256(&configs_path)?,
        forcing_path.display(),
        forcing_hash,
        geometry_path.display(),
        sha256(&geometry_path)?,
        source_manifest_path.display(),
        sha256(&source_manifest_path)?,
        authority_path.display(),
        sha256(&authority_path)?,
        authority_resolution_path.display(),
        sha256(&authority_resolution_path)?,
        source.display(),
        sha256(&source)?,
        executable.display(),
        sha256(&executable)?,
        args.join(" "),
        failures_path.display(),
        sha256(&failures_path)?,
    );
    fs::write(identity_path, identity)?;
    println!(
        "PASS candidates={} lanes={} days_per_lane={} bytes={observed_bytes}",
        configs.len(),
        lanes.len(),
        CALIBRATION_DAYS_PER_LANE
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::write_plot_year;
    use cal04b_executor::{Climate, CALIBRATION_DAYS_PER_YEAR};
    use openwepp_plant_phenology::GsiParameters;

    #[test]
    fn plot_year_writer_cold_starts_every_invocation() {
        let forcing = (1..=CALIBRATION_DAYS_PER_YEAR)
            .map(|ordinal| Climate {
                year: 2001,
                ordinal: ordinal as u16,
                tmin: -5.0 + ordinal as f64 * 0.08,
                vpd: 500.0,
            })
            .collect::<Vec<_>>();
        let mut first = Vec::new();
        let mut second = Vec::new();
        write_plot_year(GsiParameters::generalized(), 43.95, &forcing, &mut first)
            .expect("first cold-start year");
        write_plot_year(GsiParameters::generalized(), 43.95, &forcing, &mut second)
            .expect("second cold-start year");
        assert_eq!(first, second);
        assert_eq!(first.len(), CALIBRATION_DAYS_PER_YEAR * 8);
    }
}
