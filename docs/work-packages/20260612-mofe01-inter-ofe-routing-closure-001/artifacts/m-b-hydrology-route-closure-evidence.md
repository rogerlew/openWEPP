# M-B hydrology route closure evidence

Status: M-B executed; execution blocker retired; full identity acceptance incomplete

Evidence mode: Ran + Static

## Static

- Comparisons were run locally in this shell. No `comparator subagent` was used because GPT-5.3-Codex-Spark weekly quota was exhausted and the operator explicitly directed local comparison execution.
- M-B contract authority was added to `SC-RUNOFFPART-001` and `SC-WATBAL-001` for separated upstream surface/lateral carry, stale aggregate purge before array-enabled MOFE execution, current saturation carry, and conservation identities.
- No legacy `/workdir/wepp-forest_260430_baseline` or `/wc1` files were edited.

## Ran

| Command/check | Result | Notes |
| --- | --- | --- |
| `cargo test --test mofe01_inter_ofe_route_contract --test wb11_hydrology_kernel_contract --test wb14_infiltration_hyetograph_kernel_contract mofe01_mb -- --nocapture` | PASS | Contract authority, separated `UpStrmQ`/`SubRIn`, and positive top-layer excess saturation carry tests passed. |
| `cargo test -p openwepp-runner mofe01_mb_wb11_seed_purges_stale_daily_carryover_for_mofe_hourly_arrays -- --nocapture` | PASS | MOFE hourly-array seed purges stale daily `wb12_runoff_carryover`. |
| H11/H6/H9/H1 smoke run with `target/debug/openwepp-cli-hill --policy compat --legacy-sidecar-discovery` | PASS | Representative 2/3/4/5-OFE smoke surfaces completed. |
| Full H1-H36 current batch with `target/debug/openwepp-cli-hill --policy compat --legacy-sidecar-discovery` | PASS | 36/36 exit code `0`; 36 manifests; 36 `HSCHED-OK-001`; 36 WAT parquet files with 2192 rows each; no `CLIHILL-E-*` or `HKERNEL-*-E-*` log lines. |
| Single-OFE anchor `cmp` against M-A outputs for H8/H15/H19/H20/H22/H23/H28 | PASS | `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet` were byte-identical for all seven single-OFE anchors. |
| `tools/owcmp/owcmp batch h1-h39-semantic --baseline-dir /wc1/runs/ar/arboreal-dendrite/wepp/output --baseline-pattern 'H{h}.wat.dat' --candidate-dir /tmp/openwepp_mofe01_mb/output --candidate-pattern 'H{h}.wat.parquet' --output-root /tmp/openwepp_mofe01_mb/owcmp_after_fix --start 1 --end 36` | PASS execution, FAIL semantic | `summary.json`: `execution_verdict=PASS`, `semantic_verdict=FAIL`, `semantic_pass_count=0/36`, `structural_row_key_failures=350720`, first divergent H1 key `[1,1,2000]`. Focus columns had zero numeric fail counts; the blocker is row-key/publication structure, assigned to M-C. |
| `cargo fmt --check` | PASS | Final post-edit run. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Final post-edit run. |
| `cargo test --workspace` | PASS | Final post-edit full workspace run after stale contract-version assertions were updated to `SC-WATBAL-001` version 154. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --path docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md --path docs/specifications/science-contracts/contracts/SC-WATBAL-001.md --path docs/specifications/science-contracts/index.md --format plain` | PASS | 29 files validated, 0 errors, 0 warnings. |

## Disposition

M-B retired the current valid-input fail-closed blocker identified in M-A: all
29 multi-OFE arboreal-dendrite surfaces now execute through the full 2192-day
run. This is execution-route progress, not full closure acceptance. The local
owcmp semantic failure is the expected row-key/per-OFE WAT publication flag,
and the transfer/true per-element identities remain unevidenced until a real
per-OFE publication surface exists.

## Claude review addendum — independent conservation audit (2026-06-12)

Evidence mode: Ran — duckdb over `/tmp/openwepp_mofe01_mb/output/`.

The M-B gate "the three identities at noise on every executing surface" was
not evidenced in the increment record (identities were pinned in contract
but residuals not measured). Independent audit of the published WAT surface
on the smoke representatives, annual identity
`RM + Irr + UpStrmQ + SubRIn − (Interception+Q+Ep+Es+Er+Dp+latqcc+Tile)
− Δ(Total-Soil + frozwt)` (years 2+):

| surface | OFE class | worst annual residual |
|---|---|---|
| H15 | 1 | −8.95e-13 mm |
| H11 | 2 | +4.83e-13 mm |
| H6 | 3 | +6.82e-13 mm |
| H9 | 4 | +1.33e-13 mm-class (−5.12e-13) |
| H1 | 5 | +6.68e-13 mm |

The published surface conserves at the FDHP01-era noise floor across the
ladder. The transfer and true per-element identities require M-C's per-OFE
publication (current multi-OFE WAT publishes a single `ofe_id=1` row/day
with `UpStrmQ=0` and `QOFE=Q` aliased — the documented M-C scope), so the
full three-identity audit lands with M-C.

Two notes for M-C/M-E:

1. **Identity basis caution:** the M-A calibration ledger's diagnostic
   includes `Snow-Water` in the storage delta. Against an `RM`
   (rain+melt-at-ground) input basis that double-counts the snowpack — on
   openWEPP output it produces false residuals of hundreds of mm (measured:
   H15 +385 mm with snow store vs −9e-13 without). The legacy residual
   table in the calibration ledger should be re-verified for the same
   contamination before it is used as the per-OFE-count trust curve.
2. The nonzero `SubRIn` on the single published multi-OFE row needs its
   semantics pinned in M-C (the row labels `ofe_id=1`, where `SubRIn`
   should be zero by definition).
