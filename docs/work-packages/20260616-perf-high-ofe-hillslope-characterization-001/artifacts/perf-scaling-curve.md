# PERFHO01 Scaling Curve

Status: COMPLETE 2026-06-16
Evidence mode: **Ran** (release timings and manifest/parquet inspection) + **Static** (fit arithmetic)

## Measurement Setup

All measurements used:

- Binary: `target/release/openwepp-cli-hill`
- HEAD: `d6cb4ef9`
- Policy: `--policy compat --legacy-sidecar-discovery`
- Mode: staged `without_ui` run directories under `/tmp/perfho01/run-dirs/`
- Outputs: required HBP/loss plus `wat` and `pass_parquet`; `plot` was enabled
  but is a generic text optional output despite the `.parquet` extension.
- Package runfiles: `artifacts/runfiles/`

The arboreal source directory contains `wepp_ui.txt`; to keep the low-OFE ladder
comparable to H2637 `without_ui`, PERFHO01 staged clean `/tmp/perfho01/run-dirs`
with selected `p*` inputs plus `snow.txt` and `pmetpara.txt`, omitting
`wepp_ui.txt`.

## Timing Table

| Case | Source | OFEs | Elapsed s | User s | Sys s | Max RSS KB | Executed days | `wat` rows | `pass` rows | Publication policy | s/day | s/OFE-day |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---|---:|---:|
| ofe1 | p15 | 1 | 6.15 | 6.12 | 0.02 | 19,756 | 2,192 | 2,192 | 2,192 | single-row-canonicalized-hillslope-aggregate | 0.00280566 | 0.00280566 |
| ofe2 | p11 | 2 | 13.90 | 13.87 | 0.02 | 21,748 | 2,192 | 4,384 | 2,192 | per-ofe-dynamic-water-balance-state | 0.00634124 | 0.00317062 |
| ofe3 | p12 | 3 | 20.67 | 20.62 | 0.03 | 21,948 | 2,192 | 6,576 | 2,192 | per-ofe-dynamic-water-balance-state | 0.00942974 | 0.00314325 |
| ofe4 | p25 | 4 | 32.47 | 32.36 | 0.09 | 24,872 | 2,192 | 8,768 | 2,192 | per-ofe-dynamic-water-balance-state | 0.01481296 | 0.00370324 |
| ofe5 | p1 | 5 | 32.30 | 32.26 | 0.02 | 25,228 | 2,192 | 10,960 | 2,192 | per-ofe-dynamic-water-balance-state | 0.01473540 | 0.00294708 |
| h2637 | p2637 | 19 | 978.55 | 977.99 | 0.42 | 228,440 | 12,419 | 235,961 | 12,419 | per-ofe-dynamic-water-balance-state | 0.07879459 | 0.00414708 |

H2637 row check: `235,961 = 19 * 12,419`.

## Fit

Fit model: `seconds_per_sim_day = a * OFE_count^b`, with log least squares.

| Point set | a | exponent b | Interpretation |
|---|---:|---:|---|
| All points, 1-5 + 19 OFEs | 0.00281259 | 1.12139 | Modestly superlinear, but 1-OFE uses the single-row policy. |
| Dynamic per-OFE points only, 2-5 + 19 OFEs | 0.00281970 | 1.12008 | Best PERFHO01 curve for the per-OFE WB13 path. |
| Low dynamic points only, 2-5 OFEs | 0.00322858 | 1.00022 | Essentially linear within the low-OFE ladder; fixture differences explain the 4/5 non-monotonicity. |

## Scaling Verdict

The low-OFE dynamic path is linear in OFE count. H2637 adds a modest superlinear
penalty: 19-OFE s/OFE-day is `0.00414708`, versus about `0.0030-0.0037` for the
2-5 OFE ladder. The huge wall-clock gap is therefore not an explosive OFE-count
algorithmic exponent; it is a large constant cost paid per OFE-day, amplified by
19 OFEs and 12,419 days.

The CPU-bound nature is explicit in the H2637 timing: `977.99` user seconds out
of `978.55` elapsed seconds. System time and output I/O are not the primary
driver.
