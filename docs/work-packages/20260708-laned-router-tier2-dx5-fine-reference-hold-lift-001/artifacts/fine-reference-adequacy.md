# Fine-Reference Adequacy

Status: `FAIL-MN-CORN-H4-SHAPE`
Evidence mode: Ran.

Command:

```text
.venv/bin/python docs/work-packages/20260708-laned-router-tier2-dx5-fine-reference-hold-lift-001/artifacts/run_mesh_ladder.py --members mn_corn_h4 --rungs baseline_fixed10 dx20 dx10 dx5 dx2p5 dx1p25 dx0p625
```

Release binary:

- Build command: `cargo build --release -p openwepp-runner --bins`
- Path: `target/release/openwepp-cli-hill`
- SHA256: `8427529a166a880699fd06a6a39ed6f6bb23ca039a62dc670cd784ebce11e6f6`
- Git HEAD at execution: `25a9f52d2b6dba7d18188d2e0d0523c4f0d7f6a1`

## Strict Gate

The predeclared one-third adequacy rule is unchanged. The routed hourly shape
threshold is `0.05 / 3 = 0.0166667`.

| Comparison | Outlet L1 rel | Shape max L1 | Shape gate | End storage rel | Tail fold rel | Annual sed rel | Uniform row increase | Degenerate row increase | Verdict |
|---|---:|---:|---|---:|---:|---:|---:|---:|---|
| `dx2p5` vs `dx1p25` | `5.853855e-05` | `0.020180511` | FAIL | `5.555761e-05` | `8.342675e-06` | `0` | `0` | `0` | FAIL |
| `dx1p25` vs `dx0p625` | `2.806216e-05` | `0.020944940` | FAIL | `1.496140e-05` | `3.486899e-06` | `0` | `0` | `0` | FAIL |

The finer reference did not close the strict shape adequacy miss. The maximum
shape delta increased slightly from `0.020180511` to `0.020944940`, still with
zero shape rows above the production candidate tolerance `0.05`.

## Shape Attribution

The named discrete counters do not show a cliff:

| Counter | `dx1p25` | `dx0p625` | Delta |
|---|---:|---:|---:|
| `source_total_m3` | `4510.466965990125` | `4510.466965990125` | `0` |
| `terminal_outlet_total_m3` | `4473.220365201318` | `4473.152882289323` | `0.067482911994` |
| `end_storage_total_m3` | `37.246600788810` | `37.314083700790` | `-0.067482911980` |
| `tail_fold_total_m3` | `9.371413833285` | `9.355686291575` | `0.015727541710` |
| `uniform_shape_rows` | `16` | `16` | `0` |
| `erosion_source_shape_degenerate_rows` | `22` | `22` | `0` |
| `positive_shape_rows` | `83` | `83` | `0` |

Only one positive-source lane-day exceeds the one-third adequacy shape
threshold:

- `sim_day_index = 792`
- `lane_index = 1`
- shape L1 delta = `0.02094494047849004`
- `source_m3 = 1.3852874865748004` on both rungs
- `tail_fold_m3 = 0` on both rungs
- `mesh_end_storage_m3`: `0.8359925188011007` (`dx1p25`) vs
  `0.8360829335598599` (`dx0p625`)
- `uniform_shape = false` and `erosion_source_shape_degenerate = false` on
  both rungs

The max row is therefore a smooth routed-hourly-weight redistribution, not a
uniform-shape or degenerate-shape class flip and not a tail-fold cliff.

## Verdict

`mn_corn_h4` shape adequacy remains open. Per operator instruction, this is no
longer treated as a reference-depth problem. Production mesh-policy promotion
stops in this package.
