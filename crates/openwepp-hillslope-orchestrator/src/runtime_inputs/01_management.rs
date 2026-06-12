
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
        ("pltol", 0.25),
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

        let cancov_seed =
            validate_projection_fraction("cancov_seed", 0, 0, initial_data.base_line[1])?;
        let inrcov_seed =
            validate_projection_fraction("inrcov_seed", 0, 0, initial_data.base_line[5])?;
        let rilcov_seed =
            validate_projection_fraction("rilcov_seed", 0, 0, initial_data.residue_line[2])?;
        let rrinit_seed =
            validate_projection_non_negative("rrinit_seed", 0, 0, initial_data.residue_line[3])?;
        let rspace_seed =
            validate_projection_non_negative("rspace_seed", 0, 0, initial_data.residue_line[4])?;
        let tillay1_seed = validate_projection_non_negative(
            "tillay1_seed",
            0,
            0,
            initial_data.thaw_line[2],
        )?;
        let tillay2_seed = validate_projection_non_negative(
            "tillay2_seed",
            0,
            0,
            initial_data.thaw_line[3],
        )?;
        let width_seed =
            validate_projection_non_negative("width_seed", 0, 0, initial_data.thaw_line[4])?;

        if initial_data.iresd == 0 || initial_data.iresd > management.registries.plants.len() {
            return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
                field: "iresd_seed",
                slot_index: 0,
                crop_slot_index: 0,
                value: usize_to_f64("iresd_seed", initial_data.iresd)?,
                allowed: "1..=plant_scenario_count",
            });
        }
        let residue_plant = &management.registries.plants[initial_data.iresd - 1];
        let PlantScenarioData::Cropland(residue_plant_cropland) = &residue_plant.data;
        let residue_depth_m = legacy_initial_residue_depth_m(
            initial.meta.landuse,
            residue_plant_cropland.canopy_line[4],
            residue_plant_cropland.canopy_line[9],
            inrcov_seed,
            rilcov_seed,
            rspace_seed,
            width_seed,
        )?;
        let canopy_cover_coeff_seed = validate_projection_non_negative(
            "bb_seed",
            0,
            0,
            residue_plant_cropland.canopy_line[0],
        )?;
        let canopy_height_curve_seed = validate_projection_non_negative(
            "bbb_seed",
            0,
            0,
            residue_plant_cropland.canopy_line[1],
        )?;
        let flivmx_seed = validate_projection_non_negative(
            "flivmx_seed",
            0,
            0,
            residue_plant_cropland.growth_line[4],
        )?;
        let hmax_seed = validate_projection_non_negative(
            "hmax_seed",
            0,
            0,
            residue_plant_cropland.growth_line[7],
        )?;

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
        pl_growth_surface.insert(
            pl_growth_ofe_symbol("cancov_seed", ofe_index),
            BoundaryValue::scalar(cancov_seed),
        );
        pl_growth_surface.insert(
            pl_growth_ofe_symbol("bb_seed", ofe_index),
            BoundaryValue::scalar(canopy_cover_coeff_seed),
        );
        pl_growth_surface.insert(
            pl_growth_ofe_symbol("bbb_seed", ofe_index),
            BoundaryValue::scalar(canopy_height_curve_seed),
        );
        pl_growth_surface.insert(
            pl_growth_ofe_symbol("flivmx_seed", ofe_index),
            BoundaryValue::scalar(flivmx_seed),
        );
        pl_growth_surface.insert(
            pl_growth_ofe_symbol("hmax_seed", ofe_index),
            BoundaryValue::scalar(hmax_seed),
        );
        pl_growth_surface.insert(
            slope_ofe_symbol("inrcov", ofe_index),
            BoundaryValue::scalar(inrcov_seed),
        );
        pl_growth_surface.insert(
            slope_ofe_symbol("rilcov", ofe_index),
            BoundaryValue::scalar(rilcov_seed),
        );
        pl_growth_surface.insert(
            slope_ofe_symbol("rrinit", ofe_index),
            BoundaryValue::scalar(rrinit_seed),
        );
        pl_growth_surface.insert(
            slope_ofe_symbol("rspace", ofe_index),
            BoundaryValue::scalar(rspace_seed),
        );
        pl_growth_surface.insert(
            pl_growth_ofe_symbol("tillay1_m", ofe_index),
            BoundaryValue::scalar(tillay1_seed),
        );
        pl_growth_surface.insert(
            pl_growth_ofe_symbol("tillay2_m", ofe_index),
            BoundaryValue::scalar(tillay2_seed),
        );
        pl_growth_surface.insert(
            slope_ofe_symbol("width", ofe_index),
            BoundaryValue::scalar(width_seed),
        );
        pl_growth_surface.insert(
            slope_ofe_symbol("rtyp", ofe_index),
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
        pl_decomp_surface.insert(
            pl_decomp_ofe_symbol("residue_depth_m_seed", ofe_index),
            BoundaryValue::scalar(residue_depth_m),
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
            pl_growth_surface.insert(
                BoundarySymbol::from("cancov"),
                BoundaryValue::scalar(cancov_seed),
            );
            pl_growth_surface.insert(
                BoundarySymbol::from("inrcov"),
                BoundaryValue::scalar(inrcov_seed),
            );
            pl_growth_surface.insert(
                BoundarySymbol::from("rilcov"),
                BoundaryValue::scalar(rilcov_seed),
            );
            pl_growth_surface.insert(
                BoundarySymbol::from("rrinit"),
                BoundaryValue::scalar(rrinit_seed),
            );
            pl_growth_surface.insert(
                BoundarySymbol::from("rspace"),
                BoundaryValue::scalar(rspace_seed),
            );
            pl_growth_surface.insert(
                BoundarySymbol::from("management.initial.params.tillay1_m"),
                BoundaryValue::scalar(tillay1_seed),
            );
            pl_growth_surface.insert(
                BoundarySymbol::from("management.initial.params.tillay2_m"),
                BoundaryValue::scalar(tillay2_seed),
            );
            pl_growth_surface.insert(
                BoundarySymbol::from("width"),
                BoundaryValue::scalar(width_seed),
            );
            pl_growth_surface.insert(
                BoundarySymbol::from("rtyp"),
                BoundaryValue::scalar(usize_to_f64("rtyp_seed", initial_data.rtyp)?),
            );
            pl_growth_surface.insert(
                BoundarySymbol::from("bb_seed"),
                BoundaryValue::scalar(canopy_cover_coeff_seed),
            );
            pl_growth_surface.insert(
                BoundarySymbol::from("bbb_seed"),
                BoundaryValue::scalar(canopy_height_curve_seed),
            );
            pl_growth_surface.insert(
                BoundarySymbol::from("flivmx_seed"),
                BoundaryValue::scalar(flivmx_seed),
            );
            pl_growth_surface.insert(
                BoundarySymbol::from("hmax_seed"),
                BoundaryValue::scalar(hmax_seed),
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
            pl_decomp_surface.insert(
                BoundarySymbol::from("frost.runtime_residue_depth_m"),
                BoundaryValue::scalar(residue_depth_m),
            );
            pl_decomp_surface.insert(
                BoundarySymbol::from("resdep"),
                BoundaryValue::scalar(residue_depth_m),
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
                pl_schedule_surface.insert(
                    BoundarySymbol::from("drset"),
                    BoundaryValue::scalar(usize_to_f64("drset", cropland.drset)?),
                );
                if cropland.drset == 0 {
                    pl_schedule_surface.insert(
                        BoundarySymbol::from("wb19_drain_enabled"),
                        BoundaryValue::scalar(0.0),
                    );
                } else {
                    let max_drain_ref = management.registries.drains.len();
                    if cropland.drset > max_drain_ref {
                        return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
                            field: "drset",
                            slot_index,
                            crop_slot_index,
                            value: usize_to_f64("drset", cropland.drset)?,
                            allowed: "1..=drain_scenario_count",
                        });
                    }
                    let drain = &management.registries.drains[cropland.drset - 1];
                    let drain_depth = validate_projection_non_negative(
                        "wb19_drain_depth",
                        slot_index,
                        crop_slot_index,
                        drain.ddrain,
                    )?;
                    let drain_spacing = validate_projection_non_negative(
                        "wb19_drain_spacing",
                        slot_index,
                        crop_slot_index,
                        drain.sdrain,
                    )?;
                    let drain_diameter = validate_projection_non_negative(
                        "wb19_drain_diameter",
                        slot_index,
                        crop_slot_index,
                        drain.drdiam,
                    )?;
                    if drain_depth <= 0.0 {
                        return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
                            field: "wb19_drain_depth",
                            slot_index,
                            crop_slot_index,
                            value: drain_depth,
                            allowed: "> 0.0 when wb19_drain_enabled=1",
                        });
                    }
                    if drain_spacing <= 0.0 {
                        return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
                            field: "wb19_drain_spacing",
                            slot_index,
                            crop_slot_index,
                            value: drain_spacing,
                            allowed: "> 0.0 when wb19_drain_enabled=1",
                        });
                    }
                    if drain_diameter <= 0.0 {
                        return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
                            field: "wb19_drain_diameter",
                            slot_index,
                            crop_slot_index,
                            value: drain_diameter,
                            allowed: "> 0.0 when wb19_drain_enabled=1",
                        });
                    }
                    pl_schedule_surface.insert(
                        BoundarySymbol::from("wb19_drain_enabled"),
                        BoundaryValue::scalar(1.0),
                    );
                    pl_schedule_surface.insert(
                        BoundarySymbol::from("wb19_drain_depth"),
                        BoundaryValue::scalar(drain_depth),
                    );
                    pl_schedule_surface.insert(
                        BoundarySymbol::from("wb19_drain_spacing"),
                        BoundaryValue::scalar(drain_spacing),
                    );
                    pl_schedule_surface.insert(
                        BoundarySymbol::from("wb19_drain_diameter"),
                        BoundaryValue::scalar(drain_diameter),
                    );
                }
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

    apply_primary_initial_live_canopy_assimilation(&mut pl_growth_surface)?;

    Ok(HillslopePlRuntimeSurfaces {
        pl_schedule_surface,
        pl_growth_surface,
        pl_decomp_surface,
    })
}

fn legacy_initial_residue_depth_m(
    landuse: usize,
    cover_factor: f64,
    stem_diameter_m: f64,
    inrcov: f64,
    rilcov: f64,
    rspace_m: f64,
    width_m: f64,
) -> Result<f64, HillslopeRuntimeInputError> {
    let cover_factor = validate_projection_positive("cf", 0, 0, cover_factor)?;
    let stem_diameter_m = validate_projection_non_negative("diam", 0, 0, stem_diameter_m)?;
    let inrcov = inrcov.min(0.999);
    let rilcov = rilcov.min(0.999);
    let rspace_m = if rspace_m <= 0.0 { 1.0 } else { rspace_m };
    let width_m = width_m.min(rspace_m);
    let wght1 = (rspace_m - width_m) / rspace_m;
    let rill_mass_kg_m2 = if rilcov <= 0.0 {
        0.0
    } else {
        (1.0 - rilcov).ln() / -cover_factor
    };
    let interrill_mass_kg_m2 = if inrcov <= 0.0 {
        0.0
    } else {
        (1.0 - inrcov).ln() / -cover_factor
    };
    let ground_residue_mass_kg_m2 =
        wght1 * interrill_mass_kg_m2 + (1.0 - wght1) * rill_mass_kg_m2;
    let conversion_factor = legacy_residue_depth_conversion_factor(landuse, stem_diameter_m);
    if conversion_factor <= 0.0 {
        Ok(0.0)
    } else {
        Ok(ground_residue_mass_kg_m2 / (conversion_factor * 100.0))
    }
}

fn legacy_residue_depth_conversion_factor(landuse: usize, stem_diameter_m: f64) -> f64 {
    if landuse == 1 {
        if (0.03..=0.06).contains(&stem_diameter_m) {
            0.174
        } else if (0.007..0.03).contains(&stem_diameter_m) {
            0.6
        } else if (0.001..0.007).contains(&stem_diameter_m) {
            0.3
        } else {
            0.6
        }
    } else {
        0.6
    }
}

#[allow(clippy::too_many_lines)]
fn apply_primary_initial_live_canopy_assimilation(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<(), HillslopeRuntimeInputError> {
    let slot_index = 1;
    let crop_slot_index = 1;
    let imngmt = projection_usize_from_surface(
        surface,
        &pl_growth_slot_crop_symbol("imngmt", slot_index, crop_slot_index),
        "imngmt",
        slot_index,
        crop_slot_index,
    )?;
    let jdharv = projection_usize_from_surface(
        surface,
        &pl_growth_slot_crop_symbol("jdharv", slot_index, crop_slot_index),
        "jdharv",
        slot_index,
        crop_slot_index,
    )?;
    let jdplt = projection_usize_from_surface(
        surface,
        &pl_growth_slot_crop_symbol("jdplt", slot_index, crop_slot_index),
        "jdplt",
        slot_index,
        crop_slot_index,
    )?;

    let mut cancov = projection_f64_from_surface(
        surface,
        &BoundarySymbol::from("cancov"),
        "cancov",
        slot_index,
        crop_slot_index,
    )?;

    if imngmt == 3 || (imngmt == 1 && jdplt < jdharv) || (imngmt == 2 && jdplt > 0) {
        cancov = 0.0;
    }
    if cancov >= PL_GROWTH_CANCOV_MAX {
        cancov = PL_GROWTH_CANCOV_MAX;
    }
    surface.insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(cancov));

    let bb = projection_f64_from_surface(
        surface,
        &pl_growth_slot_crop_symbol("bb", slot_index, crop_slot_index),
        "bb",
        slot_index,
        crop_slot_index,
    )?;
    let bbb = projection_f64_from_surface(
        surface,
        &pl_growth_slot_crop_symbol("bbb", slot_index, crop_slot_index),
        "bbb",
        slot_index,
        crop_slot_index,
    )?;
    let hmax = projection_f64_from_surface(
        surface,
        &pl_growth_slot_crop_symbol("hmax", slot_index, crop_slot_index),
        "hmax",
        slot_index,
        crop_slot_index,
    )?;
    let xmxlai = projection_f64_from_surface(
        surface,
        &pl_growth_slot_crop_symbol("xmxlai", slot_index, crop_slot_index),
        "xmxlai",
        slot_index,
        crop_slot_index,
    )?;
    let gddmax = projection_f64_from_surface(
        surface,
        &pl_growth_slot_crop_symbol("gddmax", slot_index, crop_slot_index),
        "gddmax",
        slot_index,
        crop_slot_index,
    )?;
    let rsr = projection_f64_from_surface(
        surface,
        &pl_growth_slot_crop_symbol("rsr", slot_index, crop_slot_index),
        "rsr",
        slot_index,
        crop_slot_index,
    )?;
    let rdmax = projection_f64_from_surface(
        surface,
        &pl_growth_slot_crop_symbol("rdmax", slot_index, crop_slot_index),
        "rdmax",
        slot_index,
        crop_slot_index,
    )?;
    let rtmmax = projection_f64_from_surface(
        surface,
        &pl_growth_slot_crop_symbol("rtmmax", slot_index, crop_slot_index),
        "rtmmax",
        slot_index,
        crop_slot_index,
    )?;

    let mut vdmt = 0.0;
    let mut lai = 0.0;
    let mut canhgt = 0.0;
    if cancov > 0.0 {
        if bb <= 0.0 {
            return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
                field: "bb",
                slot_index,
                crop_slot_index,
                value: bb,
                allowed: ">0.0 when initial cancov > 0.0",
            });
        }
        if xmxlai <= 0.0 {
            return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
                field: "xmxlai",
                slot_index,
                crop_slot_index,
                value: xmxlai,
                allowed: ">0.0 when initial cancov > 0.0",
            });
        }
        vdmt = ((1.0 - cancov).ln() / -bb).max(0.0);
        canhgt = (1.0 - (-bbb * vdmt).exp()) * hmax;
        lai = if imngmt == 1 {
            xmxlai * vdmt
                / (vdmt + PL_GROWTH_ANNUAL_LAI_A * (-PL_GROWTH_ANNUAL_LAI_B * vdmt).exp())
        } else {
            xmxlai * vdmt
                / (vdmt
                    + PL_GROWTH_PERENNIAL_LAI_A
                        * (-PL_GROWTH_PERENNIAL_LAI_B * vdmt).exp())
        };
        if !lai.is_finite() || lai < 0.0 {
            return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
                field: "lai",
                slot_index,
                crop_slot_index,
                value: lai,
                allowed: "finite and >=0.0 after initial cancov assimilation",
            });
        }
    }

    let (rtd, rtmass) = if imngmt == 2 && jdplt == 0 {
        (rdmax, rtmmax)
    } else if imngmt == 1 && cancov > 0.0 {
        (rsr * canhgt, rsr * vdmt)
    } else {
        (0.0, 0.0)
    };

    let sumgdd = if lai > 0.0 && xmxlai > 0.0 && gddmax > 0.0 {
        gddmax * lai / xmxlai
    } else {
        0.0
    };

    for (symbol, value) in [
        ("sumgdd", sumgdd),
        ("vdmt", vdmt),
        ("canhgt", canhgt),
        ("lai", lai),
        ("rtmass", rtmass),
        ("rtd", rtd),
    ] {
        if !value.is_finite() || value < 0.0 {
            return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
                field: symbol,
                slot_index,
                crop_slot_index,
                value,
                allowed: "finite and >=0.0 after initial live-canopy assimilation",
            });
        }
        surface.insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
    }

    Ok(())
}

fn projection_f64_from_surface(
    surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &BoundarySymbol,
    field: &'static str,
    slot_index: usize,
    crop_slot_index: usize,
) -> Result<f64, HillslopeRuntimeInputError> {
    let Some(value) = surface.get(symbol).map(|value| (*value).as_f64()) else {
        return Err(HillslopeRuntimeInputError::NonFinitePlProjectionField {
            field,
            slot_index,
            crop_slot_index,
            value: f64::NAN,
        });
    };
    if !value.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFinitePlProjectionField {
            field,
            slot_index,
            crop_slot_index,
            value,
        });
    }
    Ok(value)
}

fn projection_usize_from_surface(
    surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &BoundarySymbol,
    field: &'static str,
    slot_index: usize,
    crop_slot_index: usize,
) -> Result<usize, HillslopeRuntimeInputError> {
    let value = projection_f64_from_surface(surface, symbol, field, slot_index, crop_slot_index)?;
    if value < 0.0 || (value.fract()).abs() > f64::EPSILON {
        return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
            field,
            slot_index,
            crop_slot_index,
            value,
            allowed: "non-negative integer scalar",
        });
    }
    format!("{value:.0}").parse::<usize>().map_err(|_| {
        HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
            field,
            slot_index,
            crop_slot_index,
            value,
            allowed: "non-negative integer scalar representable as usize",
        }
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
