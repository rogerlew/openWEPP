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

pub(crate) fn wb11_seed_failure(detail: impl Into<String>) -> HillslopeCliError {
    HillslopeCliError::RuntimeSurfaceFailure {
        surface: "wb11_seed",
        detail: format!("{SIMPIPE_GUARD_ID} {}", detail.into()),
    }
}

pub(crate) fn compute_wb11_et_demand_seed(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<Wb11EtDemandSeed, HillslopeCliError> {
    let iflget =
        runtime_surface_symbol_value(runtime_surface, "pmetpara.mode.iflget").unwrap_or(1.0);
    if !iflget.is_finite() {
        return Err(wb11_seed_failure(format!(
            "pmetpara.mode.iflget must be finite when present, observed {iflget}"
        )));
    }
    if (iflget - 1.0).abs() <= 1.0e-12 {
        return compute_priestley_taylor_wb11_et_demand(runtime_surface);
    }
    compute_evappm_wb11_et_demand(runtime_surface)
}

pub(crate) fn compute_priestley_taylor_wb11_et_demand(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<Wb11EtDemandSeed, HillslopeCliError> {
    let tmax = require_runtime_surface_scalar(runtime_surface, "tmax")?;
    let tmin = require_runtime_surface_scalar(runtime_surface, "tmin")?;
    let rad = require_runtime_surface_scalar(runtime_surface, "rad")?;
    if rad < 0.0 {
        return Err(wb11_seed_failure(format!(
            "rad must be >= 0.0, observed {rad}"
        )));
    }
    let salb = require_runtime_surface_scalar(runtime_surface, "salb")?;
    if !(0.0..=1.0).contains(&salb) {
        return Err(wb11_seed_failure(format!(
            "salb must be within [0,1], observed {salb}"
        )));
    }
    let cancov = require_runtime_surface_scalar(runtime_surface, "cancov")?;
    if cancov < 0.0 {
        return Err(wb11_seed_failure(format!(
            "cancov must be >= 0.0, observed {cancov}"
        )));
    }
    let lai = require_runtime_surface_scalar(runtime_surface, "lai")?;
    if lai < 0.0 {
        return Err(wb11_seed_failure(format!(
            "lai must be >= 0.0, observed {lai}"
        )));
    }

    let tave = 0.5 * (tmax + tmin);
    let tk = tave + 273.0;
    if tk <= 0.0 {
        return Err(wb11_seed_failure(format!(
            "derived tk must be > 0.0, observed {tk}"
        )));
    }
    let delta = (21.255 - 5304.0 / tk).exp() * 5304.0 / (tk * tk);
    let gamma = delta / (delta + 0.68);
    let eaj = (-0.5 * (cancov + 0.1)).exp();
    let alb = if lai > 0.0 {
        0.23 * (1.0 - eaj) + salb * eaj
    } else {
        salb
    };
    let demand_m = (0.00128 * ((rad * (1.0 - alb)) / 58.3) * gamma).max(0.0);
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

#[allow(clippy::manual_midpoint, clippy::similar_names, clippy::too_many_lines)]
pub(crate) fn compute_evappm_wb11_et_demand(
    runtime_surface: &HillslopeWritebackSurface,
) -> Result<Wb11EtDemandSeed, HillslopeCliError> {
    let tmax = require_runtime_surface_scalar(runtime_surface, "tmax")?;
    let tmin = require_runtime_surface_scalar(runtime_surface, "tmin")?;
    let tdpt = require_runtime_surface_scalar(runtime_surface, "tdpt")?;
    let rad = require_runtime_surface_scalar(runtime_surface, "rad")?;
    if rad < 0.0 {
        return Err(wb11_seed_failure(format!(
            "rad must be >= 0.0, observed {rad}"
        )));
    }
    let radpot = evappm_radpot_ly(runtime_surface)?;
    if radpot <= 0.0 {
        return Err(wb11_seed_failure(format!(
            "radpot must be > 0.0 for EVAPPM demand, observed {radpot}"
        )));
    }
    let vwind = require_runtime_surface_scalar(runtime_surface, "vwind")?;
    if vwind < 0.0 {
        return Err(wb11_seed_failure(format!(
            "vwind must be >= 0.0 for EVAPPM demand, observed {vwind}"
        )));
    }
    let elevm = require_runtime_surface_scalar(runtime_surface, "elevm")?;
    if elevm >= 45_076.923_076_923_08 {
        return Err(wb11_seed_failure(format!(
            "elevm keeps legacy pressure base positive, observed {elevm}"
        )));
    }
    let kcb = require_runtime_surface_scalar(runtime_surface, "pmetpara.selected.kcb")?;
    let rawp = require_runtime_surface_scalar(runtime_surface, "pmetpara.selected.rawp")?;
    let lai = require_runtime_surface_scalar(runtime_surface, "lai")?;
    if lai < 0.0 {
        return Err(wb11_seed_failure(format!(
            "lai must be >= 0.0, observed {lai}"
        )));
    }
    let canhgt = require_runtime_surface_scalar(runtime_surface, "canhgt")?;
    if canhgt < 0.0 {
        return Err(wb11_seed_failure(format!(
            "canhgt must be >= 0.0, observed {canhgt}"
        )));
    }
    let rtd = require_runtime_surface_scalar(runtime_surface, "rtd")?;
    if rtd < 0.0 {
        return Err(wb11_seed_failure(format!(
            "rtd must be >= 0.0, observed {rtd}"
        )));
    }
    let cancov = require_runtime_surface_scalar(runtime_surface, "cancov")?;
    if cancov < 0.0 {
        return Err(wb11_seed_failure(format!(
            "cancov must be >= 0.0, observed {cancov}"
        )));
    }
    let residue_interception =
        require_runtime_surface_scalar(runtime_surface, "wb17_residue_interception")?;
    if residue_interception < 0.0 {
        return Err(wb11_seed_failure(format!(
            "wb17_residue_interception must be >= 0.0, observed {residue_interception}"
        )));
    }

    let tave = 0.5 * (tmax + tmin);
    let ed = saturation_vapor_pressure_kpa(tdpt);
    let emaxt = saturation_vapor_pressure_kpa(tmax);
    let emint = saturation_vapor_pressure_kpa(tmin);
    let ee = 0.5 * (emaxt + emint);
    if emaxt <= 0.0 {
        return Err(wb11_seed_failure(format!(
            "derived emaxt must be > 0.0 for EVAPPM demand, observed {emaxt}"
        )));
    }
    let ra = rad / 23.9;
    let rso = radpot / 23.9;
    if rso <= 0.0 {
        return Err(wb11_seed_failure(format!(
            "derived rso must be > 0.0 for EVAPPM demand, observed {rso}"
        )));
    }
    let rbo = (0.34 - 0.14 * ed.sqrt())
        * 4.9e-9
        * (((tmax + 273.2).powi(4) + (tmin + 273.2).powi(4)) / 2.0)
        * (1.35 * (ra / rso) - 0.35);
    let rn_mj_m2 = ra * 0.77 - rbo;
    let fwv_m_s = vwind * 4.87 / (67.8_f64.mul_add(10.0, -5.42)).ln();
    let dlt = 4098.0 / ((tave + 237.3) * (tave + 237.3)) * saturation_vapor_pressure_kpa(tave);
    let pressure_base = 1.0 - 0.0065 * elevm / 293.0;
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
    let height_factor = (canhgt / 3.0).powf(0.3);
    let kcbadj = if lai > 0.0 && rtd > 0.0 {
        kcb + (0.04 * (fwv_m_s - 2.0) - 0.004 * (rhd_pct - 45.0)) * height_factor
    } else {
        0.0
    };
    let kcbcon = kcbadj * (1.0 - (-0.45 * lai).exp());
    let etke = if kcbadj > 0.0 {
        kcbadj * (-0.45 * lai).exp()
    } else {
        1.2
    };

    let nsl = scalar_to_usize(
        "wb11_nsl",
        runtime_surface_symbol_value(runtime_surface, "wb11_nsl")
            .or_else(|| runtime_surface_symbol_value(runtime_surface, "nsl"))
            .ok_or_else(|| wb11_seed_failure("missing required runtime symbol wb11_nsl/nsl"))?,
    )?;
    let mut profile_depth_m = 0.0_f64;
    for layer_index in 1..=nsl {
        profile_depth_m += require_evappm_layer_scalar(runtime_surface, layer_index, "wb19_dg")?;
    }
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
    for layer_index in 1..=nsl {
        let dg = require_evappm_layer_scalar(runtime_surface, layer_index, "wb19_dg")?;
        let solthk = runtime_surface_symbol_value(
            runtime_surface,
            format!("wb19_solthk_{layer_index:04}").as_str(),
        )
        .unwrap_or(cumulative_depth_m + dg);
        if solthk <= cumulative_depth_m {
            return Err(wb11_seed_failure(format!(
                "wb19_solthk_{layer_index:04} must increase with depth for EVAPPM demand"
            )));
        }
        let thetfc = require_evappm_layer_scalar(runtime_surface, layer_index, "wb19_thetfc")?;
        let thetdr = require_evappm_layer_scalar(runtime_surface, layer_index, "wb19_thetdr")?;
        let theta_store =
            require_evappm_layer_scalar(runtime_surface, layer_index, "wb18_perc_theta")?;
        if thetdr > thetfc {
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
            tew_mm += (thetfc - 0.5 * thetdr) * dg * 1_000.0 * layer_fraction;
            rew_mm += (thetfc - thetdr) * dg * 1_000.0 / 3.0 * layer_fraction;
            wfevp_mm += theta_store * 1_000.0 * layer_fraction;
        }
        cumulative_depth_m = layer_bottom_m;
        if cumulative_depth_m >= epdp_m {
            break;
        }
    }
    let wfevp_mm = wfevp_mm + residue_interception * 1_000.0;
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

    let tpdp_m = rtd.min(profile_depth_m);
    let mut taw_mm = 0.0_f64;
    let mut wftrp_mm = 0.0_f64;
    let mut cumulative_depth_m = 0.0_f64;
    for layer_index in 1..=nsl {
        let dg = require_evappm_layer_scalar(runtime_surface, layer_index, "wb19_dg")?;
        let solthk = runtime_surface_symbol_value(
            runtime_surface,
            format!("wb19_solthk_{layer_index:04}").as_str(),
        )
        .unwrap_or(cumulative_depth_m + dg);
        let thetfc = require_evappm_layer_scalar(runtime_surface, layer_index, "wb19_thetfc")?;
        let thetdr = require_evappm_layer_scalar(runtime_surface, layer_index, "wb19_thetdr")?;
        let theta_store =
            require_evappm_layer_scalar(runtime_surface, layer_index, "wb18_perc_theta")?;
        let layer_bottom_m = solthk;
        if tpdp_m <= 0.0 {
            break;
        }
        if layer_bottom_m <= tpdp_m {
            taw_mm += (thetfc - thetdr) * dg * 1_000.0;
            wftrp_mm += theta_store * 1_000.0;
        } else if cumulative_depth_m < tpdp_m {
            let layer_span_m = layer_bottom_m - cumulative_depth_m;
            if layer_span_m <= 0.0 {
                return Err(wb11_seed_failure(format!(
                    "wb19_solthk_{layer_index:04} must increase with depth for EVAPPM demand"
                )));
            }
            let fraction = (tpdp_m - cumulative_depth_m) / layer_span_m;
            taw_mm += (thetfc - thetdr) * dg * 1_000.0 * fraction;
            wftrp_mm = wfevp_mm + theta_store * 1_000.0 * fraction;
            break;
        }
        cumulative_depth_m = layer_bottom_m;
        if cumulative_depth_m >= tpdp_m {
            break;
        }
    }

    let etcsc = kcbadj * etorc_mm;
    let rawpaj = rawp + 0.04 * (5.0 - etcsc);
    let raw_mm = rawpaj * taw_mm;
    let etksden = taw_mm - raw_mm;
    let etks = if etksden <= 0.0 || (taw_mm - wftrp_mm) <= raw_mm {
        1.0
    } else {
        wftrp_mm / etksden
    };
    let potes_m = etorc_mm * etke * 0.001;
    let es_raw_m = if potes_m > residue_interception {
        let bpotes_m = potes_m - residue_interception;
        let eaj = (-0.5 * (cancov + 0.1)).exp();
        let kcmax = 1.2 + (0.04 * (fwv_m_s - 2.0) - 0.004 * (rhd_pct - 45.0)) * height_factor;
        let kecon = (etke * etkr).min(eaj * kcmax);
        kecon * bpotes_m / etke + residue_interception
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
