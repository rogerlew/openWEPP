# PERFOPT01 — Independent Review (Claude Code)

Status: APPROVE (behavior-preservation independently verified)
Evidence mode: **Static** (code-equivalence proof) + **Ran** (independent
kernel-contract tests; independent end-to-end H2637 bit-identity)

Codex executed PERFOPT01 and flagged that its dual-review/verification artifacts
were **self-review, not independent** (subagent delegation needs explicit
authorization). This artifact is the independent review that caveat called for —
authored by Claude Code (the openWEPP review role), separate from the executing
agent.

## What I reviewed

The three production changes (diff over `41469058`-equivalent base):

1. **Lazy writeback detail** (`writeback.rs`): `collect_field_violations` returns
   early when `field_satisfies_writeback_domain(field)` is true, skipping the
   `format!` subject + the check helpers.
2. **`extend` vs `merge`** (`scheduler_seed_and_runtime.rs:2128` +
   `runtime_surface_helpers.rs`): per-lane/day climate overlay applied by
   reference (`extend_runtime_surface_from`) instead of `merge_runtime_surfaces(_, climate.clone())`.
3. **Move vs clone** (`execute_persistent_scheduler_kernel_lifecycle`): lane
   `writeback_surface`s are **moved** into persistent state (consuming the
   sequence report) instead of cloned via `replace_from_report(&report)`; summary
   fields are captured first via `persistent_sequence_summary`.

## Independent findings

**Change 1 — provably equivalent, including the boundary cases the happy-path
bit-identity test cannot reach (the key independent finding).** The fast-path is
safe only if `field_satisfies_writeback_domain(true)` implies the original emits
zero violations. I traced the originals in
`openwepp-sim-contract/src/closure.rs`:
- `check_min` passes iff `value >= minimum`; `field_satisfies` uses
  `value >= minimum` — **exact** (inclusive at `min`).
- `check_max` passes iff `value <= maximum`; `field_satisfies` uses
  `value <= maximum` — **exact** (inclusive at `max`).
- `check_range`: `minimum > maximum` ⇒ **always** `Err`; else passes iff
  `(min..=max).contains(value)`. `field_satisfies` requires
  `minimum <= maximum && (min..=max).contains(value)` — so the inverted-bounds
  case returns `false` and **falls through** to `check_range` (which emits the
  `CLOSURE-PRIMITIVE-INVALID-BOUNDS` violation). **Not suppressed.**
  No boundary-exact value is silently passed; no degenerate range is masked.
- All violation `check_id`/`message_id`s (`INV-WRITEBACK-001..004`,
  `WRITEBACK_REJECT_*`) are produced on the failure path unchanged — the fast-path
  only skips the *no-violation* case.

**Change 2 — equivalent.** `merge_runtime_surfaces` is
`base.state_surface.extend(overlay.state_surface); base.flux_surface.extend(overlay.flux_surface)`
(both surfaces, overlay-wins on key collision). `extend_runtime_surface_from`
does the same by reference (cloning entries). Identical result map; one fewer
full-surface clone. `BTreeMap` key ordering is deterministic regardless.

**Change 3 — equivalent, one minor note.** Moving a `writeback_surface` yields the
same value as cloning it; `persistent_sequence_summary` extracts the same four
summary fields the old code read from `lane_reports.last()` +
`persistent_kernel_phase_message_ids` + `persistent_erod14_wave2_kernel_status_seen`.
*Minor:* the lane-count / OFE-ordering **internal-invariant error message** wording
changed (the typed error kind, `surface = per_ofe_dynamic_state`, and
`SIMPIPE_GUARD_ID` prefix are preserved; only reachable on an internal lane-count
mismatch, not on valid input). Non-blocking.

## Independent verification (Ran)

- `cargo test -p openwepp-kernel-contract` → **14 passed, 0 failed**, including
  `rejects_non_finite_payload_with_typed_status` (the writeback failure path).
- **Independent end-to-end bit-identity**: rebuilt the optimized `cli-hill` and
  re-ran H2637 `without_ui`, comparing against my **own** FARPOINT01 pre-optimization
  baseline (`/tmp/perfopt01_indep_baseline`, built from the docs-only-newer
  `41469058` code). Result: **`anchor_mismatches = 0`** — `H2637.hbp` byte-identical,
  `H2637.{wat,pass}.parquet` table-equal (`235,961` + `12,419` rows),
  `loss.json`/`plot` byte-identical. Optimized run `837 s` vs my baseline `~1016 s`
  (~1.2×), independently confirming both correctness and the speedup direction.
- Codex's bit-identity evidence (7 fixtures: HBP byte + Parquet table equality)
  and determinism (OFE5 twice, byte-identical) reviewed and found methodologically
  sound; the speedup (H2637 1.15×, within the PERFHO01 ~1.5–2.5× band's low end —
  Codex did the safe clone/lazy subset, not the riskier `BTreeMap` data-structure
  replacement) is honestly bounded, with the residual routed to `PERFHO02`.

## Verdict

The optimization is **behavior-preserving** — proven for all inputs (the boundary
and inverted-bounds cases included), and empirically confirmed on the success path.
Gates green; determinism preserved; no contract/physics/output change; line-count
WARN dispositioned below the hard threshold. The independent-review caveat is
**resolved by this review**. Approve to land. Successor: `PERFHO02`.
