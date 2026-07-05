//! GAP-OFEROUTE-006 subsurface-coupling seam (`SC-OFEROUTE-001` rev 4,
//! `INV-OFEROUTE-012`): the routed source-rate series from the two
//! contract-governed hourly carries, the hourly-lane activation
//! precondition, the DC01 mutual-exclusion rule, and the activation
//! closure identity. Pure/solver tier — runtime wiring is the
//! activation increment; the production-activation BLOCK stands until
//! it lands.

/// Seam-boundary failures (fail-closed, `INV-OFEROUTE-012`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeamError {
    /// A seam operand was non-finite or out of domain.
    InvalidOperand { what: &'static str },
    /// Activation attempted without the hourly-lane carries.
    HourlyLaneRequired,
    /// Active routing and DC01 daily-lump runon on the same lane.
    Dc01DoubleCount,
}

/// Hour-bin count shared with the runtime carries.
pub const SEAM_HOUR_BINS: usize = 24;
/// The seam's one recorded unit helper (`SC-OFEROUTE-001` rev 4 unit
/// map): depth per hour slot (m) → rate (m s⁻¹).
pub const SEAM_SECONDS_PER_HOUR: f64 = 3600.0;

/// The two hourly depth carries the seam consumes (GAP-006 D1): the
/// rainfall-excess depth series and the `ui_SCrunf`-lineage saturation
/// carry (`hourly_saturation_carry_m` in the direct runtime — the SAME
/// limb the DC01 weights unify).
#[derive(Debug, Clone, Copy)]
pub struct SeamHourlySources<'a> {
    pub wb14_hourly_excess_m: &'a [f64; SEAM_HOUR_BINS],
    pub saturation_carry_m: &'a [f64; SEAM_HOUR_BINS],
}

/// Build the routed lateral source-rate series (m s⁻¹ per hour bin):
/// `s_h = (excess_h + scrunf_h) / 3600 s`. Fail-closed on non-finite or
/// negative depths (the carries are nonnegative by their producers'
/// contracts; a violation here is upstream corruption, never clamped).
pub fn seam_source_rate_series(
    sources: SeamHourlySources<'_>,
) -> Result<[f64; SEAM_HOUR_BINS], SeamError> {
    let mut rates = [0.0_f64; SEAM_HOUR_BINS];
    for (hour, rate) in rates.iter_mut().enumerate() {
        let excess = sources.wb14_hourly_excess_m[hour];
        let carry = sources.saturation_carry_m[hour];
        if !excess.is_finite() || excess < 0.0 {
            return Err(SeamError::InvalidOperand {
                what: "seam wb14_hourly_excess_m",
            });
        }
        if !carry.is_finite() || carry < 0.0 {
            return Err(SeamError::InvalidOperand {
                what: "seam saturation_carry_m (ui_SCrunf lineage)",
            });
        }
        *rate = (excess + carry) / SEAM_SECONDS_PER_HOUR;
    }
    Ok(rates)
}

/// Hourly-lane activation precondition (`INV-OFEROUTE-012`): the seam
/// consumes the `INV-SUBHYD-023` hourly carries; a daily-lane hillslope
/// does not publish them and MUST fail closed at activation — never a
/// silent fallback to daily lumps.
pub fn seam_require_hourly_lane(hourly_carries_present: bool) -> Result<(), SeamError> {
    if hourly_carries_present {
        Ok(())
    } else {
        Err(SeamError::HourlyLaneRequired)
    }
}

/// DC01 mutual exclusion (`INV-OFEROUTE-009` activation guard): active
/// routing OWNS the hourly surface runon; DC01's daily-lump runon
/// re-infiltration must be DISABLED on the same lane — both paths
/// feeding one lane is a double-count and hard-fails.
pub fn seam_assert_dc01_superseded(
    routing_active: bool,
    dc01_runon_active: bool,
) -> Result<(), SeamError> {
    if routing_active && dc01_runon_active {
        return Err(SeamError::Dc01DoubleCount);
    }
    Ok(())
}

/// Sample the source-rate series as a `(time_s) -> m/s` lookup for the
/// cascade forcing adapter (uniform within each hour bin; zero outside
/// the day window — the routing window is one day).
#[must_use]
pub fn seam_rate_at(rates: &[f64; SEAM_HOUR_BINS], time_s: f64) -> f64 {
    #[allow(clippy::cast_precision_loss)]
    let window_s = SEAM_HOUR_BINS as f64 * SEAM_SECONDS_PER_HOUR;
    if !(0.0..window_s).contains(&time_s) {
        return 0.0;
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let bin = (time_s / SEAM_SECONDS_PER_HOUR) as usize;
    rates[bin.min(SEAM_HOUR_BINS - 1)]
}

/// Operands for the activation closure identity (`INV-OFEROUTE-012`
/// (c), GAP-006 D4), all in m³ over the accounting window:
/// `P + Q_surface_in = Q_surface_out + latqcc_outlet + ET + ΔS + deep_perc`.
/// `latqcc_outlet` is the D3 bypass term — the router neither consumes
/// nor converts it; it appears here ONLY so conservation is auditable.
#[derive(Debug, Clone, Copy)]
pub struct SeamClosureOperands {
    pub precipitation_m3: f64,
    pub surface_inflow_m3: f64,
    pub surface_outflow_m3: f64,
    pub latqcc_outlet_m3: f64,
    pub evapotranspiration_m3: f64,
    pub storage_change_m3: f64,
    pub deep_percolation_m3: f64,
}

/// The signed closure residual (m³). Callers judge it against the
/// water-balance tolerance scaled by the identity's magnitude; a
/// material residual under ACTIVE routing is a typed runtime hard fail
/// (the activation increment wires that), and both gate fixtures assert
/// it near zero.
pub fn seam_closure_residual_m3(operands: &SeamClosureOperands) -> Result<f64, SeamError> {
    for (what, value) in [
        ("closure precipitation_m3", operands.precipitation_m3),
        ("closure surface_inflow_m3", operands.surface_inflow_m3),
        ("closure surface_outflow_m3", operands.surface_outflow_m3),
        ("closure latqcc_outlet_m3", operands.latqcc_outlet_m3),
        (
            "closure evapotranspiration_m3",
            operands.evapotranspiration_m3,
        ),
        ("closure deep_percolation_m3", operands.deep_percolation_m3),
    ] {
        if !value.is_finite() || value < 0.0 {
            return Err(SeamError::InvalidOperand { what });
        }
    }
    if !operands.storage_change_m3.is_finite() {
        return Err(SeamError::InvalidOperand {
            what: "closure storage_change_m3",
        });
    }
    Ok(operands.precipitation_m3 + operands.surface_inflow_m3
        - operands.surface_outflow_m3
        - operands.latqcc_outlet_m3
        - operands.evapotranspiration_m3
        - operands.storage_change_m3
        - operands.deep_percolation_m3)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ofe_routing::cascade::{CascadeForcing, CascadeSegment, run_cascade};
    use crate::ofe_routing::kinematic_wave::{CellParameters, KinematicWaveMesh};

    #[test]
    fn source_rate_series_is_the_recorded_unit_helper() {
        let mut excess = [0.0; SEAM_HOUR_BINS];
        let mut carry = [0.0; SEAM_HOUR_BINS];
        excess[6] = 0.0036; // 3.6 mm in hour 6
        carry[6] = 0.0072; // 7.2 mm exfiltration in hour 6
        carry[7] = 0.0036; // exfiltration-only hour
        let rates = seam_source_rate_series(SeamHourlySources {
            wb14_hourly_excess_m: &excess,
            saturation_carry_m: &carry,
        })
        .expect("valid depths");
        assert!((rates[6] - (0.0036 + 0.0072) / 3600.0).abs() < 1.0e-18);
        assert!((rates[7] - 0.0036 / 3600.0).abs() < 1.0e-18);
        assert!(rates[0].abs() < 1.0e-18);
        // Fail-closed on corruption.
        let mut bad = excess;
        bad[3] = f64::NAN;
        assert!(
            seam_source_rate_series(SeamHourlySources {
                wb14_hourly_excess_m: &bad,
                saturation_carry_m: &carry,
            })
            .is_err()
        );
    }

    #[test]
    fn activation_preconditions_fail_closed() {
        assert_eq!(
            seam_require_hourly_lane(false),
            Err(SeamError::HourlyLaneRequired)
        );
        assert!(seam_require_hourly_lane(true).is_ok());
        assert_eq!(
            seam_assert_dc01_superseded(true, true),
            Err(SeamError::Dc01DoubleCount)
        );
        assert!(seam_assert_dc01_superseded(true, false).is_ok());
        assert!(seam_assert_dc01_superseded(false, true).is_ok());
    }

    /// INV-OFEROUTE-012 gate fixture A (solver tier): a 2-OFE cascade
    /// where the DOWNSLOPE OFE has hours with ZERO rainfall excess and
    /// positive `ui_SCrunf`-lineage carry — exfiltration must reach the
    /// routed toe as surface flow on those hours, and the cascade must
    /// conserve.
    #[test]
    fn gate_fixture_a_exfiltration_reaches_the_routed_toe() {
        let segs = vec![
            CascadeSegment {
                mesh: KinematicWaveMesh::uniform(10.0, 20, CellParameters::bare(0.08, 500.0)),
                width_m: 2.0,
            },
            CascadeSegment {
                mesh: KinematicWaveMesh::uniform(10.0, 20, CellParameters::bare(0.06, 500.0)),
                width_m: 2.0,
            },
        ];

        // Upslope OFE: dry all day (no excess, no carry). Downslope OFE:
        // ZERO rainfall excess; exfiltration carry only, hours 2-3.
        let zero = [0.0_f64; SEAM_HOUR_BINS];
        let mut downslope_carry = [0.0_f64; SEAM_HOUR_BINS];
        downslope_carry[2] = 0.0072;
        downslope_carry[3] = 0.0072;
        let upslope_rates = seam_source_rate_series(SeamHourlySources {
            wb14_hourly_excess_m: &zero,
            saturation_carry_m: &zero,
        })
        .expect("upslope series");
        let downslope_rates = seam_source_rate_series(SeamHourlySources {
            wb14_hourly_excess_m: &zero,
            saturation_carry_m: &downslope_carry,
        })
        .expect("downslope series");

        let per_ofe = [upslope_rates, downslope_rates];
        let excess = |ofe: usize, _cell: usize, t: f64| seam_rate_at(&per_ofe[ofe], t);
        // No rainfall anywhere (exfiltration-driven flow): the skin term's
        // intensity operand is zero.
        let intensity = |_ofe: usize, _t: f64| 0.0;
        let forcing = CascadeForcing {
            rainfall_excess_m_s: &excess,
            rainfall_intensity_m_s: &intensity,
        };
        // Route the first 5 hours (the pulse is hours 2-3).
        let res = run_cascade(&segs, &forcing, 5.0 * 3600.0, 30.0, 2.0).expect("cascade runs");

        // The routed toe carries the exfiltrated water: outlet volume is
        // materially positive and close to the injected volume minus
        // storage (cascade conservation).
        let injected_m3 = 2.0 * 0.0072 * 10.0 * 2.0; // 2 hours x depth x len x width
        assert!(
            res.mass_balance.outlet_m3 > 0.5 * injected_m3,
            "exfiltration must reach the toe (outlet {} vs injected {injected_m3})",
            res.mass_balance.outlet_m3
        );
        let residual = res.mass_balance.conservation_residual_m3().abs();
        assert!(
            residual / injected_m3 < 1.0e-2,
            "cascade conservation (residual {residual} vs injected {injected_m3})"
        );
        // And the outlet flow occurs on the pulse hours, not before.
        let pre_pulse_flow: f64 = res
            .outlet_hydrograph
            .iter()
            .filter(|s| s.time_s < 2.0 * 3600.0)
            .map(|s| s.outlet_unit_discharge_m2_s)
            .sum();
        assert!(
            pre_pulse_flow.abs() < 1.0e-12,
            "no routed flow before the exfiltration pulse"
        );
    }

    /// INV-OFEROUTE-012 gate fixture B (identity tier): an H2637-class
    /// subsurface-dominated closure vector — the D4 identity closes,
    /// the surface share is ~1%, and the ENV-Y water-yield fraction
    /// stays inside the `INV-SUBHYD-033` envelope. Operands drawn to
    /// the MAGPARITY01 class profile (steep-wet forest, ~99% routed
    /// lateral); the REAL-H2637 executed vector rides the activation
    /// increment (staged inputs located, WP record).
    #[test]
    fn gate_fixture_b_subsurface_dominated_closure_vector() {
        // Class profile: area 8,000 m2, P = 2,400 mm/yr; yield split
        // ~1% surface / ~99% lateral of a 62% total yield; ET 830 mm;
        // small deep percolation; storage change closes the identity.
        let area_m2 = 8_000.0;
        let precipitation_m3 = 2.400 * area_m2;
        let surface_outflow_m3 = 0.015 * area_m2; // ~1% of yield
        let latqcc_outlet_m3 = 1.473 * area_m2; // dominant lateral export
        let evapotranspiration_m3 = 0.830 * area_m2;
        let deep_percolation_m3 = 0.020 * area_m2;
        let storage_change_m3 = precipitation_m3
            - surface_outflow_m3
            - latqcc_outlet_m3
            - evapotranspiration_m3
            - deep_percolation_m3;
        let operands = SeamClosureOperands {
            precipitation_m3,
            surface_inflow_m3: 0.0,
            surface_outflow_m3,
            latqcc_outlet_m3,
            evapotranspiration_m3,
            storage_change_m3,
            deep_percolation_m3,
        };
        let residual = seam_closure_residual_m3(&operands).expect("closure operands");
        assert!(
            residual.abs() / precipitation_m3 < 1.0e-12,
            "the D4 identity must close (residual {residual})"
        );
        // Surface share of the combined yield is ~1% — the router owns
        // a sliver; latqcc bypass carries the rest.
        let surface_share = surface_outflow_m3 / (surface_outflow_m3 + latqcc_outlet_m3);
        assert!(surface_share < 0.02, "surface share {surface_share}");
        // ENV-Y (INV-SUBHYD-033): combined yield fraction in [0.55, 0.72].
        let env_y = (surface_outflow_m3 + latqcc_outlet_m3) / precipitation_m3;
        assert!(
            (0.55..=0.72).contains(&env_y),
            "ENV-Y {env_y} must stay in the observed envelope"
        );
        // ER sanity: ET inside the ENV-ET band (500-1000 mm).
        let et_mm = evapotranspiration_m3 / area_m2 * 1000.0;
        assert!((500.0..=1000.0).contains(&et_mm));
    }
}
