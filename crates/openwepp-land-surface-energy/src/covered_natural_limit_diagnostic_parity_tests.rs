//! Common observation-only natural iteration-limit parity; no golden replacement.
//! Reuses the original immutable `HistoricalV8` authority fixture. The detailed
//! audit observes Newton bases, not every line-search trial or LU factorization.
use super::{column, fixture, rust_failure_fixture};
use crate::{
    rejected_numerical_diagnostics, solve_covered_column, CoveredColumnAuthority,
    CoveredColumnInputs, CoveredColumnSolveOutcome, DiagnosticFailureKind, NormalizedResidual,
    NumericalDiagnostics, NumericalFailure, NumericalFailureKind, OfeId, RuntimeTileIdentity,
    Sha256Digest, SolveIdentity, SolvePass, SourceId, StepNorms, SurfaceClass, SurfaceId,
    WaterSourceType,
};
use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TileId, TransactionId};
use serde_json::{json, Value};

fn vector_bits(values: &[f64]) -> Vec<u64> {
    values.iter().map(|value| value.to_bits()).collect()
}

fn residual_bits(value: &NormalizedResidual) -> Value {
    let NormalizedResidual {
        identity,
        raw,
        scale,
        tolerance,
        normalized,
        unit,
    } = value;
    json!({
        "identity": identity, "unit": unit, "raw_bits": raw.to_bits(),
        "scale_bits": scale.to_bits(), "tolerance_bits": tolerance.to_bits(),
        "normalized_bits": normalized.to_bits(),
    })
}

fn step_bits(value: &StepNorms) -> Value {
    let StepNorms {
        temperature_k,
        humidity_kg_kg,
        ci_pa,
        hydraulic_mm,
        beta,
    } = value;
    json!({
        "temperature_k": temperature_k.map(f64::to_bits),
        "humidity_kg_kg": humidity_kg_kg.map(f64::to_bits),
        "ci_pa": ci_pa.map(f64::to_bits),
        "hydraulic_mm": hydraulic_mm.map(f64::to_bits),
        "beta": beta.map(f64::to_bits),
    })
}

fn failure_bits(value: &NumericalFailure) -> Value {
    // No rest pattern: a newly added failure field must update this observer.
    let NumericalFailure {
        kind,
        iterations,
        normalized_residuals,
        ordered_residuals,
        failed_solution,
        occupancy_id,
        active_bounds,
        backtracking_count,
        step_norms,
        pivot_magnitude,
        matrix_norm,
    } = value;
    let kind = match kind {
        NumericalFailureKind::SingularPivot => "singular_pivot",
        NumericalFailureKind::BacktrackingLimit => "backtracking_limit",
        NumericalFailureKind::IterationLimit => "iteration_limit",
    };
    json!({
        "kind": kind, "iterations": iterations,
        "normalized_residuals_bits": vector_bits(normalized_residuals),
        "ordered_residuals": ordered_residuals.iter().map(residual_bits).collect::<Vec<_>>(),
        "failed_solution_bits": vector_bits(failed_solution),
        "occupancy_id": occupancy_id, "active_bounds": active_bounds,
        "backtracking_count": backtracking_count, "step_norms_bits": step_bits(step_norms),
        "pivot_magnitude_bits": pivot_magnitude.map(f64::to_bits),
        "matrix_norm_bits": matrix_norm.map(f64::to_bits),
    })
}

fn diagnostic_bits(value: &NumericalDiagnostics) -> Value {
    // Full Serialize retains every discrete field, Option tag, identity and
    // owner receipt. Replace only the declared float-bearing fields with bits.
    let mut record = serde_json::to_value(value).expect("complete diagnostic serialization");
    record["ordered_residuals"] = json!(value
        .ordered_residuals
        .iter()
        .map(residual_bits)
        .collect::<Vec<_>>());
    record["step_norms"] = step_bits(&value.step_norms);
    record["bracket"] = json!(value
        .bracket
        .map(|v| [v.lower.to_bits(), v.upper.to_bits()]));
    record["pivot_magnitude"] = json!(value.pivot_magnitude.map(f64::to_bits));
    record["matrix_infinity_norm"] = json!(value.matrix_infinity_norm.map(f64::to_bits));
    record
}

fn diagnostic_identity(column: &CoveredColumnInputs) -> RuntimeTileIdentity {
    // Declared converter context only. These are not newly executed owners.
    let digest =
        |byte: char| Sha256Digest::try_new(byte.to_string().repeat(64)).expect("test digest");
    let owner = |name| ResourceOwnerId::try_new(name).expect("test owner");
    RuntimeTileIdentity {
        transaction_id: TransactionId(41),
        soil_thermal_transaction_id: TransactionId(41),
        lse_owner_id: owner("lse"),
        hydrology_owner_id: owner("hydrology"),
        soil_thermal_owner_id: owner("soil-thermal"),
        vegetation_owner_id: owner("vegetation"),
        biogeochemistry_owner_id: owner("biogeochemistry"),
        configuration_sha256: digest('a'),
        beginning_lse_state_sha256: digest('b'),
        beginning_hydrology_snapshot_sha256: digest('c'),
        beginning_soil_thermal_state_sha256: digest('d'),
        beginning_vegetation_state_sha256: digest('e'),
        beginning_biogeochemistry_state_sha256: digest('f'),
        ofe_id: OfeId::try_new("ofe-natural-limit").expect("test OFE"),
        tile_id: TileId::try_new("tile-natural-limit").expect("test tile"),
        surface_id: SurfaceId::try_new("surface-natural-limit").expect("test surface"),
        surface_class: SurfaceClass::BareMineralSoil,
        ground_source_type: WaterSourceType::SoilLayerLiquid,
        ground_source_id: SourceId::try_new("soil-layer-1").expect("test source"),
        ground_source_tile_id: None,
        ground_soil_layer_id: Some(SoilLayerId::try_new("soil-layer-1").expect("test layer")),
        tile_fraction: column.tile_fraction,
        interval_s: column.interval_s,
    }
}

fn column_float_bits(column: &CoveredColumnInputs) -> Vec<u64> {
    let mut bits = Vec::new();
    macro_rules! fields {
        ($v:expr; $($f:ident),+ $(,)?) => { $(bits.push($v.$f.to_bits());)+ };
    }
    macro_rules! bands {
        ($v:expr) => { fields!($v; direct_vis, diffuse_vis, direct_nir, diffuse_nir); };
    }
    fields!(column; interval_s, tile_fraction, pressure_pa, air_temperature_k,
        air_specific_humidity_kg_kg, reference_wind_m_s, atmospheric_downward_longwave_w_m2,
        ca_pa, canopy_to_atmosphere_heat_resistance_s_m,
        canopy_to_atmosphere_vapor_resistance_s_m, latent_heat_j_kg, top_rain_kg_m2_tile);
    fields!(column.under_canopy_geometry; canopy_height_m, canopy_roughness_m,
        reference_height_m, leaf_area_index);
    let ground = &column.ground;
    fields!(ground; interval_s, tile_fraction, surface_vis_albedo, surface_nir_albedo,
        surface_emissivity, surface_depth_m, surface_conductivity_w_m_k,
        surface_dry_heat_capacity_j_m2_k, air_temperature_k, air_specific_humidity_kg_kg,
        air_pressure_pa, reference_wind_m_s, atmospheric_downward_longwave_w_m2,
        surface_liquid_kg_m2_tile, surface_enthalpy_j_m2_tile, surface_temperature_warm_start_k);
    bits.extend(ground.litter_capacity_kg_m2_tile.map(f64::to_bits));
    bands!(ground.terminal_shortwave_w_m2_tile);
    fields!(ground.open_geometry; reference_height_m, roughness_momentum_m,
        roughness_heat_m, roughness_vapor_m);
    if let Some(bare) = &ground.bare_soil {
        fields!(bare; top_layer_liquid_kg_m2, top_layer_ice_kg_m2, porosity,
            saturated_matric_potential_mm, clapp_hornberger_b, theta_initial);
    }
    for soil in &ground.soil_nodes {
        fields!(soil; depth_m, conductivity_w_m_k, heat_capacity_j_m2_k, beginning_temperature_k);
    }
    for occupancy in &column.occupancies {
        fields!(occupancy; medlyn_g1_kpa_sqrt, g0_umol_m2_s, stem_area_m2_m2_tile,
            stem_absorbed_shortwave_w_m2_tile, beginning_canopy_liquid_kg_m2_tile,
            liquid_interception_fraction, liquid_capacity_kg_m2_plant, stemflow_fraction,
            gb_leaf_m_s, gb_wet_m_s, gb_stem_m_s, lai, sai, clumping_index,
            k1_sun_max_s1, k1_shade_max_s1, k2_max, k3_max_m_s, height_m,
            root_to_leaf_area, p50_leaf_mm, p50_xylem_mm, p50_root_mm, vulnerability_exponent);
        for leaf in [&occupancy.sun, &occupancy.shade] {
            fields!(leaf; leaf_area_m2_m2_tile, absorbed_shortwave_w_m2_tile,
                absorbed_par_w_m2_leaf, vcmax25, jmax25, rd25);
        }
        fields!(occupancy.biochemical; ha_vcmax_j_mol, hd_vcmax_j_mol, entropy_vcmax_j_mol_k,
            ha_jmax_j_mol, hd_jmax_j_mol, entropy_jmax_j_mol_k, kc25_pa, ha_kc_j_mol,
            ko25_pa, ha_ko_j_mol, gamma25_pa, ha_gamma_j_mol, oxygen_partial_pressure_pa,
            tp_vcmax_ratio, electron_quantum_yield, par_photon_umol_per_j,
            electron_curvature, ac_aj_curvature, ag_ap_curvature);
        for root in &occupancy.root_layers {
            fields!(root; root_fraction, soil_potential_mm, gravity_head_mm,
                z3_m, dxroot_m, ksoil_m2_s);
        }
    }
    bands!(column.shortwave.incident_w_m2_tile);
    bands!(column.shortwave.top_reflected_w_m2_tile);
    bands!(column.shortwave.ground_absorbed_by_incident_w_m2_tile);
    for optical in &column.shortwave.occupancies {
        bands!(optical.sun_leaf_absorbed_w_m2_tile);
        bands!(optical.shade_leaf_absorbed_w_m2_tile);
        bands!(optical.stem_absorbed_w_m2_tile);
    }
    if let Some(lower) = &column.stage3_lower_boundary {
        fields!(lower; snow_temperature_k, latent_heat_j_kg, sensible_to_canopy_air_w_m2,
            vapor_to_canopy_air_kg_m2_s, net_longwave_w_m2, shortwave_absorbed_w_m2,
            precipitation_advection_w_m2, snow_vis_albedo, snow_nir_albedo);
    }
    if let Some(optical) = &column.stage3_optical {
        bands!(optical.terminal_w_m2_tile);
        bands!(optical.absorbed_w_m2_tile);
        bands!(optical.reflected_w_m2_tile);
        fields!(optical; snow_vis_albedo, snow_nir_albedo);
    }
    bits
}

fn natural_fixture() -> (CoveredColumnInputs, Vec<f64>, Value) {
    let fixture = fixture();
    let expected = &fixture["exact_model_reductions"]["real_numerical_failures"]["iteration_limit"];
    let single = &fixture["exact_model_reductions"]["covered_single_rank"]["potential"];
    let (mut candidate, _) = column(&fixture, 1, single);
    // Exact original natural-limit setup (covered_oracle_conformance_tests).
    candidate.occupancies[0].k1_sun_max_s1 = 0.000_144_429_965_318_365_6;
    candidate.occupancies[0].k1_shade_max_s1 = 0.000_144_429_965_318_365_6;
    candidate.occupancies[0].k2_max = 1.538_924_383_128_636_2e-6;
    candidate.occupancies[0].k3_max_m_s = 1.969_219_977_554_62e-7;
    let start = vec![
        -5_706.990_986_525_235,
        -3_953.815_285_369_903,
        -8_895.703_204_372_228,
        -5_772.288_321_118_055,
        0.665_607_367_950_775_9,
        0.688_411_351_864_297_9,
        294.412_019_297_840_3,
        294.263_277_549_036_33,
        295.165_633_134_614_95,
        285.263_201_819_514_17,
        290.338_956_389_422_1,
        0.010_992_020_878_824_095,
        302.384_969_195_389_3,
        300.912_011_502_635_9,
        296.632_616_843_672_4,
    ];
    (candidate, start, expected.clone())
}

fn assert_failure_metadata(failure: &NumericalFailure, expected: &Value, start_len: usize) {
    assert_eq!(failure.kind, NumericalFailureKind::IterationLimit);
    assert_eq!(
        failure.iterations,
        u32::try_from(expected["rust_expected_iterations"].as_u64().unwrap())
            .expect("bounded iteration count")
    );
    assert_eq!(failure.iterations, 50);
    assert_eq!(
        failure.occupancy_id.as_deref(),
        expected["diagnostics"]["occupancy_id"].as_str()
    );
    assert_eq!(
        json!(failure.active_bounds),
        expected["diagnostics"]["active_bounds"]
    );
    assert!(expected["candidate"].is_null());
    let expected_count = expected["diagnostics"]["normalized_residuals"]
        .as_array()
        .unwrap()
        .len();
    assert_eq!(failure.normalized_residuals.len(), expected_count);
    assert_eq!(failure.failed_solution.len(), start_len);
    assert_eq!(failure.ordered_residuals.len(), expected_count);
    assert!(failure.normalized_residuals.iter().all(|v| v.is_finite()));
    assert!(failure.failed_solution.iter().all(|v| v.is_finite()));
    assert_eq!(
        failure.pivot_magnitude.is_some(),
        expected["diagnostics"]["pivot_magnitude"]
            .as_f64()
            .is_some()
    );
    assert_eq!(
        failure.matrix_norm.is_some(),
        expected["diagnostics"]["matrix_norm"].as_f64().is_some()
    );
    let declared = rust_failure_fixture();
    let metadata = &declared["records"]["iteration_limit"];
    assert_eq!(
        metadata["ordered_residuals"].as_array().unwrap().len(),
        failure.ordered_residuals.len(),
        "declared residual metadata covers every actual row"
    );
    assert_eq!(
        u64::from(failure.backtracking_count),
        metadata["backtracking_count"].as_u64().unwrap()
    );
    for (actual, expected) in failure
        .ordered_residuals
        .iter()
        .zip(metadata["ordered_residuals"].as_array().unwrap())
    {
        assert_eq!(actual.identity, expected["identity"].as_str().unwrap());
        // Original numeric golden remains untouched and separately fails in
        // release. Only its declared shape/identity/unit metadata is used here.
        let unit = match actual.unit {
            crate::ResidualUnit::WattsPerSquareMeter => "w_m-2",
            crate::ResidualUnit::KilogramsPerSquareMeterSecond => "kg_m-2_s-1",
            crate::ResidualUnit::Pascal => "pa",
            crate::ResidualUnit::Millimeter => "mm",
            crate::ResidualUnit::KilogramPerKilogram => "kg_kg-1",
            crate::ResidualUnit::Dimensionless => "dimensionless",
        };
        assert_eq!(unit, expected["unit"].as_str().unwrap());
    }
}

fn assert_audit(
    audit: &crate::solver_mechanism_audit::MechanismAudit,
    start_bits: &[u64],
) -> Vec<Vec<u64>> {
    assert!(!audit.overflow);
    assert_eq!(audit.dropped_events, 0);
    assert!(audit.iterations > 0 && audit.sweeps.starts > 0 && audit.probes.starts > 0);
    assert_eq!(audit.solves.starts, 1);
    assert_eq!(audit.solves.completions, 1); // Ok(Rejected), not a typed Result error.
    assert_eq!(audit.solves.errors, 0);
    assert_eq!(audit.component_replay.starts, 0);
    assert_eq!(audit.identity_anchor.starts, 0); // No Stage-3 boundary -> no anchor.
    for counts in [
        audit.maps,
        audit.solves,
        audit.sweeps,
        audit.probes,
        audit.evaluations,
    ] {
        assert_eq!(counts.starts, counts.completions + counts.errors);
    }
    let bases: Vec<_> = audit
        .events
        .iter()
        .filter_map(|event| {
            if let crate::solver_mechanism_audit::Event::SweepBase {
                base_bits,
                represented_snow,
                ..
            } = event
            {
                assert!(!represented_snow);
                Some(base_bits.clone())
            } else {
                None
            }
        })
        .collect();
    assert_eq!(
        u64::try_from(bases.len()).expect("bounded base count"),
        audit.sweeps.starts
    );
    assert_eq!(bases.first().map(Vec::as_slice), Some(start_bits));
    bases
}

#[expect(
    clippy::assertions_on_constants,
    reason = "ignored comparison must reject debug execution"
)]
fn require_release_posture() {
    assert!(!cfg!(debug_assertions), "same release posture required");
}

#[test]
#[ignore = "common release natural-limit diagnostic parity; explicit serial three-arm execution"]
fn natural_iteration_limit_full_diagnostic_parity() {
    require_release_posture();
    let (candidate, start, expected) = natural_fixture();
    assert_eq!(candidate.authority, CoveredColumnAuthority::HistoricalV8);
    assert!(candidate.stage3_lower_boundary.is_none());
    assert!(candidate.stage3_optical.is_none());
    let caps: Option<&crate::CoveredWaterCaps> = None;
    let before = candidate.clone();
    let before_debug = format!("{candidate:?}");
    let before_bits = column_float_bits(&candidate);
    let start_bits = vector_bits(&start);
    let audit = crate::solver_mechanism_audit::begin_mechanism_audit(true, 1_000_000)
        .expect("fresh detailed natural-limit observer");
    let result = solve_covered_column(&candidate, caps, start.clone())
        .expect("natural solve returns a mathematical outcome");
    let audit = audit.finish().expect("all actual solve scopes closed");
    let CoveredColumnSolveOutcome::Rejected(failure) = result else {
        panic!("original natural-limit vector unexpectedly accepted");
    };
    assert_failure_metadata(&failure, &expected, start.len());
    assert_eq!(candidate, before);
    assert_eq!(format!("{candidate:?}"), before_debug);
    assert_eq!(column_float_bits(&candidate), before_bits);
    assert_eq!(vector_bits(&start), start_bits);
    assert!(caps.is_none());

    let identity = diagnostic_identity(&candidate);
    let identity_before = identity.clone();
    let diagnostics = rejected_numerical_diagnostics(
        &identity,
        SolvePass::Potential,
        SolveIdentity::JointCanopyGround,
        &failure,
        Vec::new(),
    )
    .expect("public converter consumes actual natural failure");
    diagnostics
        .validate()
        .expect("complete actual failure DTO validation");
    assert!(!diagnostics.accepted);
    assert_eq!(
        diagnostics.failure_kind,
        Some(DiagnosticFailureKind::IterationLimit)
    );
    assert_eq!(identity, identity_before);
    assert!(diagnostics
        .owner_rollback_hashes
        .iter()
        .all(|r| r.before_sha256 == r.after_sha256));
    let serialized = serde_json::to_string(&diagnostics).expect("entire DTO JSON");
    let roundtrip: NumericalDiagnostics =
        serde_json::from_str(&serialized).expect("DTO JSON parse");
    roundtrip.validate().expect("roundtrip DTO validates");
    assert_eq!(diagnostic_bits(&diagnostics), diagnostic_bits(&roundtrip));
    assert_eq!(diagnostics, roundtrip);

    let bases = assert_audit(&audit, &start_bits);
    let record = json!({
        "schema": "stage3-natural-limit-diagnostic-parity-v1",
        "fixture_sha256": super::VECTOR_SHA256,
        "authority": "HistoricalV8",
        "beginning_column_debug": before_debug,
        "ending_column_debug": format!("{candidate:?}"),
        "beginning_column_float_bits": before_bits,
        "ending_column_float_bits": column_float_bits(&candidate),
        "initial_trial_bits": start_bits,
        "retained_initial_trial_bits": vector_bits(&start),
        "caps": null,
        "failure": failure_bits(&failure),
        "diagnostics": diagnostics,
        "diagnostics_serialized": serialized,
        "diagnostics_float_bits": diagnostic_bits(&roundtrip),
        "diagnostic_identity_context": format!("{identity:?}"),
        "diagnostic_identity_interval_bits": identity.interval_s.to_bits(),
        "diagnostic_identity_tile_fraction_bits": identity.tile_fraction.to_bits(),
        "newton_sweep_base_bits": bases,
        "final_failed_solution_bits": vector_bits(&failure.failed_solution),
        "audit": audit,
        "scope": "actual solver-input rollback; DTO owner hashes are conversion lineage only",
    });
    println!("STAGE3_NATURAL_LIMIT_DIAGNOSTIC {record}");
}
