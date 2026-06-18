# PERFARRAY01 Disposition

Evidence class: Static + Ran.

## Verdict

NO-GO for PERFARRAY01 as scoped.

Stage A landed. Stage B did not run because the existing production
request/scheduler seam would make any WB11 "array pilot" violate PERFARRAY01's
two structural proofs.

## What Landed

Stage A landed a default-unwired array contract shell:

- `ArrayHotState`;
- `ArrayWritebackField`;
- `ArrayWritebackPayload`;
- `evaluate_array_writeback`;
- `apply_array_writeback`;
- logical materialization from dense slots.

Focused gates for `openwepp-kernel-contract` passed.

## Why Stage B Is NO-GO

The current scheduler constructs `HillslopeKernelRequest` from logical
`BTreeMap` state/flux maps, validates consumer boundaries against logical maps,
applies writeback into logical maps, and only then synchronizes the indexed
mirror. The WB11 accessors and runoff reconciliation anchor still require
logical map reads for core scalar values.

Therefore a Stage B pilot built now would have to choose one invalid path:

- export dense state to logical maps before kernel execution, violating the
  no per-day export proof; or
- keep logical maps as authority and mirror writes into arrays, violating the
  no dual-write proof.

No H2637 floor measurement was produced. ADR-0023 is not ready for ratification.

## Target Judgment

The <=10x and 5x questions remain open. PERFARRAY01 did not measure the real
integrated floor. PERFIDX06 remains the current ratio evidence at 73.12x
legacy no-UI.

## Recommendation

Do not start broad Stages C-F migration. Scaffold a narrower
`PERFARRAY02` request/accessor authority split for WB11 runoff reconciliation.
That package should make a real array-authoritative request path possible, then
rerun the integrated floor measurement.
