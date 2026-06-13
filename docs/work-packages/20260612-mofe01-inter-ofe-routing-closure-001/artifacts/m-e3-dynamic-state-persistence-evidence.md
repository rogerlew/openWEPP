# M-E3 dynamic state persistence evidence

Status: complete for M-E3 scope; package remains active for M-E4+

Evidence mode: Ran + Static

## Scope

M-E3 persists OFE-local dynamic writeback state across days behind the
sequential OFE executor. It does not publish per-OFE WB13/WAT rows and does not
change the public aggregate publication policy.

Code changes:

- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
  - Added `OfeLanePersistentState` and `OfeLanePersistentStateSequence`.
  - Added fail-closed persistent-state lane-count/order replacement checks.
  - Added `execute_persistent_ofe_sequence_day_with_kernel` for direct
    persistent-sequence execution.
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
  - Initializes persistent lane state for multi-OFE hillslopes.
  - Executes the persistent OFE sequence each day after daily climate surface
    construction and before aggregate publication execution.
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`
  - Builds prepared per-OFE lane inputs from persistent state, overlays current
    climate, seeds WB11/calendar/PL runtime surfaces, runs the sequential
    executor, and replaces persistent state only after full sequence success.
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_publication.rs`
  - Marks multi-OFE manifests as
    `per_ofe_state_policy=persistent-dynamic-state-shadow` with dynamic state
    flags true, while preserving `per_ofe_record_count=0` and
    `publication_ofe_policy=single-row-canonicalized-hillslope-aggregate`.

## Gates

| Gate/check | Result | Notes |
| --- | --- | --- |
| Focused persistent-state tests | PASS | `cargo test -p openwepp-hillslope-orchestrator mofe01_me3 -- --nocapture`: 3/3 passed. Tests cover two-day persistence, no lane bleed, failure rollback, and non-sequential initial-state rejection. |
| Runner per-OFE tests | PASS | `cargo test -p openwepp-runner mofe01 -- --nocapture`: 8/8 passed. |
| Contract-derived per-OFE state tests | PASS | `cargo test --test mofe01_per_ofe_state_contract -- --nocapture`: 4/4 passed. |
| `cargo fmt --check` | PASS | Final post-doc run. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Final post-doc run. |
| `cargo test --workspace` | PASS | Final post-doc full Rust closure loop. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| `bash tools/release/check_authority_suite_antievasion.sh` | PASS | Authority suite anti-evasion checks passed. |
| Work-package docs lint | PASS | `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --format plain`: 34 files validated, 0 errors, 0 warnings. |
| M-E3 required H smoke | PASS | H1/H6/H9/H11 exited zero under `/tmp/openwepp_mofe01_me3_runtime_h1`; elapsed times: H1 `4:38.90`, H6 `169s`, H9 `218s`, H11 `121s`. |
| Manifest state-policy audit | PASS | `/tmp/openwepp_mofe01_me3_runtime_h1/m-e3-publication-audit.json`: 4/4 smoke manifests report persistent dynamic shadow state and unchanged aggregate publication; 7/7 single-OFE anchor manifests keep static shadow state. |
| Single-OFE anchor comparison | PASS | H8/H15/H19/H20/H22/H23/H28 are byte-identical to M-E2 outputs for `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet` (28/28 pass). |
| Local owcmp command execution, no comparator subagent | PASS | H1/H6/H9/H11 all returned `execution_verdict=PASS`. |
| Local owcmp semantic comparison | FAIL | Expected publication-boundary failure: each smoke surface remained `semantic_pass_count=0/1` because public WAT remains single-row aggregate. First divergent key is `[1, 1, 2000]` for each H surface; focus columns have zero failures and max diff `0.0`. |
| Line count governance | PASS | Touched Rust files remain below the 2000-line warning threshold: scheduler `1994`, runner seed/runtime `1961`, runner intake `1508`, scheduler publication `344`. |

## Comparison details

Local comparisons were run directly with `tools/owcmp/owcmp`; no comparator
subagent was used per operator instruction.

| H | Execution verdict | Semantic verdict | Structural row-key failures | Focus-column max diff |
| ---: | --- | --- | ---: | ---: |
| 1 | PASS | FAIL | 13152 | 0.0 |
| 6 | PASS | FAIL | 8768 | 0.0 |
| 9 | PASS | FAIL | 10960 | 0.0 |
| 11 | PASS | FAIL | 6576 | 0.0 |

The semantic failure is still the known aggregate-publication boundary: M-E3
persists dynamic per-OFE state but does not yet produce authoritative per-OFE
WB13 records or flip public WAT publication.

## Residuals

- Full H1-H36 replay was not rerun after runner wiring. The staged M-E3 gate
  requires H1/H6/H9/H11 smoke execution; those passed. Full-cohort replay under
  the new N-lane shadow path is debug-mode expensive and belongs with M-E6 or
  a dedicated performance optimization.
- Per-element and transfer identities remain blocked until M-E4 produces
  internal per-OFE WB13 records.
- Public WAT row cardinality remains unchanged until M-E5.
- The dynamic state is a full writeback-surface instance per OFE, so WB storage,
  frost, snow, and profile symbols persist independently by lane. Named
  state-family fixtures, especially frost, remain useful strengthening work for
  M-E4/M-E6.

## Claude review addendum (2026-06-13) — endorsed; 3 tracked items pinned to M-E4/M-E5

Evidence mode: Ran (wiring + test + line-count inspection).

**Endorsed — and the single-pass approach is better than my M-D sub-split
recommendation, not worse.** I flagged M-E3 for per-stateful-family
sub-splitting on the assumption per-OFE would need per-process surgery.
Codex instead instances the **whole writeback surface per OFE lane**
(`OfeLanePersistentState` = a full `runtime_surface` clone per lane), so
WB storage, frost, snow, and profile all go per-OFE wholesale without
threading each process individually. Cleaner; my concern is resolved by
construction. The M-D-required behavioral gate landed (3/3 `mofe01_me3`:
two-day persistence, no lane bleed, failure rollback, nonsequential
rejection) — the real gate, not just "4 hillslopes exit 0".

**Verified architecture:** multi-OFE days now run BOTH
`execute_persistent_scheduler_kernel_lifecycle` (per-OFE shadow, real
per-OFE physics, drives persistent state) AND
`execute_scheduler_kernel_lifecycle` (aggregate, drives published WAT) —
`00_runner_intake_and_lane_setup.rs:1192` + `:~1227`. Single-OFE runs only
the aggregate path (N=1 → `None`), hence byte-identical anchors. The per-OFE
state is genuine, not the M-C2 fake-shadow; it just doesn't publish yet.

Three tracked items (none blocks M-E4; all pinned into the staged plan):

1. **Transitional double-execution must be retired at M-E5.** Running both
   lifecycles per multi-OFE day is a ~2× cost (H1 5-OFE took 4:39) and is
   why full-cohort replay was skipped. Legitimate transitionally, but the
   aggregate path MUST be retired when M-E5 flips publication to per-OFE
   records — otherwise the doubling is permanent. Now an explicit M-E5
   obligation.
2. **Frost-per-OFE identity fixture is a named M-E4 deliverable** (not the
   evidence's vague "M-E4/M-E6"). Frost state is now structurally per-OFE;
   M-E4 closes identities, so that is where a named fixture must prove the
   FDHP01 frost closure re-instances per OFE without perturbing the
   single-OFE frost anchor.
3. **`scheduler.rs` is at 1994 lines — 6 under the 2000 WARN.** M-E4/M-E5
   will cross it; a line-count-governance split of `scheduler.rs` is due
   in the next increment that touches it.

Full-cohort replay deferral is legitimate (M-D's M-E3 gate was the
H1/H6/H9/H11 smoke set, which passed) — but 25/29 multi-OFE hillslopes now
run an unexercised doubled path; watch item for M-E5/M-E6.
