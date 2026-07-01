# Kernel-Boundary CQR Burndown ExecPlan

Status: **active** — coverage-restoration + code-quality burndown following the
symbol-map kernel-boundary deletion.
Owner: maintainers (Claude Code maintains this doc; Codex runs the CQR packages).
Last updated: 2026-06-30.

## Why this exists (and the coverage state it tracks)

The wholesale kernel-boundary deletion (`a381702b`, "Delete symbol-map kernel boundary
runtime") reached the single-authority terminal state — `scheduler.rs`/`day_frame.rs`/
carriers removed, typed `DirectRunFrame` the sole runtime authority, H2637
byte/value-identical, `compatibility_edge_invocations=0`. In doing so it **deleted 85
test files (~657 tests; the full suite went 1,878 → 1,221) with zero replacements.**

Some of those were genuinely scheduler-era (symbol-surface projection, writeback
roundtrip, `day_frame` shadow) and are correctly gone. But many validated **surviving
behavior and ratified contracts** — the WEPP input parsers, the FC/WP policy contract,
the BASECOND01 harmonic-`ssc` conductivity closure, WB11–19 publication closure,
hydrology, growth, and MOFE per-OFE conservation. That coverage is **not obsolete** — it
must be restored.

**Explicit coverage-state note:** `main` is at reduced coverage (1,221 vs 1,878) until
this burndown completes. This document makes that gap **tracked, not silent** — the
guardrail that was missing when the tests vanished.

## Why CQR, not 1:1 test restoration

Re-writing the deleted tests 1:1 would re-couple coverage to the exact
`runtime_surface`/scheduler-era functions the migration is deleting — tests destined to
be deleted again. Each CQR package instead **revises its module (code quality) AND
restores coverage stated at the stable surface** (the `SC-*#INV-*` contract, the WB
closure identity, the parser input→output behavior), so the tests survive further
refactors.

## Governing rules (CQR discipline)

1. **Coverage requirements are contract/behavior assertions, not function-signature
   tests.** Anchor each to an existing `SC-*#INV-*` or WB identity where one exists;
   otherwise state the behavior explicitly (parser input→output, conservation).
2. **Tests are written against the typed boundary / contract**, never the symbol
   surface or a soon-to-refactor helper.
3. **No silent retirement.** A coverage requirement (or a deleted test's behavior) may be
   retired only with an explicit, named obsolescence justification (the behavior genuinely
   no longer exists). Every one of the 85 deleted files is accounted as either
   *restore-as-requirement* or *retired-with-justification* below.
4. **Net-coverage gate.** No CQR package closes with a net test-count decrease except for
   provably-obsolete scheduler-only tests, each named. The suite must climb back toward
   (and past, with better-targeted tests) the 1,878 baseline.
5. Standard openWEPP ExecPlan discipline applies (dual review, gate-evidence
   non-deferral, `.rs` line-count governance, full gates per package).

## Burndown — modules to revise, with coverage requirements

Each row is a candidate CQR work-package. `Status`: `todo` / `active` / `done`.

| # | Module(s) | Coverage requirements (contract/behavior) | Deleted-test source | Status |
|---|---|---|---|---|
| 1 | `runtime_inputs/` **soil parser** | FC/WP policy matches producer-corrected θ (`SC-SUBHYD`/HPHYS0202 lineage); harmonic vertical `ssc` below the top interval (BASECOND01, `SC-SUBHYD-001`); rock-corrected pore/`avpor`; soil parse input→typed-state | `08_tests/soil.rs` (894) | todo |
| 2 | `runtime_inputs/` **climate parser** | climate parse input→typed-state, breakpoint/hyetograph normalization, phase/temperature fields | `08_tests/climate.rs` (1070) | todo |
| 3 | `runtime_inputs/` **management + plant parser** | management/plant parse input→typed-state; senescence/decomposition params; the `Dec_*`/`Tah_*` distinctions | `08_tests/management.rs` (1455) | todo |
| 4 | `runtime_inputs/` **slope + core-types + irrigation** | slope geometry parse; core typed-state invariants; irrigation fixed-date parse | `08_tests/{slope,core_types,common,irrigation_fixeddate,snow_frost_irrigation}.rs` | todo |
| 5 | `hydrology/` **kernel water balance** | WB11–19 closure identities on the H2637 lane; runoff/infiltration/percolation/lateral/ET conservation; the typed phase-boundary result contracts | `tests_mod/hydrology.rs` (1492) | todo |
| 6 | `hydrology/`/`direct_runtime/` **growth + decomposition** | growth-state/`cancov` from biomass; decomposition/residue transitions; the residue-cover coupling (`SC-RESIDUE-001#INV-RESIDUE-019`) | `tests_mod/growth.rs` (368) | todo |
| 7 | **WB publication** (`direct_runtime`/runner) | WB11 seed, WB13, WB19/WB12/WB16 publication rows match the WB identities; PASS/WAT value/schema/row-count | `tests03/publication/{wb11_seed(2046),wb13,wb13_guard,wb19_wb12_wb16,scheduler_pl_activation}.rs` | todo |
| 8 | **per-OFE state / MOFE conservation** | MOFE per-OFE state conservation contract; per-OFE transfer/topology closure through >10 OFEs (FARPOINT01 lineage) | `tests03/per_ofe_state.rs` (766); `integration/mofe01_per_ofe_state_contract.rs` | todo |
| 9 | **typed kernel boundary** (new code from the migration) | the typed phase context / result / mutation APIs, typed diagnostic events, and typed publication events introduced by `a381702b` carry their own unit + contract coverage (they replaced `state_access.rs` symbol lookups) | new modules — no prior tests | todo |

### Retired with justification (delete, do NOT restore)

These validated the removed symbol-map runtime and are genuinely obsolete:

- `tests_mod/{writeback.rs (1875), day_frame.rs, boundaries.rs, phase.rs, schedule_export.rs}` — symbol writeback / scheduler phase / boundary export; the runtime they test is deleted.
- `tests03/trace.rs` (symbol-surface trace) and `integration/kernel_writeback_contract.rs` — superseded by row #9 (typed events/boundary).
- Any `*_runtime_surface_projects_*` / `*_runtime_surface_contains_canonical_state_symbols` cases inside otherwise-restored files — the symbol projection is gone; extract the *behavior* assertions into the row above and drop the projection assertions.

## Sequencing

Order by leverage and risk: **contract-bearing first** (rows 1, 5, 7, 8 — FC/WP,
conductivity, WB closure, MOFE conservation restore the ratified-contract coverage that
matters most), then the parsers (2–4), then growth (6), then the new typed-boundary code
(9). Each is an independent CQR package with dual review, contract-anchored coverage
requirements, and the net-coverage gate; the burndown is done when every row is `done`
and the suite is at or above 1,878 with better-targeted tests.
