#[allow(clippy::too_many_lines)]
pub(crate) fn decomposition_phase_dispatch_for_state(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<DecompositionPhaseDispatch, HillslopeDecompositionBoundaryError> {
    if !state_surface.contains_key(&BoundarySymbol::from(PL_DECOMP_RUNTIME_SENTINEL)) {
        return Ok(DecompositionPhaseDispatch::Skip);
    }

    let active_slot_selection =
        resolve_active_pl_slot_selection(state_surface).map_err(|source| {
            HillslopeDecompositionBoundaryError::ActiveSlotResolution { phase, source }
        })?;

    let runtime_day =
        require_integral_pl_dispatch_symbol_in_range(state_surface, PL_RUNTIME_DAY_SYMBOL, 1, 366)
            .map_err(
                |source| HillslopeDecompositionBoundaryError::ActiveSlotResolution {
                    phase,
                    source,
                },
            )?;

    let imngmt_symbol = pl_growth_slot_crop_symbol(
        "imngmt",
        active_slot_selection.slot_index,
        active_slot_selection.crop_slot_index,
    );
    let imngmt =
        require_finite_state_value_for_decomposition(phase, state_surface, imngmt_symbol.as_str())?;
    let management_class =
        normalize_management_class_for_decomposition(phase, imngmt, imngmt_symbol.as_str())?;
    let order_decomp_before_soil = require_ordering_flag_for_decomposition(
        phase,
        state_surface,
        PL_ORDER_DECOMP_BEFORE_SOIL_SYMBOL,
        1.0,
    )?;
    let order_growth_after_decomp = require_ordering_flag_for_decomposition(
        phase,
        state_surface,
        PL_ORDER_GROWTH_AFTER_DECOMP_SYMBOL,
        1.0,
    )?;

    let iresd_seed = require_finite_state_value_for_decomposition(
        phase,
        state_surface,
        PL_DECOMP_IRESD_SEED_SYMBOL,
    )?;
    let sumrtm_seed = require_finite_state_value_for_decomposition(
        phase,
        state_surface,
        PL_DECOMP_SUMRTM_SEED_SYMBOL,
    )?;
    let sumsrm_seed = require_finite_state_value_for_decomposition(
        phase,
        state_surface,
        PL_DECOMP_SUMSRM_SEED_SYMBOL,
    )?;

    let control = match management_class {
        1 | 3 => {
            HillslopeDecompositionTransitionControl::Annual(build_annual_decomposition_control(
                phase,
                state_surface,
                active_slot_selection.slot_index,
                active_slot_selection.crop_slot_index,
                runtime_day,
            )?)
        }
        2 => HillslopeDecompositionTransitionControl::Perennial(
            build_perennial_decomposition_control(
                phase,
                state_surface,
                active_slot_selection.slot_index,
                active_slot_selection.crop_slot_index,
                runtime_day,
            )?,
        ),
        _ => {
            return Err(
                HillslopeDecompositionBoundaryError::UnsupportedManagementClass {
                    phase,
                    symbol: BoundarySymbol::from(imngmt_symbol.as_str()),
                    value: imngmt,
                },
            );
        }
    };

    let (sumrtm_seed, sumsrm_seed) = compute_equation_decomposition_seed_surface(
        phase,
        state_surface,
        active_slot_selection.slot_index,
        active_slot_selection.crop_slot_index,
        control,
        sumrtm_seed,
        sumsrm_seed,
    )?;

    let management_class = if management_class == 2 {
        HillslopeDecompositionManagementClass::Perennial
    } else {
        HillslopeDecompositionManagementClass::AnnualOrFallow
    };

    let transition_payload = HillslopeDecompositionTransitionPayload {
        active_slot_index: active_slot_selection.slot_index,
        active_crop_slot_index: active_slot_selection.crop_slot_index,
        runtime_day_of_year: usize_to_u16_for_decomposition(
            phase,
            BoundarySymbol::from(PL_RUNTIME_DAY_SYMBOL),
            runtime_day,
        )?,
        iresd_seed,
        sumrtm_seed,
        sumsrm_seed,
        control,
    };

    Ok(DecompositionPhaseDispatch::Execute(
        HillslopeDecompositionKernelContext::new(
            management_class,
            order_decomp_before_soil,
            order_growth_after_decomp,
        )
        .with_transition_payload(transition_payload),
    ))
}

#[allow(clippy::too_many_lines)]
pub(crate) fn growth_phase_dispatch_for_state(
    phase: HillslopePhase,
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<GrowthPhaseDispatch, HillslopeGrowthBoundaryError> {
    if !state_surface.contains_key(&BoundarySymbol::from(PL_GROWTH_RUNTIME_SENTINEL)) {
        return Ok(GrowthPhaseDispatch::Skip);
    }

    let active_slot_selection = resolve_active_pl_slot_selection(state_surface)
        .map_err(|source| HillslopeGrowthBoundaryError::ActiveSlotResolution { phase, source })?;
    let imngmt_symbol = pl_growth_slot_crop_symbol(
        "imngmt",
        active_slot_selection.slot_index,
        active_slot_selection.crop_slot_index,
    );
    let imngmt = require_finite_state_value(phase, state_surface, imngmt_symbol.as_str())?;
    let management_class = normalize_management_class(phase, imngmt, imngmt_symbol.as_str())?;
    let runtime_day = require_integral_state_value_in_range_for_growth(
        phase,
        state_surface,
        PL_RUNTIME_DAY_SYMBOL,
        1,
        366,
    )?;
    let order_growth_after_decomp = require_ordering_flag(
        phase,
        state_surface,
        PL_ORDER_GROWTH_AFTER_DECOMP_SYMBOL,
        1.0,
    )?;
    let order_watbal_after_growth = require_ordering_flag(
        phase,
        state_surface,
        PL_ORDER_WATBAL_AFTER_GROWTH_SYMBOL,
        1.0,
    )?;
    let state_before = require_growth_state_surface(phase, state_surface)?;

    match phase {
        HillslopePhase::AnnualGrowthTransition => {
            if management_class == 2 {
                return Ok(GrowthPhaseDispatch::Skip);
            }
            debug_assert!(management_class == 1 || management_class == 3);

            let jdharv_symbol = pl_growth_slot_crop_symbol(
                "jdharv",
                active_slot_selection.slot_index,
                active_slot_selection.crop_slot_index,
            );
            let jdplt_symbol = pl_growth_slot_crop_symbol(
                "jdplt",
                active_slot_selection.slot_index,
                active_slot_selection.crop_slot_index,
            );
            let rw_symbol = pl_growth_slot_crop_symbol(
                "rw",
                active_slot_selection.slot_index,
                active_slot_selection.crop_slot_index,
            );
            let resmgt_symbol = pl_decomp_slot_crop_symbol(
                "resmgt",
                active_slot_selection.slot_index,
                active_slot_selection.crop_slot_index,
            );

            let jdharv = require_integral_state_value_in_range_for_growth(
                phase,
                state_surface,
                jdharv_symbol.as_str(),
                1,
                366,
            )?;
            let jdplt = require_integral_state_value_in_range_for_growth(
                phase,
                state_surface,
                jdplt_symbol.as_str(),
                1,
                366,
            )?;
            let rw = require_finite_state_value(phase, state_surface, rw_symbol.as_str())?;
            let _resmgt = require_integral_state_value_in_range_for_growth(
                phase,
                state_surface,
                resmgt_symbol.as_str(),
                1,
                6,
            )?;

            let active_action = if runtime_day == jdplt {
                HillslopeAnnualGrowthAction::PlantingReset
            } else if runtime_day == jdharv {
                HillslopeAnnualGrowthAction::HarvestReset
            } else {
                HillslopeAnnualGrowthAction::None
            };

            let state_after = match active_action {
                HillslopeAnnualGrowthAction::None => compute_equation_growth_state_surface(
                    phase,
                    state_surface,
                    active_slot_selection.slot_index,
                    active_slot_selection.crop_slot_index,
                    management_class,
                    state_before,
                )?,
                HillslopeAnnualGrowthAction::PlantingReset
                | HillslopeAnnualGrowthAction::HarvestReset
                | HillslopeAnnualGrowthAction::SenescenceReset => {
                    reset_growth_state_surface(state_before)
                }
            };

            let transition_payload = HillslopeGrowthTransitionPayload {
                active_slot_index: active_slot_selection.slot_index,
                active_crop_slot_index: active_slot_selection.crop_slot_index,
                runtime_day_of_year: usize_to_u16_for_growth(
                    phase,
                    BoundarySymbol::from(PL_RUNTIME_DAY_SYMBOL),
                    runtime_day,
                )?,
                state_before,
                state_after,
                control: HillslopeGrowthTransitionControl::Annual(HillslopeAnnualGrowthControl {
                    jdharv: usize_to_u16_for_growth(
                        phase,
                        BoundarySymbol::from(jdharv_symbol.as_str()),
                        jdharv,
                    )?,
                    jdplt: usize_to_u16_for_growth(
                        phase,
                        BoundarySymbol::from(jdplt_symbol.as_str()),
                        jdplt,
                    )?,
                    rw,
                    active_action,
                }),
            };

            Ok(GrowthPhaseDispatch::Execute(
                HillslopeGrowthKernelContext::new(
                    HillslopeGrowthManagementClass::AnnualOrFallow,
                    order_growth_after_decomp,
                    order_watbal_after_growth,
                )
                .with_transition_payload(transition_payload),
            ))
        }
        HillslopePhase::PerennialGrowthTransition => {
            if management_class == 1 || management_class == 3 {
                return Ok(GrowthPhaseDispatch::Skip);
            }
            debug_assert!(management_class == 2);

            let jdharv_symbol = pl_growth_slot_crop_symbol(
                "jdharv",
                active_slot_selection.slot_index,
                active_slot_selection.crop_slot_index,
            );
            let jdplt_symbol = pl_growth_slot_crop_symbol(
                "jdplt",
                active_slot_selection.slot_index,
                active_slot_selection.crop_slot_index,
            );
            let rw_symbol = pl_growth_slot_crop_symbol(
                "rw",
                active_slot_selection.slot_index,
                active_slot_selection.crop_slot_index,
            );
            let jdstop_symbol = pl_growth_slot_crop_symbol(
                "jdstop",
                active_slot_selection.slot_index,
                active_slot_selection.crop_slot_index,
            );
            let mgtopt_symbol = pl_growth_slot_crop_symbol(
                "mgtopt",
                active_slot_selection.slot_index,
                active_slot_selection.crop_slot_index,
            );

            let jdharv = require_integral_state_value_in_range_for_growth(
                phase,
                state_surface,
                jdharv_symbol.as_str(),
                0,
                366,
            )?;
            let jdplt = require_integral_state_value_in_range_for_growth(
                phase,
                state_surface,
                jdplt_symbol.as_str(),
                0,
                366,
            )?;
            let rw = require_finite_state_value(phase, state_surface, rw_symbol.as_str())?;
            let jdstop = require_integral_state_value_in_range_for_growth(
                phase,
                state_surface,
                jdstop_symbol.as_str(),
                0,
                366,
            )?;
            let mgtopt = require_integral_state_value_in_range_for_growth(
                phase,
                state_surface,
                mgtopt_symbol.as_str(),
                1,
                3,
            )?;

            let active_action = if runtime_day == jdplt {
                HillslopePerennialGrowthAction::PlantingReset
            } else if jdstop != 0 && runtime_day == jdstop {
                HillslopePerennialGrowthAction::StopReset
            } else {
                HillslopePerennialGrowthAction::None
            };

            let state_after = match active_action {
                HillslopePerennialGrowthAction::None => compute_equation_growth_state_surface(
                    phase,
                    state_surface,
                    active_slot_selection.slot_index,
                    active_slot_selection.crop_slot_index,
                    management_class,
                    state_before,
                )?,
                HillslopePerennialGrowthAction::PlantingReset
                | HillslopePerennialGrowthAction::StopReset => {
                    reset_growth_state_surface(state_before)
                }
            };

            let transition_payload = HillslopeGrowthTransitionPayload {
                active_slot_index: active_slot_selection.slot_index,
                active_crop_slot_index: active_slot_selection.crop_slot_index,
                runtime_day_of_year: usize_to_u16_for_growth(
                    phase,
                    BoundarySymbol::from(PL_RUNTIME_DAY_SYMBOL),
                    runtime_day,
                )?,
                state_before,
                state_after,
                control: HillslopeGrowthTransitionControl::Perennial(
                    HillslopePerennialGrowthControl {
                        jdharv: usize_to_u16_for_growth(
                            phase,
                            BoundarySymbol::from(jdharv_symbol.as_str()),
                            jdharv,
                        )?,
                        jdplt: usize_to_u16_for_growth(
                            phase,
                            BoundarySymbol::from(jdplt_symbol.as_str()),
                            jdplt,
                        )?,
                        jdstop: usize_to_u16_for_growth(
                            phase,
                            BoundarySymbol::from(jdstop_symbol.as_str()),
                            jdstop,
                        )?,
                        mgtopt: usize_to_u8_for_growth(
                            phase,
                            BoundarySymbol::from(mgtopt_symbol.as_str()),
                            mgtopt,
                        )?,
                        rw,
                        active_action,
                    },
                ),
            };

            Ok(GrowthPhaseDispatch::Execute(
                HillslopeGrowthKernelContext::new(
                    HillslopeGrowthManagementClass::Perennial,
                    order_growth_after_decomp,
                    order_watbal_after_growth,
                )
                .with_transition_payload(transition_payload),
            ))
        }
        _ => Ok(GrowthPhaseDispatch::Skip),
    }
}

