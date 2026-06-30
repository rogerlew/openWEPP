#[allow(clippy::wildcard_imports)]
use super::super::*;
#[derive(Debug, Clone, Copy)]
pub(crate) struct Wb11EtDemandSeed {
    demand_m: f64,
    branch_evappm: bool,
    diagnostics: Option<EvappmDemandDiagnostics>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct EvappmDemandDiagnostics {
    etorc_mm: f64,
    rn_mj_m2: f64,
    fwv_m_s: f64,
    rhd_pct: f64,
    kcbadj: f64,
    kcbcon: f64,
    etke: f64,
    etkr: f64,
    etks: f64,
    tew_mm: f64,
    rew_mm: f64,
    wfevp_mm: f64,
    taw_mm: f64,
    raw_mm: f64,
    wftrp_mm: f64,
    es_m: f64,
    es_storage_return_m: f64,
    ep_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct TypedWb11PriestleyTaylorEtInput {
    pub(crate) tmax_c: f64,
    pub(crate) tmin_c: f64,
    pub(crate) radiation_ly: f64,
    pub(crate) soil_albedo: f64,
    pub(crate) canopy_cover_fraction: f64,
    pub(crate) leaf_area_index: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct TypedWb11EvappmEtLayerInput {
    pub(crate) depth_m: f64,
    pub(crate) bottom_depth_m: Option<f64>,
    pub(crate) field_capacity_theta: f64,
    pub(crate) residual_theta: f64,
    pub(crate) theta_store_m: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TypedWb11EvappmEtInput {
    pub(crate) tmax_c: f64,
    pub(crate) tmin_c: f64,
    pub(crate) dewpoint_c: f64,
    pub(crate) radiation_ly: f64,
    pub(crate) potential_radiation_ly: f64,
    pub(crate) wind_m_s: f64,
    pub(crate) elevation_m: f64,
    pub(crate) crop_coefficient: f64,
    pub(crate) readily_available_water_fraction: f64,
    pub(crate) leaf_area_index: f64,
    pub(crate) canopy_height_m: f64,
    pub(crate) root_depth_m: f64,
    pub(crate) canopy_cover_fraction: f64,
    pub(crate) residue_interception_m: f64,
    pub(crate) layers: Vec<TypedWb11EvappmEtLayerInput>,
}

pub(crate) fn wb11_seed_failure(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "wb11_seed",
        detail: format!("{SIMPIPE_GUARD_ID} {}", detail.into()),
    }
}

pub(crate) fn compute_wb11_et_demand_seed(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<Wb11EtDemandSeed, HillslopeCliError> {
    let use_evappm = project_typed_wb11_et_demand_mode(runtime_surface_symbol_value(
        runtime_surface,
        "pmetpara.mode.iflget",
    ))?;
    if use_evappm {
        return compute_evappm_wb11_et_demand(runtime_surface);
    }
    compute_priestley_taylor_wb11_et_demand(runtime_surface)
}

pub(crate) fn project_typed_wb11_et_demand_mode(
    iflget: Option<f64>,
) -> Result<bool, HillslopeCliError> {
    let iflget = iflget.unwrap_or(1.0);
    if !iflget.is_finite() {
        return Err(wb11_seed_failure(format!(
            "pmetpara.mode.iflget must be finite when present, observed {iflget}"
        )));
    }
    Ok((iflget - 1.0).abs() > 1.0e-12)
}

pub(crate) fn compute_priestley_taylor_wb11_et_demand(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<Wb11EtDemandSeed, HillslopeCliError> {
    project_typed_wb11_priestley_taylor_et_demand(TypedWb11PriestleyTaylorEtInput {
        tmax_c: require_runtime_surface_scalar(runtime_surface, "tmax")?,
        tmin_c: require_runtime_surface_scalar(runtime_surface, "tmin")?,
        radiation_ly: require_runtime_surface_scalar(runtime_surface, "rad")?,
        soil_albedo: require_runtime_surface_scalar(runtime_surface, "salb")?,
        canopy_cover_fraction: require_runtime_surface_scalar(runtime_surface, "cancov")?,
        leaf_area_index: require_runtime_surface_scalar(runtime_surface, "lai")?,
    })
}

pub(crate) fn project_typed_wb11_priestley_taylor_et_demand(
    input: TypedWb11PriestleyTaylorEtInput,
) -> Result<Wb11EtDemandSeed, HillslopeCliError> {
    if input.radiation_ly < 0.0 {
        return Err(wb11_seed_failure(format!(
            "rad must be >= 0.0, observed {}",
            input.radiation_ly
        )));
    }
    if !(0.0..=1.0).contains(&input.soil_albedo) {
        return Err(wb11_seed_failure(format!(
            "salb must be within [0,1], observed {}",
            input.soil_albedo
        )));
    }
    if input.canopy_cover_fraction < 0.0 {
        return Err(wb11_seed_failure(format!(
            "cancov must be >= 0.0, observed {}",
            input.canopy_cover_fraction
        )));
    }
    if input.leaf_area_index < 0.0 {
        return Err(wb11_seed_failure(format!(
            "lai must be >= 0.0, observed {}",
            input.leaf_area_index
        )));
    }

    let tave = 0.5 * (input.tmax_c + input.tmin_c);
    let tk = tave + 273.0;
    if tk <= 0.0 {
        return Err(wb11_seed_failure(format!(
            "derived tk must be > 0.0, observed {tk}"
        )));
    }
    let delta = (21.255 - 5304.0 / tk).exp() * 5304.0 / (tk * tk);
    let gamma = delta / (delta + 0.68);
    let eaj = (-0.5 * (input.canopy_cover_fraction + 0.1)).exp();
    let alb = if input.leaf_area_index > 0.0 {
        0.23 * (1.0 - eaj) + input.soil_albedo * eaj
    } else {
        input.soil_albedo
    };
    let demand_m = (0.00128 * ((input.radiation_ly * (1.0 - alb)) / 58.3) * gamma).max(0.0);
    if !demand_m.is_finite() {
        return Err(wb11_seed_failure(format!(
            "derived wb11_et_demand is non-finite ({demand_m})"
        )));
    }

    Ok(Wb11EtDemandSeed {
        demand_m,
        branch_evappm: false,
        diagnostics: None,
    })
}

pub(crate) fn compute_evappm_wb11_et_demand(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<Wb11EtDemandSeed, HillslopeCliError> {
    let input = read_typed_wb11_evappm_et_input(runtime_surface)?;
    project_typed_wb11_evappm_et_demand(&input)
}

#[allow(clippy::too_many_lines)]
fn read_typed_wb11_evappm_et_input(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<TypedWb11EvappmEtInput, HillslopeCliError> {
    let tmax_c = require_runtime_surface_scalar(runtime_surface, "tmax")?;
    let tmin_c = require_runtime_surface_scalar(runtime_surface, "tmin")?;
    let dewpoint_c = require_runtime_surface_scalar(runtime_surface, "tdpt")?;
    let radiation_ly = require_runtime_surface_scalar(runtime_surface, "rad")?;
    if radiation_ly < 0.0 {
        return Err(wb11_seed_failure(format!(
            "rad must be >= 0.0, observed {radiation_ly}"
        )));
    }
    let potential_radiation_ly = evappm_radpot_ly(runtime_surface)?;
    if potential_radiation_ly <= 0.0 {
        return Err(wb11_seed_failure(format!(
            "radpot must be > 0.0 for EVAPPM demand, observed {potential_radiation_ly}"
        )));
    }
    let wind_m_s = require_runtime_surface_scalar(runtime_surface, "vwind")?;
    if wind_m_s < 0.0 {
        return Err(wb11_seed_failure(format!(
            "vwind must be >= 0.0 for EVAPPM demand, observed {wind_m_s}"
        )));
    }
    let elevation_m = require_runtime_surface_scalar(runtime_surface, "elevm")?;
    if elevation_m >= 45_076.923_076_923_08 {
        return Err(wb11_seed_failure(format!(
            "elevm keeps legacy pressure base positive, observed {elevation_m}"
        )));
    }
    let crop_coefficient =
        require_runtime_surface_scalar(runtime_surface, "pmetpara.selected.kcb")?;
    let readily_available_water_fraction =
        require_runtime_surface_scalar(runtime_surface, "pmetpara.selected.rawp")?;
    let leaf_area_index = require_runtime_surface_scalar(runtime_surface, "lai")?;
    if leaf_area_index < 0.0 {
        return Err(wb11_seed_failure(format!(
            "lai must be >= 0.0, observed {leaf_area_index}"
        )));
    }
    let canopy_height_m = require_runtime_surface_scalar(runtime_surface, "canhgt")?;
    if canopy_height_m < 0.0 {
        return Err(wb11_seed_failure(format!(
            "canhgt must be >= 0.0, observed {canopy_height_m}"
        )));
    }
    let root_depth_m = require_runtime_surface_scalar(runtime_surface, "rtd")?;
    if root_depth_m < 0.0 {
        return Err(wb11_seed_failure(format!(
            "rtd must be >= 0.0, observed {root_depth_m}"
        )));
    }
    let canopy_cover_fraction = require_runtime_surface_scalar(runtime_surface, "cancov")?;
    if canopy_cover_fraction < 0.0 {
        return Err(wb11_seed_failure(format!(
            "cancov must be >= 0.0, observed {canopy_cover_fraction}"
        )));
    }
    let residue_interception_m =
        require_runtime_surface_scalar(runtime_surface, "wb17_residue_interception")?;
    if residue_interception_m < 0.0 {
        return Err(wb11_seed_failure(format!(
            "wb17_residue_interception must be >= 0.0, observed {residue_interception_m}"
        )));
    }

    let nsl = scalar_to_usize(
        "wb11_nsl",
        runtime_surface_symbol_value(runtime_surface, "wb11_nsl")
            .or_else(|| runtime_surface_symbol_value(runtime_surface, "nsl"))
            .ok_or_else(|| wb11_seed_failure("missing required runtime symbol wb11_nsl/nsl"))?,
    )?;
    let mut layers = Vec::with_capacity(nsl);
    for layer_index in 1..=nsl {
        layers.push(TypedWb11EvappmEtLayerInput {
            depth_m: require_evappm_layer_scalar(runtime_surface, layer_index, "wb19_dg")?,
            bottom_depth_m: runtime_surface_symbol_value(
                runtime_surface,
                format!("wb19_solthk_{layer_index:04}").as_str(),
            ),
            field_capacity_theta: require_evappm_layer_scalar(
                runtime_surface,
                layer_index,
                "wb19_thetfc",
            )?,
            residual_theta: require_evappm_layer_scalar(
                runtime_surface,
                layer_index,
                "wb19_thetdr",
            )?,
            theta_store_m: require_evappm_layer_scalar(
                runtime_surface,
                layer_index,
                "wb18_perc_theta",
            )?,
        });
    }

    Ok(TypedWb11EvappmEtInput {
        tmax_c,
        tmin_c,
        dewpoint_c,
        radiation_ly,
        potential_radiation_ly,
        wind_m_s,
        elevation_m,
        crop_coefficient,
        readily_available_water_fraction,
        leaf_area_index,
        canopy_height_m,
        root_depth_m,
        canopy_cover_fraction,
        residue_interception_m,
        layers,
    })
}

#[allow(clippy::manual_midpoint, clippy::similar_names, clippy::too_many_lines)]
pub(crate) fn project_typed_wb11_evappm_et_demand(
    input: &TypedWb11EvappmEtInput,
) -> Result<Wb11EtDemandSeed, HillslopeCliError> {
    validate_typed_wb11_evappm_et_input(input)?;

    let tave = 0.5 * (input.tmax_c + input.tmin_c);
    let ed = saturation_vapor_pressure_kpa(input.dewpoint_c);
    let emaxt = saturation_vapor_pressure_kpa(input.tmax_c);
    let emint = saturation_vapor_pressure_kpa(input.tmin_c);
    let ee = 0.5 * (emaxt + emint);
    if emaxt <= 0.0 {
        return Err(wb11_seed_failure(format!(
            "derived emaxt must be > 0.0 for EVAPPM demand, observed {emaxt}"
        )));
    }
    let ra = input.radiation_ly / 23.9;
    let rso = input.potential_radiation_ly / 23.9;
    if rso <= 0.0 {
        return Err(wb11_seed_failure(format!(
            "derived rso must be > 0.0 for EVAPPM demand, observed {rso}"
        )));
    }
    let rbo = (0.34 - 0.14 * ed.sqrt())
        * 4.9e-9
        * (((input.tmax_c + 273.2).powi(4) + (input.tmin_c + 273.2).powi(4)) / 2.0)
        * (1.35 * (ra / rso) - 0.35);
    let rn_mj_m2 = ra * 0.77 - rbo;
    let fwv_m_s = input.wind_m_s * 4.87 / (67.8_f64.mul_add(10.0, -5.42)).ln();
    let dlt = 4098.0 / ((tave + 237.3) * (tave + 237.3)) * saturation_vapor_pressure_kpa(tave);
    let pressure_base = 1.0 - 0.0065 * input.elevation_m / 293.0;
    if pressure_base <= 0.0 {
        return Err(wb11_seed_failure(format!(
            "legacy pressure base must be > 0.0 for EVAPPM demand, observed {pressure_base}"
        )));
    }
    let pb = 101.3 * pressure_base.powf(5.26);
    let gma = 0.000_665 * pb;
    let denominator = dlt + gma * (1.0 + 0.34 * fwv_m_s);
    if denominator <= 0.0 {
        return Err(wb11_seed_failure(format!(
            "EVAPPM etorc denominator must be > 0.0, observed {denominator}"
        )));
    }
    let etorc_mm = (0.408 * dlt * rn_mj_m2 + gma * (900.0 / (tave + 273.0)) * (ee - ed) * fwv_m_s)
        / denominator;
    let rhd_pct = ed / emaxt * 100.0;
    let height_factor = (input.canopy_height_m / 3.0).powf(0.3);
    let kcbadj = if input.leaf_area_index > 0.0 && input.root_depth_m > 0.0 {
        input.crop_coefficient + (0.04 * (fwv_m_s - 2.0) - 0.004 * (rhd_pct - 45.0)) * height_factor
    } else {
        0.0
    };
    let kcbcon = kcbadj * (1.0 - (-0.45 * input.leaf_area_index).exp());
    let etke = if kcbadj > 0.0 {
        kcbadj * (-0.45 * input.leaf_area_index).exp()
    } else {
        1.2
    };

    let profile_depth_m: f64 = input.layers.iter().map(|layer| layer.depth_m).sum();
    if profile_depth_m <= 0.0 {
        return Err(wb11_seed_failure(
            "soil profile depth must be > 0.0 for EVAPPM demand",
        ));
    }

    let epdp_m = 0.1_f64.min(profile_depth_m);
    let mut tew_mm = 0.0_f64;
    let mut rew_mm = 0.0_f64;
    let mut wfevp_mm = 0.0_f64;
    let mut cumulative_depth_m = 0.0_f64;
    for (layer_offset, layer) in input.layers.iter().copied().enumerate() {
        let layer_index = layer_offset + 1;
        let solthk = layer
            .bottom_depth_m
            .unwrap_or(cumulative_depth_m + layer.depth_m);
        if solthk <= cumulative_depth_m {
            return Err(wb11_seed_failure(format!(
                "wb19_solthk_{layer_index:04} must increase with depth for EVAPPM demand"
            )));
        }
        if layer.residual_theta > layer.field_capacity_theta {
            return Err(wb11_seed_failure(format!(
                "wb19_thetdr_{layer_index:04} must be <= wb19_thetfc_{layer_index:04}"
            )));
        }
        let layer_bottom_m = solthk;
        let layer_fraction = if layer_bottom_m <= epdp_m {
            1.0
        } else if cumulative_depth_m < epdp_m {
            (epdp_m - cumulative_depth_m) / (layer_bottom_m - cumulative_depth_m)
        } else {
            0.0
        };
        if layer_fraction > 0.0 {
            tew_mm += (layer.field_capacity_theta - 0.5 * layer.residual_theta)
                * layer.depth_m
                * 1_000.0
                * layer_fraction;
            rew_mm += (layer.field_capacity_theta - layer.residual_theta) * layer.depth_m * 1_000.0
                / 3.0
                * layer_fraction;
            wfevp_mm += layer.theta_store_m * 1_000.0 * layer_fraction;
        }
        cumulative_depth_m = layer_bottom_m;
        if cumulative_depth_m >= epdp_m {
            break;
        }
    }
    let wfevp_mm = wfevp_mm + input.residue_interception_m * 1_000.0;
    let etkr = if (tew_mm - wfevp_mm) <= rew_mm {
        1.0
    } else {
        let denominator = tew_mm - rew_mm;
        if denominator <= 0.0 {
            1.0
        } else {
            (wfevp_mm / denominator).powi(2)
        }
    };

    let tpdp_m = input.root_depth_m.min(profile_depth_m);
    let mut taw_mm = 0.0_f64;
    let mut wftrp_mm = 0.0_f64;
    let mut cumulative_depth_m = 0.0_f64;
    for (layer_offset, layer) in input.layers.iter().copied().enumerate() {
        let layer_index = layer_offset + 1;
        let solthk = layer
            .bottom_depth_m
            .unwrap_or(cumulative_depth_m + layer.depth_m);
        let layer_bottom_m = solthk;
        if tpdp_m <= 0.0 {
            break;
        }
        if layer_bottom_m <= tpdp_m {
            taw_mm += (layer.field_capacity_theta - layer.residual_theta) * layer.depth_m * 1_000.0;
            wftrp_mm += layer.theta_store_m * 1_000.0;
        } else if cumulative_depth_m < tpdp_m {
            let layer_span_m = layer_bottom_m - cumulative_depth_m;
            if layer_span_m <= 0.0 {
                return Err(wb11_seed_failure(format!(
                    "wb19_solthk_{layer_index:04} must increase with depth for EVAPPM demand"
                )));
            }
            let fraction = (tpdp_m - cumulative_depth_m) / layer_span_m;
            taw_mm += (layer.field_capacity_theta - layer.residual_theta)
                * layer.depth_m
                * 1_000.0
                * fraction;
            wftrp_mm = wfevp_mm + layer.theta_store_m * 1_000.0 * fraction;
            break;
        }
        cumulative_depth_m = layer_bottom_m;
        if cumulative_depth_m >= tpdp_m {
            break;
        }
    }

    let etcsc = kcbadj * etorc_mm;
    let rawpaj = input.readily_available_water_fraction + 0.04 * (5.0 - etcsc);
    let raw_mm = rawpaj * taw_mm;
    let etksden = taw_mm - raw_mm;
    let etks = if etksden <= 0.0 || (taw_mm - wftrp_mm) <= raw_mm {
        1.0
    } else {
        wftrp_mm / etksden
    };
    let potes_m = etorc_mm * etke * 0.001;
    let es_raw_m = if potes_m > input.residue_interception_m {
        let bpotes_m = potes_m - input.residue_interception_m;
        let eaj = (-0.5 * (input.canopy_cover_fraction + 0.1)).exp();
        let kcmax = 1.2 + (0.04 * (fwv_m_s - 2.0) - 0.004 * (rhd_pct - 45.0)) * height_factor;
        let kecon = (etke * etkr).min(eaj * kcmax);
        kecon * bpotes_m / etke + input.residue_interception_m
    } else {
        potes_m
    };
    let es_storage_return_m = if es_raw_m < 0.0 { -es_raw_m } else { 0.0 };
    let es_m = if es_raw_m < 0.0 { 0.0 } else { es_raw_m };
    let ep_raw_m = etorc_mm * etks * kcbcon * 0.001;
    let ep_m = if ep_raw_m < 0.0 { 0.0 } else { ep_raw_m };

    let diagnostics = EvappmDemandDiagnostics {
        etorc_mm,
        rn_mj_m2,
        fwv_m_s,
        rhd_pct,
        kcbadj,
        kcbcon,
        etke,
        etkr,
        etks,
        tew_mm,
        rew_mm,
        wfevp_mm,
        taw_mm,
        raw_mm,
        wftrp_mm,
        es_m,
        es_storage_return_m,
        ep_m,
    };
    for (name, value) in [
        ("pmet.etorc_mm", diagnostics.etorc_mm),
        ("pmet.rn_mj_m2", diagnostics.rn_mj_m2),
        ("pmet.fwv_m_s", diagnostics.fwv_m_s),
        ("pmet.rhd_pct", diagnostics.rhd_pct),
        ("pmet.kcbadj", diagnostics.kcbadj),
        ("pmet.kcbcon", diagnostics.kcbcon),
        ("pmet.etke", diagnostics.etke),
        ("pmet.etkr", diagnostics.etkr),
        ("pmet.etks", diagnostics.etks),
        ("pmet.tew_mm", diagnostics.tew_mm),
        ("pmet.rew_mm", diagnostics.rew_mm),
        ("pmet.wfevp_mm", diagnostics.wfevp_mm),
        ("pmet.taw_mm", diagnostics.taw_mm),
        ("pmet.raw_mm", diagnostics.raw_mm),
        ("pmet.wftrp_mm", diagnostics.wftrp_mm),
        ("pmet.es_m", diagnostics.es_m),
        ("pmet.es_storage_return_m", diagnostics.es_storage_return_m),
        ("pmet.ep_m", diagnostics.ep_m),
    ] {
        if !value.is_finite() {
            return Err(wb11_seed_failure(format!(
                "derived {name} must be finite, observed {value}"
            )));
        }
    }

    Ok(Wb11EtDemandSeed {
        demand_m: ep_m,
        branch_evappm: true,
        diagnostics: Some(diagnostics),
    })
}

fn validate_typed_wb11_evappm_et_input(
    input: &TypedWb11EvappmEtInput,
) -> Result<(), HillslopeCliError> {
    if input.radiation_ly < 0.0 {
        return Err(wb11_seed_failure(format!(
            "rad must be >= 0.0, observed {}",
            input.radiation_ly
        )));
    }
    if input.potential_radiation_ly <= 0.0 {
        return Err(wb11_seed_failure(format!(
            "radpot must be > 0.0 for EVAPPM demand, observed {}",
            input.potential_radiation_ly
        )));
    }
    if input.wind_m_s < 0.0 {
        return Err(wb11_seed_failure(format!(
            "vwind must be >= 0.0 for EVAPPM demand, observed {}",
            input.wind_m_s
        )));
    }
    if input.elevation_m >= 45_076.923_076_923_08 {
        return Err(wb11_seed_failure(format!(
            "elevm keeps legacy pressure base positive, observed {}",
            input.elevation_m
        )));
    }
    if input.leaf_area_index < 0.0 {
        return Err(wb11_seed_failure(format!(
            "lai must be >= 0.0, observed {}",
            input.leaf_area_index
        )));
    }
    if input.canopy_height_m < 0.0 {
        return Err(wb11_seed_failure(format!(
            "canhgt must be >= 0.0, observed {}",
            input.canopy_height_m
        )));
    }
    if input.root_depth_m < 0.0 {
        return Err(wb11_seed_failure(format!(
            "rtd must be >= 0.0, observed {}",
            input.root_depth_m
        )));
    }
    if input.canopy_cover_fraction < 0.0 {
        return Err(wb11_seed_failure(format!(
            "cancov must be >= 0.0, observed {}",
            input.canopy_cover_fraction
        )));
    }
    if input.residue_interception_m < 0.0 {
        return Err(wb11_seed_failure(format!(
            "wb17_residue_interception must be >= 0.0, observed {}",
            input.residue_interception_m
        )));
    }
    for (layer_offset, layer) in input.layers.iter().enumerate() {
        let layer_index = layer_offset + 1;
        if !layer.depth_m.is_finite() {
            return Err(wb11_seed_failure(format!(
                "wb19_dg_{layer_index:04} must be finite for EVAPPM demand, observed {}",
                layer.depth_m
            )));
        }
        if !layer.field_capacity_theta.is_finite() {
            return Err(wb11_seed_failure(format!(
                "wb19_thetfc_{layer_index:04} must be finite for EVAPPM demand, observed {}",
                layer.field_capacity_theta
            )));
        }
        if !layer.residual_theta.is_finite() {
            return Err(wb11_seed_failure(format!(
                "wb19_thetdr_{layer_index:04} must be finite for EVAPPM demand, observed {}",
                layer.residual_theta
            )));
        }
        if !layer.theta_store_m.is_finite() {
            return Err(wb11_seed_failure(format!(
                "wb18_perc_theta_{layer_index:04} must be finite for EVAPPM demand, observed {}",
                layer.theta_store_m
            )));
        }
    }
    Ok(())
}

pub(crate) fn saturation_vapor_pressure_kpa(temperature_c: f64) -> f64 {
    0.6108 * (17.27 * temperature_c / (temperature_c + 237.3)).exp()
}

pub(crate) fn require_evappm_layer_scalar(
    runtime_surface: &HillslopeWritebackSurface,
    layer_index: usize,
    root: &str,
) -> Result<f64, HillslopeCliError> {
    let symbol = wb13_primary_layer_symbol(root, layer_index);
    let value = require_runtime_surface_scalar(runtime_surface, symbol.as_str())?;
    if !value.is_finite() {
        return Err(wb11_seed_failure(format!(
            "{symbol} must be finite for EVAPPM demand, observed {value}"
        )));
    }
    Ok(value)
}

pub(crate) fn evappm_radpot_ly(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<f64, HillslopeCliError> {
    if let Some(radpot) = runtime_surface_symbol_value(runtime_surface, "radpot") {
        if !radpot.is_finite() {
            return Err(wb11_seed_failure(format!(
                "radpot must be finite when present, observed {radpot}"
            )));
        }
        return Ok(radpot);
    }

    let deglat = require_runtime_surface_scalar(runtime_surface, "deglat")?;
    let year = require_runtime_surface_scalar(runtime_surface, "year")?;
    let mon = require_runtime_surface_scalar(runtime_surface, "mon")?;
    let day = require_runtime_surface_scalar(runtime_surface, "day")?;
    let year = scalar_to_i32("year", year)?;
    let mon = scalar_to_i32("mon", mon)?;
    let day = scalar_to_i32("day", day)?;
    let sdate = f64::from(day_of_year(year, mon, day)?);
    Ok(legacy_sunmap_horizontal_radpot_ly(deglat, sdate))
}

pub(crate) fn legacy_sunmap_horizontal_radpot_ly(deglat: f64, sdate: f64) -> f64 {
    let pi = std::f64::consts::PI;
    let radlat = deglat * pi / 180.0;
    let declination = 0.00698 - 0.4067 * ((sdate + 10.0) * 0.0172).cos();
    let earth_sun_distance_factor = 1.0 - 0.0167 * ((sdate - 3.0) * 0.0172).cos();
    let radiation_factor = (60.0 * 1.94) / (earth_sun_distance_factor * earth_sun_distance_factor);
    let sunset_argument = -(radlat.tan() * declination.tan()).clamp(-1.0, 1.0);
    let sunset_angle = sunset_argument.acos();
    radiation_factor
        * ((declination.sin() * radlat.sin() * (sunset_angle - -sunset_angle) * 12.0 / pi)
            + (declination.cos()
                * radlat.cos()
                * (sunset_angle.sin() - (-sunset_angle).sin())
                * 12.0
                / pi))
}

pub(crate) fn publish_wb11_et_demand_seed(
    runtime_surface: &mut HillslopeWritebackSurface,
    seed: Wb11EtDemandSeed,
) -> Result<(), HillslopeCliError> {
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb11_et_demand"),
        BoundaryValue::scalar(seed.demand_m),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb11_et_seed_branch_priestley_taylor"),
        BoundaryValue::scalar(if seed.branch_evappm { 0.0 } else { 1.0 }),
    );
    runtime_surface.state_surface.insert(
        BoundarySymbol::from("wb11_et_seed_branch_evappm"),
        BoundaryValue::scalar(if seed.branch_evappm { 1.0 } else { 0.0 }),
    );
    if let Some(diagnostics) = seed.diagnostics {
        runtime_surface.state_surface.insert(
            BoundarySymbol::from("pmet.es_storage_return_m"),
            BoundaryValue::water_depth_meters(diagnostics.es_storage_return_m).map_err(|error| {
                wb11_seed_failure(format!(
                    "pmet.es_storage_return_m must be a non-negative finite water depth: {error}"
                ))
            })?,
        );
        for (symbol, value) in [
            ("pmet.etorc_mm", diagnostics.etorc_mm),
            ("pmet.rn_mj_m2", diagnostics.rn_mj_m2),
            ("pmet.fwv_m_s", diagnostics.fwv_m_s),
            ("pmet.rhd_pct", diagnostics.rhd_pct),
            ("pmet.kcbadj", diagnostics.kcbadj),
            ("pmet.kcbcon", diagnostics.kcbcon),
            ("pmet.etke", diagnostics.etke),
            ("pmet.etkr", diagnostics.etkr),
            ("pmet.etks", diagnostics.etks),
            ("pmet.tew_mm", diagnostics.tew_mm),
            ("pmet.rew_mm", diagnostics.rew_mm),
            ("pmet.wfevp_mm", diagnostics.wfevp_mm),
            ("pmet.taw_mm", diagnostics.taw_mm),
            ("pmet.raw_mm", diagnostics.raw_mm),
            ("pmet.wftrp_mm", diagnostics.wftrp_mm),
            ("pmet.es_m", diagnostics.es_m),
            ("pmet.ep_m", diagnostics.ep_m),
        ] {
            runtime_surface
                .state_surface
                .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
        }
    }
    Ok(())
}
