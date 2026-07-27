use cal04b_executor::{arg_value, sha256};
use openwepp_plant_phenology::{
    realize_forest_canopy, ForestCanopyError, ForestCanopyParameters, ForestCanopyRealization,
    GsiError, GsiParameters,
};
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::f64::consts::PI;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

const DESIGN_SHA256: &str = "dee46ab60cc364a9a4ffce2180763f4202dda2ebe2e90b779982f970f650b0a7";
const DAILY_TRACE_DAYS: usize = 365;
const DIFFERENCE_STEP: f64 = 1.0e-6;

const DESIGN_HEADER: [&str; 15] = [
    "design_id",
    "stage",
    "role",
    "axis",
    "units",
    "levels",
    "hidden_truth",
    "operator",
    "objective",
    "recovery_acceptance",
    "sensitivity",
    "boundary_failure",
    "enumeration",
    "stopping",
    "upstream_rule",
];

const EXPECTED_DESIGNS: [(&str, &str, &str); 6] = [
    (
        "EMP-BFBS-01",
        "foliar_structural_partition",
        "CALIBRATION_SUM_ONLY",
    ),
    (
        "REC-BFBS-01",
        "foliar_structural_partition",
        "ASSUMED_FOR_EXECUTION",
    ),
    ("REC-FE-01", "evergreen_fraction", "ASSUMED_FOR_EXECUTION"),
    ("EMP-LAI-01", "peak_lai", "CONDITIONAL_MATURE_LAI"),
    ("REC-LAI-01", "peak_lai", "ASSUMED_FOR_EXECUTION"),
    (
        "REC-CSBB-01",
        "canopy_floor_closure",
        "ASSUMED_FOR_EXECUTION",
    ),
];

#[derive(Clone, Copy)]
struct CanopyInputs {
    bf: f64,
    bs: f64,
    fe: f64,
    lai: f64,
    cs: f64,
    bb: f64,
}

struct StageFiles {
    membership_path: PathBuf,
    membership: BufWriter<File>,
    parent_results_path: PathBuf,
    parent_results: BufWriter<File>,
    rows: u64,
}

struct StageReceipt {
    membership_path: PathBuf,
    rows: u64,
    membership_hash: String,
    parent_results_path: PathBuf,
    parent_results_hash: String,
}

fn parse_csv(text: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut records = Vec::new();
    let mut record = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut chars = text.chars().peekable();
    while let Some(character) = chars.next() {
        match character {
            '"' if quoted && chars.peek() == Some(&'"') => {
                field.push('"');
                chars.next();
            }
            '"' => quoted = !quoted,
            ',' if !quoted => {
                record.push(std::mem::take(&mut field));
            }
            '\n' if !quoted => {
                record.push(std::mem::take(&mut field));
                if !record.iter().all(String::is_empty) {
                    records.push(std::mem::take(&mut record));
                } else {
                    record.clear();
                }
            }
            '\r' if !quoted => {}
            other => field.push(other),
        }
    }
    if quoted {
        return Err("unterminated quoted field in later-stage design".into());
    }
    if !field.is_empty() || !record.is_empty() {
        record.push(field);
        records.push(record);
    }
    Ok(records)
}

fn authenticate_design(path: &Path) -> Result<String, Box<dyn Error>> {
    let digest = sha256(path)?;
    if digest != DESIGN_SHA256 {
        return Err(format!(
            "later-stage design checksum mismatch: expected {DESIGN_SHA256}, got {digest}"
        )
        .into());
    }
    let records = parse_csv(&fs::read_to_string(path)?)?;
    if records.len() != EXPECTED_DESIGNS.len() + 1 {
        return Err("later-stage design row count mismatch".into());
    }
    if records[0].iter().map(String::as_str).ne(DESIGN_HEADER) {
        return Err("later-stage design header mismatch".into());
    }
    for (record, expected) in records[1..].iter().zip(EXPECTED_DESIGNS) {
        if record.len() != DESIGN_HEADER.len()
            || record[0] != expected.0
            || record[1] != expected.1
            || record[2] != expected.2
            || record[13] != "no refinement"
            || !record[14].contains("full accepted")
        {
            return Err(format!("later-stage design row mismatch for {}", expected.0).into());
        }
    }
    Ok(digest)
}

fn accepted_ids(path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let records = parse_csv(&text)?;
    if records.is_empty() || records[0].first().map(String::as_str) != Some("candidate_id") {
        return Err("accepted ensemble header mismatch".into());
    }
    let mut ids = Vec::new();
    let mut unique = HashSet::new();
    for record in &records[1..] {
        let id = record.first().ok_or("bad accepted row")?;
        if id.is_empty() || !unique.insert(id.clone()) {
            return Err(format!("empty or duplicate accepted candidate ID: {id}").into());
        }
        ids.push(id.clone());
    }
    if ids.is_empty() {
        return Err("empty accepted ensemble".into());
    }
    if !ids.windows(2).all(|pair| pair[0] < pair[1]) {
        return Err("accepted candidate IDs are not in canonical ascending order".into());
    }
    Ok(ids)
}

fn canopy(
    inputs: CanopyInputs,
    gsi: f64,
    previous_foliar_biomass: f64,
) -> Result<ForestCanopyRealization, ForestCanopyError> {
    realize_forest_canopy(
        ForestCanopyParameters {
            gsi: GsiParameters::generalized(),
            summer_foliar_biomass_kg_m2: inputs.bf,
            maximum_leaf_area_index: inputs.lai,
            evergreen_fraction: inputs.fe,
            structural_canopy_cover_fraction: inputs.cs,
            structural_biomass_kg_m2: inputs.bs,
            canopy_cover_coefficient_m2_kg: inputs.bb,
        },
        gsi,
        previous_foliar_biomass,
    )
}

fn seasonal_gsi() -> Vec<f64> {
    (0..DAILY_TRACE_DAYS)
        .map(|day| 0.5 - 0.5 * (2.0 * PI * day as f64 / DAILY_TRACE_DAYS as f64).cos())
        .collect()
}

fn bfbs_standardized_residual(sum_milli_kg_m2: i32) -> f64 {
    f64::from((sum_milli_kg_m2 - 18_990).abs()) / 248.0
}

fn daily_trace(
    inputs: CanopyInputs,
    gsi_values: &[f64],
) -> Result<Vec<ForestCanopyRealization>, Box<dyn Error>> {
    let first_activity = inputs.fe + (1.0 - inputs.fe) * gsi_values[0];
    let mut previous = inputs.bf * first_activity;
    let mut trace = Vec::with_capacity(gsi_values.len());
    for &gsi in gsi_values {
        let realization = canopy(inputs, gsi, previous)?;
        previous = realization.live_foliar_biomass_kg_m2;
        trace.push(realization);
    }
    Ok(trace)
}

fn sum_squared_error(
    modeled: &[ForestCanopyRealization],
    truth: &[ForestCanopyRealization],
    select: impl Fn(&ForestCanopyRealization) -> f64,
) -> f64 {
    modeled
        .iter()
        .zip(truth)
        .map(|(modeled, truth)| (select(modeled) - select(truth)).powi(2))
        .sum()
}

fn finite_difference(
    center: f64,
    lower: f64,
    upper: f64,
    objective: impl Fn(f64) -> Result<f64, Box<dyn Error>>,
) -> Result<(f64, &'static str), Box<dyn Error>> {
    if center - DIFFERENCE_STEP >= lower && center + DIFFERENCE_STEP <= upper {
        Ok((
            (objective(center + DIFFERENCE_STEP)? - objective(center - DIFFERENCE_STEP)?)
                / (2.0 * DIFFERENCE_STEP),
            "CENTRAL",
        ))
    } else if center <= lower {
        Ok((
            (objective(center + DIFFERENCE_STEP)? - objective(center)?) / DIFFERENCE_STEP,
            "FORWARD_BOUNDARY",
        ))
    } else {
        Ok((
            (objective(center)? - objective(center - DIFFERENCE_STEP)?) / DIFFERENCE_STEP,
            "BACKWARD_BOUNDARY",
        ))
    }
}

fn expected_failure(
    case_id: &str,
    design_id: &str,
    inputs: CanopyInputs,
    expected: ForestCanopyError,
    results: &mut BufWriter<File>,
) -> Result<(), Box<dyn Error>> {
    match canopy(inputs, 0.5, 0.0) {
        Ok(_) => Err(format!("invalid case {case_id} unexpectedly succeeded").into()),
        Err(error) if error == expected => {
            let message = error.to_string().replace(',', ";");
            writeln!(
                results,
                "{case_id},{design_id},INVALID-CASE,TYPED_FAILURE,invalid_native_parameters,not_applicable,expected_native_error,NaN,not_applicable,INVALID,{},EXPECTED_TYPED_FAILURE,not_applicable,NATIVE_FOREST_TYPED_ERROR",
                message
            )?;
            Ok(())
        }
        Err(error) => {
            Err(format!("invalid case {case_id} returned {error:?}, expected {expected:?}").into())
        }
    }
}

fn stage_files(root: &Path, stem: &str) -> Result<StageFiles, Box<dyn Error>> {
    let membership_path = root.join(format!("{stem}-membership.csv"));
    let parent_results_path = root.join(format!("{stem}-parent-results.csv"));
    let mut membership = BufWriter::with_capacity(8 * 1024 * 1024, File::create(&membership_path)?);
    let mut parent_results =
        BufWriter::with_capacity(8 * 1024 * 1024, File::create(&parent_results_path)?);
    writeln!(
        membership,
        "stage_member_id,design_id,gsi_candidate_id,bf_max_kg_m2,structural_biomass_kg_m2,evergreen_fraction,xmxlai_m2_m2,structural_cover_fraction,bb_m2_kg,parent_stage_member_id,parent_membership_sha256,state"
    )?;
    writeln!(
        parent_results,
        "stage_member_id,parent_stage_member_id,design_id,result_template_id,result_template_sha256,state"
    )?;
    Ok(StageFiles {
        membership_path,
        membership,
        parent_results_path,
        parent_results,
        rows: 0,
    })
}

fn finish_stage(mut stage: StageFiles) -> Result<StageReceipt, Box<dyn Error>> {
    stage.membership.flush()?;
    stage.parent_results.flush()?;
    let membership_hash = sha256(&stage.membership_path)?;
    let parent_results_hash = sha256(&stage.parent_results_path)?;
    Ok(StageReceipt {
        membership_path: stage.membership_path,
        rows: stage.rows,
        membership_hash,
        parent_results_path: stage.parent_results_path,
        parent_results_hash,
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let design = PathBuf::from(arg_value(&args, "--design")?);
    let accepted = PathBuf::from(arg_value(&args, "--accepted")?);
    let out = PathBuf::from(arg_value(&args, "--out")?);
    let design_hash = authenticate_design(&design)?;
    let gsi_ids = accepted_ids(&accepted)?;
    let accepted_hash = sha256(&accepted)?;
    fs::create_dir_all(&out)?;

    let object_root = PathBuf::from("/home/workdir/cal04b-objects/readiness");
    fs::create_dir_all(&object_root)?;
    let results_path = out.join("later-stage-results.csv");
    let mut results = BufWriter::new(File::create(&results_path)?);
    writeln!(
        results,
        "result_id,design_id,stage_member_id,evidence_role,operand_values,observation_or_truth,objective_components,objective,sensitivity,boundary_flags,failure,accepted_or_recovered,equifinal_set,evidence"
    )?;

    let base = CanopyInputs {
        bf: 0.2,
        bs: 0.1,
        fe: 0.5,
        lai: 6.0,
        cs: 0.2,
        bb: 5.0,
    };
    let gsi_values = seasonal_gsi();
    let truth_trace = daily_trace(base, &gsi_values)?;

    let sum_milli: [i32; 3] = [18_742, 18_990, 19_238];
    let sums = sum_milli.map(|value| f64::from(value) / 1_000.0);
    let bfs: [f64; 3] = [0.10, 0.20, 0.30];
    for (sum_index, sum) in sums.iter().enumerate() {
        for (bf_index, bf) in bfs.iter().enumerate() {
            let bs = sum - bf;
            let mature_total = canopy(
                CanopyInputs {
                    bf: *bf,
                    bs,
                    ..base
                },
                1.0,
                *bf,
            )?
            .live_foliar_biomass_kg_m2
                + bs;
            let expected_native_total = *bf + bs;
            if mature_total.to_bits() != expected_native_total.to_bits() {
                return Err(format!(
                    "native BFBS total {mature_total:.17} differs bit-exactly from \
                     Bf+Bs {expected_native_total:.17} kg/m2"
                )
                .into());
            }
            let objective = bfbs_standardized_residual(sum_milli[sum_index]);
            if objective > 1.0 {
                return Err(format!("frozen BFBS endpoint {sum} escaped closed acceptance").into());
            }
            let bf_sensitivity = finite_difference(*bf, DIFFERENCE_STEP, f64::INFINITY, |value| {
                let realization = canopy(
                    CanopyInputs {
                        bf: value,
                        bs,
                        ..base
                    },
                    1.0,
                    value,
                )?;
                Ok(realization.live_foliar_biomass_kg_m2 + realization.structural_biomass_kg_m2)
            })?
            .0;
            let bs_sensitivity = finite_difference(bs, 0.0, f64::INFINITY, |value| {
                let realization = canopy(
                    CanopyInputs {
                        bf: *bf,
                        bs: value,
                        ..base
                    },
                    1.0,
                    *bf,
                )?;
                Ok(realization.live_foliar_biomass_kg_m2 + realization.structural_biomass_kg_m2)
            })?
            .0;
            writeln!(
                results,
                "EMP-BFBS-{}-{},EMP-BFBS-01,RESULT-TEMPLATE-EMP-BFBS,CALIBRATION_COMBINATION_CONSTRAINT,Bf={bf:.3};Bs={bs:.3},mature_total=18.990_SE=0.248,standardized_absolute_residual={objective:.12},{objective:.12},d_total_d_Bf={bf_sensitivity:.9};d_total_d_Bs={bs_sensitivity:.9},{},NONE,{},all_Bf_Bs_pairs_at_sum,CAL03-OBS-HB-001_NATIVE_COMBINATION_OPERATOR",
                sum_index + 1,
                bf_index + 1,
                if sum_index == 0 || sum_index == 2 {
                    "OBSERVED_RANGE_BOUNDARY"
                } else {
                    "INTERIOR"
                },
                if objective <= 1.0 {
                    "RETAINED_COMBINATION"
                } else {
                    "REJECTED"
                }
            )?;
        }
    }

    let synthetic_bf: [f64; 3] = [0.10, 0.20, 0.30];
    let synthetic_bs: [f64; 3] = [0.00, 0.10, 0.20];
    let mut bfbs_recovery_rows = Vec::new();
    for (bf_index, bf) in synthetic_bf.iter().enumerate() {
        for (bs_index, bs) in synthetic_bs.iter().enumerate() {
            let modeled = canopy(
                CanopyInputs {
                    bf: *bf,
                    bs: *bs,
                    ..base
                },
                1.0,
                *bf,
            )?;
            let modeled_total =
                modeled.live_foliar_biomass_kg_m2 + modeled.structural_biomass_kg_m2;
            let truth_total = base.bf + base.bs;
            let objective = (modeled_total - truth_total).powi(2);
            bfbs_recovery_rows.push((*bf, *bs, objective));
            let bf_sensitivity = finite_difference(*bf, DIFFERENCE_STEP, f64::INFINITY, |value| {
                let realization = canopy(
                    CanopyInputs {
                        bf: value,
                        bs: *bs,
                        ..base
                    },
                    1.0,
                    value,
                )?;
                Ok(
                    (realization.live_foliar_biomass_kg_m2 + realization.structural_biomass_kg_m2
                        - truth_total)
                        .powi(2),
                )
            })?
            .0;
            let bs_sensitivity = finite_difference(*bs, 0.0, f64::INFINITY, |value| {
                let realization = canopy(
                    CanopyInputs {
                        bf: *bf,
                        bs: value,
                        ..base
                    },
                    1.0,
                    *bf,
                )?;
                Ok(
                    (realization.live_foliar_biomass_kg_m2 + realization.structural_biomass_kg_m2
                        - truth_total)
                        .powi(2),
                )
            })?
            .0;
            writeln!(
                results,
                "REC-BFBS-{}-{},REC-BFBS-01,RESULT-TEMPLATE-REC-BFBS,ASSUMED_FOR_EXECUTION,Bf={bf:.2};Bs={bs:.2},Bf=0.20;Bs=0.10,native_mature_total_squared_error={objective:.12},{objective:.12},d_objective_d_Bf={bf_sensitivity:.9};d_objective_d_Bs={bs_sensitivity:.9},{},NONE,{},sum=0.30,NATIVE_FOREST_SYNTHETIC_OPERATOR",
                bf_index + 1,
                bs_index + 1,
                if *bs == 0.0 || *bf == 0.1 || *bf == 0.3 {
                    "ENUMERATION_BOUNDARY"
                } else {
                    "INTERIOR"
                },
                if objective <= 1.0e-15 {
                    "RECOVERED_EQUIFINAL"
                } else {
                    "REJECTED"
                }
            )?;
        }
    }
    expected_failure(
        "INVALID-BFBS-NONPOSITIVE-BF",
        "REC-BFBS-01",
        CanopyInputs { bf: 0.0, ..base },
        ForestCanopyError::OutOfDomain {
            field: "summer_foliar_biomass_kg_m2",
        },
        &mut results,
    )?;
    expected_failure(
        "INVALID-BFBS-NEGATIVE-BS",
        "REC-BFBS-01",
        CanopyInputs { bs: -0.1, ..base },
        ForestCanopyError::OutOfDomain {
            field: "structural_biomass_kg_m2",
        },
        &mut results,
    )?;

    let fe_levels: [f64; 5] = [0.0, 0.25, 0.5, 0.75, 1.0];
    let mut fe_recovery_rows = Vec::new();
    for (index, fe) in fe_levels.iter().enumerate() {
        let modeled = daily_trace(CanopyInputs { fe: *fe, ..base }, &gsi_values)?;
        let objective =
            sum_squared_error(&modeled, &truth_trace, |row| row.foliar_activity_fraction);
        fe_recovery_rows.push((*fe, objective));
        let (sensitivity, method) = finite_difference(*fe, 0.0, 1.0, |value| {
            let candidate = daily_trace(CanopyInputs { fe: value, ..base }, &gsi_values)?;
            Ok(sum_squared_error(&candidate, &truth_trace, |row| {
                row.foliar_activity_fraction
            }))
        })?;
        writeln!(
            results,
            "REC-FE-{},REC-FE-01,RESULT-TEMPLATE-REC-FE,ASSUMED_FOR_EXECUTION,fe={fe:.2},fe=0.50;days={DAILY_TRACE_DAYS},daily_native_foliar_activity_SSE={objective:.12},{objective:.12},d_objective_d_fe={sensitivity:.9};method={method},{},NONE,{},fe=0.50,NATIVE_COMPLETE_DAILY_TRACE",
            index + 1,
            if index == 0 || index + 1 == fe_levels.len() {
                "ENUMERATION_BOUNDARY"
            } else {
                "INTERIOR"
            },
            if objective <= 1.0e-15 {
                "RECOVERED"
            } else {
                "REJECTED"
            }
        )?;
    }
    expected_failure(
        "INVALID-FE-ABOVE-ONE",
        "REC-FE-01",
        CanopyInputs { fe: 1.01, ..base },
        ForestCanopyError::Gsi(GsiError::UnitIntervalViolation {
            field: "evergreen_fraction",
        }),
        &mut results,
    )?;

    let lai_levels: [f64; 6] = [3.5, 4.0, 5.0, 6.0, 7.0, 8.0];
    let mut lai_recovery_rows = Vec::new();
    for (index, lai) in lai_levels.iter().enumerate() {
        let modeled = daily_trace(CanopyInputs { lai: *lai, ..base }, &gsi_values)?;
        let recovery_objective =
            sum_squared_error(&modeled, &truth_trace, |row| row.leaf_area_index);
        lai_recovery_rows.push((*lai, recovery_objective));
        let interval_distance = if *lai < 3.5 {
            3.5 - lai
        } else if *lai > 8.0 {
            lai - 8.0
        } else {
            0.0
        };
        let empirical_sensitivity =
            finite_difference(*lai, DIFFERENCE_STEP, f64::INFINITY, |value| {
                let realization = canopy(CanopyInputs { lai: value, ..base }, 1.0, base.bf)?;
                Ok(realization.leaf_area_index)
            })?
            .0;
        let (recovery_sensitivity, method) =
            finite_difference(*lai, DIFFERENCE_STEP, f64::INFINITY, |value| {
                let candidate = daily_trace(CanopyInputs { lai: value, ..base }, &gsi_values)?;
                Ok(sum_squared_error(&candidate, &truth_trace, |row| {
                    row.leaf_area_index
                }))
            })?;
        writeln!(
            results,
            "EMP-LAI-{},EMP-LAI-01,RESULT-TEMPLATE-EMP-LAI,CONDITIONAL_MATURE_LAI_CONSTRAINT,xmxlai={lai:.2},admitted_closed_interval_3.5_8.0,closed_interval_distance={interval_distance:.12},{interval_distance:.12},d_native_mature_LAI_d_xmxlai={empirical_sensitivity:.9},{},NONE,RETAINED_CONDITIONAL_RANGE,3.5_to_8.0,CAL03-OBS-HB-005_CONDITIONAL_NATIVE_OPERATOR",
            index + 1,
            if index == 0 || index + 1 == lai_levels.len() {
                "OBSERVED_EXTREMUM_NOT_PHYSIOLOGICAL_BOUND"
            } else {
                "INTERIOR"
            }
        )?;
        writeln!(
            results,
            "REC-LAI-{},REC-LAI-01,RESULT-TEMPLATE-REC-LAI,ASSUMED_FOR_EXECUTION,xmxlai={lai:.2},xmxlai=6.00;days={DAILY_TRACE_DAYS},daily_native_LAI_SSE={recovery_objective:.12},{recovery_objective:.12},d_objective_d_xmxlai={recovery_sensitivity:.9};method={method},{},NONE,{},xmxlai=6.00,NATIVE_COMPLETE_DAILY_TRACE",
            index + 1,
            if index == 0 || index + 1 == lai_levels.len() {
                "ENUMERATION_BOUNDARY"
            } else {
                "INTERIOR"
            },
            if recovery_objective <= 1.0e-15 {
                "RECOVERED"
            } else {
                "REJECTED"
            }
        )?;
    }
    expected_failure(
        "INVALID-LAI-NONPOSITIVE",
        "REC-LAI-01",
        CanopyInputs { lai: 0.0, ..base },
        ForestCanopyError::OutOfDomain {
            field: "maximum_leaf_area_index",
        },
        &mut results,
    )?;

    let cs_levels: [f64; 4] = [0.0, 0.2, 0.5, 0.8];
    let bb_levels: [f64; 4] = [1.0, 2.5, 5.0, 10.0];
    let mut csbb_rows = Vec::new();
    for cs in cs_levels {
        for bb in bb_levels {
            let inputs = CanopyInputs { cs, bb, ..base };
            let modeled = daily_trace(inputs, &gsi_values)?;
            let objective =
                sum_squared_error(&modeled, &truth_trace, |row| row.canopy_cover_fraction);
            let (cs_sensitivity, cs_method) = finite_difference(cs, 0.0, 0.8, |value| {
                let candidate = daily_trace(
                    CanopyInputs {
                        cs: value,
                        bb,
                        ..base
                    },
                    &gsi_values,
                )?;
                Ok(sum_squared_error(&candidate, &truth_trace, |row| {
                    row.canopy_cover_fraction
                }))
            })?;
            let (bb_sensitivity, bb_method) = finite_difference(bb, 1.0, 10.0, |value| {
                let candidate = daily_trace(
                    CanopyInputs {
                        cs,
                        bb: value,
                        ..base
                    },
                    &gsi_values,
                )?;
                Ok(sum_squared_error(&candidate, &truth_trace, |row| {
                    row.canopy_cover_fraction
                }))
            })?;
            csbb_rows.push((
                cs,
                bb,
                objective,
                cs_sensitivity,
                cs_method,
                bb_sensitivity,
                bb_method,
            ));
        }
    }
    let csbb_minimum = csbb_rows
        .iter()
        .map(|row| row.2)
        .min_by(f64::total_cmp)
        .ok_or("empty CS/BB design")?;
    for (index, row) in csbb_rows.iter().enumerate() {
        let (cs, bb, objective, cs_sensitivity, cs_method, bb_sensitivity, bb_method) = row;
        writeln!(
            results,
            "REC-CSBB-{},REC-CSBB-01,RESULT-TEMPLATE-REC-CSBB,ASSUMED_FOR_EXECUTION,Cs={cs:.2};bb={bb:.2},Cs=0.20;bb=5.00;days={DAILY_TRACE_DAYS},daily_native_cover_SSE={objective:.12},{objective:.12},d_objective_d_Cs={cs_sensitivity:.9};method_Cs={cs_method};d_objective_d_bb={bb_sensitivity:.9};method_bb={bb_method},{},NONE,{},minimum_tied_pairs,NATIVE_COMPLETE_DAILY_TRACE",
            index + 1,
            if *cs == 0.0 || *cs == 0.8 || *bb == 1.0 || *bb == 10.0 {
                "ENUMERATION_BOUNDARY"
            } else {
                "INTERIOR"
            },
            if (*objective - csbb_minimum).abs() <= 1.0e-15 {
                "RECOVERED_EQUIFINAL"
            } else {
                "REJECTED"
            }
        )?;
    }
    expected_failure(
        "INVALID-CS-ABOVE-CAP",
        "REC-CSBB-01",
        CanopyInputs { cs: 1.0, ..base },
        ForestCanopyError::OutOfDomain {
            field: "structural_canopy_cover_fraction",
        },
        &mut results,
    )?;
    expected_failure(
        "INVALID-BB-NONPOSITIVE",
        "REC-CSBB-01",
        CanopyInputs { bb: 0.0, ..base },
        ForestCanopyError::OutOfDomain {
            field: "canopy_cover_coefficient_m2_kg",
        },
        &mut results,
    )?;
    results.flush()?;
    let results_hash = sha256(&results_path)?;

    let bfbs_minimum = bfbs_recovery_rows
        .iter()
        .map(|row| row.2)
        .min_by(f64::total_cmp)
        .ok_or("empty BFBS recovery design")?;
    let recovered_bfbs = bfbs_recovery_rows
        .iter()
        .filter(|row| (row.2 - bfbs_minimum).abs() <= 1.0e-15)
        .map(|row| format!("Bf={:.2}_Bs={:.2}", row.0, row.1))
        .collect::<Vec<_>>();
    if !recovered_bfbs.iter().any(|row| row == "Bf=0.20_Bs=0.10") {
        return Err("BFBS recovery set excludes hidden truth".into());
    }
    let fe_minimum = fe_recovery_rows
        .iter()
        .map(|row| row.1)
        .min_by(f64::total_cmp)
        .ok_or("empty FE recovery design")?;
    let recovered_fe = fe_recovery_rows
        .iter()
        .filter(|row| (row.1 - fe_minimum).abs() <= 1.0e-15)
        .map(|row| format!("fe={:.2}", row.0))
        .collect::<Vec<_>>();
    if !recovered_fe.iter().any(|row| row == "fe=0.50") {
        return Err("FE recovery set excludes hidden truth".into());
    }
    let lai_minimum = lai_recovery_rows
        .iter()
        .map(|row| row.1)
        .min_by(f64::total_cmp)
        .ok_or("empty LAI recovery design")?;
    let recovered_lai = lai_recovery_rows
        .iter()
        .filter(|row| (row.1 - lai_minimum).abs() <= 1.0e-15)
        .map(|row| format!("xmxlai={:.2}", row.0))
        .collect::<Vec<_>>();
    if !recovered_lai.iter().any(|row| row == "xmxlai=6.00") {
        return Err("LAI recovery set excludes hidden truth".into());
    }
    let recovered_csbb = csbb_rows
        .iter()
        .filter(|row| (row.2 - csbb_minimum).abs() <= 1.0e-15)
        .map(|row| format!("Cs={:.2}_bb={:.2}", row.0, row.1))
        .collect::<Vec<_>>();
    if !recovered_csbb.iter().any(|row| row == "Cs=0.20_bb=5.00") {
        return Err("CS/BB recovery set excludes hidden truth".into());
    }
    let mut recovery = BufWriter::new(File::create(out.join("later-stage-recovery.csv"))?);
    writeln!(
        recovery,
        "design_id,trace_days,hidden_truth,recovered_set,recovery_status,results_sha256,evidence_limit"
    )?;
    writeln!(
        recovery,
        "REC-BFBS-01,1,Bf=0.20;Bs=0.10,{},RECOVERED_{},{results_hash},SYNTHETIC_RECOVERY_NOT_EMPIRICAL_CALIBRATION",
        recovered_bfbs.join("|"),
        if recovered_bfbs.len() == 1 { "UNIQUE" } else { "EQUIFINAL" },
    )?;
    writeln!(
        recovery,
        "REC-FE-01,{DAILY_TRACE_DAYS},fe=0.50,{},RECOVERED_{},{results_hash},SYNTHETIC_RECOVERY_NOT_EMPIRICAL_CALIBRATION",
        recovered_fe.join("|"),
        if recovered_fe.len() == 1 { "UNIQUE" } else { "EQUIFINAL" },
    )?;
    writeln!(
        recovery,
        "REC-LAI-01,{DAILY_TRACE_DAYS},xmxlai=6.00,{},RECOVERED_{},{results_hash},SYNTHETIC_RECOVERY_NOT_EMPIRICAL_CALIBRATION",
        recovered_lai.join("|"),
        if recovered_lai.len() == 1 { "UNIQUE" } else { "EQUIFINAL" },
    )?;
    writeln!(
        recovery,
        "REC-CSBB-01,{DAILY_TRACE_DAYS},Cs=0.20;bb=5.00,{},RECOVERED_{},{results_hash},SYNTHETIC_RECOVERY_NOT_EMPIRICAL_CALIBRATION",
        recovered_csbb.join("|"),
        if recovered_csbb.len() == 1 { "UNIQUE" } else { "EQUIFINAL" },
    )?;
    recovery.flush()?;

    let bfbs_template_hash = sha256(&results_path)?;
    let mut bfbs = stage_files(&object_root, "bfbs")?;
    for gsi in &gsi_ids {
        for (sum_index, sum) in sums.iter().enumerate() {
            for (bf_index, bf) in bfs.iter().enumerate() {
                let bs = sum - bf;
                let id = format!("BFBS-{gsi}-{}-{}", sum_index + 1, bf_index + 1);
                let result_id = format!("EMP-BFBS-{}-{}", sum_index + 1, bf_index + 1);
                writeln!(
                    bfbs.membership,
                    "{id},EMP-BFBS-01,{gsi},{bf:.3},{bs:.3},,,,,{gsi},{accepted_hash},ACCEPTED_COMBINATION_CONSTRAINT"
                )?;
                writeln!(
                    bfbs.parent_results,
                    "{id},{gsi},EMP-BFBS-01,{result_id},{bfbs_template_hash},EXECUTED_AND_RETAINED"
                )?;
                bfbs.rows += 1;
            }
        }
    }
    let bfbs_receipt = finish_stage(bfbs)?;

    let mut fe_stage = stage_files(&object_root, "fe")?;
    for gsi in &gsi_ids {
        for (sum_index, sum) in sums.iter().enumerate() {
            for (bf_index, bf) in bfs.iter().enumerate() {
                let bs = sum - bf;
                let parent = format!("BFBS-{gsi}-{}-{}", sum_index + 1, bf_index + 1);
                for (fe_index, fe) in fe_levels.iter().enumerate() {
                    let id = format!(
                        "FE-{gsi}-{}-{}-{}",
                        sum_index + 1,
                        bf_index + 1,
                        fe_index + 1
                    );
                    let result_id = format!("REC-FE-{}", fe_index + 1);
                    writeln!(
                        fe_stage.membership,
                        "{id},REC-FE-01,{gsi},{bf:.3},{bs:.3},{fe:.2},,,,{parent},{},RETAINED_SYNTHETIC_READINESS",
                        bfbs_receipt.membership_hash
                    )?;
                    writeln!(
                        fe_stage.parent_results,
                        "{id},{parent},REC-FE-01,{result_id},{results_hash},EXECUTED_AND_RETAINED"
                    )?;
                    fe_stage.rows += 1;
                }
            }
        }
    }
    let fe_receipt = finish_stage(fe_stage)?;

    let mut lai_stage = stage_files(&object_root, "lai")?;
    for gsi in &gsi_ids {
        for (sum_index, sum) in sums.iter().enumerate() {
            for (bf_index, bf) in bfs.iter().enumerate() {
                let bs = sum - bf;
                for (fe_index, fe) in fe_levels.iter().enumerate() {
                    let parent = format!(
                        "FE-{gsi}-{}-{}-{}",
                        sum_index + 1,
                        bf_index + 1,
                        fe_index + 1
                    );
                    for (lai_index, lai) in lai_levels.iter().enumerate() {
                        let id = format!(
                            "LAI-{gsi}-{}-{}-{}-{}",
                            sum_index + 1,
                            bf_index + 1,
                            fe_index + 1,
                            lai_index + 1
                        );
                        let result_id = format!("EMP-LAI-{}", lai_index + 1);
                        writeln!(
                            lai_stage.membership,
                            "{id},EMP-LAI-01,{gsi},{bf:.3},{bs:.3},{fe:.2},{lai:.2},,,{parent},{},ACCEPTED_CONDITIONAL_MATURE_LAI",
                            fe_receipt.membership_hash
                        )?;
                        writeln!(
                            lai_stage.parent_results,
                            "{id},{parent},EMP-LAI-01,{result_id},{results_hash},EXECUTED_AND_RETAINED"
                        )?;
                        lai_stage.rows += 1;
                    }
                }
            }
        }
    }
    let lai_receipt = finish_stage(lai_stage)?;

    let mut csbb_stage = stage_files(&object_root, "csbb")?;
    for gsi in &gsi_ids {
        for (sum_index, sum) in sums.iter().enumerate() {
            for (bf_index, bf) in bfs.iter().enumerate() {
                let bs = sum - bf;
                for (fe_index, fe) in fe_levels.iter().enumerate() {
                    for (lai_index, lai) in lai_levels.iter().enumerate() {
                        let parent = format!(
                            "LAI-{gsi}-{}-{}-{}-{}",
                            sum_index + 1,
                            bf_index + 1,
                            fe_index + 1,
                            lai_index + 1
                        );
                        for (cs_index, cs) in cs_levels.iter().enumerate() {
                            for (bb_index, bb) in bb_levels.iter().enumerate() {
                                let id = format!(
                                    "CSBB-{gsi}-{}-{}-{}-{}-{}-{}",
                                    sum_index + 1,
                                    bf_index + 1,
                                    fe_index + 1,
                                    lai_index + 1,
                                    cs_index + 1,
                                    bb_index + 1
                                );
                                let result_id = format!(
                                    "REC-CSBB-{}",
                                    cs_index * bb_levels.len() + bb_index + 1
                                );
                                writeln!(
                                    csbb_stage.membership,
                                    "{id},REC-CSBB-01,{gsi},{bf:.3},{bs:.3},{fe:.2},{lai:.2},{cs:.2},{bb:.2},{parent},{},RETAINED_SYNTHETIC_READINESS",
                                    lai_receipt.membership_hash
                                )?;
                                writeln!(
                                    csbb_stage.parent_results,
                                    "{id},{parent},REC-CSBB-01,{result_id},{results_hash},EXECUTED_AND_RETAINED"
                                )?;
                                csbb_stage.rows += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    let csbb_receipt = finish_stage(csbb_stage)?;

    let mut index = BufWriter::new(File::create(out.join("later-stage-membership.csv"))?);
    writeln!(
        index,
        "stage,membership_path,membership_rows,membership_sha256,parent_membership_sha256,parent_results_path,parent_results_rows,parent_results_sha256,design_sha256,state"
    )?;
    let membership_total =
        bfbs_receipt.rows + fe_receipt.rows + lai_receipt.rows + csbb_receipt.rows;
    for (stage, path, rows, hash, parent_hash, result_path, result_hash) in [
        (
            "foliar_structural_partition",
            bfbs_receipt.membership_path,
            bfbs_receipt.rows,
            bfbs_receipt.membership_hash.clone(),
            accepted_hash.clone(),
            bfbs_receipt.parent_results_path,
            bfbs_receipt.parent_results_hash,
        ),
        (
            "evergreen_fraction",
            fe_receipt.membership_path,
            fe_receipt.rows,
            fe_receipt.membership_hash.clone(),
            bfbs_receipt.membership_hash,
            fe_receipt.parent_results_path,
            fe_receipt.parent_results_hash,
        ),
        (
            "peak_lai",
            lai_receipt.membership_path,
            lai_receipt.rows,
            lai_receipt.membership_hash.clone(),
            fe_receipt.membership_hash,
            lai_receipt.parent_results_path,
            lai_receipt.parent_results_hash,
        ),
        (
            "canopy_floor_closure",
            csbb_receipt.membership_path,
            csbb_receipt.rows,
            csbb_receipt.membership_hash,
            lai_receipt.membership_hash,
            csbb_receipt.parent_results_path,
            csbb_receipt.parent_results_hash,
        ),
    ] {
        writeln!(
            index,
            "{stage},{},{rows},{hash},{parent_hash},{},{rows},{result_hash},{design_hash},PASS",
            path.display(),
            result_path.display()
        )?;
    }
    index.flush()?;
    let index_path = out.join("later-stage-membership.csv");
    let executable = env::current_exe()?;
    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/bin/readiness.rs");
    fs::write(
        object_root.join("execution-receipt.csv"),
        format!(
            "field,value\nstate,PASS\nexact_command,{}\nsource_path,{}\nsource_sha256,{}\nbinary_path,{}\nbinary_sha256,{}\ndesign_sha256,{}\naccepted_sha256,{}\nresults_sha256,{}\nrecovery_sha256,{}\nmembership_index_sha256,{}\naccepted_count,{}\nmembership_count,{membership_total}\n",
            args.join(" "),
            source.display(),
            sha256(&source)?,
            executable.display(),
            sha256(&executable)?,
            design_hash,
            accepted_hash,
            results_hash,
            sha256(&out.join("later-stage-recovery.csv"))?,
            sha256(&index_path)?,
            gsi_ids.len(),
        ),
    )?;

    println!(
        "PASS accepted_gsi={} memberships={} daily_trace_days={DAILY_TRACE_DAYS}",
        gsi_ids.len(),
        membership_total
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_parser_preserves_quoted_design_fields() {
        let records = parse_csv(
            "design_id,axis,levels\nREC-CSBB-01,\"Cs;bb\",\"Cs=0.00|0.20;bb=1.00|2.50\"\n",
        )
        .expect("valid CSV");
        assert_eq!(records.len(), 2);
        assert_eq!(records[1][1], "Cs;bb");
        assert_eq!(records[1][2], "Cs=0.00|0.20;bb=1.00|2.50");
    }

    #[test]
    fn frozen_design_authenticates_exactly() {
        let path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../artifacts/later-stage-design.csv");
        assert_eq!(
            authenticate_design(&path).expect("frozen design"),
            DESIGN_SHA256
        );
    }

    #[test]
    fn native_daily_trace_is_complete_and_recovers_hidden_fe() {
        let base = CanopyInputs {
            bf: 0.2,
            bs: 0.1,
            fe: 0.5,
            lai: 6.0,
            cs: 0.2,
            bb: 5.0,
        };
        let gsi = seasonal_gsi();
        let truth = daily_trace(base, &gsi).expect("native truth trace");
        let competitor =
            daily_trace(CanopyInputs { fe: 0.25, ..base }, &gsi).expect("native competitor trace");
        assert_eq!(gsi.len(), DAILY_TRACE_DAYS);
        assert_eq!(truth.len(), DAILY_TRACE_DAYS);
        assert_eq!(
            sum_squared_error(&truth, &truth, |row| row.foliar_activity_fraction),
            0.0
        );
        assert!(sum_squared_error(&competitor, &truth, |row| row.foliar_activity_fraction) > 0.0);
    }

    #[test]
    fn finite_difference_selects_central_and_boundary_schemes() {
        let central = finite_difference(0.5, 0.0, 1.0, |value| Ok(value * value)).expect("central");
        let forward = finite_difference(0.0, 0.0, 1.0, |value| Ok(value * value)).expect("forward");
        assert_eq!(central.1, "CENTRAL");
        assert!((central.0 - 1.0).abs() < 1.0e-8);
        assert_eq!(forward.1, "FORWARD_BOUNDARY");
        assert!(forward.0 >= 0.0);
    }

    #[test]
    fn declared_invalid_native_inputs_fail() {
        let base = CanopyInputs {
            bf: 0.2,
            bs: 0.1,
            fe: 0.5,
            lai: 6.0,
            cs: 0.2,
            bb: 5.0,
        };
        assert!(canopy(CanopyInputs { bf: 0.0, ..base }, 0.5, 0.0).is_err());
        assert!(canopy(CanopyInputs { fe: 1.01, ..base }, 0.5, 0.0).is_err());
        assert!(canopy(CanopyInputs { lai: 0.0, ..base }, 0.5, 0.0).is_err());
        assert!(canopy(CanopyInputs { cs: 1.0, ..base }, 0.5, 0.0).is_err());
        assert!(canopy(CanopyInputs { bb: 0.0, ..base }, 0.5, 0.0).is_err());
    }

    #[test]
    fn bfbs_closed_one_se_endpoints_are_accepted_exactly() {
        assert_eq!(bfbs_standardized_residual(18_742), 1.0);
        assert_eq!(bfbs_standardized_residual(18_990), 0.0);
        assert_eq!(bfbs_standardized_residual(19_238), 1.0);
    }
}
