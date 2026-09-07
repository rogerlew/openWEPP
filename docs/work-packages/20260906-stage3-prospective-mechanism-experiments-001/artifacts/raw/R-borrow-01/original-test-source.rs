// Runtime-derived EXP-STAGE3-20260906-R assertions, authored before behavior.
macro_rules! replay_scalar_bits {
    ($a:expr, $b:expr; $($field:ident),+ $(,)?) => {$(
        assert_eq!($a.$field.to_bits(), $b.$field.to_bits(), stringify!($field));
    )+};
}

fn replay_vector_bits(a: &[f64], b: &[f64]) {
    assert_eq!(a.len(), b.len());
    for (index, (a, b)) in a.iter().zip(b).enumerate() {
        assert_eq!(a.to_bits(), b.to_bits(), "float row {index}");
    }
}

#[cfg(test)]
fn replay_residual_result_bits(
    a: Result<Vec<f64>, LandSurfaceEnergyError>,
    b: Result<Vec<f64>, LandSurfaceEnergyError>,
) {
    match (a, b) {
        (Ok(a), Ok(b)) => replay_vector_bits(&a, &b),
        (Err(a), Err(b)) => assert_eq!(a, b),
        other => panic!("replay/complete result mismatch: {other:?}"),
    }
}

fn replay_liquid_bits(
    a: &crate::CoveredOccupancyLiquidLedger,
    b: &crate::CoveredOccupancyLiquidLedger,
) {
    assert_eq!(a.pass, b.pass);
    replay_scalar_bits!(a, b; beginning_store_kg_m2_tile, incident_rain_kg_m2_tile,
        ending_store_kg_m2_tile, evaporation_kg_m2_tile, condensation_kg_m2_tile,
        throughfall_kg_m2_tile, stemflow_kg_m2_tile, initial_drainage_kg_m2_tile,
        second_drainage_kg_m2_tile, wet_fraction, wet_surface_temperature_k,
        wet_surface_specific_enthalpy_j_kg);
}

fn replay_evaluation_bits(a: &CoveredColumnEvaluation, b: &CoveredColumnEvaluation) {
    assert_eq!(a, b);
    replay_scalar_bits!(a, b; canopy_air_temperature_k, canopy_air_specific_humidity_kg_kg,
        ground_temperature_k, ground_storage_w_m2_tile, ending_surface_enthalpy_j_m2_tile,
        ground_canopy_release_kg_m2_tile, ground_stemflow_kg_m2_tile,
        ground_sensible_to_canopy_air_w_m2, lower_boundary_vapor_to_canopy_air_kg_m2_s,
        canopy_sensible_w_m2, canopy_vapor_kg_m2_s, sensible_to_reference_air_w_m2,
        vapor_to_reference_air_kg_m2_s, shared_heat_residual_w_m2, shared_heat_tolerance_w_m2,
        shared_vapor_residual_kg_m2_s, shared_vapor_tolerance_kg_m2_s);
    for (a, b) in [
        (&a.raw_residuals, &b.raw_residuals),
        (&a.tolerances, &b.tolerances),
        (&a.normalized_residuals, &b.normalized_residuals),
        (&a.soil_temperature_k, &b.soil_temperature_k),
        (&a.ground_heat_cn_w_m2_tile, &b.ground_heat_cn_w_m2_tile),
    ] {
        replay_vector_bits(a, b);
    }
    replay_scalar_bits!(a.ground_water, b.ground_water; law_kg_m2_tile_s,
        final_kg_m2_tile_s, request_kg_m2_stand_ground, finalized_use_kg_m2_stand_ground,
        condensation_credit_kg_m2_stand_ground);
    assert_eq!(
        a.ground_water
            .authorization_kg_m2_stand_ground
            .map(f64::to_bits),
        b.ground_water
            .authorization_kg_m2_stand_ground
            .map(f64::to_bits)
    );
    let (la, lb) = (&a.whole_column_longwave, &b.whole_column_longwave);
    replay_scalar_bits!(la, lb; ground_net_w_m2, top_upward_w_m2);
    replay_vector_bits(&la.transmissivities, &lb.transmissivities);
    replay_vector_bits(&la.downward_boundaries_w_m2, &lb.downward_boundaries_w_m2);
    replay_vector_bits(&la.upward_boundaries_w_m2, &lb.upward_boundaries_w_m2);
    for (a, b) in la.component_net_w_m2.iter().zip(&lb.component_net_w_m2) {
        replay_vector_bits(a, b);
    }
    for (a, b) in a.occupancies.iter().zip(&b.occupancies) {
        replay_scalar_bits!(a, b; canopy_sensible_w_m2, canopy_vapor_kg_m2_s, wet_vapor_kg_m2_s);
        replay_vector_bits(&a.residuals, &b.residuals);
        replay_vector_bits(&a.tolerances, &b.tolerances);
        replay_liquid_bits(&a.liquid, &b.liquid);
        for (a, b) in [
            (&a.ci_pa, &b.ci_pa),
            (&a.emax_kg_m2_s, &b.emax_kg_m2_s),
            (
                &a.gross_assimilation_umol_co2_m2_leaf_s,
                &b.gross_assimilation_umol_co2_m2_leaf_s,
            ),
            (
                &a.net_assimilation_umol_co2_m2_leaf_s,
                &b.net_assimilation_umol_co2_m2_leaf_s,
            ),
            (
                &a.dark_respiration_umol_co2_m2_leaf_s,
                &b.dark_respiration_umol_co2_m2_leaf_s,
            ),
        ] {
            replay_vector_bits(a, b);
        }
        for (a, b) in [
            (&a.component_temperatures_k, &b.component_temperatures_k),
            (&a.absorbed_shortwave_w_m2, &b.absorbed_shortwave_w_m2),
            (&a.net_longwave_w_m2, &b.net_longwave_w_m2),
            (
                &a.sensible_to_canopy_air_w_m2,
                &b.sensible_to_canopy_air_w_m2,
            ),
            (
                &a.signed_vapor_to_canopy_air_kg_m2_s,
                &b.signed_vapor_to_canopy_air_kg_m2_s,
            ),
            (&a.component_areas_m2_m2_tile, &b.component_areas_m2_m2_tile),
            (
                &a.component_emissive_areas_m2_m2_tile,
                &b.component_emissive_areas_m2_m2_tile,
            ),
            (
                &a.component_heat_conductance_m_s_tile,
                &b.component_heat_conductance_m_s_tile,
            ),
            (
                &a.component_vapor_conductance_m_s_tile,
                &b.component_vapor_conductance_m_s_tile,
            ),
            (
                &a.component_surface_specific_humidity_kg_kg,
                &b.component_surface_specific_humidity_kg_kg,
            ),
        ] {
            replay_vector_bits(a, b);
        }
        assert_eq!(
            a.component_vapor_authorization_kg_m2_tile_s
                .map(|v| v.map(f64::to_bits)),
            b.component_vapor_authorization_kg_m2_tile_s
                .map(|v| v.map(f64::to_bits))
        );
        for (a, b) in a.source_water.iter().zip(&b.source_water) {
            replay_scalar_bits!(a, b; law_kg_m2_tile_s, final_kg_m2_tile_s,
                request_kg_m2_stand_ground, finalized_use_kg_m2_stand_ground);
            assert_eq!(
                a.authorization_kg_m2_stand_ground.map(f64::to_bits),
                b.authorization_kg_m2_stand_ground.map(f64::to_bits)
            );
        }
    }
}

fn replay_capture_bits(a: &CoveredReplayCapture, b: &CoveredReplayCapture) {
    assert_eq!(a.preparations.len(), b.preparations.len());
    assert_eq!(a.ledgers.len(), b.ledgers.len());
    assert_eq!(a.leaves.len(), b.leaves.len());
    for (a, b) in a.preparations.iter().zip(&b.preparations) {
        replay_scalar_bits!(a, b; beginning_store, incident_rain, preliminary_store,
            throughfall, stemflow, initial_drainage, capacity, wet_fraction);
    }
    for (a, b) in a.ledgers.iter().zip(&b.ledgers) {
        replay_liquid_bits(a, b);
    }
    for (a, b) in a.leaves.iter().flatten().zip(b.leaves.iter().flatten()) {
        assert_eq!(a.gas_branch, b.gas_branch);
        replay_scalar_bits!(a, b; surface_q, rs_s_m, ci_pa,
            gross_assimilation_umol_co2_m2_leaf_s, net_assimilation_umol_co2_m2_leaf_s,
            dark_respiration_umol_co2_m2_leaf_s);
    }
}

#[cfg(test)]
pub(super) fn assert_prospective_component_replay(column: &CoveredColumnInputs, trial: &[f64]) {
    assert!(
        replay_probe_corpus(column, trial, None),
        "potential full-sweep corpus"
    );
    replay_solve_trace_parity(column, trial, None);
    replay_selection_and_custody(column, trial);
    replay_fixed_and_boundary_corpus(column, trial);
    replay_current_leaf_first_errors(column, trial);
    replay_zero_par_ci_first_errors(column, trial);
    replay_additional_leaf_boundary_errors(column, trial);
    replay_pc1_selection_matrix(column, trial, None);
    replay_pc1_mixed_selection_matrix(column, trial, None);
    let immutable_before = column.clone();
    let validated = ValidatedCoveredEvaluationInputs::try_new(column, None).expect("inputs");
    let base = ValidatedCoveredJacobianBase::evaluate(&validated, trial).expect("base");
    for coordinate in (0..column.occupancies.len()).flat_map(|o| (6..10).map(move |k| 10 * o + k)) {
        let h = f64::EPSILON.sqrt() * trial[coordinate].abs().max(1.0);
        for sign in [-1.0, 1.0] {
            let mut probe = trial.to_vec();
            probe[coordinate] += sign * h;
            if !covered_trial_is_valid(&probe, column.occupancies.len(), false) {
                continue;
            }
            begin_covered_jacobian_full_probe_audit();
            let rollback = ReplayRollbackSnapshot::new(&base);
            let actual = covered_jacobian_probe_residuals(&base, &probe, coordinate);
            assert_eq!(
                take_covered_jacobian_full_probe_audit(),
                u32::from(!replay_pc1_expected_from_raw_trial(
                    trial,
                    coordinate,
                    column.occupancies.len()
                )),
                "R-PC1 raw-beta component classification"
            );
            let expected =
                evaluate_covered_column_validated(&validated, &probe, Some(&base.frozen), None)
                    .map(|evaluation| evaluation.normalized_residuals);
            replay_residual_result_bits(actual, expected);
            rollback.assert_unchanged(&base);
        }
    }
    assert_eq!(*column, immutable_before);
    begin_covered_jacobian_full_probe_audit();
    let replay_solve = solve_covered_column(column, None, trial.to_vec());
    let replay_complete_calls = take_covered_jacobian_full_probe_audit();
    begin_forced_complete_covered_jacobian_probe_audit();
    let complete_solve = solve_covered_column(column, None, trial.to_vec());
    let complete_calls = take_covered_jacobian_full_probe_audit();
    assert_eq!(replay_solve, complete_solve);
    assert!(replay_complete_calls < complete_calls);
}

// Explicit value encoding, not Rust memory/padding or Debug formatting. Typed
// clones below independently retain all IDs, enum/Option tags and vector order.
fn replay_column_float_bits(column: &CoveredColumnInputs) -> Vec<u64> {
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

fn replay_caps_bits(caps: Option<&CoveredWaterCaps>) -> Vec<u64> {
    caps.into_iter()
        .flat_map(|caps| {
            caps.root
                .values()
                .chain(std::iter::once(&caps.ground))
                .flat_map(|cap| {
                    [
                        cap.request_rate_kg_m2_tile_s.to_bits(),
                        cap.authorization_rate_kg_m2_tile_s.to_bits(),
                    ]
                })
        })
        .collect()
}

struct ReplayRollbackSnapshot {
    base_owner: usize,
    lifecycle: Option<crate::solver_mechanism_audit::LifecycleContext>,
    column: CoveredColumnInputs,
    column_bits: Vec<u64>,
    caps: Option<CoveredWaterCaps>,
    cap_bits: Vec<u64>,
    evaluation: CoveredColumnEvaluation,
    capture: Option<CoveredReplayCapture>,
    trial: Vec<f64>,
    frozen: CoveredFrozenBranches,
    owner: usize,
    column_owner: usize,
    caps_owner: Option<usize>,
    boundary_owner: Option<usize>,
    graph_hash: Option<[u8; 32]>,
}

impl ReplayRollbackSnapshot {
    fn new(base: &ValidatedCoveredJacobianBase<'_>) -> Self {
        let validated = base.validated;
        Self {
            base_owner: std::ptr::from_ref(base) as usize,
            lifecycle: base.lifecycle,
            column: validated.column.clone(),
            column_bits: replay_column_float_bits(validated.column),
            caps: validated.caps.cloned(),
            cap_bits: replay_caps_bits(validated.caps),
            evaluation: base.evaluation.clone(),
            capture: base.capture.as_ref().map(|capture| CoveredReplayCapture {
                preparations: capture.preparations.clone(),
                ledgers: capture.ledgers.clone(),
                leaves: capture.leaves.clone(),
            }),
            trial: base.trial.clone(),
            frozen: base.frozen.clone(),
            owner: std::ptr::from_ref(validated) as usize,
            column_owner: std::ptr::from_ref(validated.column) as usize,
            caps_owner: validated.caps.map(|v| std::ptr::from_ref(v) as usize),
            boundary_owner: validated
                .stage3_boundary
                .map(|v| std::ptr::from_ref(v) as usize),
            graph_hash: validated.component_graph.as_ref().map(|v| v.schema_hash),
        }
    }

    fn assert_unchanged(&self, base: &ValidatedCoveredJacobianBase<'_>) {
        let validated = base.validated;
        assert_eq!(self.base_owner, std::ptr::from_ref(base) as usize);
        assert_eq!(self.lifecycle, base.lifecycle);
        assert_eq!(&self.column, validated.column);
        assert_eq!(self.column_bits, replay_column_float_bits(validated.column));
        assert_eq!(self.caps.as_ref(), validated.caps);
        assert_eq!(self.cap_bits, replay_caps_bits(validated.caps));
        replay_evaluation_bits(&self.evaluation, &base.evaluation);
        match (&self.capture, &base.capture) {
            (Some(a), Some(b)) => replay_capture_bits(a, b),
            (None, None) => {}
            _ => panic!("rollback changed capture presence"),
        }
        replay_vector_bits(&self.trial, &base.trial);
        assert_eq!(self.frozen, base.frozen);
        assert_eq!(self.owner, std::ptr::from_ref(validated) as usize);
        assert_eq!(
            self.column_owner,
            std::ptr::from_ref(validated.column) as usize
        );
        assert_eq!(
            self.caps_owner,
            validated.caps.map(|v| std::ptr::from_ref(v) as usize)
        );
        assert_eq!(
            self.boundary_owner,
            validated
                .stage3_boundary
                .map(|v| std::ptr::from_ref(v) as usize)
        );
        assert_eq!(
            self.graph_hash,
            validated.component_graph.as_ref().map(|v| v.schema_hash)
        );
    }
}

fn replay_probe_corpus(
    column: &CoveredColumnInputs,
    trial: &[f64],
    caps: Option<&CoveredWaterCaps>,
) -> bool {
    let before = column.clone();
    let validated = ValidatedCoveredEvaluationInputs::try_new(column, caps).expect("corpus inputs");
    let base =
        ValidatedCoveredJacobianBase::evaluate(&validated, trial).expect("successful corpus base");
    let rollback = ReplayRollbackSnapshot::new(&base);
    let capture = base
        .capture
        .as_ref()
        .expect("actual successful production sweep capture");
    let (frozen_evaluation, frozen_capture) =
        evaluate_covered_column_with_replay_capture(&validated, trial, Some(&base.frozen), None)
            .expect("capture from canonical successful evaluation");
    replay_evaluation_bits(&base.evaluation, &frozen_evaluation);
    replay_capture_bits(capture, &frozen_capture);
    for coordinate in 0..trial.len() {
        let common = 10 * column.occupancies.len();
        let unit = if coordinate < common && coordinate % 10 < 4 {
            1000.0
        } else if coordinate == common + 1 {
            0.001
        } else {
            1.0
        };
        let h = f64::EPSILON.sqrt() * trial[coordinate].abs().max(unit);
        let mut minus = trial.to_vec();
        let mut plus = trial.to_vec();
        minus[coordinate] -= h;
        plus[coordinate] += h;
        let stencil = covered_finite_difference_stencil(
            trial,
            &minus,
            &plus,
            column.occupancies.len(),
            false,
        )
        .expect("unchanged canonical stencil");
        let mut actual_signs = Vec::new();
        let mut expected_signs = Vec::new();
        for probe in [&minus, &plus] {
            if !covered_trial_is_valid(probe, column.occupancies.len(), false) {
                actual_signs.push(None);
                expected_signs.push(None);
                continue;
            }
            let component =
                replay_pc1_expected_from_raw_trial(trial, coordinate, column.occupancies.len());
            begin_covered_jacobian_full_probe_audit();
            let actual = covered_jacobian_probe_residuals(&base, probe, coordinate);
            let complete_calls = take_covered_jacobian_full_probe_audit();
            let expected =
                evaluate_covered_column_validated(&validated, probe, Some(&base.frozen), None);
            if component {
                assert_eq!(
                    complete_calls, 0,
                    "replay cannot recover via complete evaluation"
                );
            } else if coordinate < 10 * column.occupancies.len() && coordinate % 10 >= 6 {
                assert_eq!(
                    complete_calls, 1,
                    "unproved leaf component must select complete before replay"
                );
            }
            match (actual, expected) {
                (Ok(actual), Ok(expected)) => {
                    replay_vector_bits(&actual, &expected.normalized_residuals);
                    if component {
                        begin_covered_leaf_trial_audit(false);
                        let (actual_eval, actual_capture) =
                            evaluate_covered_column_with_replay_capture(
                                &validated,
                                probe,
                                Some(&base.frozen),
                                Some((capture, coordinate)),
                            )
                            .expect("shared replay nodes");
                        let replay_leaf_calls = take_covered_leaf_trial_audit();
                        begin_covered_leaf_trial_audit(false);
                        let (full_eval, full_capture) =
                            evaluate_covered_column_with_replay_capture(
                                &validated,
                                probe,
                                Some(&base.frozen),
                                None,
                            )
                            .expect("complete nodes");
                        let complete_leaf_calls = take_covered_leaf_trial_audit();
                        assert!(
                            replay_leaf_calls < complete_leaf_calls,
                            "real leaf work must be eliminated"
                        );
                        replay_evaluation_bits(&actual_eval, &full_eval);
                        replay_capture_bits(&actual_capture, &full_capture);
                    }
                    actual_signs.push(Some(actual));
                    expected_signs.push(Some(expected.normalized_residuals));
                }
                (Err(a), Err(b)) => {
                    assert_eq!(a, b);
                    rollback.assert_unchanged(&base);
                    assert_eq!(*column, before);
                    return false; // Canonical sweep also ends at its first error.
                }
                other => panic!("dispatcher/complete outcome mismatch: {other:?}"),
            }
        }
        for row in 0..trial.len() {
            let value = |signs: &[Option<Vec<f64>>]| {
                covered_finite_difference_value(
                    stencil,
                    base.evaluation.normalized_residuals[row],
                    signs[0].as_ref().map(|v| v[row]),
                    signs[1].as_ref().map(|v| v[row]),
                    h,
                )
                .expect("canonical dense Jacobian entry")
            };
            assert_eq!(
                value(&actual_signs).to_bits(),
                value(&expected_signs).to_bits(),
                "dense Jacobian row {row}, coordinate {coordinate}"
            );
        }
    }
    assert_eq!(*column, before);
    true
}

fn replay_pc1_expected_from_raw_trial(
    trial: &[f64],
    coordinate: usize,
    occupancies: usize,
) -> bool {
    coordinate < 10 * occupancies
        && match coordinate % 10 {
            6 | 7 => trial[coordinate - 2].to_bits() == 1.0_f64.to_bits(),
            8 | 9 => true,
            _ => false,
        }
}

#[cfg(test)]
fn replay_pc1_mixed_selection_matrix(
    column: &CoveredColumnInputs,
    trial: &[f64],
    caps: Option<&CoveredWaterCaps>,
) {
    let adjacent = f64::from_bits(1.0_f64.to_bits() - 1);
    for phase in 0..2 {
        let mut mixed = trial.to_vec();
        for occupancy in 0..column.occupancies.len() {
            let sun_is_one = (occupancy + phase) % 2 == 0;
            mixed[10 * occupancy + 4] = if sun_is_one { 1.0 } else { adjacent };
            mixed[10 * occupancy + 5] = if sun_is_one { adjacent } else { 1.0 };
        }
        // The matrix changes/probes one class while the other class and
        // other occupancies retain opposite exact/adjacent beta states.
        // This rejects any-one-class or wrong-occupancy admission shortcuts.
        replay_pc1_selection_matrix(column, &mixed, caps);
    }
}

#[cfg(test)]
pub(super) fn replay_pc1_selection_matrix(
    column: &CoveredColumnInputs,
    trial: &[f64],
    caps: Option<&CoveredWaterCaps>,
) {
    // Prospective R-PC1 test: expected dispatch uses raw base bits, never
    // the production eligibility predicate or its labels.
    for occupancy in 0..column.occupancies.len() {
        for class in [6, 7] {
            for beta in [1.0, f64::from_bits(1.0_f64.to_bits() - 1), 0.0, 0.4] {
                let mut current = trial.to_vec();
                current[10 * occupancy + class - 2] = beta;
                let validated =
                    ValidatedCoveredEvaluationInputs::try_new(column, caps).expect("PC1 inputs");
                let base = ValidatedCoveredJacobianBase::evaluate(&validated, &current)
                    .expect("PC1 successful base");
                for component in [class, 8, 9] {
                    let coordinate = 10 * occupancy + component;
                    let h = f64::EPSILON.sqrt() * current[coordinate].abs().max(1.0);
                    let pair = CoveredCanonicalProbePair::new(&base, coordinate, h)
                        .expect("canonical signed pair");
                    let stencil = pair.stencil;
                    let expected_replay = component >= 8 || beta.to_bits() == 1.0_f64.to_bits();
                    for (capability, sign, displacement) in [
                        (pair.minus, CoveredProbeSign::Minus, -h),
                        (pair.plus, CoveredProbeSign::Plus, h),
                    ] {
                        let mut probe = current.clone();
                        probe[coordinate] += displacement;
                        let capability =
                            capability.expect("interior fixture admits both canonical signs");
                        begin_covered_jacobian_full_probe_audit();
                        begin_covered_leaf_trial_audit(false);
                        let observation =
                            crate::solver_mechanism_audit::begin_mechanism_audit(false, 0)
                                .expect("independent PC1 dispatch audit");
                        let actual = capability.evaluate_for(&base, coordinate, sign, h, stencil);
                        let observed = observation.finish().expect("completed PC1 dispatch audit");
                        let actual_calls = take_covered_leaf_trial_audit();
                        assert_eq!(
                            take_covered_jacobian_full_probe_audit(),
                            u32::from(!expected_replay)
                        );
                        assert_eq!(observed.probes.starts, 1);
                        assert_eq!(
                            observed.component_replay.starts,
                            u64::from(expected_replay),
                            "ineligible beta must start no replay"
                        );
                        assert_eq!(observed.complete.starts, u64::from(!expected_replay));
                        begin_covered_leaf_trial_audit(false);
                        let expected = evaluate_covered_column_validated(
                            &validated,
                            &probe,
                            Some(&base.frozen),
                            None,
                        )
                        .expect("PC1 complete counterpart");
                        let complete_calls = take_covered_leaf_trial_audit();
                        replay_residual_result_bits(
                            actual,
                            Ok(expected.normalized_residuals.clone()),
                        );
                        if expected_replay {
                            assert!(actual_calls < complete_calls);
                            let (actual, capture) = evaluate_covered_column_with_replay_capture(
                                &validated,
                                &probe,
                                Some(&base.frozen),
                                Some((base.capture.as_ref().expect("actual capture"), coordinate)),
                            )
                            .expect("proved PC1 full-node result");
                            replay_evaluation_bits(&actual, &expected);
                            if component < 8 {
                                let leaf = &capture.leaves[occupancy];
                                // Exact beta1 makes the canonical maximum reuse the
                                // successful affected current state at this probe T.
                                let (a, b) = (&leaf[class - 6], &leaf[class - 4]);
                                assert_eq!(a.gas_branch, b.gas_branch);
                                replay_scalar_bits!(a, b; surface_q, rs_s_m, ci_pa,
                            gross_assimilation_umol_co2_m2_leaf_s, net_assimilation_umol_co2_m2_leaf_s,
                            dark_respiration_umol_co2_m2_leaf_s);
                            } else {
                                for (a, b) in capture.leaves.iter().flatten().zip(
                                    base.capture
                                        .as_ref()
                                        .expect("base capture")
                                        .leaves
                                        .iter()
                                        .flatten(),
                                ) {
                                    assert_eq!(a.gas_branch, b.gas_branch);
                                    replay_scalar_bits!(a, b; surface_q, rs_s_m, ci_pa,
                                    gross_assimilation_umol_co2_m2_leaf_s, net_assimilation_umol_co2_m2_leaf_s,
                                    dark_respiration_umol_co2_m2_leaf_s);
                                }
                            }
                        } else {
                            assert_eq!(
                                actual_calls, complete_calls,
                                "unproved beta invokes only unchanged complete leaf calls"
                            );
                        }
                    }
                }
                replay_vector_bits(&base.trial, &current);
            }
        }
    }
}

#[cfg(test)]
fn replay_solve_trace_parity(
    column: &CoveredColumnInputs,
    trial: &[f64],
    caps: Option<&CoveredWaterCaps>,
) {
    begin_covered_solve_trace();
    begin_covered_jacobian_full_probe_audit();
    let actual = solve_covered_column(column, caps, trial.to_vec());
    let actual_count = take_covered_jacobian_full_probe_audit();
    let actual_trace = take_covered_solve_trace();
    begin_covered_solve_trace();
    begin_forced_complete_covered_jacobian_probe_audit();
    let expected = solve_covered_column(column, caps, trial.to_vec());
    let expected_count = take_covered_jacobian_full_probe_audit();
    let expected_trace = take_covered_solve_trace();
    assert_eq!(actual, expected);
    match (&actual, &expected) {
        (
            Ok(CoveredColumnSolveOutcome::Accepted(a)),
            Ok(CoveredColumnSolveOutcome::Accepted(b)),
        ) => {
            replay_vector_bits(&a.solution, &b.solution);
            replay_vector_bits(&a.soil_temperature_k, &b.soil_temperature_k);
            replay_evaluation_bits(&a.evaluation, &b.evaluation);
            replay_scalar_bits!(a, b; surface_enthalpy_j_m2_tile);
            replay_scalar_bits!(a.step_norms, b.step_norms; hydraulic_mm, beta, temperature_k, humidity_kg_kg, ci_pa);
            replay_scalar_bits!(a.ground_water, b.ground_water; law_kg_m2_tile_s,
                final_kg_m2_tile_s, request_kg_m2_stand_ground, finalized_use_kg_m2_stand_ground,
                condensation_credit_kg_m2_stand_ground);
            assert_eq!(
                a.ground_water
                    .authorization_kg_m2_stand_ground
                    .map(f64::to_bits),
                b.ground_water
                    .authorization_kg_m2_stand_ground
                    .map(f64::to_bits)
            );
            for (a, b) in a.root_water.iter().zip(&b.root_water) {
                replay_scalar_bits!(a, b; law_kg_m2_tile_s, final_kg_m2_tile_s,
                    request_kg_m2_stand_ground, finalized_use_kg_m2_stand_ground);
                assert_eq!(
                    a.authorization_kg_m2_stand_ground.map(f64::to_bits),
                    b.authorization_kg_m2_stand_ground.map(f64::to_bits)
                );
            }
        }
        (
            Ok(CoveredColumnSolveOutcome::Rejected(a)),
            Ok(CoveredColumnSolveOutcome::Rejected(b)),
        ) => {
            replay_vector_bits(&a.normalized_residuals, &b.normalized_residuals);
            replay_vector_bits(&a.failed_solution, &b.failed_solution);
            assert_eq!(
                a.pivot_magnitude.map(f64::to_bits),
                b.pivot_magnitude.map(f64::to_bits)
            );
            assert_eq!(
                a.matrix_norm.map(f64::to_bits),
                b.matrix_norm.map(f64::to_bits)
            );
            for (a, b) in a.ordered_residuals.iter().zip(&b.ordered_residuals) {
                replay_scalar_bits!(a, b; raw, scale, tolerance, normalized);
            }
            for (a, b) in [
                (a.step_norms.temperature_k, b.step_norms.temperature_k),
                (a.step_norms.humidity_kg_kg, b.step_norms.humidity_kg_kg),
                (a.step_norms.ci_pa, b.step_norms.ci_pa),
                (a.step_norms.hydraulic_mm, b.step_norms.hydraulic_mm),
                (a.step_norms.beta, b.step_norms.beta),
            ] {
                assert_eq!(a.map(f64::to_bits), b.map(f64::to_bits));
            }
        }
        _ => {} // Exact typed error/discrete outcome equality was checked above.
    }
    assert!(
        !actual_trace.is_empty(),
        "trace must observe the real canonical solve"
    );
    assert_eq!(
        actual_trace, expected_trace,
        "all matrix, RHS, linear and trajectory bits"
    );
    assert!(
        actual_count < expected_count,
        "authentic probes consumed replay/anchor paths"
    );
}

#[cfg(test)]
pub(super) fn assert_boundary_component_replay(column: &CoveredColumnInputs, trial: &[f64]) {
    assert!(
        replay_probe_corpus(column, trial, None),
        "authentic boundary full-sweep success"
    );
    if column
        .occupancies
        .iter()
        .all(|o| o.sun.leaf_area_m2_m2_tile == 0.0 && o.shade.leaf_area_m2_m2_tile == 0.0)
    {
        replay_inactive_conductance_first_errors(column, trial);
    }
}

#[cfg(test)]
fn replay_inactive_conductance_first_errors(column: &CoveredColumnInputs, trial: &[f64]) {
    // Positive finite external g0 has no physiological floor. Construct the
    // actual IEEE multiplication/division boundary, not a private-node poison.
    let g0 = 1.0e-316;
    let quantum = f64::from_bits(1);
    let pre_temperature_quanta = (g0 * 1.0e-6 * MOLAR_GAS_CONSTANT) / quantum;
    let lower_quanta = (pre_temperature_quanta * 300.0).floor();
    let rounding_temperature = (lower_quanta + 0.5) / pre_temperature_quanta;
    let temperature = rounding_temperature + 0.25 * f64::EPSILON.sqrt() * rounding_temperature;
    for occupancy in 0..column.occupancies.len() {
        for class in [6, 7] {
            let mut boundary = column.clone();
            boundary.pressure_pa = 2.0 * lower_quanta;
            boundary.occupancies[occupancy].g0_umol_m2_s = g0;
            let mut current = trial.to_vec();
            current[10 * occupancy + 6] = 310.0;
            current[10 * occupancy + 7] = 310.0;
            let coordinate = 10 * occupancy + class;
            current[coordinate] = temperature;
            current[coordinate - 2] = 1.0; // R-PC1: affected leaf replay has exact maximum proof.
            let before = boundary.clone();
            let validated = ValidatedCoveredEvaluationInputs::try_new(&boundary, None)
                .expect("inactive conductance inputs");
            let base = ValidatedCoveredJacobianBase::evaluate(&validated, &current)
                .expect("successful positive subnormal inactive conductance base");
            let mut probe = current.clone();
            probe[coordinate] -= f64::EPSILON.sqrt() * temperature;
            begin_covered_jacobian_full_probe_audit();
            begin_covered_leaf_trial_audit(false);
            let rollback = ReplayRollbackSnapshot::new(&base);
            let actual = covered_jacobian_probe_residuals(&base, &probe, coordinate);
            assert_eq!(take_covered_jacobian_full_probe_audit(), 0);
            assert_eq!(take_covered_leaf_trial_audit(), 1);
            let expected =
                evaluate_covered_column_validated(&validated, &probe, Some(&base.frozen), None)
                    .map(|v| v.normalized_residuals);
            assert_eq!(
                expected,
                Err(LandSurfaceEnergyError::ConstitutiveDomain(
                    "v10_zero_area_stomatal_conductance"
                ))
            );
            replay_residual_result_bits(actual, expected);
            rollback.assert_unchanged(&base);
            assert_eq!(boundary, before);
            replay_vector_bits(&base.trial, &current);
        }
    }
}

#[cfg(test)]
fn replay_selection_and_custody(column: &CoveredColumnInputs, trial: &[f64]) {
    let before = column.clone();
    let validated = ValidatedCoveredEvaluationInputs::try_new(column, None).expect("inputs");
    let base = ValidatedCoveredJacobianBase::evaluate(&validated, trial).expect("first base");
    let foreign = ValidatedCoveredJacobianBase::evaluate(&validated, trial)
        .expect("distinct real sweep base");
    let h = f64::EPSILON.sqrt() * trial[6].abs().max(1.0);
    let pair = CoveredCanonicalProbePair::new(&base, 6, h).expect("canonical pair");
    let stencil = pair.stencil;
    let capability = pair.plus.expect("interior plus capability");
    begin_covered_jacobian_full_probe_audit();
    assert_eq!(
        capability.evaluate_for(&foreign, 6, CoveredProbeSign::Plus, h, stencil),
        Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "covered_component_dependency_replay_integrity"
        ))
    );
    assert_eq!(take_covered_jacobian_full_probe_audit(), 0);
    for mismatch in 0..4 {
        let pair = CoveredCanonicalProbePair::new(&base, 6, h).expect("fresh canonical pair");
        let stencil = pair.stencil;
        let capability = pair.plus.expect("fresh plus capability");
        begin_covered_jacobian_full_probe_audit();
        begin_covered_leaf_trial_audit(false);
        let result = capability.evaluate_for(
            &base,
            if mismatch == 0 { 7 } else { 6 },
            if mismatch == 1 {
                CoveredProbeSign::Minus
            } else {
                CoveredProbeSign::Plus
            },
            if mismatch == 2 { h * 2.0 } else { h },
            if mismatch == 3 {
                CoveredFiniteDifferenceStencil::InwardFromLowerBound
            } else {
                stencil
            },
        );
        assert_eq!(
            result,
            Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "covered_component_dependency_replay_integrity"
            ))
        );
        assert_eq!(take_covered_jacobian_full_probe_audit(), 0);
        assert_eq!(
            take_covered_leaf_trial_audit(),
            0,
            "wrong binding rejects before any physical node"
        );
    }
    for (coordinate, probe) in [
        (6, trial[..trial.len() - 1].to_vec()),
        (trial.len(), trial.to_vec()),
        (6, {
            let mut p = trial.to_vec();
            p[6] += h;
            p[7] += h;
            p
        }),
        (6, {
            let mut p = trial.to_vec();
            p[6] = f64::NAN;
            p
        }),
    ] {
        begin_covered_jacobian_full_probe_audit();
        let rollback = ReplayRollbackSnapshot::new(&base);
        let actual = covered_jacobian_probe_residuals(&base, &probe, coordinate);
        assert_eq!(take_covered_jacobian_full_probe_audit(), 1);
        let expected =
            evaluate_covered_column_validated(&validated, &probe, Some(&base.frozen), None)
                .map(|v| v.normalized_residuals);
        replay_residual_result_bits(actual, expected);
        rollback.assert_unchanged(&base);
    }
    assert_eq!(*column, before);
}

#[cfg(test)]
fn replay_fixed_and_boundary_corpus(column: &CoveredColumnInputs, trial: &[f64]) {
    let potential = evaluate_covered_column(column, trial, None, None).expect("potential operands");
    let caps = CoveredWaterCaps {
        root: potential
            .occupancies
            .iter()
            .flat_map(|o| o.source_water.iter())
            .map(|s| {
                let request =
                    s.request_kg_m2_stand_ground / (column.tile_fraction * column.interval_s);
                (
                    (s.occupancy_id.clone(), s.layer_id.clone()),
                    SourceWaterCap {
                        request_rate_kg_m2_tile_s: request,
                        authorization_rate_kg_m2_tile_s: request,
                    },
                )
            })
            .collect(),
        ground: SourceWaterCap {
            request_rate_kg_m2_tile_s: potential.ground_water.request_kg_m2_stand_ground
                / (column.tile_fraction * column.interval_s),
            authorization_rate_kg_m2_tile_s: potential.ground_water.request_kg_m2_stand_ground
                / (column.tile_fraction * column.interval_s),
        },
    };
    assert!(
        replay_probe_corpus(column, trial, Some(&caps)),
        "fixed-final full-sweep corpus"
    );
    replay_solve_trace_parity(column, trial, Some(&caps));
    replay_pc1_selection_matrix(column, trial, Some(&caps));
    replay_pc1_mixed_selection_matrix(column, trial, Some(&caps));
    let mut exact_beta = trial.to_vec();
    for o in 0..column.occupancies.len() {
        exact_beta[10 * o + 4] = 1.0;
        exact_beta[10 * o + 5] = 1.0;
    }
    assert!(
        replay_probe_corpus(column, &exact_beta, None),
        "exact-beta boundary success"
    );
    for store in [0.0, 1.0] {
        let mut boundary = column.clone();
        for o in &mut boundary.occupancies {
            o.beginning_canopy_liquid_kg_m2_tile =
                store * o.liquid_capacity_kg_m2_plant * (o.lai + o.sai);
        }
        assert!(
            replay_probe_corpus(&boundary, trial, None),
            "dry/exact-capacity success"
        );
    }
    let mut dark = column.clone();
    for o in &mut dark.occupancies {
        o.sun.absorbed_par_w_m2_leaf = 0.0;
        o.shade.absorbed_par_w_m2_leaf = -0.0;
    }
    assert!(
        replay_probe_corpus(&dark, &exact_beta, None),
        "zero-PAR signed-zero success"
    );
}

#[cfg(test)]
fn replay_current_leaf_first_errors(column: &CoveredColumnInputs, trial: &[f64]) {
    // Canonical infinitesimal minus crosses the existing surface_vpd guard.
    // Both base and signed probe use real complete leaf physics; no fault seam.
    for occupancy in 0..column.occupancies.len() {
        for class in [6, 7] {
            let mut current = trial.to_vec();
            let coordinate = 10 * occupancy + class;
            let temperature: f64 = 300.0;
            let h = f64::EPSILON.sqrt() * temperature;
            for o in 0..column.occupancies.len() {
                current[10 * o + 6] = temperature + 1.0;
                current[10 * o + 7] = temperature + 1.0;
            }
            current[coordinate] = temperature;
            current[10 * column.occupancies.len() + 1] =
                canopy_saturation_q(temperature - 0.5 * h, column.pressure_pa)
                    .expect("source-real humidity");
            current[coordinate - 2] = 1.0;
            let before = column.clone();
            let validated =
                ValidatedCoveredEvaluationInputs::try_new(column, None).expect("leaf inputs");
            let base = ValidatedCoveredJacobianBase::evaluate(&validated, &current)
                .expect("source-real successful base immediately above VPD boundary");
            let mut probe = current.clone();
            probe[coordinate] -= h;
            assert!(covered_trial_is_valid(
                &probe,
                column.occupancies.len(),
                false
            ));
            begin_covered_jacobian_full_probe_audit();
            begin_covered_leaf_trial_audit(false);
            let rollback = ReplayRollbackSnapshot::new(&base);
            let actual = covered_jacobian_probe_residuals(&base, &probe, coordinate);
            let replay_leaf_calls = take_covered_leaf_trial_audit();
            assert_eq!(
                take_covered_jacobian_full_probe_audit(),
                0,
                "post-start error cannot fall back"
            );
            assert_eq!(
                replay_leaf_calls, 1,
                "first reachable leaf fails; no later leaf executes"
            );
            begin_covered_leaf_trial_audit(false);
            let expected =
                evaluate_covered_column_validated(&validated, &probe, Some(&base.frozen), None)
                    .map(|v| v.normalized_residuals);
            let complete_leaf_calls = take_covered_leaf_trial_audit();
            let previous_leaf_calls: u32 = base
                .evaluation
                .occupancies
                .iter()
                .take(occupancy)
                .enumerate()
                .map(|(rank, value)| {
                    2 + (0..2)
                        .filter(|leaf| {
                            current[10 * rank + 4 + *leaf].to_bits() != 1.0_f64.to_bits()
                                && !matches!(
                                    value.gas_branches[*leaf],
                                    V10LeafGasBranch::Inactive | V10LeafGasBranch::ExactZeroPar
                                )
                        })
                        .count() as u32
                })
                .sum();
            assert_eq!(
                complete_leaf_calls,
                previous_leaf_calls + (class - 5) as u32,
                "complete source-order position includes preceding occupancies and current sun"
            );
            assert_eq!(
                expected,
                Err(LandSurfaceEnergyError::ConstitutiveDomain("surface_vpd"))
            );
            replay_residual_result_bits(actual, expected);
            rollback.assert_unchanged(&base);
            assert_eq!(*column, before);
            replay_vector_bits(&base.trial, &current);
        }
    }
}

#[cfg(test)]
pub(super) fn replay_pc1_multirank_contracts(
    column: &CoveredColumnInputs,
    trial: &[f64],
    caps: &CoveredWaterCaps,
) {
    assert!(column.occupancies.len() >= 2, "authentic multirank fixture");
    replay_pc1_selection_matrix(column, trial, None);
    replay_pc1_selection_matrix(column, trial, Some(caps));
    replay_pc1_mixed_selection_matrix(column, trial, None);
    replay_pc1_mixed_selection_matrix(column, trial, Some(caps));
    replay_current_leaf_first_errors(column, trial);
    replay_zero_par_ci_first_errors(column, trial);
    replay_additional_leaf_boundary_errors(column, trial);
}

#[cfg(test)]
fn replay_zero_par_ci_first_errors(column: &CoveredColumnInputs, trial: &[f64]) {
    for occupancy in 0..column.occupancies.len() {
        for class in [6, 7] {
            let mut boundary = column.clone();
            let mut current = trial.to_vec();
            let coordinate = 10 * occupancy + class;
            let temperature: f64 = 300.0;
            let h = f64::EPSILON.sqrt() * temperature;
            let middle = temperature + 0.5 * h;
            let o = &mut boundary.occupancies[occupancy];
            let gs = o.g0_umol_m2_s * 1.0e-6 * MOLAR_GAS_CONSTANT * middle / column.pressure_pa;
            let resistance = 1.4 / o.gb_leaf_m_s + 1.6 / gs;
            let rd_factor =
                peaked(middle, 46_390.0, 150_650.0, 490.0).expect("canonical Rd factor");
            let leaf = if class == 6 { &mut o.sun } else { &mut o.shade };
            leaf.absorbed_par_w_m2_leaf = 0.0;
            // Solve the existing analytic zero-PAR input boundary at the
            // midpoint; the actual trial and probe still run canonical physics.
            leaf.rd25 = (column.pressure_pa - column.ca_pa)
                / (resistance * MOLAR_GAS_CONSTANT * middle * 1.0e-6 * rd_factor);
            current[coordinate] = temperature;
            current[coordinate - 2] = 1.0;
            let before = boundary.clone();
            let validated = ValidatedCoveredEvaluationInputs::try_new(&boundary, None)
                .expect("zero-PAR boundary inputs");
            let base = ValidatedCoveredJacobianBase::evaluate(&validated, &current)
                .expect("successful physical base before Ci reaches pressure");
            let mut probe = current.clone();
            probe[coordinate] += h;
            assert!(covered_trial_is_valid(
                &probe,
                boundary.occupancies.len(),
                false
            ));
            begin_covered_jacobian_full_probe_audit();
            begin_covered_leaf_trial_audit(false);
            let rollback = ReplayRollbackSnapshot::new(&base);
            let actual = covered_jacobian_probe_residuals(&base, &probe, coordinate);
            assert_eq!(take_covered_jacobian_full_probe_audit(), 0);
            assert_eq!(take_covered_leaf_trial_audit(), 1);
            let expected =
                evaluate_covered_column_validated(&validated, &probe, Some(&base.frozen), None)
                    .map(|v| v.normalized_residuals);
            assert_eq!(
                expected,
                Err(LandSurfaceEnergyError::ConstitutiveDomain(
                    "v10_zero_par_ci"
                ))
            );
            replay_residual_result_bits(actual, expected);
            rollback.assert_unchanged(&base);
            assert_eq!(boundary, before);
            replay_vector_bits(&base.trial, &current);
        }
    }
}

std::thread_local! {
    static RUNTIME_REPLAY_ORACLE: std::cell::Cell<Option<(bool, bool)>> = const { std::cell::Cell::new(None) };
}

#[cfg(test)]
fn replay_additional_leaf_boundary_errors(column: &CoveredColumnInputs, trial: &[f64]) {
    for occupancy in 0..column.occupancies.len() {
        for class in [6, 7] {
            for error in ["surface_co2", "v10_low_light_ci_dark", "peaked_response"] {
                let mut seed = column.clone();
                let mut current = trial.to_vec();
                let coordinate = 10 * occupancy + class;
                current[coordinate] = 300.0;
                current[coordinate - 2] = 1.0;
                let target = &mut seed.occupancies[occupancy];
                // Keep the preceding class an authentic dark branch, so a
                // source-position vector cannot fail in its predecessor.
                let other = if class == 6 {
                    &mut target.shade
                } else {
                    &mut target.sun
                };
                other.absorbed_par_w_m2_leaf = 0.0;
                other.rd25 = 1.0e-6;
                let leaf = if class == 6 {
                    &mut target.sun
                } else {
                    &mut target.shade
                };
                if error == "v10_low_light_ci_dark" {
                    leaf.absorbed_par_w_m2_leaf = 1.0e-3;
                }
                if error == "peaked_response" {
                    current[10 * occupancy + 6] = 300.0;
                    current[10 * occupancy + 7] = 300.0;
                    current[coordinate] = 310.0;
                    target.sun.absorbed_par_w_m2_leaf = 0.0;
                    target.shade.absorbed_par_w_m2_leaf = 0.0;
                    target.sun.vcmax25 = 1.0e-300;
                    target.shade.vcmax25 = 1.0e-300;
                }
                let candidate = |parameter: f64| {
                    let mut value = seed.clone();
                    let target = &mut value.occupancies[occupancy];
                    if error == "surface_co2" {
                        target.gb_leaf_m_s = 0.1 * (-parameter).exp();
                    } else if error == "v10_low_light_ci_dark" {
                        let leaf = if class == 6 {
                            &mut target.sun
                        } else {
                            &mut target.shade
                        };
                        leaf.rd25 = parameter.exp();
                    } else {
                        target.biochemical.ha_vcmax_j_mol = parameter;
                    }
                    value
                };
                // Inputs, not intermediate state, vary along this bracket.
                // Every trial invokes the unchanged complete column evaluator.
                let (mut low, mut high) = match error {
                    "surface_co2" => (0.0, 12.0),
                    "v10_low_light_ci_dark" => (0.0, 16.0),
                    _ => (65_330.0, 1.0e9),
                };
                assert!(
                    evaluate_covered_column(&candidate(low), &current, None, None).is_ok(),
                    "successful physical low bracket for {error}"
                );
                assert_eq!(
                    evaluate_covered_column(&candidate(high), &current, None, None),
                    Err(LandSurfaceEnergyError::ConstitutiveDomain(error)),
                    "typed high bracket for {error}"
                );
                for _ in 0..60 {
                    let middle = low + 0.5 * (high - low);
                    match evaluate_covered_column(&candidate(middle), &current, None, None) {
                        Ok(_) => low = middle,
                        Err(LandSurfaceEnergyError::ConstitutiveDomain(found))
                            if found == error =>
                        {
                            high = middle
                        }
                        other => panic!(
                            "unclassified source-real boundary while bracketing {error}: {other:?}"
                        ),
                    }
                }
                let boundary = candidate(low);
                let before = boundary.clone();
                let validated = ValidatedCoveredEvaluationInputs::try_new(&boundary, None)
                    .expect("boundary inputs");
                let base = ValidatedCoveredJacobianBase::evaluate(&validated, &current)
                    .expect("successful side of boundary");
                let h = f64::EPSILON.sqrt() * current[coordinate].abs().max(1.0);
                let mut crossed = false;
                for sign in [-1.0, 1.0] {
                    let mut probe = current.clone();
                    probe[coordinate] += sign * h;
                    assert!(covered_trial_is_valid(
                        &probe,
                        boundary.occupancies.len(),
                        false
                    ));
                    begin_covered_jacobian_full_probe_audit();
                    begin_covered_leaf_trial_audit(false);
                    let rollback = ReplayRollbackSnapshot::new(&base);
                    let actual = covered_jacobian_probe_residuals(&base, &probe, coordinate);
                    let calls = take_covered_leaf_trial_audit();
                    assert_eq!(take_covered_jacobian_full_probe_audit(), 0);
                    let expected = evaluate_covered_column_validated(
                        &validated,
                        &probe,
                        Some(&base.frozen),
                        None,
                    )
                    .map(|v| v.normalized_residuals);
                    if expected == Err(LandSurfaceEnergyError::ConstitutiveDomain(error)) {
                        crossed = true;
                        assert_eq!(calls, 1, "first reachable current leaf fails at {error}");
                    }
                    replay_residual_result_bits(actual, expected);
                    rollback.assert_unchanged(&base);
                }
                assert!(
                    crossed,
                    "must prove authentic canonical-probe crossability of {error}"
                );
                assert_eq!(boundary, before);
                replay_vector_bits(&base.trial, &current);
            }
        }
    }
}

#[cfg(test)]
#[allow(dead_code)] // Historical bounded investigation, not rerun by R-PC1 qualification.
fn replay_targeted_maximum_compensation_boundary(column: &CoveredColumnInputs, trial: &[f64]) {
    // One bounded investigation of the only remaining maximum-specific
    // numerical question: accepted low-light Brent state near An=0. Every
    // candidate is a complete column; error evidence requires a successful
    // base and its unmodified canonical temperature probe.
    let mut witnessed = 0_u32;
    let mut successful_bases = 0_u32;
    let mut bracket_errors = 0_u32;
    let mut rejected_bases = 0_u32;
    for class in [6, 7] {
        for beta in [0.1, 0.4, 0.8] {
            let mut seed = column.clone();
            let other = if class == 6 {
                &mut seed.occupancies[0].shade
            } else {
                &mut seed.occupancies[0].sun
            };
            other.absorbed_par_w_m2_leaf = 0.0;
            let mut middle_trial = trial.to_vec();
            middle_trial[class] = 300.0;
            middle_trial[class - 2] = beta;
            let with_par = |par: f64| {
                let mut inputs = seed.clone();
                let target = if class == 6 {
                    &mut inputs.occupancies[0].sun
                } else {
                    &mut inputs.occupancies[0].shade
                };
                target.absorbed_par_w_m2_leaf = par;
                inputs
            };
            let (mut low, mut high) = (0.001, 200.0);
            for _ in 0..56 {
                let middle = low + 0.5 * (high - low);
                match evaluate_covered_column(&with_par(middle), &middle_trial, None, None) {
                    Ok(value) => {
                        if value.occupancies[0].net_assimilation_umol_co2_m2_leaf_s[class - 6]
                            <= 0.0
                        {
                            low = middle;
                        } else {
                            high = middle;
                        }
                    }
                    Err(error) => {
                        bracket_errors += 1;
                        eprintln!(
                            "R_MAXIMUM_BRACKET_ERROR class={class} beta_bits={} par_bits={} error={error:?}",
                            beta.to_bits(),
                            middle.to_bits()
                        );
                        low = middle;
                        high = middle;
                        break;
                    }
                }
            }
            for par in [low, low + 0.5 * (high - low), high] {
                let boundary = with_par(par);
                let before = boundary.clone();
                let validated = ValidatedCoveredEvaluationInputs::try_new(&boundary, None)
                    .expect("compensation inputs");
                for offset in -5..=5 {
                    let mut current = middle_trial.clone();
                    current[class] += f64::from(offset) * f64::EPSILON.sqrt() * 300.0;
                    let base = match ValidatedCoveredJacobianBase::evaluate(&validated, &current) {
                        Ok(base) => base,
                        Err(error) => {
                            rejected_bases += 1;
                            eprintln!(
                                "R_MAXIMUM_BASE_REJECTED class={class} beta_bits={} par_bits={} base_t_bits={} error={error:?}",
                                beta.to_bits(),
                                par.to_bits(),
                                current[class].to_bits()
                            );
                            continue;
                        }
                    };
                    successful_bases += 1;
                    let h = f64::EPSILON.sqrt() * current[class].abs().max(1.0);
                    for sign in [-1.0, 1.0] {
                        let mut probe = current.clone();
                        probe[class] += sign * h;
                        begin_covered_jacobian_full_probe_audit();
                        begin_covered_leaf_trial_audit(false);
                        let rollback = ReplayRollbackSnapshot::new(&base);
                        let actual = covered_jacobian_probe_residuals(&base, &probe, class);
                        let actual_calls = take_covered_leaf_trial_audit();
                        assert_eq!(take_covered_jacobian_full_probe_audit(), 0);
                        begin_covered_leaf_trial_audit(false);
                        let expected = evaluate_covered_column_validated(
                            &validated,
                            &probe,
                            Some(&base.frozen),
                            None,
                        )
                        .map(|value| value.normalized_residuals);
                        let expected_calls = take_covered_leaf_trial_audit();
                        if let Err(
                            error @ LandSurfaceEnergyError::ConstitutiveDomain(
                                "v10_low_light_accepted_branch",
                            ),
                        ) = &actual
                        {
                            if actual_calls == 2 {
                                assert_eq!(
                                    expected_calls, 3,
                                    "two current classes precede the affected maximum"
                                );
                                witnessed += 1;
                                eprintln!(
                                    "R_MAXIMUM_COUNTEREXAMPLE class={class} beta_bits={} par_bits={} base_t_bits={} probe_t_bits={} error={error:?}",
                                    beta.to_bits(),
                                    par.to_bits(),
                                    current[class].to_bits(),
                                    probe[class].to_bits()
                                );
                            }
                        }
                        replay_residual_result_bits(actual, expected);
                        rollback.assert_unchanged(&base);
                        replay_vector_bits(&base.trial, &current);
                    }
                    assert_eq!(boundary, before);
                }
            }
        }
    }
    assert!(
        successful_bases > 0,
        "targeted route must exercise admitted complete bases"
    );
    eprintln!(
        "R_MAXIMUM_BOUNDED_RESULT successful_bases={successful_bases} rejected_bases={rejected_bases} bracket_errors={bracket_errors} maximum_only_first_errors={witnessed}"
    );
    // Zero witnesses is explicitly not a theorem of global noncrossability.
}

pub(super) fn begin_runtime_replay_oracle() {
    RUNTIME_REPLAY_ORACLE.with(|state| state.set(Some((false, false))));
}

pub(super) fn take_runtime_replay_oracle() -> (bool, bool) {
    RUNTIME_REPLAY_ORACLE.with(|state| {
        state
            .take()
            .expect("runtime oracle must be explicitly enabled")
    })
}

pub(super) fn observe_runtime_base(
    column: &CoveredColumnInputs,
    trial: &[f64],
    caps: Option<&CoveredWaterCaps>,
) {
    let Some((potential, fixed)) = RUNTIME_REPLAY_ORACLE.with(std::cell::Cell::get) else {
        return;
    };
    if column.occupancies.len() != 2
        || column.ground.soil_nodes.len() != 6
        || column.stage3_lower_boundary.is_none()
        || if caps.is_some() { fixed } else { potential }
    {
        return;
    }
    struct Restore {
        forced: bool,
        complete: Option<u32>,
        leaf: Option<(u32, bool)>,
    }
    impl Drop for Restore {
        fn drop(&mut self) {
            FORCE_COMPLETE_COVERED_JACOBIAN_PROBES.with(|v| v.set(self.forced));
            COVERED_JACOBIAN_FULL_PROBE_AUDIT.with(|v| v.set(self.complete));
            COVERED_LEAF_TRIAL_AUDIT.with(|v| v.set(self.leaf));
        }
    }
    let restore = Restore {
        forced: FORCE_COMPLETE_COVERED_JACOBIAN_PROBES.with(std::cell::Cell::get),
        complete: COVERED_JACOBIAN_FULL_PROBE_AUDIT.with(std::cell::Cell::get),
        leaf: COVERED_LEAF_TRIAL_AUDIT.with(std::cell::Cell::get),
    };
    let complete = replay_probe_corpus(column, trial, caps);
    drop(restore);
    if complete {
        RUNTIME_REPLAY_ORACLE
            .with(|state| state.set(Some((potential || caps.is_none(), fixed || caps.is_some()))));
    }
}
