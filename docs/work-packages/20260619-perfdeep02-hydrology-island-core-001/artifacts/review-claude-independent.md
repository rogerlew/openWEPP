# PERFDEEP02 — Independent Review (Claude Code)

Verdict: **NO-GO confirmed — honest, and the root cause is a fixable implementation deviation from the
PERFARCH03-validated design, NOT a falsification of the array-native floor.** Identity held and the
deferred PERFDEEP01 round-trip gate is *closed*; the perf is blocked by a per-OFE-day frame-lifecycle bug.
The spec §7 falsification ("a *properly-sized* island that doesn't move the endpoint → halt") does **not**
fire, because this island was not built to the validated design.

Evidence mode: **Static** (read `day_frame.rs`, the scheduler island hooks, the kernel-contract dense-slot
plumbing, all artifacts) + **Ran** (none; relied on Codex's gate + endpoint runs).

## The good — genuinely delivered

- **Identity is airtight.** The pre-final island run produced HBP/loss/plot/WAT checksum identity + PASS
  row/schema identity vs the PERFDEEP01 snapshot. The migration is *correct*. ✓
- **The deferred PERFDEEP01 round-trip gate is CLOSED.** A real H2637 run seeded/flushed a frame every
  OFE-day and asserted `to_bits()` identity: **235,961 rows, zero mismatch**, max **4038** state symbols/row,
  all **24/24** MOFE hourly families covered. The frame is a faithful representation of the *full real
  symbol set across all branches* — exactly the completeness PERFDEEP01 owed. ✓
- **Disabled by default, fail-closed** (`OPENWEPP_PERFDEEP02_FRAME_ISLAND=1`); the dense-first reads are
  inert when the flag is off (`dense_state_slots: None` → free branch). Default path static-safe. ✓
- Gates green; honest NO-GO (the opposite of over-claiming — Codex correctly refused to ship it). ✓

## The NO-GO — endpoint 2417 s (3.6× regression), root cause verified

The island regressed H2637 to **2417 s** (whole-frame flush), with dirty-id-flush and indexed-seed variants
**terminated >23 min**. Verified root cause:

- `day_frame.rs:169`: the frame is `vec![None; symbol_registry.len()]` — sized to the **full registry**.
- The registry is **~4038 state symbols** (artifact-confirmed), inflated by the climate forcing series
  (`timem`/`intsty`, up to 1500 each) + per-layer + MOFE families.
- `scheduler.rs:1428/1612`: the frame is allocated **fresh per OFE-day** and seeded from the logical
  surface at the first island phase.

So every one of **235,961 OFE-days** allocates + seeds + flushes ~4038×2 slots (~190 KB) of
`Option<BoundaryValue>`. That memory churn is the 2417 s.

## Why this is a deviation, not a falsification

PERFARCH03 measured the array-native floor at **0.96 µs/OFE-day** using a **small** dense slot set
(~543 symbols, **18 KB** working set) in a tight loop — *not* the full registry, and *not* re-allocated per
day. PERFDEEP02 built a frame **~7× larger** (full 4038-symbol registry, including read-only climate forcing
that spec §4.1 explicitly says to **borrow, not slot**, plus publication/diagnostic symbols that are not hot
hydrology state) **and** re-allocated it per OFE-day. Two compounding deviations from the validated design.
The floor is intact; the implementation missed it.

The proof the fix works is arithmetic: a **small, persistent** frame (allocate once; carry across OFE-days;
hold only the hot hydrology working set; borrow forcing) drops the island's internal cost from
~8×140 µs = 1120 µs/OFE-day to ~8 µs, paying only a per-day *edge* seed/flush over the island's read/write
set (~150 symbols ≈ ~15 µs/OFE-day). Net ≈ **−1100 µs/OFE-day → ~407 s ≈ ~44× legacy** — exactly the
predicted Stage-1 "73× → ~44×" win. The catastrophe was the full-registry-per-day frame, nothing deeper.

## The staging caution this surfaces (spec reconciliation owed)

Even built correctly, a **partial** island pays per-OFE-day **edge boundaries** (seed read-set in, flush
write-set out) because the surrounding non-island phases + day-to-day persistence are still logical. The win
holds only if the frame is **small + persistent** so the edge cost stays far below the internal saving.
Spec §4.1 must state the hot-set-only / persistent / borrow-forcing constraint explicitly (PERFDEEP02
violated all three), and §8 must note the per-day edge-boundary cost of partial stages. I will amend these
once the path is confirmed.

## Operator direction — the path (2026-06-19)

The operator sharpened the diagnosis to its root: **the failure is the *ownership/lifecycle* pattern, not
the dense-slot mechanics.** PERFDEEP02 kept the logical/indexed surfaces as the *real* runtime state and
repeatedly built a **temporary dense mirror around them** per OFE-day — the PERFIDX dual-representation
ceiling again, in frame form. (Frame *size* was a compounding factor; the *temporary-mirror* pattern is the
root.) The current opt-in path is therefore retained on `main` as a **verified negative benchmark**: it
proves temporary dense mirrors are the wrong approach.

**Keep (verified, reused):** `HillslopeKernelRequest` dense-slot support; hydrology dense-first read
helpers; `HillslopeDayFrame` mechanics; dirty-tracking helpers; the round-trip diagnostic; tests + evidence;
the fail-closed env gate.

**Retire / replace in PERFDEEP03:** the scheduler-level *temporary island* that seeds/flushes around the old
surfaces; `Vec<Option<BoundaryValue>>` as the *eventual* hot representation; per-phase/per-boundary
reconciliation as a normal execution pattern.

**The move is ownership:** *from* "scheduler creates and reconciles a temporary `HillslopeDayFrame`" *to*
"the lane runtime **owns a persistent dense state frame**; scheduler phases **borrow views** into it."
Concrete shape: create the dense frame once at the start of per-day/per-OFE lane execution; keep it alive
across the full hydrology phase chain; hydrology reads + kernel writebacks update it directly; track dirty
slots with a compact dirty bitset/id list; **materialize back to logical/indexed only at true boundaries**
(non-migrated phase, output serialization, diagnostics/contract evidence, external API); **no full-frame
seed/flush loops inside scheduler phase execution**; default stays disabled until H2637 proves a real win.

**Hard pass criteria for PERFDEEP03 (operator):** H2637 output identity preserved; full workspace gates
pass; **the opt-in dense path beats the PERFDEEP01 baseline (real H2637 endpoint, not a microbenchmark)**;
no default activation until that holds. This is **not** the spec §7 halt — the PERFARCH03 floor stands; the
runtime-state ownership must finally match it.
