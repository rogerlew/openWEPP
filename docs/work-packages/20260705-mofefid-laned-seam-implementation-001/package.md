# MOFEFID LANE D — SUBSURFACE SEAM IMPLEMENTATION (INV-OFEROUTE-012, solver tier)

Status: `EXECUTED — AWAITING CODEX REVIEW` (Claude-executed; operator: "scaffold and execute
with that scope", 2026-07-05). Branch: `laned-seam-implementation`.

## Scope adjustment vs the recommendation (entry recon, on record)

Recon found D4–D6 landed the router as PURE + D-val layers only —
`ofe_routing::` has NO runtime invocation (shadow or otherwise), so
"runtime shadow wiring" is necessarily its own increment regardless of
this package. THIS package therefore implements the GAP-006 seam at the
tier that exists:

- the seam machinery (`ofe_routing::seam`): the source-rate series
  (`(wb14_hourly_excess + ui_SCrunf-lineage carry)/3600`, the recorded
  unit helper), the hourly-lane precondition, the DC01
  mutual-exclusion assertion, the cascade-forcing adapter, and the
  INV-012 (c) closure-identity computation;
- gate fixture A at the SOLVER tier (crafted exfiltration-to-runoff
  through `run_cascade` with seam forcing);
- gate fixture B at the IDENTITY tier (H2637-class subsurface-dominated
  closure vector, operands drawn to the MAGPARITY01 class profile);
- SC-OFEROUTE-001 rev 5 recording the status honestly: seam implemented
  + fixtures passing at the solver/identity tier; RUNTIME wiring (and
  the real-H2637 executed vector, staged inputs located at
  `/home/workdir/wepp-forest/docs/ablation/20260430_…_h2637_…/artifacts/repro/staged/runs/`)
  remain the activation increment — INV-012's production-activation
  BLOCK stays.

## Recon items (resolved)

- **ui_SCrunf runtime surface** = `hourly_saturation_carry_m` (24-slot,
  `subsurface.rs` lateral tail → `runoff.rs` DC01 weights at `:1416` —
  the same limb, confirming the D1 "same two limbs" claim in code).
- **Erosion-coupling touchpoint (recorded, decided later):** the E.2
  sediment substrate takes its hourly shape from the DC01 weights,
  which unify the SAME two limbs this seam consumes. At routing
  activation the erosion hourly shape should derive from the ROUTED
  hydrograph (ADR-0036's "modeled hourly flow" becoming the actual
  routed flow) — an activation-increment design note, NOT this package.
- **H2637 inputs**: located (above); execution rides activation.

## Stages

1. `ofe_routing::seam` module (pure; typed fail-closed).
2. Closure-identity machinery + DC01 exclusion assertion.
3. Fixture A: 2-OFE cascade, downslope hours with zero rainfall excess
   and positive saturation carry → routed toe flow on those hours +
   cascade conservation.
4. Fixture B: H2637-class closure vector (identity closes; surface
   share ≈1%; ENV-Y inside [0.55, 0.72]).

Gates per the local-ci standard; full at branch head; Codex review.

## Execution record (2026-07-05)

- `ofe_routing::seam` (new module): `seam_source_rate_series` (the
  recorded `/3600` unit helper; fail-closed on non-finite/negative
  depths), `seam_require_hourly_lane`, `seam_assert_dc01_superseded`,
  `seam_rate_at` (the cascade-forcing sampler), and
  `seam_closure_residual_m3` (the D4 identity) with typed `SeamError`.
- **Gate fixture A (solver tier, Ran):** 2-OFE cascade, downslope OFE
  with exfiltration-only hours (zero rainfall excess) — the routed toe
  carries the exfiltrated volume (outlet > 0.5× injected), cascade
  conservation < 1e-2 relative, and NO flow before the pulse.
- **Gate fixture B (identity tier, Ran):** H2637-class operands
  (P 2,400 mm, ~99% lateral share of a 62% yield, ET 830 mm) — the D4
  identity closes to 1e-12 relative, surface share < 2%, ENV-Y inside
  [0.55, 0.72], ENV-ET in-band.
- SC-OFEROUTE-001 rev 5: INV-OFEROUTE-012 status records the tier
  honestly; the activation BLOCK stands (runtime wiring + real-H2637
  executed vector = the activation increment).

## Codex review round 1 — response record (2026-07-05)

Four findings (3 Medium + 1 Low), all CONFIRMED and fixed
(SC-OFEROUTE-001 rev 14):
1. **Versioning non-monotonic:** the frontmatter had lagged at 2 while
   the history ran to 10, and my 2026-07-05 amendments (3/4/5) collided
   with the 2026-07-02 rows. Renumbered 3/4/5 → 11/12/13; frontmatter
   → 14 with the correction row; in-body rev citations updated.
2. **Seam status contradictions:** the INV-012 evidence cell, the
   OBL-OFEROUTE-P-006 obligation, and the activation BEI row all
   reconciled to design-RESOLVED (rev 11) + machinery/fixtures landed
   at the solver/identity tier (rev 13) + runtime wiring outstanding.
3. **`latqcc` unit governance:** corrected to mm (the SC-SUBHYD-001
   publication unit; the earlier `m` was wrong) with the
   `mm/1000 × A_outlet` closure-operand conversion RECORDED so the
   activation wiring cannot double-convert the bypass term; the
   `seam.rs` closure-operand doc carries the same note.
4. **Stale rev-4 citations in `seam.rs`:** updated to the corrected
   numbering.
