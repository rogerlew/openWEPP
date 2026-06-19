# PERFDEEP03 — Persistent Lane-Owned Dense State (array-native Stage 1, redone)

Status: executed 2026-06-19. Verdict: `NO-GO - section 7 falsification / re-profile before expanding`.
Re-does Stage 1 of the array-native re-architecture
([ADR-0025](../../decisions/0025-array-native-hillslope-day-frame.md); spec §4.6, §8) after PERFDEEP02's
NO-GO. **The dense frame becomes the carried lane runtime state — not a temporary mirror around the old
maps.**

Package type: **Production migration — ownership move (refactor forward on PERFDEEP02's verified pieces).**
PERFDEEP02 proved the temporary-mirror approach regresses 3.6×; this package keeps its verified mechanics
and moves *ownership* of the runtime state into a persistent lane-owned frame.

## Why this exists — the PERFDEEP02 lesson, stated as the fix

PERFDEEP02 regressed H2637 to **2417 s (3.6×)** because it kept the logical/indexed surfaces as the *real*
runtime state and **rebuilt a `Vec<Option<BoundaryValue>>` frame sized to the full registry (~4038 slots),
re-seeded/flushed per OFE-day** (×235,961) — a temporary dense mirror around old maps (the PERFIDX
dual-representation ceiling in frame form; commit `fa29c34b`, kept opt-in as a **verified negative
benchmark**). The PERFARCH03 floor (0.96 µs) is intact; the *runtime-state ownership* did not match it.

**The fix is ownership, not optimization:** stop using the dense frame as a temporary mirror; make the lane
runtime **own a persistent dense state frame**, and have scheduler phases **borrow views** into it.

## Move ownership

- **From:** `scheduler.rs` creates and reconciles a temporary `HillslopeDayFrame` per OFE-day around the
  authoritative logical/indexed surfaces.
- **To:** the **lane runtime owns a persistent dense state frame**; scheduler phases borrow `&`/`&mut`
  views; logical surfaces are materialized **only at true boundaries**.

## Concrete shape (operator-specified)

1. **Create the dense frame once** at the start of per-day / per-OFE lane execution.
2. **Keep it alive** across the full hydrology phase chain.
3. **Hydrology reads and kernel writebacks update the frame directly** (no per-phase rebuild).
4. **Track dirty slots** with a compact dirty bitset / id list.
5. **Materialize back to logical/indexed only at true boundaries:** a non-migrated phase boundary, output
   serialization, diagnostics/contract evidence, the external API boundary.
6. **No full-frame seed/flush loops inside scheduler phase execution.**
7. **Default production stays disabled** until H2637 proves a real endpoint win
   (`OPENWEPP_PERFDEEP02_FRAME_ISLAND` lineage / a PERFDEEP03 opt-in gate).

## Keep (verified PERFDEEP02 pieces — reuse, do not rebuild)

- `HillslopeKernelRequest` dense-slot support;
- hydrology dense-first read helpers;
- `HillslopeDayFrame` mechanics;
- dirty-tracking helpers;
- the real-surface round-trip diagnostic (`OPENWEPP_PERFDEEP02_FRAME_ROUNDTRIP_PATH`, 235,961-row zero-mismatch);
- the focused tests + evidence docs;
- the fail-closed env gate.

## Retire / replace

- the **scheduler-level temporary island** that seeds/flushes around old surfaces (replace with the
  lane-owned persistent frame);
- **`Vec<Option<BoundaryValue>>` as the *eventual* hot runtime representation** — the persistent lane state
  must be structured so the representation can tighten (toward packed/typed) without another rewrite; a
  full-registry slot vector must not be the carried state. (The immediate win is ownership + hot-set +
  persistence; further representation tightening is a lever pulled only if the endpoint still demands it
  after the ownership move.);
- **per-phase / per-boundary reconciliation as a normal execution pattern** (reconciliation happens only at
  true boundaries, not every phase).

## Hot working set, not the full registry

The carried frame holds the **hot hydrology working set**, not all ~4038 registry symbols. Climate forcing
is **borrowed** (spec §4.1), not slotted; publication/diagnostic-only symbols are not hot-frame state. This
is the difference between PERFARCH03's measured 18 KB / ~543-symbol working set and PERFDEEP02's
~190 KB / 4038-symbol per-day allocation.

## Hard pass criteria (operator — non-negotiable)

- **H2637 output identity preserved** (`.hbp`/`.wat` byte-identical, `pass` Arrow-equal; the PERFDEEP02
  round-trip diagnostic stays zero-mismatch).
- **Full workspace gates pass** (fmt, check, clippy `-D warnings`, test, `cargo deny`, markdown, determinism).
- **The opt-in dense path BEATS the PERFDEEP01 baseline (669.97 s) on the real H2637 endpoint** — a real
  end-to-end run, **not a microbenchmark**. The §8 Stage-1 target is ~73× → ~43–50× (≈407–450 s); the gate
  is simply *measurably faster than 669.97 s* with the prediction as the aim.
- **No default activation** until the endpoint win holds. Until then the path stays opt-in / fail-closed.

## Falsification boundary (spec §7)

This is the lane-owned frame built correctly. If the opt-in dense path — persistent, lane-owned, hot-set,
materializing only at true boundaries — **still** does not beat 669.97 s, that is the genuine §7
falsification signal (the partial-island edge cost or a deeper factor dominates): stop and re-profile before
expanding the island. PERFDEEP02 does **not** trigger §7 (it wasn't the lane-owned design); PERFDEEP03 can.

## Scope

In scope: the ownership refactor (lane-owned persistent frame + borrowed views); hot-set sizing + borrowed
forcing; dirty-slot tracking + boundary-only materialization; the real H2637 endpoint measurement; the
opt-in gate. Reuse the PERFDEEP02 verified mechanics.

Out of scope: output schema changes; science/numeric formula changes (byte-identical); default activation
(gated on the endpoint win); migrating non-hydrology phases (erosion/growth are later stages); deleting the
logical hot path (Stage 5).

## Acceptance Criteria

- Lane runtime owns one persistent dense frame; scheduler phases borrow views; no per-day temporary frame
  construction; no full-frame seed/flush loop inside phase execution (static + diff evidence).
- Frame holds the hot working set (not the full registry); forcing borrowed; dirty-slot tracking;
  materialization only at true boundaries.
- H2637 output identity + round-trip zero-mismatch preserved; workspace gates green.
- **Real H2637 endpoint measured; opt-in dense path < 669.97 s** (CONTINUE → expand the island / default
  activation), **or** a documented §7 falsification (still ≥ baseline → re-profile).
- Default path unchanged (dense path opt-in until the win holds); confirm default endpoint flat.

## Deliverables

- the ownership refactor (lane-owned persistent frame + borrowed views; production Rust, opt-in)
- `artifacts/perfdeep03-ownership-refactor.md` (from-temporary-mirror to lane-owned; the boundary set)
- `artifacts/perfdeep03-endpoint.md` (**real H2637 endpoint vs 669.97 s**, the load-bearing gate; RSS)
- `artifacts/perfdeep03-identity.md` (output identity + round-trip zero-mismatch preserved)
- `artifacts/perfdeep03-gate-results.md` (workspace gates)
- `artifacts/perfdeep03_disposition.md` (CONTINUE + measured win → expand/default, or §7 falsification → re-profile)

## Execution Result

PERFDEEP03 implemented the lane-owned persistent compact dense state and passed
the required correctness gates, but failed the load-bearing H2637 endpoint gate.

Ran opt-in:

```text
env OPENWEPP_PERFDEEP03_LANE_DENSE_STATE=1 \
  /usr/bin/time -f "h2637_perfdeep03_lane_dense_hot_first\t%e\t%M" \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file docs/work-packages/20260619-perfdeep03-persistent-lane-owned-dense-state-001/artifacts/runfiles/perfdeep03-h2637.run \
  --output-dir /tmp/perfdeep03/current/h2637_same_manifest \
  --policy compat \
  --legacy-sidecar-discovery
```

Result:

```text
h2637_perfdeep03_lane_dense_hot_first 1147.96 229580
```

The required gate was `< 669.97 s`, so the package closes `NO-GO`. Identity
passed for the scoped outputs: HBP/WAT byte identity, PASS Arrow equivalence, and
235961 diagnostic roundtrip rows with no mismatch rows. Full Rust closure gates
passed:

```text
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
```

Default production remains disabled. The next action is not default activation
and not blind expansion; re-profile the PERFDEEP03 endpoint to identify the
remaining edge/fallback costs before opening a follow-on package.

## Dependencies

- `docs/work-packages/20260619-perfdeep02-hydrology-island-core-001/` — the verified scaffolding to reuse +
  the negative benchmark + `artifacts/review-claude-independent.md` (the ownership diagnosis)
- [`docs/architecture/array-native-runtime-specification.md`](../../architecture/array-native-runtime-specification.md)
  **§4.6 (ownership — binding)**, §4.1 (hot-set/borrow-forcing), §7 (falsification), §8 (staging)
- `docs/work-packages/20260618-perfarch03-full-array-native-floor-prototype-001/` — the 0.96 µs floor; the
  small (18 KB) working set the carried frame must match
- `crates/openwepp-hillslope-orchestrator/src/{scheduler.rs,day_frame.rs}` — the temporary-island site to
  refactor into lane-owned ownership; the OfeLane persistent-state seam
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/02_boundary_values_and_kernel_requests.rs` — the
  dense-slot request support to reuse
- `AGENTS.md`; `crates/AGENTS.md`; `docs/numerics/README.md`; `docs/work-packages/AGENTS.md`

## Subagent Requirement

None required. The refactor + the H2637 endpoint measurement are local. The endpoint is the deliverable.

## Autonomy

Execute end-to-end: move ownership to a lane-owned persistent dense frame (the seven-point shape), reuse the
verified PERFDEEP02 mechanics, hold the hot working set, materialize only at true boundaries, and **measure
the real H2637 endpoint vs 669.97 s**. The endpoint win is the gate — not a microbenchmark, not a
projection. CONTINUE only on a measured win; otherwise report the §7 falsification honestly. Keep default
disabled until the win holds.
