# Contract Implementation Evidence

Status: complete
Evidence mode: Static

## Static: Canonical Contract Amendments

- `SC-SNOWFREEZE-001` moved to `contract_version: 19` and added `INV-SNOWFREEZE-019` for corrected negative-melt carry-state lineage.
- `SC-WATBAL-001` moved to `contract_version: 103` and added `INV-WATBAL-059` for the WB13 `RM`/`S` routed-melt versus `Snow-Water` carry-state split.
- `SC-WATBAL-001` guard map now includes `INV-WATBAL-058` and `INV-WATBAL-059` HPHYS gates.

## Static: Baseline Provenance

- Corrected target authority: `/workdir/wepp-forest/src/winter.for` lines 441-460 at commit `03fee4558456535138592630b5dedc4d81ce8d06`.
- Pinned hourly snow/melt authority remains `/workdir/wepp-forest_260430_baseline/src/snowd.for` and `/workdir/wepp-forest_260430_baseline/src/melt.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

## Disposition

- Contract-first authority is implemented.
- The amendment is not the rejected pinned-baseline sign/scale bug; it completes the corrected `/workdir/wepp-forest` state-lineage port by preserving the companion carried snow-depth/SWE adjustment.
