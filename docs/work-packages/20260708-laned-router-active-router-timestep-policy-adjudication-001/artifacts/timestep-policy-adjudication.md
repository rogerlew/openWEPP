# Timestep Policy Adjudication

Evidence mode: Ran.

## Verdict

Classification: `TIMESTEP-POLICY-ARTIFACT-CLOSED`.

The fixed-300 s miss closes when the spatial pair is compared under the same refined 75 s timestep cap

Contract action: amend SC-OFEROUTE-001 before any renewed mesh promotion so target-dx adequacy is evaluated on a coupled space-time basis.

This package does not amend the routed-shape tolerance and does not promote
a production target `dx` default.

## Command Provenance

Rerun command:

```bash
OPENWEPP_LANED_ACTIVE_TRACE_DETAIL=792:1 \
  .venv/bin/python \
  docs/work-packages/20260708-laned-router-active-router-timestep-policy-adjudication-001/artifacts/run_timestep_policy_ladder.py \
  --members mn_corn_h4
```

Analysis command:

```bash
.venv/bin/python \
  docs/work-packages/20260708-laned-router-active-router-timestep-policy-adjudication-001/artifacts/analyze_timestep_policy.py
```

Release binary:

- Build command: `cargo build --release -p openwepp-runner --bins`
- Binary: `target/release/openwepp-cli-hill`
- SHA256: `8876fa04ca520126b958d83a7c5777da6f793e51fba4c346432f065b31647aaa`
- Git HEAD at build: `07a12de694040e0e30edc714f297cfdc79a67674`

## Rung Masses

| Rung | Cells | dx m | max dt s | Source m3 | Outlet m3 | End storage m3 | Step count | Max Courant | Bin recon Linf m3 | Clamp m3 | TVD-limited steps | Stage-limiter reductions |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `dx1p25_dt300` | 65 | 1.2492307692307694 | 300 | 1.3852874865748004 | 0.5492949677736999 | 0.8359925188011007 | 228 | 0.85874995859419834 | 3.4694469519536142e-18 | 0 | 0 | 0 |
| `dx1p25_dt150` | 65 | 1.2492307692307694 | 150 | 1.3852874865748004 | 0.54835726250901329 | 0.83693022406578643 | 456 | 0.39188085047703552 | 3.4694469519536142e-18 | 0 | 0 | 0 |
| `dx1p25_dt75` | 65 | 1.2492307692307694 | 75 | 1.3852874865748004 | 0.54803784448798631 | 0.83724964208680663 | 912 | 0.18658980939306125 | 6.9388939039072284e-18 | 0 | 0 | 0 |
| `dx0p625_dt300` | 130 | 0.62461538461538468 | 300 | 1.3852874865748004 | 0.54920455301493876 | 0.83608293355985985 | 330 | 0.90000000000000002 | 1.5265566588595902e-16 | 0 | 0 | 0 |
| `dx0p625_dt150` | 130 | 0.62461538461538468 | 150 | 1.3852874865748004 | 0.54855546468877114 | 0.83673202188602891 | 458 | 0.90000000000000002 | 8.3266726846886741e-17 | 0 | 0 | 0 |
| `dx0p625_dt75` | 130 | 0.62461538461538468 | 75 | 1.3852874865748004 | 0.54882295573615869 | 0.83646453083863181 | 912 | 0.37317962873284899 | 6.9388939039072284e-18 | 0 | 0 | 0 |

## Pair Evidence

Shape threshold: `0.016666666666666666`.

| Pair | Role | Hour shape L1 | Passes 1/60 | Hour CDF Linf | Bin mass L1 m3 | Bin CDF Linf m3 | Sampled hydrograph L1 m3/s | Outlet delta m3 | Storage delta m3 |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `dx1p25_timestep_300` vs `150` | same_dx_timestep | 0.011126438147409356 | True | 0.0036736656303355542 | 0.0063381265152652631 | 0.0014612749264290015 | 7.0423627947391803e-06 | 0.00093770526468661686 | -0.00093770526468572868 |
| `dx1p25_timestep_150` vs `75` | same_dx_timestep | 0.0032256604681117603 | True | 0.0016128302340557843 | 0.0018167640470906799 | 0.00072341652960411507 | 2.0186267189896315e-06 | 0.0003194180210269737 | -0.00031941802102020134 |
| `dx0p625_timestep_300` vs `150` | same_dx_timestep | 0.020011501893244476 | False | 0.0078552604364313661 | 0.026579983514342577 | 0.0049135916048153039 | 2.9533315015936181e-05 | 0.00064908832616761281 | -0.0006490883261690561 |
| `dx0p625_timestep_150` vs `75` | same_dx_timestep | 0.0080422722820031947 | True | 0.0013483681931056868 | 0.012394483763136456 | 0.0014325406895947523 | 1.3771648625707167e-05 | -0.00026749104738754692 | 0.00026749104739709484 |
| `spatial_dx1p25` vs `dx0p625_dt300` | same_dt_spatial | 0.020944940478490041 | False | 0.0099207330198687327 | 0.018646671573609861 | 0.0053651985495015708 | 2.0718523970677621e-05 | 9.0414758761148128e-05 | -9.0414758759149727e-05 |
| `spatial_dx1p25` vs `dx0p625_dt150` | same_dt_spatial | 0.0078094781966157063 | True | 0.0024709774565545573 | 0.011787424711053575 | 0.002096701247011834 | 1.3097138567837304e-05 | -0.00019820217975785592 | 0.00019820217975752286 |
| `spatial_dx1p25` vs `dx0p625_dt75` | same_dt_spatial | 0.0029828040053040839 | True | 0.00074669277382122257 | 0.0013131573219307951 | 0.00098583895246889819 | 1.4590636910342013e-06 | -0.00078511124817237654 | 0.00078511124817481903 |

Top original fixed-300 outlet-bin difference:

- Bin index: `70`
- Window: `63000` to `63900` s
- Absolute delta: `0.0055405842968888272 m3`
- Signed delta (`dx1p25_dt300 - dx0p625_dt300`): `-0.0055405842968888272 m3`

Maximum original fixed-300 cumulative outlet difference:

- Bin index: `71`
- Window: `63900` to `64800` s
- Absolute CDF delta: `0.0053651985495015708 m3`
- Signed CDF delta (`dx1p25_dt300 - dx0p625_dt300`): `-0.0053651985495015708 m3`

Refined same-75 spatial pair:

- Hour shape L1: `0.0029828040053040839`
- Bin mass CDF Linf: `0.00098583895246889819 m3`
- Outlet delta: `-0.00078511124817237654 m3`

## Step-Trace Discriminants

Original and refined spatial-pair source and upstream controls:

- `dx1p25_dt300` step source total: `1.3852874865748002 m3`
- `dx0p625_dt300` step source total: `1.3852874865748011 m3`
- `dx1p25_dt75` step source total: `1.3852874865748002 m3`
- `dx0p625_dt75` step source total: `1.3852874865748019 m3`
- `dx1p25_dt300` upstream inflow total: `0 m3`
- `dx0p625_dt300` upstream inflow total: `0 m3`
- `dx1p25_dt75` upstream inflow total: `0 m3`
- `dx0p625_dt75` upstream inflow total: `0 m3`

Boundary and limiter controls:

- `dx1p25_dt300` negative outlet-outflow steps: `0`
- `dx0p625_dt300` negative outlet-outflow steps: `0`
- `dx1p25_dt75` negative outlet-outflow steps: `0`
- `dx0p625_dt75` negative outlet-outflow steps: `0`
- `dx1p25_dt300` min predictor/corrector outlet face: `0` / `0` m3/s
- `dx0p625_dt300` min predictor/corrector outlet face: `0` / `0` m3/s
- `dx1p25_dt75` min predictor/corrector outlet face: `0` / `0` m3/s
- `dx0p625_dt75` min predictor/corrector outlet face: `0` / `0` m3/s
- `dx1p25_dt300` TVD-limited steps: `0`
- `dx0p625_dt300` TVD-limited steps: `0`
- `dx1p25_dt75` TVD-limited steps: `0`
- `dx0p625_dt75` TVD-limited steps: `0`
- `dx1p25_dt300` stage-limiter reductions: `0`
- `dx0p625_dt300` stage-limiter reductions: `0`
- `dx1p25_dt75` stage-limiter reductions: `0`
- `dx0p625_dt75` stage-limiter reductions: `0`

Top-bin step-window summaries are recorded in
`timestep-policy-adjudication.json` under
`comparisons[*].top_bin_step_windows`.
