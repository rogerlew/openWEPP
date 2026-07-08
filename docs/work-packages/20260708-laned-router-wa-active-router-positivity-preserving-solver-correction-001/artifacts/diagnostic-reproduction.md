# Diagnostic Reproduction

Status: EXECUTED
Evidence mode: Ran.

## Pre-Correction Current-Tree Rerun

Command:

```bash
.venv/bin/python docs/work-packages/20260708-laned-router-wa-active-router-positivity-preserving-solver-correction-001/artifacts/run_mesh_ladder.py --members wa_cascades_forest_h1 --rungs baseline_fixed10 dx5 --expect-fail-guard laned_active_clamp_exceeds_source
```

Release binary:

- Build command: `cargo build --release -p openwepp-runner --bins`
- SHA256: `2331d10073cc4c0428d12b8a717d6e934e5eff14ba5fff07e56daa4a2b236579`
- Git HEAD: `3b01de4e6e957c4a0905d86d0267dac5a5b73524`

Results:

| Rung | Status | Failure phase | Day | Clamp m3 | Source cap m3 | Clamp/source |
|---|---|---|---:|---:|---:|---:|
| `baseline_fixed10` | FAIL | `laned_active_clamp_exceeds_source` | 1418 | 145981.7238747406 | 10214.84019227633 | 14.291141234409194 |
| `dx5` | FAIL | `laned_active_clamp_exceeds_source` | 1167 | 95922302.77342197 | 8461.820908299935 | 11335.893753002358 |

The summary was preserved as:

- `artifacts/pre-correction-mesh-ladder-summary.json`
- `artifacts/pre-correction-mesh-ladder-summary.md`

## Interpretation

This reproduces the predecessor hold exactly: active routing fails before row
publication/commit because booked positivity-clamp mass exceeds external routed
source mass. The failure is the rev-40 protected publication guard, not a new
source-producer or output-writer failure.
