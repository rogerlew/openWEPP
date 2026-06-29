# Verification

Evidence mode: Ran.

Commands:

```bash
.venv/bin/python -m py_compile docs/work-packages/20260629-frost-step3-residue-parameterization-001/artifacts/run_residue_parameterization.py
.venv/bin/python docs/work-packages/20260629-frost-step3-residue-parameterization-001/artifacts/run_residue_parameterization.py --binary target/release/openwepp-cli-hill
```

Results:

- Python compile: pass.
- Entry-gate run and diagnostic artifact generation: pass.
- Decision branch: `C`.
- Core Sleepers A-versus-B re-score: intentionally not run because the entry
  gate failed.

Package gate table:

| Gate | Status | Evidence |
| --- | --- | --- |
| Entry-gate residue trace captured | PASS | `entry_gate_residue_trace_summary.json` |
| `Dec_*` drives seasonal residue to solver | FAIL | Flat `0.02302585092994045 m` in all rows |
| Core Sleepers A-vs-B comparison | NOT RUN | Correctly blocked by failed entry gate |
| Branch A/B/C routing | PASS | Branch `C` |
| Diagnostic-only boundary | PASS | Docs/package artifacts only |
| `GAP-SNOWFREEZE-002` disposition updated | PASS | `gap-disposition.md` |

Doc validation:

```bash
markdown-doc lint --path docs/work-packages/20260629-frost-step3-residue-parameterization-001 --no-ignore
markdown-doc validate --path docs/work-packages/20260629-frost-step3-residue-parameterization-001 --no-ignore
markdown-doc lint --path docs/work-packages/README.md --path docs/planning/snow-frost-fidelity-strategy.md --path docs/backlog/20260626-frost-daylength-canopy-decline-hemisphere-robust.md --no-ignore
markdown-doc validate --path docs/work-packages/README.md --path docs/planning/snow-frost-fidelity-strategy.md --path docs/backlog/20260626-frost-daylength-canopy-decline-hemisphere-robust.md --no-ignore
```

Results:

- Package lint/validate: `9` Markdown files, `0` errors, `0` warnings.
- Updated catalog/planning/backlog lint/validate: `3` Markdown files,
  `0` errors, `0` warnings.
