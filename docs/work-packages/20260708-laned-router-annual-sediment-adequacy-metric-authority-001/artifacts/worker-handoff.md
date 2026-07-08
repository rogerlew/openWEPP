# Worker Handoff

Status: `READY`
Evidence mode: Static + Ran.

## Next Package

Suggested name:

`20260708-laned-router-dx5-production-mesh-policy-ratification-001`

## Objective

Ratify and, if all gates pass, implement the `dx5` active production
mesh-policy default on the `SC-OFEROUTE-001` rev-44 metric basis.

## Required Inputs

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` rev 44
- This package:
  - `artifacts/metric-authority-decision.md`
  - `artifacts/annual-sediment-metric-replay.md`
  - `artifacts/annual-sediment-metric-replay.json`
  - `artifacts/contract-disposition.md`
- `docs/work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001/artifacts/coupled-spacetime-summary.json`
- `docs/work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001/artifacts/mesh-policy-ratification.md`
- `docs/work-packages/20260708-laned-router-active-router-timestep-policy-adjudication-001/artifacts/`
- `docs/work-packages/20260708-laned-router-wa-sediment-reference-adequacy-attribution-001/artifacts/classification.md`

## First Actions

1. Reconstruct the selected-cohort promotion matrix on the rev-44 basis:
   candidate-vs-reference, adequate fine-reference, and same-`dx`
   timestep-control surfaces.
2. Confirm that annual pass-sediment now passes under rev 44 and that no other
   judged surface remains open.
3. Decide the production policy explicitly:
   - active production mesh default;
   - shadow mesh treatment, either changed under contract or recorded
     out-of-scope;
   - cost recorded under the standing fidelity-first posture, not used as a
     blocker unless the operator changes that posture.
4. If production promotion is supported, amend `SC-OFEROUTE-001` before code
   and implement the default flip with full protected/default/off and active
   closure/consumer evidence.
5. If any surface remains open, hold with the exact blocker and first
   actionable follow-on.

## Binding Rules

- Do not use this package as a production flip; it is metric authority only.
- Do not widen tolerances or alter routed-shape thresholds.
- Do not change sediment process physics.
- Do not silently change the shadow mesh; decide it explicitly.
- Runtime cost is priced and recorded, but fidelity remains the promotion
  priority under the standing operator posture.

## Required Gates For A Flip

- `git diff --check`
- Markdown/doc lint for touched docs
- Contract/profile/BEI checks for touched contracts
- Focused Lane D / `ofe_routing` tests
- Protected-output byte identity with subsystem/default off
- Active closure and `INV-OFEROUTE-012` evidence
- DC01-disable / no-double-feed proof for active lanes
- Routed-hydrograph-to-erosion consumer proof
- Selected-cohort active HBP/pass-parquet evidence with exact release-binary
  provenance
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
