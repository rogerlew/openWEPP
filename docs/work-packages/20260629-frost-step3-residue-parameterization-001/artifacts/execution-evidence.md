# Execution Evidence

Evidence mode: Ran.

Command:

```bash
.venv/bin/python docs/work-packages/20260629-frost-step3-residue-parameterization-001/artifacts/run_residue_parameterization.py --binary target/release/openwepp-cli-hill
```

Result: exit code `0`.

Entry-gate run:

- Fixture: `tests/fixtures/cancov_forest/hubbardbrook_deciduous_nh`
- Run stem: `p10`
- Seasonal management: `Dec_4899`
- Runtime: `openwepp-cli-hill --direct-production-executor`
- Trace env: `OPENWEPP_R7G_FROST_TRACE_PATH`
- Generated output root:
  `target/frost_step3_residue_parameterization/runs/entry_gate_hubbardbrook_deciduous/`

Trace result:

- Solver trace rows: `32874`
- `residue_depth_m` min/max:
  `0.02302585092994045` / `0.02302585092994045`
- Rounded unique residue-depth count at `1e-6 m`: `1`
- Autumn mean: `0.02302585092994045 m`
- Spring mean: `0.02302585092994045 m`
- Monthly trajectory artifact:
  `docs/work-packages/20260629-frost-step3-residue-parameterization-001/artifacts/entry_gate_residue_monthly_trajectory.csv`
- JSON summary artifact:
  `docs/work-packages/20260629-frost-step3-residue-parameterization-001/artifacts/entry_gate_residue_trace_summary.json`

Decision:

- Branch: `C`
- Reason: `Dec_*` did not drive a seasonal `residue_depth_m` trajectory to the
  frost solver.
- Core Sleepers A-versus-B re-score: not run, per the package entry gate.

No production fixtures, defaults, schema, selectors, runtime code, snow code, or
frost-model code were changed.
