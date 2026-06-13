# M-E2 sequential OFE lane executor evidence

Status: complete for M-E2 scope; package remains active for M-E3+

Evidence mode: Ran + Static

## Scope

M-E2 adds the same-day sequential OFE lane executor around the existing
hillslope phase graph. It is an executor/wiring increment only:

- explicit `TransferInput` is overlaid onto the current lane before scheduler
  execution,
- explicit `TransferOutput` is extracted from current-lane 24-slot transfer
  arrays after scheduler execution,
- nonterminal lane output becomes the next lane's input through
  `TransferOutput::as_downstream_input()` with downstream area-ratio scaling,
- malformed transfer arrays fail closed,
- dynamic per-OFE state persistence, per-OFE WB13 record production, and WAT
  publication remain later M-E scope.

## Static implementation

Rust code changed:

- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
  - Added `OfeLaneExecutionInput`, `OfeLaneExecutionReport`,
    `OfeLaneSequenceExecutionReport`, and `OfeLaneSequenceError`.
  - Added `HillslopePhaseScheduler::execute_ofe_sequence_with_kernel`.
  - Added transfer overlay helpers that write `UpStrmQ`, `SubRIn`,
    `wb12_runon_input`, `wb12_runoff_carryover`,
    `mofe_hourly_carry_arrays_enabled`, `mofe_hourly_upstream_area_ratio`,
    `ui_SUrunf_0001..0024`, and `ui_LfUrf_0001..0024` from
    `TransferInput`.
  - Added transfer extraction helpers that read
    `ui_SCrunf_0001..0024` and `ui_LfCrf_0001..0024` into
    `TransferOutput`.
  - Clears stale current-lane transfer output arrays before each lane run so
    missing fresh output cannot be accepted as current evidence.
  - Added fail-closed validation for lane order, source/recipient identity,
    non-finite or negative transfer slots, non-finite/overflowed daily totals,
    positive area ratio, and daily scalar/array sum mismatch.
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
  - Re-exported the new sequence input/report/error types.
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/writeback.rs`
  - Added focused M-E2 synthetic executor tests.

No runner CLI path, dynamic persistence path, WB13/WAT publication path,
science contract, dependency, legacy, or `/wc1` substrate file was edited.

## Scoped Validation

| Command/check | Result | Evidence |
| --- | --- | --- |
| `cargo fmt --check` | PASS | Final post-format gate. |
| `cargo test -p openwepp-hillslope-orchestrator mofe01_me2 -- --nocapture` | PASS | 6 M-E2 tests passed. |
| `cargo test -p openwepp-runner mofe01_me1 -- --nocapture` | PASS | M-E1 runner tests remain green. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Workspace clippy passed. |
| `cargo test --test mofe01_per_ofe_state_contract -- --nocapture` | PASS | Contract-derived per-OFE structural target remains green. |
| `cargo test -p openwepp-hillslope-orchestrator --lib writeback:: -- --nocapture` | PASS | Existing writeback tests plus M-E2 tests passed. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| `bash tools/release/check_authority_suite_antievasion.sh` | PASS | Authority suite anti-evasion checks passed. |
| `cargo test --workspace` | PASS | Full Rust closure loop passed. |
| `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --format plain` | PASS | 33 files validated, 0 errors, 0 warnings after final M-E2 verification records. |
| `cargo build -p openwepp-runner --bin openwepp-cli-hill` | PASS | Built final hillslope CLI before replay. |
| Fresh H1-H36 final CLI batch | PASS | `/tmp/openwepp_mofe01_me2_final/exit-codes.tsv`: 36/36 exit code `0`; 36 manifests; 144 output files. |
| Local `owcmp` H1-H36 command execution, no comparator subagent | PASS | `/tmp/openwepp_mofe01_me2_final/owcmp/summary.json`: `execution_verdict=PASS`. |
| No-publication-flip manifest audit | PASS | `/tmp/openwepp_mofe01_me2_final/m-e2-publication-audit.json`: 36/36 manifests preserve aggregate publication, dynamic per-OFE flags false, `per_ofe_record_count=0`, and static slice count equal to contributor count. |
| Single-OFE anchor comparison | PASS | `/tmp/openwepp_mofe01_me2_final/single-ofe-anchor-cmp.tsv`: H8/H15/H19/H20/H22/H23/H28 byte-identical to M-E1 for `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet` (28/28 PASS). |

## Residual Future-Boundary Checks

These checks are retained as truthfulness evidence. They are not M-E2 scoped
acceptance gates because M-E2 deliberately does not flip public WAT
publication or persist dynamic per-OFE records.

| Command/check | Result | Evidence |
| --- | --- | --- |
| Local `owcmp` H1-H36 semantic comparison | FAIL | Expected publication-boundary fail: `semantic_pass_count=0/36`, `structural_row_key_failures=350720`, first divergent H1 key `[1, 1, 2000]`; focus columns all zero diff. |
| Per-element identity gate | BLOCKED | Not measurable until later increments persist OFE-local dynamic daily records. |
| Runtime transfer identity gate | BLOCKED | Synthetic executor handoff is proven; full runtime sent/received identity remains blocked until dynamic per-OFE records exist. |

## Focused test coverage

`mofe01_me2_sequential_executor_carries_first_ofe_arrays_to_second_lane`:

- runs two OFE lanes through the existing scheduler phase graph,
- seeds stale downstream `UpStrmQ`, `SubRIn`, `ui_SUrunf_*`, and
  `ui_LfUrf_*` values before lane 2,
- proves lane 2 observes `UpStrmQ=0.75`, `SubRIn=0.75`,
  `ui_SUrunf_0001=0.25`, and `ui_LfUrf_0004=0.75` from OFE 1's current
  transfer arrays, not from stale lane-2 state,
- proves stale OFE 1 current output arrays are cleared and replaced by fresh
  kernel-published current arrays,
- proves lane 1 emits a nonterminal output to OFE 2 and lane 2 emits a
  terminal output.

`mofe01_me2_sequential_executor_applies_downstream_area_ratio`:

- sets lane 2 upstream area ratio to `2.0`,
- proves OFE 2 receives scaled `UpStrmQ=0.50` and `SubRIn=1.0` from OFE 1's
  current arrays.

`mofe01_me2_sequential_executor_rejects_stale_current_output_arrays`:

- seeds stale OFE 1 current output arrays but uses a kernel that publishes no
  fresh current arrays,
- proves the executor clears stale arrays and fails closed on missing fresh
  output.

`mofe01_me2_sequential_executor_rejects_malformed_transfer_arrays`:

- writes `ui_SCrunf_0002=-0.10` in the current-lane output array,
- proves the executor returns `OfeLaneSequenceError::InvalidTransferValue`.

`mofe01_me2_sequential_executor_rejects_transfer_total_overflow`:

- writes finite per-hour transfer slots whose daily sum overflows,
- proves the executor rejects the non-finite daily total before handoff.

`mofe01_me2_sequential_executor_rejects_nonsequential_lane_ids`:

- passes a first lane with `ofe_id=2`,
- proves the executor returns `OfeLaneSequenceError::NonSequentialLaneOfeId`.

## Gate disposition

M-E2 proves the sequential executor and same-day transfer overlay/extraction
surface on synthetic two-OFE vectors. It does not claim dynamic-state
persistence, per-OFE WB13 records, per-element identity closure, or public WAT
row publication. Those gates remain blocked by the declared M-E3/M-E4/M-E5
sequence.

## Claude review addendum (2026-06-13) — endorsed; honest gate split

Evidence mode: Ran (wiring/test/gate inspection).

- **Executor is correctly shadow.** Referenced only in tests
  (`tests/tests_mod/writeback.rs`, `tests03/per_ofe_state.rs`), NOT wired
  into the runner daily loop — so production still uses the aggregate path,
  single-OFE anchors stay byte-identical (28/28), and multi-OFE still
  publishes aggregate. Right staged state.
- **Tests are behavioral, not structural** (the E1→E2 strengthening):
  arrays carry OFE1→OFE2, downstream area-ratio applied, and four
  fail-closed rejection paths (stale output, malformed arrays, transfer
  overflow, nonsequential lane ids). This is M-E2's actual M-D gate
  ("OFE 2 receives nonzero UpStrmQ/SubRIn only from OFE 1; malformed
  hard-fails") — PASS.
- **The gate split is honest.** M-E2 scoped acceptance gates (executor,
  behavioral tests, anchor, no-flip) all PASS; the per-element and transfer
  identity gates are classified BLOCKED under "residual future-boundary
  checks — NOT M-E2 acceptance gates," with correct rationale (not
  measurable until M-E3/M-E4 persist OFE-local dynamic state). This matches
  the M-D breakdown (identities close at M-E4) and does **not** punt an
  M-E2-required gate. The verifier-flagged governance gap was exactly this
  classification, correctly fixed — the non-deferral rule working as
  intended (verifiers checking gate legitimacy).

Endorsed. Next is M-E3 (per-OFE dynamic state persistence) — per the M-D
review flag, the largest increment, where the whole hillslope state model
incl. the FDHP01 frost state machine goes per-OFE; recommend it sub-split
per stateful family with a named frost-per-OFE fixture.
