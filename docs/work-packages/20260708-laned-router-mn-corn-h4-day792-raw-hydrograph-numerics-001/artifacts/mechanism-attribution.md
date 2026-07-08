# Raw-Hydrograph Mechanism Attribution

Evidence mode: Ran.

## Verdict

Classification: `MECHANISM-HOLD-CFL-TIMESTEP-TRANSITION`.

Fine rung leaves the 300 s cap and becomes CFL-limited while the middle rung remains cap-limited; the failing comparison is not a pure spatial reference check

This package does not amend the routed-shape tolerance and does not promote
a production target `dx` default.

## Command Provenance

Rerun command:

```bash
OPENWEPP_LANED_ACTIVE_TRACE_DETAIL=792:1 \
  .venv/bin/python \
  docs/work-packages/20260708-laned-router-mn-corn-h4-day792-raw-hydrograph-numerics-001/artifacts/run_raw_hydrograph_numerics_ladder.py \
  --members mn_corn_h4 --rungs dx2p5 dx1p25 dx0p625
```

Analysis command:

```bash
.venv/bin/python \
  docs/work-packages/20260708-laned-router-mn-corn-h4-day792-raw-hydrograph-numerics-001/artifacts/analyze_raw_hydrograph_numerics.py
```

Release binary:

- Build command: `cargo build --release -p openwepp-runner --bins`
- Binary: `target/release/openwepp-cli-hill`
- SHA256: `df6fa6cd7fcfb2312cfc9d1fb75f9e1a79372d0c2cd7b1d61618ba7c07c698fd`
- Git HEAD at build: `88b0cea5708f44b265fc41a94dcd3075a7b84caf`

## Rung Masses

| Rung | Cells | dx m | Source m3 | Outlet m3 | End storage m3 | Step count | Max Courant | Bin recon Linf m3 | Clamp m3 | TVD-limited steps | Stage-limiter reductions |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `dx2p5` | 33 | 2.4606060606060609 | 1.3852874865748004 | 0.54733964274448332 | 0.83794784383031706 | 228 | 0.43594738672899525 | 3.4694469519536142e-18 | 0 | 0 | 0 |
| `dx1p25` | 65 | 1.2492307692307694 | 1.3852874865748004 | 0.5492949677736999 | 0.8359925188011007 | 228 | 0.85874995859419834 | 3.4694469519536142e-18 | 0 | 0 | 0 |
| `dx0p625` | 130 | 0.62461538461538468 | 1.3852874865748004 | 0.54920455301493876 | 0.83608293355985985 | 330 | 0.90000000000000002 | 1.5265566588595902e-16 | 0 | 0 | 0 |

## Fine-Pair Raw-Hydrograph Evidence

| Pair | Hour shape L1 | Hour CDF Linf | Bin mass L1 m3 | Bin CDF Linf m3 | Sampled hydrograph L1 m3/s | Outlet delta m3 | Storage delta m3 |
|---|---:|---:|---:|---:|---:|---:|---:|
| `dx2p5` vs `dx1p25` | 0.0087600931235007444 | 0.001801155375319774 | 0.0055935927057736276 | 0.0031141348041756056 | 6.2151030064151318e-06 | -0.0019553250292165814 | 0.0019553250292163593 |
| `dx1p25` vs `dx0p625` | 0.020944940478490041 | 0.0099207330198687327 | 0.018646671573609861 | 0.0053651985495015708 | 2.0718523970677621e-05 | 9.0414758761148128e-05 | -9.0414758759149727e-05 |

Top fine-pair outlet-bin difference:

- Bin index: `70`
- Window: `63000` to `63900` s
- Absolute delta: `0.0055405842968888272 m3`
- Signed delta (`dx1p25 - dx0p625`): `-0.0055405842968888272 m3`

Maximum fine-pair cumulative outlet difference:

- Bin index: `71`
- Window: `63900` to `64800` s
- Absolute CDF delta: `0.0053651985495015708 m3`
- Signed CDF delta (`dx1p25 - dx0p625`): `-0.0053651985495015708 m3`

## Step-Trace Discriminants

Fine-pair source and upstream controls:

- `dx1p25` step source total: `1.3852874865748002 m3`
- `dx0p625` step source total: `1.3852874865748011 m3`
- `dx1p25` upstream inflow total: `0 m3`
- `dx0p625` upstream inflow total: `0 m3`
- `dx1p25` clipped step-to-bin reconstruction Linf: `3.4694469519536142e-18 m3`
- `dx0p625` clipped step-to-bin reconstruction Linf: `1.5265566588595902e-16 m3`
- `dx1p25` max-Courant cell/x: `64` / `80.575384615384621 m`
- `dx0p625` max-Courant cell/x: `8` / `5.3092307692307701 m`

Fine-pair boundary and limiter controls:

- `dx1p25` negative outlet-outflow steps: `0`
- `dx0p625` negative outlet-outflow steps: `0`
- `dx1p25` min predictor/corrector outlet face: `0` / `0` m3/s
- `dx0p625` min predictor/corrector outlet face: `0` / `0` m3/s
- `dx1p25` TVD-limited steps: `0`
- `dx0p625` TVD-limited steps: `0`
- `dx1p25` stage-limiter reductions: `0`
- `dx0p625` stage-limiter reductions: `0`

Top-bin step-window summaries are recorded in
`mechanism-attribution.json` under
`comparisons[*].top_bin_step_windows`.
