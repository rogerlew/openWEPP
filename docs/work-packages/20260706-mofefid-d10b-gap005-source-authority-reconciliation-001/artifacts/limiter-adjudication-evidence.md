# Limiter / Scheme Adjudication Evidence (D10B-S1, Leg A)

Status: executed
Evidence mode: Ran (page renders + text extraction of acquired primaries;
code read) — adjudication conclusions are source-bound, causality claims
for Case-4 metrics remain harness questions (S3/S4).

## A1. Flux-limiter branch — ADJUDICATED: R-63 (11c) is a transcription error

- R-63 printed (11c): `phi = min(2r,1) for r < 0; 0 for r >= 0`
  (`references/copyrighted/Papanicolaou2018.md` §2.3).
- Davis 1984 eq. (3.20), read visually from the rendered p. 9 of
  `19840021490.pdf`: `phi(r) = min(2r,1) if r > 0; 0 if r <= 0`.
- Mingham 2001 eq. (31f), extracted from `mingham2001.pdf`:
  `phi(x) = min(2x,1), x > 0; 0, x <= 0`.
- Both sources R-63 cites for the scheme agree; the printed (11c) swaps the
  branch conditions. Under the printed form, dissipation is applied at FULL
  strength in smooth/monotone regions (`r >= 0 -> phi = 0 -> Gr = 0.5 Cf`)
  and AMPLIFIED at extrema (`phi < 0 -> 1 - phi > 1`), destroying the
  scheme's second-order accuracy exactly where the limiter should disable
  dissipation.
- Current code implements the printed (11c) literally:
  `kinematic_wave.rs` `phi()` — with the intent confusion visible in its own
  `limiter_ratio` comment ("treat as monotone (phi -> 0, standard
  dissipation)": under the source convention, monotone smooth regions take
  `phi -> 1`, i.e. NO dissipation).
- BINDING: the source-correct branch is `phi(r) = max(0, min(2r, 1))`
  (equivalently: `min(2r,1)` for `r > 0`, else `0`).

## A2. Ratio form — ADJUDICATED: two-sided face form is the source form

- R-63 (11a)+(11d) use ONE ratio per cell
  (`r_i = (h_i - h_{i-1})/(h_{i+1} - h_i)`) and one `Gr_i` per cell.
- Davis (3.17)-(3.18): dissipation coefficients live at faces `k+1/2` and
  carry BOTH `phi(r+_k)` and `phi(r-_{k+1})` (upwind and downwind ratios).
- Mingham (31a): `TVD_i = [G(r+_i) + G(r-_{i+1})] dU_{i+1/2}
  - [G(r+_{i-1}) + G(r-_i)] dU_{i-1/2}`, with (31g) defining `r+_i` and
  `r-_i` (scalar-product forms; for a scalar state they reduce to
  `r+_i = dU_{i-1/2}/dU_{i+1/2}`, `r-_i = dU_{i+1/2}/dU_{i-1/2}`).
- BINDING: the face-based two-sided form. Note the conservation corollary:
  face-based accumulation with zero dissipative flux at the domain
  boundary faces telescopes EXACTLY; the S0 ledger measured the current
  cell-indexed boundary-exempt form leaking mass anti-convergently
  (`seam-conservation-ledger.md`, Mechanism 2). The source-faithful form
  and the conservative form are the same edit.

## A3. `Cf` and CFL — ADJUDICATED: current implementation matches source

- Mingham `C(x) = x(1-x), x <= 0.5; 0.25, x > 0.5` — matches
  `kinematic_wave.rs` `cf()` exactly. (Davis's original uses
  `|v|(1-|v|)/2` without the cap; the cap variant is Mingham's, and R-63
  (11e) follows Mingham. The implementation follows the R-63/Mingham
  variant — no change required.)
- Mingham runs at Courant 0.9 "to ensure stability" — matches
  `CFL_TARGET = 0.9`. No change required.
- CELERITY CAVEAT (recorded, not a Leg-A defect): the solver's celerity
  expression `c = 1.5 alpha h^0.5` treats `alpha` as depth-independent.
  For depth-dependent friction (Manning limb: true `dq/dh = (5/3) q/h`;
  laminar `k_o/Re` limb: true equilibrium `dq/dh = 3 q/h`) the true
  kinematic celerity exceeds the expression. Within one step the scheme's
  flux is `alpha_frozen h^1.5`, so the per-step CFL bound is consistent
  with the discrete flux actually advanced; the alpha lag is a
  first-order-in-time effect that the S3 convergence harness measures
  directly. Adjudication: no source contradiction; convergence evidence
  decides whether additional treatment is required.

## A4. `alpha` update timing — ADJUDICATED: paper silent; frozen-alpha is
consistent; Tseng is the named precedent surface

- R-63 does not state when `alpha` (eq. 9/14) is re-evaluated. The current
  implementation evaluates a fixed-point `alpha` per cell per step from the
  pre-step state (D14 OPT-1) and freezes it through
  predictor/corrector/commit.
- Tseng 2010 (R-63's citation for the applied scheme) develops an IMPLICIT
  MacCormack for exactly this friction-nonlinearity class and judges all
  variants against analytic solutions. The explicit variant with lagged
  coefficients is the comparison baseline there — i.e., frozen-alpha
  explicit stepping is a recognized member of the family, adjudicated by
  convergence, not by prescription.
- BINDING: keep frozen-alpha; the S3 oracle convergence sweep is the
  acceptance instrument. If convergence to the oracle stalls at a
  first-order floor attributable to alpha lag, that becomes a NEW named
  defect with Tseng's implicit treatment as its authority anchor.

## A5. Boundary treatment — ADJUDICATED: R-63 silent; conservation gate owns

- R-63 §2.3 specifies no upstream/downstream boundary stencil and no
  inter-OFE handoff discretization. These are openWEPP-owned machinery.
- Authority: the conservation invariant (program hard gate). The S0 ledger
  (`seam-conservation-ledger.md`) measured the current treatment's booking
  mismatches (Mechanisms 1, 3, 4, 5). Correction classes bound for S4:
  - upstream: make the discrete injection equal the physical BC
    (`q_up dt`) by giving the predictor an upstream ghost flux, or book
    the scheme-actual mean — the S4 decision records which, with the
    conservation identity as acceptance;
  - downstream: replace the over-discharging extrapolated predictor ghost
    with a boundary treatment whose booked and actual outflow agree
    (kinematic outflow carries only downstream-traveling information, so a
    one-sided/upwind-consistent closure at the outlet is admissible without
    any external prescription);
  - TVD: face-based accumulation (A2) with zero boundary-face flux;
  - handoff: flux-integral (piecewise-linear-exact) injection of the
    upstream sampled hydrograph; ledger-based seam volumes;
  - clamp booking: half-weight stage clamps (commit averages the stages).

## A6. Iwagaki friction mapping — ADJUDICATED: bind the primary's own law

- Iwagaki 1955 (R-74) experiment (B) IS the D-val Case-4 configuration
  (geometry/slopes/supplies/duration extracted verbatim; see
  `source-acquisition-record.md`), analyzed with a Manning-type resistance,
  `n = 0.009` (m-s units).
- The current D-val runs Case 4 with `k_o = 200` (transitional
  `f = k_o/Re` at `I = 0`), an operand the paper never specifies — D10
  correctly refused to tune it.
- BINDING: the Case-4 acceptance configuration uses the PRIMARY'S law —
  Manning `n = 0.009` — via the definitional identity
  `f = 8 g n^2 / h^(1/3)` (equivalently `alpha = sqrt(S) h^(1/6) / n`,
  `q = (sqrt(S)/n) h^(5/3)`), so both the oracle and the solver run the
  same friction closure and the comparison isolates NUMERICS. `k_o`
  disappears from Case-4 acceptance entirely (it remains a D11-owned
  production operand elsewhere). The oracle additionally records the
  wide-channel simplification (R = h) as the like-for-like form matching
  the solver's PDE; Iwagaki's own sidewall-corrected `R = Bh/(B+2h)`
  (B = 19.6 cm) and laminar/transitional switching (Re 500/1500) are
  recorded as fidelity caveats, not acceptance surfaces.

## Consequence

All five surfaces named by the D10 hold are now bound (A1, A2, A6) or
adjudicated-no-change/measured (A3, A4, A5). No surface remains
authority-blocked; the package proceeds to the S2 contract amendment and
the S3/S4 conversion. D10's rejected limiter-flip trial is superseded:
it flipped ONE branch in isolation, was judged against the demoted
digitized-trace oracle, and did not carry the face-form (A2) or
conservation (A5) corrections that the S0 ledger shows are coupled.
