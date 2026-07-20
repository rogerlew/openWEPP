fn validate_snow_control_finite(
    field: &'static str,
    value: f64,
) -> Result<f64, HillslopeRuntimeInputError> {
    if !value.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFiniteSnowControl { field, value });
    }
    Ok(value)
}

fn validate_frost_control_finite(
    field: &'static str,
    value: f64,
) -> Result<f64, HillslopeRuntimeInputError> {
    if !value.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFiniteFrostControl { field, value });
    }
    Ok(value)
}

fn validate_slope_profile_shape(slope: &SlopeProfile) -> Result<(), HillslopeRuntimeInputError> {
    if slope.ofes.is_empty() {
        return Err(HillslopeRuntimeInputError::MissingSlopeOfe);
    }

    if slope.ofe_count != slope.ofes.len() {
        return Err(HillslopeRuntimeInputError::SlopeOfeCountMismatch {
            declared_ofe_count: slope.ofe_count,
            observed_ofes: slope.ofes.len(),
        });
    }

    Ok(())
}

fn validate_slope_ofe_shape(
    ofe_index: usize,
    declared_nslpts: usize,
    observed_points: usize,
) -> Result<(), HillslopeRuntimeInputError> {
    if declared_nslpts != observed_points {
        return Err(HillslopeRuntimeInputError::SlopePointCountMismatch {
            ofe_index,
            declared_nslpts,
            observed_points,
        });
    }

    if observed_points < 2 {
        return Err(HillslopeRuntimeInputError::InsufficientSlopePoints {
            ofe_index,
            observed_points,
        });
    }

    Ok(())
}

fn validate_slope_points(
    ofe_index: usize,
    points: &[SlopePoint],
) -> Result<(), HillslopeRuntimeInputError> {
    for (point_position, point) in points.iter().enumerate() {
        let point_index = point_position + 1;
        if !point.xinput.is_finite() {
            return Err(HillslopeRuntimeInputError::NonFiniteXinput {
                ofe_index,
                point_index,
                value: point.xinput,
            });
        }

        if !point.slpinp.is_finite() {
            return Err(HillslopeRuntimeInputError::NonFiniteSlpinp {
                ofe_index,
                point_index,
                value: point.slpinp,
            });
        }
    }

    for (segment_position, window) in points.windows(2).enumerate() {
        let left_point_index = segment_position + 1;
        let right_point_index = segment_position + 2;
        let left_value = window[0].xinput;
        let right_value = window[1].xinput;
        if right_value < left_value {
            return Err(HillslopeRuntimeInputError::NonMonotoneXinput {
                ofe_index,
                left_point_index,
                left_value,
                right_point_index,
                right_value,
            });
        }
    }

    Ok(())
}

fn derive_avgslp(
    ofe_index: usize,
    points: &[SlopePoint],
    non_positive_floor: Option<f64>,
) -> Result<(f64, bool), HillslopeRuntimeInputError> {
    let slen = points.last().map(|point| point.xinput).ok_or(
        HillslopeRuntimeInputError::InsufficientSlopePoints {
            ofe_index,
            observed_points: 0,
        },
    )?;
    if !slen.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFiniteDerivedSlopeLength {
            ofe_index,
            value_m: slen,
        });
    }
    if slen <= 0.0 {
        return Err(HillslopeRuntimeInputError::NonPositiveDerivedSlopeLength {
            ofe_index,
            value_m: slen,
        });
    }

    let mut top_elevation = 0.0;
    for pair in points.windows(2) {
        let left = &pair[0];
        let right = &pair[1];
        top_elevation += (right.xinput - left.xinput) * (left.slpinp + right.slpinp) / 2.0;
    }

    let avgslp = top_elevation / slen;
    if !avgslp.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFiniteDerivedAverageSlope {
            ofe_index,
            value: avgslp,
        });
    }
    if avgslp <= 0.0 {
        if let Some(floor) = non_positive_floor {
            return Ok((floor, true));
        }
        return Err(HillslopeRuntimeInputError::NonPositiveDerivedAverageSlope {
            ofe_index,
            value: avgslp,
        });
    }

    Ok((avgslp, false))
}

fn usize_to_f64(field: &'static str, value: usize) -> Result<f64, HillslopeRuntimeInputError> {
    let value_u32 = u32::try_from(value)
        .map_err(|_| HillslopeRuntimeInputError::PlProjectionCountOutOfRange { field, value })?;
    Ok(f64::from(value_u32))
}

fn project_growth_equation_symbols(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    plant: &openwepp_input_contract::parsers::management::PlantCroplandData,
) -> Result<(), HillslopeRuntimeInputError> {
    for (root, value) in growth_equation_parameter_values(slot_index, crop_slot_index, plant)? {
        surface.insert(
            pl_growth_slot_crop_symbol(root, slot_index, crop_slot_index),
            BoundaryValue::scalar(value),
        );
    }
    Ok(())
}

fn project_primary_growth_equation_aliases(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    plant: &openwepp_input_contract::parsers::management::PlantCroplandData,
) -> Result<(), HillslopeRuntimeInputError> {
    for (root, value) in growth_equation_parameter_values(1, 1, plant)? {
        surface.insert(BoundarySymbol::from(root), BoundaryValue::scalar(value));
    }
    Ok(())
}

fn project_decomposition_equation_symbols(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    plant: &openwepp_input_contract::parsers::management::PlantCroplandData,
) -> Result<(), HillslopeRuntimeInputError> {
    for (root, value) in
        decomposition_equation_parameter_values(slot_index, crop_slot_index, plant)?
    {
        surface.insert(
            pl_decomp_slot_crop_symbol(root, slot_index, crop_slot_index),
            BoundaryValue::scalar(value),
        );
    }
    Ok(())
}

fn project_primary_decomposition_equation_aliases(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    plant: &openwepp_input_contract::parsers::management::PlantCroplandData,
) -> Result<(), HillslopeRuntimeInputError> {
    for (root, value) in decomposition_equation_parameter_values(1, 1, plant)? {
        surface.insert(BoundarySymbol::from(root), BoundaryValue::scalar(value));
    }
    Ok(())
}

fn project_growth_equation_symbols_forest(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    plant: &openwepp_input_contract::parsers::management::PlantForestData,
) -> Result<(), HillslopeRuntimeInputError> {
    for (root, value) in
        growth_equation_parameter_values_forest(slot_index, crop_slot_index, plant)?
    {
        surface.insert(
            pl_growth_slot_crop_symbol(root, slot_index, crop_slot_index),
            BoundaryValue::scalar(value),
        );
    }
    if let Some(values) = forest_phenology_parameter_values(slot_index, crop_slot_index, plant)? {
        for (root, value) in values {
            surface.insert(
                pl_growth_slot_crop_symbol(root, slot_index, crop_slot_index),
                BoundaryValue::scalar(value),
            );
        }
    }
    Ok(())
}

fn project_primary_growth_equation_aliases_forest(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    plant: &openwepp_input_contract::parsers::management::PlantForestData,
) -> Result<(), HillslopeRuntimeInputError> {
    for (root, value) in growth_equation_parameter_values_forest(1, 1, plant)? {
        surface.insert(BoundarySymbol::from(root), BoundaryValue::scalar(value));
    }
    if let Some(values) = forest_phenology_parameter_values(1, 1, plant)? {
        for (root, value) in values {
            surface.insert(BoundarySymbol::from(root), BoundaryValue::scalar(value));
        }
    }
    Ok(())
}

fn forest_phenology_parameter_values(
    slot_index: usize,
    crop_slot_index: usize,
    plant: &openwepp_input_contract::parsers::management::PlantForestData,
) -> Result<Option<[(&'static str, f64); 11]>, HillslopeRuntimeInputError> {
    let Some(phenology) = plant.phenology else {
        return Ok(None);
    };
    let summer_foliar_biomass_kg_m2 = validate_projection_positive(
        "forest_summer_foliar_biomass_kg_m2",
        slot_index,
        crop_slot_index,
        phenology.summer_foliar_biomass_kg_m2,
    )?;
    let evergreen_fraction = validate_projection_fraction(
        "forest_evergreen_fraction",
        slot_index,
        crop_slot_index,
        phenology.evergreen_fraction,
    )?;
    let structural_canopy_cover_fraction = validate_projection_fraction(
        "forest_structural_canopy_cover_fraction",
        slot_index,
        crop_slot_index,
        phenology.structural_canopy_cover_fraction,
    )?;
    if structural_canopy_cover_fraction > 0.999 {
        return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
            field: "forest_structural_canopy_cover_fraction",
            slot_index,
            crop_slot_index,
            value: structural_canopy_cover_fraction,
            allowed: "<=0.999",
        });
    }
    let structural_biomass_kg_m2 = validate_projection_non_negative(
        "forest_structural_biomass_kg_m2",
        slot_index,
        crop_slot_index,
        phenology.structural_biomass_kg_m2,
    )?;
    let minimum_temperature_inactive_c = validate_projection_finite(
        "forest_minimum_temperature_inactive_c",
        slot_index,
        crop_slot_index,
        phenology.minimum_temperature_inactive_c,
    )?;
    let minimum_temperature_unconstrained_c = validate_projection_finite(
        "forest_minimum_temperature_unconstrained_c",
        slot_index,
        crop_slot_index,
        phenology.minimum_temperature_unconstrained_c,
    )?;
    let vapor_pressure_deficit_unconstrained_pa = validate_projection_non_negative(
        "forest_vapor_pressure_deficit_unconstrained_pa",
        slot_index,
        crop_slot_index,
        phenology.vapor_pressure_deficit_unconstrained_pa,
    )?;
    let vapor_pressure_deficit_inactive_pa = validate_projection_non_negative(
        "forest_vapor_pressure_deficit_inactive_pa",
        slot_index,
        crop_slot_index,
        phenology.vapor_pressure_deficit_inactive_pa,
    )?;
    let photoperiod_inactive_hours = validate_projection_non_negative(
        "forest_photoperiod_inactive_hours",
        slot_index,
        crop_slot_index,
        phenology.photoperiod_inactive_hours,
    )?;
    let photoperiod_unconstrained_hours = validate_projection_non_negative(
        "forest_photoperiod_unconstrained_hours",
        slot_index,
        crop_slot_index,
        phenology.photoperiod_unconstrained_hours,
    )?;
    for (lower_name, lower, upper_name, upper) in [
        (
            "forest_minimum_temperature_inactive_c",
            minimum_temperature_inactive_c,
            "forest_minimum_temperature_unconstrained_c",
            minimum_temperature_unconstrained_c,
        ),
        (
            "forest_vapor_pressure_deficit_unconstrained_pa",
            vapor_pressure_deficit_unconstrained_pa,
            "forest_vapor_pressure_deficit_inactive_pa",
            vapor_pressure_deficit_inactive_pa,
        ),
        (
            "forest_photoperiod_inactive_hours",
            photoperiod_inactive_hours,
            "forest_photoperiod_unconstrained_hours",
            photoperiod_unconstrained_hours,
        ),
    ] {
        if lower >= upper {
            return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
                field: lower_name,
                slot_index,
                crop_slot_index,
                value: lower,
                allowed: upper_name,
            });
        }
    }
    if photoperiod_unconstrained_hours > 24.0 {
        return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
            field: "forest_photoperiod_unconstrained_hours",
            slot_index,
            crop_slot_index,
            value: photoperiod_unconstrained_hours,
            allowed: "<=24.0",
        });
    }
    Ok(Some([
        ("forest_phenology_model", 1.0),
        (
            "forest_summer_foliar_biomass_kg_m2",
            summer_foliar_biomass_kg_m2,
        ),
        ("forest_evergreen_fraction", evergreen_fraction),
        (
            "forest_structural_canopy_cover_fraction",
            structural_canopy_cover_fraction,
        ),
        ("forest_structural_biomass_kg_m2", structural_biomass_kg_m2),
        (
            "forest_minimum_temperature_inactive_c",
            minimum_temperature_inactive_c,
        ),
        (
            "forest_minimum_temperature_unconstrained_c",
            minimum_temperature_unconstrained_c,
        ),
        (
            "forest_vapor_pressure_deficit_unconstrained_pa",
            vapor_pressure_deficit_unconstrained_pa,
        ),
        (
            "forest_vapor_pressure_deficit_inactive_pa",
            vapor_pressure_deficit_inactive_pa,
        ),
        (
            "forest_photoperiod_inactive_hours",
            photoperiod_inactive_hours,
        ),
        (
            "forest_photoperiod_unconstrained_hours",
            photoperiod_unconstrained_hours,
        ),
    ]))
}

fn project_decomposition_equation_symbols_forest(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    plant: &openwepp_input_contract::parsers::management::PlantForestData,
) -> Result<(), HillslopeRuntimeInputError> {
    for (root, value) in
        decomposition_equation_parameter_values_forest(slot_index, crop_slot_index, plant)?
    {
        surface.insert(
            pl_decomp_slot_crop_symbol(root, slot_index, crop_slot_index),
            BoundaryValue::scalar(value),
        );
    }
    Ok(())
}

fn project_primary_decomposition_equation_aliases_forest(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    plant: &openwepp_input_contract::parsers::management::PlantForestData,
) -> Result<(), HillslopeRuntimeInputError> {
    for (root, value) in decomposition_equation_parameter_values_forest(1, 1, plant)? {
        surface.insert(BoundarySymbol::from(root), BoundaryValue::scalar(value));
    }
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn growth_equation_parameter_values(
    slot_index: usize,
    crop_slot_index: usize,
    plant: &openwepp_input_contract::parsers::management::PlantCroplandData,
) -> Result<[(&'static str, f64); 19], HillslopeRuntimeInputError> {
    let bb =
        validate_projection_non_negative("bb", slot_index, crop_slot_index, plant.canopy_line[0])?;
    let bbb =
        validate_projection_non_negative("bbb", slot_index, crop_slot_index, plant.canopy_line[1])?;
    let beinp = validate_projection_non_negative(
        "beinp",
        slot_index,
        crop_slot_index,
        plant.canopy_line[2],
    )?;
    let btemp =
        validate_projection_finite("btemp", slot_index, crop_slot_index, plant.canopy_line[3])?;
    let decfct =
        validate_projection_fraction("decfct", slot_index, crop_slot_index, plant.canopy_line[8])?;

    let dlai =
        validate_projection_fraction("dlai", slot_index, crop_slot_index, plant.growth_line[0])?;
    if dlai <= 0.0 {
        return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
            field: "dlai",
            slot_index,
            crop_slot_index,
            value: dlai,
            allowed: ">0.0 and <=1.0",
        });
    }
    let dropfc =
        validate_projection_fraction("dropfc", slot_index, crop_slot_index, plant.growth_line[1])?;
    let extnct = validate_projection_non_negative(
        "extnct",
        slot_index,
        crop_slot_index,
        plant.growth_line[2],
    )?;
    let flivmx = validate_projection_non_negative(
        "flivmx",
        slot_index,
        crop_slot_index,
        plant.growth_line[4],
    )?;
    let gddmax = validate_projection_non_negative(
        "gddmax",
        slot_index,
        crop_slot_index,
        plant.growth_line[5],
    )?;
    let hmax = validate_projection_non_negative(
        "hmax",
        slot_index,
        crop_slot_index,
        plant.growth_line[7],
    )?;
    let hi = validate_projection_fraction("hi", slot_index, crop_slot_index, plant.growth_line[6])?;

    let otemp =
        validate_projection_finite("otemp", slot_index, crop_slot_index, plant.residue_line[2])?;
    if otemp <= btemp {
        return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
            field: "otemp",
            slot_index,
            crop_slot_index,
            value: otemp,
            allowed: "> btemp",
        });
    }
    let pltol =
        validate_projection_finite("pltol", slot_index, crop_slot_index, plant.residue_line[3])?;
    let rdmax =
        validate_projection_positive("rdmax", slot_index, crop_slot_index, plant.residue_line[5])?;
    let rsr = validate_projection_non_negative(
        "rsr",
        slot_index,
        crop_slot_index,
        plant.residue_line[6],
    )?;
    let rtmmax = validate_projection_non_negative(
        "rtmmax",
        slot_index,
        crop_slot_index,
        plant.residue_line[7],
    )?;
    let spriod = validate_projection_non_negative(
        "spriod",
        slot_index,
        crop_slot_index,
        plant.residue_line[8],
    )?;
    let xmxlai = validate_projection_non_negative(
        "xmxlai",
        slot_index,
        crop_slot_index,
        plant.terminal_line[1],
    )?;

    Ok([
        ("btemp", btemp),
        ("otemp", otemp),
        ("gddmax", gddmax),
        ("dlai", dlai),
        ("dropfc", dropfc),
        ("decfct", decfct),
        ("spriod", spriod),
        ("bb", bb),
        ("bbb", bbb),
        ("beinp", beinp),
        ("extnct", extnct),
        ("flivmx", flivmx),
        ("hmax", hmax),
        ("hi", hi),
        ("pltol", pltol),
        ("xmxlai", xmxlai),
        ("rsr", rsr),
        ("rtmmax", rtmmax),
        ("rdmax", rdmax),
    ])
}

/// Forest analogue of [`growth_equation_parameter_values`]: emits the same 19
/// growth-kernel symbols in the same order and applies the same domain
/// validation, sourced from the first-class `PlantForestData.growth` block so
/// the daily growth kernel is unchanged (`LANUSE-AUTH-1`). Lookup-owned symbols
/// (`xmxlai`, `rdmax`, `decfct`, `dropfc`) carry the reconciled `(texture ×
/// class)` values; the remainder are the explicit forest-authority operands.
fn growth_equation_parameter_values_forest(
    slot_index: usize,
    crop_slot_index: usize,
    plant: &openwepp_input_contract::parsers::management::PlantForestData,
) -> Result<[(&'static str, f64); 19], HillslopeRuntimeInputError> {
    let growth = &plant.growth;
    let bb = validate_projection_non_negative("bb", slot_index, crop_slot_index, growth.bb)?;
    let bbb = validate_projection_non_negative("bbb", slot_index, crop_slot_index, growth.bbb)?;
    let beinp =
        validate_projection_non_negative("beinp", slot_index, crop_slot_index, growth.beinp)?;
    let btemp = validate_projection_finite("btemp", slot_index, crop_slot_index, growth.btemp)?;
    let decfct =
        validate_projection_fraction("decfct", slot_index, crop_slot_index, growth.decfct)?;

    let dlai = validate_projection_fraction("dlai", slot_index, crop_slot_index, growth.dlai)?;
    if dlai <= 0.0 {
        return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
            field: "dlai",
            slot_index,
            crop_slot_index,
            value: dlai,
            allowed: ">0.0 and <=1.0",
        });
    }
    let dropfc =
        validate_projection_fraction("dropfc", slot_index, crop_slot_index, growth.dropfc)?;
    let extnct =
        validate_projection_non_negative("extnct", slot_index, crop_slot_index, growth.extnct)?;
    let flivmx =
        validate_projection_non_negative("flivmx", slot_index, crop_slot_index, growth.flivmx)?;
    let gddmax =
        validate_projection_non_negative("gddmax", slot_index, crop_slot_index, growth.gddmax)?;
    let hmax = validate_projection_non_negative("hmax", slot_index, crop_slot_index, growth.hmax)?;
    let hi = validate_projection_fraction("hi", slot_index, crop_slot_index, growth.hi)?;

    let otemp = validate_projection_finite("otemp", slot_index, crop_slot_index, growth.otemp)?;
    if otemp <= btemp {
        return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
            field: "otemp",
            slot_index,
            crop_slot_index,
            value: otemp,
            allowed: "> btemp",
        });
    }
    let pltol = validate_projection_finite("pltol", slot_index, crop_slot_index, growth.pltol)?;
    let rdmax = validate_projection_positive("rdmax", slot_index, crop_slot_index, growth.rdmax)?;
    let rsr = validate_projection_non_negative("rsr", slot_index, crop_slot_index, growth.rsr)?;
    let rtmmax =
        validate_projection_non_negative("rtmmax", slot_index, crop_slot_index, growth.rtmmax)?;
    let spriod =
        validate_projection_non_negative("spriod", slot_index, crop_slot_index, growth.spriod)?;
    let xmxlai =
        validate_projection_non_negative("xmxlai", slot_index, crop_slot_index, growth.xmxlai)?;

    Ok([
        ("btemp", btemp),
        ("otemp", otemp),
        ("gddmax", gddmax),
        ("dlai", dlai),
        ("dropfc", dropfc),
        ("decfct", decfct),
        ("spriod", spriod),
        ("bb", bb),
        ("bbb", bbb),
        ("beinp", beinp),
        ("extnct", extnct),
        ("flivmx", flivmx),
        ("hmax", hmax),
        ("hi", hi),
        ("pltol", pltol),
        ("xmxlai", xmxlai),
        ("rsr", rsr),
        ("rtmmax", rtmmax),
        ("rdmax", rdmax),
    ])
}

fn decomposition_equation_parameter_values_forest(
    slot_index: usize,
    crop_slot_index: usize,
    plant: &openwepp_input_contract::parsers::management::PlantForestData,
) -> Result<[(&'static str, f64); 2], HillslopeRuntimeInputError> {
    let annual_decay_rate = validate_projection_non_negative(
        "oratea",
        slot_index,
        crop_slot_index,
        plant.decomposition.oratea,
    )?;
    let root_decay_rate = validate_projection_non_negative(
        "orater",
        slot_index,
        crop_slot_index,
        plant.decomposition.orater,
    )?;
    Ok([("oratea", annual_decay_rate), ("orater", root_decay_rate)])
}

fn decomposition_equation_parameter_values(
    slot_index: usize,
    crop_slot_index: usize,
    plant: &openwepp_input_contract::parsers::management::PlantCroplandData,
) -> Result<[(&'static str, f64); 2], HillslopeRuntimeInputError> {
    let annual_decay_rate = validate_projection_non_negative(
        "oratea",
        slot_index,
        crop_slot_index,
        plant.residue_line[0],
    )?;
    let root_decay_rate = validate_projection_non_negative(
        "orater",
        slot_index,
        crop_slot_index,
        plant.residue_line[1],
    )?;
    Ok([("oratea", annual_decay_rate), ("orater", root_decay_rate)])
}

fn validate_projection_finite(
    field: &'static str,
    slot_index: usize,
    crop_slot_index: usize,
    value: f64,
) -> Result<f64, HillslopeRuntimeInputError> {
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

fn validate_projection_non_negative(
    field: &'static str,
    slot_index: usize,
    crop_slot_index: usize,
    value: f64,
) -> Result<f64, HillslopeRuntimeInputError> {
    let value = validate_projection_finite(field, slot_index, crop_slot_index, value)?;
    if value < 0.0 {
        return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
            field,
            slot_index,
            crop_slot_index,
            value,
            allowed: ">=0.0",
        });
    }
    Ok(value)
}

#[derive(Debug, Clone, PartialEq)]
struct AnnualExtensionProjection {
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

impl AnnualExtensionProjection {
    const fn zeroed() -> Self {
        Self {
            jdherb: 0,
            jdburn: 0,
            jdslge: 0,
            jdcut: 0,
            jdmove: 0,
            fbrnag: 0.0,
            fbrnog: 0.0,
            frcut: 0.0,
            frmove: 0.0,
        }
    }
}

fn validate_projection_day(
    field: &'static str,
    slot_index: usize,
    crop_slot_index: usize,
    value: usize,
    allow_zero: bool,
) -> Result<usize, HillslopeRuntimeInputError> {
    if (allow_zero && value == 0) || (1..=366).contains(&value) {
        return Ok(value);
    }

    Err(HillslopeRuntimeInputError::PlProjectionDayOutOfDomain {
        field,
        slot_index,
        crop_slot_index,
        value,
        allowed: if allow_zero { "0 or 1..366" } else { "1..366" },
    })
}

fn validate_projection_fraction(
    field: &'static str,
    slot_index: usize,
    crop_slot_index: usize,
    value: f64,
) -> Result<f64, HillslopeRuntimeInputError> {
    if !value.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFinitePlProjectionField {
            field,
            slot_index,
            crop_slot_index,
            value,
        });
    }
    if !(0.0..=1.0).contains(&value) {
        return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
            field,
            slot_index,
            crop_slot_index,
            value,
            allowed: "0.0..=1.0",
        });
    }
    Ok(value)
}

fn validate_projection_positive(
    field: &'static str,
    slot_index: usize,
    crop_slot_index: usize,
    value: f64,
) -> Result<f64, HillslopeRuntimeInputError> {
    if !value.is_finite() {
        return Err(HillslopeRuntimeInputError::NonFinitePlProjectionField {
            field,
            slot_index,
            crop_slot_index,
            value,
        });
    }
    if value <= 0.0 {
        return Err(HillslopeRuntimeInputError::PlProjectionFieldOutOfDomain {
            field,
            slot_index,
            crop_slot_index,
            value,
            allowed: ">0.0",
        });
    }
    Ok(value)
}

fn annual_extension_variant_name(extension: Option<&YearlyAnnualExtension>) -> &'static str {
    match extension {
        Some(YearlyAnnualExtension::Herbicide { .. }) => "herbicide",
        Some(YearlyAnnualExtension::Burn { .. }) => "burn",
        Some(YearlyAnnualExtension::Silage { .. }) => "silage",
        Some(YearlyAnnualExtension::Cut { .. }) => "cut",
        Some(YearlyAnnualExtension::Remove { .. }) => "remove",
        None => "none",
    }
}

fn annual_extension_mismatch(
    slot_index: usize,
    crop_slot_index: usize,
    resmgt: usize,
    expected: &'static str,
    observed: Option<&YearlyAnnualExtension>,
) -> HillslopeRuntimeInputError {
    HillslopeRuntimeInputError::PlAnnualExtensionMismatch {
        slot_index,
        crop_slot_index,
        resmgt,
        expected,
        observed: annual_extension_variant_name(observed),
    }
}

fn project_annual_extension_controls(
    slot_index: usize,
    crop_slot_index: usize,
    resmgt: usize,
    extension: Option<&YearlyAnnualExtension>,
) -> Result<AnnualExtensionProjection, HillslopeRuntimeInputError> {
    match resmgt {
        1 => project_herbicide_annual_extension_controls(slot_index, crop_slot_index, extension),
        2 => project_burn_annual_extension_controls(slot_index, crop_slot_index, extension),
        3 => project_silage_annual_extension_controls(slot_index, crop_slot_index, extension),
        4 => project_cut_annual_extension_controls(slot_index, crop_slot_index, extension),
        5 => project_remove_annual_extension_controls(slot_index, crop_slot_index, extension),
        6 => project_no_annual_extension_controls(slot_index, crop_slot_index, resmgt, extension),
        7 => Err(
            HillslopeRuntimeInputError::PlProjectionUnsupportedPayloadCombination {
                field: "resmgt",
                slot_index,
                crop_slot_index,
                reason: "resmgt=7 annual-cut payload is not represented by runtime projection",
            },
        ),
        _ => Err(HillslopeRuntimeInputError::UnsupportedPlManagementOption {
            field: "resmgt",
            value: resmgt,
            allowed: "1..7",
        }),
    }
}

fn project_herbicide_annual_extension_controls(
    slot_index: usize,
    crop_slot_index: usize,
    extension: Option<&YearlyAnnualExtension>,
) -> Result<AnnualExtensionProjection, HillslopeRuntimeInputError> {
    let Some(YearlyAnnualExtension::Herbicide { jdherb }) = extension else {
        return Err(annual_extension_mismatch(
            slot_index,
            crop_slot_index,
            1,
            "herbicide",
            extension,
        ));
    };

    let mut projection = AnnualExtensionProjection::zeroed();
    projection.jdherb =
        validate_projection_day("jdherb", slot_index, crop_slot_index, *jdherb, false)?;
    Ok(projection)
}

fn project_burn_annual_extension_controls(
    slot_index: usize,
    crop_slot_index: usize,
    extension: Option<&YearlyAnnualExtension>,
) -> Result<AnnualExtensionProjection, HillslopeRuntimeInputError> {
    let Some(YearlyAnnualExtension::Burn {
        jdburn,
        fbmag,
        fbrnog,
    }) = extension
    else {
        return Err(annual_extension_mismatch(
            slot_index,
            crop_slot_index,
            2,
            "burn",
            extension,
        ));
    };

    let mut projection = AnnualExtensionProjection::zeroed();
    projection.jdburn =
        validate_projection_day("jdburn", slot_index, crop_slot_index, *jdburn, false)?;
    projection.fbrnag =
        validate_projection_fraction("fbrnag", slot_index, crop_slot_index, *fbmag)?;
    projection.fbrnog =
        validate_projection_fraction("fbrnog", slot_index, crop_slot_index, *fbrnog)?;
    Ok(projection)
}

fn project_silage_annual_extension_controls(
    slot_index: usize,
    crop_slot_index: usize,
    extension: Option<&YearlyAnnualExtension>,
) -> Result<AnnualExtensionProjection, HillslopeRuntimeInputError> {
    let Some(YearlyAnnualExtension::Silage { jdslge }) = extension else {
        return Err(annual_extension_mismatch(
            slot_index,
            crop_slot_index,
            3,
            "silage",
            extension,
        ));
    };

    let mut projection = AnnualExtensionProjection::zeroed();
    projection.jdslge =
        validate_projection_day("jdslge", slot_index, crop_slot_index, *jdslge, false)?;
    Ok(projection)
}

fn project_cut_annual_extension_controls(
    slot_index: usize,
    crop_slot_index: usize,
    extension: Option<&YearlyAnnualExtension>,
) -> Result<AnnualExtensionProjection, HillslopeRuntimeInputError> {
    let Some(YearlyAnnualExtension::Cut { jdcut, frcut }) = extension else {
        return Err(annual_extension_mismatch(
            slot_index,
            crop_slot_index,
            4,
            "cut",
            extension,
        ));
    };

    let mut projection = AnnualExtensionProjection::zeroed();
    projection.jdcut =
        validate_projection_day("jdcut", slot_index, crop_slot_index, *jdcut, false)?;
    projection.frcut = validate_projection_fraction("frcut", slot_index, crop_slot_index, *frcut)?;
    Ok(projection)
}

fn project_remove_annual_extension_controls(
    slot_index: usize,
    crop_slot_index: usize,
    extension: Option<&YearlyAnnualExtension>,
) -> Result<AnnualExtensionProjection, HillslopeRuntimeInputError> {
    let Some(YearlyAnnualExtension::Remove { jdmove, frmove }) = extension else {
        return Err(annual_extension_mismatch(
            slot_index,
            crop_slot_index,
            5,
            "remove",
            extension,
        ));
    };

    let mut projection = AnnualExtensionProjection::zeroed();
    projection.jdmove =
        validate_projection_day("jdmove", slot_index, crop_slot_index, *jdmove, false)?;
    projection.frmove =
        validate_projection_fraction("frmove", slot_index, crop_slot_index, *frmove)?;
    Ok(projection)
}

fn project_no_annual_extension_controls(
    slot_index: usize,
    crop_slot_index: usize,
    resmgt: usize,
    extension: Option<&YearlyAnnualExtension>,
) -> Result<AnnualExtensionProjection, HillslopeRuntimeInputError> {
    if extension.is_some() {
        return Err(annual_extension_mismatch(
            slot_index,
            crop_slot_index,
            resmgt,
            "none",
            extension,
        ));
    }

    Ok(AnnualExtensionProjection::zeroed())
}

fn project_annual_extension_symbols(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    projection: &AnnualExtensionProjection,
) -> Result<(), HillslopeRuntimeInputError> {
    surface.insert(
        pl_decomp_slot_crop_symbol("jdherb", slot_index, crop_slot_index),
        BoundaryValue::scalar(usize_to_f64("jdherb", projection.jdherb)?),
    );
    surface.insert(
        pl_decomp_slot_crop_symbol("jdburn", slot_index, crop_slot_index),
        BoundaryValue::scalar(usize_to_f64("jdburn", projection.jdburn)?),
    );
    surface.insert(
        pl_decomp_slot_crop_symbol("jdslge", slot_index, crop_slot_index),
        BoundaryValue::scalar(usize_to_f64("jdslge", projection.jdslge)?),
    );
    surface.insert(
        pl_decomp_slot_crop_symbol("jdcut", slot_index, crop_slot_index),
        BoundaryValue::scalar(usize_to_f64("jdcut", projection.jdcut)?),
    );
    surface.insert(
        pl_decomp_slot_crop_symbol("jdmove", slot_index, crop_slot_index),
        BoundaryValue::scalar(usize_to_f64("jdmove", projection.jdmove)?),
    );
    surface.insert(
        pl_decomp_slot_crop_symbol("fbrnag", slot_index, crop_slot_index),
        BoundaryValue::scalar(projection.fbrnag),
    );
    surface.insert(
        pl_decomp_slot_crop_symbol("fbrnog", slot_index, crop_slot_index),
        BoundaryValue::scalar(projection.fbrnog),
    );
    surface.insert(
        pl_decomp_slot_crop_symbol("frcut", slot_index, crop_slot_index),
        BoundaryValue::scalar(projection.frcut),
    );
    surface.insert(
        pl_decomp_slot_crop_symbol("frmove", slot_index, crop_slot_index),
        BoundaryValue::scalar(projection.frmove),
    );
    Ok(())
}

fn project_primary_annual_extension_aliases(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    projection: &AnnualExtensionProjection,
) -> Result<(), HillslopeRuntimeInputError> {
    surface.insert(
        BoundarySymbol::from("jdherb"),
        BoundaryValue::scalar(usize_to_f64("jdherb", projection.jdherb)?),
    );
    surface.insert(
        BoundarySymbol::from("jdburn"),
        BoundaryValue::scalar(usize_to_f64("jdburn", projection.jdburn)?),
    );
    surface.insert(
        BoundarySymbol::from("jdslge"),
        BoundaryValue::scalar(usize_to_f64("jdslge", projection.jdslge)?),
    );
    surface.insert(
        BoundarySymbol::from("jdcut"),
        BoundaryValue::scalar(usize_to_f64("jdcut", projection.jdcut)?),
    );
    surface.insert(
        BoundarySymbol::from("jdmove"),
        BoundaryValue::scalar(usize_to_f64("jdmove", projection.jdmove)?),
    );
    surface.insert(
        BoundarySymbol::from("fbrnag"),
        BoundaryValue::scalar(projection.fbrnag),
    );
    surface.insert(
        BoundarySymbol::from("fbrnog"),
        BoundaryValue::scalar(projection.fbrnog),
    );
    surface.insert(
        BoundarySymbol::from("frcut"),
        BoundaryValue::scalar(projection.frcut),
    );
    surface.insert(
        BoundarySymbol::from("frmove"),
        BoundaryValue::scalar(projection.frmove),
    );
    Ok(())
}

fn project_perennial_cutday_symbols(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    perennial: &openwepp_input_contract::parsers::management::YearlyPerennialData,
) -> Result<(), HillslopeRuntimeInputError> {
    for (position, cutday) in perennial.cut_days.iter().enumerate() {
        let cut_index = position + 1;
        let day = validate_projection_day("cutday", slot_index, crop_slot_index, *cutday, false)?;
        surface.insert(
            pl_decomp_slot_crop_indexed_symbol("cutday", slot_index, crop_slot_index, cut_index),
            BoundaryValue::scalar(usize_to_f64("cutday", day)?),
        );
    }
    Ok(())
}

fn project_perennial_grazing_cycle_symbols(
    surface: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    slot_index: usize,
    crop_slot_index: usize,
    perennial: &openwepp_input_contract::parsers::management::YearlyPerennialData,
) -> Result<(), HillslopeRuntimeInputError> {
    for (position, cycle) in perennial.grazing_cycles.iter().enumerate() {
        let cycle_index = position + 1;
        let gday = validate_projection_day("gday", slot_index, crop_slot_index, cycle.gday, false)?;
        let gend = validate_projection_day("gend", slot_index, crop_slot_index, cycle.gend, false)?;
        if gday >= gend {
            return Err(HillslopeRuntimeInputError::PlGrazingWindowOutOfDomain {
                slot_index,
                crop_slot_index,
                cycle_index,
                gday,
                gend,
            });
        }

        let animal =
            validate_projection_positive("animal", slot_index, crop_slot_index, cycle.animal)?;
        let bodywt =
            validate_projection_positive("bodywt", slot_index, crop_slot_index, cycle.bodywt)?;
        let area = validate_projection_positive("area", slot_index, crop_slot_index, cycle.area)?;
        let digest =
            validate_projection_fraction("digest", slot_index, crop_slot_index, cycle.digest)?;

        surface.insert(
            pl_decomp_slot_crop_indexed_symbol("gday", slot_index, crop_slot_index, cycle_index),
            BoundaryValue::scalar(usize_to_f64("gday", gday)?),
        );
        surface.insert(
            pl_decomp_slot_crop_indexed_symbol("gend", slot_index, crop_slot_index, cycle_index),
            BoundaryValue::scalar(usize_to_f64("gend", gend)?),
        );
        surface.insert(
            pl_decomp_slot_crop_indexed_symbol("animal", slot_index, crop_slot_index, cycle_index),
            BoundaryValue::scalar(animal),
        );
        surface.insert(
            pl_decomp_slot_crop_indexed_symbol("bodywt", slot_index, crop_slot_index, cycle_index),
            BoundaryValue::scalar(bodywt),
        );
        surface.insert(
            pl_decomp_slot_crop_indexed_symbol("area", slot_index, crop_slot_index, cycle_index),
            BoundaryValue::scalar(area),
        );
        surface.insert(
            pl_decomp_slot_crop_indexed_symbol("digest", slot_index, crop_slot_index, cycle_index),
            BoundaryValue::scalar(digest),
        );
    }
    Ok(())
}

fn pl_schedule_ofe_symbol(root: &str, ofe_index: usize) -> BoundarySymbol {
    BoundarySymbol::from(format!("pl_schedule_ofe{ofe_index}_{root}"))
}

fn pl_schedule_slot_symbol(root: &str, slot_index: usize) -> BoundarySymbol {
    BoundarySymbol::from(format!("pl_schedule_slot_{slot_index:04}_{root}"))
}

fn pl_schedule_slot_crop_symbol(
    root: &str,
    slot_index: usize,
    crop_slot_index: usize,
) -> BoundarySymbol {
    BoundarySymbol::from(format!(
        "pl_schedule_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}"
    ))
}

fn pl_growth_ofe_symbol(root: &str, ofe_index: usize) -> BoundarySymbol {
    BoundarySymbol::from(format!("pl_growth_ofe{ofe_index}_{root}"))
}

fn pl_growth_slot_crop_symbol(
    root: &str,
    slot_index: usize,
    crop_slot_index: usize,
) -> BoundarySymbol {
    BoundarySymbol::from(format!(
        "pl_growth_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}"
    ))
}

fn pl_decomp_ofe_symbol(root: &str, ofe_index: usize) -> BoundarySymbol {
    BoundarySymbol::from(format!("pl_decomp_ofe{ofe_index}_{root}"))
}

fn pl_decomp_slot_crop_symbol(
    root: &str,
    slot_index: usize,
    crop_slot_index: usize,
) -> BoundarySymbol {
    BoundarySymbol::from(format!(
        "pl_decomp_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}"
    ))
}

fn pl_decomp_slot_crop_indexed_symbol(
    root: &str,
    slot_index: usize,
    crop_slot_index: usize,
    index: usize,
) -> BoundarySymbol {
    BoundarySymbol::from(format!(
        "pl_decomp_slot_{slot_index:04}_crop_{crop_slot_index:04}_{root}_{index:04}"
    ))
}

fn slope_ofe_symbol(root: &str, ofe_index: usize) -> BoundarySymbol {
    BoundarySymbol::from(format!("ofe{ofe_index}_{root}"))
}

#[cfg(test)]
mod cqr_row4_projection_helper_tests {
    use super::*;
    use openwepp_input_contract::parsers::management::{
        YearlyAnnualExtension, YearlyPerennialData, YearlyPerennialGrazingCycle,
    };

    fn perennial_with_cycle(cycle: YearlyPerennialGrazingCycle) -> YearlyPerennialData {
        YearlyPerennialData {
            jdharv: 100,
            jdplt: 120,
            jdstop: 260,
            rw: 0.5,
            mgtopt: 3,
            cut_days: vec![180],
            grazing_cycles: vec![cycle],
        }
    }

    fn valid_grazing_cycle() -> YearlyPerennialGrazingCycle {
        YearlyPerennialGrazingCycle {
            animal: 2.0,
            area: 3.0,
            bodywt: 450.0,
            digest: 0.61,
            gday: 90,
            gend: 120,
        }
    }

    #[test]
    fn cqr_row4_annual_extension_variant_names_cover_all_payloads() {
        let herbicide = YearlyAnnualExtension::Herbicide { jdherb: 140 };
        let burn = YearlyAnnualExtension::Burn {
            jdburn: 150,
            fbmag: 0.1,
            fbrnog: 0.2,
        };
        let silage = YearlyAnnualExtension::Silage { jdslge: 160 };
        let cut = YearlyAnnualExtension::Cut {
            jdcut: 170,
            frcut: 0.4,
        };
        let remove = YearlyAnnualExtension::Remove {
            jdmove: 180,
            frmove: 0.5,
        };

        assert_eq!(annual_extension_variant_name(Some(&herbicide)), "herbicide");
        assert_eq!(annual_extension_variant_name(Some(&burn)), "burn");
        assert_eq!(annual_extension_variant_name(Some(&silage)), "silage");
        assert_eq!(annual_extension_variant_name(Some(&cut)), "cut");
        assert_eq!(annual_extension_variant_name(Some(&remove)), "remove");
        assert_eq!(annual_extension_variant_name(None), "none");
    }

    #[test]
    fn cqr_row4_perennial_grazing_cycle_symbols_project_typed_payload() {
        let perennial = perennial_with_cycle(valid_grazing_cycle());
        let mut surface = BTreeMap::new();

        project_perennial_grazing_cycle_symbols(&mut surface, 2, 3, &perennial)
            .expect("valid grazing cycle should project");

        let expected = [
            ("gday", 90.0),
            ("gend", 120.0),
            ("animal", 2.0),
            ("bodywt", 450.0),
            ("area", 3.0),
            ("digest", 0.61),
        ];
        for (root, value) in expected {
            let symbol = pl_decomp_slot_crop_indexed_symbol(root, 2, 3, 1);
            let actual = surface
                .get(&symbol)
                .unwrap_or_else(|| panic!("{symbol} should be projected"))
                .as_f64();
            assert!((actual - value).abs() < 1.0e-12);
        }
    }

    #[test]
    fn cqr_row4_perennial_grazing_cycle_symbols_reject_invalid_domains() {
        let mut invalid_window = valid_grazing_cycle();
        invalid_window.gend = invalid_window.gday;
        let perennial = perennial_with_cycle(invalid_window);
        let mut surface = BTreeMap::new();
        let error = project_perennial_grazing_cycle_symbols(&mut surface, 2, 3, &perennial)
            .expect_err("gday >= gend must fail closed");
        assert_eq!(error.code(), "HS-RUNTIME-E-049");

        let mut invalid_digest = valid_grazing_cycle();
        invalid_digest.digest = 1.5;
        let perennial = perennial_with_cycle(invalid_digest);
        let error = project_perennial_grazing_cycle_symbols(&mut surface, 2, 3, &perennial)
            .expect_err("digest outside unit interval must fail closed");
        assert_eq!(error.code(), "HS-RUNTIME-E-050");
    }
}
