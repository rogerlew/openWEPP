# Implementation and test evidence

Status: `PASS`

Evidence mode: `Ran`

## Diagnosis

Default-off, non-persisted iteration tracing partitioned the baseline 128 caps
into 100 cold-content-only caps, 27 materially nonconverged maps, and one
receipt-replay cap. The 100 residuals ranged from
`1.010305823001545e-6` to `9.568239875079598e-6 J m^-2`; every history revisited
finalization. The trace showed finalization replacing the damped iterate with
the raw candidate. After correcting that transition, it exposed one remaining
on/off tolerance cycle, corrected by the single guarded stabilization update.
All temporary tracing was removed before terminal qualification.

## Initial one-day result — superseded for closure identity

Baseline commit: `792af753e`. Terminal command:

`env CARGO_PROFILE_TEST_OPT_LEVEL=3 RUST_MIN_STACK=67108864 nix develop -c cargo test -p openwepp-runner cqr_stage3_one_day_qualification_with_telemetry --lib -- --ignored --nocapture`

| Metric | Baseline | Terminal | Change |
|---|---:|---:|---:|
| accepted supports | 504 | 491 | -13 (-2.58%) |
| rejected trials | 227 | 205 | -22 (-9.69%) |
| total trials | 731 | 696 | -35 (-4.79%) |
| fixed-point caps | 128 | 32 | -96 (-75.0%) |
| cold-content-only caps | 100 | 2 | -98 (-98.0%) |
| exact 60-second supports | 49 | 49 | unchanged |
| body wall | 374.23 s | 336.52 s | -37.71 s (-10.08%) |

Full command elapsed was 397.06 s (`user=774.14 s`, `sys=7.21 s`, maximum RSS
5,899,548 KiB). Accepted-width distribution: 60 s: 49; 120 s: 92; 180 s:
320; 240 s: 17; 300 s: 3; 420 s: 3; 480 s: 1; 900 s: 3; 1800 s: 3.

Limiting reasons are 32 fixed-point caps and 45 scaled physical-comparison
rejections; discrete comparison rejections are zero. Remaining cap signatures
are 22 900-second Picard mass-SWE, two 180-second Picard mass-SWE, two
420-second Picard mass-SWE, two 840-second Picard temperature, two 420-second
finalization cold-content, one 600-second all-map fingerprint, and one
1800-second receipt-replay fingerprint. Comparison owners are surface-liquid
WB14 parent working state (33), snow temperature (6), snow cumulative
deposition (5), and persistent surface-liquid owner (1).

Ledger closure: maximum mass residual
`3.55271367880050093e-15 kg m^-2` versus `1e-9`; maximum energy residual
`1.39698386192321777e-9 J m^-2` versus `1e-6`. Receipt reseal: energy
`9.96351445792242885e-10 J m^-2` versus `1e-9`; temperature
`1.07434061646927148e-11 K` versus `1e-8`. Log:
`/tmp/stage3_fp_cold/one-day-terminal-source.log`, SHA-256
`6dc433684d8d8470f88d43273afcbbad42544b10f9981e286f6f0b5c1a9835c0`.

Review finding RB-004 correctly notes that this run's log identifies a dirty
tree and the artifact did not retain a complete source manifest. Its metrics
remain diagnostic evidence but no longer support the exact-terminal closure
claim. A clean-commit replacement run is required after review corrections are
committed.
