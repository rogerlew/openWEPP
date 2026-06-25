# Worker Handoff

Current state: SNOWDENSITY-01 is complete.

Keep:

- `artifacts/snotel-density-delta-ledger.json`
- `artifacts/snotel-density-delta-ledger.md`
- `artifacts/evidence-reconciliation.md`
- `artifacts/rubric-cell-classification.md`
- `artifacts/snowd-shen-archaeology.md`

Do not:

- Tune `ssd`.
- Promote PySnobal to runtime dependency or correctness authority.
- Resume frost heat-flow, frozen-K/SFCC, impedance, or `Qwet` work before snow
  insulation is passable or bounded.
- Edit production snow physics from SNOWDENSITY-01.

Next recommended package:

`SNOWDENSITY-02 Contract + ADR`

Objective:

- Amend `SC-SNOWFREEZE-001` with the opt-in `physics_bulk` snow-model envelope,
  no-site-tuning rule, state variables, conservation obligations, and candidate
  equation authority.
- Draft the deliberate-legacy-divergence ADR for `snow_model =
  legacy_wepp | physics_bulk`.
- Add red contract tests only. Do not implement production runtime physics in
  SNOWDENSITY-02 unless the package is explicitly expanded before execution.

First actionable item:

- Scaffold `docs/work-packages/20260625-snowdensity-02-contract-adr-001/` with
  contract-first authority, then author the `SC-SNOWFREEZE-001` amendment and ADR
  before any code work.
