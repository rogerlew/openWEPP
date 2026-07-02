//! MOFEFID Lane D / D6 (SC-OFEROUTE-001, INV-OFEROUTE-009): Green-Ampt-
//! Mein-Larsen per-OFE infiltration, coupling rainfall to the rainfall-excess
//! the cascade routes (Papanicolaou et al. 2018, assumption 2). Pure kernel +
//! a rainfall -> excess hyetograph helper; shadow-first / opt-in.
//!
//! GAP-OFEROUTE-003 physics (resolved D6, faithful Papanicolaou): infiltration
//! is computed **per OFE on the RAINFALL rate** (`r`), producing a
//! rainfall-excess `v = r - i_f` that the cascade routes; the upstream OFE's
//! outlet hydrograph is added as a downstream **surface boundary condition**
//! (D5 handoff) and is NOT re-infiltrated. When this routing subsystem is
//! active it therefore **supersedes** DC01's daily-lump runon re-infiltration
//! (`SC-RUNOFFPART-001#INV-RUNOFFPART-031`) with hydraulic surface routing —
//! it does not compose a second re-infiltration of the routed runon. Runtime
//! DC01-disable is the production activation gate.

/// Green-Ampt-Mein-Larsen soil parameters for one OFE.
#[derive(Debug, Clone, Copy)]
pub struct GreenAmptSoil {
    /// Saturated hydraulic conductivity `Ks` (m/s).
    pub saturated_conductivity_m_s: f64,
    /// Wetting-front suction head `psi` (m).
    pub suction_head_m: f64,
    /// Moisture deficit `delta_theta` = (saturated - initial) volumetric water
    /// content (dimensionless, 0-1).
    pub moisture_deficit: f64,
}

impl GreenAmptSoil {
    /// An impermeable surface (Case 4): `Ks = 0`, all rainfall becomes excess.
    #[must_use]
    pub fn impermeable() -> Self {
        Self {
            saturated_conductivity_m_s: 0.0,
            suction_head_m: 0.0,
            moisture_deficit: 0.0,
        }
    }

    /// `psi * delta_theta` (m), the storage-suction product used by Green-Ampt.
    #[must_use]
    fn suction_storage_m(&self) -> f64 {
        self.suction_head_m * self.moisture_deficit
    }

    /// Validate the parameter domain (finite; non-negative; deficit in [0,1]).
    #[must_use]
    pub fn is_valid(&self) -> bool {
        [
            self.saturated_conductivity_m_s,
            self.suction_head_m,
            self.moisture_deficit,
        ]
        .iter()
        .all(|v| v.is_finite() && *v >= 0.0)
            && self.moisture_deficit <= 1.0
    }
}

/// Cumulative infiltration state (m) for one point, carried across steps.
#[derive(Debug, Clone, Copy, Default)]
pub struct InfiltrationState {
    /// Cumulative infiltrated depth `F` (m).
    pub cumulative_m: f64,
}

/// Result of one infiltration step.
#[derive(Debug, Clone, Copy)]
pub struct InfiltrationStep {
    /// Infiltrated depth this step (m).
    pub infiltrated_m: f64,
    /// Rainfall-excess depth this step (m) = `rainfall - infiltrated`.
    pub excess_m: f64,
    /// Updated cumulative infiltration `F` (m).
    pub cumulative_m: f64,
}

/// Green-Ampt infiltration capacity `fc(F) = Ks (1 + psi*dtheta / F)` (m/s).
/// Diverges as `F -> 0` (infinite initial capacity); returns `f64::INFINITY`.
#[must_use]
pub fn infiltration_capacity_m_s(soil: &GreenAmptSoil, cumulative_m: f64) -> f64 {
    if soil.saturated_conductivity_m_s <= 0.0 {
        return 0.0; // impermeable
    }
    if cumulative_m <= 0.0 {
        return f64::INFINITY;
    }
    soil.saturated_conductivity_m_s * (1.0 + soil.suction_storage_m() / cumulative_m)
}

/// Advance Green-Ampt-Mein-Larsen one step of duration `dt` (s) under a
/// constant rainfall rate `rainfall_rate_m_s` over the step. Handles the
/// unponded -> ponded transition and integrates the implicit Green-Ampt
/// relation by Newton iteration once ponded. Returns infiltration + excess.
#[must_use]
pub fn green_ampt_step(
    soil: &GreenAmptSoil,
    state: InfiltrationState,
    rainfall_rate_m_s: f64,
    dt_s: f64,
) -> InfiltrationStep {
    let f0 = state.cumulative_m.max(0.0);
    if rainfall_rate_m_s <= 0.0 || dt_s <= 0.0 {
        return InfiltrationStep {
            infiltrated_m: 0.0,
            excess_m: 0.0,
            cumulative_m: f0,
        };
    }
    let rain_depth = rainfall_rate_m_s * dt_s;
    if soil.saturated_conductivity_m_s <= 0.0 {
        // impermeable: all rainfall is excess
        return InfiltrationStep {
            infiltrated_m: 0.0,
            excess_m: rain_depth,
            cumulative_m: f0,
        };
    }
    let ks = soil.saturated_conductivity_m_s;
    let s = soil.suction_storage_m();

    // Capacity if all rainfall infiltrated this step (test for ponding).
    let f_all = f0 + rain_depth;
    let cap_at_f_all = infiltration_capacity_m_s(soil, f_all);
    if rainfall_rate_m_s <= cap_at_f_all {
        // rainfall rate never exceeds capacity over the step: no ponding,
        // all rainfall infiltrates.
        return InfiltrationStep {
            infiltrated_m: rain_depth,
            excess_m: 0.0,
            cumulative_m: f_all,
        };
    }

    // Ponded (rainfall exceeds capacity). Integrate the implicit Green-Ampt
    // relation over the step: F - F0 - s*ln((F+s)/(F0+s)) = Ks*dt.
    // (When F0 = 0 the suction term uses a small floor to avoid ln(0).)
    let f0_eff = if f0 <= 0.0 { 1.0e-9 } else { f0 };
    let target = ks * dt_s;
    let mut f = f0_eff + ks * dt_s; // initial guess
    for _ in 0..50 {
        let g = (f - f0_eff) - s * ((f + s) / (f0_eff + s)).ln() - target;
        let dg = 1.0 - s / (f + s);
        if dg.abs() < 1.0e-15 {
            break;
        }
        let step = g / dg;
        f -= step;
        if f <= f0_eff {
            f = f0_eff;
        }
        if step.abs() <= 1.0e-14 {
            break;
        }
    }
    // Infiltration cannot exceed the available rainfall over the step.
    let infiltrated = (f - f0).clamp(0.0, rain_depth);
    let excess = (rain_depth - infiltrated).max(0.0);
    InfiltrationStep {
        infiltrated_m: infiltrated,
        excess_m: excess,
        cumulative_m: f0 + infiltrated,
    }
}

/// A discretized rainfall hyetograph interval.
#[derive(Debug, Clone, Copy)]
pub struct RainfallInterval {
    pub start_s: f64,
    pub end_s: f64,
    pub rate_m_s: f64,
}

/// Rainfall-excess hyetograph produced by Green-Ampt on a rainfall series.
#[derive(Debug, Clone)]
pub struct ExcessHyetograph {
    /// Excess-rate intervals (m/s).
    pub intervals: Vec<RainfallInterval>,
    /// Total rainfall depth (m).
    pub rainfall_depth_m: f64,
    /// Total infiltrated depth (m).
    pub infiltrated_depth_m: f64,
    /// Total excess depth (m).
    pub excess_depth_m: f64,
}

impl ExcessHyetograph {
    /// Event runoff coefficient (excess / rainfall).
    #[must_use]
    pub fn runoff_coefficient(&self) -> f64 {
        if self.rainfall_depth_m <= 0.0 {
            return 0.0;
        }
        self.excess_depth_m / self.rainfall_depth_m
    }

    /// Excess rate (m/s) at `time_s`, for use as a routing forcing.
    #[must_use]
    pub fn excess_rate_at(&self, time_s: f64) -> f64 {
        for interval in &self.intervals {
            if time_s >= interval.start_s && time_s < interval.end_s {
                return interval.rate_m_s;
            }
        }
        0.0
    }
}

/// Apply Green-Ampt-Mein-Larsen to a rainfall series, sub-stepping each
/// interval at `substep_s`, to produce the rainfall-excess hyetograph one OFE
/// routes (Papanicolaou assumption 2: infiltration on rainfall, per OFE).
#[must_use]
pub fn green_ampt_excess_hyetograph(
    soil: &GreenAmptSoil,
    rainfall: &[RainfallInterval],
    substep_s: f64,
) -> ExcessHyetograph {
    let mut state = InfiltrationState::default();
    let mut intervals = Vec::new();
    let mut rainfall_depth = 0.0;
    let mut infiltrated_depth = 0.0;
    let mut excess_depth = 0.0;
    let substep_s = if substep_s > 0.0 { substep_s } else { 1.0 };

    for interval in rainfall {
        let duration = interval.end_s - interval.start_s;
        if duration <= 0.0 || interval.rate_m_s < 0.0 {
            continue;
        }
        // March a fixed sub-step across the interval, taking a partial final
        // step to land exactly on `end_s` (avoids an f64 -> integer count).
        let mut start = interval.start_s;
        while start < interval.end_s {
            let dt = substep_s.min(interval.end_s - start);
            let end = start + dt;
            let step = green_ampt_step(soil, state, interval.rate_m_s, dt);
            state.cumulative_m = step.cumulative_m;
            rainfall_depth += interval.rate_m_s * dt;
            infiltrated_depth += step.infiltrated_m;
            excess_depth += step.excess_m;
            intervals.push(RainfallInterval {
                start_s: start,
                end_s: end,
                rate_m_s: if dt > 0.0 { step.excess_m / dt } else { 0.0 },
            });
            start = end;
        }
    }

    ExcessHyetograph {
        intervals,
        rainfall_depth_m: rainfall_depth,
        infiltrated_depth_m: infiltrated_depth,
        excess_depth_m: excess_depth,
    }
}

/// Combined rainfall-to-runoff result: per-OFE Green-Ampt infiltration
/// composed with the D5 cascade routing (`GAP-OFEROUTE-003` supersede model).
#[derive(Debug, Clone)]
pub struct InfiltratedCascadeResult {
    /// The routed cascade result (outlet hydrograph, cascade mass balance, ...).
    pub cascade: super::cascade::CascadeResult,
    /// Per-OFE runoff coefficient (excess / rainfall) from Green-Ampt.
    pub per_ofe_runoff_coefficient: Vec<f64>,
    /// Total rainfall depth summed over all OFEs x widths (m^3).
    pub rainfall_m3: f64,
    /// Total infiltrated depth summed over all OFEs x widths (m^3).
    pub infiltrated_m3: f64,
}

impl InfiltratedCascadeResult {
    /// End-to-end rainfall-balance residual (m^3): rainfall = infiltrated +
    /// cascade outlet + cascade storage - clamp. Should be ~0.
    #[must_use]
    pub fn rainfall_balance_residual_m3(&self) -> f64 {
        self.rainfall_m3
            - self.infiltrated_m3
            - self.cascade.mass_balance.outlet_m3
            - self.cascade.mass_balance.storage_change_m3
            + self.cascade.mass_balance.positivity_clamp_m3
    }
}

/// Full rainfall-to-runoff over an OFE cascade (Papanicolaou assumption 2):
/// each OFE's Green-Ampt infiltration converts its RAINFALL to rainfall-excess,
/// which the cascade routes with the upstream OFE's hydrograph as a surface
/// boundary (D5 handoff). Runon is NOT re-infiltrated (supersedes DC01's
/// daily-lump admission with hydraulic routing).
///
/// `rainfall` and `soils` are indexed by OFE (same length as `segments`).
pub fn run_infiltrated_cascade(
    segments: &[super::cascade::CascadeSegment],
    rainfall: &[Vec<RainfallInterval>],
    soils: &[GreenAmptSoil],
    infiltration_substep_s: f64,
    end_time_s: f64,
    sample_dt_s: f64,
    max_dt_s: f64,
) -> Result<InfiltratedCascadeResult, super::kinematic_wave::RoutingError> {
    use super::kinematic_wave::RoutingError;
    if segments.is_empty() || rainfall.len() != segments.len() || soils.len() != segments.len() {
        return Err(RoutingError::DegenerateConfiguration);
    }
    for soil in soils {
        if !soil.is_valid() {
            return Err(RoutingError::InvalidCellParameter);
        }
    }

    // Per-OFE Green-Ampt rainfall -> excess hyetographs.
    let hyetos: Vec<ExcessHyetograph> = soils
        .iter()
        .zip(rainfall.iter())
        .map(|(soil, rain)| green_ampt_excess_hyetograph(soil, rain, infiltration_substep_s))
        .collect();

    // Cascade forcing: excess routes; the skin-resistance term uses the
    // RAINFALL intensity (Papanicolaou eq. 2 depends on I, not excess).
    let excess = |ofe: usize, _cell: usize, t: f64| hyetos[ofe].excess_rate_at(t);
    let intensity = |ofe: usize, t: f64| {
        rainfall[ofe]
            .iter()
            .find(|iv| t >= iv.start_s && t < iv.end_s)
            .map_or(0.0, |iv| iv.rate_m_s)
    };
    let forcing = super::cascade::CascadeForcing {
        rainfall_excess_m_s: &excess,
        rainfall_intensity_m_s: &intensity,
    };
    let cascade =
        super::cascade::run_cascade(segments, &forcing, end_time_s, sample_dt_s, max_dt_s)?;

    let per_ofe_rc: Vec<f64> = hyetos
        .iter()
        .map(ExcessHyetograph::runoff_coefficient)
        .collect();
    let rainfall_m3: f64 = hyetos
        .iter()
        .zip(segments.iter())
        .map(|(h, seg)| h.rainfall_depth_m * seg.width_m * seg_length(seg))
        .sum();
    let infiltrated_m3: f64 = hyetos
        .iter()
        .zip(segments.iter())
        .map(|(h, seg)| h.infiltrated_depth_m * seg.width_m * seg_length(seg))
        .sum();

    Ok(InfiltratedCascadeResult {
        cascade,
        per_ofe_runoff_coefficient: per_ofe_rc,
        rainfall_m3,
        infiltrated_m3,
    })
}

/// OFE length (m) = cell length x cell count.
fn seg_length(seg: &super::cascade::CascadeSegment) -> f64 {
    seg.mesh.cell_length_m * f64::from(u32::try_from(seg.mesh.cells.len()).unwrap_or(u32::MAX))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() <= tol
    }

    #[test]
    fn capacity_diverges_at_dry_and_decays_with_cumulative() {
        let soil = GreenAmptSoil {
            saturated_conductivity_m_s: 1.0e-5,
            suction_head_m: 0.1,
            moisture_deficit: 0.3,
        };
        assert!(soil.is_valid());
        assert!(infiltration_capacity_m_s(&soil, 0.0).is_infinite());
        let c1 = infiltration_capacity_m_s(&soil, 0.001);
        let c2 = infiltration_capacity_m_s(&soil, 0.01);
        assert!(c1 > c2, "capacity decays as F grows: {c1} vs {c2}");
        // asymptotes to Ks as F -> infinity
        let c_big = infiltration_capacity_m_s(&soil, 100.0);
        assert!(approx(c_big, 1.0e-5, 1.0e-7));
    }

    #[test]
    fn low_rainfall_below_ks_never_ponds_all_infiltrates() {
        let soil = GreenAmptSoil {
            saturated_conductivity_m_s: 1.0e-5,
            suction_head_m: 0.1,
            moisture_deficit: 0.3,
        };
        // rainfall below Ks: no excess ever
        let r = 0.5e-5;
        let mut state = InfiltrationState::default();
        let mut total_excess = 0.0;
        for _ in 0..1000 {
            let s = green_ampt_step(&soil, state, r, 1.0);
            state.cumulative_m = s.cumulative_m;
            total_excess += s.excess_m;
        }
        assert!(total_excess < 1.0e-12, "no excess below Ks: {total_excess}");
    }

    #[test]
    fn high_rainfall_ponds_and_produces_excess_with_decaying_infiltration() {
        // Case 1-like: 60 mm/h rainfall on a moderately permeable soil.
        let soil = GreenAmptSoil {
            saturated_conductivity_m_s: 2.0e-6, // ~7.2 mm/h
            suction_head_m: 0.1,
            moisture_deficit: 0.3,
        };
        let r = 60.0 / 3.6e6; // m/s
        let rainfall = vec![RainfallInterval {
            start_s: 0.0,
            end_s: 5.0 * 3600.0,
            rate_m_s: r,
        }];
        let hyeto = green_ampt_excess_hyetograph(&soil, &rainfall, 10.0);
        // some rainfall infiltrates, some runs off
        assert!(hyeto.infiltrated_depth_m > 0.0);
        assert!(hyeto.excess_depth_m > 0.0);
        // conservation: rainfall = infiltrated + excess
        assert!(
            approx(
                hyeto.rainfall_depth_m,
                hyeto.infiltrated_depth_m + hyeto.excess_depth_m,
                1.0e-9
            ),
            "rainfall must split into infiltration + excess"
        );
        // runoff coefficient in (0,1); high rain over low-Ks soil -> most runs off
        let rc = hyeto.runoff_coefficient();
        assert!(rc > 0.5 && rc < 1.0, "runoff coefficient {rc}");
        // early intervals infiltrate fully (pre-ponding excess ~ 0); later
        // intervals produce excess as capacity decays.
        assert!(hyeto.intervals[0].rate_m_s < hyeto.intervals[hyeto.intervals.len() - 1].rate_m_s);
    }

    #[test]
    fn impermeable_soil_makes_all_rainfall_excess() {
        let soil = GreenAmptSoil::impermeable();
        let r = 74.0 / 3.6e6;
        let rainfall = vec![RainfallInterval {
            start_s: 0.0,
            end_s: 3600.0,
            rate_m_s: r,
        }];
        let hyeto = green_ampt_excess_hyetograph(&soil, &rainfall, 10.0);
        assert!(approx(hyeto.runoff_coefficient(), 1.0, 1.0e-12));
        assert!(approx(hyeto.infiltrated_depth_m, 0.0, 1.0e-15));
    }

    #[test]
    fn conservation_holds_across_variable_rainfall() {
        let soil = GreenAmptSoil {
            saturated_conductivity_m_s: 5.0e-6,
            suction_head_m: 0.08,
            moisture_deficit: 0.25,
        };
        let rainfall = vec![
            RainfallInterval {
                start_s: 0.0,
                end_s: 600.0,
                rate_m_s: 40.0 / 3.6e6,
            },
            RainfallInterval {
                start_s: 600.0,
                end_s: 1200.0,
                rate_m_s: 90.0 / 3.6e6,
            },
            RainfallInterval {
                start_s: 1200.0,
                end_s: 1800.0,
                rate_m_s: 20.0 / 3.6e6,
            },
        ];
        let hyeto = green_ampt_excess_hyetograph(&soil, &rainfall, 5.0);
        assert!(approx(
            hyeto.rainfall_depth_m,
            hyeto.infiltrated_depth_m + hyeto.excess_depth_m,
            1.0e-9
        ));
        // the intense middle burst produces the most excess
        assert!(hyeto.excess_depth_m > 0.0 && hyeto.excess_depth_m < hyeto.rainfall_depth_m);
    }
    #[test]
    fn coupled_case1_rainfall_to_runoff_conserves_and_produces_runoff() {
        use super::super::cascade::CascadeSegment;
        use super::super::kinematic_wave::{CellParameters, KinematicWaveMesh};
        // Case 1-like: 60 mm/h for 1 h on a bare, moderately permeable soil.
        let r = 60.0 / 3.6e6;
        let rainfall = vec![vec![RainfallInterval {
            start_s: 0.0,
            end_s: 3600.0,
            rate_m_s: r,
        }]];
        let soils = vec![GreenAmptSoil {
            saturated_conductivity_m_s: 2.0e-6,
            suction_head_m: 0.1,
            moisture_deficit: 0.3,
        }];
        let segments = vec![CascadeSegment {
            mesh: KinematicWaveMesh::uniform(7.5, 20, CellParameters::bare(0.09, 500.0)),
            width_m: 1.2,
        }];
        let res = run_infiltrated_cascade(&segments, &rainfall, &soils, 10.0, 5400.0, 10.0, 2.0)
            .expect("coupled run");

        // infiltration reduced runoff: 0 < runoff coefficient < 1
        let rc = res.per_ofe_runoff_coefficient[0];
        assert!(rc > 0.0 && rc < 1.0, "runoff coefficient {rc}");
        // end-to-end rainfall balance: rainfall = infiltration + outlet + storage
        assert!(res.rainfall_m3 > 0.0);
        assert!(
            res.rainfall_balance_residual_m3().abs() / res.rainfall_m3 < 1.0e-2,
            "rainfall balance residual {} vs rainfall {}",
            res.rainfall_balance_residual_m3(),
            res.rainfall_m3
        );
        assert!(res.cascade.max_courant <= 1.0 + 1.0e-9);
        // the routed outlet carries the excess (nonzero peak)
        assert!(res.cascade.peak_total_discharge_m3_s > 0.0);
    }

    #[test]
    fn coupled_impermeable_routes_all_rainfall() {
        use super::super::cascade::CascadeSegment;
        use super::super::kinematic_wave::{CellParameters, KinematicWaveMesh};
        let r = 74.0 / 3.6e6;
        let rainfall = vec![vec![RainfallInterval {
            start_s: 0.0,
            end_s: 1800.0,
            rate_m_s: r,
        }]];
        let soils = vec![GreenAmptSoil::impermeable()];
        let segments = vec![CascadeSegment {
            mesh: KinematicWaveMesh::uniform(10.0, 20, CellParameters::bare(0.05, 500.0)),
            width_m: 1.0,
        }];
        let res = run_infiltrated_cascade(&segments, &rainfall, &soils, 10.0, 3600.0, 10.0, 2.0)
            .expect("coupled run");
        // impermeable: all rainfall runs off (coefficient 1)
        assert!(approx(res.per_ofe_runoff_coefficient[0], 1.0, 1.0e-12));
        assert!(approx(res.infiltrated_m3, 0.0, 1.0e-12));
        assert!(res.rainfall_balance_residual_m3().abs() / res.rainfall_m3 < 1.0e-2);
    }

    #[test]
    fn coupled_two_ofe_infiltration_varies_runoff_downslope() {
        use super::super::cascade::CascadeSegment;
        use super::super::kinematic_wave::{CellParameters, KinematicWaveMesh};
        let r = 74.0 / 3.6e6;
        let rain_series = || {
            vec![RainfallInterval {
                start_s: 0.0,
                end_s: 3600.0,
                rate_m_s: r,
            }]
        };
        let rainfall = vec![rain_series(), rain_series()];
        // upslope permeable, downslope less permeable -> higher downslope RC
        let soils = vec![
            GreenAmptSoil {
                saturated_conductivity_m_s: 5.0e-6,
                suction_head_m: 0.1,
                moisture_deficit: 0.3,
            },
            GreenAmptSoil {
                saturated_conductivity_m_s: 1.0e-6,
                suction_head_m: 0.1,
                moisture_deficit: 0.3,
            },
        ];
        let seg = || CascadeSegment {
            mesh: KinematicWaveMesh::uniform(10.0, 20, CellParameters::bare(0.07, 500.0)),
            width_m: 2.0,
        };
        let segments = vec![seg(), seg()];
        let res = run_infiltrated_cascade(&segments, &rainfall, &soils, 10.0, 5400.0, 10.0, 2.0)
            .expect("coupled run");
        // less permeable downslope OFE has the higher runoff coefficient
        assert!(
            res.per_ofe_runoff_coefficient[1] > res.per_ofe_runoff_coefficient[0],
            "downslope (lower Ks) must run off more: {:?}",
            res.per_ofe_runoff_coefficient
        );
        assert!(res.rainfall_balance_residual_m3().abs() / res.rainfall_m3 < 1.0e-2);
        assert!(res.cascade.max_courant <= 1.0 + 1.0e-9);
    }

    #[test]
    fn coupled_run_validates_config_and_soil_domains() {
        use super::super::cascade::CascadeSegment;
        use super::super::kinematic_wave::{CellParameters, KinematicWaveMesh, RoutingError};
        let segments = vec![CascadeSegment {
            mesh: KinematicWaveMesh::uniform(10.0, 10, CellParameters::bare(0.05, 100.0)),
            width_m: 1.0,
        }];
        let rainfall = vec![vec![RainfallInterval {
            start_s: 0.0,
            end_s: 100.0,
            rate_m_s: 1.0e-5,
        }]];
        // mismatched soils length
        assert!(matches!(
            run_infiltrated_cascade(&segments, &rainfall, &[], 10.0, 100.0, 10.0, 2.0),
            Err(RoutingError::DegenerateConfiguration)
        ));
        // invalid soil (negative Ks)
        let bad_soil = vec![GreenAmptSoil {
            saturated_conductivity_m_s: -1.0,
            suction_head_m: 0.1,
            moisture_deficit: 0.3,
        }];
        assert!(matches!(
            run_infiltrated_cascade(&segments, &rainfall, &bad_soil, 10.0, 100.0, 10.0, 2.0),
            Err(RoutingError::InvalidCellParameter)
        ));
    }
}
