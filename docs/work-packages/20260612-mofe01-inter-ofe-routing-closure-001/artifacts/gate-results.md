# gate results

Status: M-C executed-hold; execution green; publication and identity gates blocked

Evidence mode: Ran + Static

## M-C ran

| Gate/check | Result | Notes |
| --- | --- | --- |
| Fresh H1-H36 current batch | PASS execution | 36/36 exit code `0` under `/tmp/openwepp_mofe01_mc`; no production edits were made during M-C. |
| Local owcmp H1-H36 semantic batch, no comparator subagent | PASS execution, FAIL semantic | Operator explicitly directed local comparison because GPT-5.3-Codex-Spark weekly quota was exhausted. `execution_verdict=PASS`; `semantic_verdict=FAIL`; `semantic_pass_count=0/36`; `structural_row_key_failures=350720`; first divergent H1 key `[1,1,2000]`. |
| M-C direct publication audit | FAIL | All 29 multi-OFE surfaces still publish a single `OFE=1` row/day, `UpStrmQ=0`, `QOFE=Q`, and `publication_ofe_policy=single-row-canonicalized-hillslope-aggregate`. |
| H1 day-1 five-row publication red test | FAIL | Candidate H1 day 1 publishes only `OFE=1`; legacy-compatible M-C lane requires five OFE rows or a contracted equivalent per-OFE surface. |
| Downstream handoff printed-precision red test | BLOCKED | No downstream OFE rows are emitted, so current `UpStrmQ == previous QOFE` and current `SubRIn == previous latqcc` cannot be observed on WAT output. |
| Single-OFE anchor comparison | PASS | H8/H15/H19/H20/H22/H23/H28 byte-identical to M-B outputs for `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet`. |
| Full three-identity acceptance | BLOCKED | Aggregate annual identity remains at noise on M-B smoke representatives, but transfer and true per-element identities require real per-OFE publication. |
| Dual review | PASS with accepted findings | Review A found M-B overclaim and missing comparator-subagent override disclosure; Review B found missing override disclosure in implementation evidence and stale reading-map status. Findings accepted and fixed. |
| Dual verification | PASS | Verification A found low wording/status cleanup items that were fixed; Verification B reported no findings. |
| Full Rust closure loop | NOT RUN | M-C made no production Rust, science-contract, dependency, or test edits. |
| `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --format plain` | PASS | 27 files validated, 0 errors, 0 warnings. |

Detailed evidence: `m-c-wat-publication-closure-evidence.md`.

## M-B ran

| Gate/check | Result | Notes |
| --- | --- | --- |
| Focused M-B contract/kernel tests | PASS | `mofe01_inter_ofe_route_contract`, `wb11_hydrology_kernel_contract`, and `wb14_infiltration_hyetograph_kernel_contract` M-B tests passed. |
| Runner seed regression | PASS | `mofe01_mb_wb11_seed_purges_stale_daily_carryover_for_mofe_hourly_arrays` passed. |
| H11/H6/H9/H1 smoke | PASS | Representative 2/3/4/5-OFE smoke surfaces completed. |
| Full H1-H36 current batch | PASS | 36/36 exit code `0`; 36 manifests completed; 36 WAT parquet outputs with 2192 rows each. |
| Single-OFE anchor comparison | PASS | H8/H15/H19/H20/H22/H23/H28 byte-identical to M-A outputs for `.hbp`, `.loss.json`, `.plot.parquet`, `.wat.parquet`. |
| Local owcmp H1-H36 semantic batch, no comparator subagent | PASS execution, FAIL semantic | `execution_verdict=PASS`; `semantic_verdict=FAIL`; `structural_row_key_failures=350720`; row-key/per-OFE WAT publication closure moves to M-C. |
| `cargo fmt --check` | PASS | Final post-edit run. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Final post-edit run. |
| `cargo test --workspace` | PASS | Final post-edit run. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| `markdown-doc lint --path ... --format plain` | PASS | Package plus touched SC docs: 29 files validated, 0 errors, 0 warnings. |

Detailed evidence: `m-b-hydrology-route-closure-evidence.md`.

## M-A ran

| Gate/check | Result | Notes |
| --- | --- | --- |
| `tools/owcmp/owcmp env --manifest tools/owcmp/suites/wa-cascades-mofe-ksflag0.json` | PASS | Confirmed `.venv/bin/python`, pyarrow 24.0.0, and arboreal-dendrite H1-H36 legacy outputs. |
| `cargo build -p openwepp-runner --bin openwepp-cli-hill` | PASS | Built current hillslope binary for isolated batch. |
| `cargo build -p openwepp-runner --bin open_wepp_runner` | PASS | Built launcher boundary used by wrapper contract checks. |
| Isolated current H1-H36 batch | RAN, expected evidence failure for MOFE | 7/7 single-OFE surfaces passed; 29/29 multi-OFE surfaces failed before output publication. |
| Local legacy H1-H36 WAT parse | PASS | Parsed 271,808 rows and produced per-OFE-count closure/routing calibration. |

## Not run / anti-evasion

| Gate/check | Reason |
| --- | --- |
| `cargo fmt --check` | M-A made documentation/evidence edits only; no production Rust edits. |
| `cargo clippy --workspace --all-targets -- -D warnings` | M-A made documentation/evidence edits only; no production Rust edits. |
| `cargo test --workspace` | M-A made documentation/evidence edits only; no production Rust edits. |
| `cargo deny check` | M-A made documentation/evidence edits only; no dependency edits. |
| `bash tools/release/check_authority_suite_antievasion.sh` | No external-authority suite posture, cohort fixture binding, or required-case binding was edited by M-A, M-B, or M-C. |
| `cargo test --test auth11_required_suite_obligation_guards_contract` | Same anti-evasion non-trigger as above; the full workspace test did include this target and it passed. |
