use std::fs;

use sha2::{Digest, Sha256};

use openwepp_hillslope_orchestrator::{
    DirectDayConstructorInputs, DirectDayFrame, DirectInfiltrationDepressionInputs,
    DirectRunIdentity, DirectWb14HyetographInterval, DirectWb14InfiltrationProducerInputs,
};

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md";
const LSE: &str = "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md";
const RUNOFF: &str = "crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs";
const PACKAGE: &str =
    "docs/work-packages/20260814-persistent-snow-free-surface-liquid-hydrology-custody-001";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

fn authorize(beginning_tile: f64, tile_fraction: f64, demands: &[f64]) -> Vec<f64> {
    let supply = tile_fraction * beginning_tile;
    let demand_sum: f64 = demands.iter().sum();
    if demand_sum == 0.0 || supply == 0.0 {
        return vec![0.0; demands.len()];
    }
    if demand_sum <= supply {
        return demands.to_vec();
    }
    let raw = demands
        .iter()
        .map(|demand| demand * supply / demand_sum)
        .collect::<Vec<_>>();
    let raw_sum = raw.iter().sum::<f64>();
    if raw_sum <= supply {
        return raw;
    }
    let tolerance = 1.0e-14 + 64.0 * f64::EPSILON * (raw_sum.abs() + supply.abs());
    assert!(raw_sum - supply <= tolerance);
    let initial_scale = supply / raw_sum;
    let scaled_sum = |scale: f64| raw.iter().map(|amount| amount * scale).sum::<f64>();
    let scale = if scaled_sum(initial_scale) <= supply {
        initial_scale
    } else {
        let mut lower_bits = 0_u64;
        let mut upper_bits = initial_scale.to_bits();
        for _ in 0..64 {
            if lower_bits + 1 >= upper_bits {
                break;
            }
            let middle_bits = lower_bits + (upper_bits - lower_bits) / 2;
            if scaled_sum(f64::from_bits(middle_bits)) <= supply {
                lower_bits = middle_bits;
            } else {
                upper_bits = middle_bits;
            }
        }
        assert_eq!(lower_bits + 1, upper_bits);
        f64::from_bits(lower_bits)
    };
    let authorizations = raw.iter().map(|amount| amount * scale).collect::<Vec<_>>();
    assert!(authorizations.iter().sum::<f64>() <= supply);
    authorizations
}

fn split_amount_and_energy(mass: f64, energy: f64, child_mass: f64) -> (f64, f64) {
    if mass == 0.0 {
        return (0.0, 0.0);
    }
    let child_energy = child_mass / mass * energy;
    (child_energy, energy - child_energy)
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ReferenceContinuation {
    day: u32,
    next_interval: u8,
    cumulative_supply_m: f64,
    cumulative_infiltration_m: f64,
}

fn advance_reference_continuation(
    mut state: ReferenceContinuation,
    supply_m: f64,
    infiltration_m: f64,
) -> ReferenceContinuation {
    assert!((0..=48).contains(&state.next_interval));
    assert!(supply_m >= 0.0 && infiltration_m >= 0.0 && infiltration_m <= supply_m);
    if state.next_interval == 48 {
        state.day += 1;
        state.next_interval = 0;
        state.cumulative_supply_m = 0.0;
        state.cumulative_infiltration_m = 0.0;
    }
    state.cumulative_supply_m += supply_m;
    state.cumulative_infiltration_m += infiltration_m;
    state.next_interval += 1;
    state
}

#[test]
fn contract_binds_existing_lse_identity_and_restart_bytes() {
    let contract = read(CONTRACT);
    for required in [
        "contract_id: SC-SURFACELIQUID-001",
        "contract_version: 8",
        "status: in_review",
        "version 7 remains the released runtime baseline",
        "INV-SURFACELIQUID-001",
        "INV-SURFACELIQUID-002",
        "(run_id, ofe_id, tile_id, surface_id, surface_class, source_type, source_id)",
        "`bare_mineral_soil` | `surface_liquid`",
        "`forest_litter` | `litter_liquid`",
        "`soil_layer_liquid` remains the soil-layer owner",
        "`ground_ingress_mode` is exactly `open_raw_precipitation` or",
        "surface class does not infer exposure",
        "SurfaceLiquidOfeBinding",
        "production_lane_index",
        "ordered_soil_layer_ids",
        "infiltration_soil_thermal_layer_id",
        "apply_same_pass_infiltration",
        "generic category plus prose detail is not the canonical payload",
        "Only `state_sha256`",
        "No executable `Default`",
        "SURFACELIQUID-E-001",
        "SURFACELIQUID-E-002",
        "greatest positive finite binary64 `c_k<=c_0`",
        "no canonical-last remainder",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
    let water = read("crates/openwepp-land-surface-energy/src/water.rs");
    for required in [
        "pub struct GroundWaterKey",
        "pub surface_class: Option<SurfaceClass>",
        "pub source_type: WaterSourceType",
        "pub source_id: SourceId",
        "pub amount_kg_m2_stand_ground: f64",
    ] {
        assert!(
            water.contains(required),
            "LSE water identity missing {required}"
        );
    }
    assert!(
        read(LSE)
            .contains("Hydrology exclusively owns ponded, litter-held and soil-layer water mass")
    );
}

#[test]
fn version_8_child_slab_authority_binds_complete_transaction_before_release() {
    let contract = read(CONTRACT);
    for required in [
        "INV-SURFACELIQUID-012",
        "INV-SURFACELIQUID-013",
        "INV-SURFACELIQUID-014",
        "production-lane identity",
        "WB14 model-definition identity",
        "effective-conductivity",
        "matric-potential",
        "storage-capacity",
        "may be shorter because of an event, restart, output, parent",
        "processes all OFEs in topology order",
        "upstream runoff becomes",
        "downstream runon within that child",
        "Inactive owners are byte-identical carries",
        "Independent `validate()` reconstruction",
        "one 1800-second child bit-identical complete production-",
        "two unequal-area OFEs",
        "final-owner-join rollback",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn independent_daf_and_condensation_vectors_bind_one_basis_conversion() {
    let authorizations = authorize(4.0, 0.25, &[0.75, 1.25]);
    assert_eq!(authorizations, vec![0.375, 0.625]);

    let finalized = [0.25, 0.5];
    for ((use_amount, authorization), request) in
        finalized.iter().zip(&authorizations).zip([0.75, 1.25])
    {
        assert!(*use_amount <= *authorization);
        assert!(*authorization <= request);
    }
    let beginning_tile = 4.0;
    let condensation_ofe = 0.2;
    let ending_raw_tile =
        beginning_tile - finalized.iter().sum::<f64>() / 0.25 + condensation_ofe / 0.25;
    assert_eq!(ending_raw_tile.to_bits(), 1.8_f64.to_bits());

    let overflow_raw_tile = beginning_tile + 0.75 / 0.25;
    let ending_tile = overflow_raw_tile.min(2.0);
    let overflow_ofe = 0.25 * (overflow_raw_tile - 2.0).max(0.0);
    assert_eq!(ending_tile.to_bits(), 2.0_f64.to_bits());
    assert_eq!(overflow_ofe.to_bits(), 1.25_f64.to_bits());

    let contract = read(CONTRACT);
    for required in [
        "S_k = f_t * W_0,k",
        "0 <= F_i <= A_i <= D_i",
        "surface_liquid.condensation_kg_m2_ofe_ground",
        "OFE-ground here",
        "No request inflation",
        "second authorization",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn independent_joint_supply_vector_is_symmetric_and_never_overdraws() {
    let supply = 0.894_550_366_544_562_1_f64;
    let demands = [1.550_567_735_753_300_3, 0.666_084_441_700_219_5];
    let raw_sum = demands.iter().sum::<f64>();
    let raw = demands
        .iter()
        .map(|demand| demand * supply / raw_sum)
        .collect::<Vec<_>>();
    assert!(raw.iter().sum::<f64>() > supply);

    let forward = authorize(supply, 1.0, &demands);
    let reversed = authorize(supply, 1.0, &[demands[1], demands[0]]);
    assert_eq!(forward[0].to_bits(), reversed[1].to_bits());
    assert_eq!(forward[1].to_bits(), reversed[0].to_bits());
    assert!(forward.iter().sum::<f64>() <= supply);

    let common_scale = supply / raw.iter().sum::<f64>();
    assert_eq!(forward[0].to_bits(), (raw[0] * common_scale).to_bits());
    assert_eq!(forward[1].to_bits(), (raw[1] * common_scale).to_bits());
    assert_ne!(forward[1].to_bits(), (supply - forward[0]).to_bits());

    let equal = authorize(1.0, 1.0, &[1.0, 1.0, 1.0]);
    assert_eq!(equal, vec![1.0 / 3.0; 3]);

    let canonical_debit = |rows: &[(&str, f64)]| {
        let mut rows = rows.to_vec();
        rows.sort_by_key(|(key, _)| *key);
        rows.iter().map(|(_, amount)| amount).sum::<f64>()
    };
    let forward_uses = [("request-a", 0.1), ("request-b", 0.2), ("request-c", 0.3)];
    let reverse_uses = [("request-c", 0.3), ("request-b", 0.2), ("request-a", 0.1)];
    assert_ne!(
        forward_uses
            .iter()
            .map(|(_, amount)| amount)
            .sum::<f64>()
            .to_bits(),
        reverse_uses
            .iter()
            .map(|(_, amount)| amount)
            .sum::<f64>()
            .to_bits()
    );
    assert_eq!(
        canonical_debit(&forward_uses).to_bits(),
        canonical_debit(&reverse_uses).to_bits()
    );
}

#[test]
fn independent_attribution_retention_and_enthalpy_vectors_close() {
    let inputs = [2.0_f64, 1.0];
    let infiltration = 1.2_f64;
    let total: f64 = inputs.iter().sum();
    let first_infiltration = inputs[0] / total * infiltration;
    let second_infiltration = infiltration - first_infiltration;
    let excess = [
        inputs[0] - first_infiltration,
        inputs[1] - second_infiltration,
    ];
    assert!((first_infiltration - 0.8).abs() < f64::EPSILON);
    assert!((second_infiltration - 0.4).abs() < f64::EPSILON);
    assert!((excess.iter().sum::<f64>() - 1.8).abs() < 8.0 * f64::EPSILON);

    let remaining_capacity_ofe = 0.25 * (4.0 - 1.0);
    let retained = excess[0].min(remaining_capacity_ofe);
    let runoff = excess[0] - retained;
    assert_eq!(retained.to_bits(), 0.75_f64.to_bits());
    assert!((runoff - 0.45).abs() < f64::EPSILON);

    let specific_enthalpy = 4218.0 * (290.0 - 273.15);
    let parent_energy = excess[0] * specific_enthalpy;
    let (retained_energy, runoff_energy) =
        split_amount_and_energy(excess[0], parent_energy, retained);
    assert!((retained_energy + runoff_energy - parent_energy).abs() < 1.0e-9);
    assert!((retained_energy / retained - specific_enthalpy).abs() < 1.0e-10);

    let contract = read(CONTRACT);
    for required in [
        "existing daily WB14",
        "`depression_storage_capacity_m=0`",
        "never once per source",
        "Q_child = r * Q_parent",
        "Strictly increasing topology indices",
        "producer residuals",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn contract_binds_actual_wb14_and_rejects_duplicate_depression_custody() {
    let runoff = read(RUNOFF);
    for required in [
        "fn compute_wb14_infiltration_depression_with_profile(",
        "dc01_hourly_supply_basis(&inputs.hyetograph",
        "compute_green_ampt_interval_infiltration(",
        "inputs.depression_storage_capacity_m",
        "DirectWb14OutcomeWithProfile",
    ] {
        assert!(runoff.contains(required), "actual WB14 missing {required}");
    }
    let contract = read(CONTRACT);
    assert!(contract.contains("replaces, rather"));
    assert!(contract.contains("than augments"));
    assert!(contract.contains("returned depression delta"));
    assert!(contract.contains("must be exact zero"));
    assert!(contract.contains("one actual stateful shared WB14 transition"));
}

#[test]
fn actual_wb14_non_degenerate_timed_vector_executes() {
    let identity = DirectRunIdentity::new(91, 14, 1, 1).expect("identity");
    let mut inputs = DirectDayConstructorInputs::zero();
    inputs.infiltration_depression_inputs = DirectInfiltrationDepressionInputs {
        cumulative_infiltration_handoff_m: 0.0,
        depression_storage_delta_handoff_m: 0.0,
        producer_inputs: Some(DirectWb14InfiltrationProducerInputs {
            hyetograph: vec![
                DirectWb14HyetographInterval {
                    start_s: 0.0,
                    end_s: 1_800.0,
                    intensity_m_s: 0.006 / 1_800.0,
                },
                DirectWb14HyetographInterval {
                    start_s: 1_800.0,
                    end_s: 3_600.0,
                    intensity_m_s: 0.004 / 1_800.0,
                },
            ],
            hourly_additional_supply_m: [0.0; 24],
            effective_conductivity_m_s: 7.5e-7,
            matric_potential_m: 0.12,
            storage_capacity_m: 0.005,
            depression_storage_capacity_m: 0.0,
        }),
    };
    let mut frame =
        DirectDayFrame::from_constructor_inputs(identity, 0, 0, inputs).expect("strict day frame");
    frame
        .run_r4k_infiltration_depression_span()
        .expect("actual WB14 timed producer");
    assert_eq!(
        frame
            .infiltration_depression
            .depression_storage_delta_m
            .to_bits(),
        0.0_f64.to_bits()
    );
    assert!(frame.infiltration_depression.cumulative_infiltration_m > 0.0);
    assert_eq!(
        frame
            .infiltration_depression
            .cumulative_infiltration_m
            .to_bits(),
        0.005_f64.to_bits()
    );
    let excess: f64 = frame.wb14_hourly_excess_m.iter().sum();
    assert!(
        (frame.infiltration_depression.cumulative_infiltration_m + excess - 0.010).abs() < 1.0e-12
    );
}

#[test]
fn open_and_covered_ingress_are_mutually_exclusive_and_routing_scales_area() {
    let raw_rain_tile = 0.010_f64;
    let canopy_release_tile = 0.006_f64;
    let open_fraction = 0.30_f64;
    let covered_fraction = 0.70_f64;
    let ground_supply = open_fraction * raw_rain_tile + covered_fraction * canopy_release_tile;
    assert!((ground_supply - 0.0072).abs() < f64::EPSILON);
    let forbidden_duplicate = ground_supply + covered_fraction * raw_rain_tile;
    assert!((forbidden_duplicate - ground_supply).abs() > 0.006);

    let upstream_depth = 0.004_f64;
    let upstream_area = 250.0_f64;
    let downstream_area = 1_000.0_f64;
    let downstream_depth = upstream_depth * upstream_area / downstream_area;
    let upstream_energy = 40_000.0_f64;
    let downstream_energy = upstream_energy * upstream_area / downstream_area;
    assert_eq!(downstream_depth.to_bits(), 0.001_f64.to_bits());
    assert_eq!(downstream_energy.to_bits(), 10_000.0_f64.to_bits());
    assert_eq!(
        (upstream_depth * upstream_area).to_bits(),
        (downstream_depth * downstream_area).to_bits()
    );
    assert_eq!(
        (upstream_energy * upstream_area).to_bits(),
        (downstream_energy * downstream_area).to_bits()
    );

    let contract = read(CONTRACT);
    for required in [
        "P_ground,o = sum_open_tiles",
        "no raw precipitation",
        "DirectWb14ContinuationState",
        "exactly 48 consecutive `1800 s` transactions",
        "m_runon,d = m_runoff,u * A_u/A_d",
        "Q_retained,k,tile",
        "rho_w` | `1000 kg m^-3`",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn independent_48_step_continuation_and_day_reset_vector() {
    let initial = ReferenceContinuation {
        day: 17,
        next_interval: 0,
        cumulative_supply_m: 0.0,
        cumulative_infiltration_m: 0.0,
    };
    let ending = (0..48).fold(initial, |state, _| {
        advance_reference_continuation(state, 0.001, 0.0004)
    });
    assert_eq!(ending.day, 17);
    assert_eq!(ending.next_interval, 48);
    assert!((ending.cumulative_supply_m - 0.048).abs() < 1.0e-15);
    assert!((ending.cumulative_infiltration_m - 0.0192).abs() < 1.0e-15);

    let next_day = advance_reference_continuation(ending, 0.002, 0.001);
    assert_eq!(next_day.day, 18);
    assert_eq!(next_day.next_interval, 1);
    assert_eq!(next_day.cumulative_supply_m.to_bits(), 0.002_f64.to_bits());
    assert_eq!(
        next_day.cumulative_infiltration_m.to_bits(),
        0.001_f64.to_bits()
    );
    assert_ne!(next_day, ending);

    let contract = read(CONTRACT);
    for required in [
        "continuations` is an exact map keyed by `ofe_id`",
        "Continuation records serialize after store records",
        "basis_ofe_id=d",
        "No excess crosses tile or source identity",
        "executed 48-step/daily parity required at implementation gate",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn canonical_identity_digest_is_order_sensitive_and_field_sensitive() {
    let canonical_a = b"owner\0run\0ofe-1\0tile-a\0surface-a\0bare_mineral_soil\0surface_liquid\0source-a\x3f\xd0\0\0\0\0\0\0";
    let canonical_b = b"owner\0run\0ofe-1\0tile-b\0surface-a\0bare_mineral_soil\0surface_liquid\0source-a\x3f\xd0\0\0\0\0\0\0";
    let digest_a = Sha256::digest(canonical_a);
    let digest_b = Sha256::digest(canonical_b);
    assert_ne!(digest_a, digest_b);
    assert_eq!(digest_a, Sha256::digest(canonical_a));

    let contract = read(CONTRACT);
    assert!(contract.contains("16-character lowercase big-endian IEEE-754"));
    assert!(contract.contains("replacing only its own value with 64 zeroes"));
}

#[test]
fn package_preserves_hold_and_preimplementation_prohibition() {
    let package = read(&format!("{PACKAGE}/package.md"));
    let gate = read(&format!(
        "{PACKAGE}/artifacts/pre-implementation-contract-gate.md"
    ));
    for required in [
        "LSE-HYDRO-CUSTODY-001",
        "historical HOLD",
        "Production selection",
        "contract-derived tests",
        "byte-identical rollback",
    ] {
        assert!(
            package.contains(required)
                || read(&format!(
                    "{PACKAGE}/prompts/archived/20260814-persistent-snow-free-surface-liquid-hydrology-custody-001_kickoff_agent_prompt.md"
                ))
                .contains(required),
            "package surfaces missing {required}"
        );
    }
    assert!(gate.contains("contract is implementation-authoritative"));
    assert!(gate.contains("Production dispatch"));
}
