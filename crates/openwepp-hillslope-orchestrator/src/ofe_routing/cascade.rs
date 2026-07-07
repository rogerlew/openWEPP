//! MOFEFID Lane D / D5 (SC-OFEROUTE-001, INV-OFEROUTE-008/009): OFE-by-OFE
//! overland-flow cascade. Each OFE is routed with the D4 single-OFE
//! TVD-MacCormack solver; the upstream OFE's outlet hydrograph becomes the
//! downstream OFE's upstream boundary condition (Papanicolaou et al. 2018,
//! assumption 1: route sequentially summit -> outlet). Shadow-first / opt-in;
//! not wired into any production phase span.
//!
//! GAP-OFEROUTE-003 (runon reconciliation with DC01, resolved D6 = SUPERSEDE):
//! this cascade routes SURFACE runon as an hourly hydrograph. When the routing
//! subsystem is active it OWNS the hourly runon supply and SUPERSEDES DC01's
//! daily-lump runon re-infiltration (`SC-RUNOFFPART-001#INV-RUNOFFPART-031`)
//! with hydraulic surface routing. The upstream OFE's outlet hydrograph is a
//! downstream surface boundary condition and is NOT re-infiltrated; per-OFE
//! Green-Ampt infiltration (see `super::infiltration`) acts on the RAINFALL
//! only (Papanicolaou assumption 2). This module implements the routing +
//! handoff; the rainfall->excess coupling is `super::infiltration`.

use super::friction::skin_rain_term;
use super::implicit_recession::implicit_step_with_discharges;
use super::kinematic_wave::{
    Forcing, HydrographSample, KinematicWaveMesh, KinematicWaveSolver, MassBalance, RoutingError,
    RoutingResult,
};
use super::profile;
use super::seam::{SEAM_HOUR_BINS, SEAM_SECONDS_PER_HOUR, seam_rate_at};

/// One OFE in the cascade: its routing mesh and flow width. The flow width
/// converts unit-width discharge (m^2/s) between OFEs so total discharge
/// `Q = q * width` is continuous across the handoff.
#[derive(Debug, Clone)]
pub struct CascadeSegment {
    pub mesh: KinematicWaveMesh,
    /// OFE flow width (m).
    pub width_m: f64,
}

/// Per-OFE forcing supplied to the cascade run.
pub struct CascadeForcing<'a> {
    /// Rainfall-excess rate at `(ofe_index, cell_index, time_s)` -> m/s.
    pub rainfall_excess_m_s: &'a dyn Fn(usize, usize, f64) -> f64,
    /// Rainfall intensity at `(ofe_index, time_s)` -> m/s (skin term).
    pub rainfall_intensity_m_s: &'a dyn Fn(usize, f64) -> f64,
}

/// Width-aware cascade mass balance (total volume, m^3).
#[derive(Debug, Clone, Copy, Default)]
pub struct CascadeMassBalance {
    /// Total rainfall-excess volume over all OFEs (m^3).
    pub rainfall_excess_m3: f64,
    /// Outlet volume leaving the terminal OFE (m^3).
    pub outlet_m3: f64,
    /// Storage change summed across all OFEs (m^3).
    pub storage_change_m3: f64,
    /// Positivity-clamp injection summed across all OFEs (m^3).
    pub positivity_clamp_m3: f64,
}

impl CascadeMassBalance {
    /// Clamp-adjusted cascade conservation residual (m^3): should be ~0.
    #[must_use]
    pub fn conservation_residual_m3(&self) -> f64 {
        self.rainfall_excess_m3 - self.outlet_m3 - self.storage_change_m3 + self.positivity_clamp_m3
    }
}

/// Result of a cascade run.
#[derive(Debug, Clone)]
pub struct CascadeResult {
    /// Terminal-OFE outlet hydrograph (unit discharge at the cascade outlet).
    pub outlet_hydrograph: Vec<HydrographSample>,
    /// Peak outlet total discharge (m^3/s) and its time.
    pub peak_total_discharge_m3_s: f64,
    pub time_to_peak_s: f64,
    /// Per-OFE peak unit discharge (m^2/s), upstream to downstream.
    pub per_ofe_peak_unit_discharge_m2_s: Vec<f64>,
    /// Per-OFE outlet volume (m^3), upstream to downstream.
    pub per_ofe_outlet_m3: Vec<f64>,
    /// Per-OFE received upstream (runon) volume (m^3); OFE 0 is 0.
    pub per_ofe_received_upstream_m3: Vec<f64>,
    /// Per-OFE solver mass ledgers (per unit width, m^2), upstream to
    /// downstream. D10B seam diagnostic: exposes each OFE's own
    /// inflow/rain/outflow/storage/clamp ledger so cascade conservation
    /// residuals can be decomposed into per-OFE solver residuals and
    /// inter-OFE handoff (sampling/injection) mismatches.
    pub per_ofe_solver_mass: Vec<MassBalance>,
    pub mass_balance: CascadeMassBalance,
    /// Max Courant number over the whole cascade (CFL evidence).
    pub max_courant: f64,
}

/// Linearly interpolate an outlet hydrograph's unit discharge at `time_s`.
/// Clamps to the endpoints outside the recorded range.
fn interpolate_unit_discharge(hydrograph: &[HydrographSample], time_s: f64) -> f64 {
    if hydrograph.is_empty() {
        return 0.0;
    }
    if time_s <= hydrograph[0].time_s {
        return hydrograph[0].outlet_unit_discharge_m2_s.max(0.0);
    }
    let last = hydrograph[hydrograph.len() - 1];
    if time_s >= last.time_s {
        return last.outlet_unit_discharge_m2_s.max(0.0);
    }
    // binary search for the bracketing interval
    let mut lo = 0usize;
    let mut hi = hydrograph.len() - 1;
    while hi - lo > 1 {
        let mid = lo.midpoint(hi);
        if hydrograph[mid].time_s <= time_s {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let a = hydrograph[lo];
    let b = hydrograph[hi];
    let span = b.time_s - a.time_s;
    if span <= 0.0 {
        return a.outlet_unit_discharge_m2_s.max(0.0);
    }
    let frac = (time_s - a.time_s) / span;
    (a.outlet_unit_discharge_m2_s
        + frac * (b.outlet_unit_discharge_m2_s - a.outlet_unit_discharge_m2_s))
        .max(0.0)
}

/// Exact integral of the CONSERVATIVE per-bin outflow series over
/// `[t0, t1]` (m^2 per unit width): bin `k` holds the actual outflow mass
/// over `[k bin_dt, (k+1) bin_dt)`; the rate within a bin is uniform.
/// Rev 24 (Algorithm item 6): the downstream OFE's injection integrates
/// this exactly so the handoff conserves the upstream scheme's actual
/// discharged mass. Zero outside the recorded bins.
fn integrate_bin_series(bins_m2: &[f64], spans_s: &[f64], bin_dt_s: f64, t0: f64, t1: f64) -> f64 {
    if bins_m2.is_empty() || bin_dt_s <= 0.0 || t1 <= t0 {
        return 0.0;
    }
    let mut total = 0.0_f64;
    let start = t0.max(0.0);
    // Review-B M1: iterate the INTEGER bin index (guaranteed progress; the
    // floating-point boundary re-derivation has zero-progress witnesses).
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let mut k = ((start / bin_dt_s).floor().max(0.0)) as usize;
    while k < bins_m2.len() {
        // Review-B M3: each bin covers `[k bin_dt, k bin_dt + span_k)` at
        // rate `mass_k / span_k`; the final bin's span is its actual
        // covered remainder, so its whole mass is injectable by the
        // window end.
        #[allow(clippy::cast_precision_loss)]
        let bin_start = k as f64 * bin_dt_s;
        let span = spans_s.get(k).copied().unwrap_or(bin_dt_s);
        if span > 0.0 {
            let lo = bin_start.max(start);
            let hi = (bin_start + span).min(t1);
            if hi > lo {
                total += bins_m2[k] * (hi - lo) / span;
            }
        }
        if bin_start + bin_dt_s >= t1 {
            break;
        }
        k += 1;
    }
    total
}

/// Upstream handoff payload: the instantaneous sampled hydrograph (shape,
/// point-BC fallback) plus the conservative bin series (mass).
pub struct UpstreamHandoff {
    pub samples: Vec<HydrographSample>,
    pub bins_m2: Vec<f64>,
    pub bin_spans_s: Vec<f64>,
    pub bin_dt_s: f64,
    /// Width (m) of the OFE that produced this outlet series.
    pub width_m: f64,
}

/// Route ONE OFE with the D4 solver under the cascade's exact handoff
/// semantics (rev 24 conservative bin-series injection; rev-27 extraction so
/// the `run_cascade` shadow path and the D15A active production path share
/// one code path — the closure construction and width-ratio scaling here are
/// the loop body `run_cascade` previously inlined, unchanged).
///
/// `forcing_breakpoints_s` are known forcing discontinuity times (the
/// solver's High-2 exact-source-history rule: no step may straddle one).
/// The diagnostic shadow/cascade path passes `&[]` (its recorded behavior,
/// bit-identity-frozen); the ACTIVE owner passes the hourly source
/// boundaries so its booked injection integrates the piecewise-constant
/// seam series exactly (rev-27 seam cross-ledger check).
#[allow(clippy::too_many_arguments)]
pub fn route_single_ofe(
    segment: &CascadeSegment,
    rainfall_excess_m_s: &dyn Fn(usize, f64) -> f64,
    rainfall_intensity_m_s: &dyn Fn(f64) -> f64,
    upstream: Option<&UpstreamHandoff>,
    forcing_breakpoints_s: &[f64],
    end_time_s: f64,
    sample_dt_s: f64,
    max_dt_s: f64,
) -> Result<RoutingResult, RoutingError> {
    if !segment.width_m.is_finite() || segment.width_m <= 0.0 {
        return Err(RoutingError::DegenerateConfiguration);
    }
    let width_ratio = match upstream {
        Some(handoff) => handoff.width_m / segment.width_m,
        None => 0.0,
    };
    let upstream_point = |t: f64| -> f64 {
        match upstream {
            Some(h) => {
                profile::count_upstream_interpolation_calls(1);
                interpolate_unit_discharge(&h.samples, t) * width_ratio
            }
            None => 0.0,
        }
    };
    let upstream_integral = |t0: f64, t1: f64| -> f64 {
        match upstream {
            Some(h) => {
                profile::count_upstream_interpolation_calls(1);
                integrate_bin_series(&h.bins_m2, &h.bin_spans_s, h.bin_dt_s, t0, t1) * width_ratio
            }
            None => 0.0,
        }
    };
    let ofe_forcing = Forcing {
        rainfall_excess_m_s,
        upstream_inflow_m2_s: &upstream_point,
        rainfall_intensity_m_s,
    };
    let setup_span = profile::span_start();
    let mut solver = KinematicWaveSolver::new(segment.mesh.clone());
    profile::end_solver_setup(setup_span);
    let integral_closure: Option<&dyn Fn(f64, f64) -> f64> = if upstream.is_some() {
        Some(&upstream_integral)
    } else {
        None
    };
    solver.run_with_options(
        &ofe_forcing,
        integral_closure,
        forcing_breakpoints_s,
        end_time_s,
        sample_dt_s,
        max_dt_s,
    )
}

/// Run the OFE-by-OFE cascade summit -> outlet. Each OFE is routed with the
/// D4 solver; OFE `i`'s outlet hydrograph, scaled by the width ratio
/// `w_i / w_{i+1}` for discharge continuity, is OFE `i+1`'s upstream boundary.
#[allow(clippy::too_many_lines)]
pub fn run_cascade(
    segments: &[CascadeSegment],
    forcing: &CascadeForcing<'_>,
    end_time_s: f64,
    sample_dt_s: f64,
    max_dt_s: f64,
) -> Result<CascadeResult, RoutingError> {
    if segments.is_empty() {
        return Err(RoutingError::DegenerateConfiguration);
    }
    for seg in segments {
        if !seg.width_m.is_finite() || seg.width_m <= 0.0 {
            return Err(RoutingError::DegenerateConfiguration);
        }
    }

    let mut mass = CascadeMassBalance::default();
    let mut per_ofe_peak = Vec::with_capacity(segments.len());
    let mut per_ofe_outlet_m3 = Vec::with_capacity(segments.len());
    let mut per_ofe_received_m3 = Vec::with_capacity(segments.len());
    let mut per_ofe_solver_mass = Vec::with_capacity(segments.len());
    let mut prev_hydrograph: Option<UpstreamHandoff> = None;
    let mut outlet_hydrograph: Vec<HydrographSample> = Vec::new();
    let mut terminal_outlet_m3 = 0.0_f64;
    let mut max_courant = 0.0_f64;

    for (i, seg) in segments.iter().enumerate() {
        // Upstream boundary from the previous OFE's outlet, scaled to this
        // OFE's width for total-discharge continuity (rev 24: the downstream
        // solver injects the EXACT integral of the upstream CONSERVATIVE bin
        // series). The routing itself is the shared `route_single_ofe`.
        let prev = prev_hydrograph.take();
        let excess = |cell: usize, t: f64| (forcing.rainfall_excess_m_s)(i, cell, t);
        let intensity = |t: f64| (forcing.rainfall_intensity_m_s)(i, t);
        let result = route_single_ofe(
            seg,
            &excess,
            &intensity,
            prev.as_ref(),
            &[],
            end_time_s,
            sample_dt_s,
            max_dt_s,
        )?;

        // Received upstream (runon) volume into this OFE (m^3): the width-scaled
        // upstream unit-discharge integral times this OFE's width.
        let received_m2 = result.mass_balance.inflow_m2;
        per_ofe_received_m3.push(received_m2 * seg.width_m);
        // Rainfall excess volume for this OFE (m^3).
        mass.rainfall_excess_m3 += result.mass_balance.rainfall_excess_m2 * seg.width_m;
        mass.storage_change_m3 += result.mass_balance.storage_change_m2 * seg.width_m;
        mass.positivity_clamp_m3 += result.mass_balance.positivity_clamp_m2 * seg.width_m;
        per_ofe_peak.push(result.peak_unit_discharge_m2_s);
        per_ofe_solver_mass.push(result.mass_balance);
        // Rev 24: outlet volumes are booked from the solver's ACTUAL
        // outflow (== bin-series sum), not a sample-grid quadrature.
        per_ofe_outlet_m3.push(result.mass_balance.outflow_m2 * seg.width_m);
        if result.max_courant > max_courant {
            max_courant = result.max_courant;
        }

        // Hand off to the next OFE (move, no clone): the terminal OFE's
        // hydrograph is the cascade outlet; interior OFEs feed the next.
        if i + 1 < segments.len() {
            prev_hydrograph = Some(UpstreamHandoff {
                samples: result.hydrograph,
                bins_m2: result.outlet_bin_outflow_m2,
                bin_spans_s: result.outlet_bin_spans_s,
                bin_dt_s: result.outlet_bin_dt_s,
                width_m: seg.width_m,
            });
        } else {
            terminal_outlet_m3 = result.mass_balance.outflow_m2 * seg.width_m;
            outlet_hydrograph = result.hydrograph;
        }
    }

    // Cascade outlet volume: the terminal OFE's actual discharged mass.
    let terminal_width = segments[segments.len() - 1].width_m;
    mass.outlet_m3 = terminal_outlet_m3;

    // Peak total discharge (m^3/s) at the cascade outlet.
    let mut peak_total_discharge = 0.0_f64;
    let mut time_to_peak = 0.0_f64;
    for s in &outlet_hydrograph {
        let total = s.outlet_unit_discharge_m2_s * terminal_width;
        if total > peak_total_discharge {
            peak_total_discharge = total;
            time_to_peak = s.time_s;
        }
    }

    Ok(CascadeResult {
        outlet_hydrograph,
        peak_total_discharge_m3_s: peak_total_discharge,
        time_to_peak_s: time_to_peak,
        per_ofe_peak_unit_discharge_m2_s: per_ofe_peak,
        per_ofe_outlet_m3,
        per_ofe_received_upstream_m3: per_ofe_received_m3,
        per_ofe_solver_mass,
        mass_balance: mass,
        max_courant,
    })
}

/// Rev-30 deficit-carry absorption step: book one bin's non-negative mass
/// against a pending (<= 0) attribution deficit. Returns the booked value
/// (never negative); the deficit shrinks by exactly what was absorbed, so
/// `booked + new_carry == mass + old_carry` (exact total by construction).
fn absorb_deficit(mass_m2: f64, carry_m2: &mut f64) -> f64 {
    let booked = mass_m2 + *carry_m2;
    if booked < 0.0 {
        *carry_m2 = booked;
        0.0
    } else {
        *carry_m2 = 0.0;
        booked
    }
}

/// Rev-30 end-of-window carry disposition: a MATERIAL deficit still fails
/// closed — the whole day window could not absorb it, which is physically
/// unexpected (total outflow is non-negative) and must not publish
/// (`NegativeOutletBin`, the same posture as the plain path). A sub-noise
/// remainder (the recorder's noise rule applied at the composed level:
/// `1e-9` of the series gross) is absorbed BACKWARD from the trailing
/// positive bins — exact total, all bins stay non-negative
/// (`Σ bins >= |carry|` whenever the booked total outflow is
/// non-negative). Review C-L1 (documented + pinned by test): on an
/// all-zero / insufficient-gross series the sub-noise remainder cannot be
/// absorbed and is DROPPED — a bounded attribution slack (`<=` the noise
/// floor, absolute floor `1e-21 m^2`), never a mass-ledger change (the
/// ledgers book actual boundary fluxes independently of bin attribution);
/// bins are left untouched rather than published negative.
fn dispose_terminal_carry(bins_m2: &mut [f64], mut carry_m2: f64) -> Result<(), RoutingError> {
    if carry_m2 >= 0.0 {
        return Ok(());
    }
    let gross: f64 = bins_m2.iter().sum();
    let noise_floor = 1.0e-9 * gross.max(1.0e-12);
    if -carry_m2 > noise_floor {
        return Err(RoutingError::NegativeOutletBin);
    }
    for slot in bins_m2.iter_mut().rev() {
        if carry_m2 >= 0.0 {
            break;
        }
        *slot = absorb_deficit(*slot, &mut carry_m2);
    }
    Ok(())
}

/// T3-I2/T3-AGG (`SC-OFEROUTE-001` rev 28/30, EXPERIMENTAL opt-in): route
/// one OFE with HYBRID stepping — the explicit TVD-MacCormack scheme on
/// source-active sample bins, the implicit backward-Euler upwind stepper
/// (`implicit_recession`) on AGGRESSIVE smooth bins (zero source on every
/// cell; upstream inflow is booked exactly as the bin's interval mean by
/// the implicit step — rev 30 supersedes the rev-28 strict rule). The
/// switching predicate is FORCING-derived per 900 s outlet bin
/// (deterministic, hysteresis-free); implicit bins step at exactly the bin
/// cadence, so the conservative outlet-bin series keeps its exact per-bin
/// semantics. State hands off at span boundaries per the design's §2.2 seam
/// rule (depths carry; entering explicit installs the implicit solve's own
/// converged equilibrium discharges). The composed ledger is exact by
/// construction (both schemes carry exact ledgers; the seam moves no mass);
/// a short explicit span's terminal front-arrival attribution deficit
/// carries forward across span boundaries under the exact-total rule
/// (rev 30), and a material deficit surviving the whole window fails
/// closed (`NegativeOutletBin`).
#[allow(clippy::too_many_lines, clippy::too_many_arguments)]
pub fn route_single_ofe_hybrid(
    segment: &CascadeSegment,
    source_rates_m_s: &[f64; SEAM_HOUR_BINS],
    intensity_rates_m_s: &[f64; SEAM_HOUR_BINS],
    upstream: Option<&UpstreamHandoff>,
    forcing_breakpoints_s: &[f64],
    end_time_s: f64,
    sample_dt_s: f64,
    max_dt_s: f64,
) -> Result<RoutingResult, RoutingError> {
    if !segment.width_m.is_finite() || segment.width_m <= 0.0 {
        return Err(RoutingError::DegenerateConfiguration);
    }
    if sample_dt_s <= 0.0 || end_time_s <= 0.0 || !end_time_s.is_finite() {
        return Err(RoutingError::DegenerateConfiguration);
    }
    // The hybrid partitions the window by OUTLET BINS; a non-integral
    // window would need partial-bin regime logic — the active-path windows
    // are hour+tail multiples of the bin width by construction.
    let bins_f = end_time_s / sample_dt_s;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n_bins = bins_f.round() as usize;
    #[allow(clippy::cast_precision_loss)]
    let reconstructed = n_bins as f64 * sample_dt_s;
    if n_bins == 0 || (reconstructed - end_time_s).abs() > 1.0e-9 {
        return Err(RoutingError::DegenerateConfiguration);
    }
    // Rev-30 review C-M1: the smoothness mask samples the source at bin
    // START only, which is exact ONLY when every bin lies within one seam
    // hour (the source series is piecewise-constant per hour). Enforce
    // locally rather than trusting the caller: the sample cadence must
    // partition `SEAM_SECONDS_PER_HOUR` exactly, else a bin could straddle
    // an hourly source transition and route implicitly with a stale zero
    // sample (a silent source drop).
    let per_hour_f = SEAM_SECONDS_PER_HOUR / sample_dt_s;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let per_hour = per_hour_f.round() as usize;
    #[allow(clippy::cast_precision_loss)]
    let hour_reconstructed = per_hour as f64 * sample_dt_s;
    if per_hour == 0 || (hour_reconstructed - SEAM_SECONDS_PER_HOUR).abs() > 1.0e-9 {
        return Err(RoutingError::DegenerateConfiguration);
    }
    let n = segment.mesh.cell_count();
    if n == 0 {
        return Err(RoutingError::DegenerateConfiguration);
    }
    let width_ratio = match upstream {
        Some(handoff) => handoff.width_m / segment.width_m,
        None => 0.0,
    };
    let upstream_mass_in = |t0: f64, t1: f64| -> f64 {
        match upstream {
            Some(h) => {
                integrate_bin_series(&h.bins_m2, &h.bin_spans_s, h.bin_dt_s, t0, t1) * width_ratio
            }
            None => 0.0,
        }
    };
    // Per-bin AGGRESSIVE smoothness mask (rev-30 rule): implicit-eligible
    // when the bin has ZERO source on every cell. Upstream inflow does NOT
    // force explicit stepping — the implicit step books the interval-mean
    // upstream mass exactly (I0 measures 55.5 % coverage vs 30 % for the
    // superseded rev-28 strict rule). The composition defect that blocked
    // this rule (short explicit spans sandwiched between implicit bins
    // stranding front-arrival terminal deficits — `NegativeOutletBin`,
    // H2637 lane 17 day 54) is closed by the rev-30 cross-span deficit
    // carry below.
    #[allow(clippy::cast_precision_loss)]
    let bin_smooth: Vec<bool> = (0..n_bins)
        .map(|b| {
            let t0 = b as f64 * sample_dt_s;
            seam_rate_at(source_rates_m_s, t0) == 0.0
        })
        .collect();

    let mut depths = vec![0.0_f64; n];
    let mut discharges = vec![0.0_f64; n];
    let mut global_bins_m2 = vec![0.0_f64; n_bins];
    let mut mass = MassBalance::default();
    let mut max_courant = 0.0_f64;
    let mut max_tv = 0.0_f64;
    let mut peak = 0.0_f64;
    let mut time_to_peak = 0.0_f64;
    let mut clamp_total = 0.0_f64;
    // Rev-30 cross-span deficit carry (<= 0, m^2): continues the
    // BinRecorder's forward-redistribution rule ACROSS span boundaries — a
    // short explicit span's terminal front-arrival attribution deficit is
    // absorbed by subsequent composed bins under the same exact-total,
    // non-negative rule. Attribution only: every mass ledger books actual
    // boundary fluxes and never sees the carry.
    let mut carry_m2 = 0.0_f64;
    profile::count_solver_runs(1);

    let mut bin = 0_usize;
    while bin < n_bins {
        let span_smooth = bin_smooth[bin];
        let mut span_end = bin + 1;
        while span_end < n_bins && bin_smooth[span_end] == span_smooth {
            span_end += 1;
        }
        #[allow(clippy::cast_precision_loss)]
        let t0 = bin as f64 * sample_dt_s;
        #[allow(clippy::cast_precision_loss)]
        let t1 = span_end as f64 * sample_dt_s;
        if span_smooth {
            // IMPLICIT span: one step per bin at the bin cadence.
            let zero_source = vec![0.0_f64; n];
            for (offset, slot) in global_bins_m2[bin..span_end].iter_mut().enumerate() {
                #[allow(clippy::cast_precision_loss)]
                let t_bin = t0 + offset as f64 * sample_dt_s;
                let rain_term = skin_rain_term(seam_rate_at(intensity_rates_m_s, t_bin));
                // Interval-mean upstream inflow for the bin — booked exactly
                // by the implicit step (aggressive-rule recession inflow).
                let q_up_bin = upstream_mass_in(t_bin, t_bin + sample_dt_s) / sample_dt_s;
                let outcome = implicit_step_with_discharges(
                    &segment.mesh,
                    &mut depths,
                    Some(&mut discharges),
                    q_up_bin,
                    &zero_source,
                    rain_term,
                    sample_dt_s,
                )?;
                profile::count_solver_steps(1);
                profile::count_solver_steps_implicit(1);
                *slot += absorb_deficit(outcome.outflow_m2, &mut carry_m2);
                mass.inflow_m2 += outcome.inflow_m2;
                mass.scheme_inflow_m2 += outcome.inflow_m2;
                mass.outflow_m2 += outcome.outflow_m2;
                mass.scheme_outflow_m2 += outcome.outflow_m2;
                let q_mean = outcome.outflow_m2 / sample_dt_s;
                if q_mean > peak {
                    peak = q_mean;
                    time_to_peak = t_bin + sample_dt_s;
                }
            }
        } else {
            // EXPLICIT span: the D10B scheme over [t0, t1), continuing from
            // the hybrid state; forcing closures shift to global time.
            let mut solver = KinematicWaveSolver::new(segment.mesh.clone());
            solver.set_state(&depths, &discharges)?;
            let excess = |_cell: usize, t_local: f64| seam_rate_at(source_rates_m_s, t0 + t_local);
            let intensity = |t_local: f64| seam_rate_at(intensity_rates_m_s, t0 + t_local);
            let upstream_point = |t_local: f64| -> f64 {
                match upstream {
                    Some(h) => {
                        profile::count_upstream_interpolation_calls(1);
                        interpolate_unit_discharge(&h.samples, t0 + t_local) * width_ratio
                    }
                    None => 0.0,
                }
            };
            let upstream_integral = |a: f64, b: f64| -> f64 {
                profile::count_upstream_interpolation_calls(1);
                upstream_mass_in(t0 + a, t0 + b)
            };
            let forcing = Forcing {
                rainfall_excess_m_s: &excess,
                upstream_inflow_m2_s: &upstream_point,
                rainfall_intensity_m_s: &intensity,
            };
            let integral_closure: Option<&dyn Fn(f64, f64) -> f64> = if upstream.is_some() {
                Some(&upstream_integral)
            } else {
                None
            };
            let local_breakpoints: Vec<f64> = forcing_breakpoints_s
                .iter()
                .filter(|bp| **bp > t0 + 1.0e-9 && **bp < t1 - 1.0e-9)
                .map(|bp| bp - t0)
                .collect();
            let (result, span_deficit_m2) = solver.run_with_options_deficit_carry(
                &forcing,
                integral_closure,
                &local_breakpoints,
                t1 - t0,
                sample_dt_s,
                max_dt_s,
            )?;
            for (offset, bin_mass) in result.outlet_bin_outflow_m2.iter().enumerate() {
                if bin + offset < n_bins {
                    global_bins_m2[bin + offset] += absorb_deficit(*bin_mass, &mut carry_m2);
                }
            }
            // The span's own terminal deficit joins the carry AFTER its
            // bins (it belongs at the span end; the recorder's forward
            // redistribution already exhausted the in-span bins).
            carry_m2 += span_deficit_m2;
            mass.inflow_m2 += result.mass_balance.inflow_m2;
            mass.scheme_inflow_m2 += result.mass_balance.inflow_m2;
            mass.rainfall_excess_m2 += result.mass_balance.rainfall_excess_m2;
            mass.outflow_m2 += result.mass_balance.outflow_m2;
            mass.scheme_outflow_m2 += result.mass_balance.outflow_m2;
            clamp_total += result.mass_balance.positivity_clamp_m2;
            if result.max_courant > max_courant {
                max_courant = result.max_courant;
            }
            if result.max_homogeneous_tv_increase_m2_s > max_tv {
                max_tv = result.max_homogeneous_tv_increase_m2_s;
            }
            if result.peak_unit_discharge_m2_s > peak {
                peak = result.peak_unit_discharge_m2_s;
                time_to_peak = t0 + result.time_to_peak_s;
            }
            depths.copy_from_slice(solver.depth_state());
            discharges.copy_from_slice(solver.discharge_state());
        }
        bin = span_end;
    }

    dispose_terminal_carry(&mut global_bins_m2, carry_m2)?;

    mass.positivity_clamp_m2 = clamp_total;
    mass.storage_change_m2 = depths.iter().sum::<f64>() * segment.mesh.cell_length_m;
    // Bin-mean hydrograph over the composed global bins (same shape the
    // BinRecorder exports).
    let mut hydrograph = Vec::with_capacity(n_bins + 1);
    hydrograph.push(HydrographSample {
        time_s: 0.0,
        outlet_unit_discharge_m2_s: 0.0,
        outlet_depth_m: 0.0,
    });
    for (k, bin_mass) in global_bins_m2.iter().enumerate() {
        #[allow(clippy::cast_precision_loss)]
        let t_mid = k as f64 * sample_dt_s + 0.5 * sample_dt_s;
        hydrograph.push(HydrographSample {
            time_s: t_mid,
            outlet_unit_discharge_m2_s: bin_mass / sample_dt_s,
            outlet_depth_m: 0.0,
        });
    }
    profile::count_hydrograph_samples(hydrograph.len() as u64);
    Ok(RoutingResult {
        hydrograph,
        mass_balance: mass,
        peak_unit_discharge_m2_s: peak,
        time_to_peak_s: time_to_peak,
        max_courant,
        max_homogeneous_tv_increase_m2_s: max_tv,
        outlet_bin_outflow_m2: global_bins_m2,
        outlet_bin_dt_s: sample_dt_s,
        outlet_bin_spans_s: vec![sample_dt_s; n_bins],
    })
}

#[cfg(test)]
mod tests {
    use super::super::kinematic_wave::CellParameters;
    use super::*;

    fn bare_segment(length: f64, cells: usize, slope: f64, ko: f64, width: f64) -> CascadeSegment {
        CascadeSegment {
            mesh: KinematicWaveMesh::uniform(length, cells, CellParameters::bare(slope, ko)),
            width_m: width,
        }
    }

    // Two-OFE bare cascade: total rainfall excess must equal the terminal
    // outlet plus total storage (width-aware), and the handoff volume must
    // carry OFE 1's outlet into OFE 2 as received runon.
    #[test]
    fn two_ofe_cascade_conserves_and_hands_off() {
        let v = 60.0 / 3.6e6;
        let segs = vec![
            bare_segment(10.0, 20, 0.08, 500.0, 2.0),
            bare_segment(10.0, 20, 0.06, 500.0, 2.0),
        ];
        let excess = |_ofe: usize, _cell: usize, _t: f64| v;
        let intensity = |_ofe: usize, _t: f64| v;
        let forcing = CascadeForcing {
            rainfall_excess_m_s: &excess,
            rainfall_intensity_m_s: &intensity,
        };
        let res = run_cascade(&segs, &forcing, 3600.0, 10.0, 2.0).expect("cascade runs");

        // clamp-adjusted cascade conservation ~ 0 (relative to rainfall input)
        let rain = res.mass_balance.rainfall_excess_m3;
        assert!(rain > 0.0);
        assert!(
            res.mass_balance.conservation_residual_m3().abs() / rain < 1.0e-2,
            "cascade residual {} vs rain {rain}",
            res.mass_balance.conservation_residual_m3()
        );
        // OFE 2 receives OFE 1's outlet as runon (nonzero), width ratio 1.0
        assert!(
            res.per_ofe_received_upstream_m3[0].abs() < 1.0e-12,
            "top OFE has no runon"
        );
        assert!(
            res.per_ofe_received_upstream_m3[1] > 0.0,
            "OFE 2 must receive runon"
        );
        assert!(
            (res.per_ofe_received_upstream_m3[1] - res.per_ofe_outlet_m3[0]).abs()
                / res.per_ofe_outlet_m3[0]
                < 1.0e-2,
            "runon into OFE 2 must equal OFE 1 outlet (equal widths): received {} vs outlet {}",
            res.per_ofe_received_upstream_m3[1],
            res.per_ofe_outlet_m3[0]
        );
        assert!(res.max_courant <= 1.0 + 1.0e-9);
    }

    // Three-OFE cascade: discharge accumulates downslope (each OFE's outlet >=
    // the previous, given added rainfall), and CFL holds.
    #[test]
    fn three_ofe_cascade_accumulates_downslope() {
        let v = 74.0 / 3.6e6;
        let segs = vec![
            bare_segment(8.0, 16, 0.07, 500.0, 1.0),
            bare_segment(8.0, 16, 0.07, 500.0, 1.0),
            bare_segment(8.0, 16, 0.07, 500.0, 1.0),
        ];
        let excess = |_o: usize, _c: usize, _t: f64| v;
        let intensity = |_o: usize, _t: f64| v;
        let forcing = CascadeForcing {
            rainfall_excess_m_s: &excess,
            rainfall_intensity_m_s: &intensity,
        };
        let res = run_cascade(&segs, &forcing, 3600.0, 10.0, 2.0).expect("cascade runs");
        // steady outlet unit discharge grows downslope (more contributing length)
        let peaks = &res.per_ofe_peak_unit_discharge_m2_s;
        assert!(
            peaks[1] > peaks[0] && peaks[2] > peaks[1],
            "peaks accumulate: {peaks:?}"
        );
        // terminal steady discharge ~ v * total_length (3 * 8 = 24 m)
        let q_terminal = res
            .outlet_hydrograph
            .last()
            .unwrap()
            .outlet_unit_discharge_m2_s;
        assert!(
            (q_terminal - v * 24.0).abs() / (v * 24.0) < 0.05,
            "terminal steady q {q_terminal} vs v*L {}",
            v * 24.0
        );
        assert!(res.max_courant <= 1.0 + 1.0e-9);
    }

    // Case 3 (vegetation patchiness): the robust, roughness-monotone signature
    // of the filter-strip effect. At steady state the outlet DISCHARGE is fixed
    // by mass balance (v*L) regardless of roughness, so vegetation cannot change
    // the steady peak; what it changes is STORAGE - added resistance backs up
    // more water (higher steady depth) and the discharge-depth relation
    // `q = alpha h^1.5` with lower alpha requires a higher h for the same q.
    // This is monotone in f_eq and independent of storm/concentration timing.
    #[test]
    fn case3_vegetated_strip_backs_up_more_water_than_bare() {
        let v = 74.0 / 3.6e6;
        let excess = |_o: usize, _c: usize, _t: f64| v;
        let intensity = |_o: usize, _t: f64| v;
        let forcing = CascadeForcing {
            rainfall_excess_m_s: &excess,
            rainfall_intensity_m_s: &intensity,
        };

        let bare = vec![
            bare_segment(6.1, 24, 0.07, 500.0, 3.6),
            bare_segment(2.45, 12, 0.07, 500.0, 3.6),
        ];
        let bare_res = run_cascade(&bare, &forcing, 3600.0, 5.0, 1.0).expect("bare runs");

        // vegetated downslope strip (Case 3 params: LAI=1, h_c=0.1, C_d=1.0)
        let mut veg_cell = CellParameters::bare(0.07, 500.0);
        veg_cell.leaf_area_index = 1.0;
        veg_cell.canopy_height_m = 0.1;
        veg_cell.vegetation_drag_coefficient = 1.0;
        let veg = vec![
            bare_segment(6.1, 24, 0.07, 500.0, 3.6),
            CascadeSegment {
                mesh: KinematicWaveMesh::uniform(2.45, 12, veg_cell),
                width_m: 3.6,
            },
        ];
        let veg_res = run_cascade(&veg, &forcing, 3600.0, 5.0, 1.0).expect("veg runs");

        let bare_depth = bare_res.outlet_hydrograph.last().unwrap().outlet_depth_m;
        let veg_depth = veg_res.outlet_hydrograph.last().unwrap().outlet_depth_m;
        // Steady discharge from the tail-mean of the bin-flux hydrograph
        // (rev 24: the exported series is bin-mean boundary flux with a
        // bounded slow boundary ripple; the tail mean is the steady
        // measure).
        let tail_mean = |h: &[HydrographSample]| -> f64 {
            let n = 40.min(h.len());
            h[h.len() - n..]
                .iter()
                .map(|s| s.outlet_unit_discharge_m2_s)
                .sum::<f64>()
                / f64::from(u32::try_from(n).unwrap_or(u32::MAX))
        };
        let bare_q = tail_mean(&bare_res.outlet_hydrograph);
        let veg_q = tail_mean(&veg_res.outlet_hydrograph);

        // steady discharge is the same (mass balance): roughness does not change it
        assert!(
            (veg_q - bare_q).abs() / bare_q < 1.0e-2,
            "steady outlet discharge must match (mass balance): veg {veg_q} bare {bare_q}"
        );
        // but the vegetated strip backs up more water: higher steady depth
        // (directional, monotone in f_eq; ~1.7% for LAI=1 light canopy at this
        // shallow flow depth - small but unambiguously above float noise).
        assert!(
            veg_depth > bare_depth * 1.005,
            "vegetated strip must back up more water (higher steady depth): veg {veg_depth} bare {bare_depth}"
        );
        // both conserve and hold CFL
        assert!(veg_res.max_courant <= 1.0 + 1.0e-9);
        assert!(
            veg_res.mass_balance.conservation_residual_m3().abs()
                / veg_res.mass_balance.rainfall_excess_m3
                < 1.0e-2
        );
    }

    #[test]
    fn width_change_scales_handoff_for_discharge_continuity() {
        // OFE 1 width 2.0 -> OFE 2 width 1.0: unit discharge doubles at the
        // handoff so total discharge Q = q*w is continuous.
        let v = 60.0 / 3.6e6;
        let segs = vec![
            bare_segment(10.0, 20, 0.08, 500.0, 2.0),
            bare_segment(10.0, 20, 0.08, 500.0, 1.0),
        ];
        let excess = |_o: usize, _c: usize, _t: f64| v;
        let intensity = |_o: usize, _t: f64| v;
        let forcing = CascadeForcing {
            rainfall_excess_m_s: &excess,
            rainfall_intensity_m_s: &intensity,
        };
        let res = run_cascade(&segs, &forcing, 3600.0, 10.0, 2.0).expect("cascade runs");
        // total discharge continuity: OFE 2 received volume (m^3) == OFE 1
        // outlet volume (m^3), independent of width (volumes are width-aware).
        assert!(
            (res.per_ofe_received_upstream_m3[1] - res.per_ofe_outlet_m3[0]).abs()
                / res.per_ofe_outlet_m3[0]
                < 1.0e-2,
            "width-scaled handoff must conserve total volume"
        );
    }

    // D14: cascade-level slots — per-OFE solver setup span and the upstream
    // handoff interpolation counter. Holds `profile::test_flag_guard`
    // because the enable flag is process-global (libtest threads share it).
    #[test]
    fn cascade_profile_counts_setup_and_upstream_interpolation() {
        use super::super::profile;

        let _flag_guard = profile::test_flag_guard();
        profile::set_enabled(true);
        let _ = profile::snapshot_and_reset();
        let v = 60.0 / 3.6e6;
        let segs = vec![
            bare_segment(10.0, 10, 0.08, 500.0, 2.0),
            bare_segment(10.0, 10, 0.06, 500.0, 2.0),
        ];
        let excess = |_ofe: usize, _cell: usize, _t: f64| v;
        let intensity = |_ofe: usize, _t: f64| v;
        let forcing = CascadeForcing {
            rainfall_excess_m_s: &excess,
            rainfall_intensity_m_s: &intensity,
        };
        let _ = run_cascade(&segs, &forcing, 600.0, 30.0, 2.0).expect("cascade runs");
        let snapshot = profile::snapshot_and_reset();
        profile::set_enabled(false);
        assert_eq!(snapshot.solver_runs, 2, "one solver run per OFE");
        assert!(
            snapshot.upstream_interpolation_calls > 0,
            "downstream OFE must interpolate the upstream hydrograph"
        );
        assert!(snapshot.solver_setup_ns > 0, "setup slot timed");
    }

    /// T3-I2: a window whose every bin is source-active takes ONE explicit
    /// span — the hybrid must be BIT-IDENTICAL to `route_single_ofe` there
    /// (same solver, same state, same breakpoints, same window).
    #[test]
    fn hybrid_is_bit_identical_on_all_explicit_windows() {
        use super::super::seam::{SEAM_HOUR_BINS, seam_rate_at};
        let segment = bare_segment(80.0, 10, 0.05, 500.0, 2.0);
        let mut source = [0.0_f64; SEAM_HOUR_BINS];
        source[0] = 60.0 / 3.6e6;
        source[1] = 30.0 / 3.6e6;
        let intensity = [0.0_f64; SEAM_HOUR_BINS];
        let window_s = 2.0 * 3600.0; // both hours active: all bins explicit
        let breakpoints = [3600.0_f64];
        let excess = |_cell: usize, t: f64| seam_rate_at(&source, t);
        let intensity_at = |t: f64| seam_rate_at(&intensity, t);
        let explicit = route_single_ofe(
            &segment,
            &excess,
            &intensity_at,
            None,
            &breakpoints,
            window_s,
            900.0,
            300.0,
        )
        .expect("explicit route");
        let hybrid = route_single_ofe_hybrid(
            &segment,
            &source,
            &intensity,
            None,
            &breakpoints,
            window_s,
            900.0,
            300.0,
        )
        .expect("hybrid route");
        assert_eq!(
            explicit.outlet_bin_outflow_m2.len(),
            hybrid.outlet_bin_outflow_m2.len()
        );
        for (a, b) in explicit
            .outlet_bin_outflow_m2
            .iter()
            .zip(hybrid.outlet_bin_outflow_m2.iter())
        {
            assert_eq!(a.to_bits(), b.to_bits(), "all-explicit bins bit-identical");
        }
        assert_eq!(
            explicit.mass_balance.outflow_m2.to_bits(),
            hybrid.mass_balance.outflow_m2.to_bits()
        );
    }

    /// T3-I2: event day (source hour 0) + smooth tail — the hybrid ledger is
    /// exact, positivity holds (no clamps on implicit bins), and the
    /// hybrid-vs-explicit per-bin fidelity delta stays within the
    /// I1-ladder-informed regression bound.
    #[test]
    fn hybrid_event_day_ledger_exact_and_fidelity_bounded() {
        use super::super::seam::{SEAM_HOUR_BINS, seam_rate_at};
        let segment = bare_segment(80.0, 10, 0.05, 500.0, 2.0);
        let mut source = [0.0_f64; SEAM_HOUR_BINS];
        source[0] = 60.0 / 3.6e6;
        let intensity = [0.0_f64; SEAM_HOUR_BINS];
        let window_s = 7.0 * 3600.0; // hour 0 active + 6 h smooth tail
        let breakpoints = [3600.0_f64];
        let hybrid = route_single_ofe_hybrid(
            &segment,
            &source,
            &intensity,
            None,
            &breakpoints,
            window_s,
            900.0,
            300.0,
        )
        .expect("hybrid route");
        let residual = hybrid.mass_balance.conservation_residual_m2();
        let scale = hybrid.mass_balance.rainfall_excess_m2.max(1.0e-12);
        assert!(
            residual.abs() / scale < 1.0e-9,
            "hybrid ledger residual {residual} vs rain {scale}"
        );
        // Fidelity vs all-explicit on the same window.
        let excess = |_cell: usize, t: f64| seam_rate_at(&source, t);
        let intensity_at = |t: f64| seam_rate_at(&intensity, t);
        let explicit = route_single_ofe(
            &segment,
            &excess,
            &intensity_at,
            None,
            &breakpoints,
            window_s,
            900.0,
            300.0,
        )
        .expect("explicit route");
        let ref_total: f64 = explicit.outlet_bin_outflow_m2.iter().sum();
        let l1: f64 = hybrid
            .outlet_bin_outflow_m2
            .iter()
            .zip(explicit.outlet_bin_outflow_m2.iter())
            .map(|(a, b)| (a - b).abs())
            .sum::<f64>()
            / ref_total.max(1.0e-18);
        // I1 ladder measured ~0.43 per-bin L1 at dt=900 on pure recession;
        // an event day dilutes it with identical explicit bins. Regression
        // bound, not the ratified tolerance (rev-28 ratification is the
        // H2637/Case-class evidence).
        assert!(l1 < 0.5, "hybrid per-bin L1 vs explicit: {l1}");
        // Totals close (both conservative; differences are end-storage).
        let hybrid_total: f64 = hybrid.outlet_bin_outflow_m2.iter().sum();
        assert!(
            (hybrid_total - ref_total).abs() / ref_total < 0.05,
            "drained totals hybrid {hybrid_total} vs explicit {ref_total}"
        );
    }

    #[test]
    fn hybrid_rejects_non_integral_windows() {
        use super::super::seam::SEAM_HOUR_BINS;
        let segment = bare_segment(80.0, 10, 0.05, 500.0, 2.0);
        let source = [0.0_f64; SEAM_HOUR_BINS];
        let intensity = [0.0_f64; SEAM_HOUR_BINS];
        assert!(matches!(
            route_single_ofe_hybrid(
                &segment,
                &source,
                &intensity,
                None,
                &[],
                1000.0,
                900.0,
                300.0
            ),
            Err(RoutingError::DegenerateConfiguration)
        ));
    }

    #[test]
    fn degenerate_cascade_fails_closed() {
        let excess = |_o: usize, _c: usize, _t: f64| 0.0;
        let intensity = |_o: usize, _t: f64| 0.0;
        let forcing = CascadeForcing {
            rainfall_excess_m_s: &excess,
            rainfall_intensity_m_s: &intensity,
        };
        assert!(matches!(
            run_cascade(&[], &forcing, 100.0, 10.0, 2.0),
            Err(RoutingError::DegenerateConfiguration)
        ));
        let bad_width = vec![bare_segment(10.0, 10, 0.05, 100.0, 0.0)];
        assert!(matches!(
            run_cascade(&bad_width, &forcing, 100.0, 10.0, 2.0),
            Err(RoutingError::DegenerateConfiguration)
        ));
    }
}

#[cfg(test)]
mod rev30_deficit_carry_tests {
    use super::super::kinematic_wave::CellParameters;
    use super::*;

    fn bare_segment(length: f64, cells: usize, slope: f64, ko: f64, width: f64) -> CascadeSegment {
        CascadeSegment {
            mesh: KinematicWaveMesh::uniform(length, cells, CellParameters::bare(slope, ko)),
            width_m: width,
        }
    }

    /// Rev-30: `absorb_deficit` preserves the exact total
    /// (`booked + new_carry == mass + old_carry`), never books negative,
    /// and passes an unabsorbed remainder through zero bins unchanged.
    #[test]
    #[allow(clippy::float_cmp)] // exact-by-construction assertions (rev-30 exact-total rule)
    fn absorb_deficit_exact_total_and_non_negative() {
        // Full absorption.
        let mut carry = -2.0_f64;
        let booked = absorb_deficit(5.0, &mut carry);
        assert_eq!(booked, 3.0);
        assert_eq!(carry, 0.0);
        // Partial absorption: bin smaller than the deficit.
        let mut carry = -2.0_f64;
        let booked = absorb_deficit(0.5, &mut carry);
        assert_eq!(booked, 0.0);
        assert_eq!(carry, -1.5);
        // Pass-through on a zero bin.
        let mut carry = -1.5_f64;
        let booked = absorb_deficit(0.0, &mut carry);
        assert_eq!(booked, 0.0);
        assert_eq!(carry, -1.5);
        // No pending deficit: identity booking.
        let mut carry = 0.0_f64;
        let booked = absorb_deficit(7.25, &mut carry);
        assert_eq!(booked, 7.25);
        assert_eq!(carry, 0.0);
    }

    /// Rev-30: a MATERIAL end-of-window deficit fails closed
    /// (`NegativeOutletBin`) — the same posture the plain path keeps.
    #[test]
    fn dispose_terminal_carry_material_deficit_fails_closed() {
        let mut bins = vec![4.0_f64, 2.0, 1.0];
        let err = dispose_terminal_carry(&mut bins, -1.0e-3);
        assert!(matches!(err, Err(RoutingError::NegativeOutletBin)));
        // All-dry series with any beyond-floor deficit also fails closed.
        let mut dry = vec![0.0_f64; 4];
        assert!(matches!(
            dispose_terminal_carry(&mut dry, -1.0e-15),
            Err(RoutingError::NegativeOutletBin)
        ));
    }

    /// Rev-30: a sub-noise remainder is absorbed BACKWARD from the trailing
    /// positive bins — exact total, all bins non-negative, leading bins
    /// untouched.
    #[test]
    #[allow(clippy::float_cmp)] // exact-by-construction assertions (rev-30 exact-total rule)
    fn dispose_terminal_carry_subnoise_absorbs_backward_exactly() {
        let mut bins = vec![4.0_f64, 2.0, 1.0e-10, 0.0];
        let total_before: f64 = bins.iter().sum();
        // noise floor = 1e-9 * 6.0000000001 ~ 6e-9; carry below it.
        let carry = -2.0e-9_f64;
        dispose_terminal_carry(&mut bins, carry).expect("sub-noise absorbs");
        let total_after: f64 = bins.iter().sum();
        assert!(
            bins.iter().all(|v| *v >= 0.0),
            "bins non-negative: {bins:?}"
        );
        assert!(
            ((total_before + carry) - total_after).abs() < 1.0e-24,
            "exact total: before {total_before:e} carry {carry:e} after {total_after:e}"
        );
        // Backward: the tiny trailing bin zeroes first, the remainder comes
        // out of the last bin large enough to host it.
        assert_eq!(bins[3], 0.0);
        assert_eq!(bins[2], 0.0);
        assert!(bins[1] < 2.0 && bins[1] > 2.0 - 3.0e-9);
        assert_eq!(bins[0], 4.0);
        // Zero carry is a no-op.
        let mut untouched = vec![1.0_f64, 2.0];
        dispose_terminal_carry(&mut untouched, 0.0).expect("no-op");
        assert_eq!(untouched, vec![1.0, 2.0]);
    }

    /// Review C-L1 pin: on an ALL-ZERO series a sub-noise carry (under the
    /// `1e-21` absolute floor) cannot be absorbed — it is DROPPED as a
    /// bounded attribution slack: `Ok(())`, bins untouched (never published
    /// negative). Anything above the floor on the same series fails closed
    /// (covered by the material-deficit vector).
    #[test]
    #[allow(clippy::float_cmp)] // exact-by-construction assertions (rev-30 exact-total rule)
    fn dispose_terminal_carry_all_dry_subnoise_drop_is_bounded() {
        let mut bins = vec![0.0_f64, 0.0];
        dispose_terminal_carry(&mut bins, -5.0e-22).expect("bounded all-dry drop");
        assert_eq!(bins, vec![0.0, 0.0], "bins untouched, never negative");
    }

    /// Review C-M1 pin: the aggressive mask's bin-start source sample is
    /// exact only when every bin lies within one seam hour. A cadence that
    /// does NOT partition the hour (here 1000 s: bin [3000,4000) straddles
    /// the hour-0→1 source turn-on at 3600 s) must fail closed instead of
    /// routing the straddling bin implicitly with a stale zero sample.
    #[test]
    fn hybrid_rejects_cadence_that_does_not_partition_the_seam_hour() {
        let segment = bare_segment(80.0, 10, 0.05, 500.0, 2.0);
        let mut source = [0.0_f64; SEAM_HOUR_BINS];
        source[1] = 60.0 / 3.6e6; // hour 0 silent, hour 1 active
        let intensity = [0.0_f64; SEAM_HOUR_BINS];
        assert!(matches!(
            route_single_ofe_hybrid(
                &segment,
                &source,
                &intensity,
                None,
                &[3600.0],
                4000.0,
                1000.0,
                300.0
            ),
            Err(RoutingError::DegenerateConfiguration)
        ));
    }

    /// Rev-30 AGGRESSIVE rule: zero-source bins step implicitly even when
    /// upstream inflow is present (the rev-28 strict rule forced them
    /// explicit). The upstream mass must be booked EXACTLY (interval mean
    /// per bin) and the composed ledger must close at machine precision.
    #[test]
    fn hybrid_aggressive_routes_upstream_fed_zero_source_bins_implicitly() {
        let segment = bare_segment(80.0, 10, 0.05, 500.0, 2.0);
        let source = [0.0_f64; SEAM_HOUR_BINS];
        let intensity = [0.0_f64; SEAM_HOUR_BINS];
        // Upstream drain: 8 x 900 s bins of decaying mass (m^2 per bin at
        // the upstream OFE's width, same width here so ratio = 1).
        let up_bins: Vec<f64> = (0..8).map(|k| 0.4 / f64::from(1 << k)).collect();
        let upstream_total: f64 = up_bins.iter().sum();
        let upstream = UpstreamHandoff {
            samples: Vec::new(),
            bins_m2: up_bins,
            bin_spans_s: vec![900.0; 8],
            bin_dt_s: 900.0,
            width_m: 2.0,
        };
        let window_s = 2.0 * 3600.0;
        // Process-global enable flag: hold the guard for the full body so
        // concurrent profile tests cannot race (see profile::test_flag_guard).
        let _flag_guard = profile::test_flag_guard();
        profile::set_enabled(true);
        let _ = profile::snapshot_and_reset();
        let result = route_single_ofe_hybrid(
            &segment,
            &source,
            &intensity,
            Some(&upstream),
            &[],
            window_s,
            900.0,
            300.0,
        )
        .expect("aggressive hybrid route");
        let snapshot = profile::snapshot_and_reset();
        profile::set_enabled(false);
        // Every bin is zero-source => every bin steps implicitly (one step
        // per 900 s bin), no explicit sub-solver runs.
        assert_eq!(snapshot.solver_steps_implicit, 8, "all bins implicit");
        assert_eq!(snapshot.solver_steps, 8, "no explicit steps");
        // Upstream mass booked exactly.
        assert!(
            (result.mass_balance.inflow_m2 - upstream_total).abs() <= 1.0e-12 * upstream_total,
            "booked inflow {} vs upstream total {upstream_total}",
            result.mass_balance.inflow_m2
        );
        // Composed ledger closes at machine precision and the bin series
        // carries exactly the booked outflow.
        let residual = result.mass_balance.conservation_residual_m2();
        assert!(
            residual.abs() <= 1.0e-12 * upstream_total,
            "ledger residual {residual:e}"
        );
        let bin_total: f64 = result.outlet_bin_outflow_m2.iter().sum();
        assert!(
            (bin_total - result.mass_balance.outflow_m2).abs()
                <= 1.0e-12 * upstream_total.max(1.0e-12),
            "bin series total {bin_total:e} vs booked outflow {:e}",
            result.mass_balance.outflow_m2
        );
        assert!(result.outlet_bin_outflow_m2.iter().all(|v| *v >= 0.0));
    }
}
