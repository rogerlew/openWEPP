# PERFMIG02 - WB11 Consumer-Cluster Boundary Retirement (the conversion test)

Status: executed-redirect 2026-06-18. The second production rung — and the **first rung that must
show a measured endpoint win.** Gated by PERFMIG01's CONTINUE + its central lesson.

Package type: **Production migration — boundary retirement, not boundary addition.** PERFMIG01 proved the
pattern is safe but went net-negative by construction. This rung must prove the migration *converts* to a
measured speedup, or trigger the REDIRECT.

## Why this exists — PERFMIG01's lesson, stated sharply

PERFMIG01 (commit `6b54db2d`, CONTINUE) migrated the WB11 warm-rain runoff **writeback** to a dense 543+8
`SymbolId` payload, identity-airtight (H2637 `.hbp`+`wat.parquet` byte-identical). But the H2637 endpoint
went the **wrong way**: 666.82 → 669.97 s (**+0.47%**, 73.12× → 73.46×). That was predicted and allowed,
and here is exactly why:

**A writeback-only rung is net-negative by construction.** It swaps the *output representation* but the
scheduler immediately materializes dense → logical (`apply_indexed_kernel_writeback` = **107.5 µs/payload**)
because every downstream phase still reads logical maps. That dense→logical round-trip is **pure added
overhead** — symbol resolution relocated, not removed. **The apply boundary retires only when the
downstream READERS of those 543 symbols migrate to read dense directly.** Migrating more *writers* without
their *readers* makes boundaries **accumulate** and the endpoint keeps drifting up.

So PERFMIG02 does the other half: **migrate the readers**, drop the materialization for the WB11 outputs,
and **measure the boundary retire as a real endpoint improvement.**

## What this migrates — the WB11-output consumer cluster

1. **Map the reader set (step 1, do this first).** Identify the phases/sites that **read** the 543 WB11
   runoff state symbols + 8 flux symbols PERFMIG01 now writes dense. Candidate surfaces: the downstream
   hydrology phases (`hydrology_phase_infiltration_evap`, `hydrology_phase_lateral_drainage`,
   `hydrology_phase_plant_percolation`, `hydrology_phase_storage_erosion`), the scheduler transfer/carry
   logic, and `consumer_boundary.rs` / `01_phase_routing.rs`. An `Explore` read-only pass over
   `kernel_phases_mod/` + the consumer-boundary surfaces is the right first move. The deliverable names the
   exact reader set and which of the 543+8 each reads.
2. **Migrate those readers to read dense `SymbolId` slots** (the `IndexedWritebackSurface` /
   `IndexedSurface`) for the WB11 symbols, instead of reading the materialized logical maps.
3. **Drop the dense→logical materialization for the WB11 symbols** at the PERFMIG01 seam — the
   `apply_indexed` step no longer resolves+inserts those symbols into the logical map, because nothing
   downstream reads them logically. The boundary **moves outward** toward I/O (where it already exists),
   or is removed entirely for the cluster.
4. Symbols still consumed by *unmigrated* phases (publication, diagnostics, snow/frost/irrigation/MOFE
   branches) keep their named materialization — measured separately, retired on later rungs.

## The hardened gate — a MEASURED endpoint improvement (non-negotiable)

PERFMIG01 was allowed to be net-negative on the strength of PERFARCH03's proven floor. **That faith is now
spent.** This rung's acceptance is not "trajectory projection" — it is a **measured H2637 endpoint
improvement vs PERFMIG01's 669.97 s**, attributable to the retired apply-boundary:

- **PASS / CONTINUE:** H2637 endpoint **improves measurably** (recovers at least the PERFMIG01 +3.15 s and
  ideally more, as the ~25 s projected boundary cost partially retires), identity still airtight. The
  boundary-collapse mechanism is **confirmed converting** → widen to the next cluster.
- **REDIRECT (the real signal):** the endpoint is **flat or still negative** after retiring the boundary.
  That means **two consecutive net-negative rungs** — the widen-and-retire mechanism is **not** converting
  fast enough to clear budget. Pivot the attack: migrate a phase **fully array-native** (dense
  read + compute + write, capturing the ~140 µs → sub-µs internal-compute win PERFARCH03 measured),
  accepting two transitional edge boundaries, rather than widening writeback-only across many phases. State
  exactly where the cost stayed.

A measured win confirms the thesis; a flat second rung refutes the *widen-first* strategy (not the floor —
the floor is proven — but the path to it). Either outcome is decisive information.

## The deeper truth this rung must keep visible

Even with this boundary retired, **writeback-representation flip alone cannot reach the PERFARCH03 0.96 µs
floor.** That floor was a *fully* array-native branch (dense **read + compute + write**). PERFMIG01 +
PERFMIG02 migrate only the **write + the downstream read** of the WB11 *outputs*; the kernel still **reads
its inputs logical and computes over logical state** — the bulk of the ~140 µs/branch machinery (the
`BoundarySymbol`/`BTreeMap` lookups *inside* the physics, the PERFARRAY01/02 input-side lever). The full
path to ≤10× is **both** levers: retire boundaries (this rung) **and** migrate input-reads + internal
compute to dense (a later rung / the REDIRECT pivot). Do not let a modest boundary-retirement win read as
"on track for budget" — it is one of two levers.

## Honest-measurement discipline (carried forward — non-negotiable)

- **Exact identity preserved:** the PERFMIG01 543+8 focused fixture + the H2637 `.hbp`/`wat.parquet`
  byte-identity + `pass.parquet` Arrow-equality must all still hold. A reader migration that changes an
  output is a bug, not a win.
- **Real H2637 endpoint + RSS**, same machine/fixture as PERFIDX06 / PERFMIG01. The measured delta vs
  669.97 s is the deliverable.
- **No dual-read** (logical + dense) for the migrated symbols on the normal path; the dense slot is the
  single read authority for them.
- **The retired-boundary accounting:** show the `apply_indexed` cost for the migrated symbols actually
  *drops* (perf or the boundary bench), not just that the endpoint moved — so the win is *attributable*,
  not noise. (+0.47% on a 667 s run is near run-to-run variance; the attribution must be explicit.)
- Determinism + conservation gates green; workspace Rust gates (fmt, check, clippy `-D warnings`, test) +
  `cargo deny` + markdown lint green.

## Scope

In scope: map the WB11-output reader set; migrate those readers to dense `SymbolId` reads; retire/move the
WB11 materialization boundary; the measured H2637 endpoint delta + retired-boundary attribution; the
CONTINUE/REDIRECT decision.

Out of scope:

- No new writeback-only writer migrations that add boundaries without retiring one.
- No science-contract / HBP / parquet schema changes; no irrigation activation; no physics changes —
  byte-identical results (ADR-0023 Non-Decisions).
- No removal of logical surfaces from public/reporting/diagnostic boundaries (only the *internal*
  phase-to-phase materialization for the migrated symbols is dropped).

## Acceptance Criteria

- The WB11-output reader set is mapped and named (which phases read which of the 543+8).
- Those readers read dense `SymbolId` slots for the WB11 symbols; the internal dense→logical
  materialization for those symbols is dropped/moved (no dual-read on the normal path).
- **Exact identity preserved** (PERFMIG01 fixture + H2637 `.hbp`/`wat`/`pass`).
- **A measured H2637 endpoint improvement vs 669.97 s, with the retired-boundary cost shown to drop**
  (attribution, not just an endpoint number) — OR a documented flat/negative result triggering REDIRECT.
- Workspace Rust gates + `cargo deny` + determinism/conservation + markdown lint green.
- CONTINUE (widen next) or REDIRECT (pivot to deep single-phase array-native) with numbers.

## Deliverables

- `artifacts/perfmig02-reader-map.md` (the WB11-output reader set; which symbols each reads; the boundary)
- `artifacts/perfmig02-migration.md` (what was flipped to dense reads; which materialization was retired)
- `artifacts/perfmig02-bit-identity.md` (PERFMIG01 fixture + H2637 identity still exact)
- `artifacts/perfmig02-endpoint-timing.md` (H2637 s + ratio + RSS vs PERFMIG01/PERFIDX06; **retired-boundary
  attribution** — the `apply_indexed` cost for migrated symbols measured to drop)
- `artifacts/perfmig02-logical-free-proof.md` (no dual-read; the WB11 materialization dropped/moved)
- `artifacts/perfmig02_disposition.md` (CONTINUE + next cluster / REDIRECT + the deep-migration pivot)

## Execution Result

Executed by Codex on 2026-06-18.

Result: `EXECUTED-REDIRECT`. PERFMIG02 preserved identity, but the final-code H2637 endpoint was flat to
negative versus PERFMIG01 and the apply-boundary attribution subgate failed.

- Code landed dense-first hot scalar helper reads and an explicit indexed writeback logical-materialization
  policy.
- Internal logical materialization was retired for six symbols: `wb12_infiltration`,
  `wb12_runoff_reconciled`, `wb14_soil_conductivity_m_s`, `wb14_effective_conductivity_m_s`,
  `wb14_matric_potential_m`, and `wb12_runoff_carryover`.
- Final release binary SHA: `d4f7603e79fdf415e3e4123a2baa7df19a6cb7780e8d01206bfaad6ef012d63b`.
- H2637 no-UI final-code endpoints: `672.14 s`, `227636 KB`; repeat `675.00 s`, `228152 KB`.
- Delta vs PERFMIG01 `669.97 s`: `+2.17 s` (`+0.32%`) and `+5.03 s` (`+0.75%`).
- Delta vs PERFIDX06 `666.82 s`: `+5.32 s` (`+0.80%`) and `+8.18 s` (`+1.23%`).
- HBP and WAT are byte-identical against the independent PERFMIG01 output copy; PASS is Arrow-equal with
  metadata ignored.
- Transition-boundary bench: materialize-all apply `104.752336 us/payload`; PERFMIG02 skip-six apply
  `105.460510 us/payload`. The retired apply-boundary cost did not drop because fail-closed stale logical
  removal costs more than the six avoided logical inserts.

## Code Disposition (Claude, post-execution, operator-directed 2026-06-18)

The PERFMIG02 **production code was reverted**; only this package's artifacts (the REDIRECT evidence)
landed on `main`. Rationale (operator's call): the diff was identity-clean and gate-green but **regressed
the H2637 endpoint +0.5%** (672–675 s vs PERFMIG01 669.97 s) and implemented the **now-abandoned**
incremental writeback/materialization-retirement tactic — shipping a known regression and dead-end
machinery to `main` is the wrong move when the pivot re-architects this surface anyway. `main` therefore
stays at the clean PERFMIG01 baseline (`6b54db2d`, 669.97 s), and the deep-cut pivot (PERFDEEP01) starts
from there. The reverted code is recoverable from this session's transcript / Codex's working tree if the
pivot wants the dense-first read helper (`state_access.rs`) as a starting point. **The REDIRECT verdict and
its evidence stand as the durable output of this package** — the experiment was run, measured honestly, and
falsified the widen-first tactic; that is the value, not the code.

Disposition: `REDIRECT`. Do not run another writeback-only or tiny materialization-retirement rung. Pivot to
a deep single-phase array-native read+compute+write migration, or author a new package with reader-side dense
conversion as the explicit measured acceptance target.

## Dependencies

- `docs/work-packages/20260618-perfmig01-wb11-runoff-array-authoritative-production-migration-001/` — the
  543+8 dense writeback now in production; the 107.5 µs apply-boundary this rung retires; the fixtures
- `docs/work-packages/20260618-perfarch03-full-array-native-floor-prototype-001/` — the 0.96 µs floor; the
  reminder that the full win also needs the internal-compute lever
- `docs/decisions/0023-array-authoritative-hot-path-state.md` — the ratified array-authoritative authority
- `docs/work-packages/20260618-perfidx06-high-ofe-target-assessment-001/` — the 73.12× anchor, budgets, timing protocol
- `crates/openwepp-hillslope-orchestrator/src/{scheduler.rs,consumer_boundary.rs,phase.rs,hydrology/01_phase_routing.rs}`
  + `hydrology/kernel_phases_mod/` — the reader surfaces to map and migrate
- `crates/openwepp-kernel-contract/src/lib_mod/writeback.rs` — `apply_indexed_kernel_writeback` (the boundary)
- `AGENTS.md`; `crates/AGENTS.md`; `docs/numerics/README.md`; `docs/work-packages/AGENTS.md`

## Subagent Requirement

**Recommended: one `Explore` read-only pass** to map the WB11-output reader set across `kernel_phases_mod/`
+ `consumer_boundary.rs` + the scheduler transfer logic before migrating. The migration + measurement are
local thereafter.

## Autonomy

Execute end-to-end: map the readers, migrate them to dense reads, retire the WB11 materialization boundary,
measure the H2637 endpoint + attribute the retired-boundary cost, and make the CONTINUE/REDIRECT call.
**A measured endpoint improvement is the deliverable this time — not a projection.** If the endpoint stays
flat/negative after retiring the boundary, that is the REDIRECT signal (two net-negative rungs → pivot to
deep single-phase array-native migration); report it honestly with the numbers, do not absorb it as
"another expected transitional rung."
