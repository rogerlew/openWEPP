# WA Rerun Evidence

Status: EXECUTED
Evidence mode: Ran.

Command:

```bash
.venv/bin/python docs/work-packages/20260708-laned-router-wa-active-router-positivity-preserving-solver-correction-001/artifacts/run_mesh_ladder.py --members wa_cascades_forest_h1 --rungs baseline_fixed10 dx5
```

Release binary:

- Build command: `cargo build --release -p openwepp-runner --bins`
- SHA256: `8427529a166a880699fd06a6a39ed6f6bb23ca039a62dc670cd784ebce11e6f6`
- Git HEAD: `3b01de4e6e957c4a0905d86d0267dac5a5b73524`

| Rung | Status | User s | Wall | Solver steps | Total clamp m3 | Cascade rel | Seam rel | Identity rel |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| `baseline_fixed10` | PASS | 15.93 | 0:16.01 | 3284995 | 2.717124262301002e-13 | 1.532462467225031e-14 | 3.0467009154883755e-14 | 5.933273356520313e-14 |
| `dx5` | PASS | 62.77 | 1:02.82 | 4891877 | 7.305156020320419e-13 | 4.705058001136025e-14 | 4.832475752036399e-14 | 5.933273356520313e-14 |

Both runs produced manifests, pass parquet outputs, HBP outputs, and
`laned_active_trace.jsonl` with `10960` rows.

## Result

The rev-40 `laned_active_clamp_exceeds_source` guard no longer trips for WA
fixed10 or `dx5`. The retained publication guard is still present; these runs
pass because the solver no longer creates material clamp mass.
