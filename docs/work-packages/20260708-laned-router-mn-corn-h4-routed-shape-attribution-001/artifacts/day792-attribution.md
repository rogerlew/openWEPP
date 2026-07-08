# Day-792 Routed-Shape Attribution

Evidence mode: Ran.

## Verdict

`mn_corn_h4` day 792 lane 1 is classified as
`SOLVER-CLASS / RAW-HYDROGRAPH-NONCONVERGED` for this package. The binding
metric-repair path is not available because the normalized shape miss is not
noise-scale, the hourly CDF distance does not converge, and the raw outlet
hydrograph comparison also worsens on the finer rung pair.

No `SC-OFEROUTE-001` shape-gate amendment lands in this package.

## Command Provenance

Rerun command:

```bash
OPENWEPP_LANED_ACTIVE_TRACE_DETAIL=792:1 \
  .venv/bin/python \
  docs/work-packages/20260708-laned-router-mn-corn-h4-routed-shape-attribution-001/artifacts/run_shape_attribution_ladder.py \
  --members mn_corn_h4 --rungs dx2p5 dx1p25 dx0p625
```

Analysis command:

```bash
.venv/bin/python \
  docs/work-packages/20260708-laned-router-mn-corn-h4-routed-shape-attribution-001/artifacts/analyze_day792_attribution.py
```

Release binary:

- Build command: `cargo build --release -p openwepp-runner --bins`
- Binary: `target/release/openwepp-cli-hill`
- SHA256: `319fbe119e89193018ce9b2894dc7dab56babb7fee2543a0ec9f06f62674b56c`
- Git HEAD at build: `69813293686fcbdb7d46cfab02b5daa5d500d5d6`

Runner environment:

- `OPENWEPP_LANED_ACTIVE=1`
- `OPENWEPP_LANED_ACTIVE_TRACE=1`
- `OPENWEPP_LANED_ACTIVE_TRACE_DETAIL=792:1`
- `OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M=<rung-metres>`

Trace detail selector:

- Env var: `OPENWEPP_LANED_ACTIVE_TRACE_DETAIL=792:1`
- Selector convention: one-based `sim_day:lane`
- Captured detail rows: one row per rung for `sim_day_index=792`,
  `lane_index=1`
- Detail sizes: 76 outlet bins and 77 raw hydrograph samples per rung

## Rung Masses

All values are for `mn_corn_h4`, day 792, lane 1.

| Rung | Source m3 | Outlet m3 | End storage m3 | Tail fold m3 |
|---|---:|---:|---:|---:|
| `dx2p5` | 1.3852874865748004 | 0.54733964274448332 | 0.83794784383031706 | 0 |
| `dx1p25` | 1.3852874865748004 | 0.5492949677736999 | 0.8359925188011007 | 0 |
| `dx0p625` | 1.3852874865748004 | 0.54920455301493876 | 0.83608293355985985 | 0 |

## Discriminating Tests

### 1. Normalization-Amplification

The failing fine-reference pair is `dx1p25` vs `dx0p625`. Its normalized
hourly-shape L1 is `0.020944940478490041`, above the one-third adequacy
threshold `0.0166667`.

Converted to absolute hourly mass, the same delta is
`0.011445388178193001 m3`. The outlet/storage total difference on that pair is
only `9.0414758761148128e-05 m3` outlet and
`9.0414758759149727e-05 m3` storage.
The reshuffled hourly mass is therefore about `126.6x` the end-window storage
difference. This is not a near-zero denominator or noise-scale amplification.

Result: `FAIL-METRIC-CLASS`.

### 2. Hour-Edge Aliasing

Hourly CDF max distance does not converge on the finer pair:

| Pair | Hourly L1 weight | Hourly CDF Linf weight | Hourly L1 mass m3 | Hourly CDF Linf m3 |
|---|---:|---:|---:|---:|
| `dx2p5` vs `dx1p25` | 0.0087600931235007444 | 0.001801155375319774 | 0.0044430831936446355 | 0.0026193895181312321 |
| `dx1p25` vs `dx0p625` | 0.020944940478490041 | 0.0099207330198687327 | 0.011445388178193001 | 0.0053651985495017929 |

The binned L1 increase is accompanied by a larger CDF distance, so this is not
only a mass packet straddling an hour boundary with converged cumulative
arrival.

Result: `FAIL-PROJECTION-ALIASING`.

### 3. Raw Unbinned Outlet Hydrograph

The raw outlet-hydrograph comparison also worsens on the finer pair:

| Pair | Bin mass L1 m3 | Bin CDF Linf m3 | Sampled hydrograph L1 m3/s | Sampled hydrograph Linf m3/s |
|---|---:|---:|---:|---:|
| `dx2p5` vs `dx1p25` | 0.0055935927057736276 | 0.0031141348041756056 | 6.2151030064151318e-06 | 7.8654525986449026e-07 |
| `dx1p25` vs `dx0p625` | 0.018646671573609861 | 0.0053651985495015708 | 2.0718523970677621e-05 | 6.1562047743209181e-06 |

The raw unbinned outlet signal is therefore not converged under the tested
rungs. This satisfies the handoff's solver/day classification branch.

Result: `SOLVER-CLASS-HOLD`.

## Important Row Clarification

The prior hold shorthand said the `mn_corn_h4` miss was flat around
`0.0202..0.0209` on day 792. The trace-enabled rerun sharpens that statement:
day 792 is the `dx1p25` vs `dx0p625` max at
`0.020944940478490041`, but day 792 is only
`0.0087600931235007444` on the `dx2p5` vs `dx1p25` pair.
The `dx2p5` vs `dx1p25` package-level max remains `0.02018051100943346` on a
different positive-source day. This package's attribution is therefore for the
actual day-792 fine-reference blocker named by the handoff.

## Stored Evidence

- Compact run summary:
  `artifacts/shape-attribution-summary.md`
- Machine-readable run summary:
  `artifacts/shape-attribution-summary.json`
- Machine-readable day attribution:
  `artifacts/day792-attribution.json`
- Raw run trees:
  `artifacts/shape-attribution-runs/`, package-ignored
