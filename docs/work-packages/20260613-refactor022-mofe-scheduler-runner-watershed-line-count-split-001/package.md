# REFACTOR022 — MOFE Scheduler/Runner/Watershed Line-Count Split

Status: queued (follow-on from MOFE01; line-count governance; operator-directed 2026-06-13)

Package type: mechanical refactor (behavior-preserving)

## Objective

Split the three `.rs` files that crossed the 2000-line WARN threshold during
the MOFE01 build, before any of them approach the 3000-line refactor-required
bound. Mechanical, behavior-preserving modularization per
`docs/standards/mechanical-refactor-authoring-guide.md`.

## Files (line counts at MOFE01 M-H, re-measure at kickoff)

- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` (~1994+ at M-E3;
  grew through M-F/M-G/M-H with the per-OFE types/executor/identity).
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`
  (crossed 2000 at M-F).
- `crates/openwepp-cli-watershed/src/.../openwepp-cli-watershed.rs` (crossed
  2000 at M-F per the M-F line-count note).

## Included scope

- Behavior-preserving module extraction (per-OFE state types, the lane
  executor, the per-OFE WB13 identity, runtime seed helpers, watershed CLI
  sections) into bounded submodules.
- Reconcile tests mechanically only; hide no semantic change in the refactor.

## Excluded scope / protected boundaries

- No behavior change — bit-identical outputs (multi-OFE WAT + single-OFE
  anchors) before/after.
- No contract or physics edits.

## Acceptance / exit criteria

- Each target file under the 2000 WARN (or a justified bounded exception).
- Required Rust closure loop: `cargo fmt --check`, `cargo clippy --workspace
  --all-targets -- -D warnings`, `cargo test --workspace`, `cargo deny check`.
- Bit-identical MOFE cohort + single-OFE anchor outputs (behavior preservation).

## Dependencies

- MOFE01 (and ideally after M-I, so the final scheduler.rs shape is refactored
  once).
- `docs/standards/mechanical-refactor-authoring-guide.md`,
  `docs/work-packages/AGENTS.md` line-count governance.
