#[derive(Debug, Clone, Copy)]
struct DecompositionEquationInputs {
    ws: f64,
    tmax: f64,
    tmin: f64,
    prcp: f64,
    oratea: f64,
    orater: f64,
}

#[allow(clippy::too_many_lines)]
fn compute_equation_decomposition_seed_surface(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    control: HillslopeDecompositionTransitionControl,
    sumrtm_seed: f64,
    sumsrm_seed: f64,
) -> Result<(f64, f64), HillslopeDecompositionBoundaryError> {
    validate_decomposition_state_range(
        phase,
        PL_DECOMP_SUMRTM_SEED_SYMBOL,
        sumrtm_seed,
        Some(0.0),
        None,
        "sumrtm_seed must be non-negative",
    )?;
    validate_decomposition_state_range(
        phase,
        PL_DECOMP_SUMSRM_SEED_SYMBOL,
        sumsrm_seed,
        Some(0.0),
        None,
        "sumsrm_seed must be non-negative",
    )?;

    let inputs =
        require_decomposition_equation_inputs(phase, state_surface, slot_index, crop_slot_index)?;
    let tave = f64::midpoint(inputs.tmax, inputs.tmin);

    let tmpfac = if tave <= -PL_DECOMP_TEMP_ATEMP || tave >= PL_DECOMP_TEMP_ACTIVE_UPPER {
        0.0
    } else {
        let t1 = (tave + PL_DECOMP_TEMP_ATEMP).powi(2);
        let numerator = t1 * (2.0 * PL_DECOMP_TEMP_T2 - t1);
        let denominator = PL_DECOMP_TEMP_T2.powi(2);
        if denominator <= 0.0 {
            return Err(
                HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                    phase,
                    symbol: BoundarySymbol::from(PL_DECOMP_CLIMATE_TMAX_SYMBOL),
                    value: denominator,
                    reason: "temperature-factor denominator must be positive",
                },
            );
        }
        numerator / denominator
    };
    validate_decomposition_state_range(
        phase,
        PL_DECOMP_CLIMATE_TMAX_SYMBOL,
        tmpfac,
        Some(0.0),
        Some(1.0),
        "temperature decomposition factor must be within [0, 1]",
    )?;

    let swatfc = if tave <= 0.0 {
        0.0
    } else if inputs.prcp < PL_DECOMP_STANDING_RAIN_SATURATION {
        inputs.prcp / PL_DECOMP_STANDING_RAIN_SATURATION
    } else {
        1.0
    };
    validate_decomposition_state_range(
        phase,
        PL_DECOMP_CLIMATE_PRCP_SYMBOL,
        swatfc,
        Some(0.0),
        Some(1.0),
        "standing-residue water factor must be within [0, 1]",
    )?;

    let fwatfc = inputs.ws.clamp(0.0, 1.0);
    let _senvin = tmpfac.min(swatfc);
    let envinx = tmpfac.min(fwatfc);
    validate_decomposition_state_range(
        phase,
        PL_GROWTH_WATER_STRESS_SYMBOL,
        envinx,
        Some(0.0),
        Some(1.0),
        "environmental decomposition factor must be within [0, 1]",
    )?;

    let surface_exponent = -envinx * inputs.oratea;
    let root_exponent = -envinx * inputs.orater;
    if !surface_exponent.is_finite() {
        return Err(
            HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(PL_DECOMP_PARAM_ORATEA_ROOT),
                value: surface_exponent,
                reason: "surface decomposition exponent must be finite",
            },
        );
    }
    if !root_exponent.is_finite() {
        return Err(
            HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(PL_DECOMP_PARAM_ORATER_ROOT),
                value: root_exponent,
                reason: "root decomposition exponent must be finite",
            },
        );
    }

    let surface_decay = surface_exponent.exp();
    let root_decay = root_exponent.exp();
    validate_decomposition_state_range(
        phase,
        PL_DECOMP_PARAM_ORATEA_ROOT,
        surface_decay,
        Some(0.0),
        Some(1.0),
        "surface decomposition decay factor must be within [0, 1]",
    )?;
    validate_decomposition_state_range(
        phase,
        PL_DECOMP_PARAM_ORATER_ROOT,
        root_decay,
        Some(0.0),
        Some(1.0),
        "root decomposition decay factor must be within [0, 1]",
    )?;

    let mut sumsrm_next = sumsrm_seed * surface_decay;
    let mut sumrtm_next = sumrtm_seed * root_decay;

    match control {
        HillslopeDecompositionTransitionControl::Annual(annual_control) => {
            match annual_control.active_action {
                HillslopeAnnualDecompositionAction::Burn => {
                    sumsrm_next *= 1.0 - annual_control.fbrnog;
                }
                HillslopeAnnualDecompositionAction::Remove => {
                    sumsrm_next *= 1.0 - annual_control.frmove;
                }
                HillslopeAnnualDecompositionAction::Cut => {
                    let transfer = sumsrm_next * annual_control.frcut;
                    sumsrm_next -= transfer;
                    sumrtm_next += transfer;
                }
                HillslopeAnnualDecompositionAction::None
                | HillslopeAnnualDecompositionAction::Herbicide
                | HillslopeAnnualDecompositionAction::Silage => {}
            }
        }
        HillslopeDecompositionTransitionControl::Perennial(perennial_control) => {
            if let HillslopePerennialDecompositionAction::Grazing { cycle_index } =
                perennial_control.active_action
            {
                let Some(active_cycle) = perennial_control.active_grazing_cycle else {
                    return Err(
                        HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                            phase,
                            symbol: BoundarySymbol::from("active_grazing_cycle"),
                            value: f64::from(cycle_index),
                            reason: "grazing action requires active_grazing_cycle payload instance",
                        },
                    );
                };
                if active_cycle.cycle_index != cycle_index {
                    return Err(
                        HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                            phase,
                            symbol: BoundarySymbol::from("active_grazing_cycle"),
                            value: f64::from(active_cycle.cycle_index),
                            reason: "active grazing cycle index must match active action",
                        },
                    );
                }
                validate_decomposition_state_range(
                    phase,
                    "digest",
                    active_cycle.digest,
                    Some(0.0),
                    Some(1.0),
                    "grazing digest fraction must be within [0, 1]",
                )?;
                sumsrm_next *= 1.0 - active_cycle.digest;
            }
        }
    }

    validate_decomposition_state_range(
        phase,
        PL_DECOMP_SUMRTM_SEED_SYMBOL,
        sumrtm_next,
        Some(0.0),
        None,
        "sumrtm_seed must remain non-negative",
    )?;
    validate_decomposition_state_range(
        phase,
        PL_DECOMP_SUMSRM_SEED_SYMBOL,
        sumsrm_next,
        Some(0.0),
        None,
        "sumsrm_seed must remain non-negative",
    )?;

    Ok((sumrtm_next, sumsrm_next))
}

fn require_decomposition_equation_inputs(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
) -> Result<DecompositionEquationInputs, HillslopeDecompositionBoundaryError> {
    let ws = require_finite_state_value_for_decomposition(phase, state_surface, "Ws")?;
    validate_decomposition_state_range(
        phase,
        PL_GROWTH_WATER_STRESS_SYMBOL,
        ws,
        Some(0.0),
        Some(1.0),
        "water-stress carryover must be within [0, 1]",
    )?;

    let tmax = require_finite_state_value_for_decomposition(
        phase,
        state_surface,
        PL_DECOMP_CLIMATE_TMAX_SYMBOL,
    )?;
    let tmin = require_finite_state_value_for_decomposition(
        phase,
        state_surface,
        PL_DECOMP_CLIMATE_TMIN_SYMBOL,
    )?;
    let prcp = require_finite_state_value_for_decomposition(
        phase,
        state_surface,
        PL_DECOMP_CLIMATE_PRCP_SYMBOL,
    )?;
    validate_decomposition_state_range(
        phase,
        PL_DECOMP_CLIMATE_PRCP_SYMBOL,
        prcp,
        Some(0.0),
        None,
        "precipitation forcing must be non-negative",
    )?;

    let annual_decay_rate = require_slot_decomposition_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_DECOMP_PARAM_ORATEA_ROOT,
    )?;
    validate_decomposition_state_range(
        phase,
        PL_DECOMP_PARAM_ORATEA_ROOT,
        annual_decay_rate,
        Some(0.0),
        None,
        "oratea must be non-negative",
    )?;

    let root_decay_rate = require_slot_decomposition_value(
        phase,
        state_surface,
        slot_index,
        crop_slot_index,
        PL_DECOMP_PARAM_ORATER_ROOT,
    )?;
    validate_decomposition_state_range(
        phase,
        PL_DECOMP_PARAM_ORATER_ROOT,
        root_decay_rate,
        Some(0.0),
        None,
        "orater must be non-negative",
    )?;

    Ok(DecompositionEquationInputs {
        ws,
        tmax,
        tmin,
        prcp,
        oratea: annual_decay_rate,
        orater: root_decay_rate,
    })
}

fn require_slot_decomposition_value(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    root: &str,
) -> Result<f64, HillslopeDecompositionBoundaryError> {
    let symbol = pl_decomp_slot_crop_symbol(root, slot_index, crop_slot_index);
    require_finite_state_value_for_decomposition(phase, state_surface, symbol.as_str())
}

fn validate_decomposition_state_range(
    phase: HillslopePhase,
    symbol: &str,
    value: f64,
    minimum: Option<f64>,
    maximum: Option<f64>,
    reason: &'static str,
) -> Result<(), HillslopeDecompositionBoundaryError> {
    if !value.is_finite() {
        return Err(
            HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
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
                HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
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
                HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
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

fn require_ordering_flag_for_decomposition(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
    expected: f64,
) -> Result<f64, HillslopeDecompositionBoundaryError> {
    let observed = require_finite_state_value_for_decomposition(phase, state_surface, symbol)?;
    if (observed - expected).abs() > ORDER_FLAG_EPSILON {
        return Err(
            HillslopeDecompositionBoundaryError::InvalidOrderingFlagValue {
                phase,
                symbol: BoundarySymbol::from(symbol),
                observed,
                expected,
            },
        );
    }

    Ok(observed)
}

fn require_finite_state_value_for_decomposition(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
) -> Result<f64, HillslopeDecompositionBoundaryError> {
    let symbol_key = BoundarySymbol::from(symbol);
    let value = state_surface
        .get(&symbol_key)
        .ok_or_else(
            || HillslopeDecompositionBoundaryError::MissingRequiredStateSymbol {
                phase,
                symbol: symbol_key.clone(),
            },
        )?
        .as_f64();

    if !value.is_finite() {
        return Err(
            HillslopeDecompositionBoundaryError::NonFiniteRequiredStateSymbol {
                phase,
                symbol: symbol_key,
                value,
            },
        );
    }

    Ok(value)
}

fn normalize_management_class_for_decomposition(
    phase: HillslopePhase,
    value: f64,
    symbol: &str,
) -> Result<u8, HillslopeDecompositionBoundaryError> {
    let rounded = value.round();
    if (value - rounded).abs() > MANAGEMENT_CLASS_EPSILON {
        return Err(
            HillslopeDecompositionBoundaryError::UnsupportedManagementClass {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value,
            },
        );
    }
    if !(1.0..=3.0).contains(&rounded) {
        return Err(
            HillslopeDecompositionBoundaryError::UnsupportedManagementClass {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value,
            },
        );
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

    Err(
        HillslopeDecompositionBoundaryError::UnsupportedManagementClass {
            phase,
            symbol: BoundarySymbol::from(symbol),
            value,
        },
    )
}

fn usize_to_u16_for_decomposition(
    phase: HillslopePhase,
    symbol: BoundarySymbol,
    value: usize,
) -> Result<u16, HillslopeDecompositionBoundaryError> {
    u16::try_from(value).map_err(|_| {
        HillslopeDecompositionBoundaryError::StateSymbolValueOutOfRange {
            phase,
            symbol,
            value,
            min_allowed: 0,
            max_allowed: usize::from(u16::MAX),
        }
    })
}

fn usize_to_u8_for_decomposition(
    phase: HillslopePhase,
    symbol: BoundarySymbol,
    value: usize,
) -> Result<u8, HillslopeDecompositionBoundaryError> {
    u8::try_from(value).map_err(|_| {
        HillslopeDecompositionBoundaryError::StateSymbolValueOutOfRange {
            phase,
            symbol,
            value,
            min_allowed: 0,
            max_allowed: usize::from(u8::MAX),
        }
    })
}

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn require_integral_state_value_for_decomposition(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
    min_allowed: usize,
    max_allowed: usize,
) -> Result<usize, HillslopeDecompositionBoundaryError> {
    let value = require_finite_state_value_for_decomposition(phase, state_surface, symbol)?;
    let rounded = value.round();
    if (value - rounded).abs() > MANAGEMENT_CLASS_EPSILON {
        return Err(
            HillslopeDecompositionBoundaryError::NonIntegralRequiredStateSymbol {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value,
            },
        );
    }

    let min_f64 = min_allowed as f64;
    let max_f64 = max_allowed as f64;
    if rounded < min_f64 || rounded > max_f64 {
        return Err(
            HillslopeDecompositionBoundaryError::StateSymbolValueOutOfRange {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value: rounded as usize,
                min_allowed,
                max_allowed,
            },
        );
    }

    Ok(rounded as usize)
}

fn require_day_state_value_for_decomposition(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
    allow_zero: bool,
) -> Result<usize, HillslopeDecompositionBoundaryError> {
    let min_allowed = usize::from(!allow_zero);
    require_integral_state_value_for_decomposition(phase, state_surface, symbol, min_allowed, 366)
}

fn require_fraction_state_value_for_decomposition(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
) -> Result<f64, HillslopeDecompositionBoundaryError> {
    let value = require_finite_state_value_for_decomposition(phase, state_surface, symbol)?;
    if !(0.0..=1.0).contains(&value) {
        return Err(
            HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value,
                reason: "expected fraction in [0,1]",
            },
        );
    }
    Ok(value)
}

fn require_zero_state_value_for_decomposition(
    phase: HillslopePhase,
    symbol: &str,
    value: f64,
    reason: &'static str,
) -> Result<(), HillslopeDecompositionBoundaryError> {
    if value.abs() > ORDER_FLAG_EPSILON {
        return Err(
            HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value,
                reason,
            },
        );
    }
    Ok(())
}

fn parse_indexed_suffix_for_decomposition(suffix: &str) -> Option<usize> {
    if suffix.len() != 4 {
        return None;
    }
    if !suffix.chars().all(|ch| ch.is_ascii_digit()) {
        return None;
    }
    suffix.parse::<usize>().ok()
}

fn ensure_no_overflow_indexed_symbol_roots_for_decomposition(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    root_limits: &[(&str, usize); 7],
) -> Result<(), HillslopeDecompositionBoundaryError> {
    let slot_crop_prefix = format!("pl_decomp_slot_{slot_index:04}_crop_{crop_slot_index:04}_");
    let mut first_overflow_by_root: [Option<(BoundarySymbol, usize)>; 7] =
        std::array::from_fn(|_| None);

    for symbol in state_surface.keys() {
        let Some(symbol_suffix) = symbol.as_str().strip_prefix(slot_crop_prefix.as_str()) else {
            continue;
        };
        for (root_index, (root, max_expected)) in root_limits.iter().enumerate() {
            if first_overflow_by_root[root_index].is_some() {
                continue;
            }
            let Some(index_suffix) = symbol_suffix.strip_prefix(root) else {
                continue;
            };
            let Some(index_suffix) = index_suffix.strip_prefix('_') else {
                continue;
            };
            if let Some(index) = parse_indexed_suffix_for_decomposition(index_suffix)
                && (index == 0 || index > *max_expected)
            {
                first_overflow_by_root[root_index] = Some((symbol.clone(), index));
            }
        }
    }

    for (root_index, (_, max_expected)) in root_limits.iter().enumerate() {
        if let Some((symbol, index)) = first_overflow_by_root[root_index].clone() {
            return Err(
                HillslopeDecompositionBoundaryError::UnexpectedIndexedStateSymbol {
                    phase,
                    symbol,
                    index,
                    max_expected: *max_expected,
                },
            );
        }
    }
    Ok(())
}

fn require_indexed_state_value_for_decomposition(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
    index: usize,
) -> Result<f64, HillslopeDecompositionBoundaryError> {
    if !state_surface.contains_key(&BoundarySymbol::from(symbol)) {
        return Err(
            HillslopeDecompositionBoundaryError::MissingIndexedStateSymbol {
                phase,
                symbol: BoundarySymbol::from(symbol),
                index,
            },
        );
    }
    require_finite_state_value_for_decomposition(phase, state_surface, symbol)
}

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn require_indexed_integral_state_value_for_decomposition(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
    index: usize,
    min_allowed: usize,
    max_allowed: usize,
) -> Result<usize, HillslopeDecompositionBoundaryError> {
    let value = require_indexed_state_value_for_decomposition(phase, state_surface, symbol, index)?;
    let rounded = value.round();
    if (value - rounded).abs() > MANAGEMENT_CLASS_EPSILON {
        return Err(
            HillslopeDecompositionBoundaryError::NonIntegralRequiredStateSymbol {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value,
            },
        );
    }

    let min_f64 = min_allowed as f64;
    let max_f64 = max_allowed as f64;
    if rounded < min_f64 || rounded > max_f64 {
        return Err(
            HillslopeDecompositionBoundaryError::StateSymbolValueOutOfRange {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value: rounded as usize,
                min_allowed,
                max_allowed,
            },
        );
    }

    Ok(rounded as usize)
}

fn require_indexed_positive_state_value_for_decomposition(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
    index: usize,
) -> Result<f64, HillslopeDecompositionBoundaryError> {
    let value = require_indexed_state_value_for_decomposition(phase, state_surface, symbol, index)?;
    if value <= 0.0 {
        return Err(
            HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value,
                reason: "expected positive value",
            },
        );
    }
    Ok(value)
}

fn require_indexed_fraction_state_value_for_decomposition(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &str,
    index: usize,
) -> Result<f64, HillslopeDecompositionBoundaryError> {
    let value = require_indexed_state_value_for_decomposition(phase, state_surface, symbol, index)?;
    if !(0.0..=1.0).contains(&value) {
        return Err(
            HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(symbol),
                value,
                reason: "expected fraction in [0,1]",
            },
        );
    }
    Ok(value)
}

#[derive(Debug, Clone)]
struct AnnualDecompositionControlSymbols {
    resmgt: String,
    jdherb: String,
    jdburn: String,
    jdslge: String,
    jdcut: String,
    jdmove: String,
    fbrnag: String,
    fbrnog: String,
    frcut: String,
    frmove: String,
}

#[derive(Debug, Clone, Copy)]
struct AnnualDecompositionControlValues {
    resmgt: usize,
    jdherb: usize,
    jdburn: usize,
    jdslge: usize,
    jdcut: usize,
    jdmove: usize,
    fbrnag: f64,
    fbrnog: f64,
    frcut: f64,
    frmove: f64,
}

#[derive(Debug, Clone)]
struct AnnualDecompositionControlInputs {
    symbols: AnnualDecompositionControlSymbols,
    values: AnnualDecompositionControlValues,
}

fn build_annual_decomposition_control(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    runtime_day: usize,
) -> Result<HillslopeAnnualDecompositionControl, HillslopeDecompositionBoundaryError> {
    let inputs =
        require_annual_decomposition_control_inputs(phase, state_surface, slot_index, crop_slot_index)?;
    let active_action = resolve_annual_decomposition_action(phase, &inputs, runtime_day)?;

    Ok(HillslopeAnnualDecompositionControl {
        resmgt: usize_to_u8_for_decomposition(
            phase,
            BoundarySymbol::from(inputs.symbols.resmgt.as_str()),
            inputs.values.resmgt,
        )?,
        jdherb: usize_to_u16_for_decomposition(
            phase,
            BoundarySymbol::from(inputs.symbols.jdherb.as_str()),
            inputs.values.jdherb,
        )?,
        jdburn: usize_to_u16_for_decomposition(
            phase,
            BoundarySymbol::from(inputs.symbols.jdburn.as_str()),
            inputs.values.jdburn,
        )?,
        jdslge: usize_to_u16_for_decomposition(
            phase,
            BoundarySymbol::from(inputs.symbols.jdslge.as_str()),
            inputs.values.jdslge,
        )?,
        jdcut: usize_to_u16_for_decomposition(
            phase,
            BoundarySymbol::from(inputs.symbols.jdcut.as_str()),
            inputs.values.jdcut,
        )?,
        jdmove: usize_to_u16_for_decomposition(
            phase,
            BoundarySymbol::from(inputs.symbols.jdmove.as_str()),
            inputs.values.jdmove,
        )?,
        fbrnag: inputs.values.fbrnag,
        fbrnog: inputs.values.fbrnog,
        frcut: inputs.values.frcut,
        frmove: inputs.values.frmove,
        active_action,
    })
}

#[allow(clippy::similar_names)]
fn require_annual_decomposition_control_inputs(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
) -> Result<AnnualDecompositionControlInputs, HillslopeDecompositionBoundaryError> {
    let resmgt_symbol = pl_decomp_slot_crop_symbol("resmgt", slot_index, crop_slot_index);
    let resmgt = require_integral_state_value_for_decomposition(
        phase,
        state_surface,
        resmgt_symbol.as_str(),
        1,
        6,
    )?;

    let jdherb_symbol = pl_decomp_slot_crop_symbol("jdherb", slot_index, crop_slot_index);
    let jdburn_symbol = pl_decomp_slot_crop_symbol("jdburn", slot_index, crop_slot_index);
    let jdslge_symbol = pl_decomp_slot_crop_symbol("jdslge", slot_index, crop_slot_index);
    let jdcut_symbol = pl_decomp_slot_crop_symbol("jdcut", slot_index, crop_slot_index);
    let jdmove_symbol = pl_decomp_slot_crop_symbol("jdmove", slot_index, crop_slot_index);
    let fbrnag_symbol = pl_decomp_slot_crop_symbol("fbrnag", slot_index, crop_slot_index);
    let fbrnog_symbol = pl_decomp_slot_crop_symbol("fbrnog", slot_index, crop_slot_index);
    let frcut_symbol = pl_decomp_slot_crop_symbol("frcut", slot_index, crop_slot_index);
    let frmove_symbol = pl_decomp_slot_crop_symbol("frmove", slot_index, crop_slot_index);

    let jdherb = require_day_state_value_for_decomposition(
        phase,
        state_surface,
        jdherb_symbol.as_str(),
        true,
    )?;
    let jdburn = require_day_state_value_for_decomposition(
        phase,
        state_surface,
        jdburn_symbol.as_str(),
        true,
    )?;
    let jdslge = require_day_state_value_for_decomposition(
        phase,
        state_surface,
        jdslge_symbol.as_str(),
        true,
    )?;
    let jdcut = require_day_state_value_for_decomposition(
        phase,
        state_surface,
        jdcut_symbol.as_str(),
        true,
    )?;
    let jdmove = require_day_state_value_for_decomposition(
        phase,
        state_surface,
        jdmove_symbol.as_str(),
        true,
    )?;
    let fbrnag = require_fraction_state_value_for_decomposition(
        phase,
        state_surface,
        fbrnag_symbol.as_str(),
    )?;
    let fbrnog = require_fraction_state_value_for_decomposition(
        phase,
        state_surface,
        fbrnog_symbol.as_str(),
    )?;
    let frcut = require_fraction_state_value_for_decomposition(
        phase,
        state_surface,
        frcut_symbol.as_str(),
    )?;
    let frmove = require_fraction_state_value_for_decomposition(
        phase,
        state_surface,
        frmove_symbol.as_str(),
    )?;

    Ok(AnnualDecompositionControlInputs {
        symbols: AnnualDecompositionControlSymbols {
            resmgt: resmgt_symbol,
            jdherb: jdherb_symbol,
            jdburn: jdburn_symbol,
            jdslge: jdslge_symbol,
            jdcut: jdcut_symbol,
            jdmove: jdmove_symbol,
            fbrnag: fbrnag_symbol,
            fbrnog: fbrnog_symbol,
            frcut: frcut_symbol,
            frmove: frmove_symbol,
        },
        values: AnnualDecompositionControlValues {
            resmgt,
            jdherb,
            jdburn,
            jdslge,
            jdcut,
            jdmove,
            fbrnag,
            fbrnog,
            frcut,
            frmove,
        },
    })
}

fn resolve_annual_decomposition_action(
    phase: HillslopePhase,
    inputs: &AnnualDecompositionControlInputs,
    runtime_day: usize,
) -> Result<HillslopeAnnualDecompositionAction, HillslopeDecompositionBoundaryError> {
    match inputs.values.resmgt {
        1 => annual_herbicide_decomposition_action(phase, inputs, runtime_day),
        2 => annual_burn_decomposition_action(phase, inputs, runtime_day),
        3 => annual_silage_decomposition_action(phase, inputs, runtime_day),
        4 => annual_cut_decomposition_action(phase, inputs, runtime_day),
        5 => annual_remove_decomposition_action(phase, inputs, runtime_day),
        6 => annual_noop_decomposition_action(phase, inputs),
        _ => unreachable!("resmgt domain is validated above"),
    }
}

#[allow(clippy::cast_precision_loss)]
fn annual_herbicide_decomposition_action(
    phase: HillslopePhase,
    inputs: &AnnualDecompositionControlInputs,
    runtime_day: usize,
) -> Result<HillslopeAnnualDecompositionAction, HillslopeDecompositionBoundaryError> {
    let symbols = &inputs.symbols;
    let values = inputs.values;
    if values.jdherb == 0 {
        return Err(
            HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(symbols.jdherb.as_str()),
                value: 0.0,
                reason: "resmgt=1 requires jdherb in 1..366",
            },
        );
    }
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdburn.as_str(),
        values.jdburn as f64,
        "resmgt=1 requires jdburn=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdslge.as_str(),
        values.jdslge as f64,
        "resmgt=1 requires jdslge=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdcut.as_str(),
        values.jdcut as f64,
        "resmgt=1 requires jdcut=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdmove.as_str(),
        values.jdmove as f64,
        "resmgt=1 requires jdmove=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.fbrnag.as_str(),
        values.fbrnag,
        "resmgt=1 requires fbrnag=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.fbrnog.as_str(),
        values.fbrnog,
        "resmgt=1 requires fbrnog=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.frcut.as_str(),
        values.frcut,
        "resmgt=1 requires frcut=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.frmove.as_str(),
        values.frmove,
        "resmgt=1 requires frmove=0",
    )?;
    if runtime_day == values.jdherb {
        Ok(HillslopeAnnualDecompositionAction::Herbicide)
    } else {
        Ok(HillslopeAnnualDecompositionAction::None)
    }
}

#[allow(clippy::cast_precision_loss)]
fn annual_burn_decomposition_action(
    phase: HillslopePhase,
    inputs: &AnnualDecompositionControlInputs,
    runtime_day: usize,
) -> Result<HillslopeAnnualDecompositionAction, HillslopeDecompositionBoundaryError> {
    let symbols = &inputs.symbols;
    let values = inputs.values;
    if values.jdburn == 0 {
        return Err(
            HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(symbols.jdburn.as_str()),
                value: 0.0,
                reason: "resmgt=2 requires jdburn in 1..366",
            },
        );
    }
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdherb.as_str(),
        values.jdherb as f64,
        "resmgt=2 requires jdherb=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdslge.as_str(),
        values.jdslge as f64,
        "resmgt=2 requires jdslge=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdcut.as_str(),
        values.jdcut as f64,
        "resmgt=2 requires jdcut=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdmove.as_str(),
        values.jdmove as f64,
        "resmgt=2 requires jdmove=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.frcut.as_str(),
        values.frcut,
        "resmgt=2 requires frcut=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.frmove.as_str(),
        values.frmove,
        "resmgt=2 requires frmove=0",
    )?;
    if runtime_day == values.jdburn {
        Ok(HillslopeAnnualDecompositionAction::Burn)
    } else {
        Ok(HillslopeAnnualDecompositionAction::None)
    }
}

#[allow(clippy::cast_precision_loss)]
fn annual_silage_decomposition_action(
    phase: HillslopePhase,
    inputs: &AnnualDecompositionControlInputs,
    runtime_day: usize,
) -> Result<HillslopeAnnualDecompositionAction, HillslopeDecompositionBoundaryError> {
    let symbols = &inputs.symbols;
    let values = inputs.values;
    if values.jdslge == 0 {
        return Err(
            HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(symbols.jdslge.as_str()),
                value: 0.0,
                reason: "resmgt=3 requires jdslge in 1..366",
            },
        );
    }
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdherb.as_str(),
        values.jdherb as f64,
        "resmgt=3 requires jdherb=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdburn.as_str(),
        values.jdburn as f64,
        "resmgt=3 requires jdburn=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdcut.as_str(),
        values.jdcut as f64,
        "resmgt=3 requires jdcut=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdmove.as_str(),
        values.jdmove as f64,
        "resmgt=3 requires jdmove=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.fbrnag.as_str(),
        values.fbrnag,
        "resmgt=3 requires fbrnag=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.fbrnog.as_str(),
        values.fbrnog,
        "resmgt=3 requires fbrnog=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.frcut.as_str(),
        values.frcut,
        "resmgt=3 requires frcut=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.frmove.as_str(),
        values.frmove,
        "resmgt=3 requires frmove=0",
    )?;
    if runtime_day == values.jdslge {
        Ok(HillslopeAnnualDecompositionAction::Silage)
    } else {
        Ok(HillslopeAnnualDecompositionAction::None)
    }
}

#[allow(clippy::cast_precision_loss)]
fn annual_cut_decomposition_action(
    phase: HillslopePhase,
    inputs: &AnnualDecompositionControlInputs,
    runtime_day: usize,
) -> Result<HillslopeAnnualDecompositionAction, HillslopeDecompositionBoundaryError> {
    let symbols = &inputs.symbols;
    let values = inputs.values;
    if values.jdcut == 0 {
        return Err(
            HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(symbols.jdcut.as_str()),
                value: 0.0,
                reason: "resmgt=4 requires jdcut in 1..366",
            },
        );
    }
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdherb.as_str(),
        values.jdherb as f64,
        "resmgt=4 requires jdherb=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdburn.as_str(),
        values.jdburn as f64,
        "resmgt=4 requires jdburn=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdslge.as_str(),
        values.jdslge as f64,
        "resmgt=4 requires jdslge=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdmove.as_str(),
        values.jdmove as f64,
        "resmgt=4 requires jdmove=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.fbrnag.as_str(),
        values.fbrnag,
        "resmgt=4 requires fbrnag=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.fbrnog.as_str(),
        values.fbrnog,
        "resmgt=4 requires fbrnog=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.frmove.as_str(),
        values.frmove,
        "resmgt=4 requires frmove=0",
    )?;
    if runtime_day == values.jdcut {
        Ok(HillslopeAnnualDecompositionAction::Cut)
    } else {
        Ok(HillslopeAnnualDecompositionAction::None)
    }
}

#[allow(clippy::cast_precision_loss)]
fn annual_remove_decomposition_action(
    phase: HillslopePhase,
    inputs: &AnnualDecompositionControlInputs,
    runtime_day: usize,
) -> Result<HillslopeAnnualDecompositionAction, HillslopeDecompositionBoundaryError> {
    let symbols = &inputs.symbols;
    let values = inputs.values;
    if values.jdmove == 0 {
        return Err(
            HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                phase,
                symbol: BoundarySymbol::from(symbols.jdmove.as_str()),
                value: 0.0,
                reason: "resmgt=5 requires jdmove in 1..366",
            },
        );
    }
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdherb.as_str(),
        values.jdherb as f64,
        "resmgt=5 requires jdherb=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdburn.as_str(),
        values.jdburn as f64,
        "resmgt=5 requires jdburn=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdslge.as_str(),
        values.jdslge as f64,
        "resmgt=5 requires jdslge=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdcut.as_str(),
        values.jdcut as f64,
        "resmgt=5 requires jdcut=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.fbrnag.as_str(),
        values.fbrnag,
        "resmgt=5 requires fbrnag=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.fbrnog.as_str(),
        values.fbrnog,
        "resmgt=5 requires fbrnog=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.frcut.as_str(),
        values.frcut,
        "resmgt=5 requires frcut=0",
    )?;
    if runtime_day == values.jdmove {
        Ok(HillslopeAnnualDecompositionAction::Remove)
    } else {
        Ok(HillslopeAnnualDecompositionAction::None)
    }
}

#[allow(clippy::cast_precision_loss)]
fn annual_noop_decomposition_action(
    phase: HillslopePhase,
    inputs: &AnnualDecompositionControlInputs,
) -> Result<HillslopeAnnualDecompositionAction, HillslopeDecompositionBoundaryError> {
    let symbols = &inputs.symbols;
    let values = inputs.values;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdherb.as_str(),
        values.jdherb as f64,
        "resmgt=6 requires jdherb=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdburn.as_str(),
        values.jdburn as f64,
        "resmgt=6 requires jdburn=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdslge.as_str(),
        values.jdslge as f64,
        "resmgt=6 requires jdslge=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdcut.as_str(),
        values.jdcut as f64,
        "resmgt=6 requires jdcut=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.jdmove.as_str(),
        values.jdmove as f64,
        "resmgt=6 requires jdmove=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.fbrnag.as_str(),
        values.fbrnag,
        "resmgt=6 requires fbrnag=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.fbrnog.as_str(),
        values.fbrnog,
        "resmgt=6 requires fbrnog=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.frcut.as_str(),
        values.frcut,
        "resmgt=6 requires frcut=0",
    )?;
    require_zero_state_value_for_decomposition(
        phase,
        symbols.frmove.as_str(),
        values.frmove,
        "resmgt=6 requires frmove=0",
    )?;
    Ok(HillslopeAnnualDecompositionAction::None)
}

#[allow(clippy::too_many_lines, clippy::cast_precision_loss)]
fn build_perennial_decomposition_control(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    runtime_day: usize,
) -> Result<HillslopePerennialDecompositionControl, HillslopeDecompositionBoundaryError> {
    let mgtopt_symbol = pl_decomp_slot_crop_symbol("mgtopt", slot_index, crop_slot_index);
    let ncut_symbol = pl_decomp_slot_crop_symbol("ncut", slot_index, crop_slot_index);
    let ncycle_symbol = pl_decomp_slot_crop_symbol("ncycle", slot_index, crop_slot_index);
    let mgtopt = require_integral_state_value_for_decomposition(
        phase,
        state_surface,
        mgtopt_symbol.as_str(),
        1,
        3,
    )?;
    let ncut = require_integral_state_value_for_decomposition(
        phase,
        state_surface,
        ncut_symbol.as_str(),
        0,
        usize::from(u16::MAX),
    )?;
    let ncycle = require_integral_state_value_for_decomposition(
        phase,
        state_surface,
        ncycle_symbol.as_str(),
        0,
        usize::from(u16::MAX),
    )?;

    let mut active_action = HillslopePerennialDecompositionAction::None;
    let mut active_grazing_cycle = None;

    match mgtopt {
        1 => {
            if ncut == 0 {
                return Err(
                    HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                        phase,
                        symbol: BoundarySymbol::from(ncut_symbol.as_str()),
                        value: 0.0,
                        reason: "mgtopt=1 requires ncut>=1",
                    },
                );
            }
            if ncycle != 0 {
                return Err(
                    HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                        phase,
                        symbol: BoundarySymbol::from(ncycle_symbol.as_str()),
                        value: ncycle as f64,
                        reason: "mgtopt=1 requires ncycle=0",
                    },
                );
            }

            ensure_no_overflow_indexed_symbol_roots_for_decomposition(
                phase,
                state_surface,
                slot_index,
                crop_slot_index,
                &[
                    ("cutday", ncut),
                    ("gday", 0),
                    ("gend", 0),
                    ("animal", 0),
                    ("bodywt", 0),
                    ("area", 0),
                    ("digest", 0),
                ],
            )?;

            let mut active_cut_index = None;
            for event_index in 1..=ncut {
                let cut_symbol = pl_decomp_slot_crop_indexed_symbol(
                    "cutday",
                    slot_index,
                    crop_slot_index,
                    event_index,
                );
                let cutday = require_indexed_integral_state_value_for_decomposition(
                    phase,
                    state_surface,
                    cut_symbol.as_str(),
                    event_index,
                    1,
                    366,
                )?;
                if runtime_day == cutday {
                    if active_cut_index.is_some() {
                        return Err(
                            HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                                phase,
                                symbol: BoundarySymbol::from(cut_symbol.as_str()),
                                value: cutday as f64,
                                reason: "multiple cutday entries active on runtime day",
                            },
                        );
                    }
                    active_cut_index = Some(event_index);
                }
            }

            if let Some(event_index) = active_cut_index {
                active_action = HillslopePerennialDecompositionAction::Cut {
                    event_index: usize_to_u16_for_decomposition(
                        phase,
                        BoundarySymbol::from("cutday"),
                        event_index,
                    )?,
                };
            }
        }
        2 => {
            if ncycle == 0 {
                return Err(
                    HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                        phase,
                        symbol: BoundarySymbol::from(ncycle_symbol.as_str()),
                        value: 0.0,
                        reason: "mgtopt=2 requires ncycle>=1",
                    },
                );
            }
            if ncut != 0 {
                return Err(
                    HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                        phase,
                        symbol: BoundarySymbol::from(ncut_symbol.as_str()),
                        value: ncut as f64,
                        reason: "mgtopt=2 requires ncut=0",
                    },
                );
            }

            ensure_no_overflow_indexed_symbol_roots_for_decomposition(
                phase,
                state_surface,
                slot_index,
                crop_slot_index,
                &[
                    ("cutday", 0),
                    ("gday", ncycle),
                    ("gend", ncycle),
                    ("animal", ncycle),
                    ("bodywt", ncycle),
                    ("area", ncycle),
                    ("digest", ncycle),
                ],
            )?;

            for cycle_index in 1..=ncycle {
                let gday_symbol = pl_decomp_slot_crop_indexed_symbol(
                    "gday",
                    slot_index,
                    crop_slot_index,
                    cycle_index,
                );
                let gend_symbol = pl_decomp_slot_crop_indexed_symbol(
                    "gend",
                    slot_index,
                    crop_slot_index,
                    cycle_index,
                );
                let animal_symbol = pl_decomp_slot_crop_indexed_symbol(
                    "animal",
                    slot_index,
                    crop_slot_index,
                    cycle_index,
                );
                let bodywt_symbol = pl_decomp_slot_crop_indexed_symbol(
                    "bodywt",
                    slot_index,
                    crop_slot_index,
                    cycle_index,
                );
                let area_symbol = pl_decomp_slot_crop_indexed_symbol(
                    "area",
                    slot_index,
                    crop_slot_index,
                    cycle_index,
                );
                let digest_symbol = pl_decomp_slot_crop_indexed_symbol(
                    "digest",
                    slot_index,
                    crop_slot_index,
                    cycle_index,
                );

                let gday = require_indexed_integral_state_value_for_decomposition(
                    phase,
                    state_surface,
                    gday_symbol.as_str(),
                    cycle_index,
                    1,
                    366,
                )?;
                let gend = require_indexed_integral_state_value_for_decomposition(
                    phase,
                    state_surface,
                    gend_symbol.as_str(),
                    cycle_index,
                    1,
                    366,
                )?;
                if gday >= gend {
                    return Err(HillslopeDecompositionBoundaryError::InvalidGrazingWindow {
                        phase,
                        cycle_index,
                        gday_symbol: BoundarySymbol::from(gday_symbol.as_str()),
                        gend_symbol: BoundarySymbol::from(gend_symbol.as_str()),
                        gday,
                        gend,
                    });
                }

                let animal = require_indexed_positive_state_value_for_decomposition(
                    phase,
                    state_surface,
                    animal_symbol.as_str(),
                    cycle_index,
                )?;
                let bodywt = require_indexed_positive_state_value_for_decomposition(
                    phase,
                    state_surface,
                    bodywt_symbol.as_str(),
                    cycle_index,
                )?;
                let area = require_indexed_positive_state_value_for_decomposition(
                    phase,
                    state_surface,
                    area_symbol.as_str(),
                    cycle_index,
                )?;
                let digest = require_indexed_fraction_state_value_for_decomposition(
                    phase,
                    state_surface,
                    digest_symbol.as_str(),
                    cycle_index,
                )?;
                if digest == 0.0 {
                    return Err(
                        HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                            phase,
                            symbol: BoundarySymbol::from(digest_symbol.as_str()),
                            value: digest,
                            reason: "grazing digest must be positive",
                        },
                    );
                }

                let in_window = runtime_day >= gday && runtime_day < gend;
                if in_window {
                    if active_grazing_cycle.is_some() {
                        return Err(
                            HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                                phase,
                                symbol: BoundarySymbol::from(gday_symbol.as_str()),
                                value: runtime_day as f64,
                                reason: "multiple grazing cycles active on runtime day",
                            },
                        );
                    }
                    active_grazing_cycle = Some(HillslopeActiveGrazingCycle {
                        cycle_index: usize_to_u16_for_decomposition(
                            phase,
                            BoundarySymbol::from("cycle_index"),
                            cycle_index,
                        )?,
                        gday: usize_to_u16_for_decomposition(
                            phase,
                            BoundarySymbol::from(gday_symbol.as_str()),
                            gday,
                        )?,
                        gend: usize_to_u16_for_decomposition(
                            phase,
                            BoundarySymbol::from(gend_symbol.as_str()),
                            gend,
                        )?,
                        animal,
                        bodywt,
                        area,
                        digest,
                    });
                }
            }

            if let Some(cycle) = active_grazing_cycle {
                active_action = HillslopePerennialDecompositionAction::Grazing {
                    cycle_index: cycle.cycle_index,
                };
            }
        }
        3 => {
            if ncut != 0 {
                return Err(
                    HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                        phase,
                        symbol: BoundarySymbol::from(ncut_symbol.as_str()),
                        value: ncut as f64,
                        reason: "mgtopt=3 requires ncut=0",
                    },
                );
            }
            if ncycle != 0 {
                return Err(
                    HillslopeDecompositionBoundaryError::InvalidTransitionPayloadState {
                        phase,
                        symbol: BoundarySymbol::from(ncycle_symbol.as_str()),
                        value: ncycle as f64,
                        reason: "mgtopt=3 requires ncycle=0",
                    },
                );
            }

            ensure_no_overflow_indexed_symbol_roots_for_decomposition(
                phase,
                state_surface,
                slot_index,
                crop_slot_index,
                &[
                    ("cutday", 0),
                    ("gday", 0),
                    ("gend", 0),
                    ("animal", 0),
                    ("bodywt", 0),
                    ("area", 0),
                    ("digest", 0),
                ],
            )?;
        }
        _ => unreachable!("mgtopt domain is validated above"),
    }

    Ok(HillslopePerennialDecompositionControl {
        mgtopt: usize_to_u8_for_decomposition(phase, BoundarySymbol::from(mgtopt_symbol), mgtopt)?,
        ncut: usize_to_u16_for_decomposition(phase, BoundarySymbol::from(ncut_symbol), ncut)?,
        ncycle: usize_to_u16_for_decomposition(phase, BoundarySymbol::from(ncycle_symbol), ncycle)?,
        active_action,
        active_grazing_cycle,
    })
}
