# PERFIDX03B - Indexed Kernel Seam / Export-Cache Blocker Closure

Status: complete 2026-06-17 (operator-directed follow-on to held PERFIDX03)

Package type: **Behavior-preserving performance blocker closure**. This package
is the required blocker closure before `PERFIDX04-hot-symbol-id-tables-001`.

## Objective

Close the PERFIDX03 authority-flip blocker: the attempted indexed-authority path
cloned the sparse surface, then paid a full `BTreeMap` export at the kernel seam
on each lane/day. That regressed OFE5 from a baseline mean of `27.01s` to an
active-flip mean of `38.34s`.

Implement an indexed-authority runtime path that avoids per-lane/day full map
export while preserving:

- `BoundarySymbol` compatibility at external/logical seams.
- Kernel writeback payload shape.
- Sorted-symbol deterministic effects.
- Bit-identical/logically identical output surfaces.

If the seam cannot be closed without widening public API or changing writeback
payload shape, stop in `HOLD` and record the exact blocker.

## Scope

In scope:

- `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- Focused tests and package artifacts.

Out of scope:

- No `SC-*` contract changes.
- No irrigation activation or sidecar wiring changes.
- No writeback payload shape change unless a new explicit decision/package
  authorizes it.
- No Stage 4 hot-symbol-id table migration except for the minimum seam support
  needed to prove the authority flip no longer regresses.

## Required Approach

1. Start from the clean PERFIDX02-complete tree plus docs-only PERFIDX03 record.
2. Inspect the current kernel execution seam and indexed surface API.
3. Choose the smallest behavior-preserving seam closure:
   - Prefer lending kernel-readable logical maps from a cached export that is
     invalidated only when indexed authority changes, or
   - Add an indexed read seam only if it can be kept internal and writeback
     payloads remain unchanged.
4. Re-enable indexed authority only after avoiding the repeated full export cost.
5. Prove no OFE5 regression against baseline/no-flip before running expensive H2637.

## Acceptance Criteria

- Active indexed authority has no OFE5 regression against baseline/no-flip.
  Use the same `/tmp/perfho01/run-dirs/ofe5` timing setup from PERFIDX03.
- Same-run-name OFE5 outputs match:
  - hash-stable outputs byte-for-byte (`H1.hbp`, loss JSON, wat parquet, plot parquet);
  - `H1.pass.parquet` logical rows compare equal if container bytes churn.
- Full PERFIDX03 anchor then passes:
  - H2637 both `wepp_ui` variants, if available within validation budget;
  - OFE1-OFE5 ladder or documented available equivalent.
- Determinism preserved.
- Rust gates pass:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
- `git diff --check` and line-count governance recorded.

## Deliverables

- `artifacts/perfidx03b-seam-design.md`
- `artifacts/perfidx03b-ofe5-speed-and-identity.md`
- `artifacts/perfidx03b-anchor-evidence.md`
- `artifacts/perfidx03b-gate-results.md`
- `artifacts/perfidx03b-line-count-governance.md`
- `artifacts/perfidx03b-review-a.md`
- `artifacts/perfidx03b-review-b.md`
- `artifacts/perfidx03b-verification-a.md`
- `artifacts/perfidx03b-verification-b.md`
- `artifacts/perfidx03b-worker-handoff.md`
- `artifacts/perfidx03b_disposition.md`

## Dependencies

- `docs/decisions/0022-indexed-runtime-surface-representation.md`
- `docs/work-packages/20260616-perfidx03-indexed-surface-authority-001/artifacts/perfidx03_disposition.md`
- `docs/work-packages/20260616-perfidx03-indexed-surface-authority-001/artifacts/perfidx03-worker-handoff.md`
- `docs/work-packages/20260616-perfidx03-indexed-surface-authority-001/artifacts/perfidx03-realized-speedup.md`
- `docs/work-packages/20260616-perfidx02-indexed-shadow-runtime-surface-001/artifacts/perfidx02-clone-economics-measurement.md`
- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `crates/AGENTS.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`

## Subagent Requirement

None for this execution. The available subagent tool requires explicit user
authorization for delegation, and the operator request did not ask for subagents.
Run closure gates locally and record command evidence.

## Autonomy

Execute end-to-end through implementation, measurement, gates, review,
verification, line-count governance, and disposition. Stop only on a declared
hard blocker and record the first actionable follow-on.
