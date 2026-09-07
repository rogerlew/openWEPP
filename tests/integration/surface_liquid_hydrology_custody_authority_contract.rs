use std::fs;
use std::path::Path;

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

fn read_rust_tree(path: &Path) -> String {
    let mut paths = fs::read_dir(path)
        .unwrap_or_else(|error| panic!("read directory {}: {error}", path.display()))
        .map(|entry| entry.expect("directory entry").path())
        .collect::<Vec<_>>();
    paths.sort();
    let mut source = String::new();
    for path in paths {
        if path.is_dir() {
            source.push_str(&read_rust_tree(&path));
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            source.push_str(&read(
                path.to_str().expect("repository Rust path is valid UTF-8"),
            ));
        }
    }
    source
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
        "contract_version: 25",
        "status: approved",
        "maturity: active",
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
        "INV-SURFACELIQUID-024",
        "OBL-SURFACELIQUID-C-014",
        "INV-SURFACELIQUID-025",
        "ParentLocalPartial",
        "PersistentParentFinal",
        "INV-SURFACELIQUID-026",
        "OBL-SURFACELIQUID-C-016",
        "SURFACELIQUID-STAGE3-INACTIVE-LITTER-CUSTODY",
        "inactive byte-identical custody",
        "Litter liquid/ice vapor, phase transfer, surface-storage arithmetic",
        "current-ingress adoption, and WB14 may neither run",
        "only the positive\npost-event snow-free child may resume",
        "INV-SURFACELIQUID-027",
        "OBL-SURFACELIQUID-C-017",
        "SURFACELIQUID-V3-UNPUBLISHED-SOIL-CUSTODY",
        "mutually exclusive branches selected before projection",
        "owner-envelope, restart, checkpoint, accepted-\nreceipt",
        "fields are absent, not empty/defaulted",
        "one ordinary publishable owner projection",
        "INV-SURFACELIQUID-028",
        "OBL-SURFACELIQUID-C-018",
        "SURFACELIQUID-V3-LITTER-PHASE-CAPACITY-SPILL",
        "LitterPhaseCapacitySpillV1",
        "`LitterPhaseOverflow` source parcel",
        "not\n`CondensationOverflow`",
        "must not pass the raw state to\n`SurfaceLiquidOwnerEnvelopeV2::try_replace_v2_state`",
        "generic category plus prose detail is not the canonical payload",
        "INV-SURFACELIQUID-029",
        "OBL-SURFACELIQUID-C-019",
        "SURFACELIQUID-V3-HETEROGENEOUS-FINALIZED-USE-JOIN",
        "SurfaceLiquidV2HeterogeneousResourceJoinV1",
        "INV-SURFACELIQUID-030",
        "OBL-SURFACELIQUID-C-020",
        "SURFACELIQUID-V16-TOPOLOGY-RANKED-EXACT-OWNER",
        "INV-SURFACELIQUID-031",
        "OBL-SURFACELIQUID-C-021",
        "SURFACELIQUID-V24-VALIDATED-IN-MEMORY-HANDOFF",
        "INV-SURFACELIQUID-032",
        "OBL-SURFACELIQUID-C-022",
        "SURFACELIQUID-V25-LANED-ACTIVE-LOCAL-ROUTING",
        "OFE identifiers are opaque identities",
        "bare envelope/receipt parsing may",
        "Only `state_sha256`",
        "No executable `Default`",
        "SURFACELIQUID-E-001",
        "SURFACELIQUID-E-002",
        "select the greatest positive finite",
        "binary64 `c_k<=c_0` satisfying both predicates",
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
fn version_twenty_four_binds_private_validated_handoffs_without_moving_trust_boundaries() {
    let contract = read(CONTRACT);
    for required in [
        "## Validated In-Memory Surface-Resource Handoff Amendment",
        "INV-SURFACELIQUID-031",
        "OBL-SURFACELIQUID-C-021",
        "SURFACELIQUID-V24-VALIDATED-IN-MEMORY-HANDOFF",
        "private immutable nonserializable validated handoff",
        "no mutable dereference, public constructor, unchecked constructor",
        "discarded with the\nproof on any mutation",
        "Proof transfer between candidates is forbidden",
        "restart or\ncheckpoint restore, external bytes, durable publication",
        "untrusted executor always undergo the existing full parse",
        "zero repeated nested-owner serialization",
        "byte-exact rollback",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
    let index = read("docs/specifications/science-contracts/index.md");
    assert!(index.contains(
        "v24 admits private immutable revision-bound surface-resource validation handoffs"
    ));
    assert!(read(LSE).contains("INV-LANDSURFACEENERGY-159"));
}

#[test]
fn version_twenty_five_binds_laned_active_local_surface_routing() {
    let contract = read(CONTRACT);
    for required in [
        "## Lane-D-Local SurfaceLiquid Routing Posture Amendment",
        "INV-SURFACELIQUID-032",
        "OBL-SURFACELIQUID-C-022",
        "SURFACELIQUID-V25-LANED-ACTIVE-LOCAL-ROUTING",
        "all-local posture is whole-configuration semantics",
        "Lane D alone owns downstream transfer and outlet routing",
        "mixed/incomplete posture",
        "zero DC01 runon",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
    assert!(
        read("docs/specifications/science-contracts/index.md")
            .contains("v25 admits the complete all-local SurfaceLiquid routing posture")
    );
}

#[test]
fn version_twenty_three_binds_exact_owner_order_to_surface_topology() {
    let contract = read(CONTRACT);
    for required in [
        "## Topology-Ranked Exact-Surface Owner Amendment",
        "INV-SURFACELIQUID-030",
        "OBL-SURFACELIQUID-C-020",
        "SURFACELIQUID-V16-TOPOLOGY-RANKED-EXACT-OWNER",
        "complete `SurfaceLiquidConfigurationV2` record sequence",
        "ofe-9 -> ofe-10",
        "Numeric parsing, lexical sorting",
        "bare envelope/receipt parsing may",
        "Canonical order and complete membership are proven",
        "Duplicate keys or topology entries",
        "stale configuration or\ndigest",
        "complete rollback",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
    assert!(read(LSE).contains("INV-LANDSURFACEENERGY-158"));
    assert!(
        read("docs/specifications/science-contracts/index.md").contains(
            "v23 binds V16 exact-owner records and operands to authenticated configuration-topology rank"
        )
    );
}

#[test]
fn version_twenty_one_binds_phase_spill_to_one_generated_wb14_parcel() {
    let contract = read(CONTRACT);
    for required in [
        "## Exact V3 Litter-Phase Capacity-Spill Custody Amendment",
        "W_raw = W_retained + m_spill,tile",
        "U_raw = U_retained + m_spill,tile*C_w*(T_raw-T_ref)",
        "m_spill,ofe = f_t*m_spill,tile",
        "one liquid debit equal to the typed tile-\nbasis spill",
        "full accepted child `[0,dt)`",
        "joins the ordinary\ncanonical parcel ordering and well-mixed WB14 supply once",
        "Only a named retained-ingress receipt may return mass/enthalpy",
        "No capacity normalization, tolerance, phase equation",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }

    let tile_fraction = 0.625_f64;
    let spill_tile = 0.125_f64;
    let temperature = 276.0_f64;
    let specific_enthalpy = 4218.0_f64 * (temperature - 273.15_f64);
    let spill_ofe = tile_fraction * spill_tile;
    let enthalpy_ofe = spill_ofe * specific_enthalpy;
    assert_eq!(spill_ofe.to_bits(), 0.078125_f64.to_bits());
    assert_eq!(
        enthalpy_ofe.to_bits(),
        (spill_ofe * specific_enthalpy).to_bits()
    );
    assert!(enthalpy_ofe > 0.0);
}

#[test]
fn version_twenty_two_binds_heterogeneous_finalized_use_join() {
    let contract = read(CONTRACT);
    for required in [
        "## Exact Heterogeneous V3 Finalized-Use Join Amendment",
        "consumed only by that phase\nreceipt",
        "All unmatched surface rows form the complete ordinary set",
        "applies the ordinary `F/f_t` liquid debit once",
        "An empty ordinary set is bit-identical resource identity",
        "retains native litter ice, phase-adjusted thermodynamic\ncoordinates",
        "spill remains a separately ordered\ninternal `LitterPhaseOverflow` parcel",
        "neither creates an ingress parcel nor supplies a new sensible,\nlatent, fusion, or exact-surface energy operand",
        "every unified finalized row is\naccounted exactly once",
        "one resource candidate and ingress",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }

    let phase_adjusted_liquid = 2.5_f64;
    let tile_fraction = 0.4_f64;
    let finalized = [0.08_f64, 0.12_f64];
    let debit_ofe = finalized.into_iter().sum::<f64>();
    let debit_tile = debit_ofe / tile_fraction;
    let ending = phase_adjusted_liquid - debit_tile;
    assert_eq!(debit_ofe.to_bits(), 0.2_f64.to_bits());
    assert_eq!(debit_tile.to_bits(), 0.5_f64.to_bits());
    assert_eq!(ending.to_bits(), 2.0_f64.to_bits());
}

#[test]
fn version_14_binds_frozen_litter_surface_owner_v2_before_production() {
    let contract = read(CONTRACT);
    for required in [
        "INV-SURFACELIQUID-016",
        "INV-SURFACELIQUID-017",
        "INV-SURFACELIQUID-018",
        "INV-SURFACELIQUID-019",
        "INV-SURFACELIQUID-020",
        "INV-SURFACELIQUID-021",
        "SurfaceLiquidOwnerEnvelopeV2",
        "SurfaceLiquidOwnedStateV2",
        "SurfaceLiquidStateRecordV2",
        "litter_ice_kg_m2_tile",
        "litter_ice_capacity_kg_m2_tile=0.85*rho_w*litter_depth_m",
        "initializes `W_i` to positive-zero bits exactly",
        "Production V2-to-V1 downgrade is always rejected",
        "p_i=W_i/(W_l+W_i)",
        "No `xwgmin` denominator floor is admitted",
        "U_end=U*+L_f*m_frz-L_f*m_mlt",
        "`T += Q_phase/C_old` is prohibited",
        "current ingress into liquid custody",
        "Litter ice is unavailable to",
        "soil `frozwt`",
        "producer-supplied residual is never an",
        "unchanged `p61` and",
        "native-forest fixtures",
        "`zertol` tiny-ice deletion",
        "same-support phase re-solve",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }

    let beginning_liquid = 0.75_f64;
    let beginning_ice = 0.25_f64;
    let evaporation = 0.125_f64;
    let condensation = 0.0625_f64;
    let sublimation = 0.0625_f64;
    let deposition = 0.03125_f64;
    let post_vapor_liquid = beginning_liquid - evaporation + condensation;
    let post_vapor_ice = beginning_ice - sublimation + deposition;
    let frozen = 0.125_f64;
    let melted = 0.0_f64;
    let ending_liquid = post_vapor_liquid - frozen + melted;
    let ending_ice = post_vapor_ice + frozen - melted;
    assert_eq!(
        (ending_liquid + ending_ice).to_bits(),
        (beginning_liquid + beginning_ice - evaporation + condensation - sublimation + deposition)
            .to_bits()
    );
    let fusion_energy = 333_700.0_f64 * (frozen - melted);
    assert_eq!(fusion_energy.to_bits(), 41_712.5_f64.to_bits());
    let forbidden_ingress_donation = post_vapor_liquid + 0.50;
    assert!(frozen <= post_vapor_liquid);
    assert_ne!(frozen.to_bits(), forbidden_ingress_donation.to_bits());

    let production = read_rust_tree(Path::new(
        "crates/openwepp-hillslope-orchestrator/src/direct_runtime",
    ));
    for required in [
        "pub enum SurfaceLiquidOwnerEnvelopeV2",
        "pub struct SurfaceLiquidOwnedStateV2",
        "pub struct SurfaceLiquidStateRecordV2",
        "litter_ice_kg_m2_tile",
        "SurfaceLiquidCompleteOwnerProjectionV3",
        "migrate_v1_to_v2",
    ] {
        assert!(
            production.contains(required),
            "unchanged production is missing frozen-litter V2 obligation {required}"
        );
    }
}

#[test]
fn version_15_binds_exact_soil_energy_credit_custody_before_production() {
    let contract = read(CONTRACT);
    for required in [
        "INV-SURFACELIQUID-022",
        "E=exact(H_hi)+R",
        "ExactDyadicEnthalpy",
        "sign: -1 | 0 | 1",
        "coefficient_hex: lowercase hexadecimal nonnegative integer",
        "Zero has the sole form `(sign=0, coefficient_hex=\"0\", exponent2=0)`",
        "no-op transactions preserve existing high-term zero bits",
        "positive odd integer",
        "SoilThermalOwnedStateV2",
        "SoilThermalLayerStateV2",
        "SoilThermalOwnerEnvelopeV2",
        "SoilThermalEnergyCreditReceiptV2",
        "SoilThermalOwnerRestartV2",
        "SoilThermalOwnerCheckpointV2",
        "E_candidate,k = E_begin,k + sum(Q_soil,k) + sum(Q_top,k) + sum(Q_inf,k)",
        "round-to-nearest,\n   ties-to-even",
        "Production\nV2-to-V1 downgrade is prohibited",
        "H_hi=-34315.42154113602 J m^-2",
        "Q_inf=-8.0670339832330148e-19 J m^-2",
        "(sign=-1,coefficient_hex=\"1dc319224e55f\",exponent2=-109)",
        "exact-halfway ties with even-low and\nodd-low high terms",
        "minimum-positive and\nminimum-negative subnormal operands",
        "omission, duplication, reorder, and substitution",
        "wrong schema,\ndefinition, configuration, state, version, owner, transaction, predecessor",
        "Every poison proves exact\nrollback",
        "Restart gates split before and after a nonzero credit",
        "canonical WAT5 transaction,\nunchanged `p61`, and unchanged native-forest successor paths",
        "persisted\nmicrostepping or exact-carry diagnostics",
        "exact 60-second fallback floor\nremain unchanged",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }

    assert!(read(LSE).contains("INV-LANDSURFACEENERGY-150"));
    let registry = read("docs/specifications/science-contracts/index.md");
    assert!(
        registry.contains(
            "v23 binds V16 exact-owner records and operands to authenticated configuration-topology rank"
        )
    );
}

#[test]
fn version_15_requires_exact_carry_owner_receipt_and_restart_symbols() {
    let orchestrator = read_rust_tree(Path::new("crates/openwepp-hillslope-orchestrator/src"));
    let lse = read_rust_tree(Path::new("crates/openwepp-land-surface-energy/src"));
    let restart = read_rust_tree(Path::new("crates/openwepp-persisted-restart-v1/src"));
    let production = format!("{orchestrator}\n{lse}\n{restart}");
    for required in [
        "pub struct ExactDyadicEnthalpy",
        "pub struct SoilThermalOwnedStateV2",
        "pub struct SoilThermalLayerStateV2",
        "SoilThermalOwnerEnvelopeV2",
        "pub struct SoilThermalEnergyCreditReceiptV2",
        "pub struct SoilThermalOwnerRestartV2",
        "pub struct SoilThermalOwnerCheckpointV2",
    ] {
        assert!(
            production.contains(required),
            "unchanged production is missing required v15 exact-carry symbol {required}"
        );
    }
}

#[test]
fn version_16_binds_exact_surface_enthalpy_custody_before_production() {
    let contract = read(CONTRACT);
    for required in [
        "## Version 16 Exact LSE Surface-Enthalpy-Carry Amendment",
        "INV-SURFACELIQUID-023",
        "U_t = exact(U_hi,t) + R_U,t",
        "LseSurfaceEnthalpyOwnerEnvelopeV1",
        "LseSurfaceEnthalpyEnergyCreditReceiptV1",
        "SurfaceLiquidCompleteOwnerProjectionV4",
        "nonauthoritative high mirrors",
        "retained_ingress_tile_credit",
        "U_candidate,t = U_begin,t + sum_j exact(Q_surface,t,j)",
        "rounded once to binary64 nearest-even",
        "176400000000000..178200000000000 ns",
        "exact high bits and retained tile-credit operands were not\npreserved",
        "exact\n60-second floor remains a minimum fallback",
        "SURFACELIQUID-E-012",
        "LSEB-E-050",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }

    assert!(read(LSE).contains("INV-LANDSURFACEENERGY-151"));
    let registry = read("docs/specifications/science-contracts/index.md");
    assert!(
        registry.contains(
            "v23 binds V16 exact-owner records and operands to authenticated configuration-topology rank"
        )
    );
}

#[test]
fn version_16_requires_exact_surface_owner_receipt_restart_and_projection_symbols() {
    let orchestrator = read_rust_tree(Path::new("crates/openwepp-hillslope-orchestrator/src"));
    let lse = read_rust_tree(Path::new("crates/openwepp-land-surface-energy/src"));
    let restart = read_rust_tree(Path::new("crates/openwepp-persisted-restart-v1/src"));
    let production = format!("{orchestrator}\n{lse}\n{restart}");
    for required in [
        "pub struct LseSurfaceEnthalpyOwnerEnvelopeV1",
        "pub struct LseSurfaceEnthalpyEnergyCreditReceiptV1",
        "pub struct LseSurfaceEnthalpyOwnerRestartV1",
        "pub struct LseSurfaceEnthalpyOwnerCheckpointV1",
        "SurfaceLiquidCompleteOwnerProjectionV4",
        "SURFACELIQUID-E-012",
    ] {
        assert!(
            production.contains(required),
            "unchanged production is missing required v16 exact-surface symbol {required}"
        );
    }
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
        "One 1800-second child must return the historical interval outcome bitwise.",
        "two unequal-area OFEs",
        "final-owner-join rollback",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn version_9_binds_multi_lane_stage3_parent_release() {
    let contract = read(CONTRACT);
    let registry = read("docs/specifications/science-contracts/index.md");
    for required in [
        "SURFACELIQUID-V9-MULTI-LANE-STAGE3",
        "multiple lanes have resolved snow",
        "common earliest latest-state proposal",
        "no cross-lane energy, vapor, or snow scalar",
        "dual-resolved-snow attachment fixtures",
        "GAP-SURFACELIQUID-005` multi-production-lane covered Stage-3 parent execution | `CLOSED`",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
    let registry_row = registry
        .lines()
        .find(|line| line.starts_with("| `SC-SURFACELIQUID-001` |"))
        .expect("surface-liquid lifecycle registry row");
    assert!(
        registry_row.starts_with(
            "| `SC-SURFACELIQUID-001` | Persistent Snow-Free Surface-Liquid Hydrology Custody Contract | `approved` | `active` |"
        )
    );
    assert!(
        registry_row.contains("v25 admits the complete all-local SurfaceLiquid routing posture")
    );
    assert!(!registry.contains("multi-lane covered Stage-3 remains unauthorized"));
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
        "exactly 48 consecutive `Delta t_parent=1800 s` parent",
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
        "48-parent daily parity, and short-child attachment vectors",
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
