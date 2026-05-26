# SIMIMPL35 Hold-Lift Decision Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-26
Decision: HOLD

## Static
- SIMIMPL35 is the FROSTPLAN01-defined final hold-lift gate after SIMIMPL34.
- Hold-lift requires rerun evidence attributable to post-SIMIMPL34 runtime
  behavior.

## Ran
- Replay bundle: `artifacts/replay-run-20260526T160058Z/`
- Required gates bundle: `artifacts/gates-20260526T160354Z/`

## Decision rationale
- HOLD is required for three independent reasons:
  1. Fresh post-SIMIMPL34 candidate generation failed on shared fixture with
     typed runtime domain violation (`KWRITEBACK-E-DOMAIN-VIOLATION`).
  2. Fresh direct `/wc1` candidate generation failed due parser compatibility
     blocker (`SOL-E-006` for legacy soil variant arity).
  3. Unfiltered `/wc1` comparator lane remains non-admissible due duplicate
     semantic keys in multi-hillslope parquet (`duplicate row key (1,1,1997)`).

- Additional evidence noted:
  - Filtered `wepp_id=5` lanes are semantically admissible and show
    `common_row_count=1095`, but they derive from source parquet timestamped
    before SIMIMPL34 commit time and are therefore insufficient as post-
    migration hold-lift proof.

## Hold-lift prerequisites
- Resolve candidate runtime domain violation path so a fresh post-SIMIMPL34
  run can complete and emit comparator-ready output.
- Resolve direct `/wc1` soil parser compatibility (`SOL-E-006`) for runner
  execution using canonical `/wc1` inputs, or provide authorized equivalent
  post-SIMIMPL34 lane provenance.
- Re-run SIMIMPL35 comparator lanes with fresh post-SIMIMPL34 candidate output
  and publish updated GO/HOLD disposition.
