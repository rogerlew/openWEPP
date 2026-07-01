
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

type ManagementInitialScenario = openwepp_input_contract::parsers::management::InitialScenario;
type ManagementInitialCroplandData =
    openwepp_input_contract::parsers::management::InitialCroplandData;
type ManagementScheduleSlotData =
    openwepp_input_contract::parsers::management::ManagementScheduleSlot;
type ManagementYearlyAnnualFallowData =
    openwepp_input_contract::parsers::management::YearlyAnnualFallowData;
type ManagementYearlyCroplandData =
    openwepp_input_contract::parsers::management::YearlyCroplandData;
type ManagementYearlyPerennialData =
    openwepp_input_contract::parsers::management::YearlyPerennialData;
type ManagementPlantCroplandData = openwepp_input_contract::parsers::management::PlantCroplandData;

#[derive(Debug, Default)]
struct PlRuntimeSurfaceBuilder {
    schedule: BTreeMap<BoundarySymbol, BoundaryValue>,
    growth: BTreeMap<BoundarySymbol, BoundaryValue>,
    decomp: BTreeMap<BoundarySymbol, BoundaryValue>,
}

impl PlRuntimeSurfaceBuilder {
    fn into_runtime_surfaces(self) -> HillslopePlRuntimeSurfaces {
        HillslopePlRuntimeSurfaces {
            pl_schedule_surface: self.schedule,
            pl_growth_surface: self.growth,
            pl_decomp_surface: self.decomp,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct InitialSeedProjection {
    cancov: f64,
    inrcov: f64,
    rilcov: f64,
    rrinit: f64,
    rspace: f64,
    tillay1: f64,
    tillay2: f64,
    width: f64,
    residue_depth_m: f64,
    canopy_cover_coeff: f64,
    canopy_height_curve: f64,
    flivmx: f64,
    hmax: f64,
    sumrtm: f64,
    sumsrm: f64,
}

/// Build strict typed PL runtime projection surfaces from parsed management
/// input (`PL-MAN-SEAM-001`).
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` for unsupported branches, dangling
/// references, non-finite required controls, or schedule closure violations.
pub fn build_hillslope_pl_runtime_surfaces_from_management(
    management: &ManagementParseOutput,
) -> Result<HillslopePlRuntimeSurfaces, HillslopeRuntimeInputError> {
    validate_management_schedule_shape(management)?;

    let mut surfaces = PlRuntimeSurfaceBuilder::default();
    seed_schedule_metadata(&mut surfaces.schedule, management)?;
    seed_growth_defaults(&mut surfaces.growth);
    project_initial_seed_surfaces(&mut surfaces, management)?;
    project_schedule_slot_surfaces(&mut surfaces, management)?;
    apply_primary_initial_live_canopy_assimilation(&mut surfaces.growth)?;

    Ok(surfaces.into_runtime_surfaces())
}

fn validate_management_schedule_shape(
    management: &ManagementParseOutput,
) -> Result<(), HillslopeRuntimeInputError> {
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

    Ok(())
}

fn seed_schedule_metadata(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    management: &ManagementParseOutput,
) -> Result<(), HillslopeRuntimeInputError> {
    surface.insert(
        BoundarySymbol::from("pl_schedule_nofe"),
        BoundaryValue::scalar(usize_to_f64("pl_schedule_nofe", management.topology_count)?),
    );
    surface.insert(
        BoundarySymbol::from("pl_schedule_rotation_repeats"),
        BoundaryValue::scalar(usize_to_f64(
            "pl_schedule_rotation_repeats",
            management.schedule.rotation_repeats,
        )?),
    );
    surface.insert(
        BoundarySymbol::from("pl_schedule_rotation_years"),
        BoundaryValue::scalar(usize_to_f64(
            "pl_schedule_rotation_years",
            management.schedule.rotation_years,
        )?),
    );
    surface.insert(
        BoundarySymbol::from("pl_schedule_slot_count"),
        BoundaryValue::scalar(usize_to_f64(
            "pl_schedule_slot_count",
            management.schedule.slots.len(),
        )?),
    );
    surface.insert(
        BoundarySymbol::from("pl_order_decomp_before_soil"),
        BoundaryValue::scalar(1.0),
    );
    surface.insert(
        BoundarySymbol::from("pl_order_growth_after_decomp"),
        BoundaryValue::scalar(1.0),
    );
    surface.insert(
        BoundarySymbol::from("pl_order_watbal_after_growth"),
        BoundaryValue::scalar(1.0),
    );
    Ok(())
}

fn seed_growth_defaults(surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>) {
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
        surface.insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
    }
}

fn project_initial_seed_surfaces(
    surfaces: &mut PlRuntimeSurfaceBuilder,
    management: &ManagementParseOutput,
) -> Result<(), HillslopeRuntimeInputError> {
    for (ofe_position, initial_ref) in management.schedule.ofe_initial_refs.iter().enumerate() {
        let ofe_index = ofe_position + 1;
        let initial = initial_scenario_for_ofe(management, ofe_index, *initial_ref)?;
        if initial.meta.landuse != 1 {
            return Err(HillslopeRuntimeInputError::UnsupportedPlLanduse {
                section: "initial",
                value: initial.meta.landuse,
            });
        }
        let InitialScenarioData::Cropland(initial_data) = &initial.data;
        let seed = build_initial_seed_projection(management, initial.meta.landuse, initial_data)?;

        insert_initial_seed_symbols(
            surfaces,
            ofe_index,
            *initial_ref,
            initial.meta.landuse,
            initial_data,
            &seed,
        )?;

        if let Some(understory) = initial_data.understory_line {
            project_initial_understory_symbols(
                &mut surfaces.decomp,
                ofe_index,
                understory,
            )?;
        }

        if ofe_index == 1 {
            insert_primary_initial_seed_aliases(
                surfaces,
                initial.meta.landuse,
                initial_data,
                &seed,
            )?;
        }
    }

    Ok(())
}

fn initial_scenario_for_ofe(
    management: &ManagementParseOutput,
    ofe_index: usize,
    initial_ref: usize,
) -> Result<&ManagementInitialScenario, HillslopeRuntimeInputError> {
    if initial_ref == 0 || initial_ref > management.registries.initials.len() {
        return Err(
            HillslopeRuntimeInputError::ManagementInitialReferenceOutOfRange {
                ofe_index,
                initial_ref,
                max_initial_ref: management.registries.initials.len(),
            },
        );
    }

    Ok(&management.registries.initials[initial_ref - 1])
}

fn build_initial_seed_projection(
    management: &ManagementParseOutput,
    landuse: usize,
    initial_data: &ManagementInitialCroplandData,
) -> Result<InitialSeedProjection, HillslopeRuntimeInputError> {
    let cancov = validate_projection_fraction("cancov_seed", 0, 0, initial_data.base_line[1])?;
    let inrcov = validate_projection_fraction("inrcov_seed", 0, 0, initial_data.base_line[5])?;
    let rilcov = validate_projection_fraction("rilcov_seed", 0, 0, initial_data.residue_line[2])?;
    let rrinit =
        validate_projection_non_negative("rrinit_seed", 0, 0, initial_data.residue_line[3])?;
    let rspace =
        validate_projection_non_negative("rspace_seed", 0, 0, initial_data.residue_line[4])?;
    let tillay1 =
        validate_projection_non_negative("tillay1_seed", 0, 0, initial_data.thaw_line[2])?;
    let tillay2 =
        validate_projection_non_negative("tillay2_seed", 0, 0, initial_data.thaw_line[3])?;
    let width = validate_projection_non_negative("width_seed", 0, 0, initial_data.thaw_line[4])?;

    let residue_plant = residue_plant_for_initial(management, initial_data)?;
    let PlantScenarioData::Cropland(residue_plant_cropland) = &residue_plant.data;
    let residue_depth_m = legacy_initial_residue_depth_m(
        landuse,
        residue_plant_cropland.canopy_line[4],
        residue_plant_cropland.canopy_line[9],
        inrcov,
        rilcov,
        rspace,
        width,
    )?;
    let canopy_cover_coeff = validate_projection_non_negative(
        "bb_seed",
        0,
        0,
        residue_plant_cropland.canopy_line[0],
    )?;
    let canopy_height_curve = validate_projection_non_negative(
        "bbb_seed",
        0,
        0,
        residue_plant_cropland.canopy_line[1],
    )?;
    let flivmx = validate_projection_non_negative(
        "flivmx_seed",
        0,
        0,
        residue_plant_cropland.growth_line[4],
    )?;
    let hmax = validate_projection_non_negative(
        "hmax_seed",
        0,
        0,
        residue_plant_cropland.growth_line[7],
    )?;
    let sumrtm = validate_initial_terminal_seed("sumrtm_seed", initial_data.terminal_line[0])?;
    let sumsrm = validate_initial_terminal_seed("sumsrm_seed", initial_data.terminal_line[1])?;

    Ok(InitialSeedProjection {
        cancov,
        inrcov,
        rilcov,
        rrinit,
        rspace,
        tillay1,
        tillay2,
        width,
        residue_depth_m,
        canopy_cover_coeff,
        canopy_height_curve,
        flivmx,
        hmax,
        sumrtm,
        sumsrm,
    })
}

fn residue_plant_for_initial<'a>(
    management: &'a ManagementParseOutput,
    initial_data: &ManagementInitialCroplandData,
) -> Result<
    &'a openwepp_input_contract::parsers::management::PlantScenario,
    HillslopeRuntimeInputError,
>
{
    if initial_data.iresd == 0 || initial_data.iresd > management.registries.plants.len() {
        return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
            field: "iresd_seed",
            slot_index: 0,
            crop_slot_index: 0,
            value: usize_to_f64("iresd_seed", initial_data.iresd)?,
            allowed: "1..=plant_scenario_count",
        });
    }

    Ok(&management.registries.plants[initial_data.iresd - 1])
}

fn validate_initial_terminal_seed(
    field: &'static str,
    value: f64,
) -> Result<f64, HillslopeRuntimeInputError> {
    if !value.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFinitePlProjectionField {
            field,
            slot_index: 0,
            crop_slot_index: 0,
            value,
        });
    }
    Ok(value)
}

fn insert_initial_seed_symbols(
    surfaces: &mut PlRuntimeSurfaceBuilder,
    ofe_index: usize,
    initial_ref: usize,
    landuse: usize,
    initial_data: &ManagementInitialCroplandData,
    seed: &InitialSeedProjection,
) -> Result<(), HillslopeRuntimeInputError> {
    surfaces.schedule.insert(
        pl_schedule_ofe_symbol("initial_ref", ofe_index),
        BoundaryValue::scalar(usize_to_f64("initial_ref", initial_ref)?),
    );
    surfaces.schedule.insert(
        pl_schedule_ofe_symbol("lanuse", ofe_index),
        BoundaryValue::scalar(usize_to_f64("lanuse", landuse)?),
    );

    insert_initial_growth_seed_symbols(
        &mut surfaces.growth,
        ofe_index,
        initial_data,
        seed,
    )?;
    insert_initial_decomp_seed_symbols(
        &mut surfaces.decomp,
        ofe_index,
        initial_data,
        seed,
    )
}

fn insert_initial_growth_seed_symbols(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    ofe_index: usize,
    initial_data: &ManagementInitialCroplandData,
    seed: &InitialSeedProjection,
) -> Result<(), HillslopeRuntimeInputError> {
    for (symbol, value) in [
        (
            pl_growth_ofe_symbol("imngmt_seed", ofe_index),
            usize_to_f64("imngmt_seed", initial_data.imngmt)?,
        ),
        (
            pl_growth_ofe_symbol("rtyp_seed", ofe_index),
            usize_to_f64("rtyp_seed", initial_data.rtyp)?,
        ),
        (pl_growth_ofe_symbol("cancov_seed", ofe_index), seed.cancov),
        (
            pl_growth_ofe_symbol("bb_seed", ofe_index),
            seed.canopy_cover_coeff,
        ),
        (
            pl_growth_ofe_symbol("bbb_seed", ofe_index),
            seed.canopy_height_curve,
        ),
        (pl_growth_ofe_symbol("flivmx_seed", ofe_index), seed.flivmx),
        (pl_growth_ofe_symbol("hmax_seed", ofe_index), seed.hmax),
        (slope_ofe_symbol("inrcov", ofe_index), seed.inrcov),
        (slope_ofe_symbol("rilcov", ofe_index), seed.rilcov),
        (slope_ofe_symbol("rrinit", ofe_index), seed.rrinit),
        (slope_ofe_symbol("rspace", ofe_index), seed.rspace),
        (pl_growth_ofe_symbol("tillay1_m", ofe_index), seed.tillay1),
        (pl_growth_ofe_symbol("tillay2_m", ofe_index), seed.tillay2),
        (slope_ofe_symbol("width", ofe_index), seed.width),
        (
            slope_ofe_symbol("rtyp", ofe_index),
            usize_to_f64("rtyp_seed", initial_data.rtyp)?,
        ),
    ] {
        surface.insert(symbol, BoundaryValue::scalar(value));
    }
    Ok(())
}

fn insert_initial_decomp_seed_symbols(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    ofe_index: usize,
    initial_data: &ManagementInitialCroplandData,
    seed: &InitialSeedProjection,
) -> Result<(), HillslopeRuntimeInputError> {
    for (symbol, value) in [
        (
            pl_decomp_ofe_symbol("iresd_seed", ofe_index),
            usize_to_f64("iresd_seed", initial_data.iresd)?,
        ),
        (pl_decomp_ofe_symbol("sumrtm_seed", ofe_index), seed.sumrtm),
        (pl_decomp_ofe_symbol("sumsrm_seed", ofe_index), seed.sumsrm),
        (
            pl_decomp_ofe_symbol("residue_depth_m_seed", ofe_index),
            seed.residue_depth_m,
        ),
    ] {
        surface.insert(symbol, BoundaryValue::scalar(value));
    }
    Ok(())
}

fn project_initial_understory_symbols(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    ofe_index: usize,
    understory: [f64; 2],
) -> Result<(), HillslopeRuntimeInputError> {
    let usinrcol = validate_initial_terminal_seed("usinrcol_seed", understory[0])?;
    let usrilcol = validate_initial_terminal_seed("usrilcol_seed", understory[1])?;
    surface.insert(
        pl_decomp_ofe_symbol("usinrcol_seed", ofe_index),
        BoundaryValue::scalar(usinrcol),
    );
    surface.insert(
        pl_decomp_ofe_symbol("usrilcol_seed", ofe_index),
        BoundaryValue::scalar(usrilcol),
    );
    Ok(())
}

fn insert_primary_initial_seed_aliases(
    surfaces: &mut PlRuntimeSurfaceBuilder,
    landuse: usize,
    initial_data: &ManagementInitialCroplandData,
    seed: &InitialSeedProjection,
) -> Result<(), HillslopeRuntimeInputError> {
    surfaces.schedule.insert(
        BoundarySymbol::from("lanuse"),
        BoundaryValue::scalar(usize_to_f64("lanuse", landuse)?),
    );
    insert_primary_initial_growth_aliases(&mut surfaces.growth, initial_data, seed)?;
    insert_primary_initial_decomp_aliases(&mut surfaces.decomp, initial_data, seed)
}

fn insert_primary_initial_growth_aliases(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    initial_data: &ManagementInitialCroplandData,
    seed: &InitialSeedProjection,
) -> Result<(), HillslopeRuntimeInputError> {
    for (symbol, value) in [
        (
            "imngmt_seed",
            usize_to_f64("imngmt_seed", initial_data.imngmt)?,
        ),
        ("cancov", seed.cancov),
        ("inrcov", seed.inrcov),
        ("rilcov", seed.rilcov),
        ("rrinit", seed.rrinit),
        ("rspace", seed.rspace),
        ("management.initial.params.tillay1_m", seed.tillay1),
        ("management.initial.params.tillay2_m", seed.tillay2),
        ("width", seed.width),
        ("rtyp", usize_to_f64("rtyp_seed", initial_data.rtyp)?),
        ("bb_seed", seed.canopy_cover_coeff),
        ("bbb_seed", seed.canopy_height_curve),
        ("flivmx_seed", seed.flivmx),
        ("hmax_seed", seed.hmax),
    ] {
        surface.insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
    }
    Ok(())
}

fn insert_primary_initial_decomp_aliases(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    initial_data: &ManagementInitialCroplandData,
    seed: &InitialSeedProjection,
) -> Result<(), HillslopeRuntimeInputError> {
    for (symbol, value) in [
        ("iresd_seed", usize_to_f64("iresd_seed", initial_data.iresd)?),
        ("sumrtm_seed", seed.sumrtm),
        ("sumsrm_seed", seed.sumsrm),
        ("frost.runtime_residue_depth_m", seed.residue_depth_m),
        ("resdep", seed.residue_depth_m),
    ] {
        surface.insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
    }
    Ok(())
}

fn project_schedule_slot_surfaces(
    surfaces: &mut PlRuntimeSurfaceBuilder,
    management: &ManagementParseOutput,
) -> Result<(), HillslopeRuntimeInputError> {
    for (slot_position, slot) in management.schedule.slots.iter().enumerate() {
        project_one_schedule_slot_surfaces(surfaces, management, slot_position + 1, slot)?;
    }

    Ok(())
}

fn project_one_schedule_slot_surfaces(
    surfaces: &mut PlRuntimeSurfaceBuilder,
    management: &ManagementParseOutput,
    slot_index: usize,
    slot: &ManagementScheduleSlotData,
) -> Result<(), HillslopeRuntimeInputError> {
    validate_schedule_slot_shape(management, slot_index, slot)?;
    let ofe_index = slot.ofe_index + 1;
    insert_schedule_slot_symbols(&mut surfaces.schedule, slot_index, slot, ofe_index)?;

    for (crop_slot_position, yearly_ref) in slot.yearly_refs.iter().enumerate() {
        project_yearly_crop_slot_surfaces(
            surfaces,
            management,
            slot_index,
            crop_slot_position + 1,
            *yearly_ref,
        )?;
    }

    Ok(())
}

fn validate_schedule_slot_shape(
    management: &ManagementParseOutput,
    slot_index: usize,
    slot: &ManagementScheduleSlotData,
) -> Result<(), HillslopeRuntimeInputError> {
    if slot.crop_slots != slot.yearly_refs.len() {
        return Err(HillslopeRuntimeInputError::ManagementScheduleSlotArityMismatch {
            slot_index,
            crop_slots: slot.crop_slots,
            yearly_refs: slot.yearly_refs.len(),
        });
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

    Ok(())
}

fn insert_schedule_slot_symbols(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    slot: &ManagementScheduleSlotData,
    ofe_index: usize,
) -> Result<(), HillslopeRuntimeInputError> {
    for (symbol, value) in [
        (
            pl_schedule_slot_symbol("rotation_index", slot_index),
            usize_to_f64("rotation_index", slot.rotation_index + 1)?,
        ),
        (
            pl_schedule_slot_symbol("year_in_rotation", slot_index),
            usize_to_f64("year_in_rotation", slot.year_in_rotation + 1)?,
        ),
        (
            pl_schedule_slot_symbol("ofe_index", slot_index),
            usize_to_f64("ofe_index", ofe_index)?,
        ),
        (
            pl_schedule_slot_symbol("crop_slots", slot_index),
            usize_to_f64("crop_slots", slot.crop_slots)?,
        ),
    ] {
        surface.insert(symbol, BoundaryValue::scalar(value));
    }
    Ok(())
}

fn project_yearly_crop_slot_surfaces(
    surfaces: &mut PlRuntimeSurfaceBuilder,
    management: &ManagementParseOutput,
    slot_index: usize,
    crop_slot_index: usize,
    yearly_ref: usize,
) -> Result<(), HillslopeRuntimeInputError> {
    let yearly = yearly_scenario_for_slot(management, slot_index, crop_slot_index, yearly_ref)?;
    if yearly.meta.landuse != 1 {
        return Err(HillslopeRuntimeInputError::UnsupportedPlLanduse {
            section: "yearly",
            value: yearly.meta.landuse,
        });
    }
    let YearlyScenarioData::Cropland(cropland) = &yearly.data;

    insert_yearly_schedule_symbols(
        &mut surfaces.schedule,
        slot_index,
        crop_slot_index,
        yearly_ref,
        yearly.meta.landuse,
        cropland,
    )?;
    insert_yearly_growth_identity_symbols(
        &mut surfaces.growth,
        slot_index,
        crop_slot_index,
        cropland,
    )?;

    let plant_cropland = plant_cropland_for_yearly(management, slot_index, crop_slot_index, cropland)?;
    project_growth_equation_symbols(
        &mut surfaces.growth,
        slot_index,
        crop_slot_index,
        plant_cropland,
    )?;
    project_decomposition_equation_symbols(
        &mut surfaces.decomp,
        slot_index,
        crop_slot_index,
        plant_cropland,
    )?;
    if slot_index == 1 && crop_slot_index == 1 {
        project_primary_crop_aliases_and_drain_controls(
            surfaces,
            management,
            slot_index,
            crop_slot_index,
            cropland,
            plant_cropland,
        )?;
    }

    match &cropland.branch {
        YearlyCroplandBranch::AnnualOrFallow(annual) => {
            project_annual_or_fallow_crop_slot(
                surfaces,
                slot_index,
                crop_slot_index,
                cropland,
                annual,
            )?;
        }
        YearlyCroplandBranch::Perennial(perennial) => {
            project_perennial_crop_slot(surfaces, slot_index, crop_slot_index, perennial)?;
        }
    }

    Ok(())
}

fn yearly_scenario_for_slot(
    management: &ManagementParseOutput,
    slot_index: usize,
    crop_slot_index: usize,
    yearly_ref: usize,
) -> Result<&openwepp_input_contract::parsers::management::YearlyScenario, HillslopeRuntimeInputError>
{
    if yearly_ref == 0 || yearly_ref > management.registries.yearlies.len() {
        return Err(
            HillslopeRuntimeInputError::ManagementYearlyReferenceOutOfRange {
                slot_index,
                crop_slot_index,
                yearly_ref,
                max_yearly_ref: management.registries.yearlies.len(),
            },
        );
    }

    Ok(&management.registries.yearlies[yearly_ref - 1])
}

fn insert_yearly_schedule_symbols(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    yearly_ref: usize,
    landuse: usize,
    cropland: &ManagementYearlyCroplandData,
) -> Result<(), HillslopeRuntimeInputError> {
    for (symbol, value) in [
        (
            pl_schedule_slot_crop_symbol("yearly_ref", slot_index, crop_slot_index),
            usize_to_f64("yearly_ref", yearly_ref)?,
        ),
        (
            pl_schedule_slot_crop_symbol("lanuse", slot_index, crop_slot_index),
            usize_to_f64("lanuse", landuse)?,
        ),
        (
            pl_schedule_slot_crop_symbol("itype", slot_index, crop_slot_index),
            usize_to_f64("itype", cropland.itype)?,
        ),
        (
            pl_schedule_slot_crop_symbol("tilseq", slot_index, crop_slot_index),
            usize_to_f64("tilseq", cropland.tilseq)?,
        ),
        (
            pl_schedule_slot_crop_symbol("conset", slot_index, crop_slot_index),
            usize_to_f64("conset", cropland.conset)?,
        ),
        (
            pl_schedule_slot_crop_symbol("drset", slot_index, crop_slot_index),
            usize_to_f64("drset", cropland.drset)?,
        ),
        (
            pl_schedule_slot_crop_symbol("imngmt", slot_index, crop_slot_index),
            usize_to_f64("imngmt", cropland.imngmt)?,
        ),
    ] {
        surface.insert(symbol, BoundaryValue::scalar(value));
    }
    Ok(())
}

fn insert_yearly_growth_identity_symbols(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    cropland: &ManagementYearlyCroplandData,
) -> Result<(), HillslopeRuntimeInputError> {
    for (symbol, value) in [
        (
            pl_growth_slot_crop_symbol("itype", slot_index, crop_slot_index),
            usize_to_f64("itype", cropland.itype)?,
        ),
        (
            pl_growth_slot_crop_symbol("imngmt", slot_index, crop_slot_index),
            usize_to_f64("imngmt", cropland.imngmt)?,
        ),
    ] {
        surface.insert(symbol, BoundaryValue::scalar(value));
    }
    Ok(())
}

fn plant_cropland_for_yearly<'a>(
    management: &'a ManagementParseOutput,
    slot_index: usize,
    crop_slot_index: usize,
    cropland: &ManagementYearlyCroplandData,
) -> Result<&'a ManagementPlantCroplandData, HillslopeRuntimeInputError> {
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
    Ok(plant_cropland)
}

fn project_primary_crop_aliases_and_drain_controls(
    surfaces: &mut PlRuntimeSurfaceBuilder,
    management: &ManagementParseOutput,
    slot_index: usize,
    crop_slot_index: usize,
    cropland: &ManagementYearlyCroplandData,
    plant_cropland: &ManagementPlantCroplandData,
) -> Result<(), HillslopeRuntimeInputError> {
    project_primary_growth_equation_aliases(&mut surfaces.growth, plant_cropland)?;
    project_primary_decomposition_equation_aliases(&mut surfaces.decomp, plant_cropland)?;
    project_primary_drain_controls(
        &mut surfaces.schedule,
        management,
        slot_index,
        crop_slot_index,
        cropland.drset,
    )
}

fn project_primary_drain_controls(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    management: &ManagementParseOutput,
    slot_index: usize,
    crop_slot_index: usize,
    drset: usize,
) -> Result<(), HillslopeRuntimeInputError> {
    surface.insert(
        BoundarySymbol::from("drset"),
        BoundaryValue::scalar(usize_to_f64("drset", drset)?),
    );
    if drset == 0 {
        surface.insert(
            BoundarySymbol::from("wb19_drain_enabled"),
            BoundaryValue::scalar(0.0),
        );
        return Ok(());
    }

    let max_drain_ref = management.registries.drains.len();
    if drset > max_drain_ref {
        return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
            field: "drset",
            slot_index,
            crop_slot_index,
            value: usize_to_f64("drset", drset)?,
            allowed: "1..=drain_scenario_count",
        });
    }
    let drain = &management.registries.drains[drset - 1];
    let drain_depth =
        validate_enabled_drain_geometry("wb19_drain_depth", slot_index, crop_slot_index, drain.ddrain)?;
    let drain_spacing = validate_enabled_drain_geometry(
        "wb19_drain_spacing",
        slot_index,
        crop_slot_index,
        drain.sdrain,
    )?;
    let drain_diameter = validate_enabled_drain_geometry(
        "wb19_drain_diameter",
        slot_index,
        crop_slot_index,
        drain.drdiam,
    )?;
    for (symbol, value) in [
        ("wb19_drain_enabled", 1.0),
        ("wb19_drain_depth", drain_depth),
        ("wb19_drain_spacing", drain_spacing),
        ("wb19_drain_diameter", drain_diameter),
    ] {
        surface.insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
    }
    Ok(())
}

fn validate_enabled_drain_geometry(
    field: &'static str,
    slot_index: usize,
    crop_slot_index: usize,
    value: f64,
) -> Result<f64, HillslopeRuntimeInputError> {
    let value = validate_projection_non_negative(field, slot_index, crop_slot_index, value)?;
    if value <= 0.0 {
        return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
            field,
            slot_index,
            crop_slot_index,
            value,
            allowed: "> 0.0 when wb19_drain_enabled=1",
        });
    }
    Ok(value)
}

fn project_annual_or_fallow_crop_slot(
    surfaces: &mut PlRuntimeSurfaceBuilder,
    slot_index: usize,
    crop_slot_index: usize,
    cropland: &ManagementYearlyCroplandData,
    annual: &ManagementYearlyAnnualFallowData,
) -> Result<(), HillslopeRuntimeInputError> {
    let rw = validate_projection_finite("rw", slot_index, crop_slot_index, annual.rw)?;
    let jdharv = validate_projection_day("jdharv", slot_index, crop_slot_index, annual.jdharv, false)?;
    let jdplt = validate_projection_day("jdplt", slot_index, crop_slot_index, annual.jdplt, false)?;
    for (symbol, value) in [
        (
            pl_growth_slot_crop_symbol("jdharv", slot_index, crop_slot_index),
            usize_to_f64("jdharv", jdharv)?,
        ),
        (
            pl_growth_slot_crop_symbol("jdplt", slot_index, crop_slot_index),
            usize_to_f64("jdplt", jdplt)?,
        ),
        (pl_growth_slot_crop_symbol("rw", slot_index, crop_slot_index), rw),
    ] {
        surfaces
            .growth
            .insert(symbol, BoundaryValue::scalar(value));
    }

    surfaces.decomp.insert(
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
        &mut surfaces.decomp,
        slot_index,
        crop_slot_index,
        &annual_extension_projection,
    )?;

    if slot_index == 1 && crop_slot_index == 1 {
        project_primary_annual_crop_aliases(
            surfaces,
            cropland,
            annual,
            jdharv,
            jdplt,
            &annual_extension_projection,
        )?;
    }
    Ok(())
}

fn project_primary_annual_crop_aliases(
    surfaces: &mut PlRuntimeSurfaceBuilder,
    cropland: &ManagementYearlyCroplandData,
    annual: &ManagementYearlyAnnualFallowData,
    jdharv: usize,
    jdplt: usize,
    annual_extension_projection: &AnnualExtensionProjection,
) -> Result<(), HillslopeRuntimeInputError> {
    for (symbol, value) in [
        ("itype", usize_to_f64("itype", cropland.itype)?),
        ("imngmt", usize_to_f64("imngmt", cropland.imngmt)?),
        ("jdharv", usize_to_f64("jdharv", jdharv)?),
        ("jdplt", usize_to_f64("jdplt", jdplt)?),
        ("rw", annual.rw),
    ] {
        surfaces
            .growth
            .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
    }
    surfaces.decomp.insert(
        BoundarySymbol::from("resmgt"),
        BoundaryValue::scalar(usize_to_f64("resmgt", annual.resmgt)?),
    );
    project_primary_annual_extension_aliases(
        &mut surfaces.decomp,
        annual_extension_projection,
    )
}

fn project_perennial_crop_slot(
    surfaces: &mut PlRuntimeSurfaceBuilder,
    slot_index: usize,
    crop_slot_index: usize,
    perennial: &ManagementYearlyPerennialData,
) -> Result<(), HillslopeRuntimeInputError> {
    if perennial.mgtopt > 3 {
        return Err(HillslopeRuntimeInputError::UnsupportedPlManagementOption {
            field: "mgtopt",
            value: perennial.mgtopt,
            allowed: "1..3",
        });
    }
    let rw = validate_projection_finite("rw", slot_index, crop_slot_index, perennial.rw)?;
    let jdharv = validate_projection_day("jdharv", slot_index, crop_slot_index, perennial.jdharv, true)?;
    let jdplt = validate_projection_day("jdplt", slot_index, crop_slot_index, perennial.jdplt, true)?;
    let jdstop =
        validate_projection_day("jdstop", slot_index, crop_slot_index, perennial.jdstop, true)?;
    for (symbol, value) in [
        (
            pl_growth_slot_crop_symbol("jdharv", slot_index, crop_slot_index),
            usize_to_f64("jdharv", jdharv)?,
        ),
        (
            pl_growth_slot_crop_symbol("jdplt", slot_index, crop_slot_index),
            usize_to_f64("jdplt", jdplt)?,
        ),
        (
            pl_growth_slot_crop_symbol("jdstop", slot_index, crop_slot_index),
            usize_to_f64("jdstop", jdstop)?,
        ),
        (pl_growth_slot_crop_symbol("rw", slot_index, crop_slot_index), rw),
        (
            pl_growth_slot_crop_symbol("mgtopt", slot_index, crop_slot_index),
            usize_to_f64("mgtopt", perennial.mgtopt)?,
        ),
    ] {
        surfaces
            .growth
            .insert(symbol, BoundaryValue::scalar(value));
    }

    surfaces.decomp.insert(
        pl_decomp_slot_crop_symbol("mgtopt", slot_index, crop_slot_index),
        BoundaryValue::scalar(usize_to_f64("mgtopt", perennial.mgtopt)?),
    );
    let (ncut, ncycle) = project_perennial_management_payload(
        &mut surfaces.decomp,
        slot_index,
        crop_slot_index,
        perennial,
    )?;
    surfaces.decomp.insert(
        pl_decomp_slot_crop_symbol("ncut", slot_index, crop_slot_index),
        BoundaryValue::scalar(usize_to_f64("ncut", ncut)?),
    );
    surfaces.decomp.insert(
        pl_decomp_slot_crop_symbol("ncycle", slot_index, crop_slot_index),
        BoundaryValue::scalar(usize_to_f64("ncycle", ncycle)?),
    );
    Ok(())
}

fn project_perennial_management_payload(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    perennial: &ManagementYearlyPerennialData,
) -> Result<(usize, usize), HillslopeRuntimeInputError> {
    match perennial.mgtopt {
        1 => project_perennial_cutday_payload(surface, slot_index, crop_slot_index, perennial),
        2 => project_perennial_grazing_payload(surface, slot_index, crop_slot_index, perennial),
        3 => validate_perennial_idle_payload(slot_index, crop_slot_index, perennial),
        _ => Err(HillslopeRuntimeInputError::UnsupportedPlManagementOption {
            field: "mgtopt",
            value: perennial.mgtopt,
            allowed: "1..3",
        }),
    }
}

fn project_perennial_cutday_payload(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    perennial: &ManagementYearlyPerennialData,
) -> Result<(usize, usize), HillslopeRuntimeInputError> {
    if perennial.cut_days.is_empty() {
        return Err(HillslopeRuntimeInputError::PlProjectionCardinalityInvalid {
            field: "ncut",
            slot_index,
            crop_slot_index,
            value: 0,
            expected: ">=1 for mgtopt=1",
        });
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

    project_perennial_cutday_symbols(surface, slot_index, crop_slot_index, perennial)?;
    Ok((perennial.cut_days.len(), 0))
}

fn project_perennial_grazing_payload(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    perennial: &ManagementYearlyPerennialData,
) -> Result<(usize, usize), HillslopeRuntimeInputError> {
    if perennial.grazing_cycles.is_empty() {
        return Err(HillslopeRuntimeInputError::PlProjectionCardinalityInvalid {
            field: "ncycle",
            slot_index,
            crop_slot_index,
            value: 0,
            expected: ">=1 for mgtopt=2",
        });
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

    project_perennial_grazing_cycle_symbols(surface, slot_index, crop_slot_index, perennial)?;
    Ok((0, perennial.grazing_cycles.len()))
}

fn validate_perennial_idle_payload(
    slot_index: usize,
    crop_slot_index: usize,
    perennial: &ManagementYearlyPerennialData,
) -> Result<(usize, usize), HillslopeRuntimeInputError> {
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
    Ok((0, 0))
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

#[derive(Debug, Clone, PartialEq)]
struct PrimaryInitialLiveCanopyInputs {
    imngmt: usize,
    jdharv: usize,
    jdplt: usize,
    cancov: f64,
    bb: f64,
    bbb: f64,
    hmax: f64,
    xmxlai: f64,
    gddmax: f64,
    rsr: f64,
    rdmax: f64,
    rtmmax: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PrimaryInitialCanopyGeometry {
    vdmt: f64,
    canhgt: f64,
    lai: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PrimaryInitialLiveCanopyState {
    cancov: f64,
    sumgdd: f64,
    vdmt: f64,
    canhgt: f64,
    lai: f64,
    rtmass: f64,
    rtd: f64,
}

fn apply_primary_initial_live_canopy_assimilation(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<(), HillslopeRuntimeInputError> {
    let inputs = read_primary_initial_live_canopy_inputs(surface)?;
    let state = compute_primary_initial_live_canopy_state(&inputs)?;
    write_primary_initial_live_canopy_state(surface, state);
    Ok(())
}

fn read_primary_initial_live_canopy_inputs(
    surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<PrimaryInitialLiveCanopyInputs, HillslopeRuntimeInputError> {
    Ok(PrimaryInitialLiveCanopyInputs {
        imngmt: read_primary_growth_usize(surface, "imngmt")?,
        jdharv: read_primary_growth_usize(surface, "jdharv")?,
        jdplt: read_primary_growth_usize(surface, "jdplt")?,
        cancov: read_primary_state_scalar(surface, "cancov")?,
        bb: read_primary_growth_f64(surface, "bb")?,
        bbb: read_primary_growth_f64(surface, "bbb")?,
        hmax: read_primary_growth_f64(surface, "hmax")?,
        xmxlai: read_primary_growth_f64(surface, "xmxlai")?,
        gddmax: read_primary_growth_f64(surface, "gddmax")?,
        rsr: read_primary_growth_f64(surface, "rsr")?,
        rdmax: read_primary_growth_f64(surface, "rdmax")?,
        rtmmax: read_primary_growth_f64(surface, "rtmmax")?,
    })
}

fn read_primary_growth_usize(
    surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    field: &'static str,
) -> Result<usize, HillslopeRuntimeInputError> {
    let slot_index = 1;
    let crop_slot_index = 1;
    projection_usize_from_surface(
        surface,
        &pl_growth_slot_crop_symbol(field, slot_index, crop_slot_index),
        field,
        slot_index,
        crop_slot_index,
    )
}

fn read_primary_growth_f64(
    surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    field: &'static str,
) -> Result<f64, HillslopeRuntimeInputError> {
    let slot_index = 1;
    let crop_slot_index = 1;
    projection_f64_from_surface(
        surface,
        &pl_growth_slot_crop_symbol(field, slot_index, crop_slot_index),
        field,
        slot_index,
        crop_slot_index,
    )
}

fn read_primary_state_scalar(
    surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    field: &'static str,
) -> Result<f64, HillslopeRuntimeInputError> {
    let slot_index = 1;
    let crop_slot_index = 1;
    projection_f64_from_surface(
        surface,
        &BoundarySymbol::from(field),
        field,
        slot_index,
        crop_slot_index,
    )
}

fn compute_primary_initial_live_canopy_state(
    inputs: &PrimaryInitialLiveCanopyInputs,
) -> Result<PrimaryInitialLiveCanopyState, HillslopeRuntimeInputError> {
    let cancov = normalized_primary_initial_cancov(inputs);
    let geometry = compute_primary_initial_canopy_geometry(inputs, cancov)?;
    let (rtd, rtmass) = compute_primary_initial_root_state(inputs, cancov, geometry);
    let sumgdd = compute_primary_initial_sumgdd(inputs, geometry.lai);
    let state = PrimaryInitialLiveCanopyState {
        cancov,
        sumgdd,
        vdmt: geometry.vdmt,
        canhgt: geometry.canhgt,
        lai: geometry.lai,
        rtmass,
        rtd,
    };
    validate_primary_initial_live_canopy_state(state)?;
    Ok(state)
}

fn normalized_primary_initial_cancov(inputs: &PrimaryInitialLiveCanopyInputs) -> f64 {
    let mut cancov = inputs.cancov;
    if inputs.imngmt == 3
        || (inputs.imngmt == 1 && inputs.jdplt < inputs.jdharv)
        || (inputs.imngmt == 2 && inputs.jdplt > 0)
    {
        cancov = 0.0;
    }
    if cancov >= PL_GROWTH_CANCOV_MAX {
        cancov = PL_GROWTH_CANCOV_MAX;
    }
    cancov
}

fn compute_primary_initial_canopy_geometry(
    inputs: &PrimaryInitialLiveCanopyInputs,
    cancov: f64,
) -> Result<PrimaryInitialCanopyGeometry, HillslopeRuntimeInputError> {
    if cancov <= 0.0 {
        return Ok(PrimaryInitialCanopyGeometry {
            vdmt: 0.0,
            canhgt: 0.0,
            lai: 0.0,
        });
    }

    validate_positive_initial_canopy_driver("bb", inputs.bb)?;
    validate_positive_initial_canopy_driver("xmxlai", inputs.xmxlai)?;
    let vdmt = ((1.0 - cancov).ln() / -inputs.bb).max(0.0);
    let canhgt = (1.0 - (-inputs.bbb * vdmt).exp()) * inputs.hmax;
    let lai = compute_primary_initial_lai(inputs, vdmt);
    validate_primary_initial_lai(lai)?;
    Ok(PrimaryInitialCanopyGeometry { vdmt, canhgt, lai })
}

fn validate_positive_initial_canopy_driver(
    field: &'static str,
    value: f64,
) -> Result<(), HillslopeRuntimeInputError> {
    let slot_index = 1;
    let crop_slot_index = 1;
    if value <= 0.0 {
        return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
            field,
            slot_index,
            crop_slot_index,
            value,
            allowed: ">0.0 when initial cancov > 0.0",
        });
    }
    Ok(())
}

fn compute_primary_initial_lai(inputs: &PrimaryInitialLiveCanopyInputs, vdmt: f64) -> f64 {
    if inputs.imngmt == 1 {
        inputs.xmxlai * vdmt
            / (vdmt + PL_GROWTH_ANNUAL_LAI_A * (-PL_GROWTH_ANNUAL_LAI_B * vdmt).exp())
    } else {
        inputs.xmxlai * vdmt
            / (vdmt
                + PL_GROWTH_PERENNIAL_LAI_A * (-PL_GROWTH_PERENNIAL_LAI_B * vdmt).exp())
    }
}

fn validate_primary_initial_lai(lai: f64) -> Result<(), HillslopeRuntimeInputError> {
    if !lai.is_finite() || lai < 0.0 {
        return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
            field: "lai",
            slot_index: 1,
            crop_slot_index: 1,
            value: lai,
            allowed: "finite and >=0.0 after initial cancov assimilation",
        });
    }
    Ok(())
}

fn compute_primary_initial_root_state(
    inputs: &PrimaryInitialLiveCanopyInputs,
    cancov: f64,
    geometry: PrimaryInitialCanopyGeometry,
) -> (f64, f64) {
    if inputs.imngmt == 2 && inputs.jdplt == 0 {
        (inputs.rdmax, inputs.rtmmax)
    } else if inputs.imngmt == 1 && cancov > 0.0 {
        (inputs.rsr * geometry.canhgt, inputs.rsr * geometry.vdmt)
    } else {
        (0.0, 0.0)
    }
}

fn compute_primary_initial_sumgdd(inputs: &PrimaryInitialLiveCanopyInputs, lai: f64) -> f64 {
    if lai > 0.0 && inputs.xmxlai > 0.0 && inputs.gddmax > 0.0 {
        inputs.gddmax * lai / inputs.xmxlai
    } else {
        0.0
    }
}

fn validate_primary_initial_live_canopy_state(
    state: PrimaryInitialLiveCanopyState,
) -> Result<(), HillslopeRuntimeInputError> {
    for (symbol, value) in [
        ("sumgdd", state.sumgdd),
        ("vdmt", state.vdmt),
        ("canhgt", state.canhgt),
        ("lai", state.lai),
        ("rtmass", state.rtmass),
        ("rtd", state.rtd),
    ] {
        if !value.is_finite() || value < 0.0 {
            return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
                field: symbol,
                slot_index: 1,
                crop_slot_index: 1,
                value,
                allowed: "finite and >=0.0 after initial live-canopy assimilation",
            });
        }
    }
    Ok(())
}

fn write_primary_initial_live_canopy_state(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    state: PrimaryInitialLiveCanopyState,
) {
    surface.insert(
        BoundarySymbol::from("cancov"),
        BoundaryValue::scalar(state.cancov),
    );
    for (symbol, value) in [
        ("sumgdd", state.sumgdd),
        ("vdmt", state.vdmt),
        ("canhgt", state.canhgt),
        ("lai", state.lai),
        ("rtmass", state.rtmass),
        ("rtd", state.rtd),
    ] {
        surface.insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
    }
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

#[cfg(test)]
mod cqr_row3_management_tests {
    use super::*;
    use openwepp_input_contract::parsers::management::{
        DrainScenario, ManagementParseOutput, ManagementScenarioRegistries, ManagementSchedule,
        ManagementSectionCounts, ManagementSectionMeta, ScenarioMeta,
    };

    fn scenario_meta(name: &str) -> ScenarioMeta {
        ScenarioMeta {
            name: name.to_string(),
            description: [String::new(), String::new(), String::new()],
            landuse: 1,
        }
    }

    fn management_meta() -> ManagementSectionMeta {
        ManagementSectionMeta {
            name: "management".to_string(),
            description: [String::new(), String::new(), String::new()],
            nofes: 1,
        }
    }

    fn drain_scenario(ddrain: f64, sdrain: f64, drdiam: f64) -> DrainScenario {
        DrainScenario {
            meta: scenario_meta("drain"),
            ddrain,
            drainc: 0.0,
            drdiam,
            sdrain,
        }
    }

    fn management_with_drains(drains: Vec<DrainScenario>) -> ManagementParseOutput {
        ManagementParseOutput {
            datver: "98.4".to_string(),
            topology_count: 1,
            declared_total_years: 1,
            section_counts: ManagementSectionCounts {
                ncrop: 0,
                nop: 0,
                nini: 0,
                nseq: 0,
                ncnt: 0,
                ndrain: drains.len(),
                nscen: 0,
            },
            registries: ManagementScenarioRegistries {
                plants: Vec::new(),
                operations: Vec::new(),
                initials: Vec::new(),
                surfaces: Vec::new(),
                contours: Vec::new(),
                drains,
                yearlies: Vec::new(),
                management_meta: management_meta(),
            },
            schedule: ManagementSchedule {
                ofe_initial_refs: Vec::new(),
                rotation_repeats: 0,
                rotation_years: 0,
                slots: Vec::new(),
            },
        }
    }

    fn scalar(surface: &BTreeMap<BoundarySymbol, BoundaryValue>, symbol: &str) -> f64 {
        surface
            .get(&BoundarySymbol::from(symbol))
            .unwrap_or_else(|| panic!("missing scalar {symbol}"))
            .as_f64()
    }

    fn assert_scalar(surface: &BTreeMap<BoundarySymbol, BoundaryValue>, symbol: &str, expected: f64) {
        let actual = scalar(surface, symbol);
        assert!(
            (actual - expected).abs() <= f64::EPSILON,
            "{symbol}: expected {expected}, observed {actual}"
        );
    }

    #[test]
    fn cqr_row3_project_primary_drain_controls_disables_absent_drain() {
        let management = management_with_drains(Vec::new());
        let mut surface = BTreeMap::new();

        project_primary_drain_controls(&mut surface, &management, 1, 1, 0).unwrap();

        assert_scalar(&surface, "drset", 0.0);
        assert_scalar(&surface, "wb19_drain_enabled", 0.0);
        assert!(!surface.contains_key(&BoundarySymbol::from("wb19_drain_depth")));
        assert!(!surface.contains_key(&BoundarySymbol::from("wb19_drain_spacing")));
        assert!(!surface.contains_key(&BoundarySymbol::from("wb19_drain_diameter")));
    }

    #[test]
    fn cqr_row3_project_primary_drain_controls_projects_enabled_geometry() {
        let management = management_with_drains(vec![drain_scenario(1.25, 2.5, 0.75)]);
        let mut surface = BTreeMap::new();

        project_primary_drain_controls(&mut surface, &management, 2, 3, 1).unwrap();

        assert_scalar(&surface, "drset", 1.0);
        assert_scalar(&surface, "wb19_drain_enabled", 1.0);
        assert_scalar(&surface, "wb19_drain_depth", 1.25);
        assert_scalar(&surface, "wb19_drain_spacing", 2.5);
        assert_scalar(&surface, "wb19_drain_diameter", 0.75);
    }

    #[test]
    fn cqr_row3_project_primary_drain_controls_rejects_dangling_reference() {
        let management = management_with_drains(vec![drain_scenario(1.25, 2.5, 0.75)]);
        let mut surface = BTreeMap::new();

        let err = project_primary_drain_controls(&mut surface, &management, 4, 5, 2)
            .expect_err("dangling drain reference must fail closed");

        assert!(matches!(
            err,
            HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
                field: "drset",
                slot_index: 4,
                crop_slot_index: 5,
                value: 2.0,
                allowed: "1..=drain_scenario_count"
            }
        ));
    }

    #[test]
    fn cqr_row3_project_primary_drain_controls_rejects_zero_geometry() {
        let management = management_with_drains(vec![drain_scenario(1.25, 0.0, 0.75)]);
        let mut surface = BTreeMap::new();

        let err = project_primary_drain_controls(&mut surface, &management, 6, 7, 1)
            .expect_err("enabled drain with zero spacing must fail closed");

        assert!(matches!(
            err,
            HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
                field: "wb19_drain_spacing",
                slot_index: 6,
                crop_slot_index: 7,
                value: 0.0,
                allowed: "> 0.0 when wb19_drain_enabled=1"
            }
        ));
    }
}
