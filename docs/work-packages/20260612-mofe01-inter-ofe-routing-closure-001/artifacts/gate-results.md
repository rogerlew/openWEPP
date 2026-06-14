# gate results

Status: M-F-REDO-CLONE executed; runoff anti-clone and identity gates close.
Package remains active for M-F-REDO2 `QOFE` local-depth publication closure.

Evidence mode: Ran + Static

## M-F-REDO-CLONE scoped acceptance gates

| Gate/check | Result | Notes |
| --- | --- | --- |
| Root-cause stale multi-step infiltration | PASS | WB14 no longer trusts seeded/published WB12 infiltration on multi-step lanes without the WB18 same-pass lineage marker. |
| WB14 lane-local infiltration tests | PASS | `mofe01_mfredo_clone_wb14_recomputes_multistep_seeded_infiltration` and `mofe01_mfredo_clone_multistep_local_runoff_responds_to_lane_conductivity` pass. |
| Required H smoke | PASS | H1/H6/H9/H11 exited zero under `/tmp/openwepp_mofe01_mfredo_clone_current`; elapsed seconds H1 `212`, H6 `132`, H9 `170`, H11 `95`. |
| Public per-OFE row cardinality | PASS | H1/H6/H9/H11 row counts remain `day_count * contributor_ofe_count`: `10960`, `6576`, `8768`, `4384`. |
| Per-OFE local runoff distinctness | PASS | All-identical active local-runoff days are zero for H1/H6/H9/H11. |
| Full-vector anti-clone | PASS | Full-vector clone days are zero for H1/H6/H9/H11. |
| Genuine per-element identity | PASS | Max residuals: H1 `2.5579538487363607e-13`, H6 `2.0250467969162855e-13`, H9 `1.9895196601282805e-13`, H11 `2.2737367544323206e-13` mm. |
| Transfer and aggregate identities | PASS | Runtime audit reports transfer and aggregate internal-transfer residual maxima at `0.0`. |
| Local semantic comparisons, no comparator subagent | PASS / INVESTIGATION FAIL | Commands exited zero and row keys align for H1/H6/H9/H11 under `/tmp/openwepp_mofe01_mfredo_clone_current/owcmp-smoke/reports`; semantic value pass remains false pending M-F-REDO2. |
| Single-OFE anchor comparison | PASS | `/tmp/openwepp_mofe01_mfredo_clone_single_final`: H8/H15/H19/H20/H22/H23/H28 byte-identical to M-E2 for `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet` (28/28 PASS). |
| `cargo fmt --check` | PASS | Final post-edit run. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Final post-edit run. |
| `cargo test --workspace` | PASS | Full Rust closure loop passed after the stale-token contract test repair. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| `bash tools/release/check_authority_suite_antievasion.sh` | PASS | Authority suite anti-evasion checks passed. |
| `cargo test --test auth11_required_suite_obligation_guards_contract` | PASS | Authority obligation guard tests passed. |
| `git diff --check` | PASS | Final whitespace check passed after evidence updates. |
| `wctl doc-lint --path ...` | NON-SUBSTANTIVE | Wrapper exited zero but reported 0 files validated, so this is recorded but not counted as a substantive markdown gate. |
| Line count governance | PASS / WARN | Touched files remain below 2000 lines; pre-existing global warnings include `coupling.rs` at 3052 lines outside this increment write set. |

Detailed evidence: `m-f-per-ofe-wat-publication-evidence.md`.

## M-F-REDO scoped acceptance gates (superseded by M-F-REDO-CLONE)

Claude review superseded the M-F-REDO anti-clone acceptance reading: M-F-REDO
made active handoff measurable, but local runoff was still cloned on most H1
active runoff days. The controlling runoff distinctness evidence is the
M-F-REDO-CLONE table above.

| Gate/check | Result | Notes |
| --- | --- | --- |
| Static per-OFE lane differentiation | PASS | Multi-OFE lanes now build OFE-local soil/slope/management runtime surfaces instead of cloning the aggregate static runtime surface. |
| Active surface handoff | PASS | Fresh H1/H6/H9/H11 audit under `/tmp/openwepp_mofe01_mfredo_final` reports downstream nonzero `UpStrmQ` rows: H1 `4195`, H6 `2073`, H9 `3144`, H11 `990`. |
| Surface handoff residual with nonzero operands | PASS | `current UpStrmQ == previous QOFE` max residual is `0.0` on active edges for H1/H6/H9/H11. |
| Active lateral handoff | PASS | Downstream nonzero `SubRIn` rows: H1 `341`, H6 `132`, H9 `180`, H11 `79`; `current SubRIn == previous latqcc` max residual is `0.0`. |
| Anti-clone gate | PASS | `active_surface_all_clone_day_count=0` for H1/H6/H9/H11; max distinct `Q` and `SoilWaterTotal` per day match OFE count on each smoke surface. |
| Public per-OFE row cardinality | PASS | H1/H6/H9/H11 row counts remain `day_count * contributor_ofe_count`: `10960`, `6576`, `8768`, `4384`. |
| No `QOFE=Q` alias | FAIL | Candidate `max_abs_qofe_minus_q=0.0` for H1/H6/H9/H11; legacy-clean ladder has max `abs(QOFE-Q)` of `362.13991`, `177.51694`, `185.89531`, and `84.64425` mm. |
| Local semantic comparisons, no comparator subagent | FAIL | Commands exited zero and row keys align, but semantic pass is false for H1/H6/H9/H11; value families still fail for `Q`, `QOFE`, `UpStrmQ`, `SubRIn`, storage, ET, and percolation. |
| Single-OFE anchor comparison | PASS | `/tmp/openwepp_mofe01_mfredo_single_final/single-ofe-anchor-cmp.tsv`: H8/H15/H19/H20/H22/H23/H28 byte-identical to M-E2 for `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet` (28/28 PASS). |
| `cargo fmt --check` | PASS | Final post-edit run. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Final post-edit run. |
| `cargo test --workspace` | PASS | Full Rust closure loop passed after WB12-family fixture reconciliation. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| `git diff --check` | PASS | Final whitespace check passed. |
| `markdown-doc lint` | PASS | Work-package plus touched SC-WATBAL/SC-SYSTEM contracts: 38 files validated, 0 errors, 0 warnings. |
| Line count governance | WARN | `scheduler_seed_and_runtime.rs` is 2122 lines; above the 2000-line warning threshold and below 3000. |

Detailed evidence: `m-f-per-ofe-wat-publication-evidence.md`.

## M-F scoped acceptance gates

| Gate/check | Result | Notes |
| --- | --- | --- |
| Public per-OFE row cardinality | PASS | H1/H6/H9/H11 row counts equal `day_count * contributor_ofe_count`; H1 has 10960 rows for 2192 days * 5 OFEs. |
| Manifest per-OFE publication metadata | PASS | Manifests report `publication_ofe_policy=per-ofe-dynamic-water-balance-state`, `storage_lineage_policy=per-ofe-dynamic-wb-state`, grouped first/last OFE keys, and matching row counts. |
| No `QOFE=Q` alias | SUPERSEDED / FAIL | M-F's apparent non-alias conclusion is superseded by M-F-REDO; candidate H1/H6/H9/H11 now show `max_abs_qofe_minus_q=0.0` once active handoff is fixed. |
| Surface handoff identity | STRUCTURAL PASS / ACCEPTANCE FAIL | `current UpStrmQ == previous QOFE` residual is `0.0`, but only because candidate surface carry is zero. |
| No downstream `UpStrmQ=0` | FAIL | H1/H6/H9/H11 all have `max_upstrmq=0.0` and zero downstream nonzero `UpStrmQ` rows. |
| Lateral handoff identity | PASS | `current SubRIn == previous latqcc` residual is `0.0` with nonzero downstream `SubRIn` rows observed. |
| Local semantic comparisons, no comparator subagent | FAIL | Row keys align, but value families fail: H1 `UpStrmQ` fail count `730`, max diff `342.5`; H6/H9/H11 also fail. |
| Single-OFE anchor | PARTIAL | Single-OFE code path stayed aggregate and focused/full Rust tests pass; substrate byte-identity anchor was not rerun because the multi-OFE surface gate failed. |
| `cargo fmt --check` | PASS | Final post-edit run. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Final post-edit run. |
| `cargo test --workspace` | PASS | Full Rust closure loop passed. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| `git diff --check` | PASS | Final whitespace check passed. |
| Line count governance | WARN | `openwepp-cli-watershed.rs` is 2012 lines and `scheduler_seed_and_runtime.rs` is 2115 lines; both crossed the 2000-line warning threshold but remain below 3000. |

Detailed evidence: `m-f-per-ofe-wat-publication-evidence.md`.

## M-E4-REDO scoped acceptance gates

| Gate/check | Result | Notes |
| --- | --- | --- |
| M-E4-REDO scope boundary | PASS | Rebuilt internal WB13 identity validation only; public WAT publication remains aggregate-only for M-F. |
| Non-tautological per-element identity | PASS | H1/H6/H9/H11 real internal records close with nonzero-at-noise max residuals `1.048e-13` to `1.403e-13` mm under `TOL-WATBAL-007 <= 1e-11 mm`. |
| True adjacent transfer identity | PASS | Runtime audit closes adjacent sent-vs-received transfer residuals; focused negative test mutates downstream received input independently and fails closed. |
| Frost per-OFE fixture | PASS | Focused M-E4-REDO fixture includes frozen-water storage delta in per-OFE closure. |
| SC-WATBAL-001 tolerance pin | PASS | Version 156 adds `TOL-WATBAL-007 <= 1e-11 mm` and the M-E4-REDO internal WB13 identity acceptance addendum. |
| `cargo fmt --check` | PASS | Post-edit run. |
| `cargo test -p openwepp-runner mofe01_me4_redo -- --nocapture` | PASS | 4 focused M-E4-REDO tests passed. |
| `cargo test -p openwepp-runner mofe01 -- --nocapture` | PASS | 12 runner per-OFE tests passed. |
| `cargo test --test mofe01_per_ofe_state_contract -- --nocapture` | PASS | 5 contract-derived tests passed. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Full workspace clippy passed. |
| `cargo test --workspace` | PASS | Full Rust closure loop passed. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| `bash tools/release/check_authority_suite_antievasion.sh` | PASS | Authority suite anti-evasion checks passed. |
| `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` | PASS | 2 authority obligation guard tests passed. |
| `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --path docs/specifications/science-contracts/contracts/SC-WATBAL-001.md --format plain` | PASS | 36 files validated, 0 errors, 0 warnings. |
| Required H smoke | PASS | H1/H6/H9/H11 exited zero under `/tmp/openwepp_mofe01_me4_redo`; elapsed H1 `279.79s`, H6 `174.67s`, H9 `227.05s`, H11 `127.55s`. |
| Local semantic comparisons, no comparator subagent | FAIL | Expected publication-boundary fail: execution completed for H1/H6/H9/H11, but public WAT still emits aggregate rows, so each smoke surface has `semantic_pass_count=0/1`; focus-column max diff remains `0.0`. |
| Single-OFE anchor comparison | PASS | `/tmp/openwepp_mofe01_me4_redo_single_anchors/single-ofe-anchor-cmp.tsv`: H8/H15/H19/H20/H22/H23/H28 byte-identical to M-E2 for `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet` (28/28 PASS). |
| Line count governance | PASS | Touched Rust files remain below thresholds; `scheduler.rs` is 1994 lines and `scheduler_seed_and_runtime.rs` is 1973 lines. |

Detailed evidence: `m-e4-internal-wb13-record-evidence.md`.

## M-E4 scoped acceptance gates

This historical table is retained as the superseded M-E4 evidence that the
Claude blocking review rejected. The controlling closure evidence is the
M-E4-REDO table above.

| Gate/check | Result | Notes |
| --- | --- | --- |
| M-E4 scope boundary | PASS | Internal per-OFE WB13 records are produced from persisted lane state; public WAT publication remains aggregate-only for M-F. |
| Internal per-OFE WB13 records | PASS | Added authoritative internal records and run summary with transfer, per-element, and aggregate internal-transfer cancellation checks. |
| Identity manifest audit | PASS | H1/H6/H9/H11 report record counts equal to `row_count * contributor_ofe_count` and all three residual maxima at `0.0` mm. |
| `cargo fmt --check` | PASS | Post-edit run. |
| `cargo test -p openwepp-runner mofe01_me4 -- --nocapture` | PASS | 3 focused M-E4 tests passed. |
| `cargo test -p openwepp-runner mofe01 -- --nocapture` | PASS | 11 runner per-OFE tests passed. |
| `cargo test --test mofe01_per_ofe_state_contract -- --nocapture` | PASS | 4 contract-derived tests passed. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Full workspace clippy passed. |
| `cargo test --workspace` | PASS | Full Rust closure loop passed. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| `bash tools/release/check_authority_suite_antievasion.sh` | PASS | Authority suite anti-evasion checks passed. |
| `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --format plain` | PASS | 35 files validated, 0 errors, 0 warnings. |
| Required H smoke | PASS | H1/H6/H9/H11 exited zero under `/tmp/openwepp_mofe01_me4_runtime_smoke`; elapsed H1 `281s`, H6 `169s`, H9 `219s`, H11 `122s`. |
| Local owcmp smoke command execution, no comparator subagent | PASS | User explicitly directed comparisons without the comparator subagent because GPT-5.3-Codex-Spark quota was exhausted; H1/H6/H9/H11 returned `execution_verdict=PASS`. |
| Local owcmp smoke semantic comparison | FAIL | Expected publication-boundary fail: each smoke surface remained `semantic_pass_count=0/1`; focus columns all zero diff. |
| Single-OFE anchor comparison | PASS | H8/H15/H19/H20/H22/H23/H28 byte-identical to M-E2 for `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet` (28/28 pass). |
| Line count governance | PASS | Touched Rust files remain below the 2000-line warning threshold; `scheduler.rs` is still a near-threshold watch item at 1994 lines. |

## M-E4 residual future-boundary checks

| Gate/check | Result | Notes |
| --- | --- | --- |
| Public WAT row-cardinality gate | BLOCKED | Remains M-F publication-policy scope. |
| Full H1-H36 replay | NOT RUN | The staged M-E4 gate names targeted identity fixtures; full-cohort replay under the doubled debug path remains M-E6/performance-hardening scope unless M-F requires it. |

Detailed evidence: `m-e4-internal-wb13-record-evidence.md`.

## M-E3 scoped acceptance gates

| Gate/check | Result | Notes |
| --- | --- | --- |
| M-E3 scope boundary | PASS | Multi-OFE runner path now executes persistent OFE-local dynamic state across days; no per-OFE WB13 records or WAT publication flip. |
| Persistent OFE state model | PASS | Added `OfeLanePersistentState` and `OfeLanePersistentStateSequence` with fail-closed cardinality/order replacement checks. |
| Runner daily lifecycle wiring | PASS | Multi-OFE hillslopes initialize persistent lane state, overlay daily climate, seed runtime surfaces, run the sequential OFE executor, and replace persistent state only after sequence success. |
| Publication boundary preservation | PASS | Multi-OFE smoke manifests report `persistent-dynamic-state-shadow`, dynamic flags true, `per_ofe_record_count=0`, and `publication_ofe_policy=single-row-canonicalized-hillslope-aggregate`. |
| `cargo fmt --check` | PASS | Post-edit run. |
| `cargo test -p openwepp-hillslope-orchestrator mofe01_me3 -- --nocapture` | PASS | 3 focused M-E3 tests passed. |
| `cargo test -p openwepp-runner mofe01 -- --nocapture` | PASS | 8 runner per-OFE tests passed. |
| `cargo test --test mofe01_per_ofe_state_contract -- --nocapture` | PASS | 4 contract-derived tests passed. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Final post-doc run. |
| `cargo test --workspace` | PASS | Final post-doc full Rust closure loop. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| `bash tools/release/check_authority_suite_antievasion.sh` | PASS | Authority suite anti-evasion checks passed. |
| `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --format plain` | PASS | 34 files validated, 0 errors, 0 warnings. |
| Required H smoke | PASS | H1/H6/H9/H11 exited zero under `/tmp/openwepp_mofe01_me3_runtime_h1`; elapsed H1 `4:38.90`, H6 `169s`, H9 `218s`, H11 `121s`. |
| Local owcmp smoke command execution, no comparator subagent | PASS | User explicitly directed comparisons without the comparator subagent because GPT-5.3-Codex-Spark quota was exhausted; H1/H6/H9/H11 returned `execution_verdict=PASS`. |
| Local owcmp smoke semantic comparison | FAIL | Expected publication-boundary fail: each smoke surface remained `semantic_pass_count=0/1`; focus columns all zero diff. |
| Single-OFE anchor comparison | PASS | H8/H15/H19/H20/H22/H23/H28 byte-identical to M-E2 for `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet` (28/28 pass). |
| Manifest audit | PASS | `/tmp/openwepp_mofe01_me3_runtime_h1/m-e3-publication-audit.json`: smoke and anchor predicates pass. |
| Line count governance | PASS | Touched Rust files remain below the 2000-line warning threshold. |

## M-E3 residual future-boundary checks

| Gate/check | Result | Notes |
| --- | --- | --- |
| Full H1-H36 replay | NOT RUN | The staged M-E3 gate names H1/H6/H9/H11 smoke execution; full-cohort replay under the new N-lane shadow path is debug-mode expensive and remains M-E6/performance-hardening scope. |
| Per-element identity gate | BLOCKED | Requires internal per-OFE WB13 record production in M-E4. |
| Transfer identity gate | BLOCKED | Requires internal per-OFE daily records in M-E4 to expose authoritative sent/received terms. |
| WAT row-cardinality gate | BLOCKED | Remains M-F publication-policy scope. |

Detailed evidence: `m-e3-dynamic-state-persistence-evidence.md`.

## M-E2 scoped acceptance gates

| Gate/check | Result | Notes |
| --- | --- | --- |
| M-E2 scope boundary | PASS | Added sequential OFE lane executor only; no dynamic per-OFE state persistence, WB13 record production, or WAT publication flip. |
| Sequential OFE lane executor | PASS | `execute_ofe_sequence_with_kernel` runs the existing phase graph once per ordered OFE lane and carries `TransferOutput` downstream as `TransferInput`. |
| Transfer input overlay | PASS | Executor writes explicit `UpStrmQ`, `SubRIn`, `wb12_runon_input`, `wb12_runoff_carryover`, enabled flag, area ratio, and 24-slot upstream surface/lateral arrays before each lane run. |
| Transfer output extraction | PASS | Executor reads `ui_SCrunf_0001..0024` and `ui_LfCrf_0001..0024` after each lane run into `TransferOutput`. |
| Two-OFE synthetic transfer vector | PASS | Focused test proves OFE 2 receives non-zero `UpStrmQ`/`SubRIn` only from OFE 1 transfer arrays, despite stale downstream scalars/arrays seeded before execution. |
| Downstream area-ratio scaling | PASS | Focused test proves OFE 2 receives scaled transfer totals when its upstream area ratio is `2.0`. |
| Stale current output rejection | PASS | Focused test proves stale current output arrays are cleared before extraction and missing fresh output fails closed. |
| Malformed transfer arrays | PASS | Negative current transfer array slot returns typed `OfeLaneSequenceError::InvalidTransferValue`. |
| Overflowed transfer totals | PASS | Finite per-hour slots whose daily total overflows are rejected as typed `InvalidTransferValue`. |
| Non-sequential lane IDs | PASS | First lane `ofe_id=2` returns typed `OfeLaneSequenceError::NonSequentialLaneOfeId`. |
| `cargo fmt --check` | PASS | Final post-format run. |
| `cargo test -p openwepp-hillslope-orchestrator mofe01_me2 -- --nocapture` | PASS | 6 M-E2 focused tests passed. |
| `cargo test -p openwepp-runner mofe01_me1 -- --nocapture` | PASS | M-E1 runner tests remain green. |
| `cargo test --test mofe01_per_ofe_state_contract -- --nocapture` | PASS | Contract-derived per-OFE structural target remains green. |
| `cargo test -p openwepp-hillslope-orchestrator --lib writeback:: -- --nocapture` | PASS | Existing writeback tests plus M-E2 tests passed: 10 total. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Workspace clippy passed. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| `bash tools/release/check_authority_suite_antievasion.sh` | PASS | Authority suite anti-evasion checks passed. |
| `cargo test --workspace` | PASS | Full Rust closure loop passed. |
| `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --format plain` | PASS | 33 files validated, 0 errors, 0 warnings after final M-E2 verification records. |
| `cargo build -p openwepp-runner --bin openwepp-cli-hill` | PASS | Built final hillslope CLI before replay. |
| Final H1-H36 CLI batch | PASS | 36/36 exit code `0`; 36 manifests; 144 output files under `/tmp/openwepp_mofe01_me2_final`. |
| Local owcmp H1-H36 command execution, no comparator subagent | PASS | User explicitly directed comparisons without the comparator subagent because GPT-5.3-Codex-Spark quota was exhausted; `execution_verdict=PASS`. |
| No-publication-flip audit | PASS | 36/36 manifests preserve aggregate publication policy, dynamic per-OFE flags false, `per_ofe_record_count=0`, and static slice count equal to contributor count. |
| Single-OFE anchor comparison | PASS | H8/H15/H19/H20/H22/H23/H28 byte-identical to M-E1 outputs for `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet`. |
| Aggregate identity unchanged | PASS | M-E2 is not CLI-wired; fresh replay stayed green, single-OFE anchors stayed byte-identical, and owcmp focus-column diffs remained zero. |
| Dual review | PASS | Review A and Review B findings were accepted and fixed; see `review_agent_a.md`, `review_agent_b.md`, and `m-e2-sequential-ofe-lane-executor-evidence.md`. |
| Dual verification | PASS | Verification A and Verification B findings were accepted and fixed; see `verification_agent_a.md` and `verification_agent_b.md`. |

## M-E2 residual future-boundary checks

These checks are not M-E2 acceptance gates. They are recorded to preserve
truthful comparison and identity posture before M-E3/M-E4/M-E5.

| Gate/check | Result | Notes |
| --- | --- | --- |
| Local owcmp H1-H36 semantic comparison | FAIL | Expected publication-boundary fail: `semantic_pass_count=0/36`, `structural_row_key_failures=350720`, first divergent H1 key `[1, 1, 2000]`; focus columns all zero diff. |
| Per-element identity gate | BLOCKED | Still not measurable until M-E3/M-E4 persist OFE-local dynamic state and produce authoritative per-OFE daily records. |
| Transfer identity gate | BLOCKED | M-E2 proves same-day executor handoff on synthetic vectors; full runtime transfer identity remains blocked until dynamic per-OFE records exist. |

Detailed evidence: `m-e2-sequential-ofe-lane-executor-evidence.md`.

## M-E1 ran

| Gate/check | Result | Notes |
| --- | --- | --- |
| M-E1 scope boundary | PASS | Implemented data-model shadow state and static lane slices only; no dynamic per-OFE runner state and no WAT publication flip. |
| Per-OFE collection and record model | PASS | Added `PerOfeDailyWaterBalanceRecord` and `PerOfeDailyWaterBalanceCollection`; N=1 aggregate adapter is constrained to single-OFE only. |
| Transfer payload model | PASS | Added `TransferInput`/`TransferOutput`; downstream conversion and collection insertion fail closed on source/recipient mismatch. |
| Static per-OFE lane slices | PASS | Added exact-cardinality slope/soil/management slices and negative topology/geometry tests. |
| Publication-policy manifest gate | PASS | Manifest now reports aggregate publication policy, `static_per_ofe_slice_count`, `per_ofe_record_count=0`, dynamic per-OFE flags false, and identity statuses as shadow-state-only. |
| `cargo fmt --check` | PASS | Final post-edit run. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Final post-review run. |
| `cargo test -p openwepp-runner mofe01_me1 -- --nocapture` | PASS | 7 focused M-E1 tests passed. |
| `cargo test --test mofe01_per_ofe_state_contract -- --nocapture` | PASS | M-E0 structural red gates are green without weakening the target. |
| HPHYS0319/0320 WATBAL version-pin repair tests | PASS | Both authority tests pass after removing stale exact `SC-WATBAL-001` version pins while retaining invariant/addendum checks. |
| `cargo test --workspace` | PASS | Full Rust closure loop restored after M-E1. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| `bash tools/release/check_authority_suite_antievasion.sh` | PASS | Source-level authority-suite anti-evasion guard passed. |
| `cargo build -p openwepp-runner --bin openwepp-cli-hill` | PASS | Built final hillslope CLI before replay. |
| Fresh H1-H36 final CLI batch | PASS | 36/36 exit code `0`; 36 manifests; 144 output files under `/tmp/openwepp_mofe01_me1_final`. |
| Local owcmp H1-H36 command execution, no comparator subagent | PASS | User explicitly directed comparisons without the comparator subagent because GPT-5.3-Codex-Spark quota was exhausted; `execution_verdict=PASS`. |
| Local owcmp H1-H36 semantic comparison | FAIL | Expected M-E1 publication-boundary fail: `semantic_pass_count=0/36`, `structural_row_key_failures=350720`, first divergent H1 key `[1, 1, 2000]`; focus columns all zero diff. |
| No-publication-flip audit | PASS | 36/36 manifests preserve aggregate publication policy, dynamic per-OFE flags false, `per_ofe_record_count=0`, and static slice count equal to contributor count. |
| Single-OFE anchor comparison | PASS | H8/H15/H19/H20/H22/H23/H28 byte-identical to M-C2 for `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet`. |
| Per-element identity gate | BLOCKED | Still not measurable until the runner populates real dynamic per-OFE daily records in later M-E sub-increments. |
| Transfer identity gate | BLOCKED | Same dynamic-state blocker; M-E1 validates the typed payload shape but does not execute sequential OFE handoff. |
| Aggregate identity unchanged | PASS | Single-OFE anchors remain byte-identical; owcmp focus-column diffs remain zero. |
| Dual review | PASS | Review A and Review B findings were accepted and fixed; see `review_agent_a.md`, `review_agent_b.md`, and `m-e1-data-model-shadow-state-evidence.md`. |
| Dual verification | PASS | Final verification records added after review fixes and final gates. |

Detailed evidence: `m-e1-data-model-shadow-state-evidence.md`.

## M-E0 ran

| Gate/check | Result | Notes |
| --- | --- | --- |
| M-E0 production-code boundary | PASS | No production Rust implementation path was edited. M-E0 changed science contracts, test registration, contract tests, and work-package evidence only. |
| `SC-RUNOFFPART-001` amendment | PASS | Version 43 adds `INV-RUNOFFPART-029` and the per-OFE runoff lane-state addendum binding typed `TransferInput`/`TransferOutput`, no aggregate handoff synthesis, and single-OFE anchors. |
| `SC-WATBAL-001` amendment | PASS | Version 155 adds `INV-WATBAL-097` and the per-OFE dynamic water-balance state addendum binding `PerOfeDailyWaterBalanceRecord` semantics and publication-transition limits. |
| `SC-SYSTEM-001` amendment | PASS | Version 79 adds `INV-SYSTEM-030` and the per-OFE dynamic-state publication-policy manifest gate. The stale header version 77 was corrected while adding version 79. |
| Science-contract registry update | PASS | `docs/specifications/science-contracts/index.md` updated to `Last updated: 2026-06-13` and review dates for `SC-RUNOFFPART-001`, `SC-WATBAL-001`, and `SC-SYSTEM-001`. |
| Contract-test target registration | PASS | Added `mofe01_per_ofe_state_contract` in `Cargo.toml`. |
| `cargo test --test mofe01_per_ofe_state_contract mofe01_me0_contract_authority_is_present -- --nocapture` | PASS | Positive M-E0 authority-presence test passed. |
| `cargo test --test mofe01_inter_ofe_route_contract -- --nocapture` | PASS | Adjacent M-B authority smoke test passed after removing the stale fixed-date registry assertion. |
| `cargo test --test mofe01_per_ofe_state_contract -- --nocapture` | FAIL | Expected M-E0 red target: 1 authority test passed; 3 structural red gates failed for missing per-OFE state collection, missing transfer input/output payloads, and missing publication-policy manifest gate. |
| Per-OFE dynamic state implementation | BLOCKED | M-E0 forbids production implementation. M-E1 must introduce the state type/collection or an explicitly contracted equivalent. |
| Per-element identity gate | BLOCKED | Not measurable until OFE-keyed dynamic records exist. |
| Transfer identity gate | BLOCKED | Not measurable until adjacent OFE transfer input/output records exist. |
| Aggregate identity unchanged | PASS | No production code changed; M-E0 only installs contract/test authority. |
| Single-OFE anchor | NOT RUN | No production runtime changed and the intentional red target blocks full runtime closure; M-E1 must rerun single-OFE anchors after state scaffolding. |
| `cargo fmt --check` | PASS | Final post-edit run. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Final post-edit run. Clippy compiles all targets but does not execute the intentional red test. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Docs lint | PASS | Final post-evidence run: `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --path docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md --path docs/specifications/science-contracts/contracts/SC-WATBAL-001.md --path docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md --path docs/specifications/science-contracts/index.md --format plain`: 35 files validated, 0 errors, 0 warnings. |
| Full Rust closure loop | BLOCKED | The package now contains an intentional red M-E0 target. Running `cargo test --workspace` would be expected to fail until M-E1 implements the per-OFE state surface. |
| Mergeable closure | BLOCKED | M-E0 is an executed-hold scaffold with a normally registered failing target; it must not be treated as a green/mergeable closure increment until M-E1 turns the red gates green without weakening them. |
| Comparator/heavy comparison | NOT RUN | M-E0 is contract/test scaffolding and made no runtime-output change. No comparator subagent was used. |
| Anti-evasion guards | NOT RUN | M-E0 did not edit external-authority suite posture, cohort fixture bindings, or required-case bindings. |
| Dual review | PASS | Review A and Review B findings on preclaimed review/verification coverage, red-test strength, registry row assertions, clippy/deny taxonomy, and red-hold mergeability were accepted and fixed or explicitly dispositioned. |
| Dual verification | PASS | Verification A/B completed read-only checks; post-review local gates confirm the strengthened red target, clippy, deny, fmt, focused authority tests, and docs lint results recorded here. |

Detailed evidence: `m-e0-contract-test-scaffold-evidence.md`.

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
