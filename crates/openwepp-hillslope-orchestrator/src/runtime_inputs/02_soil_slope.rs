/// Build an orchestrator-owned hillslope runtime surface from parsed soil input.
///
/// This seam is strict by design: missing runtime-critical fields fail
/// explicitly instead of defaulting.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when required parser outputs are
/// missing or non-finite.
#[allow(clippy::too_many_lines)]
pub fn build_hillslope_runtime_surface_from_soil(
    soil: &SoilProfile,
) -> Result<HillslopeWritebackSurface, HillslopeRuntimeInputError> {
    let primary_ofe = soil
        .ofes
        .first()
        .ok_or(HillslopeRuntimeInputError::MissingSoilOfe)?;
    let primary_top_layer = primary_ofe
        .layers
        .first()
        .ok_or(HillslopeRuntimeInputError::MissingSoilLayer)?;

    let primary_profile_depth_mm = primary_ofe
        .layers
        .last()
        .ok_or(HillslopeRuntimeInputError::MissingSoilLayer)?
        .depth_mm;
    if !primary_profile_depth_mm.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFiniteProfileDepth {
            value_mm: primary_profile_depth_mm,
        });
    }
    if primary_profile_depth_mm <= 0.0 {
        return Err(HillslopeRuntimeInputError::NonPositiveProfileDepth {
            value_mm: primary_profile_depth_mm,
        });
    }

    let primary_top_layer_depth_mm = primary_top_layer.depth_mm;
    if !primary_top_layer_depth_mm.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFiniteTopLayerDepth {
            value_mm: primary_top_layer_depth_mm,
        });
    }
    if primary_top_layer_depth_mm <= 0.0 {
        return Err(HillslopeRuntimeInputError::NonPositiveTopLayerDepth {
            value_mm: primary_top_layer_depth_mm,
        });
    }

    let primary_thetdr = primary_top_layer
        .theta_r_rosetta
        .or(primary_top_layer.wp_measured)
        .ok_or(HillslopeRuntimeInputError::MissingThetaResidual)?;
    if !primary_thetdr.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFiniteThetaResidual {
            value: primary_thetdr,
        });
    }

    let primary_thetfc = primary_top_layer
        .fc_rosetta
        .or(primary_top_layer.fc_measured)
        .ok_or(HillslopeRuntimeInputError::MissingThetaFieldCapacity)?;
    if !primary_thetfc.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFiniteThetaFieldCapacity {
            value: primary_thetfc,
        });
    }
    let primary_wb13_profile_symbols = compute_wb13_profile_symbols(primary_ofe, soil.datver);

    if soil.ntemp != soil.ofes.len() {
        return Err(HillslopeRuntimeInputError::SoilOfeCountMismatch {
            declared_ofe_count: soil.ntemp,
            observed_ofes: soil.ofes.len(),
        });
    }
    let ntemp = u32::try_from(soil.ntemp)
        .map_err(|_| HillslopeRuntimeInputError::SoilOfeCountOutOfRange { value: soil.ntemp })?;

    let mut state_surface = BTreeMap::new();
    state_surface.insert(
        BoundarySymbol::from("ntemp"),
        BoundaryValue::scalar(f64::from(ntemp)),
    );
    state_surface.insert(
        BoundarySymbol::from("solwpv"),
        BoundaryValue::scalar(soil.datver_raw),
    );
    if let Some(restrictive_layer) = soil.restrictive_layer.as_ref() {
        state_surface.insert(
            BoundarySymbol::from("slflag"),
            BoundaryValue::scalar(if restrictive_layer.slflag { 1.0 } else { 0.0 }),
        );
        state_surface.insert(
            BoundarySymbol::from("ui_bdrkth"),
            BoundaryValue::scalar(restrictive_layer.ui_bdrkth_mm / 1_000.0),
        );
        state_surface.insert(
            BoundarySymbol::from("kslast"),
            BoundaryValue::scalar(restrictive_layer.kslast_mm_h / 3.6e6),
        );
    }

    for (ofe_position, ofe) in soil.ofes.iter().enumerate() {
        let ofe_index = ofe_position + 1;
        let corrected_layer_runtime_symbols =
            compute_corrected_layer_runtime_symbols(ofe, ofe_index, soil.datver)?;
        let primary_wb11_layer_runtime_symbols = if ofe_index == 1 {
            Some(compute_normalized_wb11_primary_layer_runtime_symbols(
                ofe,
                ofe_index,
                soil.datver,
            )?)
        } else {
            None
        };
        let primary_layer_fc_store_mm = primary_wb11_layer_runtime_symbols
            .as_ref()
            .map(|layers| aggregate_profile_fc_store_mm_from_primary_wb11_layers(layers));
        if ofe.nsl != ofe.layers.len() {
            return Err(HillslopeRuntimeInputError::SoilLayerCountMismatch {
                ofe_index,
                declared_nsl: ofe.nsl,
                observed_layers: ofe.layers.len(),
            });
        }
        let nsl = u32::try_from(ofe.nsl).map_err(|_| {
            HillslopeRuntimeInputError::SoilLayerCountOutOfRange {
                ofe_index,
                value: ofe.nsl,
            }
        })?;
        state_surface.insert(
            soil_ofe_symbol("nsl", ofe_index),
            BoundaryValue::scalar(f64::from(nsl)),
        );
        if ofe_index == 1 {
            state_surface.insert(
                BoundarySymbol::from("nsl"),
                BoundaryValue::scalar(f64::from(nsl)),
            );
        }
        state_surface.insert(
            soil_ofe_symbol("sat", ofe_index),
            BoundaryValue::scalar(ofe.sat),
        );

        let (ofe_ksatadj, ofe_ksatfac, ofe_ksatrec, ofe_lkeff) = match &ofe.policy {
            Some(DisturbedPolicy::V9002 {
                ksatadj,
                ksatfac_mm_h,
                ksatrec_per_day,
                ..
            }) => (*ksatadj, Some(*ksatfac_mm_h), Some(*ksatrec_per_day), None),
            Some(
                DisturbedPolicy::V9003 {
                    ksatadj,
                    lkeff_mm_h,
                    ..
                }
                | DisturbedPolicy::V9005 {
                    ksatadj,
                    lkeff_mm_h,
                    ..
                },
            ) => (*ksatadj, None, None, Some(*lkeff_mm_h)),
            None => (false, None, None, None),
        };

        let ofe_ksatadj_value = if ofe_ksatadj { 1.0 } else { 0.0 };
        state_surface.insert(
            soil_ofe_symbol("ksatadj", ofe_index),
            BoundaryValue::scalar(ofe_ksatadj_value),
        );
        if let Some(value) = ofe_ksatfac {
            state_surface.insert(
                soil_ofe_symbol("ksatfac", ofe_index),
                BoundaryValue::scalar(value),
            );
        }
        if let Some(value) = ofe_ksatrec {
            state_surface.insert(
                soil_ofe_symbol("ksatrec", ofe_index),
                BoundaryValue::scalar(value),
            );
        }
        if let Some(value) = ofe_lkeff {
            state_surface.insert(
                soil_ofe_symbol("lkeff", ofe_index),
                BoundaryValue::scalar(value),
            );
        }

        let mut previous_depth_mm = 0.0_f64;
        for (layer_position, layer) in ofe.layers.iter().enumerate() {
            let layer_index = layer_position + 1;
            let layer_depth_mm = layer.depth_mm;
            if !layer_depth_mm.is_finite() {
                return Err(HillslopeRuntimeInputError::NonFiniteLayerDepth {
                    ofe_index,
                    layer_index,
                    value_mm: layer_depth_mm,
                });
            }
            if layer_depth_mm <= 0.0 {
                return Err(HillslopeRuntimeInputError::NonPositiveLayerDepth {
                    ofe_index,
                    layer_index,
                    value_mm: layer_depth_mm,
                });
            }
            if layer_depth_mm <= previous_depth_mm {
                return Err(HillslopeRuntimeInputError::NonMonotoneLayerDepth {
                    ofe_index,
                    upper_layer_index: layer_index.saturating_sub(1),
                    upper_depth_mm: previous_depth_mm,
                    lower_layer_index: layer_index,
                    lower_depth_mm: layer_depth_mm,
                });
            }

            let layer_dg_m = (layer_depth_mm - previous_depth_mm) / 1_000.0;
            let layer_solthk_m = layer_depth_mm / 1_000.0;

            let raw_layer_thetdr = layer
                .theta_r_rosetta
                .or(layer.wp_measured)
                .ok_or(HillslopeRuntimeInputError::MissingThetaResidual)?;
            if !raw_layer_thetdr.is_finite() {
                return Err(HillslopeRuntimeInputError::NonFiniteThetaResidual {
                    value: raw_layer_thetdr,
                });
            }

            let raw_layer_thetfc = layer
                .fc_rosetta
                .or(layer.fc_measured)
                .ok_or(HillslopeRuntimeInputError::MissingThetaFieldCapacity)?;
            if !raw_layer_thetfc.is_finite() {
                return Err(HillslopeRuntimeInputError::NonFiniteThetaFieldCapacity {
                    value: raw_layer_thetfc,
                });
            }
            let corrected_layer = corrected_layer_runtime_symbols.get(layer_position).ok_or(
                HillslopeRuntimeInputError::CorrectedLayerMappingIncomplete {
                    ofe_index,
                    layer_index,
                    layer_top_depth_mm: previous_depth_mm,
                    layer_bottom_depth_mm: layer_depth_mm,
                    covered_depth_mm: 0.0,
                },
            )?;
            let layer_thetfc = corrected_layer.thetfc;
            let layer_thetdr = corrected_layer.thetdr;
            let layer_porosity = corrected_layer.porosity;
            let layer_cpm = corrected_layer.cpm;
            let layer_coca = corrected_layer.coca;
            if !layer_thetfc.is_finite() {
                return Err(HillslopeRuntimeInputError::NonFiniteThetaFieldCapacity {
                    value: layer_thetfc,
                });
            }
            if !layer_thetdr.is_finite() {
                return Err(HillslopeRuntimeInputError::NonFiniteThetaResidual {
                    value: layer_thetdr,
                });
            }

            let layer_ksat_mm_h = layer.ksat_mm_h.ok_or(
                HillslopeRuntimeInputError::MissingSaturatedConductivity {
                    ofe_index,
                    layer_index,
                },
            )?;
            if !layer_ksat_mm_h.is_finite() {
                return Err(HillslopeRuntimeInputError::NonFiniteSaturatedConductivity {
                    ofe_index,
                    layer_index,
                    value_mm_h: layer_ksat_mm_h,
                });
            }
            if layer_ksat_mm_h <= 0.0 {
                return Err(
                    HillslopeRuntimeInputError::NonPositiveSaturatedConductivity {
                        ofe_index,
                        layer_index,
                        value_mm_h: layer_ksat_mm_h,
                    },
                );
            }
            let layer_ssc_m_s = layer_ksat_mm_h / 3.6e6;

            state_surface.insert(
                soil_ofe_layer_symbol("solthk", ofe_index, layer_index),
                BoundaryValue::scalar(layer_solthk_m),
            );
            state_surface.insert(
                soil_ofe_layer_symbol("dg", ofe_index, layer_index),
                BoundaryValue::scalar(layer_dg_m),
            );
            state_surface.insert(
                soil_ofe_layer_symbol("thetdr", ofe_index, layer_index),
                BoundaryValue::scalar(layer_thetdr),
            );
            state_surface.insert(
                soil_ofe_layer_symbol("thetfc", ofe_index, layer_index),
                BoundaryValue::scalar(layer_thetfc),
            );
            state_surface.insert(
                soil_ofe_layer_symbol("ssc", ofe_index, layer_index),
                BoundaryValue::scalar(layer_ssc_m_s),
            );
            state_surface.insert(
                soil_ofe_layer_symbol("por", ofe_index, layer_index),
                BoundaryValue::scalar(layer_porosity),
            );
            state_surface.insert(
                soil_ofe_layer_symbol("cpm", ofe_index, layer_index),
                BoundaryValue::scalar(layer_cpm),
            );
            state_surface.insert(
                soil_ofe_layer_symbol("coca", ofe_index, layer_index),
                BoundaryValue::scalar(layer_coca),
            );

            if ofe_index == 1 {
                state_surface.insert(
                    soil_primary_layer_symbol("solthk", layer_index),
                    BoundaryValue::scalar(layer_solthk_m),
                );
                state_surface.insert(
                    soil_primary_layer_symbol("dg", layer_index),
                    BoundaryValue::scalar(layer_dg_m),
                );
                state_surface.insert(
                    soil_primary_layer_symbol("thetdr", layer_index),
                    BoundaryValue::scalar(layer_thetdr),
                );
                state_surface.insert(
                    soil_primary_layer_symbol("thetfc", layer_index),
                    BoundaryValue::scalar(layer_thetfc),
                );
                if layer_index == 1 {
                    state_surface
                        .insert(BoundarySymbol::from("dg"), BoundaryValue::scalar(layer_dg_m));
                    state_surface.insert(
                        BoundarySymbol::from("thetdr"),
                        BoundaryValue::scalar(layer_thetdr),
                    );
                    state_surface.insert(
                        BoundarySymbol::from("thetfc"),
                        BoundaryValue::scalar(layer_thetfc),
                    );
                }
            }

            previous_depth_mm = layer_depth_mm;
        }

        if let Some(last_layer) = ofe.layers.last() {
            state_surface.insert(
                soil_ofe_symbol("solthk", ofe_index),
                BoundaryValue::scalar(last_layer.depth_mm / 1_000.0),
            );
        }

        if ofe_index == 1 {
            let primary_wb11_layers = primary_wb11_layer_runtime_symbols.as_ref().ok_or(
                HillslopeRuntimeInputError::CorrectedLayerNormalizationUnavailable { ofe_index },
            )?;
            let primary_wb11_nsl = u32::try_from(primary_wb11_layers.len()).map_err(|_| {
                HillslopeRuntimeInputError::SoilLayerCountOutOfRange {
                    ofe_index,
                    value: primary_wb11_layers.len(),
                }
            })?;
            publish_primary_wb11_runtime_layer_symbols(&mut state_surface, primary_wb11_layers);
            state_surface.insert(
                BoundarySymbol::from("wb11_nsl"),
                BoundaryValue::scalar(f64::from(primary_wb11_nsl)),
            );
            state_surface.insert(
                BoundarySymbol::from("wb19_nsl"),
                BoundaryValue::scalar(f64::from(primary_wb11_nsl)),
            );
            state_surface.insert(BoundarySymbol::from("sat"), BoundaryValue::scalar(ofe.sat));
            let profile_lateral_anisotropy_ratio = match soil.datver {
                SoilDatver::V7778 | SoilDatver::V9002 | SoilDatver::V9003 | SoilDatver::V9005 => {
                    1.0
                }
                SoilDatver::V97_5 | SoilDatver::V2006_2 | SoilDatver::V7777 => {
                    primary_top_layer.anisotropy_ratio.unwrap_or(1.0)
                }
            };
            state_surface.insert(
                BoundarySymbol::from("wb19_lateral_anisotropy_ratio"),
                BoundaryValue::scalar(profile_lateral_anisotropy_ratio),
            );
            state_surface.insert(
                BoundarySymbol::from("ksatadj"),
                BoundaryValue::scalar(ofe_ksatadj_value),
            );
            if let Some(value) = ofe_ksatfac {
                state_surface.insert(
                    BoundarySymbol::from("ksatfac"),
                    BoundaryValue::scalar(value),
                );
            }
            if let Some(value) = ofe_ksatrec {
                state_surface.insert(
                    BoundarySymbol::from("ksatrec"),
                    BoundaryValue::scalar(value),
                );
            }
            if let Some(value) = ofe_lkeff {
                state_surface.insert(BoundarySymbol::from("lkeff"), BoundaryValue::scalar(value));
            }
            state_surface.insert(
                BoundarySymbol::from("solthk"),
                BoundaryValue::scalar(
                    primary_wb11_layers
                        .last()
                        .map_or(primary_profile_depth_mm / 1_000.0, |layer| layer.solthk_m),
                ),
            );
            state_surface.insert(
                BoundarySymbol::from("salb"),
                BoundaryValue::scalar(ofe.salb),
            );

            if let Some(profile_symbols) = primary_wb13_profile_symbols {
                if let Some(layer_fc_store_mm) = primary_layer_fc_store_mm {
                    let mut profile_fc_tail_mm = profile_symbols.fc_store - layer_fc_store_mm;
                    if !profile_fc_tail_mm.is_finite() {
                        return Err(HillslopeRuntimeInputError::NonFiniteProfileFcTailContribution {
                            ofe_index,
                            value_mm: profile_fc_tail_mm,
                        });
                    }
                    if profile_fc_tail_mm < 0.0 {
                        if profile_fc_tail_mm >= -PROFILE_FC_TAIL_TOLERANCE_MM {
                            profile_fc_tail_mm = 0.0;
                        } else {
                            return Err(
                                HillslopeRuntimeInputError::NegativeProfileFcTailContribution {
                                    ofe_index,
                                    value_mm: profile_fc_tail_mm,
                                },
                            );
                        }
                    }
                    state_surface.insert(
                        BoundarySymbol::from("wb13_profile_fc_tail_mm"),
                        BoundaryValue::scalar(profile_fc_tail_mm),
                    );
                }

                state_surface.insert(
                    BoundarySymbol::from("wb13_profile_depth_mm"),
                    BoundaryValue::scalar(profile_symbols.depth),
                );
                state_surface.insert(
                    BoundarySymbol::from("wb13_profile_porosity_cap_mm"),
                    BoundaryValue::scalar(profile_symbols.porosity_cap),
                );
                state_surface.insert(
                    BoundarySymbol::from("wb13_profile_fc_store_mm"),
                    BoundaryValue::scalar(profile_symbols.fc_store),
                );
                state_surface.insert(
                    BoundarySymbol::from("wb13_profile_wp_store_mm"),
                    BoundaryValue::scalar(profile_symbols.wp_store),
                );
            }
        }
    }

    Ok(HillslopeWritebackSurface {
        state_surface,
        flux_surface: BTreeMap::new(),
    })
}

#[derive(Clone, Copy, Debug)]
struct Wb13ProfileSymbols {
    depth: f64,
    porosity_cap: f64,
    fc_store: f64,
    wp_store: f64,
}

#[derive(Clone, Copy, Debug)]
struct CorrectedLayerRuntimeSymbols {
    porosity: f64,
    cpm: f64,
    coca: f64,
    thetfc: f64,
    thetdr: f64,
}

#[derive(Clone, Copy, Debug)]
struct PrimaryWb11LayerRuntimeSymbols {
    solthk_m: f64,
    dg_m: f64,
    porosity: f64,
    cpm: f64,
    coca: f64,
    thetfc: f64,
    thetdr: f64,
    ssc_m_s: f64,
    lateral_ssh_m_s: f64,
}

#[derive(Clone, Copy, Debug)]
struct LegacySoilLayerSeed {
    depth_mm: f64,
    bulk_density_g_cm3: f64,
    fc_measured: f64,
    wp_measured: f64,
    sand_pct: f64,
    clay_pct: f64,
    orgmat_pct: f64,
    cec_meq_100g: f64,
    rock_frag_pct: f64,
    fc_wp_rock_multiplier_policy: FcWpRockMultiplierPolicy,
}

#[derive(Clone, Copy, Debug)]
struct LegacySoilLayerExpanded {
    thickness_m: f64,
    bulk_density_kg_m3: f64,
    thetfc: f64,
    thetdr: f64,
    sand: f64,
    clay: f64,
    orgmat: f64,
    cec: f64,
    rfg: f64,
    fc_wp_rock_multiplier_policy: FcWpRockMultiplierPolicy,
}

#[derive(Clone, Copy, Debug)]
struct LegacyConductivityLayer {
    thickness_m: f64,
    ksat_mm_h: f64,
    lateral_ksat_mm_h: f64,
}

#[derive(Clone, Copy, Debug)]
struct NormalizedConductivityRuntimeSymbols {
    ssc_m_s: f64,
    lateral_ssh_m_s: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FcWpRockMultiplierPolicy {
    ApplyToMeasuredFcWp,
    SkipForMeasuredFcWp,
}

const WB13_PROFILE_LAYER_THICKNESS_M: f64 = 0.2;
const LEGACY_INPUT_DELTA_EPS_M: f64 = 0.001;
const PROFILE_FC_TAIL_TOLERANCE_MM: f64 = 1.0e-9;

fn fc_wp_rock_multiplier_policy(soil_datver: SoilDatver) -> FcWpRockMultiplierPolicy {
    match soil_datver {
        SoilDatver::V7777
        | SoilDatver::V7778
        | SoilDatver::V9002
        | SoilDatver::V9003
        | SoilDatver::V9005 => FcWpRockMultiplierPolicy::ApplyToMeasuredFcWp,
        SoilDatver::V97_5 | SoilDatver::V2006_2 => FcWpRockMultiplierPolicy::SkipForMeasuredFcWp,
    }
}

fn compute_wb13_profile_symbols(
    ofe: &openwepp_input_contract::parsers::soil::SoilOfe,
    soil_datver: SoilDatver,
) -> Option<Wb13ProfileSymbols> {
    let fc_wp_policy = fc_wp_rock_multiplier_policy(soil_datver);
    let mut seeds = Vec::with_capacity(ofe.layers.len());
    for layer in &ofe.layers {
        let bulk_density = layer.bulk_density_g_cm3?;
        let fc_measured = layer.fc_measured?;
        let wp_measured = layer.wp_measured?;
        let seed = LegacySoilLayerSeed {
            depth_mm: layer.depth_mm,
            bulk_density_g_cm3: bulk_density,
            fc_measured,
            wp_measured,
            sand_pct: layer.sand_pct,
            clay_pct: layer.clay_pct,
            orgmat_pct: layer.orgmat_pct,
            cec_meq_100g: layer.cec_meq_100g,
            rock_frag_pct: layer.rock_frag_pct,
            fc_wp_rock_multiplier_policy: fc_wp_policy,
        };
        if !legacy_seed_is_finite(seed) {
            return None;
        }
        seeds.push(seed);
    }
    compute_wb13_profile_symbols_from_legacy_seed(&seeds)
}

fn legacy_seed_is_finite(seed: LegacySoilLayerSeed) -> bool {
    seed.depth_mm.is_finite()
        && seed.bulk_density_g_cm3.is_finite()
        && seed.fc_measured.is_finite()
        && seed.wp_measured.is_finite()
        && seed.sand_pct.is_finite()
        && seed.clay_pct.is_finite()
        && seed.orgmat_pct.is_finite()
        && seed.cec_meq_100g.is_finite()
        && seed.rock_frag_pct.is_finite()
}

fn compute_wb13_profile_symbols_from_legacy_seed(
    seeds: &[LegacySoilLayerSeed],
) -> Option<Wb13ProfileSymbols> {
    let expanded_layers = legacy_expand_soil_layers_to_200mm(seeds)?;
    if expanded_layers.is_empty() {
        return None;
    }

    let mut profile_depth_mm = 0.0_f64;
    let mut profile_porosity_cap_mm = 0.0_f64;
    let mut profile_fc_store_mm = 0.0_f64;
    let mut profile_wp_store_mm = 0.0_f64;

    for layer in expanded_layers {
        let corrected = legacy_correct_layer_moisture(layer)?;
        let thickness_mm = corrected.thickness_m * 1000.0;
        profile_depth_mm += thickness_mm;
        profile_porosity_cap_mm += corrected.porosity * thickness_mm;
        profile_fc_store_mm += corrected.thetfc * thickness_mm;
        profile_wp_store_mm += corrected.thetdr * thickness_mm;
    }

    Some(Wb13ProfileSymbols {
        depth: profile_depth_mm,
        porosity_cap: profile_porosity_cap_mm,
        fc_store: profile_fc_store_mm,
        wp_store: profile_wp_store_mm,
    })
}

fn compute_corrected_layer_runtime_symbols(
    ofe: &openwepp_input_contract::parsers::soil::SoilOfe,
    ofe_index: usize,
    soil_datver: SoilDatver,
) -> Result<Vec<CorrectedLayerRuntimeSymbols>, HillslopeRuntimeInputError> {
    let seeds = collect_legacy_soil_layer_seeds(ofe, ofe_index, soil_datver)?;
    let normalized_corrected_layers = compute_normalized_corrected_layer_runtime_symbols_from_legacy_seed(&seeds).ok_or(
        HillslopeRuntimeInputError::CorrectedLayerNormalizationUnavailable { ofe_index },
    )?;
    map_corrected_layer_runtime_symbols_to_parser_layers(
        ofe,
        &normalized_corrected_layers,
        ofe_index,
    )
}

fn compute_normalized_wb11_primary_layer_runtime_symbols(
    ofe: &openwepp_input_contract::parsers::soil::SoilOfe,
    ofe_index: usize,
    soil_datver: SoilDatver,
) -> Result<Vec<PrimaryWb11LayerRuntimeSymbols>, HillslopeRuntimeInputError> {
    let seeds = collect_legacy_soil_layer_seeds(ofe, ofe_index, soil_datver)?;
    let normalized_corrected_layers =
        compute_normalized_corrected_layer_runtime_symbols_from_legacy_seed(&seeds).ok_or(
            HillslopeRuntimeInputError::CorrectedLayerNormalizationUnavailable { ofe_index },
        )?;
    let normalized_conductivity_values =
        compute_normalized_conductivity_runtime_symbols(ofe, &seeds, ofe_index)?;
    if normalized_corrected_layers.len() != normalized_conductivity_values.len() {
        return Err(HillslopeRuntimeInputError::CorrectedLayerNormalizationUnavailable {
            ofe_index,
        });
    }

    let mut layers = Vec::with_capacity(normalized_corrected_layers.len());
    let mut solthk_m = 0.0_f64;
    for (layer_index, corrected_layer) in normalized_corrected_layers.into_iter().enumerate() {
        solthk_m += WB13_PROFILE_LAYER_THICKNESS_M;
        layers.push(PrimaryWb11LayerRuntimeSymbols {
            solthk_m,
            dg_m: WB13_PROFILE_LAYER_THICKNESS_M,
            porosity: corrected_layer.porosity,
            cpm: corrected_layer.cpm,
            coca: corrected_layer.coca,
            thetfc: corrected_layer.thetfc,
            thetdr: corrected_layer.thetdr,
            ssc_m_s: normalized_conductivity_values[layer_index].ssc_m_s,
            lateral_ssh_m_s: normalized_conductivity_values[layer_index].lateral_ssh_m_s,
        });
    }
    if layers.is_empty() {
        return Err(HillslopeRuntimeInputError::CorrectedLayerNormalizationUnavailable {
            ofe_index,
        });
    }
    Ok(layers)
}

fn compute_normalized_conductivity_runtime_symbols(
    ofe: &openwepp_input_contract::parsers::soil::SoilOfe,
    seeds: &[LegacySoilLayerSeed],
    ofe_index: usize,
) -> Result<Vec<NormalizedConductivityRuntimeSymbols>, HillslopeRuntimeInputError> {
    let cumulative_depths_mm = legacy_cumulative_depths_mm(seeds).ok_or(
        HillslopeRuntimeInputError::CorrectedLayerNormalizationUnavailable { ofe_index },
    )?;
    let source_layers =
        legacy_conductivity_source_layers_from_seed_depths(ofe, &cumulative_depths_mm, ofe_index)?;
    let total_depth_m = source_layers
        .iter()
        .map(|layer| layer.thickness_m)
        .sum::<f64>();
    let normalized_layer_count = legacy_normalized_layer_count(total_depth_m).ok_or(
        HillslopeRuntimeInputError::CorrectedLayerNormalizationUnavailable { ofe_index },
    )?;
    Ok(legacy_normalize_conductivity_layers_to_200mm(
        &source_layers,
        normalized_layer_count,
    ))
}

fn aggregate_profile_fc_store_mm_from_primary_wb11_layers(
    layers: &[PrimaryWb11LayerRuntimeSymbols],
) -> f64 {
    layers
        .iter()
        .map(|layer| layer.thetfc * layer.dg_m * 1_000.0)
        .sum()
}

fn publish_primary_wb11_runtime_layer_symbols(
    state_surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    layers: &[PrimaryWb11LayerRuntimeSymbols],
) {
    for (layer_position, layer) in layers.iter().enumerate() {
        let layer_index = layer_position + 1;
        state_surface.insert(
            BoundarySymbol::from(format!("wb19_solthk_{layer_index:04}")),
            BoundaryValue::scalar(layer.solthk_m),
        );
        state_surface.insert(
            BoundarySymbol::from(format!("wb19_dg_{layer_index:04}")),
            BoundaryValue::scalar(layer.dg_m),
        );
        state_surface.insert(
            BoundarySymbol::from(format!("wb18_perc_dg_{layer_index:04}")),
            BoundaryValue::scalar(layer.dg_m),
        );
        state_surface.insert(
            BoundarySymbol::from(format!("wb19_thetdr_{layer_index:04}")),
            BoundaryValue::scalar(layer.thetdr),
        );
        state_surface.insert(
            BoundarySymbol::from(format!("wb18_perc_thetdr_{layer_index:04}")),
            BoundaryValue::scalar(layer.thetdr),
        );
        state_surface.insert(
            BoundarySymbol::from(format!("wb19_thetfc_{layer_index:04}")),
            BoundaryValue::scalar(layer.thetfc),
        );
        state_surface.insert(
            BoundarySymbol::from(format!("wb18_perc_thetfc_{layer_index:04}")),
            BoundaryValue::scalar(layer.thetfc),
        );
        state_surface.insert(
            soil_primary_layer_symbol("ssc", layer_index),
            BoundaryValue::scalar(layer.ssc_m_s),
        );
        state_surface.insert(
            BoundarySymbol::from(format!("wb19_lateral_ssh_{layer_index:04}")),
            BoundaryValue::scalar(layer.lateral_ssh_m_s),
        );
        state_surface.insert(
            BoundarySymbol::from(format!("wb19_por_{layer_index:04}")),
            BoundaryValue::scalar(layer.porosity),
        );
        state_surface.insert(
            soil_primary_layer_symbol("cpm", layer_index),
            BoundaryValue::scalar(layer.cpm),
        );
        state_surface.insert(
            BoundarySymbol::from(format!("wb19_coca_{layer_index:04}")),
            BoundaryValue::scalar(layer.coca),
        );

        if layer_index == 1 {
            state_surface.insert(BoundarySymbol::from("ssc"), BoundaryValue::scalar(layer.ssc_m_s));
            state_surface.insert(
                BoundarySymbol::from("por"),
                BoundaryValue::scalar(layer.porosity),
            );
            state_surface.insert(BoundarySymbol::from("cpm"), BoundaryValue::scalar(layer.cpm));
            state_surface.insert(BoundarySymbol::from("coca"), BoundaryValue::scalar(layer.coca));
        }
    }
}

fn collect_legacy_soil_layer_seeds(
    ofe: &openwepp_input_contract::parsers::soil::SoilOfe,
    ofe_index: usize,
    soil_datver: SoilDatver,
) -> Result<Vec<LegacySoilLayerSeed>, HillslopeRuntimeInputError> {
    let fc_wp_policy = fc_wp_rock_multiplier_policy(soil_datver);
    let mut seeds = Vec::with_capacity(ofe.layers.len());
    for (layer_position, layer) in ofe.layers.iter().enumerate() {
        let layer_index = layer_position + 1;
        let bulk_density =
            layer
                .bulk_density_g_cm3
                .ok_or(HillslopeRuntimeInputError::MissingCorrectedLayerNormalizationInput {
                    ofe_index,
                    layer_index,
                    field: "bulk_density_g_cm3",
                })?;
        let fc_measured =
            layer
                .fc_measured
                .ok_or(HillslopeRuntimeInputError::MissingCorrectedLayerNormalizationInput {
                    ofe_index,
                    layer_index,
                    field: "fc_measured",
                })?;
        let wp_measured =
            layer
                .wp_measured
                .ok_or(HillslopeRuntimeInputError::MissingCorrectedLayerNormalizationInput {
                    ofe_index,
                    layer_index,
                    field: "wp_measured",
                })?;
        let seed = LegacySoilLayerSeed {
            depth_mm: layer.depth_mm,
            bulk_density_g_cm3: bulk_density,
            fc_measured,
            wp_measured,
            sand_pct: layer.sand_pct,
            clay_pct: layer.clay_pct,
            orgmat_pct: layer.orgmat_pct,
            cec_meq_100g: layer.cec_meq_100g,
            rock_frag_pct: layer.rock_frag_pct,
            fc_wp_rock_multiplier_policy: fc_wp_policy,
        };
        if !legacy_seed_is_finite(seed) {
            return Err(HillslopeRuntimeInputError::CorrectedLayerNormalizationUnavailable {
                ofe_index,
            });
        }
        seeds.push(seed);
    }
    Ok(seeds)
}

fn compute_normalized_corrected_layer_runtime_symbols_from_legacy_seed(
    seeds: &[LegacySoilLayerSeed],
) -> Option<Vec<CorrectedLayerRuntimeSymbols>> {
    let expanded_layers = legacy_expand_soil_layers_to_200mm(seeds)?;
    if expanded_layers.is_empty() {
        return None;
    }
    let mut corrected_layers = Vec::with_capacity(expanded_layers.len());
    for layer in expanded_layers {
        let corrected = legacy_correct_layer_moisture(layer)?;
        corrected_layers.push(CorrectedLayerRuntimeSymbols {
            porosity: corrected.porosity,
            cpm: corrected.cpm,
            coca: corrected.coca,
            thetfc: corrected.thetfc,
            thetdr: corrected.thetdr,
        });
    }
    if corrected_layers.is_empty() {
        return None;
    }
    Some(corrected_layers)
}

fn map_corrected_layer_runtime_symbols_to_parser_layers(
    ofe: &openwepp_input_contract::parsers::soil::SoilOfe,
    normalized_corrected_layers: &[CorrectedLayerRuntimeSymbols],
    ofe_index: usize,
) -> Result<Vec<CorrectedLayerRuntimeSymbols>, HillslopeRuntimeInputError> {
    if normalized_corrected_layers.is_empty() {
        return Err(HillslopeRuntimeInputError::CorrectedLayerNormalizationUnavailable {
            ofe_index,
        });
    }

    let mut normalized_intervals = Vec::with_capacity(normalized_corrected_layers.len());
    let mut normalized_top_mm = 0.0_f64;
    for corrected_layer in normalized_corrected_layers {
        let normalized_bottom_mm = normalized_top_mm + WB13_PROFILE_LAYER_THICKNESS_M * 1_000.0;
        normalized_intervals.push((normalized_top_mm, normalized_bottom_mm, *corrected_layer));
        normalized_top_mm = normalized_bottom_mm;
    }

    let mut mapped_layers = Vec::with_capacity(ofe.layers.len());
    let mut layer_top_depth_mm = 0.0_f64;
    for (layer_position, layer) in ofe.layers.iter().enumerate() {
        let layer_index = layer_position + 1;
        let layer_bottom_depth_mm = layer.depth_mm;
        let layer_thickness_mm = layer_bottom_depth_mm - layer_top_depth_mm;
        if !layer_thickness_mm.is_finite() || layer_thickness_mm <= 0.0 {
            return Err(HillslopeRuntimeInputError::CorrectedLayerMappingIncomplete {
                ofe_index,
                layer_index,
                layer_top_depth_mm,
                layer_bottom_depth_mm,
                covered_depth_mm: 0.0,
            });
        }

        let mut weighted_thetfc = 0.0_f64;
        let mut weighted_thetdr = 0.0_f64;
        let mut weighted_porosity = 0.0_f64;
        let mut weighted_cpm = 0.0_f64;
        let mut weighted_coca = 0.0_f64;
        let mut covered_depth_mm = 0.0_f64;

        for (normalized_top_mm, normalized_bottom_mm, corrected_layer) in &normalized_intervals {
            let overlap_top_mm = layer_top_depth_mm.max(*normalized_top_mm);
            let overlap_bottom_mm = layer_bottom_depth_mm.min(*normalized_bottom_mm);
            let overlap_depth_mm = (overlap_bottom_mm - overlap_top_mm).max(0.0);
            if overlap_depth_mm <= 0.0 {
                continue;
            }
            weighted_thetfc += corrected_layer.thetfc * overlap_depth_mm;
            weighted_thetdr += corrected_layer.thetdr * overlap_depth_mm;
            weighted_porosity += corrected_layer.porosity * overlap_depth_mm;
            weighted_cpm += corrected_layer.cpm * overlap_depth_mm;
            weighted_coca += corrected_layer.coca * overlap_depth_mm;
            covered_depth_mm += overlap_depth_mm;
        }

        if covered_depth_mm <= 0.0 || (covered_depth_mm - layer_thickness_mm).abs() > 1.0e-9 {
            return Err(HillslopeRuntimeInputError::CorrectedLayerMappingIncomplete {
                ofe_index,
                layer_index,
                layer_top_depth_mm,
                layer_bottom_depth_mm,
                covered_depth_mm,
            });
        }

        mapped_layers.push(CorrectedLayerRuntimeSymbols {
            porosity: weighted_porosity / covered_depth_mm,
            cpm: weighted_cpm / covered_depth_mm,
            coca: weighted_coca / covered_depth_mm,
            thetfc: weighted_thetfc / covered_depth_mm,
            thetdr: weighted_thetdr / covered_depth_mm,
        });
        layer_top_depth_mm = layer_bottom_depth_mm;
    }
    Ok(mapped_layers)
}

#[derive(Clone, Copy, Debug)]
struct LegacyCorrectedLayerMoisture {
    thickness_m: f64,
    porosity: f64,
    cpm: f64,
    coca: f64,
    thetfc: f64,
    thetdr: f64,
}

fn legacy_correct_layer_moisture(
    layer: LegacySoilLayerExpanded,
) -> Option<LegacyCorrectedLayerMoisture> {
    let dg = layer.thickness_m;
    if !dg.is_finite() || dg <= 0.0 {
        return None;
    }
    let solcon = legacy_solcon(layer.clay, layer.cec, layer.orgmat, dg);
    let oca = 3.80
        + 1.9 * layer.clay.powi(2)
        - (3.365 * layer.sand)
        + (12.6 * solcon * layer.clay)
        + (100.0 * layer.orgmat * (layer.sand / 2.0).powi(2));
    let coca = 1.0 - (oca / 100.0);
    if !coca.is_finite() || coca <= 0.0 || coca > 1.0 {
        return None;
    }
    let cpm = 1.0
        - ((layer.rfg * layer.bulk_density_kg_m3)
            / ((layer.rfg * layer.bulk_density_kg_m3) + 2650.0 * (1.0 - layer.rfg)));
    if !cpm.is_finite() || cpm <= 0.0 || cpm > 1.0 {
        return None;
    }

    let mut por = (2650.0 - layer.bulk_density_kg_m3) / 2650.0;
    por *= coca;
    if !por.is_finite() || por <= 0.0 {
        return None;
    }

    let apply_fc_wp_rock_multiplier = matches!(
        layer.fc_wp_rock_multiplier_policy,
        FcWpRockMultiplierPolicy::ApplyToMeasuredFcWp
    );
    let mut thetfc = layer.thetfc.max(0.0);
    let mut thetdr = layer.thetdr.max(0.0);
    if apply_fc_wp_rock_multiplier {
        thetfc *= cpm;
        thetdr *= cpm;
    }
    if !thetfc.is_finite() || !thetdr.is_finite() {
        return None;
    }
    if thetfc <= 0.0 || thetdr <= 0.0 {
        return None;
    }

    let log10 = 10_f64.ln();
    let t33 = 333.3_f64.ln() / log10;
    let t15 = 15_300.0_f64.ln() / log10;
    let s33 = thetfc.ln() / log10;
    let s15 = thetdr.ln() / log10;
    let slope = ((s15 - s33) / (t15 - t33)).abs();
    if !slope.is_finite() {
        return None;
    }
    let mut sm20c = 10.0_f64.powf(slope * (t15 - (10.0_f64.ln() / log10)) + s15);
    if apply_fc_wp_rock_multiplier {
        sm20c *= cpm;
    }
    if sm20c >= por {
        sm20c = por * 0.95;
    }
    if thetfc >= sm20c {
        let delta = thetfc - thetdr;
        thetfc = sm20c * 0.99;
        thetdr = (thetfc - delta).max(0.01);
        thetfc = thetfc.max(0.01);
    }
    if (thetfc / por) > 0.83 {
        let scale = thetfc / (por * 0.83);
        thetfc = por * 0.83;
        thetdr /= scale;
    }
    thetdr = thetdr.max(0.01);
    thetfc = thetfc.max(0.01);
    if !thetfc.is_finite() || !thetdr.is_finite() {
        return None;
    }
    if thetdr > thetfc {
        return None;
    }
    if thetfc > por {
        return None;
    }

    Some(LegacyCorrectedLayerMoisture {
        thickness_m: dg,
        porosity: por,
        cpm,
        coca,
        thetfc,
        thetdr,
    })
}

fn legacy_solcon(clay: f64, cec: f64, orgmat: f64, dg: f64) -> f64 {
    if clay <= 0.0 {
        return 0.0;
    }
    let cecc = cec - orgmat * (142.0 + 170.0 * dg);
    (cecc / (100.0 * clay)).clamp(0.15, 0.65)
}

fn legacy_expand_soil_layers_to_200mm(
    seeds: &[LegacySoilLayerSeed],
) -> Option<Vec<LegacySoilLayerExpanded>> {
    let cumulative_depths_mm = legacy_cumulative_depths_mm(seeds)?;
    let source_layers = legacy_source_layers_from_seed_depths(seeds, &cumulative_depths_mm)?;
    let total_depth_m = source_layers.iter().map(|layer| layer.thickness_m).sum::<f64>();
    let normalized_layer_count = legacy_normalized_layer_count(total_depth_m)?;
    Some(legacy_normalize_layers_to_200mm(
        &source_layers,
        normalized_layer_count,
    ))
}

fn legacy_cumulative_depths_mm(seeds: &[LegacySoilLayerSeed]) -> Option<Vec<f64>> {
    if seeds.is_empty() {
        return None;
    }
    let mut cumulative_depths_mm = seeds
        .iter()
        .map(|seed| seed.depth_mm)
        .collect::<Vec<f64>>();
    let total_thickness_mm = *cumulative_depths_mm.last()?;
    if total_thickness_mm < 200.0 {
        let deficit = 200.0 - total_thickness_mm;
        if let Some(last_depth) = cumulative_depths_mm.last_mut() {
            *last_depth += deficit;
        }
    }
    if let Some(last_depth) = cumulative_depths_mm.last_mut() {
        *last_depth += 200.0;
    }
    for depth in &mut cumulative_depths_mm {
        if *depth > 1800.0 {
            *depth = 1800.0;
        }
    }
    Some(cumulative_depths_mm)
}

fn legacy_source_layers_from_seed_depths(
    seeds: &[LegacySoilLayerSeed],
    cumulative_depths_mm: &[f64],
) -> Option<Vec<LegacySoilLayerExpanded>> {
    let mut source_layers = Vec::with_capacity(seeds.len());
    let mut previous_depth_m = 0.0_f64;
    for (seed, depth_mm) in seeds.iter().zip(cumulative_depths_mm.iter().copied()) {
        let mut bulk_density = seed.bulk_density_g_cm3;
        if bulk_density > 0.0 && bulk_density < 0.8 {
            bulk_density = 0.8;
        }
        if bulk_density > 2.0 {
            bulk_density = 2.0;
        }

        let mut orgmat_pct = seed.orgmat_pct;
        if orgmat_pct > 10.0 {
            orgmat_pct = 10.0;
        }
        let mut rock_frag_pct = seed.rock_frag_pct;
        if rock_frag_pct > 85.0 {
            rock_frag_pct = 85.0;
        }

        let depth_m = depth_mm * 0.001;
        let thickness_m = depth_m - previous_depth_m;
        previous_depth_m = depth_m;
        if thickness_m <= 0.0 {
            continue;
        }

        source_layers.push(LegacySoilLayerExpanded {
            thickness_m,
            bulk_density_kg_m3: bulk_density * 1000.0,
            thetfc: seed.fc_measured,
            thetdr: seed.wp_measured,
            sand: seed.sand_pct / 100.0,
            clay: seed.clay_pct / 100.0,
            orgmat: orgmat_pct / 100.0,
            cec: seed.cec_meq_100g,
            rfg: rock_frag_pct / 100.0,
            fc_wp_rock_multiplier_policy: seed.fc_wp_rock_multiplier_policy,
        });
    }
    if source_layers.is_empty() {
        return None;
    }
    Some(source_layers)
}

fn legacy_conductivity_source_layers_from_seed_depths(
    ofe: &openwepp_input_contract::parsers::soil::SoilOfe,
    cumulative_depths_mm: &[f64],
    ofe_index: usize,
) -> Result<Vec<LegacyConductivityLayer>, HillslopeRuntimeInputError> {
    let mut source_layers = Vec::with_capacity(ofe.layers.len());
    let mut previous_depth_m = 0.0_f64;
    for (layer_position, (layer, depth_mm)) in ofe
        .layers
        .iter()
        .zip(cumulative_depths_mm.iter().copied())
        .enumerate()
    {
        let layer_index = layer_position + 1;
        let layer_ksat_mm_h = layer.ksat_mm_h.ok_or(
            HillslopeRuntimeInputError::MissingSaturatedConductivity {
                ofe_index,
                layer_index,
            },
        )?;
        if !layer_ksat_mm_h.is_finite() {
            return Err(HillslopeRuntimeInputError::NonFiniteSaturatedConductivity {
                ofe_index,
                layer_index,
                value_mm_h: layer_ksat_mm_h,
            });
        }
        if layer_ksat_mm_h <= 0.0 {
            return Err(HillslopeRuntimeInputError::NonPositiveSaturatedConductivity {
                ofe_index,
                layer_index,
                value_mm_h: layer_ksat_mm_h,
            });
        }
        let layer_lateral_anisotropy_ratio = layer.anisotropy_ratio.unwrap_or(1.0);
        if !layer_lateral_anisotropy_ratio.is_finite() || layer_lateral_anisotropy_ratio <= 0.0 {
            return Err(HillslopeRuntimeInputError::CorrectedLayerNormalizationUnavailable {
                ofe_index,
            });
        }

        let depth_m = depth_mm * 0.001;
        let thickness_m = depth_m - previous_depth_m;
        previous_depth_m = depth_m;
        if thickness_m <= 0.0 {
            continue;
        }
        source_layers.push(LegacyConductivityLayer {
            thickness_m,
            ksat_mm_h: layer_ksat_mm_h,
            lateral_ksat_mm_h: layer_ksat_mm_h * layer_lateral_anisotropy_ratio,
        });
    }
    if source_layers.is_empty() {
        return Err(HillslopeRuntimeInputError::CorrectedLayerNormalizationUnavailable {
            ofe_index,
        });
    }
    Ok(source_layers)
}

fn legacy_normalized_layer_count(total_depth_m: f64) -> Option<usize> {
    let rounded_total_mm = (total_depth_m * 1000.0).round();
    if !rounded_total_mm.is_finite() || rounded_total_mm <= 0.0 {
        return None;
    }

    let mut remaining_mm = rounded_total_mm;
    let mut normalized_layer_count = 0usize;
    while remaining_mm >= 200.0 {
        normalized_layer_count = normalized_layer_count.checked_add(1)?;
        remaining_mm -= 200.0;
    }
    if normalized_layer_count == 0 {
        return None;
    }
    Some(normalized_layer_count)
}

fn legacy_normalize_layers_to_200mm(
    source_layers: &[LegacySoilLayerExpanded],
    normalized_layer_count: usize,
) -> Vec<LegacySoilLayerExpanded> {
    let mut remaining_thicknesses = source_layers
        .iter()
        .map(|layer| layer.thickness_m)
        .collect::<Vec<f64>>();
    let mut source_index = 0usize;
    let mut normalized_layers = Vec::with_capacity(normalized_layer_count);
    for _normalized_index in 0..normalized_layer_count {
        let mut layer = LegacySoilLayerExpanded {
            thickness_m: WB13_PROFILE_LAYER_THICKNESS_M,
            bulk_density_kg_m3: 0.0,
            thetfc: 0.0,
            thetdr: 0.0,
            sand: 0.0,
            clay: 0.0,
            orgmat: 0.0,
            cec: 0.0,
            rfg: 0.0,
            fc_wp_rock_multiplier_policy: source_layers
                .first()
                .map_or(FcWpRockMultiplierPolicy::ApplyToMeasuredFcWp, |source| {
                    source.fc_wp_rock_multiplier_policy
                }),
        };
        let mut remaining = WB13_PROFILE_LAYER_THICKNESS_M;
        while remaining > 0.0 && source_index < source_layers.len() {
            let source_thickness = remaining_thicknesses[source_index];
            if source_thickness <= 0.0 {
                source_index += 1;
                continue;
            }
            if source_thickness <= remaining {
                let fraction = source_thickness / WB13_PROFILE_LAYER_THICKNESS_M;
                legacy_accumulate_weighted_layer(&mut layer, source_layers[source_index], fraction);
                remaining -= source_thickness;
                source_index += 1;
                if remaining.abs() <= LEGACY_INPUT_DELTA_EPS_M {
                    remaining = 0.0;
                }
            } else {
                let fraction = remaining / WB13_PROFILE_LAYER_THICKNESS_M;
                legacy_accumulate_weighted_layer(&mut layer, source_layers[source_index], fraction);
                remaining_thicknesses[source_index] -= remaining;
                if remaining_thicknesses[source_index].abs() <= LEGACY_INPUT_DELTA_EPS_M {
                    source_index += 1;
                }
                remaining = 0.0;
            }
        }
        normalized_layers.push(layer);
    }
    normalized_layers
}

fn legacy_normalize_conductivity_layers_to_200mm(
    source_layers: &[LegacyConductivityLayer],
    normalized_layer_count: usize,
) -> Vec<NormalizedConductivityRuntimeSymbols> {
    let mut remaining_thicknesses = source_layers
        .iter()
        .map(|layer| layer.thickness_m)
        .collect::<Vec<f64>>();
    let mut source_index = 0usize;
    let mut normalized_conductivity_values = Vec::with_capacity(normalized_layer_count);
    for _normalized_index in 0..normalized_layer_count {
        let mut weighted_ksat_mm_h = 0.0_f64;
        let mut weighted_lateral_ksat_mm_h = 0.0_f64;
        let mut remaining = WB13_PROFILE_LAYER_THICKNESS_M;
        while remaining > 0.0 && source_index < source_layers.len() {
            let source_thickness = remaining_thicknesses[source_index];
            if source_thickness <= 0.0 {
                source_index += 1;
                continue;
            }
            if source_thickness <= remaining {
                let fraction = source_thickness / WB13_PROFILE_LAYER_THICKNESS_M;
                weighted_ksat_mm_h += source_layers[source_index].ksat_mm_h * fraction;
                weighted_lateral_ksat_mm_h +=
                    source_layers[source_index].lateral_ksat_mm_h * fraction;
                remaining -= source_thickness;
                source_index += 1;
                if remaining.abs() <= LEGACY_INPUT_DELTA_EPS_M {
                    remaining = 0.0;
                }
            } else {
                let fraction = remaining / WB13_PROFILE_LAYER_THICKNESS_M;
                weighted_ksat_mm_h += source_layers[source_index].ksat_mm_h * fraction;
                weighted_lateral_ksat_mm_h +=
                    source_layers[source_index].lateral_ksat_mm_h * fraction;
                remaining_thicknesses[source_index] -= remaining;
                if remaining_thicknesses[source_index].abs() <= LEGACY_INPUT_DELTA_EPS_M {
                    source_index += 1;
                }
                remaining = 0.0;
            }
        }
        normalized_conductivity_values.push(NormalizedConductivityRuntimeSymbols {
            ssc_m_s: weighted_ksat_mm_h / 3.6e6,
            lateral_ssh_m_s: weighted_lateral_ksat_mm_h / 3.6e6,
        });
    }
    normalized_conductivity_values
}

fn legacy_accumulate_weighted_layer(
    target: &mut LegacySoilLayerExpanded,
    source: LegacySoilLayerExpanded,
    weight: f64,
) {
    target.bulk_density_kg_m3 += source.bulk_density_kg_m3 * weight;
    target.thetfc += source.thetfc * weight;
    target.thetdr += source.thetdr * weight;
    target.sand += source.sand * weight;
    target.clay += source.clay * weight;
    target.orgmat += source.orgmat * weight;
    target.cec += source.cec * weight;
    target.rfg += source.rfg * weight;
}

/// Runtime-surface options for slope parser projection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SlopeRuntimeSurfaceOptions {
    /// Optional floor to apply when derived average slope is non-positive.
    pub non_positive_avgslp_floor: Option<f64>,
}

impl SlopeRuntimeSurfaceOptions {
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            non_positive_avgslp_floor: None,
        }
    }

    #[must_use]
    pub const fn compatibility() -> Self {
        Self {
            non_positive_avgslp_floor: Some(0.000_001),
        }
    }
}

impl Default for SlopeRuntimeSurfaceOptions {
    fn default() -> Self {
        Self::strict()
    }
}

/// Build an orchestrator-owned hillslope runtime surface from parsed slope
/// input.
///
/// Canonical slope symbols are projected with explicit index-qualified runtime
/// keys while preserving first-OFE aliases for continuity:
/// - per-OFE: `ofe{idx}_{nslpts|slplen|avgslp|azm}`
/// - per-point: `ofe{idx}_{xinput|slpinp}_{point:04}`
/// - first-OFE aliases: `nslpts`, `slplen`, `avgslp`, `azm`,
///   `{xinput|slpinp}_{point:04}`
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when required slope parser outputs are
/// missing, inconsistent, non-finite, or violate runtime guard policy.
#[allow(clippy::too_many_lines)]
pub fn build_hillslope_runtime_surface_from_slope_with_options(
    slope: &SlopeProfile,
    options: SlopeRuntimeSurfaceOptions,
) -> Result<HillslopeWritebackSurface, HillslopeRuntimeInputError> {
    validate_slope_profile_shape(slope)?;

    let mut state_surface = BTreeMap::new();
    let ofe_count = u32::try_from(slope.ofe_count).map_err(|_| {
        HillslopeRuntimeInputError::SlopeOfeCountOutOfRange {
            value: slope.ofe_count,
        }
    })?;
    state_surface.insert(
        BoundarySymbol::from("nelem"),
        BoundaryValue::scalar(f64::from(ofe_count)),
    );
    state_surface.insert(
        BoundarySymbol::from("nwsofe"),
        BoundaryValue::scalar(f64::from(ofe_count)),
    );

    for (ofe_position, ofe) in slope.ofes.iter().enumerate() {
        let ofe_index = ofe_position + 1;
        validate_slope_ofe_shape(ofe_index, ofe.nslpts, ofe.points.len())?;
        validate_slope_points(ofe_index, &ofe.points)?;

        let nslpts = u32::try_from(ofe.nslpts).map_err(|_| {
            HillslopeRuntimeInputError::SlopePointCountOutOfRange {
                ofe_index,
                value: ofe.nslpts,
            }
        })?;

        let slplen = ofe.slplen;
        if !slplen.is_finite() {
            return Err(HillslopeRuntimeInputError::NonFiniteSlopeLength {
                ofe_index,
                value_m: slplen,
            });
        }
        if slplen <= 0.0 {
            return Err(HillslopeRuntimeInputError::NonPositiveSlopeLength {
                ofe_index,
                value_m: slplen,
            });
        }

        let (avgslp, avgslp_floor_applied) =
            derive_avgslp(ofe_index, &ofe.points, options.non_positive_avgslp_floor)?;
        let azm = ofe.azm;
        state_surface.insert(
            slope_ofe_symbol("nslpts", ofe_index),
            BoundaryValue::scalar(f64::from(nslpts)),
        );
        state_surface.insert(
            slope_ofe_symbol("slplen", ofe_index),
            BoundaryValue::scalar(slplen),
        );
        state_surface.insert(
            slope_ofe_symbol("avgslp", ofe_index),
            BoundaryValue::scalar(avgslp),
        );
        if avgslp_floor_applied {
            state_surface.insert(
                slope_ofe_symbol("avgslp_floor_applied", ofe_index),
                BoundaryValue::scalar(1.0),
            );
        }
        state_surface.insert(
            slope_ofe_symbol("azm", ofe_index),
            BoundaryValue::scalar(azm),
        );

        for (point_position, point) in ofe.points.iter().enumerate() {
            let point_index = point_position + 1;
            state_surface.insert(
                slope_ofe_point_symbol("xinput", ofe_index, point_index),
                BoundaryValue::scalar(point.xinput),
            );
            state_surface.insert(
                slope_ofe_point_symbol("slpinp", ofe_index, point_index),
                BoundaryValue::scalar(point.slpinp),
            );
        }

        if ofe_index == 1 {
            state_surface.insert(
                BoundarySymbol::from("nslpts"),
                BoundaryValue::scalar(f64::from(nslpts)),
            );
            state_surface.insert(
                BoundarySymbol::from("slplen"),
                BoundaryValue::scalar(slplen),
            );
            state_surface.insert(
                BoundarySymbol::from("avgslp"),
                BoundaryValue::scalar(avgslp),
            );
            if avgslp_floor_applied {
                state_surface.insert(
                    BoundarySymbol::from("avgslp_floor_applied"),
                    BoundaryValue::scalar(1.0),
                );
            }
            state_surface.insert(BoundarySymbol::from("azm"), BoundaryValue::scalar(azm));

            for (point_position, point) in ofe.points.iter().enumerate() {
                let point_index = point_position + 1;
                state_surface.insert(
                    slope_primary_point_symbol("xinput", point_index),
                    BoundaryValue::scalar(point.xinput),
                );
                state_surface.insert(
                    slope_primary_point_symbol("slpinp", point_index),
                    BoundaryValue::scalar(point.slpinp),
                );
            }
        }
    }

    Ok(HillslopeWritebackSurface {
        state_surface,
        flux_surface: BTreeMap::new(),
    })
}

/// Build an orchestrator-owned hillslope runtime surface from parsed slope
/// input using strict nonpositive-average-slope guard semantics.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when required slope parser outputs are
/// missing, inconsistent, non-finite, or violate runtime guard policy.
pub fn build_hillslope_runtime_surface_from_slope(
    slope: &SlopeProfile,
) -> Result<HillslopeWritebackSurface, HillslopeRuntimeInputError> {
    build_hillslope_runtime_surface_from_slope_with_options(
        slope,
        SlopeRuntimeSurfaceOptions::strict(),
    )
}
