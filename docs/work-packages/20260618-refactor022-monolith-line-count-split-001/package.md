# REFACTOR022 - Monolith Line-Count Split (behavior-preserving)

Status: complete 2026-06-18. The four target-tier WARN-band monoliths were split by
domain responsibility, every resulting source file is under 2000 lines, full Rust
gates passed, and the true pre-refactor HEAD anchor comparison closed with
`anchor_mismatches = 0`. The 2000-2500 line WARN tier remains deferred.

Package type: **Behavior-preserving mechanical refactor.** Pure structural split by domain
responsibility — **bit-identical outputs** are the gate (no logic change), exactly the
PERFOPT01 anchor discipline. This is **advisory hygiene**, not a required refactor (see below).

## Reality check — the "3 files" framing is stale

The ROADMAP item said "split the 3 files that crossed 2000 lines." The current reality
(measured 2026-06-18): **10 `.rs` files exceed the 2000-line WARN threshold, and *none*
exceeds the 3000-line required-refactor threshold** (`docs/standards/rust-scientific-coding-standard.md`:
WARN 2000, required 3000). Several grew from the *kept* PERFIDX04 read-side win
(`core_types.rs`, `state_access.rs`), not just MOFE. So: nothing is **blocking**; this is
WARN-band cleanup. Scope to the highest-value tier and defer the rest rather than churn all 10.

## Target tier — the 4 files over 2500 (closest to the 3000 hard threshold)

These have the least headroom before a *required* refactor; split each by domain
responsibility to under 2000:

| File | Lines | Domain |
|---|---:|---|
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing.rs` | 2807 | watershed routing kernel |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs` | 2672 | persistent scheduler lifecycle |
| `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs` | 2671 | kernel-contract core types |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs` | 2549 | WB19 lateral drainage |

## Deferred tier — the 6 files 2000-2500 (advisory WARN, not this package)

`00_runner_intake_and_lane_setup.rs` (2410), `state_access.rs` (2219),
`02_output_and_climate_helpers.rs` (2095), `scheduler.rs` (2452),
`openwepp-cli-watershed.rs` (2062), `openwepp-watershed-output/src/writers.rs` (2002).
Each has 500-1000 lines of headroom to the hard threshold. Leave as advisory WARN or a
follow-on (`REFACTOR023`); do **not** force them here.

## Method (per `docs/standards/mechanical-refactor-authoring-guide.md`)

- **Split by domain responsibility** into ordered section/submodule files — *not* arbitrary
  line-chopping to hit a number. The split must close a coherent seam.
- **No tiny diagnostic-only relays** (the guide's anti-pattern): a split that fragments a
  cohesive responsibility into stubs that can't stand alone is worse than the WARN.
- **Per-file increments:** split one file at a time; run the bit-identity anchor + gates after
  each, so any divergence localizes to one split.
- **If a file does not split coherently** by responsibility, leave it as a **documented WARN**
  rather than forcing an incoherent chop — and record why. A clean WARN beats a bad split.

## Hard stops

1. **Bit-identical outputs (load-bearing):** `anchor_mismatches = 0` on H2637 both `wepp_ui`
   variants + the OFE1-OFE5 ladder vs a pre-refactor baseline — HBP / loss / wat / plot
   byte-identical; pass-parquet rows equal (container churn expected). Any mismatch → STOP +
   diagnose (a *mechanical* split must not change outputs; a divergence means a real edit slipped in).
2. **Behavior-preserving only:** move code, do not change logic, signatures' observable
   behavior, ordering, or numerics. No `SC-*` change. No "while I'm here" fixes.
3. **Each split piece under 2000 lines**, and the moved code keeps its tests passing.

## Scope

In scope: the structural split of the 4 target-tier files (or the subset that splits coherently),
by domain responsibility; per-file bit-identity + gates; line-count governance recording the new counts.

Out of scope:

- The deferred 2000-2500 tier (advisory / `REFACTOR023`).
- **No logic / behavior / numerics change** — mechanical only.
- No `SC-*` or output-schema change. Irrigation N/A.
- No forced split of a file that lacks a coherent responsibility seam.

## Acceptance Criteria

- Each split target ends **under 2000 lines** (or is left as a documented WARN with rationale).
- **Bit-identity:** `anchor_mismatches = 0` on H2637 both variants + OFE ladder vs a pre-refactor
  baseline (HBP/loss/wat/plot byte-identical; pass rows equal).
- **Rust gates:** `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`;
  `cargo test --workspace`; `cargo deny check`; `git diff --check`.
- **Line-count governance** recorded (before/after per file; the remaining WARN inventory).
- Determinism preserved (a mechanical split cannot change it; the bit-identity gate proves it).

## Deliverables

- `artifacts/refactor022-split-plan.md` (per-file responsibility seams; what moves where)
- `artifacts/refactor022-bit-identity-evidence.md` (per-file anchor results)
- `artifacts/refactor022-line-count-governance.md` (before/after; remaining WARN inventory)
- `artifacts/refactor022-gate-results.md`
- `artifacts/refactor022-review-a.md`
- `artifacts/refactor022-review-b.md`
- `artifacts/refactor022-worker-handoff.md`
- `artifacts/refactor022_disposition.md`

## Dependencies

- `docs/standards/mechanical-refactor-authoring-guide.md` (split patterns, validation flow)
- `docs/standards/rust-scientific-coding-standard.md` (WARN 2000 / required 3000; split by responsibility)
- The PERFOPT01 / FARPOINT01 anchor method (`anchor_mismatches = 0`) for the bit-identity gate
- `AGENTS.md`; `docs/work-packages/AGENTS.md`; `crates/AGENTS.md`; `docs/numerics/README.md`

## Subagent Requirement

None required. If the operator authorizes subagents, the per-file splits are independent and
parallelizable (each in its own crate scope). Run gates + the anchor locally; record evidence.

## Autonomy

Execute end-to-end through the split plan, per-file split + bit-identity + gates, line-count
governance, dual review, and disposition. Stop on any bit-identity divergence (a mechanical
split must be byte-identical). Leaving a file as a documented WARN because it lacks a coherent
seam is a valid outcome — do not force a bad split to hit a number.
