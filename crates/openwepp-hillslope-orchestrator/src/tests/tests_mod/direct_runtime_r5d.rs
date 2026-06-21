use super::direct_runtime_test_lock;
use crate::{
    DIRECT_R5D_ANNUAL_GROWTH_PHASE_SPAN_COUNT, DIRECT_R5D_ANNUAL_GROWTH_SPAN,
    DIRECT_R5D_PERENNIAL_GROWTH_PHASE_SPAN_COUNT, DIRECT_R5D_PERENNIAL_GROWTH_SPAN, DirectDayFrame,
    DirectGrowthAction, DirectGrowthActiveContext, DirectGrowthDownstreamOperands,
    DirectGrowthInputs, DirectGrowthShadowProjection, DirectGrowthState, DirectGrowthStateSurface,
    DirectPhaseKind, DirectRunIdentity, DirectRuntimeError, reset_direct_runtime_audit_counters,
};

const EPS: f64 = 1.0e-12;

#[test]
fn r5d_annual_growth_phase_computes_mutates_downstream_shadow_and_r4n_context() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    assert_eq!(
        DIRECT_R5D_ANNUAL_GROWTH_SPAN,
        [DirectPhaseKind::AnnualGrowthTransition]
    );
    assert_eq!(
        &[
            DirectPhaseKind::ResiduePartitionTransition,
            DirectPhaseKind::AnnualGrowthTransition,
            DirectPhaseKind::PerennialGrowthTransition,
        ],
        &DirectPhaseKind::ORDERED[3..6]
    );

    let mut day = r5d_day_after_residue_partition();
    let inputs = annual_active_inputs();
    day.annual_growth_inputs = inputs;

    let report = day
        .run_r5d_annual_growth_phase()
        .expect("valid annual growth should execute");
    let expected_state = expected_growth_state(&inputs, false);
    let expected_operands = DirectGrowthDownstreamOperands::from(expected_state);
    let expected_shadow = DirectGrowthShadowProjection::from_operands(0, 0, expected_operands);

    assert_eq!(day.annual_growth_inputs, inputs);
    assert_growth_state_close(day.annual_growth, expected_state);
    assert_growth_operands_close(day.annual_growth_downstream_operands, expected_operands);
    assert_growth_shadow_close(day.annual_growth_shadow_projection, expected_shadow);
    assert_eq!(
        report.phase_count,
        DIRECT_R5D_ANNUAL_GROWTH_PHASE_SPAN_COUNT
    );
    assert_eq!(report.phase_entry_count, 1);
    assert_eq!(report.direct_compute_count, 1);
    assert_eq!(report.state_mutation_count, 1);
    assert_eq!(report.downstream_operand_count, 1);
    assert_eq!(report.shadow_projection_count, 1);
    assert_eq!(report.compatibility_edge_invocation_count, 0);
    assert_growth_shadow_value_close(report.growth_shadow_projection, expected_shadow);

    assert_close(
        day.evapotranspiration_compute_inputs.leaf_area_index,
        expected_state.state_after.leaf_area_index,
    );
    assert_close(
        day.evapotranspiration_compute_inputs.canopy_cover_fraction,
        expected_state.state_after.canopy_cover_fraction,
    );
    assert_close(
        day.evapotranspiration_compute_inputs.root_depth_m,
        expected_state.state_after.root_depth_m,
    );
    assert!(
        day.evapotranspiration_compute_inputs
            .growth_context_required
    );
    assert_r5d_growth_anti_aliases(expected_state, &day);

    let audit = crate::direct_runtime_audit_snapshot();
    assert_eq!(audit.phase_span_runs, 5);
    assert_eq!(audit.direct_phase_entries, 5);
    assert_eq!(audit.compatibility_edge_invocations, 0);
}

#[test]
fn r5d_perennial_growth_phase_supports_grazing_after_annual_phase_identity() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    assert_eq!(
        DIRECT_R5D_PERENNIAL_GROWTH_SPAN,
        [DirectPhaseKind::PerennialGrowthTransition]
    );

    let mut day = r5d_day_after_residue_partition();
    day.run_r5d_annual_growth_phase()
        .expect("annual inactive span should execute before perennial");
    let inputs = perennial_grazing_inputs();
    day.perennial_growth_inputs = inputs;

    let report = day
        .run_r5d_perennial_growth_phase()
        .expect("valid perennial grazing growth should execute");
    let expected_state = expected_growth_state(&inputs, true);

    assert_eq!(
        report.phase_count,
        DIRECT_R5D_PERENNIAL_GROWTH_PHASE_SPAN_COUNT
    );
    assert_eq!(report.compatibility_edge_invocation_count, 0);
    assert_growth_state_close(day.perennial_growth, expected_state);
    assert_eq!(
        day.perennial_growth.active_action,
        DirectGrowthAction::Grazing
    );
    assert_close(
        day.evapotranspiration_compute_inputs.root_depth_m,
        expected_state.state_after.root_depth_m,
    );
}

#[test]
fn r5d_growth_resolves_annual_rotation_boundary_gddmax_sentinel() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let mut day = r5d_day_after_residue_partition();
    let mut inputs = annual_active_inputs();
    inputs.active_context = DirectGrowthActiveContext::AnnualOrFallow {
        active_slot_index: 3,
        active_crop_slot_index: 1,
        runtime_day_of_year: 20,
    };
    inputs.planting_day = 300;
    inputs.harvest_day = 100;
    inputs.gddmax = 0.0;
    inputs.monthly_temperature_max_c = [20.0; 12];
    inputs.monthly_temperature_min_c = [10.0; 12];
    day.annual_growth_inputs = inputs;

    day.run_r5d_annual_growth_phase()
        .expect("winter annual sentinel gddmax should resolve through monthly climate");

    assert!(day.annual_growth.gddmax_effective > 0.0);
    assert!(day.annual_growth.state_after.sumgdd > inputs.state_before.sumgdd);
    assert_eq!(
        day.annual_growth_shadow_projection
            .expect("annual growth should project")
            .active_context,
        inputs.active_context
    );
}

#[test]
fn r5d_growth_rejects_missing_upstream_bad_contexts_and_invalid_domains() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");
    let mut missing_upstream_day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    missing_upstream_day.annual_growth_inputs = annual_active_inputs();
    assert_eq!(
        missing_upstream_day
            .run_r5d_annual_growth_phase()
            .expect_err("annual growth should require R5C residue partition"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "R5C residue partition transition"
        }
    );

    let mut missing_context_day = r5d_day_after_residue_partition();
    missing_context_day.annual_growth_inputs = DirectGrowthInputs {
        active_context: DirectGrowthActiveContext::Missing,
        ..annual_active_inputs()
    };
    assert_eq!(
        missing_context_day
            .run_r5d_annual_growth_phase()
            .expect_err("missing active context should fail closed"),
        DirectRuntimeError::DirectDomainViolation {
            field: "growth.active_context"
        }
    );

    let mut ambiguous_context_day = r5d_day_after_residue_partition();
    ambiguous_context_day.annual_growth_inputs = DirectGrowthInputs {
        active_context: DirectGrowthActiveContext::Ambiguous,
        ..annual_active_inputs()
    };
    assert_eq!(
        ambiguous_context_day
            .run_r5d_annual_growth_phase()
            .expect_err("ambiguous active context should fail closed"),
        DirectRuntimeError::DirectDomainViolation {
            field: "growth.active_context"
        }
    );

    let mut annual_grazing_day = r5d_day_after_residue_partition();
    annual_grazing_day.annual_growth_inputs = DirectGrowthInputs {
        active_action: DirectGrowthAction::Grazing,
        ..annual_active_inputs()
    };
    assert_eq!(
        annual_grazing_day
            .run_r5d_annual_growth_phase()
            .expect_err("annual growth should reject grazing action"),
        DirectRuntimeError::DirectDomainViolation {
            field: "growth.active_action"
        }
    );

    let mut nonfinite_state_day = r5d_day_after_residue_partition();
    nonfinite_state_day.annual_growth_inputs = DirectGrowthInputs {
        state_before: DirectGrowthStateSurface {
            leaf_area_index: f64::NAN,
            ..annual_active_inputs().state_before
        },
        ..annual_active_inputs()
    };
    assert_eq!(
        nonfinite_state_day
            .run_r5d_annual_growth_phase()
            .expect_err("nonfinite plant state should fail closed"),
        DirectRuntimeError::NonFiniteDirectValue {
            field: "growth.state_before"
        }
    );

    let mut perennial_before_annual_day = r5d_day_after_residue_partition();
    perennial_before_annual_day.perennial_growth_inputs = perennial_grazing_inputs();
    assert_eq!(
        perennial_before_annual_day
            .run_r5d_perennial_growth_phase()
            .expect_err("perennial growth should require annual phase order"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "R5D annual growth transition"
        }
    );
}

#[test]
fn r5d_r4n_required_growth_context_fails_closed_when_absent() {
    let _audit_guard = direct_runtime_test_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_direct_runtime_audit_counters();

    let mut missing_growth_day = r5d_day_after_residue_partition();
    missing_growth_day
        .run_r4c_storage_input_span()
        .expect("R4C should seed storage input");
    missing_growth_day
        .run_r4m_percolation_span()
        .expect("R4M should seed ET layers");
    missing_growth_day
        .evapotranspiration_compute_inputs
        .growth_context_required = true;
    assert_eq!(
        missing_growth_day
            .run_r4n_surface_et_span()
            .expect_err("R4N should require growth context when requested"),
        DirectRuntimeError::MissingDirectUpstream {
            upstream: "R5D growth transition"
        }
    );

    let mut growth_day = r5d_day_after_residue_partition();
    growth_day
        .run_r5d_annual_growth_phase()
        .expect("annual inactive growth should project");
    growth_day
        .run_r5d_perennial_growth_phase()
        .expect("perennial inactive growth should project");
    growth_day
        .run_r4c_storage_input_span()
        .expect("R4C should seed storage input");
    growth_day
        .run_r4m_percolation_span()
        .expect("R4M should seed ET layers");
    growth_day
        .run_r4n_surface_et_span()
        .expect("R4N should accept projected inactive growth context");
}

fn r5d_day_after_residue_partition() -> DirectDayFrame {
    let identity =
        DirectRunIdentity::new(7, 2637, 1, 1).expect("valid direct span identity should construct");
    let mut day =
        DirectDayFrame::seed(identity, 0, 0).expect("valid direct day frame should construct");
    day.forcing.precipitation_m = 0.125;
    day.water.soil_water_m = 1.25;
    day.storage_reconciliation_inputs.closure_tolerance_m = 1.0e-12;
    day.run_r5b_normalization_phase()
        .expect("R5B normalization should pass before R5D");
    day.run_r5b_storage_bounds_phase()
        .expect("R5B storage bounds should pass before R5D");
    day.run_r5c_decomposition_phase()
        .expect("R5C decomposition should pass before R5D");
    day.run_r5c_residue_partition_phase()
        .expect("R5C residue partition should pass before R5D");
    day
}

fn annual_active_inputs() -> DirectGrowthInputs {
    DirectGrowthInputs {
        active_context: DirectGrowthActiveContext::AnnualOrFallow {
            active_slot_index: 1,
            active_crop_slot_index: 2,
            runtime_day_of_year: 150,
        },
        active_action: DirectGrowthAction::Cut,
        state_before: DirectGrowthStateSurface {
            sumgdd: 100.0,
            live_biomass_kg_m2: 0.30,
            canopy_cover_fraction: 0.20,
            leaf_area_index: 0.80,
            root_mass_kg_m2: 0.05,
            root_depth_m: 0.20,
            harvest_index: 0.02,
        },
        planting_day: 100,
        harvest_day: 250,
        stop_day: 0,
        water_stress: 0.70,
        temperature_max_c: 25.0,
        temperature_min_c: 15.0,
        radiation_mj_m2: 18.0,
        monthly_temperature_max_c: [20.0; 12],
        monthly_temperature_min_c: [10.0; 12],
        soil_depth_m: 1.20,
        btemp: 5.0,
        otemp: 25.0,
        gddmax: 1000.0,
        dlai: 0.80,
        dropfc: 0.50,
        decfct: 0.60,
        spriod: 20.0,
        bb: 1.80,
        beinp: 30.0,
        extnct: 0.65,
        hi: 0.50,
        xmxlai: 5.0,
        rsr: 0.30,
        rtmmax: 0.0,
        rdmax: 1.0,
        et_demand_m: 0.006,
        residue_interception_m: 0.001,
        plant_tolerance: 0.25,
    }
}

fn perennial_grazing_inputs() -> DirectGrowthInputs {
    DirectGrowthInputs {
        active_context: DirectGrowthActiveContext::Perennial {
            active_slot_index: 4,
            active_crop_slot_index: 1,
            runtime_day_of_year: 210,
        },
        active_action: DirectGrowthAction::Grazing,
        state_before: DirectGrowthStateSurface {
            sumgdd: 220.0,
            live_biomass_kg_m2: 0.42,
            canopy_cover_fraction: 0.30,
            leaf_area_index: 1.20,
            root_mass_kg_m2: 0.35,
            root_depth_m: 0.50,
            harvest_index: 0.0,
        },
        planting_day: 0,
        harvest_day: 0,
        stop_day: 320,
        water_stress: 0.85,
        temperature_max_c: 27.0,
        temperature_min_c: 13.0,
        radiation_mj_m2: 20.0,
        monthly_temperature_max_c: [21.0; 12],
        monthly_temperature_min_c: [9.0; 12],
        soil_depth_m: 1.50,
        btemp: 4.0,
        otemp: 24.0,
        gddmax: 1200.0,
        dlai: 0.75,
        dropfc: 0.55,
        decfct: 0.65,
        spriod: 25.0,
        bb: 1.60,
        beinp: 28.0,
        extnct: 0.70,
        hi: 0.40,
        xmxlai: 4.5,
        rsr: 0.45,
        rtmmax: 1.20,
        rdmax: 1.30,
        et_demand_m: 0.007,
        residue_interception_m: 0.002,
        plant_tolerance: 0.30,
    }
}

fn expected_growth_state(inputs: &DirectGrowthInputs, perennial: bool) -> DirectGrowthState {
    let state_after = expected_growth_surface(inputs, perennial);
    let tave = f64::midpoint(inputs.temperature_max_c, inputs.temperature_min_c);
    let gdd = (tave - inputs.btemp).max(0.0);
    let gddmax_effective = if inputs.gddmax > 0.0 {
        inputs.gddmax
    } else if perennial {
        expected_legacy_gdmax(inputs, 1, 365)
    } else if inputs.harvest_day > inputs.planting_day {
        expected_legacy_gdmax(
            inputs,
            usize::from(inputs.planting_day),
            usize::from(inputs.harvest_day),
        )
    } else {
        expected_legacy_gdmax(inputs, usize::from(inputs.planting_day), 365)
            + expected_legacy_gdmax(inputs, 1, usize::from(inputs.harvest_day))
    };
    let fphu = (state_after.sumgdd / gddmax_effective).clamp(0.0, 1.0);
    let temperature_stress = (std::f64::consts::FRAC_PI_2
        * (gdd / (inputs.otemp - inputs.btemp)).min(1.0))
    .sin()
    .clamp(0.0, 1.0);
    let par = 0.02092
        * inputs.radiation_mj_m2
        * (1.0 - (-inputs.extnct * (inputs.state_before.leaf_area_index + 0.05)).exp());

    DirectGrowthState {
        active_context: inputs.active_context,
        active_action: inputs.active_action,
        state_before: inputs.state_before,
        state_after,
        water_stress: inputs.water_stress,
        temperature_stress,
        regulation_factor: inputs.water_stress.min(temperature_stress),
        gdd,
        gddmax_effective,
        fphu,
        par,
        daily_biomass_increment_kg_m2: 0.0001 * inputs.beinp * par,
        senescence_biomass_decline_fraction: 0.0,
        senescence_canopy_decline_fraction: 0.0,
        et_demand_m: inputs.et_demand_m,
        residue_interception_m: inputs.residue_interception_m,
        plant_tolerance: inputs.plant_tolerance,
    }
}

fn expected_growth_surface(
    inputs: &DirectGrowthInputs,
    perennial: bool,
) -> DirectGrowthStateSurface {
    let tave = f64::midpoint(inputs.temperature_max_c, inputs.temperature_min_c);
    let gdd = (tave - inputs.btemp).max(0.0);
    let gddmax_effective = if inputs.gddmax > 0.0 {
        inputs.gddmax
    } else if perennial {
        expected_legacy_gdmax(inputs, 1, 365)
    } else if inputs.harvest_day > inputs.planting_day {
        expected_legacy_gdmax(
            inputs,
            usize::from(inputs.planting_day),
            usize::from(inputs.harvest_day),
        )
    } else {
        expected_legacy_gdmax(inputs, usize::from(inputs.planting_day), 365)
            + expected_legacy_gdmax(inputs, 1, usize::from(inputs.harvest_day))
    };
    let sumgdd_next = (inputs.state_before.sumgdd + gdd).min(gddmax_effective);
    let fphu = (sumgdd_next / gddmax_effective).clamp(0.0, 1.0);
    let temstr = (std::f64::consts::FRAC_PI_2 * (gdd / (inputs.otemp - inputs.btemp)).min(1.0))
        .sin()
        .clamp(0.0, 1.0);
    let reg = inputs.water_stress.min(temstr);
    let par = 0.02092
        * inputs.radiation_mj_m2
        * (1.0 - (-inputs.extnct * (inputs.state_before.leaf_area_index + 0.05)).exp());
    let vdmt_next = inputs.state_before.live_biomass_kg_m2 + 0.0001 * inputs.beinp * par * reg;
    let hufh_denom = fphu + (6.5 - 10.0 * fphu).exp();
    let mut hia_next = inputs.hi * (fphu / hufh_denom);
    hia_next = hia_next.clamp(0.0, inputs.hi);
    let canopy_biomass = if perennial {
        vdmt_next
    } else {
        vdmt_next * (1.0 - hia_next)
    };
    let cancov_next = (1.0 - (-inputs.bb * canopy_biomass).exp()).clamp(0.0, 0.999);
    let lai_next = if perennial {
        let denom = vdmt_next + 0.2756 * (-13.6 * vdmt_next).exp();
        inputs.xmxlai * vdmt_next / denom
    } else {
        let vegetative_biomass = vdmt_next * (1.0 - hia_next);
        let denom = vegetative_biomass + 0.5512 * (-6.8 * vegetative_biomass).exp();
        inputs.xmxlai * vegetative_biomass / denom
    };
    let rtmass_unclamped = inputs.state_before.root_mass_kg_m2
        + (vdmt_next - inputs.state_before.live_biomass_kg_m2) * inputs.rsr;
    let rtmass_next = if perennial {
        rtmass_unclamped.clamp(0.0, inputs.rtmmax)
    } else {
        rtmass_unclamped.max(0.0)
    };
    let rtd_floor = inputs.rdmax * 0.5 * (1.0 + (3.03 * fphu / inputs.dlai - 1.47).sin());
    let rtd_candidate = if perennial {
        let growth_increment =
            ((rtmass_next - inputs.state_before.root_mass_kg_m2) / inputs.rtmmax) * inputs.rdmax;
        (inputs.state_before.root_depth_m + growth_increment).max(rtd_floor)
    } else {
        rtd_floor
    };
    let rtd_next = rtd_candidate.min(inputs.rdmax.min(inputs.soil_depth_m));

    DirectGrowthStateSurface {
        sumgdd: sumgdd_next,
        live_biomass_kg_m2: vdmt_next,
        canopy_cover_fraction: cancov_next,
        leaf_area_index: lai_next,
        root_mass_kg_m2: rtmass_next,
        root_depth_m: rtd_next,
        harvest_index: hia_next,
    }
}

fn expected_legacy_gdmax(inputs: &DirectGrowthInputs, start_day: usize, end_day: usize) -> f64 {
    const STARTS: [usize; 13] = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366];
    const LENGTHS: [usize; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let start_month = (1..=12)
        .find(|month| start_day <= STARTS[*month])
        .expect("fixture start day should resolve to month");
    let end_month = (1..=12)
        .find(|month| end_day <= STARTS[*month])
        .expect("fixture end day should resolve to month");
    let mut sumgd = 0.0;
    let start_days = STARTS[start_month] - start_day + 1;
    let end_days = end_day - STARTS[end_month - 1];
    let start_tave = f64::midpoint(
        inputs.monthly_temperature_max_c[start_month - 1],
        inputs.monthly_temperature_min_c[start_month - 1],
    );
    if start_tave > inputs.btemp {
        sumgd += (start_tave - inputs.btemp) * day_count_to_f64(start_days);
    }
    for month in (start_month + 1)..end_month {
        let tave = f64::midpoint(
            inputs.monthly_temperature_max_c[month - 1],
            inputs.monthly_temperature_min_c[month - 1],
        );
        if tave > inputs.btemp {
            sumgd += (tave - inputs.btemp) * day_count_to_f64(LENGTHS[month - 1]);
        }
    }
    let end_tave = f64::midpoint(
        inputs.monthly_temperature_max_c[end_month - 1],
        inputs.monthly_temperature_min_c[end_month - 1],
    );
    if end_tave > inputs.btemp {
        sumgd += (end_tave - inputs.btemp) * day_count_to_f64(end_days);
    }
    sumgd
}

fn day_count_to_f64(day_count: usize) -> f64 {
    f64::from(u16::try_from(day_count).expect("fixture day count fits u16"))
}

fn assert_r5d_growth_anti_aliases(expected_state: DirectGrowthState, day: &DirectDayFrame) {
    assert_ne!(
        expected_state.state_after.leaf_area_index.to_bits(),
        expected_state.state_before.leaf_area_index.to_bits()
    );
    assert_ne!(
        expected_state.state_after.root_depth_m.to_bits(),
        expected_state.et_demand_m.to_bits()
    );
    assert_ne!(
        expected_state.state_after.canopy_cover_fraction.to_bits(),
        day.publication.evapotranspiration_m.to_bits()
    );
    assert_ne!(
        expected_state.state_after.live_biomass_kg_m2.to_bits(),
        day.residue_partition.total_residue_kg_m2.to_bits()
    );
}

fn assert_growth_state_close(observed: DirectGrowthState, expected: DirectGrowthState) {
    assert_eq!(observed.active_context, expected.active_context);
    assert_eq!(observed.active_action, expected.active_action);
    assert_growth_surface_close(observed.state_before, expected.state_before);
    assert_growth_surface_close(observed.state_after, expected.state_after);
    assert_close(observed.water_stress, expected.water_stress);
    assert_close(observed.temperature_stress, expected.temperature_stress);
    assert_close(observed.regulation_factor, expected.regulation_factor);
    assert_close(observed.gdd, expected.gdd);
    assert_close(observed.gddmax_effective, expected.gddmax_effective);
    assert_close(observed.fphu, expected.fphu);
    assert_close(observed.par, expected.par);
    assert_close(
        observed.daily_biomass_increment_kg_m2,
        expected.daily_biomass_increment_kg_m2,
    );
}

fn assert_growth_surface_close(
    observed: DirectGrowthStateSurface,
    expected: DirectGrowthStateSurface,
) {
    assert_close(observed.sumgdd, expected.sumgdd);
    assert_close(observed.live_biomass_kg_m2, expected.live_biomass_kg_m2);
    assert_close(
        observed.canopy_cover_fraction,
        expected.canopy_cover_fraction,
    );
    assert_close(observed.leaf_area_index, expected.leaf_area_index);
    assert_close(observed.root_mass_kg_m2, expected.root_mass_kg_m2);
    assert_close(observed.root_depth_m, expected.root_depth_m);
    assert_close(observed.harvest_index, expected.harvest_index);
}

fn assert_growth_operands_close(
    observed: DirectGrowthDownstreamOperands,
    expected: DirectGrowthDownstreamOperands,
) {
    assert_eq!(observed.active_context, expected.active_context);
    assert_eq!(observed.active_action, expected.active_action);
    assert_growth_surface_close(observed.state_after, expected.state_after);
    assert_close(observed.root_depth_m, expected.root_depth_m);
    assert_close(observed.leaf_area_index, expected.leaf_area_index);
    assert_close(
        observed.canopy_cover_fraction,
        expected.canopy_cover_fraction,
    );
    assert_close(observed.water_stress, expected.water_stress);
    assert_close(observed.et_demand_m, expected.et_demand_m);
    assert_close(
        observed.residue_interception_m,
        expected.residue_interception_m,
    );
    assert_close(observed.plant_tolerance, expected.plant_tolerance);
}

fn assert_growth_shadow_close(
    observed: Option<DirectGrowthShadowProjection>,
    expected: DirectGrowthShadowProjection,
) {
    assert_growth_shadow_value_close(
        observed.expect("growth should produce shadow projection"),
        expected,
    );
}

fn assert_growth_shadow_value_close(
    observed: DirectGrowthShadowProjection,
    expected: DirectGrowthShadowProjection,
) {
    assert_eq!(observed.lane_index, expected.lane_index);
    assert_eq!(observed.day_index, expected.day_index);
    assert_eq!(observed.active_context, expected.active_context);
    assert_eq!(observed.active_action, expected.active_action);
    assert_growth_surface_close(observed.state_after, expected.state_after);
    assert_close(observed.root_depth_m, expected.root_depth_m);
    assert_close(observed.leaf_area_index, expected.leaf_area_index);
    assert_close(
        observed.canopy_cover_fraction,
        expected.canopy_cover_fraction,
    );
    assert_close(observed.water_stress, expected.water_stress);
    assert_close(observed.et_demand_m, expected.et_demand_m);
    assert_close(
        observed.residue_interception_m,
        expected.residue_interception_m,
    );
    assert_close(observed.plant_tolerance, expected.plant_tolerance);
}

fn assert_close(observed: f64, expected: f64) {
    assert!(
        (observed - expected).abs() <= EPS,
        "observed {observed}, expected {expected}"
    );
}
