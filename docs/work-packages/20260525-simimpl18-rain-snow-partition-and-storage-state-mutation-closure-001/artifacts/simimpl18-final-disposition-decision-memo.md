# simimpl18-final-disposition-decision-memo

Status: complete
Evidence mode: static+ran
Date: 2026-05-25
Decision: HOLD

## Static
- SIMIMPL18 delivered contract-first authority updates and replay-tooling
  baseline-year policy closure.
- Production runner physics closure for day-1 partition/storage publication was
  not completed under baseline-authoritative migration constraints.

## Ran
- Tier-A rerun evidence bundle:
  - `artifacts/replay-run-20260525T132822Z/`
- Key rerun outcomes:
  - full-span key join achieved (`common_row_count=1095`, policy-materialized).
  - day-1 and multi-day hydrology/state parity remains failing.
- Gate outcomes:
  - `cargo test --workspace` fails on SIMIMPL18 contract assertions.

## Rationale
- The package resolved comparator-span governance mechanics but did not close
  the core process-physics residuals it was intended to retire.
- Under repository governance, unresolved invariant/contract failures keep
  disposition in `HOLD`.

## Required follow-on closure focus
- Migrate baseline-authoritative process physics for affected surfaces
  (snow/winter/state publication and linked ET/storage behavior) from
  `/workdir/wepp-forest_260430_baseline` with explicit provenance mapping.
- Re-run SIMIMPL18 contract suite and Tier-A lanes after migration.
