# MAGPARITY01 Disposition

Status: **CLOSED 2026-06-18 — no defect; Stage-2 lateral/subsurface magnitude
flag**

Evidence mode: **Ran** (H2637 parquet/text parsing, manifest checks) +
**Static** (contracts, ADR-0017, external-authority references).

## Outcome

MAGPARITY01 adjudicated the H2637 71.0% openWEPP outlet `runvol` magnitude
without making code or contract changes. The suspected `INV-RUNOFFPART-028`
inter-OFE carry path passed: adjacent surface carry, adjacent lateral carry,
area scaling, Q/QOFE duality, PASS export reconstruction, and manifest closure
all hold at numerical noise.

The outlet `runvol` decomposes to `97,987 m³` local surface residual plus
`13,987,683 m³` routed lateral flow from OFE1-OFE18. OFE19 lateral flow exits as
separate PASS `sbrunv = 884,950 m³`. The remaining bounded delta against legacy
`without_ui` is a Stage-2 lateral/subsurface process-magnitude flag, not an
openWEPP defect from this package.

## Gates

| Gate | Result |
|---|---|
| Per-OFE decomposition from closed H2637 run | PASS |
| `INV-RUNOFFPART-028` adjacent transfer and closure check | PASS |
| Area scaling / QOFE-Q duality check | PASS |
| Export reconstruction from PASS/WAT operands | PASS |
| External-authority plausibility check | PASS, no hard coefficient bound found |
| Per-term verdict | PASS |
| Handoff | PASS, Stage-2 lateral/subsurface magnitude follow-on |
| Markdown lint | PASS |
| Rust gates | Not run; no Rust files touched |

## Gate Results

Ran:

```bash
markdown-doc lint --path docs/work-packages/20260618-mofe-magparity01-per-ofe-runoff-magnitude-adjudication-001 --path docs/work-packages/README.md --path docs/ROADMAP.md
```

Result: `12 files validated, 0 errors, 0 warnings`.

## Files

- `artifacts/magparity01-runoff-decomposition.md`
- `artifacts/magparity01-inv028-closure-check.md`
- `artifacts/magparity01-external-authority-plausibility.md`
- `artifacts/magparity01-per-term-verdict.md`
- `artifacts/magparity01-handoff.md`
- `artifacts/magparity01_disposition.md`
