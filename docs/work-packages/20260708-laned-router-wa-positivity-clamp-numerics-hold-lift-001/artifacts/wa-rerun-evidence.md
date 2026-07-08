# WA Rerun Evidence

Status: EXECUTED
Evidence mode: Ran.

Command:

```text
.venv/bin/python docs/work-packages/20260708-laned-router-wa-positivity-clamp-numerics-hold-lift-001/artifacts/run_mesh_ladder.py \
  --members wa_cascades_forest_h1 --rungs baseline_fixed10 dx5 \
  --expect-fail-guard laned_active_clamp_exceeds_source
```

The command exited `0` with `PASS_EXPECTED_FAIL` because both rungs failed
closed at the expected guard and the harness parsed `clamp/source > 1` for each
run.

## Release Binary

| Surface | Value |
|---|---|
| Build command | `cargo build --release -p openwepp-runner --bins` |
| Binary | `target/release/openwepp-cli-hill` |
| SHA256 | `2331d10073cc4c0428d12b8a717d6e934e5eff14ba5fff07e56daa4a2b236579` |
| Git HEAD | `148583efdef3272b717cf5fecfd0262ef9f2231d` |
| Git status | dirty working tree with this package's code/docs edits |

## Rung Outcomes

| Rung | Status | First active guard | Day | Clamp m3 | Source cap m3 | Clamp/source | Wall | User |
|---|---|---|---:|---:|---:|---:|---:|---:|
| `baseline_fixed10` | FAIL-CLOSED | `laned_active_clamp_exceeds_source` | 1418 | 145981.7238747406 | 10214.84019227633 | 14.291141234409194 | 0:09.67 | 9.65 |
| `dx5` | FAIL-CLOSED | `laned_active_clamp_exceeds_source` | 1167 | 95922302.77342197 | 8461.820908299935 | 11335.893753002358 | 0:34.57 | 34.55 |

Evidence files:

- `artifacts/mesh-ladder-summary.json`
- `artifacts/mesh-ladder-summary.md`
- `artifacts/mesh-ladder-runs/wa_cascades_forest_h1/baseline_fixed10/time.log`
- `artifacts/mesh-ladder-runs/wa_cascades_forest_h1/dx5/time.log`

## Interpretation

The guard catches the retained-default WA event before output publication:
`baseline_fixed10` no longer completes with a material day-1418 positivity
clamp. The finer `dx5` rung fails even earlier, on day 1167, because the clamp
mass is more than four orders of magnitude larger than the active source mass.

This confirms the package closes the silent-publish defect class. It also
confirms WA active routing remains not promotable until a deeper solver
correction makes these days run with bounded physical clamp behavior instead of
typed failure.
