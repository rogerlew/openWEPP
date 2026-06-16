# PERFOPT01 Kickoff — Runtime-Surface Map-Churn & Writeback Optimization

Scope: local repository production-Rust optimization; **behavior-preserving**;
determinism + bit-identity preservation required.

Execution mode: package-end-to-end (behavior-preserving optimization).

Autonomy + completion: execute through disposition-ready state without asking for
direction on intermediate steps. The package is incomplete until the bit-identity
gate, determinism check, conservation gates, and the four closure gates are run
and recorded, and required artifacts are written. The bit-identity gate is a
**hard stop**.

## Objective

Reduce the per-OFE-day wall-clock cost PERFHO01 attributed (`11/15` GDB samples)
as the dominant driver of the ~80–110× H2637 gap — success-path symbol-keyed
`BTreeMap<BoundarySymbol, BoundaryValue>` runtime-surface churn + kernel-writeback
validation detail — while preserving **bit-identical outputs** and determinism.
**No physics, contract, output-schema, or fail-closed-behavior change.** Approach
is yours (PERFHO01 suggested a deterministic stable-index representation or
reusable per-lane scratch state, and lazy validation detail).

## Steps (in order)

1. **Baseline (M1).** Run the **current** `cli-hill` on H2637 (both `wepp_ui`
   variants) and the 1–5-OFE ladder; save HBP/`.wat.parquet`/`.pass.parquet`/
   `.loss.json` + timings as the pre-optimization anchor. Declare the exact
   write-set + current line-counts of touched `.rs` files. Reuse PERFHO01 runfiles
   (`../20260616-perf-high-ofe-hillslope-characterization-001/artifacts/runfiles/`).
2. **Optimize (M2).** Hot-path edits only. **Do not** reorder floating-point
   reductions or per-OFE transfer sequencing. Keep writeback validation and typed
   guards; make validation **detail** lazy only if the same error ID + message is
   still emitted on the failure path.
3. **Bit-identity gate (M3 — HARD STOP).** Re-run the same fixtures with the
   optimized binary; compare against the M1 anchor: HBP byte-identical, parquet
   row/value-identical (`anchor_mismatches = 0`, the MOFE01/WSHED01/FARPOINT01
   anchor pattern — e.g. duckdb row compare). **Any divergence is a blocker** —
   record where, do not proceed to disposition.
4. **Determinism (M4).** Same target + pinned seed, twice → byte-identical.
5. **Conservation (M5).** H2637 both variants exit 0 (WB13 gates close); verify
   fail-closed detail equivalence on a failing input.
6. **Closure gates (M6, mandatory):** `cargo fmt --check`;
   `cargo clippy --workspace --all-targets -- -D warnings`;
   `cargo test --workspace`; `cargo deny check`. Disposition line-count governance
   (2000 WARN / 3000 refactor).
7. **Evidence + disposition.** Before/after timing + profiler re-check (named hot
   path shrank); gate results with exit codes; disposition + handoff (name
   `PERFHO02` if a residual gap remains).

## Hard constraints

- Bit-identical outputs (M3) is the primary correctness gate — **non-waivable**.
- Determinism per `docs/numerics/` — no FP-reduction reordering, no sequencing
  change, pinned-seed reproducibility.
- **Escalate (stop + branch)** if the speedup requires changing
  `BoundarySymbol`/`BoundaryValue` runtime-surface **contract semantics** or a
  fail-closed guard's behavior — that is beyond this package.
- Truthfulness: determinism/bit-identity are empirical — label `Ran:`, not
  `Static:`. If a gate is not executed or diverges, record the blocker; do not
  mark disposition-ready.

## Required reading

- `docs/work-packages/20260616-perfopt01-runtime-surface-map-churn-001/package.md`
- `AGENTS.md`, `docs/codex_exec_plans.md`
- `docs/numerics/README.md` (determinism — hard constraint)
- `docs/standards/mechanical-refactor-authoring-guide.md`,
  `docs/standards/rust-scientific-coding-standard.md` (line-count governance)
- PERFHO01 `artifacts/perf-profile-evidence.md` + `perfho01-verdict.md` (the
  named hot paths + the gain bound)
- The hot-path code: `crates/openwepp-runner/src/hillslope/scheduler_trace/`
  (`execute_persistent_scheduler_kernel_lifecycle`),
  `crates/openwepp-kernel-contract/` writeback, and the named hydrology helpers.
