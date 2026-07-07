# Codex Review Request — D15A activation + LANED-T3 hybrid stepping

Task: review commit `bd64d2c8` on `main` (diff base `94a7ac3a`; the
`9f536aad` scaffold commit between them is docs-only). Two packages landed in
one commit (interleaved shared files); they need DIFFERENT review depths.

Repo: `/home/workdir/openWEPP`. Read `AGENTS.md` +
`docs/work-packages/AGENTS.md` first. Evidence rules apply: label `Static:`
vs `Ran:`; you may execute any gate (focused suites are fast; the H2637
ignored pair is ~7 min; H2637 endpoint runs need the fixture recipe in
`docs/work-packages/20260706-mofefid-d15-active-owner-optimization-001/artifacts/baseline-profile.md`).

## Package 1 — D15A (RE-CHECK, D10B pattern)

`docs/work-packages/20260706-mofefid-d15-active-owner-optimization-001/`
closed EXECUTED-COMPLETE with in-session dual reviews (both
GO-WITH-AMENDMENTS; all accepted findings fixed —
`artifacts/review-disposition.md`). Requested: an independent Codex re-check
that the dispositions hold on the final tree, focusing on:

1. The QA-H2 repair: the soil↔router SEAM cross-ledger hard-fail
   (`laned_active.rs::laned_active_enforce_day_closure`, check (c)) — is it
   genuinely independent-ledger (solver booking vs `q_runoff × A`), and do
   the hourly forcing breakpoints (`route_single_ofe` callers) make it
   exact as claimed?
2. The two-phase active day loop
   (`03_executor.rs::run_laned_active_publication_stream`) — cross-lane
   ordering vs the default loop (lateral transfer edge, erosion-inflow
   intake refresh, commit ordering).
3. `SC-OFEROUTE-001` rev 27 text vs code (selector, window/reset, erosion
   tail-fold + full-mesh-hold degeneracy, latqcc-inside-subsurface-loss
   booking, uniform-fallback disposition, mesh-basis conversion).
4. INV-OFEROUTE-010: default/off byte identity claims
   (`artifacts/protected-output-byte-identity.md`) — the off path must be
   textually untouched; verify the dispatch isolation.

## Package 2 — LANED-T3 (PRIMARY DUAL REVIEW — required before rev-28 settles)

`docs/work-packages/20260706-laned-router-t3-hybrid-implicit-stepping-001/`
landed I0-I2: the implicit KW stepper + hybrid stepping behind the
EXPERIMENTAL `OPENWEPP_LANED_ACTIVE_IMPLICIT=1` selector
(`SC-OFEROUTE-001` rev 28). This package has NOT had its dual review —
your review is the code-correctness lane; a QA/governance lane should run
separately or be covered explicitly in your findings.

Read first: `artifacts/i0-scheme-design.md` (design + I1 correction),
`artifacts/i1-implicit-stepper-evidence.md` (ladder + the two discoveries),
`artifacts/i2-hybrid-evidence.md` (honest bottom line), the rev-28
revision-history entry.

Adversarial questions (in priority order):

1. **Filippov closure correctness**
   (`implicit_recession.rs::solve_cell/solve_cell_on_branch`): the
   filled-jump commit `q = (rhs − h)/(Δt/Δx)` at the collapse depth is
   mass-exact by construction — but can a genuine solver failure
   (equilibrium fixed-point pathology, bracket mismanagement) masquerade as
   a "jump" and silently commit a Filippov pair where a true branch root
   existed? The LOW→HIGH→Filippov chain is the guard — check its
   completeness. Is there a MISSING unit vector that pins the Filippov path
   directly (the H2637 run exercises it only implicitly)?
2. **Basin-split determinism** (`kinematic_wave.rs::
   equilibrium_discharge_converged` + Steffensen): is the accelerated
   iteration truly a pure function of `(cell, h, rain, branch)`? The
   Steffensen basin guard accepts the accelerated point only on `q2`'s side
   of `Q_c` — can acceleration hop basins via intermediate `q1`
   evaluations in a way that changes the converged value vs plain
   iteration?
3. **Hybrid span composition** (`cascade.rs::route_single_ofe_hybrid`):
   ledger stitching across explicit/implicit spans (inflow/outflow/clamp/
   storage bookkeeping, global bin offsets, the state seam via
   `set_state`/`discharge_state`); the bit-identity claim scope
   (all-explicit windows only — pinned by
   `hybrid_is_bit_identical_on_all_explicit_windows`); the strict-mask
   predicate.
4. **The dust floor** (`implicit_step_with_discharges` residual guard,
   `DRY_DEPTH_M · dx · n`): is the floor justified, and can it hide a real
   leak class on very short meshes or many accumulated dust steps?
5. **The named aggressive-rule defect**: confirm the analysis in
   `i2-hybrid-evidence.md` (short explicit spans stranding front-arrival
   terminal-bin deficits → `NegativeOutletBin`, H2637 lane 17 day 54) and
   assess the proposed fix shape (deficit carry across span boundaries) for
   soundness before anyone implements it.
6. **No-perturbation claims**: rev-27 plain-active behavior must be
   unchanged by T3 code (dispatch isolation in
   `laned_active_route_lane`/config flag); the explicit solver and shadow
   paths must be untouched (`route_single_ofe` passes `&[]` breakpoints on
   the cascade path; `alpha_q` unchanged).
7. Fidelity posture honesty: rev 28 records tolerances as UNRATIFIED and
   the selector as evidence-gathering — flag any text that overclaims.

## Output protocol

Findings severity-ordered (High/Medium/Low) with `file:line` anchors and
concrete failure scenarios; explicit verdicts per package
(GO / GO-WITH-AMENDMENTS / NO-GO). Write results to:
- D15A re-check: append a dated section to
  `20260706-mofefid-d15-active-owner-optimization-001/artifacts/review-codex.md`.
- T3: create
  `20260706-laned-router-t3-hybrid-implicit-stepping-001/artifacts/review-codex.md`
  (+ `review-qa.md` if you cover the governance lane, else say so).
Do not modify production code; findings only. Gate rerun evidence welcome
(`cargo nextest run --workspace --profile full` was 1417/1417 at commit).
