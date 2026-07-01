/// Build a hillslope-owned climate runtime request from parser output
/// (`HS-CLIM-SEAM-001`).
///
/// Runtime policy at this seam enforces `datver=0.0` override (`iclig=0`) or
/// `datver>=4.0` (`iclig=1`).
///
/// # Errors
///
/// Returns `ClimateRuntimeInputError` when climate parser output violates
/// runtime seam policy or numeric invariants.
pub fn build_hillslope_climate_runtime_request(
    climate: &ClimateFile,
) -> Result<HillslopeClimateRuntimeRequest, ClimateRuntimeInputError> {
    let shared = build_climate_runtime_request(climate)?;
    let mut day_symbol_surfaces = Vec::with_capacity(shared.daily_forcing.len());
    for forcing in &shared.daily_forcing {
        day_symbol_surfaces.push(build_hillslope_series_surface(forcing)?);
    }

    Ok(HillslopeClimateRuntimeRequest {
        shared,
        metadata: climate.metadata.clone(),
        monthly: climate.monthly.clone(),
        day_symbol_surfaces,
    })
}

#[derive(Debug, Clone, PartialEq)]
pub struct HillslopeDirectClimateDayForcing {
    pub prcp_m: f64,
    pub tmax_c: f64,
    pub tmin_c: f64,
    pub rad_ly: f64,
    pub vwind_m_s: f64,
    pub wind_deg: f64,
    pub tdpt_c: f64,
    pub timem_s: Vec<f64>,
    pub intsty_m_s: Vec<f64>,
}

impl HillslopeClimateRuntimeRequest {
    /// Borrow typed forcing for direct-runtime production inputs without
    /// materializing climate symbols onto a writeback surface.
    ///
    /// # Errors
    ///
    /// Returns `ClimateRuntimeInputError` when `day_index` exceeds the
    /// precomputed climate forcing span.
    pub fn direct_day_forcing(
        &self,
        day_index: usize,
    ) -> Result<HillslopeDirectClimateDayForcing, ClimateRuntimeInputError> {
        match select_day_forcing(&self.shared, day_index)? {
            HillslopeClimateDailyForcing::NoBreakpoint(day) => {
                Ok(HillslopeDirectClimateDayForcing {
                    prcp_m: day.prcp,
                    tmax_c: day.tmax,
                    tmin_c: day.tmin,
                    rad_ly: day.rad,
                    vwind_m_s: day.vwind,
                    wind_deg: day.wind,
                    tdpt_c: day.tdpt,
                    timem_s: day.timem.clone(),
                    intsty_m_s: day.intsty.clone(),
                })
            }
            HillslopeClimateDailyForcing::Breakpoint(day) => {
                Ok(HillslopeDirectClimateDayForcing {
                    prcp_m: day.prcp,
                    tmax_c: day.tmax,
                    tmin_c: day.tmin,
                    rad_ly: day.rad,
                    vwind_m_s: day.vwind,
                    wind_deg: day.wind,
                    tdpt_c: day.tdpt,
                    timem_s: day.timem.clone(),
                    intsty_m_s: day.intsty.clone(),
                })
            }
        }
    }

    #[must_use]
    pub const fn direct_latitude_degrees(&self) -> f64 {
        self.metadata.deglat
    }

    #[must_use]
    pub const fn direct_elevation_m(&self) -> f64 {
        self.metadata.elev
    }

    #[must_use]
    pub const fn direct_monthly_max_c(&self) -> [f64; 12] {
        self.monthly.obmaxt
    }

    #[must_use]
    pub const fn direct_monthly_min_c(&self) -> [f64; 12] {
        self.monthly.obmint
    }
}
