fn require_growth_state_surface(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<HillslopeGrowthStateSurface, HillslopeGrowthBoundaryError> {
    let sumgdd = require_finite_state_value(phase, state_surface, PL_GROWTH_STATE_SUMGDD_SYMBOL)?;
    let vdmt = require_finite_state_value(phase, state_surface, PL_GROWTH_STATE_VDMT_SYMBOL)?;
    let cancov = require_finite_state_value(phase, state_surface, PL_GROWTH_STATE_CANCOV_SYMBOL)?;
    let lai = require_finite_state_value(phase, state_surface, PL_GROWTH_STATE_LAI_SYMBOL)?;
    let rtmass = require_finite_state_value(phase, state_surface, PL_GROWTH_STATE_RTMASS_SYMBOL)?;
    let rtd = require_finite_state_value(phase, state_surface, PL_GROWTH_STATE_RTD_SYMBOL)?;
    let hia = require_finite_state_value(phase, state_surface, PL_GROWTH_STATE_HIA_SYMBOL)?;

    for (symbol, value, minimum, maximum, reason) in [
        (
            PL_GROWTH_STATE_SUMGDD_SYMBOL,
            sumgdd,
            Some(0.0),
            None,
            "sumgdd must be non-negative",
        ),
        (
            PL_GROWTH_STATE_VDMT_SYMBOL,
            vdmt,
            Some(0.0),
            None,
            "vdmt must be non-negative",
        ),
        (
            PL_GROWTH_STATE_CANCOV_SYMBOL,
            cancov,
            Some(0.0),
            Some(0.999),
            "cancov must be within [0, 0.999]",
        ),
        (
            PL_GROWTH_STATE_LAI_SYMBOL,
            lai,
            Some(0.0),
            None,
            "lai must be non-negative",
        ),
        (
            PL_GROWTH_STATE_RTMASS_SYMBOL,
            rtmass,
            Some(0.0),
            None,
            "rtmass must be non-negative",
        ),
        (
            PL_GROWTH_STATE_RTD_SYMBOL,
            rtd,
            Some(0.0),
            None,
            "rtd must be non-negative",
        ),
        (
            PL_GROWTH_STATE_HIA_SYMBOL,
            hia,
            Some(0.0),
            Some(1.0),
            "hia must be within [0, 1]",
        ),
    ] {
        if let Some(minimum) = minimum {
            if value < minimum {
                return Err(
                    HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                        phase,
                        symbol: BoundarySymbol::from(symbol),
                        value,
                        reason,
                    },
                );
            }
        }
        if let Some(maximum) = maximum {
            if value > maximum {
                return Err(
                    HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                        phase,
                        symbol: BoundarySymbol::from(symbol),
                        value,
                        reason,
                    },
                );
            }
        }
    }

    Ok(HillslopeGrowthStateSurface {
        sumgdd,
        vdmt,
        cancov,
        lai,
        rtmass,
        rtd,
        hia,
    })
}

#[derive(Debug, Clone, Copy)]
struct GrowthEquationInputs {
    ws: f64,
    tmax: f64,
    tmin: f64,
    rad: f64,
    solthk: f64,
    btemp: f64,
    otemp: f64,
    gddmax: f64,
    dlai: f64,
    dropfc: f64,
    decfct: f64,
    spriod: f64,
    bb: f64,
    beinp: f64,
    extnct: f64,
    hi: f64,
    xmxlai: f64,
    rsr: f64,
    rtmmax: f64,
    rdmax: f64,
}

#[allow(clippy::too_many_lines)]
fn compute_equation_growth_state_surface(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    management_class: u8,
    state_before: HillslopeGrowthStateSurface,
) -> Result<HillslopeGrowthStateSurface, HillslopeGrowthBoundaryError> {
    let inputs = require_growth_equation_inputs(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        management_class,
    )?;

    let tave = f64::midpoint(inputs.tmax, inputs.tmin);
    let gdd = (tave - inputs.btemp).max(0.0);
    let sumgdd_next = (state_before.sumgdd + gdd).min(inputs.gddmax);
    let fphu = (sumgdd_next / inputs.gddmax).clamp(0.0, 1.0);

    let temp_ratio = (gdd / (inputs.otemp - inputs.btemp)).min(1.0);
    let temstr = (std::f64::consts::FRAC_PI_2 * temp_ratio)
        .sin()
        .clamp(0.0, 1.0);
    let reg = inputs.ws.min(temstr);

    let par = PL_GROWTH_PAR_RAD_SCALE
        * inputs.rad
        * (1.0 - (-inputs.extnct * (state_before.lai + PL_GROWTH_PAR_LAI_OFFSET)).exp());
    if !par.is_finite() || par < 0.0 {
        return Err(
            HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(PL_GROWTH_CLIMATE_RAD_SYMBOL),
                value: par,
                reason: "PAR expression must be finite and non-negative",
            },
        );
    }

    let ddm = PL_GROWTH_DDM_SCALE * inputs.beinp * par;
    let vdmt_growth = state_before.vdmt + ddm * reg;
    let mut vdmt_next = vdmt_growth;
    if fphu >= inputs.dlai && inputs.spriod > 0.0 {
        let biomass_decline = (1.0 - inputs.dropfc) / inputs.spriod;
        let canopy_decline = (1.0 - inputs.decfct) / inputs.spriod;
        if !(0.0..=1.0).contains(&biomass_decline) {
            return Err(
                HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                    phase,
                    symbol: BoundarySymbol::from(PL_GROWTH_PARAM_DROPFC_ROOT),
                    value: biomass_decline,
                    reason: "daily biomass senescence decline must be within [0, 1]",
                },
            );
        }
        if !(0.0..=1.0).contains(&canopy_decline) {
            return Err(
                HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                    phase,
                    symbol: BoundarySymbol::from(PL_GROWTH_PARAM_DECFCT_ROOT),
                    value: canopy_decline,
                    reason: "daily canopy senescence decline must be within [0, 1]",
                },
            );
        }
        vdmt_next = vdmt_growth * (1.0 - biomass_decline);
    }
    vdmt_next = vdmt_next.max(0.0);

    let hufh_denom = fphu + (6.5 - 10.0 * fphu).exp();
    if hufh_denom <= 0.0 || !hufh_denom.is_finite() {
        return Err(
            HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(PL_GROWTH_PARAM_HI_ROOT),
                value: hufh_denom,
                reason: "harvest-index denominator must be positive and finite",
            },
        );
    }
    let mut hia_next = inputs.hi * (fphu / hufh_denom);
    let water_stress_adjustment = if (0.3..0.9).contains(&fphu) {
        (std::f64::consts::FRAC_PI_2 * (fphu - 0.3) / 0.3).sin()
    } else {
        0.0
    };
    hia_next -=
        inputs.hi * (1.0 - 1.0 / (1.0 + 0.01 * water_stress_adjustment * (0.9 - inputs.ws)));
    hia_next = hia_next.clamp(0.0, inputs.hi);

    let canopy_biomass = if management_class == 2 {
        vdmt_next
    } else {
        vdmt_next * (1.0 - hia_next)
    };
    let cancov_raw = 1.0 - (-inputs.bb * canopy_biomass).exp();
    if !cancov_raw.is_finite() {
        return Err(
            HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(PL_GROWTH_STATE_CANCOV_SYMBOL),
                value: cancov_raw,
                reason: "canopy-cover equation output must be finite",
            },
        );
    }
    let mut cancov_next = cancov_raw.clamp(0.0, PL_GROWTH_CANCOV_MAX);
    if fphu >= inputs.dlai && inputs.spriod > 0.0 {
        let canopy_decline = (1.0 - inputs.decfct) / inputs.spriod;
        cancov_next = (cancov_next * (1.0 - canopy_decline)).clamp(0.0, PL_GROWTH_CANCOV_MAX);
    }

    let lai_next = if management_class == 2 {
        let denom =
            vdmt_next + PL_GROWTH_PERENNIAL_LAI_A * (-PL_GROWTH_PERENNIAL_LAI_B * vdmt_next).exp();
        if denom <= 0.0 || !denom.is_finite() {
            return Err(
                HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                    phase,
                    symbol: BoundarySymbol::from(PL_GROWTH_STATE_LAI_SYMBOL),
                    value: denom,
                    reason: "perennial LAI denominator must be positive and finite",
                },
            );
        }
        inputs.xmxlai * vdmt_next / denom
    } else {
        let veg = vdmt_next * (1.0 - hia_next);
        let denom = veg + PL_GROWTH_ANNUAL_LAI_A * (-PL_GROWTH_ANNUAL_LAI_B * veg).exp();
        if denom <= 0.0 || !denom.is_finite() {
            return Err(
                HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                    phase,
                    symbol: BoundarySymbol::from(PL_GROWTH_STATE_LAI_SYMBOL),
                    value: denom,
                    reason: "annual LAI denominator must be positive and finite",
                },
            );
        }
        inputs.xmxlai * veg / denom
    };
    if !lai_next.is_finite() || lai_next < 0.0 {
        return Err(
            HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(PL_GROWTH_STATE_LAI_SYMBOL),
                value: lai_next,
                reason: "LAI must remain finite and non-negative",
            },
        );
    }

    let rtmass_unclamped = state_before.rtmass + (vdmt_next - state_before.vdmt) * inputs.rsr;
    let rtmass_next = if management_class == 2 {
        rtmass_unclamped.clamp(0.0, inputs.rtmmax)
    } else {
        rtmass_unclamped.max(0.0)
    };

    let rtd_floor = inputs.rdmax
        * 0.5
        * (1.0
            + (PL_GROWTH_ROOT_DEPTH_CURVE_A * fphu / inputs.dlai - PL_GROWTH_ROOT_DEPTH_CURVE_B)
                .sin());
    let rtd_upper = inputs.rdmax.min(inputs.solthk);
    if rtd_upper <= 0.0 {
        return Err(
            HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(PL_GROWTH_PARAM_RDMAX_ROOT),
                value: rtd_upper,
                reason: "root-depth upper bound must be positive",
            },
        );
    }

    let rtd_candidate = if management_class == 2 {
        let growth_increment = ((rtmass_next - state_before.rtmass) / inputs.rtmmax) * inputs.rdmax;
        (state_before.rtd + growth_increment).max(rtd_floor)
    } else {
        rtd_floor
    };
    if !rtd_candidate.is_finite() || rtd_candidate < 0.0 {
        return Err(
            HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(PL_GROWTH_STATE_RTD_SYMBOL),
                value: rtd_candidate,
                reason: "root depth must remain finite and non-negative",
            },
        );
    }
    let rtd_next = rtd_candidate.min(rtd_upper);

    let state_after = HillslopeGrowthStateSurface {
        sumgdd: sumgdd_next,
        vdmt: vdmt_next,
        cancov: cancov_next,
        lai: lai_next,
        rtmass: rtmass_next,
        rtd: rtd_next,
        hia: hia_next,
    };

    validate_growth_state_surface(phase, state_after)
}

#[allow(clippy::too_many_lines)]
fn require_growth_equation_inputs(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    management_class: u8,
) -> Result<GrowthEquationInputs, HillslopeGrowthBoundaryError> {
    let ws = require_finite_state_value(phase, state_surface, PL_GROWTH_WATER_STRESS_SYMBOL)?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_WATER_STRESS_SYMBOL,
        ws,
        Some(0.0),
        Some(1.0),
        "water-stress carryover must be within [0, 1]",
    )?;

    let tmax = require_finite_state_value(phase, state_surface, PL_GROWTH_CLIMATE_TMAX_SYMBOL)?;
    let tmin = require_finite_state_value(phase, state_surface, PL_GROWTH_CLIMATE_TMIN_SYMBOL)?;
    let rad = require_finite_state_value(phase, state_surface, PL_GROWTH_CLIMATE_RAD_SYMBOL)?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_CLIMATE_RAD_SYMBOL,
        rad,
        Some(0.0),
        None,
        "radiation forcing must be non-negative",
    )?;
    let solthk = require_finite_state_value(phase, state_surface, PL_GROWTH_SOIL_DEPTH_SYMBOL)?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_SOIL_DEPTH_SYMBOL,
        solthk,
        Some(f64::EPSILON),
        None,
        "soil-depth envelope must be positive",
    )?;

    let btemp = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_BTEMP_ROOT,
    )?;
    let otemp = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_OTEMP_ROOT,
    )?;
    if otemp <= btemp {
        return Err(
            HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(PL_GROWTH_PARAM_OTEMP_ROOT),
                value: otemp,
                reason: "otemp must be greater than btemp",
            },
        );
    }

    let gddmax_projected = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_GDDMAX_ROOT,
    )?;
    let gddmax = resolve_effective_gddmax(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        management_class,
        btemp,
        gddmax_projected,
    )?;

    let dlai = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_DLAI_ROOT,
    )?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_PARAM_DLAI_ROOT,
        dlai,
        Some(f64::EPSILON),
        Some(1.0),
        "dlai must be within (0, 1]",
    )?;

    let dropfc = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_DROPFC_ROOT,
    )?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_PARAM_DROPFC_ROOT,
        dropfc,
        Some(0.0),
        Some(1.0),
        "dropfc must be within [0, 1]",
    )?;
    let decfct = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_DECFCT_ROOT,
    )?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_PARAM_DECFCT_ROOT,
        decfct,
        Some(0.0),
        Some(1.0),
        "decfct must be within [0, 1]",
    )?;

    let spriod = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_SPRIOD_ROOT,
    )?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_PARAM_SPRIOD_ROOT,
        spriod,
        Some(0.0),
        None,
        "spriod must be non-negative",
    )?;

    let bb = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_BB_ROOT,
    )?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_PARAM_BB_ROOT,
        bb,
        Some(0.0),
        None,
        "bb must be non-negative",
    )?;

    let beinp = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_BEINP_ROOT,
    )?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_PARAM_BEINP_ROOT,
        beinp,
        Some(0.0),
        None,
        "beinp must be non-negative",
    )?;

    let extnct = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_EXTNCT_ROOT,
    )?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_PARAM_EXTNCT_ROOT,
        extnct,
        Some(0.0),
        None,
        "extnct must be non-negative",
    )?;

    let hi = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_HI_ROOT,
    )?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_PARAM_HI_ROOT,
        hi,
        Some(0.0),
        Some(1.0),
        "hi must be within [0, 1]",
    )?;

    let xmxlai = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_XMXLAI_ROOT,
    )?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_PARAM_XMXLAI_ROOT,
        xmxlai,
        Some(0.0),
        None,
        "xmxlai must be non-negative",
    )?;

    let rsr = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_RSR_ROOT,
    )?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_PARAM_RSR_ROOT,
        rsr,
        Some(0.0),
        None,
        "rsr must be non-negative",
    )?;

    let rtmmax = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_RTMMAX_ROOT,
    )?;
    if management_class == 2 {
        validate_growth_state_range(
            phase,
            PL_GROWTH_PARAM_RTMMAX_ROOT,
            rtmmax,
            Some(f64::EPSILON),
            None,
            "rtmmax must be positive for perennial growth",
        )?;
    }

    let rdmax = require_slot_growth_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_GROWTH_PARAM_RDMAX_ROOT,
    )?;
    validate_growth_state_range(
        phase,
        PL_GROWTH_PARAM_RDMAX_ROOT,
        rdmax,
        Some(f64::EPSILON),
        None,
        "rdmax must be positive",
    )?;

    Ok(GrowthEquationInputs {
        ws,
        tmax,
        tmin,
        rad,
        solthk,
        btemp,
        otemp,
        gddmax,
        dlai,
        dropfc,
        decfct,
        spriod,
        bb,
        beinp,
        extnct,
        hi,
        xmxlai,
        rsr,
        rtmmax,
        rdmax,
    })
}

fn require_slot_growth_value(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    root: &str,
) -> Result<f64, HillslopeGrowthBoundaryError> {
    let symbol = pl_growth_slot_crop_symbol(root, slot_index, crop_slot_index);
    require_finite_state_value(phase, state_surface, symbol.as_str())
}

#[allow(clippy::too_many_arguments)]
fn resolve_effective_gddmax(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    management_class: u8,
    btemp: f64,
    gddmax_projected: f64,
) -> Result<f64, HillslopeGrowthBoundaryError> {
    if gddmax_projected > 0.0 {
        return Ok(gddmax_projected);
    }
    if gddmax_projected < 0.0 {
        return Err(
            HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(PL_GROWTH_PARAM_GDDMAX_ROOT),
                value: gddmax_projected,
                reason: "gddmax must be non-negative",
            },
        );
    }

    let obmaxt =
        require_monthly_temperature_vector(phase, state_surface, PL_GROWTH_CLIMATE_OBMAX_ROOT)?;
    let obmint =
        require_monthly_temperature_vector(phase, state_surface, PL_GROWTH_CLIMATE_OBMIN_ROOT)?;

    let gddmax_resolved = if management_class == 2 {
        legacy_gdmax_from_monthly(
            phase,
            1,
            PL_GROWTH_GDMAX_YEAR_END_DAY,
            btemp,
            &obmaxt,
            &obmint,
        )?
    } else {
        let jdplt_symbol = pl_growth_slot_crop_symbol("jdplt", slot_index, crop_slot_index);
        let jdharv_symbol = pl_growth_slot_crop_symbol("jdharv", slot_index, crop_slot_index);
        let jdplt = require_integral_state_value_in_range_for_growth(
            phase,
            state_surface,
            jdplt_symbol.as_str(),
            1,
            366,
        )?;
        let jdharv = require_integral_state_value_in_range_for_growth(
            phase,
            state_surface,
            jdharv_symbol.as_str(),
            1,
            366,
        )?;

        if jdharv > jdplt {
            legacy_gdmax_from_monthly(phase, jdplt, jdharv, btemp, &obmaxt, &obmint)?
        } else {
            legacy_gdmax_from_monthly(
                phase,
                jdplt,
                PL_GROWTH_GDMAX_YEAR_END_DAY,
                btemp,
                &obmaxt,
                &obmint,
            )? + legacy_gdmax_from_monthly(phase, 1, jdharv, btemp, &obmaxt, &obmint)?
        }
    };

    if !gddmax_resolved.is_finite() || gddmax_resolved <= 0.0 {
        return Err(
            HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(PL_GROWTH_PARAM_GDDMAX_ROOT),
                value: gddmax_resolved,
                reason: "resolved gddmax must be finite and positive",
            },
        );
    }

    Ok(gddmax_resolved)
}

fn require_monthly_temperature_vector(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    root: &str,
) -> Result<[f64; 12], HillslopeGrowthBoundaryError> {
    let mut monthly = [0.0; 12];
    for (index, month_value) in monthly.iter_mut().enumerate() {
        let month = index + 1;
        let symbol = format!("{root}_{month:04}");
        *month_value = require_finite_state_value(phase, state_surface, symbol.as_str())?;
    }
    Ok(monthly)
}

fn legacy_gdmax_from_monthly(
    phase: HillslopePhase,
    start_day: usize,
    end_day: usize,
    btemp: f64,
    obmaxt: &[f64; 12],
    obmint: &[f64; 12],
) -> Result<f64, HillslopeGrowthBoundaryError> {
    if start_day == 0 || end_day == 0 || start_day > end_day || end_day > 366 {
        return Err(
            HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(PL_GROWTH_PARAM_GDDMAX_ROOT),
                value: usize_to_f64_for_growth_error(end_day),
                reason: "legacy gdmax day-window must satisfy 1 <= start <= end <= 366",
            },
        );
    }

    let start_month = legacy_gdmax_month_for_day(start_day).ok_or(
        HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
            phase,
            symbol: BoundarySymbol::from(PL_GROWTH_PARAM_GDDMAX_ROOT),
            value: usize_to_f64_for_growth_error(start_day),
            reason: "legacy gdmax could not resolve start month",
        },
    )?;
    let end_month = legacy_gdmax_month_for_day(end_day).ok_or(
        HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
            phase,
            symbol: BoundarySymbol::from(PL_GROWTH_PARAM_GDDMAX_ROOT),
            value: usize_to_f64_for_growth_error(end_day),
            reason: "legacy gdmax could not resolve end month",
        },
    )?;

    let start_days = PL_GROWTH_GDMAX_MONTH_DAY_STARTS[start_month] - start_day + 1;
    let end_days = end_day - PL_GROWTH_GDMAX_MONTH_DAY_STARTS[end_month - 1];
    let start_days = day_count_to_f64_for_gdmax(phase, start_days)?;
    let end_days = day_count_to_f64_for_gdmax(phase, end_days)?;

    let mut sumgd = 0.0;

    let start_tave = f64::midpoint(obmaxt[start_month - 1], obmint[start_month - 1]);
    if start_tave > btemp {
        sumgd += (start_tave - btemp) * start_days;
    }

    for month in (start_month + 1)..end_month {
        let tave = f64::midpoint(obmaxt[month - 1], obmint[month - 1]);
        if tave > btemp {
            let month_days =
                day_count_to_f64_for_gdmax(phase, PL_GROWTH_GDMAX_MONTH_LENGTHS[month - 1])?;
            sumgd += (tave - btemp) * month_days;
        }
    }

    let end_tave = f64::midpoint(obmaxt[end_month - 1], obmint[end_month - 1]);
    if end_tave > btemp {
        sumgd += (end_tave - btemp) * end_days;
    }

    Ok(sumgd)
}

fn legacy_gdmax_month_for_day(day: usize) -> Option<usize> {
    if day == 0 || day > PL_GROWTH_GDMAX_MONTH_DAY_STARTS[12] {
        return None;
    }
    (1..=12).find(|month| day <= PL_GROWTH_GDMAX_MONTH_DAY_STARTS[*month])
}

fn day_count_to_f64_for_gdmax(
    phase: HillslopePhase,
    day_count: usize,
) -> Result<f64, HillslopeGrowthBoundaryError> {
    let day_count_u16 = u16::try_from(day_count).map_err(|_| {
        HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
            phase,
            symbol: BoundarySymbol::from(PL_GROWTH_PARAM_GDDMAX_ROOT),
            value: usize_to_f64_for_growth_error(day_count),
            reason: "legacy gdmax day-count exceeds supported conversion range",
        }
    })?;
    Ok(f64::from(day_count_u16))
}

fn usize_to_f64_for_growth_error(value: usize) -> f64 {
    u32::try_from(value).map_or(f64::NAN, f64::from)
}

fn validate_growth_state_surface(
    phase: HillslopePhase,
    state: HillslopeGrowthStateSurface,
) -> Result<HillslopeGrowthStateSurface, HillslopeGrowthBoundaryError> {
    for (symbol, value, minimum, maximum, reason) in [
        (
            PL_GROWTH_STATE_SUMGDD_SYMBOL,
            state.sumgdd,
            Some(0.0),
            None,
            "sumgdd must be non-negative",
        ),
        (
            PL_GROWTH_STATE_VDMT_SYMBOL,
            state.vdmt,
            Some(0.0),
            None,
            "vdmt must be non-negative",
        ),
        (
            PL_GROWTH_STATE_CANCOV_SYMBOL,
            state.cancov,
            Some(0.0),
            Some(PL_GROWTH_CANCOV_MAX),
            "cancov must be within [0, 0.999]",
        ),
        (
            PL_GROWTH_STATE_LAI_SYMBOL,
            state.lai,
            Some(0.0),
            None,
            "lai must be non-negative",
        ),
        (
            PL_GROWTH_STATE_RTMASS_SYMBOL,
            state.rtmass,
            Some(0.0),
            None,
            "rtmass must be non-negative",
        ),
        (
            PL_GROWTH_STATE_RTD_SYMBOL,
            state.rtd,
            Some(0.0),
            None,
            "rtd must be non-negative",
        ),
        (
            PL_GROWTH_STATE_HIA_SYMBOL,
            state.hia,
            Some(0.0),
            Some(1.0),
            "hia must be within [0, 1]",
        ),
    ] {
        validate_growth_state_range(phase, symbol, value, minimum, maximum, reason)?;
    }

    Ok(state)
}

fn validate_growth_state_range(
    phase: HillslopePhase,
    symbol: &str,
    value: f64,
    minimum: Option<f64>,
    maximum: Option<f64>,
    reason: &'static str,
) -> Result<(), HillslopeGrowthBoundaryError> {
    if !value.is_finite() {
        return Err(
            HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value,
                reason: "state value must be finite",
            },
        );
    }
    if let Some(minimum) = minimum {
        if value < minimum {
            return Err(
                HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                    phase,
                    symbol: BoundarySymbol::from(symbol),
                    value,
                    reason,
                },
            );
        }
    }
    if let Some(maximum) = maximum {
        if value > maximum {
            return Err(
                HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                    phase,
                    symbol: BoundarySymbol::from(symbol),
                    value,
                    reason,
                },
            );
        }
    }
    Ok(())
}

fn reset_growth_state_surface(_state: HillslopeGrowthStateSurface) -> HillslopeGrowthStateSurface {
    HillslopeGrowthStateSurface {
        sumgdd: 0.0,
        vdmt: 0.0,
        cancov: 0.0,
        lai: 0.0,
        rtmass: 0.0,
        rtd: 0.0,
        hia: 0.0,
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn require_integral_state_value_for_growth(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
) -> Result<usize, HillslopeGrowthBoundaryError> {
    let value = require_finite_state_value(phase, state_surface, symbol)?;
    let rounded = value.round();
    if (value - rounded).abs() > MANAGEMENT_CLASS_EPSILON {
        return Err(
            HillslopeGrowthBoundaryError::NonIntegralRequiredStateSymbol {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value,
            },
        );
    }
    if rounded < 0.0 {
        return Err(
            HillslopeGrowthBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value,
                reason: "integral growth symbol must be non-negative",
            },
        );
    }
    Ok(rounded as usize)
}

fn require_integral_state_value_in_range_for_growth(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
    min_allowed: usize,
    max_allowed: usize,
) -> Result<usize, HillslopeGrowthBoundaryError> {
    let value = require_integral_state_value_for_growth(phase, state_surface, symbol)?;
    if value < min_allowed || value > max_allowed {
        return Err(HillslopeGrowthBoundaryError::StateSymbolValueOutOfRange {
            phase,
            symbol: BoundarySymbol::from(symbol),
            value,
            min_allowed,
            max_allowed,
        });
    }
    Ok(value)
}

fn usize_to_u16_for_growth(
    phase: HillslopePhase,
    symbol: BoundarySymbol,
    value: usize,
) -> Result<u16, HillslopeGrowthBoundaryError> {
    u16::try_from(value).map_err(
        |_| HillslopeGrowthBoundaryError::StateSymbolValueOutOfRange {
            phase,
            symbol,
            value,
            min_allowed: 0,
            max_allowed: usize::from(u16::MAX),
        },
    )
}

fn usize_to_u8_for_growth(
    phase: HillslopePhase,
    symbol: BoundarySymbol,
    value: usize,
) -> Result<u8, HillslopeGrowthBoundaryError> {
    u8::try_from(value).map_err(
        |_| HillslopeGrowthBoundaryError::StateSymbolValueOutOfRange {
            phase,
            symbol,
            value,
            min_allowed: 0,
            max_allowed: usize::from(u8::MAX),
        },
    )
}

fn require_ordering_flag(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
    expected: f64,
) -> Result<f64, HillslopeGrowthBoundaryError> {
    let observed = require_finite_state_value(phase, state_surface, symbol)?;
    if (observed - expected).abs() > ORDER_FLAG_EPSILON {
        return Err(HillslopeGrowthBoundaryError::InvalidOrderingFlagValue {
            phase,
            symbol: BoundarySymbol::from(symbol),
            observed,
            expected,
        });
    }

    Ok(observed)
}

fn require_finite_state_value(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
) -> Result<f64, HillslopeGrowthBoundaryError> {
    let symbol_key = BoundarySymbol::from(symbol);
    let value = state_surface
        .get(&symbol_key)
        .ok_or_else(
            || HillslopeGrowthBoundaryError::MissingRequiredStateSymbol {
                phase,
                symbol: symbol_key.clone(),
            },
        )?
        .as_f64();

    if !value.is_finite() {
        return Err(HillslopeGrowthBoundaryError::NonFiniteRequiredStateSymbol {
            phase,
            symbol: symbol_key,
            value,
        });
    }

    Ok(value)
}

fn normalize_management_class(
    phase: HillslopePhase,
    value: f64,
    symbol: &str,
) -> Result<u8, HillslopeGrowthBoundaryError> {
    let rounded = value.round();
    if (value - rounded).abs() > MANAGEMENT_CLASS_EPSILON {
        return Err(HillslopeGrowthBoundaryError::UnsupportedManagementClass {
            phase,
            symbol: BoundarySymbol::from(symbol),
            value,
        });
    }
    if !(1.0..=3.0).contains(&rounded) {
        return Err(HillslopeGrowthBoundaryError::UnsupportedManagementClass {
            phase,
            symbol: BoundarySymbol::from(symbol),
            value,
        });
    }
    if (rounded - 1.0).abs() <= MANAGEMENT_CLASS_EPSILON {
        return Ok(1);
    }
    if (rounded - 2.0).abs() <= MANAGEMENT_CLASS_EPSILON {
        return Ok(2);
    }
    if (rounded - 3.0).abs() <= MANAGEMENT_CLASS_EPSILON {
        return Ok(3);
    }

    Err(HillslopeGrowthBoundaryError::UnsupportedManagementClass {
        phase,
        symbol: BoundarySymbol::from(symbol),
        value,
    })
}
