use openwepp_hillslope_orchestrator::{
    DirectDayFrame, DirectDecompositionAction, DirectDecompositionActiveContext,
    DirectDecompositionInputs, DirectResiduePartitionInputs, DirectRunIdentity,
};
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const YEARS: usize = 20;
const DAYS: usize = 365;
const SOURCE_DAY: usize = 280;

#[derive(Clone)]
struct Candidate {
    id: String,
    source: f64,
    rate: f64,
    role: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        return Err("usage: executor DESIGN_CSV PRODUCER_CSV FAILURE_CSV".into());
    }
    let candidates = read_design(Path::new(&args[1]))?;
    run_candidates(&candidates, Path::new(&args[2]))?;
    run_failures(Path::new(&args[3]))?;
    Ok(())
}

fn read_design(path: &Path) -> Result<Vec<Candidate>, Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string(path)?;
    let mut rows = Vec::new();
    for line in text.lines().skip(1).filter(|line| !line.is_empty()) {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() != 5 {
            return Err(format!("invalid design row: {line}").into());
        }
        rows.push(Candidate {
            id: format!("{}-{}", fields[0], fields[2]),
            source: fields[1].parse()?,
            rate: fields[3].parse()?,
            role: fields[4].to_string(),
        });
    }
    Ok(rows)
}

fn seeded_day(day_index: usize) -> Result<DirectDayFrame, Box<dyn std::error::Error>> {
    let identity = DirectRunIdentity::new(5, 20260728, 1, YEARS * DAYS)?;
    let mut day = DirectDayFrame::seed(identity, 0, day_index)?;
    day.forcing.precipitation_m = 0.004;
    day.water.soil_water_m = 1.0;
    day.storage_reconciliation_inputs.closure_tolerance_m = 1.0e-12;
    day.run_r5b_normalization_phase()?;
    day.run_r5b_storage_bounds_phase()?;
    Ok(day)
}

fn base_inputs(
    runtime_day: u16,
    stock: f64,
    interrill: f64,
    rill: f64,
    source: f64,
    rate: f64,
) -> DirectDecompositionInputs {
    DirectDecompositionInputs {
        active_context: DirectDecompositionActiveContext::Perennial {
            active_slot_index: 1,
            active_crop_slot_index: 1,
            runtime_day_of_year: runtime_day,
        },
        active_action: DirectDecompositionAction::None,
        residue_type_selector: 0.0,
        surface_residue_seed_kg_m2: stock,
        interrill_ground_seed_kg_m2: interrill,
        rill_ground_seed_kg_m2: rill,
        residue_cover_factor: 0.0,
        root_residue_seed_kg_m2: 0.0,
        surface_litter_input_kg_m2: source,
        residue_depth_conversion_m_per_kg_m2: 0.0,
        temperature_max_c: 20.0,
        temperature_min_c: 10.0,
        precipitation_m: 0.004,
        water_stress_fraction: 1.0,
        surface_decomposition_rate: rate,
        root_decomposition_rate: 0.0,
        burn_surface_fraction: 0.0,
        remove_surface_fraction: 0.0,
        cut_transfer_fraction: 0.0,
        grazing_digest_fraction: 0.0,
    }
}

fn run_candidates(
    candidates: &[Candidate],
    output: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = BufWriter::new(File::create(output)?);
    writeln!(writer, "candidate_id,role,year,day,frame_day_index,surface_seed_kg_m2,interrill_seed_kg_m2,rill_seed_kg_m2,source_kg_m2,rate_d-1,tmax_c,tmin_c,precipitation_m,water_stress_fraction,temperature_factor,surface_water_factor,flat_water_factor,environment_index,decay_factor,surface_after_kg_m2,interrill_after_kg_m2,rill_after_kg_m2,root_after_kg_m2,residue_depth_m,downstream_surface_kg_m2,downstream_environment_index,downstream_decay_factor,partition_flat_kg_m2,partition_total_kg_m2")?;
    for candidate in candidates {
        let mut stock = 0.2;
        let mut interrill = 0.0;
        let mut rill = 0.0;
        for year in 1..=YEARS {
            for day_number in 1..=DAYS {
                let source = if day_number == SOURCE_DAY {
                    candidate.source
                } else {
                    0.0
                };
                let index = (year - 1) * DAYS + day_number - 1;
                let mut day = seeded_day(index)?;
                day.decomposition_inputs = base_inputs(
                    day_number as u16,
                    stock,
                    interrill,
                    rill,
                    source,
                    candidate.rate,
                );
                day.residue_partition_inputs = DirectResiduePartitionInputs::zero();
                day.run_r5c_decomposition_phase()?;
                day.run_r5c_residue_partition_phase()?;
                let state = day.decomposition;
                writeln!(
                    writer,
                    "{},{},{},{},{},{:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{:.17},{:.17}",
                    candidate.id,
                    candidate.role,
                    year,
                    day_number,
                    day.day_index,
                    stock,
                    interrill,
                    rill,
                    source,
                    candidate.rate,
                    20.0,
                    10.0,
                    0.004,
                    1.0,
                    state.temperature_factor,
                    state.surface_water_factor,
                    state.flat_water_factor,
                    state.environment_index,
                    state.surface_decay_factor,
                    state.surface_residue_kg_m2,
                    state.interrill_ground_residue_kg_m2,
                    state.rill_ground_residue_kg_m2,
                    state.root_residue_kg_m2,
                    state.residue_depth_m,
                    day.decomposition_downstream_operands.surface_residue_kg_m2,
                    day.decomposition_downstream_operands.environment_index,
                    day.decomposition_downstream_operands.surface_decay_factor,
                    day.residue_partition_downstream_operands.flat_residue_kg_m2,
                    day.residue_partition_downstream_operands.total_residue_kg_m2,
                )?;
                stock = state.surface_residue_kg_m2;
                interrill = state.interrill_ground_residue_kg_m2;
                rill = state.rill_ground_residue_kg_m2;
            }
        }
    }
    Ok(())
}

fn run_failures(output: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = BufWriter::new(File::create(output)?);
    writeln!(writer, "case_id,state,error")?;
    let cases = [
        ("ZERO-SOURCE", "source", 0.0),
        ("ZERO-STOCK", "stock", 0.0),
        ("ZERO-RATE", "rate", 0.0),
        ("NEG-SOURCE", "source", -0.1),
        ("NEG-STOCK", "stock", -0.1),
        ("NEG-RATE", "rate", -0.1),
        ("NEG-PRECIP", "precip", -0.1),
        ("NAN-SOURCE", "source", f64::NAN),
        ("POSINF-STOCK", "stock", f64::INFINITY),
        ("NEGINF-RATE", "rate", f64::NEG_INFINITY),
        ("STRESS-HIGH", "stress", 1.1),
        ("STRESS-LOW", "stress", -0.1),
    ];
    for (id, field, value) in cases {
        let mut input = base_inputs(1, 0.2, 0.0, 0.0, 0.1, 0.5 / 365.25);
        match field {
            "source" => input.surface_litter_input_kg_m2 = value,
            "stock" => input.surface_residue_seed_kg_m2 = value,
            "rate" => input.surface_decomposition_rate = value,
            "precip" => input.precipitation_m = value,
            "stress" => input.water_stress_fraction = value,
            _ => {}
        }
        record_case(&mut writer, id, input)?;
    }
    for (id, context) in [
        (
            "DAY-ZERO",
            DirectDecompositionActiveContext::Perennial {
                active_slot_index: 1,
                active_crop_slot_index: 1,
                runtime_day_of_year: 0,
            },
        ),
        (
            "DAY-HIGH",
            DirectDecompositionActiveContext::Perennial {
                active_slot_index: 1,
                active_crop_slot_index: 1,
                runtime_day_of_year: 367,
            },
        ),
        ("CONTEXT-MISSING", DirectDecompositionActiveContext::Missing),
        (
            "CONTEXT-AMBIGUOUS",
            DirectDecompositionActiveContext::Ambiguous,
        ),
    ] {
        let mut input = base_inputs(1, 0.2, 0.0, 0.0, 0.1, 0.5 / 365.25);
        input.active_context = context;
        record_case(&mut writer, id, input)?;
    }
    Ok(())
}

fn record_case(
    writer: &mut BufWriter<File>,
    id: &str,
    input: DirectDecompositionInputs,
) -> Result<(), Box<dyn std::error::Error>> {
    match input.compute_state() {
        Ok(_) => writeln!(writer, "{id},STATE,")?,
        Err(error) => writeln!(writer, "{id},ERROR,{error:?}")?,
    }
    Ok(())
}
