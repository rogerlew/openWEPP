//! Wave-1 hillslope sediment-continuity spatial solver (single OFE).
//!
//! Source-intent port (ADR-0024, baseline `dac3c950`) of the legacy
//! normalized-space steady-state sediment-continuity solve:
//! `route.for` -> `erod.for`/`runge.for` (RK4 detachment march) +
//! `xcrit.for`/`root.for` (shear-regime classification) +
//! `depc.for`/`depeqs.for`/`depend.for`/`depos.for` (analytic deposition),
//! with the `param.for`/`xinflo.for` normalization and the `sloss.for`
//! denormalization identity. Governing contract: `SC-SED-001`
//! (INV-SED-001..003, -006, -007, -010). Legacy is source-intent authority,
//! never a magnitude oracle (ADR-0017).
//!
//! The whole solve runs in the legacy nondimensional space: `x in [0,1]`
//! over 101 grid points `x_i = i * 0.01`, sediment load `G` normalized by
//! `tcend`, drivers `eta/tauc/theta/phi`, shear polynomial
//! `tau_f = (a x^2 + b x + c)^(2/3)` and transport polynomial
//! `(atc x^2 + btc x + ctc) * ktrato`. Dimensional sediment is recovered at
//! the OFE exit via `tcend` (`sloss.for`).
//!
//! Grid indexing: legacy point `i` (1-based, `xinput(i) = (i-1)*0.01`)
//! maps to 0-based index `i - 1` here; legacy bound `i < 102` therefore
//! reads `index < 101` (`DIRECT_WAVE1_GRID_POINTS`).

use crate::constants::WB11_ZERO_THRESHOLD;

use super::{DirectRuntimeError, validate_finite, validate_nonnegative_direct_m};

/// Number of nondimensional grid points per OFE (`xinput(101)` in legacy).
pub const DIRECT_WAVE1_GRID_POINTS: usize = 101;
/// Default nondimensional march step (`runge.for` interior step).
const WAVE1_GRID_DX: f64 = 0.01;
/// Shear exponent used by the RK4 march (`runge.for`/`erod.for`
/// `exp(0.666667*log(xterm))`).
const WAVE1_MARCH_SHEAR_EXPONENT: f64 = 0.666_667;
/// Shear exponent used by the regime classifier (`shear.for` `0.66666667`).
const WAVE1_CLASSIFIER_SHEAR_EXPONENT: f64 = 0.666_666_67;
/// Classifier shear floor (`shear.for`: `if (shear.le.0.0) shear = 0.0001`).
const WAVE1_CLASSIFIER_SHEAR_FLOOR: f64 = 0.0001;
/// Uniform-segment critical-crossing sentinel (`xcrit.for:95`).
const WAVE1_XCRIT_UNIFORM_SENTINEL: f64 = 1000.0;
/// `route.for:169` near-zero `qostar` branch for the upper-boundary rate.
const WAVE1_QOSTAR_NEAR_ZERO: f64 = 0.0011;
/// `depc.for:42` / `depeqs.for:56` `10e-8` Fortran literal (= 1.0e-7).
const WAVE1_DEP_DENOMINATOR_EPSILON: f64 = 1.0e-7;
/// `depeqs.for:56` evaluation-point shift off the flow-end singularity.
const WAVE1_DEPEQS_X_SHIFT: f64 = 0.000_001;
/// `depend.for` Newton iteration cap.
const WAVE1_DEPEND_MAX_ITERS: usize = 10;
/// `depend.for:197` residual acceptance (`abs(f) <= 0.001`).
const WAVE1_DEPEND_RESIDUAL_TOL: f64 = 0.001;
/// `depend.for:96` increasing-flow initial trial step.
const WAVE1_DEPEND_STEP_POSITIVE: f64 = 0.01;
/// `depend.for:114` decreasing-flow initial trial step.
const WAVE1_DEPEND_STEP_NEGATIVE: f64 = 0.0001;
/// `depend.for:108` flow-end proximity guard on `xu + qostar`.
const WAVE1_DEPEND_XU_QOSTAR_NEAR_ZERO: f64 = 0.0001;
/// `erod.for` deposition-onset secant iteration cap (`i.lt.10`).
const WAVE1_ONSET_MAX_ITERS: usize = 10;
/// `erod.for:453/460` onset relative convergence tolerance.
const WAVE1_ONSET_REL_TOL: f64 = 0.001;
/// `erod.for:452/459` load / transport floors inside the onset iteration.
const WAVE1_ONSET_FLOOR: f64 = 0.000_01;
/// `undflo.for` power-underflow guard (`abs(expon*log10(factor)) > 30`).
const WAVE1_UNDFLO_POWER: f64 = 30.0;
/// `cross.for:43` degenerate-slope sentinel.
const WAVE1_CROSS_DEGENERATE_SLOPE: f64 = 1.0e6;
/// `param.for:234` transport-capacity normalization floor.
const WAVE1_TCEND_FLOOR: f64 = 1.0e-10;
/// `param.for:608` `pkro` near-zero threshold for the `phi` branch.
const WAVE1_PKRO_ZERO_THRESHOLD: f64 = 1.0e-15;
/// `param.for:624` deposition-parameter cap (`abs(phi) <= 100000`).
const WAVE1_MAX_PHI: f64 = 100_000.0;
/// `xinflo.for:154` discharge-delta threshold for `qostar`.
const WAVE1_QOSTAR_DELTA_THRESHOLD: f64 = 1.0e-10;
/// `xinflo.for:175` `qostar == -1` displacement.
const WAVE1_QOSTAR_MINUS_ONE_SHIFT: f64 = -1.001;
/// `xinflo.for:195` decreasing-flow `qostar` floor magnitude.
const WAVE1_QOSTAR_DECREASING_FLOOR: f64 = 0.000_01;
/// `contin.for:977` event-size bypass: runoff depth bound (m).
const WAVE1_PASSBY_RUNOFF_M: f64 = 0.010;
/// `contin.for:977` event-size bypass: peak runoff rate bound (m/s).
const WAVE1_PASSBY_PEAKRO_M_S: f64 = 2.78e-6;
/// SC-SED-001 sandy transport adjustment floor (INV-SED-006).
const WAVE1_MIN_TCADJF: f64 = 0.30;
/// Exact telescoping publication-closure tolerance (relative). The two
/// sides of the gate are the same telescoped sum evaluated two ways, so
/// this bound only absorbs floating-point accumulation order.
const WAVE1_PUBLICATION_CLOSURE_REL_TOL: f64 = 1.0e-9;
/// Continuity flux-closure tolerance (relative to total |dG|): bounds the
/// trapezoid-vs-RK4 discretization gap on interior unclamped cells
/// (INV-SED-001 residual gate; the residual itself is always reported).
/// This is a **discretization-consistency** bound, NOT the mass-balance
/// law — that is the separate `WAVE1_PUBLICATION_CLOSURE_REL_TOL` (1e-9)
/// telescoping-identity gate, which holds independently. The trapezoid is
/// a 2nd-order quadrature of the 4th-order RK4 march, so their gap is
/// inherently `O(dx²)` and grows with rate curvature; on sharp-rate days
/// (e.g. high interrill `theta` over a steep detachment gradient) the
/// relative gap reaches a few ×1e-3 while mass still conserves to 1e-9. Set
/// to 5e-3 (still ~100× below the `O(1)` gap a real flux bug would produce)
/// after a marginal 1.0016e-3 overrun on a snow/frost single-OFE fixture.
const WAVE1_FLUX_CLOSURE_REL_TOL: f64 = 5.0e-3;
/// Absolute floor for both closure gates in nondimensional load units.
const WAVE1_CLOSURE_ABS_FLOOR: f64 = 1.0e-9;

/// Static normalized slope-segment fit (`profil.for`): within segment `k`,
/// the local slope normalized by `avgslp` is `s*(x) = a*x + b` on
/// `x in [xu, xl]` (nondimensional distances).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectWave1SlopeSegment {
    pub xu: f64,
    pub xl: f64,
    pub a: f64,
    pub b: f64,
}

/// Inter-OFE shear/transport continuity operands (`param.for:184-196` +
/// `:249-390`, the `INV-SED-008` Eq. [11.4.x] family): the receiving OFE's
/// coefficient polynomials are re-derived so shear and transport capacity
/// are continuous across the OFE boundary. `shrspv`/`tcprev`/`ktrprv` are
/// receiver-side derivations from `qin` + the PRIOR lane's static slopes;
/// the `*lst` coefficient sets are the prior lane's solve-final values
/// (the legacy Fortran-`save` state).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Wave1InterOfeContinuity {
    /// Shear at the inflow discharge on the prior OFE's AVERAGE slope
    /// (`sheart(qin·rspace, cnslp_{i-1})`, floored 1e-6 Pa).
    pub shrspv_pa: f64,
    /// Prior-boundary transport capacity `trcoef(shrtp1)·shrspv^1.5`.
    pub tcprev_kg_s_m: f64,
    /// Prior-boundary transport-coefficient ratio
    /// `trcoef((shrtp1+shrspv)/2) / trcoef(shrtp1)`.
    pub ktrprv: f64,
    /// Prior lane's final shear coefficients (`anflst`, `bnflst`, `cnflst`).
    pub prior_shear_last: (f64, f64, f64),
    /// Prior lane's final transport coefficients
    /// (`atclst`, `btclst`, `ctclst`).
    pub prior_transport_last: (f64, f64, f64),
}

/// Typed operand payload for the Wave-1 continuity solve. Every field is a
/// required operand with a legacy lineage (`param.for`/`xinflo.for`); absent
/// or invalid operands are typed hard errors (no defaults, no proxies).
#[derive(Debug, Clone, PartialEq)]
pub struct DirectWave1ContinuityInputs {
    pub enabled: bool,
    /// Inter-OFE continuity operands; `None` on OFE-1 / single-OFE lanes
    /// (the coefficient rewrite is skipped, matching the legacy
    /// `iplane > 1 ∧ qout > 0 ∧ qin > 0` guard).
    pub inter_ofe: Option<Wave1InterOfeContinuity>,
    /// Normalized slope segments from the slope profile (`profil.for` fit).
    pub segments: Vec<DirectWave1SlopeSegment>,
    /// Peak runoff rate (m/s) — legacy `peakro(iplane)`.
    pub peakro_m_s: f64,
    /// Event runoff depth (m) — legacy `runoff(iplane)`; activation gate.
    pub runoff_depth_m: f64,
    /// Unit inflow discharge from the upstream OFE (m^2/s); 0 for OFE-1.
    pub qin_m2_s: f64,
    /// Effective flow-path length (m) — legacy `efflen(iplane)`.
    pub efflen_m: f64,
    /// OFE slope length (m) — legacy `slplen(iplane)`.
    pub slplen_m: f64,
    /// Contouring length (m) — legacy `cntlen(iplane)`; equals `slplen`
    /// without contours (`xinflo.for:147`).
    pub cntlen_m: f64,
    /// Rill spacing (m) — legacy `rspace(iplane)`.
    pub rspace_m: f64,
    /// Rill width (m) — legacy `width(iplane)`.
    pub width_m: f64,
    /// Hillslope field width (m) — legacy `fwidth`; publication area scale.
    pub field_width_m: f64,
    /// Effective runoff duration (s) — legacy `effdrn = runoff/peakro`
    /// (`irs.for:725`); the WB16 `runoff_duration_s` surface.
    pub effdrn_s: f64,
    /// Duration of rainfall excess (s) — legacy `effdrr`.
    pub effdrr_s: f64,
    /// Rill erodibility (s/m) — soil file `kr`.
    pub kr_s_m: f64,
    /// Daily rill-erodibility adjustment factor — legacy `kradjf`.
    pub kradjf: f64,
    /// Critical shear (Pa) — soil file `shcrit`.
    pub shcrit_pa: f64,
    /// Daily critical-shear adjustment factor — legacy `tcadjf`
    /// (INV-SED-006: `>= 0.30`).
    pub tcadjf: f64,
    /// Interrill detachment rate (kg s^-1 m^-2) — legacy `detinr`
    /// (`param.for:482`, delivery-ratio and rill-spacing folded upstream).
    pub detinr_kg_s_m2: f64,
    /// Flow shear at the end of the average slope (Pa) — legacy `shrsol`.
    pub shrsol_pa: f64,
    /// Transport capacity at the end of the average slope (kg s^-1 m^-1)
    /// — legacy `tcend = kt * shrsol**1.5`.
    pub tcend_kg_s_m: f64,
    /// Normalized transport coefficient — legacy `ktrato = kt2/kt`.
    pub ktrato: f64,
    /// Effective particle fall velocity (m/s) — legacy `veleff`.
    pub veleff_m_s: f64,
    /// Rainfall-turbulence factor — legacy `beta` (0.5 rain / 1.0 dry).
    pub beta: f64,
    /// Nondimensional inflow sediment load — legacy `strldn`; 0 for OFE-1.
    pub strldn: f64,
    /// Legacy `param.for:396`: surface frozen to the top -> `eata = 0`.
    pub surface_frozen: bool,
    /// Legacy `param.for:530`: snow cover / melt-only / furrow-only day
    /// -> `theta = 0`.
    pub theta_suppressed: bool,
}

impl DirectWave1ContinuityInputs {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            enabled: false,
            inter_ofe: None,
            segments: Vec::new(),
            peakro_m_s: 0.0,
            runoff_depth_m: 0.0,
            qin_m2_s: 0.0,
            efflen_m: 0.0,
            slplen_m: 0.0,
            cntlen_m: 0.0,
            rspace_m: 0.0,
            width_m: 0.0,
            field_width_m: 0.0,
            effdrn_s: 0.0,
            effdrr_s: 0.0,
            kr_s_m: 0.0,
            kradjf: 0.0,
            shcrit_pa: 0.0,
            tcadjf: 0.0,
            detinr_kg_s_m2: 0.0,
            shrsol_pa: 0.0,
            tcend_kg_s_m: 0.0,
            ktrato: 0.0,
            veleff_m_s: 0.0,
            beta: 0.0,
            strldn: 0.0,
            surface_frozen: false,
            theta_suppressed: false,
        }
    }
}

/// Per-point region provenance for the continuity residual accounting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Wave1PointRegion {
    Untouched,
    Detachment,
    Deposition,
    FlowEnd,
}

/// Wave-1 continuity solve state: normalized drivers, the 101-point grid
/// trajectories, and the denormalized event totals (INV-SED-010 payload
/// operands).
#[derive(Debug, Clone, PartialEq)]
pub struct DirectWave1ContinuityState {
    pub active: bool,
    /// Hour quanta refused by the FLUX-consistency diagnostic gate
    /// (`erosion.wave1.flux_closure`, the trapezoid-vs-RK4 discretization
    /// check — NOT the 1e-9 mass-balance law, which stays hard). Refused
    /// quanta contribute zero sediment (a surfaced under-estimate, the
    /// GAP-SED-THAW pattern), never fabricated values.
    pub flux_refused_quanta: u32,
    /// The solve-final shear/transport coefficient sets (the legacy
    /// Fortran-`save` `anflst`/`atclst` families) — the inter-OFE carry
    /// the NEXT lane's continuity rewrite consumes (`param.for:368-390`).
    pub end_shear_coefficients: (f64, f64, f64),
    pub end_transport_coefficients: (f64, f64, f64),
    pub eta: f64,
    pub taucn: f64,
    pub theta: f64,
    pub phi: f64,
    pub qostar: f64,
    pub qout_m2_s: f64,
    pub load: Vec<f64>,
    pub tcap: Vec<f64>,
    pub detach: Vec<f64>,
    /// Exported sediment at the OFE toe (kg per m of hillslope width).
    pub exported_sediment_kg_m: f64,
    /// Inflow sediment at the OFE top (kg per m of hillslope width).
    pub inflow_sediment_kg_m: f64,
    /// Event total detachment for the OFE (kg).
    pub total_detachment_kg: f64,
    /// Event total deposition for the OFE (kg).
    pub total_deposition_kg: f64,
    /// Sediment concentration at the toe (kg/m^3) — `sloss.for:314` form.
    pub sediment_concentration_kg_m3: f64,
    /// Interrill contribution rate surface — legacy `irdgdx` (kg/m^2).
    pub interrill_contribution_kg_m2: f64,
    /// `exported - inflow - (detach - deposition)` residual (kg per m
    /// of width); exact telescoping identity, hard-gated.
    pub publication_closure_residual_kg_m: f64,
    /// Nondimensional continuity flux residual (INV-SED-001 reporting).
    pub flux_closure_residual: f64,
    /// Total |dG| over the grid backing the relative flux gate.
    pub flux_closure_scale: f64,
}

impl DirectWave1ContinuityState {
    #[must_use]
    pub fn inactive() -> Self {
        Self {
            active: false,
            flux_refused_quanta: 0,
            end_shear_coefficients: (0.0, 0.0, 0.0),
            end_transport_coefficients: (0.0, 0.0, 0.0),
            eta: 0.0,
            taucn: 0.0,
            theta: 0.0,
            phi: 0.0,
            qostar: 0.0,
            qout_m2_s: 0.0,
            load: Vec::new(),
            tcap: Vec::new(),
            detach: Vec::new(),
            exported_sediment_kg_m: 0.0,
            inflow_sediment_kg_m: 0.0,
            total_detachment_kg: 0.0,
            total_deposition_kg: 0.0,
            sediment_concentration_kg_m3: 0.0,
            interrill_contribution_kg_m2: 0.0,
            publication_closure_residual_kg_m: 0.0,
            flux_closure_residual: 0.0,
            flux_closure_scale: 0.0,
        }
    }
}

/// Shear-regime classification (`xcrit.for` `mshear` 1..5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wave1ShearRegime {
    BelowCritical,
    AboveCritical,
    RisingCross,
    FallingCross,
    DoubleCross,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Wave1ShearClassification {
    pub regime: Wave1ShearRegime,
    pub xc1: f64,
    pub xc2: f64,
}

/// Normalized per-segment polynomial coefficients (`xinflo.for`):
/// shear `(a, b, c)` and transport `(atc, btc, ctc)`.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Wave1SegmentCoefficients {
    xu: f64,
    xl: f64,
    a: f64,
    b: f64,
    c: f64,
    atc: f64,
    btc: f64,
    ctc: f64,
}

/// Normalized drivers shared across the OFE solve.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Wave1Drivers {
    eta: f64,
    taucn: f64,
    theta: f64,
    phi: f64,
    ktrato: f64,
    qostar: f64,
    qout_m2_s: f64,
}

/// Mutable 101-point grid state for one OFE solve.
#[derive(Debug, Clone)]
struct Wave1RouteGrid {
    load: Vec<f64>,
    tcap: Vec<f64>,
    detach: Vec<f64>,
    region: Vec<Wave1PointRegion>,
    /// Point where a load clamp or floor fired (excluded from the
    /// trapezoid flux-residual accounting; the exact telescoping gate
    /// still covers them).
    clamped: Vec<bool>,
    /// Index of the last grid point with a committed load (0-based).
    ilast: usize,
}

impl Wave1RouteGrid {
    fn new() -> Self {
        Self {
            load: vec![0.0; DIRECT_WAVE1_GRID_POINTS],
            tcap: vec![0.0; DIRECT_WAVE1_GRID_POINTS],
            detach: vec![0.0; DIRECT_WAVE1_GRID_POINTS],
            region: vec![Wave1PointRegion::Untouched; DIRECT_WAVE1_GRID_POINTS],
            clamped: vec![false; DIRECT_WAVE1_GRID_POINTS],
            ilast: 0,
        }
    }
}

#[inline]
fn wave1_grid_x(index: usize) -> f64 {
    // xinput(i) = (i-1)*0.01 in legacy 1-based indexing.
    #[allow(clippy::cast_precision_loss)]
    {
        index as f64 * WAVE1_GRID_DX
    }
}

/// `shear.for`: classifier shear with the 0.0001 floor.
#[must_use]
pub fn wave1_classifier_shear(a: f64, b: f64, c: f64, x: f64) -> f64 {
    let mut value = a * x * x + b * x + c;
    if value < 0.0 {
        value = 0.0;
    }
    let shear = value.powf(WAVE1_CLASSIFIER_SHEAR_EXPONENT);
    if shear <= 0.0 {
        WAVE1_CLASSIFIER_SHEAR_FLOOR
    } else {
        shear
    }
}

/// `runge.for`/`erod.for` march shear: `exp(0.666667*log(xterm))`, zero when
/// the polynomial is non-positive (no floor — distinct from `shear.for`).
#[inline]
fn wave1_march_shear(xterm: f64) -> f64 {
    if xterm > 0.0 {
        xterm.powf(WAVE1_MARCH_SHEAR_EXPONENT)
    } else {
        0.0
    }
}

/// `runge.for`: detachment capacity `Dc = eata*(shr - tauc)` clamped `>= 0`
/// (INV-SED-002: no rill detachment when `tau_f <= tau_c`).
#[inline]
fn wave1_detachment_capacity(eata: f64, tauc: f64, shr: f64) -> f64 {
    let dcap = eata * (shr - tauc);
    if dcap < 0.0 { 0.0 } else { dcap }
}

/// Transport capacity at `x` (`erod.for` Equation 10.4.6), clamped `>= 0`
/// (INV-SED-006).
#[inline]
fn wave1_transport_capacity(atc: f64, btc: f64, ctc: f64, ktrato: f64, x: f64) -> f64 {
    let tcap = (atc * x * x + btc * x + ctc) * ktrato;
    if tcap < 0.0 { 0.0 } else { tcap }
}

/// `root.for`: roots of `a*x^2 + b*x - tauchk = 0`, ordered `x1 <= x2`.
/// Returns a typed error when the discriminant is negative (legacy would
/// produce a NaN; fail closed instead).
fn wave1_root(a: f64, b: f64, tauchk: f64) -> Result<(f64, f64), DirectRuntimeError> {
    let discriminant = b * b + 4.0 * a * tauchk;
    if discriminant < 0.0 || a == 0.0 {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.wave1.xcrit_root_discriminant",
        });
    }
    let part = discriminant.sqrt();
    let two_a = 2.0 * a;
    let mut x1 = (-b - part) / two_a;
    let mut x2 = (-b + part) / two_a;
    if x1 > x2 {
        std::mem::swap(&mut x1, &mut x2);
    }
    Ok((x1, x2))
}

/// `cross.for`: linear zero-crossing between `(x1, y1)` and `(x2, y2)`.
// Exact float equality is the legacy branch semantics (`cross.for:40`).
#[allow(clippy::float_cmp)]
fn wave1_cross(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let slope = if x1 == x2 {
        WAVE1_CROSS_DEGENERATE_SLOPE
    } else {
        (y2 - y1) / (x2 - x1)
    };
    -y1 / slope + x1
}

/// `undflo.for`: zero the base when `|expon * log10(factor)| > 30`.
fn wave1_undflo(factor: f64, expon: f64) -> (f64, f64) {
    if factor > 0.0 && (expon * factor.log10()).abs() > WAVE1_UNDFLO_POWER {
        (0.0, 1.0)
    } else {
        (factor, expon)
    }
}

/// `xcrit.for`: classify the shear regime on `[xb, xe]`.
///
/// `tauchk = tauc^1.5 - c` is deliberately **un-clamped**, matching the
/// pinned baseline where the `>= 0` clamp is commented out
/// (`xcrit.for:82`); the deleted `erod19` port added a clamp and is
/// reconciled back to the baseline here.
// Exact float comparisons and the branch length mirror `xcrit.for`
// one-for-one (legacy symbol/branch continuity per AGENTS.md).
#[allow(
    clippy::float_cmp,
    clippy::too_many_lines,
    clippy::similar_names,
    clippy::if_not_else
)]
pub fn wave1_xcrit(
    a: f64,
    b: f64,
    c: f64,
    tauc: f64,
    xb: f64,
    xe: f64,
) -> Result<Wave1ShearClassification, DirectRuntimeError> {
    let tauchk = tauc.powf(1.5) - c;
    let taub = wave1_classifier_shear(a, b, c, xb);
    let taue = wave1_classifier_shear(a, b, c, xe);

    if a == 0.0 {
        // Uniform-slope segment.
        let xc1 = if b != 0.0 {
            tauchk / b
        } else {
            WAVE1_XCRIT_UNIFORM_SENTINEL
        };
        let regime = if taue > taub {
            if xc1 <= xb {
                Wave1ShearRegime::AboveCritical
            } else if xc1 >= xe {
                Wave1ShearRegime::BelowCritical
            } else {
                Wave1ShearRegime::RisingCross
            }
        } else if xc1 >= xe {
            Wave1ShearRegime::AboveCritical
        } else if xc1 <= xb {
            Wave1ShearRegime::BelowCritical
        } else {
            Wave1ShearRegime::FallingCross
        };
        return Ok(Wave1ShearClassification {
            regime,
            xc1,
            xc2: xe,
        });
    }

    if a > 0.0 && taue > taub {
        // Convex segment with shear increasing downslope.
        if taub >= tauc {
            return Ok(Wave1ShearClassification {
                regime: Wave1ShearRegime::AboveCritical,
                xc1: xb,
                xc2: xe,
            });
        }
        if taue <= tauc {
            return Ok(Wave1ShearClassification {
                regime: Wave1ShearRegime::BelowCritical,
                xc1: xb,
                xc2: xe,
            });
        }
        let (x1, x2) = wave1_root(a, b, tauchk)?;
        let mut xc1 = xb;
        if x1 >= xb && x1 <= xe {
            xc1 = x1;
        } else if x2 >= xb && x2 <= xe {
            xc1 = x2;
        }
        return Ok(Wave1ShearClassification {
            regime: Wave1ShearRegime::RisingCross,
            xc1,
            xc2: xe,
        });
    }

    // Any other segment shape.
    if taue >= tauc && taub >= tauc {
        return Ok(Wave1ShearClassification {
            regime: Wave1ShearRegime::AboveCritical,
            xc1: xb,
            xc2: xe,
        });
    }
    let part = b * b + 4.0 * a * tauchk;
    if part <= 0.0 {
        return Ok(Wave1ShearClassification {
            regime: Wave1ShearRegime::BelowCritical,
            xc1: xb,
            xc2: xe,
        });
    }
    let (x1, x2) = wave1_root(a, b, tauchk)?;
    if taub <= tauc && taue >= tauc {
        let xc1 = if x1 <= xb || x1 >= xe { x2 } else { x1 };
        return Ok(Wave1ShearClassification {
            regime: Wave1ShearRegime::RisingCross,
            xc1,
            xc2: xe,
        });
    }
    if taub >= tauc && taue <= tauc {
        let xc1 = if x1 <= xb || x1 >= xe { x2 } else { x1 };
        return Ok(Wave1ShearClassification {
            regime: Wave1ShearRegime::FallingCross,
            xc1,
            xc2: xe,
        });
    }
    if taub <= tauc && taue <= tauc {
        let regime = if x1 < xb || x1 > xe || x2 < xb || x2 > xe || x1 == x2 {
            Wave1ShearRegime::BelowCritical
        } else {
            Wave1ShearRegime::DoubleCross
        };
        return Ok(Wave1ShearClassification {
            regime,
            xc1: x1,
            xc2: x2,
        });
    }
    Ok(Wave1ShearClassification {
        regime: Wave1ShearRegime::BelowCritical,
        xc1: xb,
        xc2: xe,
    })
}

/// Continuity right-hand side `dG/dx = Dc*(1 - G/Tc) + theta` when
/// `Tc > 0`, else `theta` (`runge.for` `tmpvr`).
#[inline]
fn wave1_continuity_rate(dcap: f64, tcap: f64, theta: f64, load: f64) -> f64 {
    if tcap > 0.0 {
        dcap * ((tcap - load) / tcap) + theta
    } else {
        theta
    }
}

/// `runge.for` internals: returns the RK4 result and whether the interrill
/// floor (`runge.for:219`) fired.
#[allow(clippy::too_many_arguments)]
fn wave1_runge_step_traced(
    a: f64,
    b: f64,
    c: f64,
    atc: f64,
    btc: f64,
    ctc: f64,
    eata: f64,
    tauc: f64,
    theta: f64,
    ktrato: f64,
    dx: f64,
    x: f64,
    ldold: f64,
) -> (f64, bool) {
    // K1 at x.
    let shr1 = wave1_march_shear(a * x * x + b * x + c);
    let dcap1 = wave1_detachment_capacity(eata, tauc, shr1);
    let tcap1 = wave1_transport_capacity(atc, btc, ctc, ktrato, x);
    let k1 = dx * wave1_continuity_rate(dcap1, tcap1, theta, ldold);

    // K2 at x + dx/2.
    let xmid = x + dx / 2.0;
    let shr2 = wave1_march_shear(a * xmid * xmid + b * xmid + c);
    let dcap2 = wave1_detachment_capacity(eata, tauc, shr2);
    let tcap2 = wave1_transport_capacity(atc, btc, ctc, ktrato, xmid);
    let k2 = dx * wave1_continuity_rate(dcap2, tcap2, theta, ldold + 0.5 * k1);

    // K3 reuses K2's Dc/Tc (same midpoint); only the load changes.
    let k3 = dx * wave1_continuity_rate(dcap2, tcap2, theta, ldold + 0.5 * k2);

    // K4 at x + dx.
    let xend = x + dx;
    let shr4 = wave1_march_shear(a * xend * xend + b * xend + c);
    let dcap4 = wave1_detachment_capacity(eata, tauc, shr4);
    let tcap4 = wave1_transport_capacity(atc, btc, ctc, ktrato, xend);
    let k4 = dx * wave1_continuity_rate(dcap4, tcap4, theta, ldold + k3);

    let ldnew = ldold + (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;

    // Interrill floor (`runge.for:219`).
    let ldtest = ldold + theta * dx;
    if ldnew < ldtest {
        (ldtest, true)
    } else {
        (ldnew, false)
    }
}

/// `runge.for`: one classic RK4 step of the detachment continuity ODE.
///
/// `k3` reuses `k2`'s shear/detachment/transport capacities (same
/// `x + dx/2` evaluation point) exactly as legacy; the `/detcom/` memo is
/// reproduced by stateless recomputation, which is numerically identical
/// because `shr` and `Dc` are pure functions of `(xterm, eata, tauc)`.
/// Ends with the mandatory interrill floor `ldnew >= ldold + theta*dx`.
#[allow(clippy::too_many_arguments)]
#[must_use]
pub fn wave1_runge_step(
    a: f64,
    b: f64,
    c: f64,
    atc: f64,
    btc: f64,
    ctc: f64,
    eata: f64,
    tauc: f64,
    theta: f64,
    ktrato: f64,
    dx: f64,
    x: f64,
    ldold: f64,
) -> f64 {
    wave1_runge_step_traced(
        a, b, c, atc, btc, ctc, eata, tauc, theta, ktrato, dx, x, ldold,
    )
    .0
}

/// `depc.for`: integration constant for the analytic deposition solution.
#[allow(clippy::too_many_arguments)]
#[must_use]
pub fn wave1_depc(
    xu: f64,
    a: f64,
    b: f64,
    phi: f64,
    theta: f64,
    du: f64,
    ktrato: f64,
    qostar: f64,
) -> f64 {
    if (qostar + xu).abs() >= WAVE1_DEP_DENOMINATOR_EPSILON {
        du - (a * ktrato * phi * 2.0 * (qostar + xu) / (phi + 2.0))
            - ((b * ktrato - 2.0 * a * ktrato * qostar - theta) * phi / (phi + 1.0))
    } else {
        0.0
    }
}

/// `depeqs.for`: analytic deposition rate `D(x)`.
///
/// Legacy shifts the evaluation point off the flow-end singularity by
/// mutating the caller's `x` (grid points included, by reference); the
/// shift is applied locally here without mutating the caller's grid — that
/// side effect is unreachable for `qostar >= 0` single-OFE solves and is
/// deferred with the Increment-2 decreasing-flow scope.
#[allow(clippy::too_many_arguments)]
#[must_use]
pub fn wave1_depeqs(
    xu: f64,
    cdep: f64,
    a: f64,
    b: f64,
    phi: f64,
    theta: f64,
    x: f64,
    ktrato: f64,
    qostar: f64,
) -> f64 {
    let x_eval = if (qostar + x).abs() < WAVE1_DEP_DENOMINATOR_EPSILON {
        -qostar - WAVE1_DEPEQS_X_SHIFT
    } else {
        x
    };
    let mut ratio = (xu + qostar) / (x_eval + qostar);
    if qostar >= 0.0 && ratio > 1.0 {
        ratio = 1.0;
    }
    let (ratio, expon) = wave1_undflo(ratio, 1.0 + phi);
    let tmpvr1 = 2.0 * a * ktrato;
    (tmpvr1 * phi * (x_eval + qostar) / (2.0 + phi))
        + (phi / (1.0 + phi)) * (b * ktrato - theta - (tmpvr1 * qostar))
        + cdep * ratio.powf(expon)
}

/// `depend.for`: where deposition ends (Newton, at most 10 iterations,
/// increasing-flow `qostar >= 0` vs decreasing-flow branches).
#[allow(clippy::too_many_arguments)]
#[must_use]
pub fn wave1_depend(
    xu: f64,
    xl: f64,
    a: f64,
    b: f64,
    cdep: f64,
    phi: f64,
    theta: f64,
    ktrato: f64,
    qostar: f64,
) -> f64 {
    let tmpvr1 = 2.0 * a * ktrato;
    let r1 = (phi / (1.0 + phi)) * (b * ktrato - theta - (tmpvr1 * qostar));
    let r2 = tmpvr1 * phi / (2.0 + phi);

    let mut xdend;
    if qostar >= 0.0 {
        xdend = xl;
        let ratio = (xu + qostar) / (xdend + qostar);
        let (ratio, expon) = wave1_undflo(ratio, 1.0 + phi);
        let f = r1 + r2 * (xdend + qostar) + cdep * ratio.powf(expon);
        if f < 0.0 {
            return xdend;
        }
        xdend = xu + WAVE1_DEPEND_STEP_POSITIVE;
        if xdend > xl {
            xdend = f64::midpoint(xu, xl);
        }
    } else {
        if (xu + qostar).abs() <= WAVE1_DEPEND_XU_QOSTAR_NEAR_ZERO {
            return -qostar;
        }
        xdend = xu + WAVE1_DEPEND_STEP_NEGATIVE;
        if xdend > xl {
            xdend = f64::midpoint(xu, xl);
        }
        let ratio = (xu + qostar) / (xdend + qostar);
        let (ratio, expon) = wave1_undflo(ratio, 1.0 + phi);
        let f = r1 + r2 * (xdend + qostar) + cdep * ratio.powf(expon);
        if f >= 0.0 {
            return xdend;
        }
    }

    let mut converged = false;
    let mut xmin = xl;
    let mut positive_count = 0_u32;
    for _ in 0..WAVE1_DEPEND_MAX_ITERS {
        let denominator = xdend + qostar;
        let denominator_nonzero = denominator.abs() > 0.0;
        let mut ratio = if denominator_nonzero {
            (xu + qostar) / denominator
        } else {
            1.0
        };
        if ratio < 0.0 {
            ratio = 1.0;
        }
        let (ratio, expon) = wave1_undflo(ratio, 1.0 + phi);
        let ratio_pow = ratio.powf(expon);
        let f = r1 + r2 * (xdend + qostar) + cdep * ratio_pow;

        if f > 0.0 && qostar < 0.0 {
            positive_count += 1;
            if xdend < xmin {
                xmin = xdend;
            }
        }

        if f.abs() <= WAVE1_DEPEND_RESIDUAL_TOL {
            converged = true;
            break;
        }

        if denominator_nonzero {
            let df = r2 - (1.0 + phi) * cdep * ratio_pow / denominator;
            if df.abs() > 0.0 {
                xdend -= f / df;
                if qostar < 0.0 {
                    if xdend < xu {
                        xdend = xu + WAVE1_DEPEND_STEP_NEGATIVE;
                    }
                    if xdend > -qostar {
                        xdend = -qostar - WAVE1_DEPEND_STEP_NEGATIVE;
                    }
                    if xdend > xl {
                        xdend = xl;
                    }
                }
            } else {
                xdend = xu + WAVE1_DEPEND_STEP_NEGATIVE;
            }
        }
        if xdend < xu {
            xdend = xu + WAVE1_DEPEND_STEP_NEGATIVE;
        }
    }

    if !converged && qostar < 0.0 {
        if positive_count == 0 {
            xdend = xl;
        } else {
            xdend = xmin;
        }
    }
    xdend
}

/// `depos.for`: write the analytic deposition profile
/// `G = Tc - D(x)*(x + qostar)/phi` over grid points in `[xb, xe]`, with
/// the monotonic guard for `theta <= 0` and the `G >= 0` clamp; leaves
/// the deposition rate `dl` and load `ldlast` at `x = xe`.
#[allow(clippy::too_many_arguments)]
fn wave1_depos(
    grid: &mut Wave1RouteGrid,
    xb: f64,
    xe: f64,
    cdep: f64,
    seg: &Wave1SegmentCoefficients,
    drivers: &Wave1Drivers,
    dl: &mut f64,
    ldlast: &mut f64,
) {
    let phi = drivers.phi;
    let theta = drivers.theta;
    let ktrato = drivers.ktrato;
    let qostar = drivers.qostar;
    let (atc, btc, ctc) = (seg.atc, seg.btc, seg.ctc);
    let ibeg = grid.ilast + 1;
    // Legacy `depos.for:70` `if (ibeg.lt.102)`.
    if ibeg >= DIRECT_WAVE1_GRID_POINTS {
        return;
    }

    let end_rate_and_load = |dl_out: &mut f64, ldlast_out: &mut f64| {
        let rate = wave1_depeqs(xb, cdep, atc, btc, phi, theta, xe, ktrato, qostar);
        let xterm = atc * xe * xe + btc * xe + ctc;
        let mut tclast = xterm * ktrato;
        if tclast <= 0.0 {
            tclast = 0.0;
        }
        *dl_out = rate;
        *ldlast_out = tclast - rate * (xe + qostar) / phi;
    };

    if wave1_grid_x(ibeg) > xe {
        // Deposition region ends before the next grid point
        // (`depos.for:73`).
        if qostar <= -1.0 || qostar >= 0.0 || xe <= -qostar {
            end_rate_and_load(dl, ldlast);
        } else {
            *ldlast = 0.0;
        }
    } else {
        let mut i = grid.ilast;
        loop {
            i += 1;
            if i >= DIRECT_WAVE1_GRID_POINTS {
                break;
            }
            let x = wave1_grid_x(i);
            if x > xe {
                break;
            }
            if qostar <= -1.0 || qostar >= 0.0 || x <= -qostar {
                let rate = wave1_depeqs(xb, cdep, atc, btc, phi, theta, x, ktrato, qostar);
                grid.detach[i] = rate;
                let xterm = atc * x * x + btc * x + ctc;
                let mut tcap = xterm * ktrato;
                if tcap < 0.0 {
                    tcap = 0.0;
                }
                grid.tcap[i] = tcap;
                grid.load[i] = tcap - rate * (x + qostar) / phi;
                // `depos.for:116`: prevent erroneous detachment from the
                // deposition equation when there is no interrill source.
                if theta <= 0.0 && i > 0 && grid.load[i] > grid.load[i - 1] {
                    grid.load[i] = grid.load[i - 1];
                    grid.clamped[i] = true;
                }
                grid.region[i] = Wave1PointRegion::Deposition;
            } else {
                grid.load[i] = 0.0;
                grid.tcap[i] = 0.0;
                grid.region[i] = Wave1PointRegion::FlowEnd;
            }
            if grid.load[i] < 0.0 {
                grid.load[i] = 0.0;
                grid.clamped[i] = true;
            }
            grid.ilast = i;
            if x >= 1.0 {
                break;
            }
        }

        // `depos.for:143-173` end-of-region rate and load with the
        // case-4 flow-end guards.
        if qostar >= 0.0 || qostar <= -1.0 || xe < -qostar {
            end_rate_and_load(dl, ldlast);
        } else {
            *ldlast = 0.0;
            *dl = 0.0;
        }
        if *ldlast < 0.0 {
            *ldlast = 0.0;
        }
    }
}

/// Outcome of one `erod.for` detachment march over `[xb, xe]`.
struct Wave1ErodOutcome {
    /// Deposition detected (`ndep = 1`): `xdbeg` holds the onset point.
    ndep: bool,
    xdbeg: f64,
}

/// `erod.for`: RK4 detachment march over the grid points in `[xb, xe]`,
/// including the case-4 flow-end handling and the deposition-onset secant
/// solve (`cross.for`).
// Single-character coefficient names and exact float comparisons preserve
// the legacy `erod.for` symbols and branch semantics (AGENTS.md naming
// continuity for contract-pinned kernels).
#[allow(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::many_single_char_names,
    clippy::float_cmp,
    clippy::if_not_else
)]
fn wave1_erod(
    grid: &mut Wave1RouteGrid,
    seg: &Wave1SegmentCoefficients,
    eata: f64,
    drivers: &Wave1Drivers,
    xb: f64,
    xe: f64,
    dl: &mut f64,
    ldlast: &mut f64,
) -> Wave1ErodOutcome {
    let tauc = drivers.taucn;
    let theta = drivers.theta;
    let phi = drivers.phi;
    let ktrato = drivers.ktrato;
    let qostar = drivers.qostar;
    let (a, b, c) = (seg.a, seg.b, seg.c);
    let (atc, btc, ctc) = (seg.atc, seg.btc, seg.ctc);

    let mut outcome = Wave1ErodOutcome {
        ndep: false,
        xdbeg: 0.0,
    };

    // Legacy `erod.for:135-136`: proceed only when the next grid point
    // exists and lies inside the march interval.
    let ibeg = grid.ilast + 1;
    if ibeg >= DIRECT_WAVE1_GRID_POINTS || wave1_grid_x(ibeg) > xe {
        return outcome;
    }

    let mut kflag = 0_u8;
    let mut ldrat = 0.0;
    let mut ldrat2 = 0.0;

    // *** L2 loop: march the grid points inside [xb, xe]. ***
    let mut i = grid.ilast;
    let currpt = loop {
        i += 1;
        if i >= DIRECT_WAVE1_GRID_POINTS {
            break DIRECT_WAVE1_GRID_POINTS - 1;
        }
        let x = wave1_grid_x(i);
        if x > xe {
            break i;
        }

        let (dx, x_from, ld_from) = if i <= ibeg {
            (x - xb, xb, *ldlast)
        } else {
            (WAVE1_GRID_DX, wave1_grid_x(i - 1), grid.load[i - 1])
        };

        let shr = wave1_march_shear(a * x * x + b * x + c);
        let dcap = wave1_detachment_capacity(eata, tauc, shr);
        let tcap = wave1_transport_capacity(atc, btc, ctc, ktrato, x);
        grid.tcap[i] = tcap;

        let mut loopfg = false;
        if qostar > -1.0 && qostar < 0.0 && x > -qostar {
            // Case-4 plane past where runoff ends (`erod.for:208`).
            grid.load[i] = 0.0;
            grid.region[i] = Wave1PointRegion::FlowEnd;
            kflag = 4;
            outcome.ndep = false;
        } else {
            let (ldnew, floored) = wave1_runge_step_traced(
                a, b, c, atc, btc, ctc, eata, tauc, theta, ktrato, dx, x_from, ld_from,
            );
            grid.load[i] = ldnew;
            grid.clamped[i] = floored;
            grid.region[i] = Wave1PointRegion::Detachment;
            if tcap > 0.0 {
                ldrat = 1.0 - grid.load[i] / tcap;
                kflag = 1;
                grid.detach[i] = dcap * ldrat;
                if grid.load[i] > 0.0 {
                    ldrat2 = tcap / grid.load[i] - 1.0;
                    kflag = 2;
                }
            } else if grid.load[i] > 0.0 {
                ldrat2 = tcap / grid.load[i] - 1.0;
                kflag = 2;
            } else {
                grid.load[i] = 0.0;
                kflag = 3;
            }
        }

        if (kflag == 2 && ldrat2 < 0.0) || (kflag == 1 && ldrat < 0.0) {
            outcome.ndep = true;
            loopfg = true;
        } else {
            grid.ilast = i;
        }
        if x >= 1.0 {
            loopfg = true;
        }
        if loopfg {
            break i;
        }
    };

    let ldlast_local;
    let mut xlast;
    let xfrt;
    let mut detfrt;
    let mut detlst = 0.0;

    if !outcome.ndep {
        // *** M2 IF (`erod.for:282`): no deposition inside the march —
        // integrate the sub-grid tail to xe. ***
        if kflag == 4 {
            *ldlast = 0.0;
            *dl = 0.0;
            return outcome;
        }
        let x_ilast = wave1_grid_x(grid.ilast);
        if xe == x_ilast {
            ldlast_local = grid.load[grid.ilast];
            xlast = x_ilast;
        } else {
            let dx = xe - x_ilast;
            ldlast_local = wave1_runge_step(
                a,
                b,
                c,
                atc,
                btc,
                ctc,
                eata,
                tauc,
                theta,
                ktrato,
                dx,
                x_ilast,
                grid.load[grid.ilast],
            );
            xlast = xe;
        }

        let shr = wave1_march_shear(a * xlast * xlast + b * xlast + c);
        let dcap = wave1_detachment_capacity(eata, tauc, shr);
        let tcap = wave1_transport_capacity(atc, btc, ctc, ktrato, xlast);

        if tcap > 0.0 {
            ldrat = 1.0 - ldlast_local / tcap;
            *dl = dcap * ldrat;
            // Legacy sets kflag = 1 here (`erod.for:344`); the store is
            // dead because every continuing path below re-flags kflag = 2
            // before the onset iteration.
            if ldrat >= 0.0 {
                *ldlast = ldlast_local;
                return outcome;
            }
        } else if ldlast_local <= 0.0 {
            *ldlast = 0.0;
            *dl = 0.0;
            return outcome;
        }

        // Deposition begins at the segment end (`erod.for:363`).
        ldrat2 = tcap / ldlast_local - 1.0;
        kflag = 2;
        detfrt = ldrat2;
        if grid.load[grid.ilast] > 0.0 {
            detlst = grid.tcap[grid.ilast] / grid.load[grid.ilast] - 1.0;
        }
        outcome.ndep = true;
        xfrt = xlast;

        if wave1_grid_x(grid.ilast) == xfrt {
            let prev = grid.ilast.saturating_sub(1);
            xlast = wave1_grid_x(prev);
            if detfrt == ldrat2 {
                if grid.load[prev] > 0.0 {
                    detlst = grid.tcap[prev] / grid.load[prev] - 1.0;
                }
            } else if grid.tcap[prev] > 0.0 {
                detlst = 1.0 - grid.load[prev] / grid.tcap[prev];
            }
        } else {
            xlast = wave1_grid_x(grid.ilast);
        }
        // Legacy keeps `ldlast` (the load computed at xe) paired with the
        // rewound `xlast` for the first onset iteration; reproduced as-is.
    } else {
        // *** M2 ELSE (`erod.for:390`): deposition inside the march —
        // reconstruct the (xlast, ldlast, tclast) bracket of the last
        // committed point (or the segment entry when none committed). ***
        xfrt = wave1_grid_x(currpt);
        let tclast_local;
        if grid.ilast + 1 == ibeg {
            xlast = xb;
            ldlast_local = *ldlast;
            let xtrmtc0 = atc * xb * xb + btc * xb + ctc;
            tclast_local = (xtrmtc0 * ktrato).max(0.0);
        } else {
            xlast = wave1_grid_x(grid.ilast);
            ldlast_local = grid.load[grid.ilast];
            tclast_local = grid.tcap[grid.ilast];
        }
        detfrt = 0.0;
        if xlast <= 0.0 && tclast_local <= 0.0 && ldlast_local <= 0.0 {
            kflag = 5;
            detlst = *dl;
            detfrt =
                (phi / (phi + 1.0)) * (ktrato * (atc * xfrt * xfrt + btc * xfrt + ctc) - theta);
        }
        if kflag == 1 {
            detfrt = ldrat;
            detlst = if tclast_local > 0.0 {
                1.0 - ldlast_local / tclast_local
            } else {
                0.0
            };
        } else if kflag == 2 {
            detfrt = ldrat2;
            detlst = if ldlast_local > 0.0 {
                tclast_local / ldlast_local - 1.0
            } else {
                0.0
            };
        }

        // `erod.for:419`: top-of-OFE degenerate bracket.
        if detfrt <= 0.0 && detlst <= 0.0 && xlast <= 0.0 {
            outcome.xdbeg = 0.0;
            *ldlast = ldlast_local;
            return outcome;
        }
        if detlst < 0.0 {
            detlst = 0.0;
        }
    }

    // *** N2 loop (`erod.for:434`): cross/secant iteration for the
    // deposition onset xdbeg. ***
    let mut xtry = xfrt;
    let mut ldtry = ldlast_local;
    let mut xlast_iter = xlast;
    let mut detlst_iter = detlst;
    let mut detfrt_iter = detfrt;
    let mut xfrt_iter = xfrt;
    let mut ldlast_iter = ldlast_local;
    for _ in 0..WAVE1_ONSET_MAX_ITERS {
        xtry = wave1_cross(xlast_iter, detlst_iter, xfrt_iter, detfrt_iter);
        let dx = xtry - xlast_iter;
        ldtry = wave1_runge_step(
            a,
            b,
            c,
            atc,
            btc,
            ctc,
            eata,
            tauc,
            theta,
            ktrato,
            dx,
            xlast_iter,
            ldlast_iter,
        );
        let mut tcap = (atc * xtry * xtry + btc * xtry + ctc) * ktrato;
        if tcap < 0.0 {
            tcap = 0.0;
        }

        let mut converged = false;
        let mut dettry = 0.0;
        if kflag == 2 {
            if ldtry <= 0.0 {
                ldtry = WAVE1_ONSET_FLOOR;
            }
            if ((tcap - ldtry) / ldtry).abs() < WAVE1_ONSET_REL_TOL {
                converged = true;
            } else {
                dettry = tcap / ldtry - 1.0;
            }
        } else if kflag == 1 {
            if tcap <= 0.0 {
                tcap = WAVE1_ONSET_FLOOR;
            }
            if ((ldtry - tcap) / tcap).abs() < WAVE1_ONSET_REL_TOL {
                converged = true;
            } else {
                dettry = 1.0 - ldtry / tcap;
            }
        } else if kflag == 5 {
            dettry = (phi / (phi + 1.0)) * (tcap - theta);
        }

        if converged {
            break;
        }
        if dettry <= 0.0 {
            detfrt_iter = dettry;
            xfrt_iter = xtry;
        } else {
            xlast_iter = xtry;
            detlst_iter = dettry;
            ldlast_iter = ldtry;
        }
    }

    outcome.xdbeg = xtry;
    *dl = 0.0;
    *ldlast = ldtry;
    outcome
}

/// `route.for` mshear dispatch: pass rill erodibility `eata` only on
/// above-critical sub-intervals and `0.0` on below-critical ones.
#[allow(clippy::too_many_arguments)]
fn wave1_dispatch_detachment(
    grid: &mut Wave1RouteGrid,
    seg: &Wave1SegmentCoefficients,
    classification: &Wave1ShearClassification,
    eata: f64,
    drivers: &Wave1Drivers,
    start_x: f64,
    dl: &mut f64,
    ldlast: &mut f64,
) -> Wave1ErodOutcome {
    let xl = seg.xl;
    let xc1 = classification.xc1;
    let xc2 = classification.xc2;
    match classification.regime {
        Wave1ShearRegime::BelowCritical => {
            wave1_erod(grid, seg, 0.0, drivers, start_x, xl, dl, ldlast)
        }
        Wave1ShearRegime::AboveCritical => {
            wave1_erod(grid, seg, eata, drivers, start_x, xl, dl, ldlast)
        }
        Wave1ShearRegime::RisingCross => {
            if start_x <= xc1 {
                let outcome = wave1_erod(grid, seg, 0.0, drivers, start_x, xc1, dl, ldlast);
                if outcome.ndep {
                    return outcome;
                }
                wave1_erod(grid, seg, eata, drivers, xc1, xl, dl, ldlast)
            } else {
                wave1_erod(grid, seg, eata, drivers, start_x, xl, dl, ldlast)
            }
        }
        Wave1ShearRegime::FallingCross => {
            if start_x <= xc1 {
                let outcome = wave1_erod(grid, seg, eata, drivers, start_x, xc1, dl, ldlast);
                if outcome.ndep {
                    return outcome;
                }
                wave1_erod(grid, seg, 0.0, drivers, xc1, xl, dl, ldlast)
            } else {
                wave1_erod(grid, seg, 0.0, drivers, start_x, xl, dl, ldlast)
            }
        }
        Wave1ShearRegime::DoubleCross => {
            if start_x <= xc1 {
                let outcome = wave1_erod(grid, seg, 0.0, drivers, start_x, xc1, dl, ldlast);
                if outcome.ndep {
                    return outcome;
                }
                let outcome = wave1_erod(grid, seg, eata, drivers, xc1, xc2, dl, ldlast);
                if outcome.ndep {
                    return outcome;
                }
                wave1_erod(grid, seg, 0.0, drivers, xc2, xl, dl, ldlast)
            } else if start_x > xc2 {
                wave1_erod(grid, seg, 0.0, drivers, start_x, xl, dl, ldlast)
            } else {
                let outcome = wave1_erod(grid, seg, eata, drivers, start_x, xc2, dl, ldlast);
                if outcome.ndep {
                    return outcome;
                }
                wave1_erod(grid, seg, 0.0, drivers, xc2, xl, dl, ldlast)
            }
        }
    }
}

/// `route.for`: route sediment through the OFE profile.
// The segment control flow mirrors `route.for`'s big do-loop one-for-one.
#[allow(clippy::too_many_lines)]
fn wave1_route(
    segments: &[Wave1SegmentCoefficients],
    drivers: &Wave1Drivers,
    strldn: f64,
) -> Result<Wave1RouteGrid, DirectRuntimeError> {
    let mut grid = Wave1RouteGrid::new();
    let first = segments
        .first()
        .ok_or(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.wave1.segments",
        })?;

    let mut ldlast = strldn;
    // Transport capacity and load at the first point on the OFE
    // (`route.for:136-137`). The first point carries no detach rate in
    // legacy (detach(1) never written), so it stays region-untagged for
    // the flux accounting.
    grid.tcap[0] = first.ctc * drivers.ktrato;
    grid.load[0] = strldn;

    // Upper-boundary deposition-rate estimate (`route.for:169-173`).
    let mut dl = if drivers.qostar.abs() < WAVE1_QOSTAR_NEAR_ZERO {
        drivers.phi / (drivers.phi + 1.0) * (drivers.ktrato * first.btc - drivers.theta)
    } else {
        drivers.phi / drivers.qostar * (drivers.ktrato * first.ctc - ldlast)
    };
    validate_finite("erosion.wave1.route_dl", dl)?;

    let qout_positive = drivers.qout_m2_s > 0.0;

    for seg in segments {
        // Case-4 bypass: flow has ended before this segment
        // (`route.for:188`).
        if !(qout_positive || seg.xu < -drivers.qostar) {
            continue;
        }
        let xcrit_end = if qout_positive || seg.xl < -drivers.qostar {
            seg.xl
        } else {
            -drivers.qostar
        };
        let classification = wave1_xcrit(seg.a, seg.b, seg.c, drivers.taucn, seg.xu, xcrit_end)?;

        let mut ndep = false;
        let mut xdbeg = 0.0;
        let du = dl;
        if du < 0.0 {
            // Deposition at the upper end of the segment (`route.for:212`).
            let cdep = wave1_depc(
                seg.xu,
                seg.atc,
                seg.btc,
                drivers.phi,
                drivers.theta,
                du,
                drivers.ktrato,
                drivers.qostar,
            );
            let mut xdend = wave1_depend(
                seg.xu,
                seg.xl,
                seg.atc,
                seg.btc,
                cdep,
                drivers.phi,
                drivers.theta,
                drivers.ktrato,
                drivers.qostar,
            );
            validate_finite("erosion.wave1.route_xdend", xdend)?;
            if xdend >= seg.xl {
                // Deposition does not end within the segment.
                xdend = seg.xl;
                wave1_depos(
                    &mut grid,
                    seg.xu,
                    xdend,
                    cdep,
                    seg,
                    drivers,
                    &mut dl,
                    &mut ldlast,
                );
            } else {
                // Deposition ends inside the segment; detachment follows.
                wave1_depos(
                    &mut grid,
                    seg.xu,
                    xdend,
                    cdep,
                    seg,
                    drivers,
                    &mut dl,
                    &mut ldlast,
                );
                let outcome = wave1_dispatch_detachment(
                    &mut grid,
                    seg,
                    &classification,
                    drivers.eta,
                    drivers,
                    xdend,
                    &mut dl,
                    &mut ldlast,
                );
                ndep = outcome.ndep;
                xdbeg = outcome.xdbeg;
            }
        } else {
            // Detachment at the upper end of the segment (`route.for:356`).
            dl = 0.0;
            let outcome = wave1_dispatch_detachment(
                &mut grid,
                seg,
                &classification,
                drivers.eta,
                drivers,
                seg.xu,
                &mut dl,
                &mut ldlast,
            );
            ndep = outcome.ndep;
            xdbeg = outcome.xdbeg;
        }

        // Deposition tail after a detachment section reached transport
        // capacity (`route.for:436`).
        if ndep {
            dl = 0.0;
            let cdep = wave1_depc(
                xdbeg,
                seg.atc,
                seg.btc,
                drivers.phi,
                drivers.theta,
                0.0,
                drivers.ktrato,
                drivers.qostar,
            );
            wave1_depos(
                &mut grid,
                xdbeg,
                seg.xl,
                cdep,
                seg,
                drivers,
                &mut dl,
                &mut ldlast,
            );
        }
    }

    // Fail-closed: the toe must have been computed by the march or the
    // deposition writer for an active (runoff) day.
    let toe = DIRECT_WAVE1_GRID_POINTS - 1;
    if grid.region[toe] == Wave1PointRegion::Untouched {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.wave1.route_toe_uncomputed",
        });
    }
    Ok(grid)
}

/// Validate the **routed-event** Wave-1 operand payload (fail-closed;
/// INV-SED-004 hydrology surfaces, INV-SED-006 `tcadjf`, INV-SED-007
/// denominators). Called only after the runoff-day/`passby` activation
/// gates: on non-routed days the sediment operands are legitimately
/// zeroed and never inspected (legacy `contin.for` ordering).
fn validate_wave1_inputs(inputs: &DirectWave1ContinuityInputs) -> Result<(), DirectRuntimeError> {
    if inputs.segments.is_empty() {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.wave1.segments",
        });
    }
    let mut previous_xl = 0.0;
    for segment in &inputs.segments {
        validate_finite("erosion.wave1.segment_a", segment.a)?;
        validate_finite("erosion.wave1.segment_b", segment.b)?;
        validate_nonnegative_direct_m("erosion.wave1.segment_xu", segment.xu)?;
        validate_nonnegative_direct_m("erosion.wave1.segment_xl", segment.xl)?;
        if segment.xl <= segment.xu || (segment.xu - previous_xl).abs() > WB11_ZERO_THRESHOLD {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "erosion.wave1.segment_bounds",
            });
        }
        previous_xl = segment.xl;
    }
    if (previous_xl - 1.0).abs() > WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.wave1.segment_toe",
        });
    }
    wave1_validate_min("erosion.wave1.peakro_m_s", inputs.peakro_m_s, 0.0)?;
    wave1_validate_min("erosion.wave1.runoff_depth_m", inputs.runoff_depth_m, 0.0)?;
    wave1_validate_min("erosion.wave1.qin_m2_s", inputs.qin_m2_s, 0.0)?;
    wave1_validate_min(
        "erosion.wave1.efflen_m",
        inputs.efflen_m,
        WB11_ZERO_THRESHOLD,
    )?;
    wave1_validate_min(
        "erosion.wave1.slplen_m",
        inputs.slplen_m,
        WB11_ZERO_THRESHOLD,
    )?;
    wave1_validate_min(
        "erosion.wave1.cntlen_m",
        inputs.cntlen_m,
        WB11_ZERO_THRESHOLD,
    )?;
    wave1_validate_min(
        "erosion.wave1.rspace_m",
        inputs.rspace_m,
        WB11_ZERO_THRESHOLD,
    )?;
    wave1_validate_min("erosion.wave1.width_m", inputs.width_m, WB11_ZERO_THRESHOLD)?;
    wave1_validate_min(
        "erosion.wave1.field_width_m",
        inputs.field_width_m,
        WB11_ZERO_THRESHOLD,
    )?;
    wave1_validate_min(
        "erosion.wave1.effdrn_s",
        inputs.effdrn_s,
        WB11_ZERO_THRESHOLD,
    )?;
    // Interrill-supply operands: strict-positive on interrill-active
    // quanta, but a theta-suppressed quantum (`qout <= qin`, the
    // decreasing-flow / full-reinfiltration case where `param.for:540`
    // zeroes theta) legitimately carries zero rainfall-excess operands —
    // there `effdrr` may be 0 (still finite/non-negative, fail-closed).
    let theta_suppressed_quantum =
        inputs.theta_suppressed || inputs.peakro_m_s * inputs.efflen_m <= inputs.qin_m2_s;
    if theta_suppressed_quantum {
        wave1_validate_min("erosion.wave1.effdrr_s", inputs.effdrr_s, 0.0)?;
    } else {
        wave1_validate_min(
            "erosion.wave1.effdrr_s",
            inputs.effdrr_s,
            WB11_ZERO_THRESHOLD,
        )?;
    }
    wave1_validate_min("erosion.wave1.kr_s_m", inputs.kr_s_m, WB11_ZERO_THRESHOLD)?;
    wave1_validate_min("erosion.wave1.kradjf", inputs.kradjf, WB11_ZERO_THRESHOLD)?;
    wave1_validate_min("erosion.wave1.shcrit_pa", inputs.shcrit_pa, 0.0)?;
    wave1_validate_min("erosion.wave1.tcadjf", inputs.tcadjf, WAVE1_MIN_TCADJF)?;
    wave1_validate_min("erosion.wave1.detinr_kg_s_m2", inputs.detinr_kg_s_m2, 0.0)?;
    wave1_validate_min(
        "erosion.wave1.shrsol_pa",
        inputs.shrsol_pa,
        WB11_ZERO_THRESHOLD,
    )?;
    wave1_validate_min(
        "erosion.wave1.tcend_kg_s_m",
        inputs.tcend_kg_s_m,
        WAVE1_TCEND_FLOOR,
    )?;
    wave1_validate_min("erosion.wave1.ktrato", inputs.ktrato, WB11_ZERO_THRESHOLD)?;
    wave1_validate_min("erosion.wave1.veleff_m_s", inputs.veleff_m_s, 0.0)?;
    wave1_validate_min("erosion.wave1.beta", inputs.beta, 0.0)?;
    wave1_validate_min("erosion.wave1.strldn", inputs.strldn, 0.0)?;
    Ok(())
}

fn wave1_validate_min(
    field: &'static str,
    value: f64,
    minimum: f64,
) -> Result<(), DirectRuntimeError> {
    validate_finite(field, value)?;
    // Positive minimums are strict (a zeroed operand is a missing operand,
    // fail-closed); zero minimums keep the workspace float-noise slack.
    if minimum > 0.0 {
        if value < minimum {
            return Err(DirectRuntimeError::DirectDomainViolation { field });
        }
    } else if value < minimum - WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation { field });
    }
    Ok(())
}

/// `xinflo.for` (no-contours branch): `qostar` and the normalized
/// per-segment shear/transport coefficients.
// The exact `qostar == -1.0` displacement is legacy branch semantics
// (`xinflo.for:175`).
#[allow(clippy::float_cmp)]
fn wave1_xinflo(
    segments: &[DirectWave1SlopeSegment],
    qout: f64,
    qin: f64,
    efflen_m: f64,
    slplen_m: f64,
) -> (f64, Vec<Wave1SegmentCoefficients>) {
    let del = qout - qin;
    let mut qostar = if qout <= 0.0 {
        -efflen_m / slplen_m
    } else if del.abs() > WAVE1_QOSTAR_DELTA_THRESHOLD {
        if qin <= 0.0 { 0.0 } else { qin / del }
    } else if del >= 0.0 {
        qin / WAVE1_QOSTAR_DELTA_THRESHOLD
    } else {
        -qin / WAVE1_QOSTAR_DELTA_THRESHOLD
    };

    let coefficients = if qout > 0.0 {
        if qostar == -1.0 {
            qostar = WAVE1_QOSTAR_MINUS_ONE_SHIFT;
        }
        segments
            .iter()
            .map(|segment| {
                let denom = qostar + 1.0;
                let a = segment.a / denom;
                let b = (segment.a * qostar + segment.b) / denom;
                let c = segment.b * qostar / denom;
                Wave1SegmentCoefficients {
                    xu: segment.xu,
                    xl: segment.xl,
                    a,
                    b,
                    c,
                    atc: a,
                    btc: b,
                    ctc: c,
                }
            })
            .collect()
    } else {
        if qostar.abs() < WAVE1_QOSTAR_DECREASING_FLOOR {
            qostar = -WAVE1_QOSTAR_DECREASING_FLOOR;
        }
        segments
            .iter()
            .map(|segment| {
                let a = segment.a / qostar;
                let b = (segment.a * qostar + segment.b) / qostar;
                let c = segment.b;
                Wave1SegmentCoefficients {
                    xu: segment.xu,
                    xl: segment.xl,
                    a,
                    b,
                    c,
                    atc: a,
                    btc: b,
                    ctc: c,
                }
            })
            .collect()
    };
    (qostar, coefficients)
}

/// `param.for` normalized drivers from the raw operand payload
/// (INV-SED-007: finite denominators, adjusted Chapter-7 operands).
fn wave1_param_drivers(
    inputs: &DirectWave1ContinuityInputs,
    qout: f64,
    qostar: f64,
) -> Result<Wave1Drivers, DirectRuntimeError> {
    let tcend = inputs.tcend_kg_s_m.max(WAVE1_TCEND_FLOOR);

    // Rill drivers (`param.for:396-410`).
    let eta = if inputs.surface_frozen {
        0.0
    } else {
        inputs.cntlen_m * inputs.kr_s_m * inputs.kradjf * inputs.shrsol_pa / tcend
    };
    validate_wave1_nonnegative("erosion.wave1.eta", eta)?;
    let taucn = inputs.tcadjf * inputs.shcrit_pa / inputs.shrsol_pa;
    validate_wave1_nonnegative("erosion.wave1.taucn", taucn)?;

    // Interrill driver (`param.for:540-551`).
    let theta = if inputs.theta_suppressed || qout <= inputs.qin_m2_s {
        0.0
    } else {
        (inputs.cntlen_m * inputs.detinr_kg_s_m2 / tcend) * (inputs.effdrr_s / inputs.effdrn_s)
    };
    validate_wave1_nonnegative("erosion.wave1.theta", theta)?;

    // Deposition driver (`param.for:593-625`).
    let mut pkro = -1.0e-10;
    if qout > 0.0 {
        pkro = (qout - inputs.qin_m2_s) / inputs.slplen_m;
    } else if inputs.qin_m2_s > 0.0 {
        if inputs.efflen_m > 1.0e-10 {
            pkro = -inputs.qin_m2_s / inputs.efflen_m;
        } else {
            pkro = -1.0e-10;
        }
    }
    let mut phi = if pkro.abs() >= WAVE1_PKRO_ZERO_THRESHOLD {
        inputs.beta * inputs.veleff_m_s / pkro
    } else if qostar >= 0.0 {
        WAVE1_MAX_PHI
    } else {
        -WAVE1_MAX_PHI
    };
    phi = phi.clamp(-WAVE1_MAX_PHI, WAVE1_MAX_PHI);
    validate_finite("erosion.wave1.phi", phi)?;

    Ok(Wave1Drivers {
        eta,
        taucn,
        theta,
        phi,
        ktrato: inputs.ktrato,
        qostar,
        qout_m2_s: qout,
    })
}

fn validate_wave1_nonnegative(field: &'static str, value: f64) -> Result<(), DirectRuntimeError> {
    validate_finite(field, value)?;
    if value < -WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::NegativeDirectValue { field });
    }
    Ok(())
}

/// `sloss.for` denormalization + conservation gates.
fn wave1_totals(
    inputs: &DirectWave1ContinuityInputs,
    drivers: &Wave1Drivers,
    grid: &Wave1RouteGrid,
    coefficients: &[Wave1SegmentCoefficients],
) -> Result<DirectWave1ContinuityState, DirectRuntimeError> {
    // Dimensional per-point load scale (kg per m of hillslope width):
    // `dslod = load * effdrn * tcend * width / rspace` (`sloss.for:166`).
    let tcend = inputs.tcend_kg_s_m.max(WAVE1_TCEND_FLOOR);
    let denorm = inputs.effdrn_s * tcend * inputs.width_m / inputs.rspace_m;
    validate_finite("erosion.wave1.denorm_scale", denorm)?;

    let inflow_kg_m = grid.load[0] * denorm;
    let exported_kg_m = grid.load[DIRECT_WAVE1_GRID_POINTS - 1] * denorm;
    let mut total_detach_kg_m = 0.0;
    let mut total_depos_kg_m = 0.0;
    for j in 1..DIRECT_WAVE1_GRID_POINTS {
        let delta = (grid.load[j] - grid.load[j - 1]) * denorm;
        if delta >= 0.0 {
            total_detach_kg_m += delta;
        } else {
            total_depos_kg_m -= delta;
        }
    }

    // Hard publication-closure gate: telescoping identity between the
    // signed per-cell sums and the boundary loads (INV-SED-010 payload
    // consistency).
    let closure_residual = (exported_kg_m - inflow_kg_m) - (total_detach_kg_m - total_depos_kg_m);
    // The identity's scale must span EVERY operand in it: a pure-deposition
    // quantum (ADR-0036 full-reinfiltration hour: exported = detach = 0,
    // inflow = deposition > 0) would otherwise degenerate the relative gate
    // to an absolute 1e-18 and reject f64 accumulation noise on a closed
    // balance (latent pre-E.2: single-OFE zero-qin days always detached).
    let closure_scale = exported_kg_m
        .abs()
        .max(inflow_kg_m.abs())
        .max(total_detach_kg_m.abs())
        .max(total_depos_kg_m.abs())
        .max(WAVE1_CLOSURE_ABS_FLOOR);
    if closure_residual.abs() > WAVE1_PUBLICATION_CLOSURE_REL_TOL * closure_scale {
        return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
            field: "erosion.wave1.publication_closure",
        });
    }

    // Continuity flux residual (INV-SED-001): per-cell |dG - trapz(rate)|
    // over cells whose endpoints share a computed region and carry no
    // clamp; reported always, hard-gated at the named discretization
    // tolerance.
    let mut flux_residual = 0.0;
    let mut flux_scale = 0.0;
    for j in 1..DIRECT_WAVE1_GRID_POINTS {
        let delta = grid.load[j] - grid.load[j - 1];
        flux_scale += delta.abs();
        if grid.region[j] != grid.region[j - 1] || grid.clamped[j] || grid.clamped[j - 1] {
            continue;
        }
        match grid.region[j] {
            Wave1PointRegion::Detachment | Wave1PointRegion::Deposition => {
                let rate_prev = grid.detach[j - 1] + drivers.theta;
                let rate_here = grid.detach[j] + drivers.theta;
                let trapezoid = 0.5 * (rate_prev + rate_here) * WAVE1_GRID_DX;
                flux_residual += (delta - trapezoid).abs();
            }
            Wave1PointRegion::FlowEnd | Wave1PointRegion::Untouched => {}
        }
    }
    let flux_gate_scale = flux_scale.max(WAVE1_CLOSURE_ABS_FLOOR);
    if flux_residual > WAVE1_FLUX_CLOSURE_REL_TOL * flux_gate_scale {
        return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
            field: "erosion.wave1.flux_closure",
        });
    }

    // Interrill contribution surface (`sloss.for:229`).
    let interrill_kg_m2 = (drivers.theta * tcend * inputs.effdrn_s * inputs.width_m)
        / (inputs.rspace_m * inputs.slplen_m);

    // Toe concentration (`sloss.for:314` form): avsole / (runoff * efflen).
    let concentration = if inputs.peakro_m_s <= 0.0 {
        0.0
    } else {
        exported_kg_m / (inputs.runoff_depth_m * inputs.efflen_m)
    };
    validate_wave1_nonnegative("erosion.wave1.sediment_concentration", concentration)?;

    validate_wave1_nonnegative("erosion.wave1.exported_sediment", exported_kg_m)?;
    validate_wave1_nonnegative("erosion.wave1.total_detachment", total_detach_kg_m)?;
    validate_wave1_nonnegative("erosion.wave1.total_deposition", total_depos_kg_m)?;

    // The legacy Fortran-`save` end state (`param.for:368-390`): the LAST
    // segment's final coefficient values; with no inflow the transport set
    // mirrors the shear set (the `qin <= 0` do-20 reset).
    let (end_shear_coefficients, end_transport_coefficients) =
        coefficients
            .last()
            .map_or(((0.0, 0.0, 0.0), (0.0, 0.0, 0.0)), |last| {
                let shear = (last.a, last.b, last.c);
                let transport = if inputs.qin_m2_s > 0.0 {
                    (last.atc, last.btc, last.ctc)
                } else {
                    shear
                };
                (shear, transport)
            });

    Ok(DirectWave1ContinuityState {
        active: true,
        flux_refused_quanta: 0,
        end_shear_coefficients,
        end_transport_coefficients,
        eta: drivers.eta,
        taucn: drivers.taucn,
        theta: drivers.theta,
        phi: drivers.phi,
        qostar: drivers.qostar,
        qout_m2_s: drivers.qout_m2_s,
        load: grid.load.clone(),
        tcap: grid.tcap.clone(),
        detach: grid.detach.clone(),
        exported_sediment_kg_m: exported_kg_m,
        inflow_sediment_kg_m: inflow_kg_m,
        total_detachment_kg: total_detach_kg_m * inputs.field_width_m,
        total_deposition_kg: total_depos_kg_m * inputs.field_width_m,
        sediment_concentration_kg_m3: concentration,
        interrill_contribution_kg_m2: interrill_kg_m2,
        publication_closure_residual_kg_m: closure_residual,
        flux_closure_residual: flux_residual,
        flux_closure_scale: flux_scale,
    })
}

/// Whether a day routes sediment (`contin.for` `norun == 1` above the
/// `passby` event-size bypass): `false` on non-runoff and sub-gate days.
/// Single-sources the legacy activation gate so operand assembly can gate
/// **before** computing routed operands (matching the legacy
/// gate-before-`frcfac`/`param` call order). `runoff_depth_m`/`peakro_m_s`
/// are assumed already finite-validated by the caller.
#[must_use]
pub fn wave1_day_routes_sediment(runoff_depth_m: f64, peakro_m_s: f64) -> bool {
    if runoff_depth_m <= 0.0 || peakro_m_s <= 0.0 {
        return false;
    }
    !(runoff_depth_m <= WAVE1_PASSBY_RUNOFF_M && peakro_m_s <= WAVE1_PASSBY_PEAKRO_M_S)
}

/// ADR-0036 D1 / `INV-SED-013`: whether a solve quantum is hydraulically
/// active — local outflow routes sediment OR upstream inflow is positive.
/// The `qin > 0` limb covers the full-reinfiltration case (`qout <= 0`
/// with incoming load that must deposit — the legacy `xinflo.for:206`
/// `qshear = qin·rspace` branch); excess-only activation would skip it.
#[must_use]
pub fn wave1_quantum_is_hydraulically_active(
    runoff_depth_m: f64,
    peakro_m_s: f64,
    qin_m2_s: f64,
) -> bool {
    wave1_day_routes_sediment(runoff_depth_m, peakro_m_s) || qin_m2_s > 0.0
}

/// Compute the Wave-1 single-OFE sediment-continuity solve for one runoff
/// day. Returns the inactive state on non-runoff days and on events below
/// the legacy sediment-routing size gate (`contin.for:977` `passby`).
///
/// Validation is split to match the legacy call order: `contin.for` gates
/// on `norun`/`passby` BEFORE `frcfac`/`xinflo`/`param` ever run, so the
/// routed-event operand payload (`effdrn`, `shrsol`, `tcend`, ...) is
/// only required on days the event actually routes. On non-routed days
/// the runtime legitimately supplies zeroed sediment operands (WB16
/// publishes `runoff_duration_s = 0` without runoff), and this function
/// must return the inactive state, not a typed operand error.
pub fn compute_direct_wave1_continuity(
    inputs: &DirectWave1ContinuityInputs,
) -> Result<DirectWave1ContinuityState, DirectRuntimeError> {
    compute_direct_wave1_continuity_quantum(inputs, false)
}

/// The passby-exempt solve entry (ADR-0036 D1 / `INV-SED-013`): hour
/// quanta of a day that already passed the day-level `passby` event-size
/// gate must NOT re-apply that event-scale bound at hour scale (a routed
/// 12 mm day spread over hours would otherwise never route an hour).
/// `passby_exempt = false` preserves the legacy day/event semantics for
/// the daily form and every existing caller.
pub fn compute_direct_wave1_continuity_quantum(
    inputs: &DirectWave1ContinuityInputs,
    passby_exempt: bool,
) -> Result<DirectWave1ContinuityState, DirectRuntimeError> {
    if !inputs.enabled {
        return Ok(DirectWave1ContinuityState::inactive());
    }

    // Activation operands are validated fail-closed even on inert days (a
    // NaN runoff must never silently pass the `<= 0` activation branch).
    wave1_validate_min("erosion.wave1.peakro_m_s", inputs.peakro_m_s, 0.0)?;
    wave1_validate_min("erosion.wave1.runoff_depth_m", inputs.runoff_depth_m, 0.0)?;
    wave1_validate_min("erosion.wave1.qin_m2_s", inputs.qin_m2_s, 0.0)?;

    // Activation (ADR-0036 D1 / INV-SED-013): local runoff above the
    // legacy event-size bypass (`contin.for` `norun == 1`), OR positive
    // upstream inflow — the full-reinfiltration quantum (`qout <= 0`,
    // `qin > 0`) must still solve so the incoming load deposits
    // (`xinflo.for:206` `qshear = qin·rspace` branch). Passby-exempt
    // quanta still require positive flow or inflow.
    let active = if passby_exempt {
        (inputs.runoff_depth_m > 0.0 && inputs.peakro_m_s > 0.0) || inputs.qin_m2_s > 0.0
    } else {
        wave1_quantum_is_hydraulically_active(
            inputs.runoff_depth_m,
            inputs.peakro_m_s,
            inputs.qin_m2_s,
        )
    };
    if !active {
        return Ok(DirectWave1ContinuityState::inactive());
    }

    // Routed event: the full sediment operand payload is now required.
    validate_wave1_inputs(inputs)?;

    // `xinflo.for:150`: unit outflow discharge.
    let qout = inputs.peakro_m_s * inputs.efflen_m;
    let (qostar, coefficients) = wave1_xinflo(
        &inputs.segments,
        qout,
        inputs.qin_m2_s,
        inputs.efflen_m,
        inputs.slplen_m,
    );
    validate_finite("erosion.wave1.qostar", qostar)?;

    // Inter-OFE continuity rewrite (`param.for:249-390`, INV-SED-008):
    // applied only for a downstream OFE with positive local outflow AND
    // positive inflow — the legacy `iplane > 1 ∧ qout > 0 ∧ qin > 0` guard.
    let mut coefficients = coefficients;
    if let Some(inter_ofe) = &inputs.inter_ofe {
        if qout > 0.0 && inputs.qin_m2_s > 0.0 {
            wave1_apply_inter_ofe_continuity(&mut coefficients, inputs, inter_ofe, qostar)?;
        }
    }

    let drivers = wave1_param_drivers(inputs, qout, qostar)?;
    let grid = wave1_route(&coefficients, &drivers, inputs.strldn)?;
    wave1_totals(inputs, &drivers, &grid, &coefficients)
}

/// `param.for:249-390`: re-derive the receiving OFE's normalized shear and
/// transport coefficient polynomials so both are continuous across the OFE
/// boundary, from the prior lane's end state. Every legacy singular guard
/// is preserved: the `spart1`/`shrspv` validity test (fall back to the
/// plain `xinflo` coefficients via the zero-slope `qostar` substitution),
/// the `sratio`/`tprod` 1e-5 floors, the 2012 `shrati <= 1e12` overflow
/// cap, and the ±0.001 denominator floors.
fn wave1_apply_inter_ofe_continuity(
    coefficients: &mut [Wave1SegmentCoefficients],
    inputs: &DirectWave1ContinuityInputs,
    inter_ofe: &Wave1InterOfeContinuity,
    qostar: f64,
) -> Result<(), DirectRuntimeError> {
    validate_finite("erosion.wave1.inter_ofe.shrspv", inter_ofe.shrspv_pa)?;
    validate_finite("erosion.wave1.inter_ofe.tcprev", inter_ofe.tcprev_kg_s_m)?;
    validate_finite("erosion.wave1.inter_ofe.ktrprv", inter_ofe.ktrprv)?;
    let (anflst, bnflst, cnflst) = inter_ofe.prior_shear_last;
    let (atclst, btclst, ctclst) = inter_ofe.prior_transport_last;
    validate_finite("erosion.wave1.inter_ofe.anflst", anflst)?;
    validate_finite("erosion.wave1.inter_ofe.bnflst", bnflst)?;
    validate_finite("erosion.wave1.inter_ofe.cnflst", cnflst)?;
    validate_finite("erosion.wave1.inter_ofe.atclst", atclst)?;
    validate_finite("erosion.wave1.inter_ofe.btclst", btclst)?;
    validate_finite("erosion.wave1.inter_ofe.ctclst", ctclst)?;

    // The receiver's shrsol/tcend/ktrato are the same operands the drivers
    // consume; the first segment's raw slope intercept `b(2)` comes from
    // the un-normalized profile segments.
    let first_segment_b = inputs
        .segments
        .first()
        .map(|segment| segment.a * segment.xu + segment.b)
        .ok_or(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.wave1.inter_ofe.first_segment",
        })?;
    let tcend = inputs.tcend_kg_s_m.max(WAVE1_TCEND_FLOOR);

    let spart1 = anflst + bnflst + cnflst;
    let (shrati, tcrati) = if spart1 > 1.0e-5 && inter_ofe.shrspv_pa > 0.0 {
        // Shear continuity ratio (`param.for:260-282`).
        let sterm1 = first_segment_b / spart1;
        let sterm2 = (inter_ofe.shrspv_pa / inputs.shrsol_pa).powf(1.5);
        let sratio = if sterm2.abs() > 1.0e-10 {
            (sterm1 / sterm2) - 1.0
        } else if sterm1 >= 0.0 {
            1.0e-5
        } else {
            -1.0e-5
        };
        let shrati = if sratio.abs() > 1.0e-5 {
            1.0 / sratio
        } else if sratio >= 0.0 {
            1.0 / 1.0e-5
        } else {
            -1.0 / 1.0e-5
        };

        // Transport continuity ratio (`param.for:284-303`).
        let tpart1 = atclst + btclst + ctclst;
        let tterm1 = if tpart1 > 1.0e-5 {
            first_segment_b / tpart1
        } else {
            first_segment_b / 1.0e-5
        };
        let tcprev = if inter_ofe.tcprev_kg_s_m.abs() > 1.0e-10 {
            inter_ofe.tcprev_kg_s_m
        } else {
            1.0e-10
        };
        let ktrprv = if inter_ofe.ktrprv.abs() > 1.0e-10 {
            inter_ofe.ktrprv
        } else {
            1.0e-10
        };
        let tterm2 = (tcend / tcprev) * (inputs.ktrato / ktrprv);
        let tprod = (tterm1 * tterm2) - 1.0;
        let tcrati = if tprod.abs() > 1.0e-5 {
            1.0 / tprod
        } else if tprod >= 0.0 {
            1.0 / 1.0e-5
        } else {
            -1.0 / 1.0e-5
        };
        (shrati, tcrati)
    } else {
        // Zero transport capacity at the boundary (zero-slope condition):
        // `qostar` keeps shear/transport continuous (`param.for:308-318`).
        (qostar, qostar)
    };

    // Coefficient rewrite (`param.for:322-390`), per segment, with the
    // overflow cap and denominator floors.
    let shrati = if shrati > 1.0e12 { 1.0e12 } else { shrati };
    for (segment, coefficient) in inputs.segments.iter().zip(coefficients.iter_mut()) {
        let raw_a = segment.a;
        let raw_b = segment.b;

        let mut denom = shrati + 1.0;
        if denom.abs() < 1.0e-3 {
            denom = if denom >= 0.0 { 0.001 } else { -0.001 };
        }
        coefficient.a = raw_a / denom;
        coefficient.b = (raw_a * shrati + raw_b) / denom;
        coefficient.c = raw_b * shrati / denom;

        let mut denom = tcrati + 1.0;
        if denom.abs() < 1.0e-3 {
            denom = if denom >= 0.0 { 0.001 } else { -0.001 };
        }
        coefficient.atc = raw_a / denom;
        coefficient.btc = (raw_a * tcrati + raw_b) / denom;
        coefficient.ctc = raw_b * tcrati / denom;

        validate_finite("erosion.wave1.inter_ofe.coefficient_a", coefficient.a)?;
        validate_finite("erosion.wave1.inter_ofe.coefficient_atc", coefficient.atc)?;
    }
    Ok(())
}

/// `profil.for`: derive the normalized slope-segment fit from the slope
/// profile points `(x_m, slope)` (dimensional distance in meters, slope as
/// tangent). Production seed derivation for the Wave-1 payload; also used
/// by contract tests.
pub fn derive_wave1_slope_segments(
    points: &[(f64, f64)],
    slplen_m: f64,
    avgslp: f64,
) -> Result<Vec<DirectWave1SlopeSegment>, DirectRuntimeError> {
    if points.len() < 2 {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.wave1.slope_points",
        });
    }
    if !slplen_m.is_finite() || slplen_m <= 0.0 || !avgslp.is_finite() || avgslp <= 0.0 {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "erosion.wave1.slope_geometry",
        });
    }
    let mut segments = Vec::with_capacity(points.len() - 1);
    for window in points.windows(2) {
        let (x0_m, s0) = window[0];
        let (x1_m, s1) = window[1];
        validate_finite("erosion.wave1.slope_point_x", x0_m)?;
        validate_finite("erosion.wave1.slope_point_x", x1_m)?;
        validate_finite("erosion.wave1.slope_point_s", s0)?;
        validate_finite("erosion.wave1.slope_point_s", s1)?;
        let xu = x0_m / slplen_m;
        let xl = x1_m / slplen_m;
        if xl <= xu {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "erosion.wave1.slope_point_order",
            });
        }
        let sstar_u = s0 / avgslp;
        let sstar_l = s1 / avgslp;
        let a = (sstar_l - sstar_u) / (xl - xu);
        let b = sstar_u - a * xu;
        segments.push(DirectWave1SlopeSegment { xu, xl, a, b });
    }
    Ok(segments)
}
