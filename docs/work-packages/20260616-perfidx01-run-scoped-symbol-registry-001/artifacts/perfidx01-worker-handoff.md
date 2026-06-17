# PERFIDX01 Worker Handoff

Status: HANDOFF READY 2026-06-16
Evidence mode: **Static** + **Ran**

## Summary

PERFIDX01 completed ADR-0022 Stage 1. The run-scoped symbol registry and
validation hook exist, the runtime universe was proven enumerable on the target
cohort, and the BTreeMap runtime surface remains authoritative.

## Important Paths

- Audit reports: `/tmp/perfidx01/audit/*.json`
- Current audit manifests: `/tmp/perfidx01/outputs/*/openwepp_hillslope_run_manifest.json`
- Science outputs compared to anchor:
  - `/tmp/perfho01/outputs/ofe1` through `/tmp/perfho01/outputs/ofe5`
  - `/tmp/perfho01/outputs/h2637`
  - `/tmp/perfopt01/outputs/h2637_with_ui`
- Anchor outputs: `/tmp/perfopt01/after`
- Determinism outputs: `/tmp/perfidx01/determinism/ofe5_run1` and
  `/tmp/perfidx01/determinism/ofe5_run2`

## Next Recommended Work

Open Stage 2 as:

```text
PERFIDX02-indexed-shadow-runtime-surface-001
```

Recommended Stage 2 scope:

- Add indexed shadow storage beside the authoritative BTreeMap runtime surface.
- Keep BTreeMap authoritative for outputs and writeback acceptance.
- Validate id-ordered shadow export against BTreeMap ordering on the same
  H2637 plus OFE1-5 cohort.
- Reuse `OPENWEPP_SYMBOL_REGISTRY_AUDIT_PATH` or add a sibling env-gated shadow
  equality report.
- Preserve `anchor_mismatches = 0` and same-config determinism before any
  authority flip.

## Caveat

Independent dual review was not delegated in this run. Local review and
verification artifacts are included with explicit limitations.

