# PERFOPT01 — Runtime-Surface Map-Churn & Writeback-Validation Optimization

Status: complete 2026-06-16 (implementation + command gates passed; Codex-executed,
independently reviewed by Claude Code — the dual-review caveat is resolved:
behavior-preservation proven for all inputs + bit-identity confirmed
`anchor_mismatches = 0` against an independent pre-opt baseline. See
`artifacts/review-claude-independent.md`.)

Package type: **Behavior-preserving optimization** (mechanical-refactor shape —
production Rust change for performance only; **no** physics, contract, output, or
fail-closed-behavior change).

## Objective

Cut the per-OFE-day cost that PERFHO01 attributed (GDB-sampled `11/15`) as the
dominant driver of openWEPP's ~80–110× single-hillslope wall-clock gap vs legacy:
repeated success-path symbol-keyed `BTreeMap<BoundarySymbol, BoundaryValue>`
runtime-surface clone/insert/remove/lookup, plus success-path kernel-writeback
validation detail construction. Deliver a **measured speedup** while preserving
**bit-identical outputs and within-config determinism**.

PERFHO01 scoped the gain: ~1.5–2.5× expected, 3.75× Amdahl cap on the named
component — the **first necessary optimization, not full closure** of the gap.
A second profiling round (PERFHO02) is the expected successor.

## Rationale (PERFHO01, Ran)

- H2637 (19 OFE × 34 yr) `978.55 s` vs legacy `~10 s`; CPU-bound
  (`977.99/978.55` user s) — not I/O or parquet.
- Scaling roughly linear-to-modestly-superlinear (`b≈1.12`): a large **constant
  per-OFE-day cost**, amplified by 19 OFEs × 12,419 days.
- Named hot path (PERFHO01 `perf-profile-evidence.md`):
  `execute_persistent_scheduler_kernel_lifecycle` + the `BoundarySymbol`-keyed
  runtime-surface map churn; `apply_kernel_writeback` / `collect_field_violations`
  / `evaluate_kernel_writeback` success-path sort/alloc/detail; and repeated
  symbol-string formatting in hydrology helpers (`require_state_scalar_for_symbol`,
  `hourly_symbol`, `compute_active_frost_coupling`, `require_shadow_fine_state_domains`).

## Behavior-preservation contract (hard invariants)

This package **must preserve, exactly**:
- **Bit-identical outputs** for the same target + inputs + seed: HBP shard
  byte-identical; `.wat.parquet` / `.pass.parquet` / `.plot` / `.loss.json`
  value/row-identical vs a pre-optimization baseline captured at M1.
- **Determinism** per `docs/numerics/`: within-config bit-reproducibility
  (single thread, pinned seed); **no reordering of floating-point reductions** or
  per-OFE transfer sequencing; FMA unchanged.
- **All conservation gates**: per-element + transfer + hillslope-total WB13
  identities still close at their tolerances (the run still exits 0 on H2637).
- **Fail-closed behavior**: writeback validation and typed guards unchanged;
  validation **detail** may be made lazy only if the **same** error ID + message
  is still emitted on the failure path (verify with a fault-injection or an
  existing failing-input test).
- **Public surfaces / contract authority**: no `SC-*` change; no kernel physics
  or branch-arity change; runtime-surface public types stay compatible.

## Included scope

- Performance-only edits to the runtime-surface map lifecycle / access, the
  kernel-writeback success path, and success-path symbol formatting in the named
  hydrology helpers. Codex chooses the mechanism — PERFHO01 suggested a
  **deterministic stable-index representation** or **reusable per-lane scratch
  state** to replace per-day clone/insert/remove, and **lazy** validation-detail
  construction. Approach and exact write-set are Codex's to finalize at M1.
- Baseline capture + bit-identity / determinism / conservation verification.

## Excluded scope / escalation boundary

- No physics, formula, threshold, contract, or output-schema change.
- No change to per-OFE sequencing or FP reduction order.
- **Escalate (stop + branch), do not proceed,** if the speedup requires changing
  the `BoundarySymbol`/`BoundaryValue` runtime-surface **contract semantics** or a
  fail-closed guard's behavior — that is beyond a behavior-preserving
  optimization and needs its own authority decision.
- The residual gap beyond the named component → `PERFHO02` (next characterization),
  not this package.

## Intended write set

- Production: the runtime-surface / scheduler-lifecycle and kernel-writeback
  paths named above (primarily `crates/openwepp-runner/src/hillslope/scheduler_trace/`
  and `crates/openwepp-kernel-contract/`), plus the named hydrology support
  helpers — exact files declared by Codex at M1.
- `docs/work-packages/20260616-perfopt01-runtime-surface-map-churn-001/**`.

## Validation plan

1. **M1 baseline** — capture pre-optimization H2637 + 1–5-OFE ladder outputs
   (HBP/wat/pass/loss) and timings; declare the exact write-set + current
   line-counts of touched `.rs` files.
2. **M2 optimize** — apply hot-path edits only; preserve FP reduction order and
   per-OFE sequencing.
3. **M3 bit-identity gate (HARD STOP)** — re-run the same fixtures with the
   optimized binary; compare HBP byte-identity + parquet row/value-identity vs the
   M1 baseline (`anchor_mismatches = 0`, the MOFE01/WSHED01/FARPOINT01 anchor
   pattern). Any divergence is a blocker — record location, do not mark complete.
4. **M4 determinism** — run the same target twice with a pinned seed; outputs
   byte-identical.
5. **M5 conservation** — H2637 both `wepp_ui` variants exit 0 (the WB13 gates
   close); fail-closed detail equivalence verified on a failing input.
6. **M6 closure gates** — `cargo fmt --check`; `cargo clippy --workspace
   --all-targets -- -D warnings`; `cargo test --workspace`; `cargo deny check`.
   Line-count governance dispositioned (2000 WARN / 3000 refactor).

## Acceptance criteria

- **Measured speedup** on H2637 (before/after wall-clock, same binary build flags)
  — report the factor; ≥ a meaningful fraction of the PERFHO01 ~1.5–2.5× target.
- **Bit-identical outputs** (M3) — `anchor_mismatches = 0` vs the M1 baseline on
  H2637 + the ladder. This is the primary correctness gate and is non-waivable.
- Determinism (M4), conservation closure (M5), and all four closure gates (M6)
  green, with command-level `Ran:` evidence.
- A profiler re-check (PERFHO01's GDB method or better) showing the named hot path
  shrank.

## Deliverables

- `artifacts/perfopt01-before-after-profiling-evidence.md`
- `artifacts/perfopt01-bit-identity-and-determinism-evidence.md` (the
  anchor-comparison + pinned-seed evidence)
- `artifacts/perfopt01-line-count-governance-checklist.md`
- `artifacts/perfopt01-gate-results.md`
- `artifacts/perfopt01_disposition.md` + `artifacts/perfopt01-worker-handoff.md`
  (naming `PERFHO02` if a residual gap remains)
- dual review + verification artifacts per the work-package convention.

## Dependencies

- PERFHO01 (`20260616-perf-high-ofe-hillslope-characterization-001/`) — the
  attribution, named hot paths, scaling curve, fixture runfiles.
- `docs/numerics/README.md` (determinism — the hard constraint); ADR-0003
  (semantic-parity / within-config bit-reproducibility).
- `AGENTS.md`, `docs/codex_exec_plans.md`,
  `docs/standards/mechanical-refactor-authoring-guide.md`,
  `docs/standards/rust-scientific-coding-standard.md` (line-count governance).
- H2637 fixture + the arboreal-dendrite 1–5-OFE ladder.

## Autonomy

Execute end-to-end (baseline → optimize → bit-identity/determinism/conservation
gates → closure gates → disposition) without asking for direction on intermediate
steps. The bit-identity gate (M3) and the escalation boundary are the hard stops:
if outputs diverge or the speedup requires a semantic/contract change, stop and
record the blocker rather than relaxing a gate.
