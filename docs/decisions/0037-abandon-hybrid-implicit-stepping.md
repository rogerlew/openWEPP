# ADR-0037: Abandon Hybrid Implicit-Explicit Kinematic-Wave Stepping

Status: **Accepted** (ratified 2026-07-07 by operator direction — Roger Lew:
"agreed strip, keep ADR and work-packages as part of the historical record.
get rid of code and contract"; abandonment direction stated same-session after
the H2637 evidence-base reassessment. Authored 2026-07-07 by Claude Code on
operator direction.)

Deciders: Roger Lew, Claude Code

Provenance:
[`docs/work-packages/20260707-laned-router-d16-hybrid-viability-adjudication-001/`](../work-packages/20260707-laned-router-d16-hybrid-viability-adjudication-001/)
(EXECUTED-HOLD-HYBRID-VIABILITY + the predeclared kill criteria in
`artifacts/review-claude.md` CL-M3);
[`docs/work-packages/20260707-laned-router-d16-hybrid-noharm-selector-solvecost-hold-lift-001/`](../work-packages/20260707-laned-router-d16-hybrid-noharm-selector-solvecost-hold-lift-001/)
(the final executed hybrid state: rev-5 no-harm selector).

Relates: ADR-0011 (contract authority), ADR-0017 (comparator distrust),
ADR-0031 (precedent: terminal deletion of a subsystem with the record
retained), ADR-0033 (the OFE-by-OFE routing lane the hybrid rode),
`SC-OFEROUTE-001` (retains all plain-path routing authority),
`SC-OFEROUTE-002` (the abandoned subsystem's contract, removed from main by
this decision).

## Context

The hybrid implicit-explicit stepper (LANED-T3 arc, `SC-OFEROUTE-001` revs
28-31, consolidated as `SC-OFEROUTE-002` revs 1-5) replaced explicit
kinematic-wave sub-stepping with backward-Euler implicit bins during
source-quiet spans of active Lane D routing. Over 2026-07-06/07 the arc
resolved every named defect it encountered: exact cross-span deficit carry
(rev 30), deterministic warm seeding (rev 31), the Case-4 shock-cooldown
switching predicate (rev 3, `GAP-OFEHYB-001`), an exact bare-skin branch
evaluator eliminating H2637's implicit map-evaluation cost (rev 4,
`GAP-OFEHYB-002`), and a deterministic no-harm request/fallback selector
(rev 5, `GAP-OFEHYB-003`). Final state: 1442/1442 workspace tests, Case-4
hybrid ladder passing unignored, selected-cohort timing no-harm lifted.

The evidence base then failed, not the engineering:

- **The only demonstrated win is synthetic.** H2637 (19 uniform 26.11 m
  OFEs — a topology legacy WEPP's ~10-OFE ceiling could not even run) is a
  constructed stress case, and it is the sole member where the hybrid wins
  (`40.05 s` plain vs `33.62 s`, all lane-days bare-skin eligible). The
  operator adjudicated it heavily discounted as promotion evidence.
- **Every real member declines or loses.** All three real selected-cohort
  members (`mn_corn_h4`, `n_idaho_forest_h1`, `wa_cascades_forest_h1`) are
  non-bare: under the rev-5 selector they fall back to plain on 7299/7299
  requested lane-days (hash-identical outputs, zero value); without the
  selector they regress (WA Cascades `+56.5 %` paying `98.2 M` generic map
  evaluations). The forest fleet — this program's priority class — is
  vegetated, i.e. structurally non-bare.
- **The narrowed scope has no evidence either.** A bare-skin-only hybrid
  (post-fire/disturbed classes) was the recorded retreat, but the cohort
  contains no real burned member; that value is hypothesis-only.
- **Mesh policy dominates the residual value.** The 10-cells/OFE working
  resolution gives H2637 `Δx ≈ 2.6 m` while a real single-OFE 300 m member
  runs `Δx = 30 m`; a future Δx-rational mesh policy collapses H2637's cell
  count and with it the hybrid's one win, while any refinement need lands on
  non-bare members where the hybrid is blocked by generic solve cost.

Against the predeclared kill criteria (viability-adjudication
`review-claude.md` CL-M3): none of the four falsifiers technically fired —
F1 (fidelity-savings death curve) and F2 (non-bare algebraic wall) were
never tested, F3 (Tier-2 supersession) is not yet ratified, and F4
(time-box) was *satisfied* when the no-harm gate flipped in one package.
The abandonment ground is an **evidence-base discount** outside that list:
the demonstrated-win evidence does not generalize to any real member, so
the option's expected value is approximately zero against a permanent
carrying cost (a second stepper path through the router, a full contract, a
hybrid test surface in every workspace gate, and hybrid-invariant
preservation obligations on every future routing change). The criteria were
sufficient conditions, not necessary ones; this record states the actual
ground rather than retrofitting a falsifier.

## Decision

1. **Abandon the hybrid stepper subsystem.** No further design,
   optimization, promotion, or tolerance work.
2. **Archive, then strip.** The final working state (rev-5 no-harm
   selector, all gates green) is preserved on the branch
   `abandoned/hybrid-implicit-stepping`, cut from the commit containing the
   executed no-harm selector package before any removal edit. Main then
   removes the hybrid **code and contract** entirely — no deprecated code
   quarantine, no dormant selector.
3. **The historical record stays on main.** All hybrid work-package
   directories, the `SC-OFEROUTE-001` revision-history entries (revs 28-31
   and the hybrid-era amendments), the work-packages README execution log,
   and this ADR remain. The record of the work is not carry; the code and
   its normative contract are.
4. **Tests are retired with the code** (the viability review's CL-M3
   test-retirement obligation): the Case-4 *hybrid* ladder, cooldown,
   deficit-carry, warm-seed, bare-skin-evaluator, selector, and counter
   tests are deleted in the same package. The plain Case-4 oracle ladder
   (`INV-OFEROUTE-011`) is Lane D's acceptance surface and is untouched.
5. **Durable knowledge is extracted before deletion:**
   - the **Z-shaped equilibrium-rating discovery** (bistable rating from
     the `INV-OFEROUTE-002` regime dispatch at `Q_c = 1000 nu`; a hazard
     for any future equilibrium solve against the rating) moves to a
     `docs/numerics/` note;
   - the **selector-determinism input-class principle** (runtime policy
     selectors must be pure functions of run inputs; wall-clock, host
     load, and measured runtime counters are prohibited inputs — rev-5's
     `INV-OFEHYB-011`) moves to the `docs/numerics/` determinism policy.
6. **Removal is executed by
   `docs/work-packages/20260707-laned-router-hybrid-abandonment-removal-001/`**
   with the plain-path byte-identity gate as the acceptance criterion:
   `INV-OFEHYB-007` guaranteed hybrid-off byte identity throughout the
   subsystem's life, so active-plain outputs for H2637 and the three real
   cohort members must be hash-identical before and after the strip.

## What is kept (landed during the arc, not hybrid-specific)

- The `ow-lanuse-1` native landuse management consumer path (parsing,
  projection, route-coefficient consumption, fail-closed posture); the
  WEPPpy-side Disturbed native production is a separate repository and is
  unaffected.
- The daily `canhgt`/`Hc` publication and the Lane D friction-operand
  re-point (`SC-PLANT-001` rev 19, `SC-OFEROUTE-001` rev 36) — a real
  active-runtime defect fix independent of the hybrid.
- The entire rev-27 plain active Lane D path and the plain Case-4 oracle
  acceptance surface.
- The selected-cohort materialization and active-suite harness (reusable
  for plain validation, mesh adjudication, and tolerance ratification).
- Explicit-path profile counters and the build-provenance (QA-M3)
  workflow guidance.
- The extracted numerics knowledge (Decision item 5).

## Consequences

- The router has exactly one stepper again; future routing changes carry
  no hybrid-invariant preservation obligations, and the workspace test
  gate stops paying the hybrid test tax.
- `OPENWEPP_LANED_ACTIVE_IMPLICIT` ceases to exist as a selector. The
  removal package decides and records the posture for a set-but-unknown
  request (recommended: typed startup rejection naming this ADR, matching
  the repository's fail-closed norm) rather than silently ignoring it.
- H2637 is demoted from "the hard case" to a synthetic stress case: it
  remains useful for closure/soak coverage, but future performance or
  promotion claims must not rest on H2637-only evidence.
- The Tier-2 mesh-resolution question survives independently and will be
  re-scoped (a Δx-target policy adjudicated on Δx-anchored oracle rungs
  and fine-mesh self-convergence, not cells/OFE counts ratified on
  short-OFE configurations); a fleet-topology survey is explicitly NOT the
  instrument, because existing fleet inventories inherit legacy's ~10-OFE
  ceiling.
- Revival, if ever justified by new evidence (e.g. real bare/low-cover
  members demonstrating a material win), starts from the archive branch
  and re-enters through a new contract — not by reverting this ADR
  in-place.
