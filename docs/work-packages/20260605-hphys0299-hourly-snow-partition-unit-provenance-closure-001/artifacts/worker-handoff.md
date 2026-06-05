# Worker Handoff

Status: complete

Evidence mode: static

## Handoff

HPHYS0299 is complete and remains in `executed-hold` because semantic parity is
not closed and all nine target windows remain `OPENWEPP-DEFECTIVE` under the
corrected ledger.

## Key Artifacts

- Corrected partition ledger:
  `docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/artifacts/corrected-partition-ledger.md`
- Corrected ledger JSON:
  `docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/artifacts/corrected-partition-ledger.json`
- Full 39 metrics:
  `docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/artifacts/full-39-suite-metrics.md`
- Archived full-suite JSON:
  `docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/artifacts/full-39-suite-summary.json`
- Baseline observe status:
  `docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/artifacts/baseline-observe-status.tsv`
- Target trace status:
  `docs/work-packages/20260605-hphys0299-hourly-snow-partition-unit-provenance-closure-001/artifacts/target-trace-status.tsv`

## Required Continuation

Scaffold the next package around raw hourly melt and post-raw routed-melt
lineage. Use corrected HPHYS0299 routing as authority:

- `raw-hourly-melt`: 7 windows.
- `negative-melt-correction`: 1 H7 first-2013 row, but with
  `OPENWEPP-DEFECTIVE`, `baseline_negative_raw_melt_sum_mm = 0.0`, and a
  follow-on post-raw routed-melt/negative-melt route.
- `hourly-forcing`: 1 H39 first-2013 row, still a corrected depth-vs-depth
  forcing divergence.

## Constraints

- Use `/workdir/wepp-forest_260430_baseline` as normative baseline authority.
- Do not restore HPHYS0298 water-equivalent mapping.
- Do not reproduce pinned-baseline negative-melt bugs.
- Do not compensate through WB17/WB18/WB19/WB13.
- Keep contract-first sequencing and dual review/disposition/verification.
