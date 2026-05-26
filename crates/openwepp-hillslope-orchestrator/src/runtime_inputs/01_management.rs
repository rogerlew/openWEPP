
/// Typed hillslope climate runtime request with precomputed boundary alias
/// projections for forcing series surfaces.
#[derive(Debug, Clone, PartialEq)]
pub struct HillslopeClimateRuntimeRequest {
    shared: SharedHillslopeClimateRuntimeRequest,
    metadata: ClimateMetadata,
    monthly: ClimateMonthlyStats,
    day_symbol_surfaces: Vec<ClimateForcingSymbolSurface>,
}

/// Typed PL management runtime projection surfaces (`PL-MAN-SEAM-001`).
#[derive(Debug, Clone, PartialEq)]
pub struct HillslopePlRuntimeSurfaces {
    pub pl_schedule_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
    pub pl_growth_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
    pub pl_decomp_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
}

impl HillslopePlRuntimeSurfaces {
    #[must_use]
    pub fn merged_state_surface(&self) -> BTreeMap<BoundarySymbol, BoundaryValue> {
        let mut merged = BTreeMap::new();
        merged.extend(self.pl_schedule_surface.clone());
        merged.extend(self.pl_growth_surface.clone());
        merged.extend(self.pl_decomp_surface.clone());
        merged
    }
}

/// Build strict typed PL runtime projection surfaces from parsed management
/// input (`PL-MAN-SEAM-001`).
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` for unsupported branches, dangling
/// references, non-finite required controls, or schedule closure violations.
#[allow(clippy::too_many_lines)]
pub fn build_hillslope_pl_runtime_surfaces_from_management(
    management: &ManagementParseOutput,
) -> Result<HillslopePlRuntimeSurfaces, HillslopeRuntimeInputError> {
    if management.topology_count != management.schedule.ofe_initial_refs.len() {
        return Err(
            HillslopeRuntimeInputError::ManagementTopologyCountMismatch {
                expected_ofes: management.topology_count,
                schedule_initial_refs: management.schedule.ofe_initial_refs.len(),
            },
        );
    }

    let expected_slots = management
        .schedule
        .rotation_repeats
        .checked_mul(management.schedule.rotation_years)
        .and_then(|value| value.checked_mul(management.topology_count))
        .ok_or(HillslopeRuntimeInputError::PlProjectionCountOutOfRange {
            field: "schedule.expected_slots",
            value: usize::MAX,
        })?;

    if management.schedule.slots.len() != expected_slots {
        return Err(
            HillslopeRuntimeInputError::ManagementScheduleSlotCountMismatch {
                expected_slots,
                observed_slots: management.schedule.slots.len(),
            },
        );
    }

    let mut pl_schedule_surface = BTreeMap::new();
    let mut pl_growth_surface = BTreeMap::new();
    let mut pl_decomp_surface = BTreeMap::new();

    pl_schedule_surface.insert(
        BoundarySymbol::from("pl_schedule_nofe"),
        BoundaryValue::scalar(usize_to_f64("pl_schedule_nofe", management.topology_count)?),
    );
    pl_schedule_surface.insert(
        BoundarySymbol::from("pl_schedule_rotation_repeats"),
        BoundaryValue::scalar(usize_to_f64(
            "pl_schedule_rotation_repeats",
            management.schedule.rotation_repeats,
        )?),
    );
    pl_schedule_surface.insert(
        BoundarySymbol::from("pl_schedule_rotation_years"),
        BoundaryValue::scalar(usize_to_f64(
            "pl_schedule_rotation_years",
            management.schedule.rotation_years,
        )?),
    );
    pl_schedule_surface.insert(
        BoundarySymbol::from("pl_schedule_slot_count"),
        BoundaryValue::scalar(usize_to_f64(
            "pl_schedule_slot_count",
            management.schedule.slots.len(),
        )?),
    );
    // Explicit scheduler preconditions from PL02 contract/baseline ordering.
    pl_schedule_surface.insert(
        BoundarySymbol::from("pl_order_decomp_before_soil"),
        BoundaryValue::scalar(1.0),
    );
    pl_schedule_surface.insert(
        BoundarySymbol::from("pl_order_growth_after_decomp"),
        BoundaryValue::scalar(1.0),
    );
    pl_schedule_surface.insert(
        BoundarySymbol::from("pl_order_watbal_after_growth"),
        BoundaryValue::scalar(1.0),
    );
    for (symbol, value) in [
        ("sumgdd", 0.0),
        ("vdmt", 0.0),
        ("cancov", 0.0),
        ("lai", 0.0),
        ("rtmass", 0.0),
        ("rtd", 0.0),
        ("hia", 0.0),
    ] {
        pl_growth_surface.insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
    }

    for (ofe_position, initial_ref) in management.schedule.ofe_initial_refs.iter().enumerate() {
        let ofe_index = ofe_position + 1;
        if *initial_ref == 0 || *initial_ref > management.registries.initials.len() {
            return Err(
                HillslopeRuntimeInputError::ManagementInitialReferenceOutOfRange {
                    ofe_index,
                    initial_ref: *initial_ref,
                    max_initial_ref: management.registries.initials.len(),
                },
            );
        }
        let initial = &management.registries.initials[*initial_ref - 1];
        if initial.meta.landuse != 1 {
            return Err(HillslopeRuntimeInputError::UnsupportedPlLanduse {
                section: "initial",
                value: initial.meta.landuse,
            });
        }
        let InitialScenarioData::Cropland(initial_data) = &initial.data;

        pl_schedule_surface.insert(
            pl_schedule_ofe_symbol("initial_ref", ofe_index),
            BoundaryValue::scalar(usize_to_f64("initial_ref", *initial_ref)?),
        );
        pl_schedule_surface.insert(
            pl_schedule_ofe_symbol("lanuse", ofe_index),
            BoundaryValue::scalar(usize_to_f64("lanuse", initial.meta.landuse)?),
        );

        pl_growth_surface.insert(
            pl_growth_ofe_symbol("imngmt_seed", ofe_index),
            BoundaryValue::scalar(usize_to_f64("imngmt_seed", initial_data.imngmt)?),
        );
        pl_growth_surface.insert(
            pl_growth_ofe_symbol("rtyp_seed", ofe_index),
            BoundaryValue::scalar(usize_to_f64("rtyp_seed", initial_data.rtyp)?),
        );

        let sumrtm = initial_data.terminal_line[0];
        if !sumrtm.is_finite() {
            return Err(HillslopeRuntimeInputError::NonFinitePlProjectionField {
                field: "sumrtm_seed",
                slot_index: 0,
                crop_slot_index: 0,
                value: sumrtm,
            });
        }
        let sumsrm = initial_data.terminal_line[1];
        if !sumsrm.is_finite() {
            return Err(HillslopeRuntimeInputError::NonFinitePlProjectionField {
                field: "sumsrm_seed",
                slot_index: 0,
                crop_slot_index: 0,
                value: sumsrm,
            });
        }
        pl_decomp_surface.insert(
            pl_decomp_ofe_symbol("iresd_seed", ofe_index),
            BoundaryValue::scalar(usize_to_f64("iresd_seed", initial_data.iresd)?),
        );
        pl_decomp_surface.insert(
            pl_decomp_ofe_symbol("sumrtm_seed", ofe_index),
            BoundaryValue::scalar(sumrtm),
        );
        pl_decomp_surface.insert(
            pl_decomp_ofe_symbol("sumsrm_seed", ofe_index),
            BoundaryValue::scalar(sumsrm),
        );

        if let Some(understory) = initial_data.understory_line {
            let usinrcol = understory[0];
            let usrilcol = understory[1];
            if !usinrcol.is_finite() {
                return Err(HillslopeRuntimeInputError::NonFinitePlProjectionField {
                    field: "usinrcol_seed",
                    slot_index: 0,
                    crop_slot_index: 0,
                    value: usinrcol,
                });
            }
            if !usrilcol.is_finite() {
                return Err(HillslopeRuntimeInputError::NonFinitePlProjectionField {
                    field: "usrilcol_seed",
                    slot_index: 0,
                    crop_slot_index: 0,
                    value: usrilcol,
                });
            }
            pl_decomp_surface.insert(
                pl_decomp_ofe_symbol("usinrcol_seed", ofe_index),
                BoundaryValue::scalar(usinrcol),
            );
            pl_decomp_surface.insert(
                pl_decomp_ofe_symbol("usrilcol_seed", ofe_index),
                BoundaryValue::scalar(usrilcol),
            );
        }

        if ofe_index == 1 {
            pl_schedule_surface.insert(
                BoundarySymbol::from("lanuse"),
                BoundaryValue::scalar(usize_to_f64("lanuse", initial.meta.landuse)?),
            );
            pl_growth_surface.insert(
                BoundarySymbol::from("imngmt_seed"),
                BoundaryValue::scalar(usize_to_f64("imngmt_seed", initial_data.imngmt)?),
            );
            pl_decomp_surface.insert(
                BoundarySymbol::from("iresd_seed"),
                BoundaryValue::scalar(usize_to_f64("iresd_seed", initial_data.iresd)?),
            );
            pl_decomp_surface.insert(
                BoundarySymbol::from("sumrtm_seed"),
                BoundaryValue::scalar(sumrtm),
            );
            pl_decomp_surface.insert(
                BoundarySymbol::from("sumsrm_seed"),
                BoundaryValue::scalar(sumsrm),
            );
        }
    }

    for (slot_position, slot) in management.schedule.slots.iter().enumerate() {
        let slot_index = slot_position + 1;
        if slot.crop_slots != slot.yearly_refs.len() {
            return Err(
                HillslopeRuntimeInputError::ManagementScheduleSlotArityMismatch {
                    slot_index,
                    crop_slots: slot.crop_slots,
                    yearly_refs: slot.yearly_refs.len(),
                },
            );
        }

        let ofe_index = slot.ofe_index + 1;
        if ofe_index == 0 || ofe_index > management.topology_count {
            return Err(
                HillslopeRuntimeInputError::ManagementScheduleOfeIndexOutOfRange {
                    slot_index,
                    ofe_index,
                    max_ofe_index: management.topology_count,
                },
            );
        }

        pl_schedule_surface.insert(
            pl_schedule_slot_symbol("rotation_index", slot_index),
            BoundaryValue::scalar(usize_to_f64("rotation_index", slot.rotation_index + 1)?),
        );
        pl_schedule_surface.insert(
            pl_schedule_slot_symbol("year_in_rotation", slot_index),
            BoundaryValue::scalar(usize_to_f64("year_in_rotation", slot.year_in_rotation + 1)?),
        );
        pl_schedule_surface.insert(
            pl_schedule_slot_symbol("ofe_index", slot_index),
            BoundaryValue::scalar(usize_to_f64("ofe_index", ofe_index)?),
        );
        pl_schedule_surface.insert(
            pl_schedule_slot_symbol("crop_slots", slot_index),
            BoundaryValue::scalar(usize_to_f64("crop_slots", slot.crop_slots)?),
        );

        for (crop_slot_position, yearly_ref) in slot.yearly_refs.iter().enumerate() {
            let crop_slot_index = crop_slot_position + 1;
            if *yearly_ref == 0 || *yearly_ref > management.registries.yearlies.len() {
                return Err(
                    HillslopeRuntimeInputError::ManagementYearlyReferenceOutOfRange {
                        slot_index,
                        crop_slot_index,
                        yearly_ref: *yearly_ref,
                        max_yearly_ref: management.registries.yearlies.len(),
                    },
                );
            }
            let yearly = &management.registries.yearlies[*yearly_ref - 1];
            if yearly.meta.landuse != 1 {
                return Err(HillslopeRuntimeInputError::UnsupportedPlLanduse {
                    section: "yearly",
                    value: yearly.meta.landuse,
                });
            }
            let YearlyScenarioData::Cropland(cropland) = &yearly.data;

            pl_schedule_surface.insert(
                pl_schedule_slot_crop_symbol("yearly_ref", slot_index, crop_slot_index),
                BoundaryValue::scalar(usize_to_f64("yearly_ref", *yearly_ref)?),
            );
            pl_schedule_surface.insert(
                pl_schedule_slot_crop_symbol("lanuse", slot_index, crop_slot_index),
                BoundaryValue::scalar(usize_to_f64("lanuse", yearly.meta.landuse)?),
            );
            pl_schedule_surface.insert(
                pl_schedule_slot_crop_symbol("itype", slot_index, crop_slot_index),
                BoundaryValue::scalar(usize_to_f64("itype", cropland.itype)?),
            );
            pl_schedule_surface.insert(
                pl_schedule_slot_crop_symbol("tilseq", slot_index, crop_slot_index),
                BoundaryValue::scalar(usize_to_f64("tilseq", cropland.tilseq)?),
            );
            pl_schedule_surface.insert(
                pl_schedule_slot_crop_symbol("conset", slot_index, crop_slot_index),
                BoundaryValue::scalar(usize_to_f64("conset", cropland.conset)?),
            );
            pl_schedule_surface.insert(
                pl_schedule_slot_crop_symbol("drset", slot_index, crop_slot_index),
                BoundaryValue::scalar(usize_to_f64("drset", cropland.drset)?),
            );
            pl_schedule_surface.insert(
                pl_schedule_slot_crop_symbol("imngmt", slot_index, crop_slot_index),
                BoundaryValue::scalar(usize_to_f64("imngmt", cropland.imngmt)?),
            );

            pl_growth_surface.insert(
                pl_growth_slot_crop_symbol("itype", slot_index, crop_slot_index),
                BoundaryValue::scalar(usize_to_f64("itype", cropland.itype)?),
            );
            pl_growth_surface.insert(
                pl_growth_slot_crop_symbol("imngmt", slot_index, crop_slot_index),
                BoundaryValue::scalar(usize_to_f64("imngmt", cropland.imngmt)?),
            );

            if cropland.itype == 0 || cropland.itype > management.registries.plants.len() {
                return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
                    field: "itype",
                    slot_index,
                    crop_slot_index,
                    value: usize_to_f64("itype", cropland.itype)?,
                    allowed: "1..=plant_scenario_count",
                });
            }
            let plant = &management.registries.plants[cropland.itype - 1];
            let PlantScenarioData::Cropland(plant_cropland) = &plant.data;
            project_growth_equation_symbols(
                &mut pl_growth_surface,
                slot_index,
                crop_slot_index,
                plant_cropland,
            )?;
            project_decomposition_equation_symbols(
                &mut pl_decomp_surface,
                slot_index,
                crop_slot_index,
                plant_cropland,
            )?;
            if slot_index == 1 && crop_slot_index == 1 {
                project_primary_growth_equation_aliases(&mut pl_growth_surface, plant_cropland)?;
                project_primary_decomposition_equation_aliases(
                    &mut pl_decomp_surface,
                    plant_cropland,
                )?;
            }

            match &cropland.branch {
                YearlyCroplandBranch::AnnualOrFallow(annual) => {
                    if !annual.rw.is_finite() {
                        return Err(HillslopeRuntimeInputError::NonFinitePlProjectionField {
                            field: "rw",
                            slot_index,
                            crop_slot_index,
                            value: annual.rw,
                        });
                    }
                    let jdharv = validate_projection_day(
                        "jdharv",
                        slot_index,
                        crop_slot_index,
                        annual.jdharv,
                        false,
                    )?;
                    let jdplt = validate_projection_day(
                        "jdplt",
                        slot_index,
                        crop_slot_index,
                        annual.jdplt,
                        false,
                    )?;
                    pl_growth_surface.insert(
                        pl_growth_slot_crop_symbol("jdharv", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("jdharv", jdharv)?),
                    );
                    pl_growth_surface.insert(
                        pl_growth_slot_crop_symbol("jdplt", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("jdplt", jdplt)?),
                    );
                    pl_growth_surface.insert(
                        pl_growth_slot_crop_symbol("rw", slot_index, crop_slot_index),
                        BoundaryValue::scalar(annual.rw),
                    );
                    pl_decomp_surface.insert(
                        pl_decomp_slot_crop_symbol("resmgt", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("resmgt", annual.resmgt)?),
                    );
                    let annual_extension_projection = project_annual_extension_controls(
                        slot_index,
                        crop_slot_index,
                        annual.resmgt,
                        annual.extension.as_ref(),
                    )?;
                    project_annual_extension_symbols(
                        &mut pl_decomp_surface,
                        slot_index,
                        crop_slot_index,
                        &annual_extension_projection,
                    )?;

                    if slot_index == 1 && crop_slot_index == 1 {
                        pl_growth_surface.insert(
                            BoundarySymbol::from("itype"),
                            BoundaryValue::scalar(usize_to_f64("itype", cropland.itype)?),
                        );
                        pl_growth_surface.insert(
                            BoundarySymbol::from("imngmt"),
                            BoundaryValue::scalar(usize_to_f64("imngmt", cropland.imngmt)?),
                        );
                        pl_growth_surface.insert(
                            BoundarySymbol::from("jdharv"),
                            BoundaryValue::scalar(usize_to_f64("jdharv", jdharv)?),
                        );
                        pl_growth_surface.insert(
                            BoundarySymbol::from("jdplt"),
                            BoundaryValue::scalar(usize_to_f64("jdplt", jdplt)?),
                        );
                        pl_growth_surface
                            .insert(BoundarySymbol::from("rw"), BoundaryValue::scalar(annual.rw));
                        pl_decomp_surface.insert(
                            BoundarySymbol::from("resmgt"),
                            BoundaryValue::scalar(usize_to_f64("resmgt", annual.resmgt)?),
                        );
                        project_primary_annual_extension_aliases(
                            &mut pl_decomp_surface,
                            &annual_extension_projection,
                        )?;
                    }
                }
                YearlyCroplandBranch::Perennial(perennial) => {
                    if perennial.mgtopt > 3 {
                        return Err(HillslopeRuntimeInputError::UnsupportedPlManagementOption {
                            field: "mgtopt",
                            value: perennial.mgtopt,
                            allowed: "1..3",
                        });
                    }
                    if !perennial.rw.is_finite() {
                        return Err(HillslopeRuntimeInputError::NonFinitePlProjectionField {
                            field: "rw",
                            slot_index,
                            crop_slot_index,
                            value: perennial.rw,
                        });
                    }
                    let jdharv = validate_projection_day(
                        "jdharv",
                        slot_index,
                        crop_slot_index,
                        perennial.jdharv,
                        true,
                    )?;
                    let jdplt = validate_projection_day(
                        "jdplt",
                        slot_index,
                        crop_slot_index,
                        perennial.jdplt,
                        true,
                    )?;
                    let jdstop = validate_projection_day(
                        "jdstop",
                        slot_index,
                        crop_slot_index,
                        perennial.jdstop,
                        true,
                    )?;
                    pl_growth_surface.insert(
                        pl_growth_slot_crop_symbol("jdharv", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("jdharv", jdharv)?),
                    );
                    pl_growth_surface.insert(
                        pl_growth_slot_crop_symbol("jdplt", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("jdplt", jdplt)?),
                    );
                    pl_growth_surface.insert(
                        pl_growth_slot_crop_symbol("jdstop", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("jdstop", jdstop)?),
                    );
                    pl_growth_surface.insert(
                        pl_growth_slot_crop_symbol("rw", slot_index, crop_slot_index),
                        BoundaryValue::scalar(perennial.rw),
                    );
                    pl_growth_surface.insert(
                        pl_growth_slot_crop_symbol("mgtopt", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("mgtopt", perennial.mgtopt)?),
                    );

                    pl_decomp_surface.insert(
                        pl_decomp_slot_crop_symbol("mgtopt", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("mgtopt", perennial.mgtopt)?),
                    );
                    let (ncut, ncycle) = match perennial.mgtopt {
                        1 => {
                            if perennial.cut_days.is_empty() {
                                return Err(
                                    HillslopeRuntimeInputError::PlProjectionCardinalityInvalid {
                                        field: "ncut",
                                        slot_index,
                                        crop_slot_index,
                                        value: 0,
                                        expected: ">=1 for mgtopt=1",
                                    },
                                );
                            }
                            if !perennial.grazing_cycles.is_empty() {
                                return Err(
                                    HillslopeRuntimeInputError::PlProjectionUnsupportedPayloadCombination {
                                        field: "grazing_cycles",
                                        slot_index,
                                        crop_slot_index,
                                        reason: "mgtopt=1 requires empty grazing_cycles",
                                    },
                                );
                            }

                            project_perennial_cutday_symbols(
                                &mut pl_decomp_surface,
                                slot_index,
                                crop_slot_index,
                                perennial,
                            )?;
                            (perennial.cut_days.len(), 0)
                        }
                        2 => {
                            if perennial.grazing_cycles.is_empty() {
                                return Err(
                                    HillslopeRuntimeInputError::PlProjectionCardinalityInvalid {
                                        field: "ncycle",
                                        slot_index,
                                        crop_slot_index,
                                        value: 0,
                                        expected: ">=1 for mgtopt=2",
                                    },
                                );
                            }
                            if !perennial.cut_days.is_empty() {
                                return Err(
                                    HillslopeRuntimeInputError::PlProjectionUnsupportedPayloadCombination {
                                        field: "cut_days",
                                        slot_index,
                                        crop_slot_index,
                                        reason: "mgtopt=2 requires empty cut_days",
                                    },
                                );
                            }

                            project_perennial_grazing_cycle_symbols(
                                &mut pl_decomp_surface,
                                slot_index,
                                crop_slot_index,
                                perennial,
                            )?;
                            (0, perennial.grazing_cycles.len())
                        }
                        3 => {
                            if !perennial.cut_days.is_empty() {
                                return Err(
                                    HillslopeRuntimeInputError::PlProjectionUnsupportedPayloadCombination {
                                        field: "cut_days",
                                        slot_index,
                                        crop_slot_index,
                                        reason: "mgtopt=3 requires empty cut_days",
                                    },
                                );
                            }
                            if !perennial.grazing_cycles.is_empty() {
                                return Err(
                                    HillslopeRuntimeInputError::PlProjectionUnsupportedPayloadCombination {
                                        field: "grazing_cycles",
                                        slot_index,
                                        crop_slot_index,
                                        reason: "mgtopt=3 requires empty grazing_cycles",
                                    },
                                );
                            }
                            (0, 0)
                        }
                        _ => {
                            return Err(
                                HillslopeRuntimeInputError::UnsupportedPlManagementOption {
                                    field: "mgtopt",
                                    value: perennial.mgtopt,
                                    allowed: "1..3",
                                },
                            );
                        }
                    };
                    pl_decomp_surface.insert(
                        pl_decomp_slot_crop_symbol("ncut", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("ncut", ncut)?),
                    );
                    pl_decomp_surface.insert(
                        pl_decomp_slot_crop_symbol("ncycle", slot_index, crop_slot_index),
                        BoundaryValue::scalar(usize_to_f64("ncycle", ncycle)?),
                    );
                }
            }
        }
    }

    Ok(HillslopePlRuntimeSurfaces {
        pl_schedule_surface,
        pl_growth_surface,
        pl_decomp_surface,
    })
}

/// Build one merged state surface from strict PL management projection
/// sub-surfaces.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when PL management projection fails.
pub fn build_hillslope_runtime_surface_from_management(
    management: &ManagementParseOutput,
) -> Result<HillslopeWritebackSurface, HillslopeRuntimeInputError> {
    let pl_surfaces = build_hillslope_pl_runtime_surfaces_from_management(management)?;
    Ok(HillslopeWritebackSurface {
        state_surface: pl_surfaces.merged_state_surface(),
        flux_surface: BTreeMap::new(),
    })
}

