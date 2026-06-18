# REFACTOR022 Kickoff — Monolith Line-Count Split (behavior-preserving)

Execution mode: behavior-preserving mechanical refactor — bit-identity is the gate.

Autonomy: execute end-to-end (split plan → per-file split + bit-identity + gates → line-count
governance → dual review → disposition). **Stop on any bit-identity divergence.** Leaving a
file as a documented WARN because it has no coherent seam is a valid outcome.

## What this is (and isn't)

Split the monolith files over the 2000-line WARN threshold by **domain responsibility**, each
piece under 2000, **bit-identical outputs**. This is **advisory hygiene** — measured 2026-06-18,
**10 files exceed 2000 but none exceeds the 3000 required-refactor threshold**, so nothing is
blocking. The ROADMAP's "3 files" is stale. **Scope to the 4 highest-value files and defer the
rest** — do not churn all 10.

## Target tier (split these — closest to the 3000 hard threshold)

| File | Lines |
|---|---:|
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing.rs` | 2807 |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs` | 2672 |
| `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs` | 2671 |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs` | 2549 |

Defer (advisory WARN, not this package): `00_runner_intake_and_lane_setup.rs` (2410),
`scheduler.rs` (2452), `state_access.rs` (2219), `02_output_and_climate_helpers.rs` (2095),
`openwepp-cli-watershed.rs` (2062), `openwepp-watershed-output/src/writers.rs` (2002).

## Method (per `docs/standards/mechanical-refactor-authoring-guide.md`)

- Split by **domain responsibility** into ordered section/submodule files — not arbitrary
  line-chopping. The split closes a coherent seam.
- **No tiny diagnostic-only relays** (the guide's anti-pattern). A clean WARN beats a bad split.
- **Per-file increments:** split one file, run the anchor + gates, then the next — so any
  divergence localizes to one split.
- If a file lacks a coherent seam, **leave it as a documented WARN** with rationale.

## Hard stops

1. **Bit-identical (load-bearing):** `anchor_mismatches = 0` on H2637 both `wepp_ui` variants +
   the OFE1-OFE5 ladder vs a pre-refactor baseline (HBP/loss/wat/plot byte-identical; pass rows
   equal). A *mechanical* split must not change outputs — any mismatch means a real edit slipped
   in. STOP + diagnose.
2. **Behavior-preserving only:** move code; do not change logic, observable behavior, ordering,
   or numerics. No `SC-*` change. No "while I'm here" fixes.
3. Each split piece **under 2000 lines**; moved code keeps its tests passing.

## Constraints

- Determinism (`docs/numerics/`) preserved — the bit-identity gate proves it.
- Rust gates: `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`;
  `cargo test --workspace`; `cargo deny check`; `git diff --check`.
- Truthfulness: bit-identity + line counts are empirical — label `Ran:`. Report the remaining
  WARN inventory honestly (don't claim "all clean" if files remain in the deferred tier).

## Required reading

- `docs/work-packages/20260618-refactor022-monolith-line-count-split-001/package.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/rust-scientific-coding-standard.md` (WARN 2000 / required 3000)
- `AGENTS.md`, `docs/work-packages/AGENTS.md`, `crates/AGENTS.md`, `docs/numerics/README.md`
- The PERFOPT01 / FARPOINT01 disposition for the `anchor_mismatches = 0` method.
