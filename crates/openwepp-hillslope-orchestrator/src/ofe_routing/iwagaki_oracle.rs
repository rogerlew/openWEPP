//! D10B Iwagaki Case-4 characteristics oracle (`SC-OFEROUTE-001` rev 24,
//! `INV-OFEROUTE-011`; package
//! `20260706-mofefid-d10b-gap005-source-authority-reconciliation-001`).
//!
//! Reference (non-production) computation of the exact entropy solution of
//! the wide-channel Manning kinematic-wave equation for the Iwagaki 1955
//! experiment-(B) configuration (REF-OFEROUTE-SHOCK-IWAGAKI):
//!
//!   `h_t + q_x = v(x, t)`,  `q = alpha(x) h^m`,  `m = 5/3`,
//!   `alpha_j = sqrt(S_j) / n`  per 8 m reach, `n = 0.009` (m-s units),
//!   S = [0.020, 0.015, 0.010], v = [1.080e-3, 0.638e-3, 0.800e-3] m/s
//!   for 0 <= t <= T (duration case (c), T = 10 s), zero after.
//!
//! Method of characteristics as a Lagrangian particle fan: along a
//! characteristic `dh/dt = v` and `dx/dt = c(h) = m alpha h^(m-1)`
//! (Iwagaki's eqs. for the turbulent/Manning branch; wide channel R = h).
//! Within a reach and a supply phase the in-step update is exact:
//! during supply `x_new - x_old = (q(h_new) - q(h_old)) / v`; after cutoff
//! characteristics are straight lines. Kinematic shocks (born at the reach
//! junctions where deeper immigrant characteristics overtake shallower
//! native ones, and after cutoff) are tracked explicitly with the
//! Rankine-Hugoniot speed `ds/dt = (q_L - q_R)/(h_L - h_R)`
//! (REF-OFEROUTE-KWE, Lighthill & Whitham 1955); particles overtaken by a
//! shock are absorbed and adjacent shocks merge.
//!
//! This module is validation-tier (same tier as `dval`): it is consumed by
//! the D-val harness and contract-derived tests, never by production
//! routing. Its own correctness evidence: exact closed-form anchors
//! (steady state, single-reach rising limb), continuous mass-conservation
//! bookkeeping, and self-convergence under fan/step refinement
//! (`numerics-convergence-evidence.md`).

/// Manning depth-discharge exponent for the wide-channel oracle.
pub const ORACLE_M: f64 = 5.0 / 3.0;

/// The Iwagaki 1955 primary's Manning roughness (m-s units), the single
/// source of truth shared by the oracle configuration and the D-val
/// solver path (Codex review High-2).
pub const IWAGAKI_MANNING_N: f64 = 0.009;

/// One reach of the piecewise-uniform channel.
#[derive(Debug, Clone, Copy)]
pub struct OracleReach {
    /// Downstream end of the reach (m); reaches partition `[0, x_end)`.
    pub x_end_m: f64,
    /// Kinematic coefficient `alpha = sqrt(S)/n` (m^(1/3)/s for m = 5/3).
    pub alpha: f64,
    /// Lateral supply rate during the supply phase (m/s).
    pub supply_m_s: f64,
}

/// Oracle configuration.
#[derive(Debug, Clone)]
pub struct OracleConfig {
    pub reaches: Vec<OracleReach>,
    /// Supply cutoff time `T` (s).
    pub supply_end_s: f64,
    /// Output window end (s).
    pub end_time_s: f64,
    /// Number of characteristic particles seeded at `t = 0`.
    pub particles: usize,
    /// Global integration step (s).
    pub dt_s: f64,
}

impl OracleConfig {
    /// The Iwagaki experiment-(B), duration case (c) `T = 10 s`
    /// configuration bound by `SC-OFEROUTE-001` rev 24 (`n = 0.009`).
    #[must_use]
    pub fn iwagaki_case4() -> Self {
        let n = IWAGAKI_MANNING_N;
        Self {
            reaches: vec![
                OracleReach {
                    x_end_m: 8.0,
                    alpha: 0.020_f64.sqrt() / n,
                    supply_m_s: 0.108e-2,
                },
                OracleReach {
                    x_end_m: 16.0,
                    alpha: 0.015_f64.sqrt() / n,
                    supply_m_s: 0.0638e-2,
                },
                OracleReach {
                    x_end_m: 24.0,
                    alpha: 0.010_f64.sqrt() / n,
                    supply_m_s: 0.080e-2,
                },
            ],
            supply_end_s: 10.0,
            end_time_s: 80.0,
            particles: 1200,
            dt_s: 2.0e-3,
        }
    }
}

/// One point of the oracle outlet hydrograph.
#[derive(Debug, Clone, Copy)]
pub struct OracleSample {
    pub time_s: f64,
    pub q_m2_s: f64,
}

/// Result of an oracle run.
#[derive(Debug, Clone)]
pub struct OracleResult {
    /// Outlet unit-discharge hydrograph (crossing-ordered, may contain
    /// same-time pairs where a shock exits the domain).
    pub hydrograph: Vec<OracleSample>,
    pub peak_m2_s: f64,
    pub time_to_peak_s: f64,
    /// 10-90% rising-limb duration (s), if resolvable.
    pub rise_10_90_s: Option<f64>,
    /// Mass check: total outlet volume + final storage - total supplied
    /// volume, relative to supplied (should be ~0).
    pub mass_residual_rel: f64,
    /// Number of shocks tracked over the run (diagnostic).
    pub shocks_tracked: usize,
}

#[derive(Debug, Clone, Copy)]
enum Node {
    /// A characteristic particle carrying depth `h`.
    Particle { x: f64, h: f64 },
    /// A tracked shock with left/right depths.
    Shock { x: f64, h_left: f64, h_right: f64 },
}

impl Node {
    fn x(&self) -> f64 {
        match self {
            Node::Particle { x, .. } | Node::Shock { x, .. } => *x,
        }
    }
}

fn reach_index(reaches: &[OracleReach], x: f64) -> usize {
    for (j, r) in reaches.iter().enumerate() {
        if x < r.x_end_m {
            return j;
        }
    }
    reaches.len() - 1
}

fn q_of(alpha: f64, h: f64) -> f64 {
    if h <= 0.0 {
        0.0
    } else {
        alpha * h.powf(ORACLE_M)
    }
}

fn celerity(alpha: f64, h: f64) -> f64 {
    if h <= 0.0 {
        0.0
    } else {
        ORACLE_M * alpha * h.powf(ORACLE_M - 1.0)
    }
}

/// Depth continuation across a reach boundary: `q` is continuous, so
/// `h_new = (q / alpha_new)^(1/m)`.
fn continue_depth(q: f64, alpha_new: f64) -> f64 {
    if q <= 0.0 {
        0.0
    } else {
        (q / alpha_new).powf(1.0 / ORACLE_M)
    }
}

/// Run the characteristics oracle. Returns the outlet hydrograph as the
/// ordered series of particle/shock crossings of the domain outlet.
#[must_use]
#[allow(clippy::too_many_lines, clippy::cast_precision_loss)]
pub fn run_oracle(config: &OracleConfig) -> OracleResult {
    let reaches = &config.reaches;
    let domain_end = reaches[reaches.len() - 1].x_end_m;
    let dt = config.dt_s;
    let mut t = 0.0_f64;

    // Seed the particle fan uniformly at t = 0 (dry bed).
    let mut nodes: Vec<Node> = (0..config.particles)
        .map(|k| Node::Particle {
            x: (k as f64 + 0.5) * domain_end / config.particles as f64,
            h: 0.0,
        })
        .collect();

    let mut hydrograph: Vec<OracleSample> = vec![OracleSample {
        time_s: 0.0,
        q_m2_s: 0.0,
    }];
    let mut outlet_volume_m2 = 0.0_f64;
    let mut last_outlet_q = 0.0_f64;
    let mut last_outlet_t = 0.0_f64;
    let mut shocks_tracked = 0usize;
    let mut rarefaction_seeded = false;
    // Continuous inflow-boundary seeding cadence: characteristics emanate
    // from x = 0 for all t (h = 0 at birth, growing under the local
    // supply); without them the fan thins upstream and the trailing limb
    // is unrepresented.
    let seed_every = 25usize;
    let mut step_index = 0usize;

    while t < config.end_time_s {
        let supply_on = t < config.supply_end_s;
        let step = dt.min(config.end_time_s - t);
        if supply_on && step_index % seed_every == 0 {
            nodes.insert(0, Node::Particle { x: 0.0, h: 0.0 });
        }
        if !supply_on && !rarefaction_seeded {
            // Cutoff boundary fan (review-B m7 requalified): the trailing
            // limb is primarily carried by the continuous inflow-boundary
            // seeding above (the first cut's -66% mass residual came from
            // MISSING BOUNDARY SEEDING, not from a missing centered
            // rarefaction — with zero upstream inflow, h(0, t) = 0
            // continuously and no boundary discontinuity forms at cutoff).
            // This fan densifies the near-boundary depth range at cutoff
            // as a resolution aid; as coded it spans the youngest seed's
            // depth and is a near-no-op, retained for fan density only.
            let h_plateau = nodes
                .iter()
                .find_map(|n| match n {
                    Node::Particle { h, .. } if *h > 0.0 => Some(*h),
                    _ => None,
                })
                .unwrap_or(0.0);
            let fan = 240usize;
            let fan_nodes: Vec<Node> = (1..=fan)
                .map(|k| Node::Particle {
                    x: 1.0e-9 * k as f64,
                    h: h_plateau * k as f64 / (fan as f64 + 1.0),
                })
                .collect();
            nodes.splice(0..0, fan_nodes);
            rarefaction_seeded = true;
        }
        step_index += 1;

        // Advance every node one step (exact in-reach updates; reach
        // crossings incur O(dt) error covered by self-convergence).
        for node in &mut nodes {
            match node {
                Node::Particle { x, h } => {
                    let j = reach_index(reaches, *x);
                    let alpha = reaches[j].alpha;
                    let v = if supply_on {
                        reaches[j].supply_m_s
                    } else {
                        0.0
                    };
                    let (x_new, h_new) = if v > 0.0 {
                        let h2 = *h + v * step;
                        let x2 = *x + (q_of(alpha, h2) - q_of(alpha, *h)) / v;
                        (x2, h2)
                    } else {
                        (*x + celerity(alpha, *h) * step, *h)
                    };
                    // Reach crossing: q continuous, h re-mapped to the new
                    // reach's alpha.
                    let j_new = reach_index(reaches, x_new);
                    let h_final = if j_new != j && x_new < domain_end {
                        continue_depth(q_of(alpha, h_new), reaches[j_new].alpha)
                    } else {
                        h_new
                    };
                    *x = x_new;
                    *h = h_final;
                }
                Node::Shock { x, h_left, h_right } => {
                    let j = reach_index(reaches, *x);
                    let alpha = reaches[j].alpha;
                    let v = if supply_on {
                        reaches[j].supply_m_s
                    } else {
                        0.0
                    };
                    // Both sides gain supply; RH speed from current states.
                    let hl = *h_left + v * step;
                    let hr = *h_right + v * step;
                    let denom = hl - hr;
                    let speed = if denom.abs() <= 1.0e-14 {
                        celerity(alpha, hl)
                    } else {
                        (q_of(alpha, hl) - q_of(alpha, hr)) / denom
                    };
                    let x_new = *x + speed * step;
                    let j_new = reach_index(reaches, x_new);
                    let (hl2, hr2) = if j_new != j && x_new < domain_end {
                        (
                            continue_depth(q_of(alpha, hl), reaches[j_new].alpha),
                            continue_depth(q_of(alpha, hr), reaches[j_new].alpha),
                        )
                    } else {
                        (hl, hr)
                    };
                    *x = x_new;
                    *h_left = hl2;
                    *h_right = hr2;
                }
            }
        }
        t += step;

        // Resolve crossings: sort-order violations become shocks; shocks
        // absorb overtaken neighbors; adjacent shocks merge.
        let mut i = 0usize;
        while i + 1 < nodes.len() {
            let (xi, xj) = (nodes[i].x(), nodes[i + 1].x());
            if xj >= xi {
                i += 1;
                continue;
            }
            // nodes[i] (upstream) has overtaken nodes[i+1]: collapse into a
            // shock at the midpoint with left state from upstream, right
            // state from downstream.
            let merged = match (nodes[i], nodes[i + 1]) {
                (Node::Particle { h: hl, .. }, Node::Particle { h: hr, .. }) => Node::Shock {
                    x: 0.5 * (xi + xj),
                    h_left: hl,
                    h_right: hr,
                },
                (Node::Shock { h_left, .. }, Node::Particle { h: hr, .. }) => Node::Shock {
                    x: 0.5 * (xi + xj),
                    h_left,
                    h_right: hr,
                },
                (Node::Particle { h: hl, .. }, Node::Shock { h_right, .. }) => Node::Shock {
                    x: 0.5 * (xi + xj),
                    h_left: hl,
                    h_right,
                },
                (Node::Shock { h_left, .. }, Node::Shock { h_right, .. }) => Node::Shock {
                    x: 0.5 * (xi + xj),
                    h_left,
                    h_right,
                },
            };
            if matches!(
                (nodes[i], nodes[i + 1]),
                (Node::Particle { .. }, Node::Particle { .. })
            ) {
                shocks_tracked += 1;
            }
            nodes[i] = merged;
            nodes.remove(i + 1);
            // re-check the upstream neighbor against the merged node
            i = i.saturating_sub(1);
        }

        // Outlet crossings: nodes past the domain end exit and stamp the
        // hydrograph with their discharge (terminal reach alpha).
        let alpha_out = reaches[reaches.len() - 1].alpha;
        let mut exited: Vec<(f64, f64)> = Vec::new();
        nodes.retain(|node| {
            let keep = node.x() < domain_end;
            if !keep {
                match node {
                    Node::Particle { h, .. } => exited.push((t, q_of(alpha_out, *h))),
                    Node::Shock {
                        h_left, h_right, ..
                    } => {
                        // Shock exit: the hydrograph jumps from the right
                        // (pre-shock) to the left (post-shock) state.
                        exited.push((t, q_of(alpha_out, *h_right)));
                        exited.push((t, q_of(alpha_out, *h_left)));
                    }
                }
            }
            keep
        });
        for (te, qe) in exited {
            // Trapezoidal outlet volume between successive crossings.
            outlet_volume_m2 += 0.5 * (last_outlet_q + qe) * (te - last_outlet_t);
            last_outlet_q = qe;
            last_outlet_t = te;
            hydrograph.push(OracleSample {
                time_s: te,
                q_m2_s: qe,
            });
        }
    }

    // Final storage from the surviving fan (trapezoid over particle
    // positions; shocks contribute their mean state at a point measure 0).
    let mut storage_m2 = 0.0_f64;
    let mut prev_x = 0.0_f64;
    let mut prev_h = nodes.first().map_or(0.0, |n| match n {
        Node::Particle { h, .. } => *h,
        Node::Shock { h_left, .. } => *h_left,
    });
    for node in &nodes {
        let (x, h) = match node {
            Node::Particle { x, h } => (*x, *h),
            Node::Shock { x, h_left, h_right } => (*x, 0.5 * (h_left + h_right)),
        };
        storage_m2 += 0.5 * (prev_h + h) * (x - prev_x).max(0.0);
        prev_x = x;
        prev_h = h;
    }
    storage_m2 += prev_h * (domain_end - prev_x).max(0.0);

    let supplied_m2: f64 = config
        .reaches
        .iter()
        .scan(0.0_f64, |x0, r| {
            let len = r.x_end_m - *x0;
            *x0 = r.x_end_m;
            Some(r.supply_m_s * len * config.supply_end_s.min(config.end_time_s))
        })
        .sum();
    let mass_residual_rel = (outlet_volume_m2 + storage_m2 - supplied_m2) / supplied_m2;

    // Peak + rise metrics from the crossing hydrograph.
    let (mut peak, mut t_peak) = (0.0_f64, 0.0_f64);
    for s in &hydrograph {
        if s.q_m2_s > peak {
            peak = s.q_m2_s;
            t_peak = s.time_s;
        }
    }
    let rise_10_90_s = rise_10_90(&hydrograph, peak, t_peak);

    OracleResult {
        hydrograph,
        peak_m2_s: peak,
        time_to_peak_s: t_peak,
        rise_10_90_s,
        mass_residual_rel,
        shocks_tracked,
    }
}

fn crossing_time(samples: &[OracleSample], target: f64) -> Option<f64> {
    for w in samples.windows(2) {
        let (a, b) = (w[0], w[1]);
        if (a.q_m2_s <= target && b.q_m2_s >= target) || (a.q_m2_s >= target && b.q_m2_s <= target)
        {
            let dq = b.q_m2_s - a.q_m2_s;
            if dq.abs() <= f64::EPSILON {
                return Some(a.time_s);
            }
            return Some(a.time_s + (target - a.q_m2_s) / dq * (b.time_s - a.time_s));
        }
    }
    None
}

fn rise_10_90(hydrograph: &[OracleSample], peak: f64, t_peak: f64) -> Option<f64> {
    if peak <= 0.0 {
        return None;
    }
    let rising: Vec<OracleSample> = hydrograph
        .iter()
        .copied()
        .take_while(|s| s.time_s <= t_peak)
        .collect();
    let t10 = crossing_time(&rising, 0.1 * peak)?;
    let t90 = crossing_time(&rising, 0.9 * peak)?;
    Some(t90 - t10)
}

/// Conservative first-order upwind reference solution of the same PDE.
///
/// Monotone finite-volume scheme (all kinematic celerities are
/// non-negative): `h_i^{n+1} = h_i - (dt/dx)(q_i - q_{i-1}) + v_i dt`,
/// inflow flux zero, outflow flux `q_{N-1}`. For a uniform-coefficient
/// scalar conservation law, monotone schemes provably converge to the
/// unique entropy solution — the same solution the method of
/// characteristics + Rankine-Hugoniot shock fitting constructs; for the
/// piecewise-constant-alpha reaches here the interface flux is continuous
/// and the independent characteristics-fan cross-validation carries the
/// convergence claim (review-B m11). The finite-volume form is exactly
/// mass-conservative by construction. This is the PRIMARY acceptance
/// oracle; the Lagrangian fan above cross-validates it.
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn run_upwind_reference(config: &OracleConfig, cells: usize) -> OracleResult {
    let reaches = &config.reaches;
    let domain_end = reaches[reaches.len() - 1].x_end_m;
    let dx = domain_end / cells as f64;
    // Per-cell alpha / supply by reach membership (cell centers).
    let mut alpha = vec![0.0_f64; cells];
    let mut supply = vec![0.0_f64; cells];
    for i in 0..cells {
        let x = (i as f64 + 0.5) * dx;
        let j = reach_index(reaches, x);
        alpha[i] = reaches[j].alpha;
        supply[i] = reaches[j].supply_m_s;
    }
    let mut h = vec![0.0_f64; cells];
    let mut q = vec![0.0_f64; cells];
    let mut t = 0.0_f64;
    let mut hydrograph = vec![OracleSample {
        time_s: 0.0,
        q_m2_s: 0.0,
    }];
    let mut outlet_volume_m2 = 0.0_f64;
    let cfl = 0.45_f64;
    let mut supplied_m2 = 0.0_f64;
    // Record the outlet at a fine uniform cadence for metric extraction.
    let record_dt = 0.02_f64;
    let mut next_record = record_dt;

    while t < config.end_time_s {
        let supply_on = t < config.supply_end_s;
        // CFL dt from the true Manning celerity.
        let mut c_max = 0.0_f64;
        for i in 0..cells {
            q[i] = q_of(alpha[i], h[i]);
            let c = celerity(alpha[i], h[i]);
            if c > c_max {
                c_max = c;
            }
        }
        let mut dt = if c_max > 0.0 { cfl * dx / c_max } else { 0.05 };
        dt = dt.min(config.end_time_s - t);
        if supply_on {
            dt = dt.min(config.supply_end_s - t).max(1.0e-9);
        }
        let q_out = q[cells - 1];
        for i in (1..cells).rev() {
            let v = if supply_on { supply[i] } else { 0.0 };
            h[i] += -(dt / dx) * (q[i] - q[i - 1]) + v * dt;
        }
        let v0 = if supply_on { supply[0] } else { 0.0 };
        h[0] += -(dt / dx) * q[0] + v0 * dt; // inflow flux zero
        if supply_on {
            supplied_m2 += supply.iter().sum::<f64>() * dx * dt;
        }
        outlet_volume_m2 += q_out * dt;
        t += dt;
        while next_record <= t + 1.0e-12 && next_record <= config.end_time_s {
            hydrograph.push(OracleSample {
                time_s: next_record,
                q_m2_s: q_of(alpha[cells - 1], h[cells - 1]),
            });
            next_record += record_dt;
        }
    }

    let storage_m2: f64 = h.iter().sum::<f64>() * dx;
    let mass_residual_rel = (outlet_volume_m2 + storage_m2 - supplied_m2) / supplied_m2;
    let (mut peak, mut t_peak) = (0.0_f64, 0.0_f64);
    for s in &hydrograph {
        if s.q_m2_s > peak {
            peak = s.q_m2_s;
            t_peak = s.time_s;
        }
    }
    let rise_10_90_s = rise_10_90(&hydrograph, peak, t_peak);
    OracleResult {
        hydrograph,
        peak_m2_s: peak,
        time_to_peak_s: t_peak,
        rise_10_90_s,
        mass_residual_rel,
        shocks_tracked: 0,
    }
}

/// Interpolate the oracle hydrograph onto a uniform sample grid for
/// pointwise comparison against solver output.
#[must_use]
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
pub fn resample(hydrograph: &[OracleSample], sample_dt_s: f64, end_time_s: f64) -> Vec<f64> {
    let count = (end_time_s / sample_dt_s).round() as usize;
    let mut out = Vec::with_capacity(count + 1);
    let mut k = 0usize;
    for i in 0..=count {
        let t = i as f64 * sample_dt_s;
        while k + 1 < hydrograph.len() && hydrograph[k + 1].time_s <= t {
            k += 1;
        }
        let q = if k + 1 >= hydrograph.len() {
            hydrograph[k].q_m2_s
        } else {
            let (a, b) = (hydrograph[k], hydrograph[k + 1]);
            let span = b.time_s - a.time_s;
            if span <= 0.0 {
                b.q_m2_s
            } else {
                a.q_m2_s + (t - a.time_s) / span * (b.q_m2_s - a.q_m2_s)
            }
        };
        out.push(q);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // Closed-form anchor for the PRIMARY (upwind) oracle: a single uniform
    // reach with constant supply reaches steady state q = v L at the
    // outlet, with the exact rising limb q(t) = alpha (v t)^m until the
    // concentration time.
    #[test]
    fn upwind_single_reach_matches_closed_form() {
        let n = 0.009;
        let alpha = 0.015_f64.sqrt() / n;
        let v = 0.0833e-2; // Iwagaki experiment (A) supply
        let config = OracleConfig {
            reaches: vec![OracleReach {
                x_end_m: 24.0,
                alpha,
                supply_m_s: v,
            }],
            supply_end_s: 1.0e9, // no cutoff within the window
            end_time_s: 60.0,
            particles: 0,
            dt_s: 0.0,
        };
        let res = run_upwind_reference(&config, 4000);
        let t_c = (24.0 / (alpha * v.powf(ORACLE_M - 1.0))).powf(1.0 / ORACLE_M);
        assert!(t_c < 60.0, "concentration inside window: {t_c}");
        let q_steady = v * 24.0;
        let last = res.hydrograph.last().unwrap().q_m2_s;
        assert!(
            (last - q_steady).abs() / q_steady < 5.0e-3,
            "steady outlet {last} vs vL {q_steady}"
        );
        // exact rising limb at t_c/2: q = alpha (v t)^m
        let t_half = 0.5 * t_c;
        let expected = alpha * (v * t_half).powf(ORACLE_M);
        let sampled = resample(&res.hydrograph, t_half, t_half);
        let got = *sampled.last().unwrap();
        assert!(
            (got - expected).abs() / expected < 2.0e-2,
            "rising limb at {t_half}s: oracle {got} vs closed form {expected}"
        );
        assert!(
            res.mass_residual_rel.abs() < 1.0e-10,
            "finite-volume oracle must conserve exactly: {}",
            res.mass_residual_rel
        );
    }

    // The bound Case-4 configuration on the primary oracle: exact
    // conservation and a post-cutoff peak inside the window.
    #[test]
    fn upwind_case4_conserves_and_peaks_after_cutoff() {
        let res = run_upwind_reference(&OracleConfig::iwagaki_case4(), 4000);
        assert!(
            res.mass_residual_rel.abs() < 1.0e-10,
            "oracle mass residual {}",
            res.mass_residual_rel
        );
        assert!(res.peak_m2_s > 0.0);
        assert!(
            res.time_to_peak_s > 10.0 && res.time_to_peak_s < 60.0,
            "peak after cutoff, inside window: {}",
            res.time_to_peak_s
        );
    }

    // Self-convergence of the primary oracle under grid refinement: the
    // headline metrics must settle well inside the acceptance tolerances.
    #[test]
    fn upwind_case4_self_convergence() {
        let cfg = OracleConfig::iwagaki_case4();
        let coarse = run_upwind_reference(&cfg, 4000);
        let fine = run_upwind_reference(&cfg, 8000);
        let dpeak = (fine.peak_m2_s - coarse.peak_m2_s).abs() / fine.peak_m2_s;
        let dt_peak = (fine.time_to_peak_s - coarse.time_to_peak_s).abs();
        assert!(dpeak < 1.0e-2, "peak self-convergence {dpeak}");
        assert!(dt_peak < 0.5, "t_peak self-convergence {dt_peak}");
    }

    // Cross-validation: the independent Lagrangian characteristics fan
    // (with junction-shock tracking and the cutoff rarefaction) must agree
    // with the monotone finite-volume reference on the Case-4 headline
    // metrics. Two independent constructions of the same entropy solution
    // agreeing is the oracle's own correctness evidence.
    #[test]
    fn characteristics_fan_cross_validates_upwind_reference() {
        let cfg = OracleConfig::iwagaki_case4();
        let fv = run_upwind_reference(&cfg, 8000);
        let moc = run_oracle(&cfg);
        assert!(
            moc.mass_residual_rel.abs() < 2.0e-2,
            "fan mass residual {}",
            moc.mass_residual_rel
        );
        assert!(moc.shocks_tracked > 0, "junction shocks must form");
        let dpeak = (moc.peak_m2_s - fv.peak_m2_s).abs() / fv.peak_m2_s;
        assert!(
            dpeak < 3.0e-2,
            "fan vs upwind peak disagreement {dpeak} (fan {}, fv {})",
            moc.peak_m2_s,
            fv.peak_m2_s
        );
        let dt_peak = (moc.time_to_peak_s - fv.time_to_peak_s).abs();
        assert!(
            dt_peak < 1.0,
            "fan vs upwind t_peak disagreement {dt_peak}s (fan {}, fv {})",
            moc.time_to_peak_s,
            fv.time_to_peak_s
        );
    }
}
