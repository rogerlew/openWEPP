//! D10B seam-decomposed conservation ledger (`SC-OFEROUTE-001` /
//! `GAP-OFEROUTE-005` diagnostics; package
//! `20260706-mofefid-d10b-gap005-source-authority-reconciliation-001`).
//!
//! Runs an H2637-shaped steep 19-OFE cascade with an hourly-pulse source
//! across a `(sample_dt, max_dt)` grid (including the three recorded shadow
//! sweep points) and decomposes the run-level cascade conservation residual
//! into named seam terms:
//!
//! - `ofe_internal_m3`: sum of per-OFE solver ledger residuals
//!   (`inflow + rain - outflow - storage`, clamp-adjusted) — the solver's own
//!   discretization/ledger mismatch.
//! - `seam_transfer_identity_m3`: per seam, the upstream solver's booked
//!   ledger outflow minus the cascade's booked per-OFE outlet volume.
//!   POST-rev-26 SEMANTICS (Codex review Medium-2): `per_ofe_outlet_m3`
//!   IS the ledger outflow, so this term is a structural identity
//!   (zero by construction), NOT sampled-quadrature evidence. The
//!   PRE-rev-26 runs of this example measured the then-sampled-trapezoid
//!   field and those historical numbers retain their original
//!   quadrature meaning.
//! - `seam_injection_m3`: per seam, the booked per-OFE outlet volume minus
//!   what the downstream solver booked as received inflow — the handoff
//!   transfer error (zero post-rev-26 by the conservative bin series).
//! - `terminal_booking_identity_m3`: terminal OFE ledger outflow minus the
//!   cascade's outlet booking — likewise a structural identity
//!   post-rev-26.
//! - `terminal_sampled_quadrature_m3`: an EXPLICIT sampled-quadrature
//!   diagnostic (trapezoid of the exported terminal bin-mean hydrograph
//!   minus the booked outlet mass) — the surviving measurement of what a
//!   sample-grid quadrature would mis-state relative to booked mass.
//!
//! Identity check: the cascade residual equals
//! `ofe_internal + seam_transfer_identity + seam_injection +
//! terminal_booking_identity` (all in m^3), so the decomposition is
//! complete, not approximate.

#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::too_many_lines,
    clippy::needless_range_loop
)]

use openwepp_hillslope_orchestrator::ofe_routing::cascade::{
    CascadeForcing, CascadeSegment, run_cascade,
};
use openwepp_hillslope_orchestrator::ofe_routing::kinematic_wave::{
    CellParameters, KinematicWaveMesh,
};

/// H2637-shaped fixture: 19 OFEs, steep forest-class gradients, ~20 m slope
/// lengths, 10 cells/OFE (the shadow working resolution), bare skin friction.
fn h2637_shaped_segments() -> Vec<CascadeSegment> {
    let ofe_count = 19usize;
    (0..ofe_count)
        .map(|i| {
            // gradients sweep 0.25..0.61 downslope (steep forest regime)
            let frac = i as f64 / (ofe_count - 1) as f64;
            let slope = 0.25 + 0.36 * frac;
            CascadeSegment {
                mesh: KinematicWaveMesh::uniform(20.0, 10, CellParameters::bare(slope, 500.0)),
                width_m: 30.0,
            }
        })
        .collect()
}

fn main() {
    let segments = h2637_shaped_segments();
    // Event-day-shaped source: 3 h of runoff-equivalent supply concentrated
    // in hours 6..9 (2, 6, 2 mm), zero elsewhere; window = last active hour
    // + 6 h drain (the shadow's clip rule).
    let hourly_depth_m = |hour: usize| -> f64 {
        match hour {
            6 | 8 => 0.002,
            7 => 0.006,
            _ => 0.0,
        }
    };
    let excess = |_ofe: usize, _cell: usize, t: f64| {
        let hour = (t / 3600.0).floor() as usize;
        if hour < 24 {
            hourly_depth_m(hour) / 3600.0
        } else {
            0.0
        }
    };
    let intensity = |_ofe: usize, _t: f64| 0.0;
    let forcing = CascadeForcing {
        rainfall_excess_m_s: &excess,
        rainfall_intensity_m_s: &intensity,
    };
    let window_s = 9.0 * 3600.0 + 6.0 * 3600.0;

    let grid: [(f64, f64); 6] = [
        (900.0, 300.0),
        (900.0, 120.0),
        (120.0, 300.0),
        (120.0, 120.0),
        (60.0, 30.0),
        (15.0, 5.0),
    ];

    println!("[");
    for (idx, (sample_dt, max_dt)) in grid.iter().enumerate() {
        let res =
            run_cascade(&segments, &forcing, window_s, *sample_dt, *max_dt).expect("cascade run");
        let n = segments.len();

        // Per-OFE internal residuals (clamp-adjusted), width-scaled to m^3.
        let mut ofe_internal_m3 = 0.0;
        // Booked-vs-scheme flux mismatches summed over OFEs (m^3): what the
        // ledger books minus what the discrete scheme actually moved.
        let mut inflow_booking_m3 = 0.0;
        let mut outflow_booking_m3 = 0.0;
        let mut tvd_leak_m3 = 0.0;
        // Scheme identity residual: rain + scheme_in - scheme_out + tvd_leak
        // + clamp - storage_change; ~0 proves the scheme's actual fluxes
        // close exactly and the booked ledger is the mismatch.
        let mut scheme_identity_m3 = 0.0;
        for (i, mb) in res.per_ofe_solver_mass.iter().enumerate() {
            let w = segments[i].width_m;
            ofe_internal_m3 += mb.conservation_residual_m2() * w;
            inflow_booking_m3 += (mb.inflow_m2 - mb.scheme_inflow_m2) * w;
            outflow_booking_m3 += (mb.outflow_m2 - mb.scheme_outflow_m2) * w;
            tvd_leak_m3 += mb.tvd_boundary_leak_m2 * w;
            scheme_identity_m3 += (mb.rainfall_excess_m2 + mb.scheme_inflow_m2
                - mb.scheme_outflow_m2
                + mb.tvd_boundary_leak_m2
                + mb.positivity_clamp_m2
                - mb.storage_change_m2)
                * w;
        }
        // Seam terms per interior handoff i -> i+1.
        let mut seam_transfer_identity_m3 = 0.0;
        let mut seam_injection_m3 = 0.0;
        for i in 0..n - 1 {
            let ledger_outflow_m3 = res.per_ofe_solver_mass[i].outflow_m2 * segments[i].width_m;
            let sampled_m3 = res.per_ofe_outlet_m3[i];
            let injected_m3 = res.per_ofe_received_upstream_m3[i + 1];
            seam_transfer_identity_m3 += ledger_outflow_m3 - sampled_m3;
            seam_injection_m3 += sampled_m3 - injected_m3;
        }
        // Terminal outlet: booking identity (ledger vs cascade booking) +
        // the explicit sampled-quadrature diagnostic (exported hydrograph
        // trapezoid vs booked mass).
        let terminal_ledger_m3 =
            res.per_ofe_solver_mass[n - 1].outflow_m2 * segments[n - 1].width_m;
        let terminal_booking_identity_m3 = terminal_ledger_m3 - res.mass_balance.outlet_m3;
        let mut hydro_trapezoid_m2 = 0.0_f64;
        for w in res.outlet_hydrograph.windows(2) {
            let dt = w[1].time_s - w[0].time_s;
            if dt > 0.0 {
                hydro_trapezoid_m2 +=
                    0.5 * (w[0].outlet_unit_discharge_m2_s + w[1].outlet_unit_discharge_m2_s) * dt;
            }
        }
        let terminal_sampled_quadrature_m3 =
            hydro_trapezoid_m2 * segments[n - 1].width_m - res.mass_balance.outlet_m3;

        let residual_m3 = res.mass_balance.conservation_residual_m3();
        let decomposed_m3 = ofe_internal_m3
            + seam_transfer_identity_m3
            + seam_injection_m3
            + terminal_booking_identity_m3;
        let rain_m3 = res.mass_balance.rainfall_excess_m3;
        let comma = if idx + 1 < grid.len() { "," } else { "" };
        println!(
            concat!(
                "  {{\"sample_dt_s\": {}, \"max_dt_s\": {}, ",
                "\"rain_m3\": {:.6}, \"residual_m3\": {:.6}, ",
                "\"residual_rel\": {:.6}, ",
                "\"ofe_internal_m3\": {:.6}, \"seam_transfer_identity_m3\": {:.6}, ",
                "\"seam_injection_m3\": {:.6}, \"terminal_booking_identity_m3\": {:.6}, ",
                "\"terminal_sampled_quadrature_m3\": {:.6}, ",
                "\"decomposition_gap_m3\": {:.3e}, ",
                "\"inflow_booking_m3\": {:.6}, \"outflow_booking_m3\": {:.6}, ",
                "\"tvd_leak_m3\": {:.6}, \"scheme_identity_m3\": {:.3e}, ",
                "\"max_courant\": {:.4}}}{}"
            ),
            sample_dt,
            max_dt,
            rain_m3,
            residual_m3,
            residual_m3.abs() / rain_m3,
            ofe_internal_m3,
            seam_transfer_identity_m3,
            seam_injection_m3,
            terminal_booking_identity_m3,
            terminal_sampled_quadrature_m3,
            (residual_m3 - decomposed_m3).abs(),
            inflow_booking_m3,
            outflow_booking_m3,
            tvd_leak_m3,
            scheme_identity_m3,
            res.max_courant,
            comma
        );
    }
    println!("]");
}
