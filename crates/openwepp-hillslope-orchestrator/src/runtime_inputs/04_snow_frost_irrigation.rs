/// Typed parsed snow-control seed projection.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypedSnowRuntimeProjection {
    pub rst_c: f64,
    pub newsnw_kg_m3: f64,
    pub ssd_kg_m3: f64,
    pub snow_file_present: bool,
    pub runtime_swe_m: f64,
    pub runtime_depth_m: f64,
    pub runtime_density_kg_m3: f64,
    pub runtime_settle_day_count: f64,
}

/// Typed parsed frost-control seed projection.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypedFrostRuntimeProjection {
    pub wint_red: bool,
    pub fine_top: i32,
    pub fine_bot: i32,
    pub ksnowf: f64,
    pub kresf: f64,
    pub ksoilf: f64,
    pub kfactor1: f64,
    pub kfactor2: f64,
    pub kfactor3: f64,
    pub frost_file_present: bool,
    pub dfrost_m: f64,
    pub dthaw_m: f64,
    pub nft: f64,
    pub ws_frz_m: f64,
    pub frwatc_soil_water_before_m: f64,
    pub frwatc_soil_water_after_m: f64,
    pub frwatc_frozen_water_before_m: f64,
    pub frwatc_frozen_water_after_m: f64,
    pub frwatc_freeze_debit_m: f64,
    pub frwatc_thaw_credit_m: f64,
    pub frwatc_net_liquid_delta_m: f64,
    pub infcap_frz_m_s: f64,
    pub frdp_m: f64,
    pub thdp_m: f64,
    pub tfrdp_m: f64,
    pub tthawd_m: f64,
    pub fgthwd_flag: f64,
    pub total_fine_layer_count: f64,
    pub kftill_w_m_k: f64,
    pub kfutil_w_m_k: f64,
    pub kres_w_m_k: f64,
    pub residue_depth_m: f64,
}

/// Project typed snow controls from parsed snow input.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when controls violate CLIM05 domains.
pub fn project_typed_snow_runtime(
    snow: &SnowParseOutput,
) -> Result<TypedSnowRuntimeProjection, HillslopeRuntimeInputError> {
    let rst = validate_snow_control_finite("snow.options.rst", snow.rst)?;
    let newsnw = validate_snow_control_finite("snow.options.newsnw", snow.newsnw)?;
    let ssd = validate_snow_control_finite("snow.options.ssd", snow.ssd)?;

    if newsnw <= 0.0 {
        return Err(HillslopeRuntimeInputError::SnowControlOutOfDomain {
            field: "snow.options.newsnw",
            value: newsnw,
            allowed: "> 0.0",
        });
    }
    if ssd <= 0.0 {
        return Err(HillslopeRuntimeInputError::SnowControlOutOfDomain {
            field: "snow.options.ssd",
            value: ssd,
            allowed: "> 0.0",
        });
    }
    if newsnw > ssd {
        return Err(HillslopeRuntimeInputError::SnowControlOutOfDomain {
            field: "snow.options.newsnw",
            value: newsnw,
            allowed: "<= snow.options.ssd",
        });
    }

    Ok(TypedSnowRuntimeProjection {
        rst_c: rst,
        newsnw_kg_m3: newsnw,
        ssd_kg_m3: ssd,
        snow_file_present: snow.sidecar_present,
        runtime_swe_m: 0.0,
        runtime_depth_m: 0.0,
        runtime_density_kg_m3: 0.0,
        runtime_settle_day_count: 0.0,
    })
}

/// Project typed frost controls from parsed frost input.
///
/// # Errors
///
/// Returns `HillslopeRuntimeInputError` when controls violate CLIM06 domains.
pub fn project_typed_frost_runtime(
    frost: &FrostParseOutput,
) -> Result<TypedFrostRuntimeProjection, HillslopeRuntimeInputError> {
    let wint_red = f64::from(frost.wint_red);
    let fine_top = f64::from(frost.fine_top);
    let fine_bot = f64::from(frost.fine_bot);
    let ksnowf = validate_frost_control_finite("frost.options.ksnowf", frost.ksnowf)?;
    let kresf = validate_frost_control_finite("frost.options.kresf", frost.kresf)?;
    let ksoilf = validate_frost_control_finite("frost.options.ksoilf", frost.ksoilf)?;
    let kfactor1 = validate_frost_control_finite("frost.options.kfactor1", frost.kfactor1)?;
    let kfactor2 = validate_frost_control_finite("frost.options.kfactor2", frost.kfactor2)?;
    let kfactor3 = validate_frost_control_finite("frost.options.kfactor3", frost.kfactor3)?;

    if frost.wint_red != 0 && frost.wint_red != 1 {
        return Err(HillslopeRuntimeInputError::FrostControlOutOfDomain {
            field: "frost.options.wintRed",
            value: wint_red,
            allowed: "{0,1}",
        });
    }
    if !(1..=10).contains(&frost.fine_top) {
        return Err(HillslopeRuntimeInputError::FrostControlOutOfDomain {
            field: "frost.options.fineTop",
            value: fine_top,
            allowed: "integer [1,10]",
        });
    }
    if !(1..=10).contains(&frost.fine_bot) {
        return Err(HillslopeRuntimeInputError::FrostControlOutOfDomain {
            field: "frost.options.fineBot",
            value: fine_bot,
            allowed: "integer [1,10]",
        });
    }
    for (field, value) in [
        ("frost.options.ksnowf", ksnowf),
        ("frost.options.kresf", kresf),
        ("frost.options.ksoilf", ksoilf),
    ] {
        if !(0.1..=10.0).contains(&value) {
            return Err(HillslopeRuntimeInputError::FrostControlOutOfDomain {
                field,
                value,
                allowed: "real [0.1,10.0]",
            });
        }
    }
    for (field, value) in [
        ("frost.options.kfactor1", kfactor1),
        ("frost.options.kfactor2", kfactor2),
        ("frost.options.kfactor3", kfactor3),
    ] {
        if !(value > 0.0 && value <= 1.0) {
            return Err(HillslopeRuntimeInputError::FrostControlOutOfDomain {
                field,
                value,
                allowed: "real (0.0,1.0]",
            });
        }
    }

    Ok(TypedFrostRuntimeProjection {
        wint_red: frost.wint_red == 1,
        fine_top: frost.fine_top,
        fine_bot: frost.fine_bot,
        ksnowf,
        kresf,
        ksoilf,
        kfactor1,
        kfactor2,
        kfactor3,
        frost_file_present: frost.frost_file_present,
        dfrost_m: 0.0,
        dthaw_m: 0.0,
        nft: 0.0,
        ws_frz_m: 0.0,
        frwatc_soil_water_before_m: 0.0,
        frwatc_soil_water_after_m: 0.0,
        frwatc_frozen_water_before_m: 0.0,
        frwatc_frozen_water_after_m: 0.0,
        frwatc_freeze_debit_m: 0.0,
        frwatc_thaw_credit_m: 0.0,
        frwatc_net_liquid_delta_m: 0.0,
        infcap_frz_m_s: 0.0,
        frdp_m: 0.0,
        thdp_m: 0.0,
        tfrdp_m: 0.0,
        tthawd_m: 0.0,
        fgthwd_flag: 0.0,
        total_fine_layer_count: 0.0,
        kftill_w_m_k: 1.75,
        kfutil_w_m_k: 2.1,
        kres_w_m_k: 0.05 * kresf,
        residue_depth_m: 0.0,
    })
}
