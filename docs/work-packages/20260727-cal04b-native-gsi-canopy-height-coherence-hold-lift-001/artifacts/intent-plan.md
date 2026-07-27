# Intent Plan

Status: `ACCEPTED / A0 ADMITTED`

Evidence class: `Static`

Authenticated base: `f4b3db6c17f25d9dfe969825c672309443963ac4`.

Intent classification: `implementation` and production-defect closure. This
package does not authorize empirical calibration, calibration-domain changes,
population execution, Harvard access, or result publication.

## Defect And Risk

`CAL04B-NATIVE-001` is a critical production state-publication defect. The
native GSI override publishes same-day foliar biomass, LAI, and cover but
retains canopy height from the preceding PL16 projection. A valid
zero-to-positive GSI transition therefore reaches the unchanged rev-21/rev-36
guard with positive LAI and zero height.

The correction changes a kernel-facing state consumed by snow, ET, WB15,
erosion, residue, frost, and Lane D routing. It therefore requires immediate
campaign-closure-strength workspace correctness after focused proof.

## Authority Decision Before Tests Or Production

The A0 decision must establish an explicit native current-day `Hc/canhgt`
equation, operand basis, units, parameter provenance, domain, ordering, and
guard behavior in `SC-PLANT-001`. Static inspection establishes that:

- legacy PL16 Equation 8.2.6 uses total above-ground dry biomass `vdmt`;
- CP-GSI02 defines `Bf` as foliar biomass and `Bs` as separate structural
  biomass;
- legacy rangeland/forest height is geometry-based rather than derived from
  foliar mass; and
- the current CP-GSI02 algorithm publishes only `Bf/LAI/Cc`.

Accordingly, directly substituting `Bf` for legacy `vdmt`, or inventing a
cover-derived height law, is prohibited. Revision 24 independently admitted
checked `Bt=Bs+Bf` as the native realization of total above-ground biomass and
retained `Bf` as the separate foliar/interception handoff. Production remains
blocked only until the contract-derived tests and pre-implementation contract
gate complete.

## Prospective Write Set

- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/specifications/science-contracts/index.md` only for lifecycle metadata
- `tools/check_sc_binding_exposure.py` for the discovered three-segment
  obligation-ID recognition defect exposed by the new Binding Exposure Index
- `tests/python/test_check_sc_binding_exposure.py` for checker anti-self-
  authentication regressions
- runner direct-publication code and owning tests named by `package.md`
- orchestrator growth/Lane D code only for a centralized projection or
  contract-derived tests; the positive-LAI/positive-height guard is protected
- orchestrator erosion daily-state assembly and owning frame fields only to
  replace the discovered optional-PMET zero fallback with the same post-growth
  height and expose the exact value consumed by active erosion
- orchestrator executor trace assignment only to record the exact dynamic
  canopy height consumed by active frost
- this package's artifacts, prompt custody, CAL-04B handoff/status, catalog,
  and canopy assurance roadmap
- the frozen CAL-04B native-proof verifier and one owning regression test only
  to recognize the canonical typed threshold-order parser wording; no case
  plan, input, semantic expectation, or production behavior may change

Any other production or authority surface requires a prospective package
amendment and renewed intent review.

## Gate DAG

1. A0 authority admission and two independent contract reviews.
2. Contract-derived transition/invalid-state tests.
3. Pre-implementation contract gate covering the amended invariant, aliases,
   operand domains, and unchanged fail-closed guard.
4. Central production projection plus focused runner/orchestrator tests.
5. Real-consumer proof for Lane D, snow, ET, WB15, erosion, residue, and frost.
6. Frozen CAL-04B native-proof replay: default, `GSI-5557`, `GSI-0001`,
   lowest saturated, all-operands, six one-at-a-time perturbations, and invalid
   vector.
7. Pre-heavy closure audit against the exact admitted diff.
8. `cargo deny check`, full-workspace campaign-strength correctness, applicable
   A1/A3 authority suites, documentation validation, diff hygiene, and
   line-count governance.
9. Dual review, finding disposition, dual independent verification, prompt
   archival, hold lift, and exact-diff reconciliation.

No current-scope gate is deferred. Coverage/CRAP follows ADR-0041 as
`DEFERRED_TO_QUALITY_CI`; it is not represented as receipt evidence.

## Expected Production Seam

If and only if A0 admits a native height law, compute height from the admitted
same-day native operands in the centralized post-phenology projection and
publish it in the same `DirectGrowthStateSurface` as native biomass, LAI, and
cover. Preserve typed validation and the existing downstream coherence guard.
