# HPHYS0235 Contract Implementation Evidence

Status: completed  
Evidence mode: Static

## Summary

Canonical hourly-lane authority was reanchored from divisor-only WB18
attenuation to baseline-authoritative iterative substep semantics.

## Contract Amendments

1. `SC-PERC-001` updated to `contract_version: 19` with:
   - algorithm requirement for hourly `24`-substep recompute loop,
   - strengthened `INV-PERC-010` lane-semantics wording,
   - lane-semantics vector obligations including hourly iterative behavior,
   - explicit HPHYS0235 addendum prohibiting divisor-only single-pass hourly
     closure claims.  
   Reference: `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
   (lines with `contract_version: 19`, `HPHYS0235`, `INV-PERC-010`).

2. `SC-WATBAL-001` updated to `contract_version: 66` with:
   - HPHYS0235 addendum tying `ui_run=1` to legacy `watbal_hourly` iterative
     shape (`ui_LFtstp=24`) and accumulated hourly seepage lineage for `Dp`,
   - explicit prohibition of divisor-only single-pass hourly treatment for
     closure.  
   Reference: `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
   (lines with `contract_version: 66` and `HPHYS0235` addendum).

## Contract-First Sequencing Check

- Contract updates completed before any production-kernel edits.
- This package intentionally lands no production-kernel behavior changes.
