# gate results

Status: M-D complete; design artifact complete; package active for M-E0

Evidence mode: Ran + Static

## M-D ran

| Gate/check | Result | Notes |
| --- | --- | --- |
| M-D production-code boundary | PASS | No production Rust, science-contract, dependency, Cargo, test, external-authority, legacy, or `/wc1` substrate files were edited. |
| Per-OFE state architecture artifact | PASS | `mofe-per-ofe-state-architecture.md` defines the target `PerOfeDailyWaterBalanceCollection`, per-OFE record contents, lifecycle, and aggregate derivation rule. |
| Current-tree file:line citations | PASS | The artifact cites current scheduler, kernel-contract, runner, publication, topology, summary accumulator, and MOFE carry seams. |
| Sequential execution model decision | PASS | M-D selects per-OFE lane iteration over the existing phase graph and records why `TopologyGraph` N-node encoding is not the M-E path. |
| Legacy `irs`/`rochek`/WATBAL mapping | PASS | The artifact maps per-plane loop, continuation classification, WATBAL mutation isolation, and surface-saturation handoff obligations to the target state model. |
| Contract surface definition | PASS | The artifact names required M-E0 amendments to `SC-RUNOFFPART-001`, `SC-WATBAL-001`, and `SC-SYSTEM-001`, with measurable per-element, transfer, single-OFE, hillslope-total, and publication identities. |
| M-E red-test and sub-increment plan | PASS | The artifact breaks M-E into M-E0 through M-E6 with measurable gates and no required gate deferred across a red boundary. |
| `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --format plain` | PASS | 30 files validated, 0 errors, 0 warnings. |
| Dual review | PASS | Review A findings on missing M-D review/verification evidence, citation precision, and stale increment taxonomy were accepted and fixed; Review B findings on M-D review provenance, carry-array citation precision, and historical gate taxonomy were accepted and fixed. |
| Dual verification | PASS | Verification A and Verification B found no blocking issues; Verification B's non-blocking WB13 citation precision note was accepted and fixed. |
| Full Rust closure loop | NOT RUN | M-D made no production Rust, science-contract, dependency, Cargo, or test edits. |
| Comparator/heavy comparison | NOT RUN | M-D is design-only and did not require batch execution or comparator comparison. |

## M-C2 ran

| Gate/check | Result | Notes |
| --- | --- | --- |
| M-C2 scoping-first seam read | PASS | Existing hourly carry arrays are hour-indexed transfer/copy-forward state, not per-OFE daily WB output state. Current writeback is aggregate scalar state/flux maps. |
| `cargo build -p openwepp-runner --bin openwepp-cli-hill` | PASS | Built the current hillslope CLI before replay. |
| Fresh H1-H36 current batch | PASS | 36/36 exit code `0` under `/tmp/openwepp_mofe01_mc2`; no production edits were made during M-C2. |
| Direct M-C2 WAT publication audit | FAIL | All 29 multi-OFE surfaces still publish a single `OFE=1` row/day, `UpStrmQ=0`, `QOFE=Q`, and policy `single-row-canonicalized-hillslope-aggregate`. |
| Local owcmp H1-H36 command execution, no comparator subagent | PASS | Operator explicitly directed local comparison because GPT-5.3-Codex-Spark weekly quota was exhausted. `execution_verdict=PASS`. |
| Local owcmp H1-H36 semantic comparison | FAIL | `semantic_verdict=FAIL`; `semantic_pass_count=0/36`; `structural_row_key_failures=350720`; first divergent H1 key `[1,1,2000]`. |
| M-C2 per-OFE daily state implementation gate | BLOCKED | No OFE-keyed daily WB state surface exists; implementing by splitting aggregate rows would be surrogate physics. |
| Per-element identity gate | BLOCKED | Not measurable without real per-OFE daily state. |
| Transfer identity gate | BLOCKED | Not measurable on daily WB/WAT surfaces without OFE-to-OFE sent/received state. |
| Aggregate identity unchanged | PASS | No production code changed; execution stayed green and focused existing M-B tests passed. |
| Single-OFE anchor comparison | PASS | H8/H15/H19/H20/H22/H23/H28 byte-identical to M-B outputs for `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet`. |
| Focused M-B carry tests | PASS | `cargo test --test wb11_hydrology_kernel_contract mofe01_mb -- --nocapture` and `cargo test -p openwepp-runner mofe01_mb_wb11_seed_purges_stale_daily_carryover_for_mofe_hourly_arrays -- --nocapture`. |
| Dual review | PASS | Review A found no blocking issues; Review B findings on write-set separation and gate taxonomy were accepted and fixed. |
| Dual verification | PASS | Verification A docs-lint count finding was accepted and fixed; Verification B found no blocking issues. |
| `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --format plain` | PASS | 28 files validated, 0 errors, 0 warnings. |
| Broader docs lint including concurrent governance amendments | PASS | `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --path docs/work-packages/AGENTS.md --path docs/standards/kernel-work-package-preparation.md --path docs/codex_exec_plans.md --format plain`: 31 files validated, 0 errors, 0 warnings. |
| Full Rust closure loop | NOT RUN | M-C2 made no production Rust, science-contract, dependency, or test edits. |

Detailed evidence: `m-c2-per-ofe-daily-state-scope-evidence.md`.

## M-C ran

| Gate/check | Result | Notes |
| --- | --- | --- |
| Fresh H1-H36 current batch | PASS | 36/36 exit code `0` under `/tmp/openwepp_mofe01_mc`; no production edits were made during M-C. |
| Local owcmp H1-H36 command execution, no comparator subagent | PASS | Operator explicitly directed local comparison because GPT-5.3-Codex-Spark weekly quota was exhausted. `execution_verdict=PASS`. |
| Local owcmp H1-H36 semantic comparison | FAIL | `semantic_verdict=FAIL`; `semantic_pass_count=0/36`; `structural_row_key_failures=350720`; first divergent H1 key `[1,1,2000]`. |
| M-C direct publication audit | FAIL | All 29 multi-OFE surfaces still publish a single `OFE=1` row/day, `UpStrmQ=0`, `QOFE=Q`, and `publication_ofe_policy=single-row-canonicalized-hillslope-aggregate`. |
| H1 day-1 five-row publication red test | FAIL | Candidate H1 day 1 publishes only `OFE=1`; legacy-compatible M-C lane requires five OFE rows or a contracted equivalent per-OFE surface. |
| Downstream handoff printed-precision red test | BLOCKED | No downstream OFE rows are emitted, so current `UpStrmQ == previous QOFE` and current `SubRIn == previous latqcc` cannot be observed on WAT output. |
| Single-OFE anchor comparison | PASS | H8/H15/H19/H20/H22/H23/H28 byte-identical to M-B outputs for `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet`. |
| Full three-identity acceptance | BLOCKED | Aggregate annual identity remains at noise on M-B smoke representatives, but transfer and true per-element identities require real per-OFE publication. |
| Dual review | PASS | Review A found M-B overclaim and missing comparator-subagent override disclosure; Review B found missing override disclosure in implementation evidence and stale reading-map status. Findings accepted and fixed. |
| Dual verification | PASS | Verification A found low wording/status cleanup items that were fixed; Verification B reported no findings. |
| Full Rust closure loop | NOT RUN | M-C made no production Rust, science-contract, dependency, or test edits. |
| `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --format plain` | PASS | Historical M-C boundary run: 27 files validated, 0 errors, 0 warnings. |

Detailed evidence: `m-c-wat-publication-closure-evidence.md`.

## M-B ran

| Gate/check | Result | Notes |
| --- | --- | --- |
| Focused M-B contract/kernel tests | PASS | `mofe01_inter_ofe_route_contract`, `wb11_hydrology_kernel_contract`, and `wb14_infiltration_hyetograph_kernel_contract` M-B tests passed. |
| Runner seed regression | PASS | `mofe01_mb_wb11_seed_purges_stale_daily_carryover_for_mofe_hourly_arrays` passed. |
| H11/H6/H9/H1 smoke | PASS | Representative 2/3/4/5-OFE smoke surfaces completed. |
| Full H1-H36 current batch | PASS | 36/36 exit code `0`; 36 manifests completed; 36 WAT parquet outputs with 2192 rows each. |
| Single-OFE anchor comparison | PASS | H8/H15/H19/H20/H22/H23/H28 byte-identical to M-A outputs for `.hbp`, `.loss.json`, `.plot.parquet`, `.wat.parquet`. |
| Local owcmp H1-H36 command execution, no comparator subagent | PASS | `execution_verdict=PASS`; row-key/per-OFE WAT publication closure moves to M-C. |
| Local owcmp H1-H36 semantic comparison | FAIL | `semantic_verdict=FAIL`; `structural_row_key_failures=350720`; row-key/per-OFE WAT publication closure moves to M-C. |
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
| Isolated current H1-H36 batch | FAIL | Expected M-A characterization failure for MOFE: 7/7 single-OFE surfaces passed; 29/29 multi-OFE surfaces failed before output publication. |
| Local legacy H1-H36 WAT parse | PASS | Parsed 271,808 rows and produced per-OFE-count closure/routing calibration. |

## Historical not run / anti-evasion

| Gate/check | Result | Reason |
| --- | --- | --- |
| `cargo fmt --check` | NOT RUN | M-A made documentation/evidence edits only; no production Rust edits. |
| `cargo clippy --workspace --all-targets -- -D warnings` | NOT RUN | M-A made documentation/evidence edits only; no production Rust edits. |
| `cargo test --workspace` | NOT RUN | M-A made documentation/evidence edits only; no production Rust edits. |
| `cargo deny check` | NOT RUN | M-A made documentation/evidence edits only; no dependency edits. |
| `bash tools/release/check_authority_suite_antievasion.sh` | NOT RUN | No external-authority suite posture, cohort fixture binding, or required-case binding was edited by M-A, M-B, M-C, M-C2, or M-D. |
| `cargo test --test auth11_required_suite_obligation_guards_contract` | NOT RUN | Same anti-evasion non-trigger as above; the M-B full workspace test did include this target and it passed. |
