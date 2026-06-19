# PERFMIG01 - WB11 Runoff Array-Authoritative Production Migration (first rung)

Status: executed 2026-06-18 (`CONTINUE`). The first **production** rung of the
array-authoritative re-architecture — gated GO by PERFARCH03's measured floor.

Package type: **Production migration — the first real rung, not a prototype.** PERFARCH03 proved the
floor; this package starts realizing it in production code, beginning with the one branch PERFARCH03
already validated (WB11 runoff), under exact-identity + endpoint-timing gates.

## Why this exists — the floor is GO, now realize it

The target — **≤10× (ideally ≤5×)** vs legacy on H2637 — is a **viability gate**. Current endpoint:
**73.12×** (666.82 s; legacy 9.12 s). PERFARCH03 (commit `b4ac1f7b`, GO) measured the decisive number
the program had never run: a **fully array-native** WB11 warm-rain runoff branch hot loop at
**0.959423 µs/OFE-day (0.0248× legacy)** — **146.8× faster than the *real* production
`Wb11HydrologyKernel` on the *same* branch** (140.83 µs), validated to exact `f64::to_bits()` identity on
**543 state + 8 flux** outputs, perf-verified logical-free, RSS ~3 MB. **The 73× is symbol machinery, not
physics; the physics floor is ~1 µs, far below both budgets.** PERFARCH03 was a measurement instrument
(artifact-local, no production change). This package is the first production migration rung.

## What this migrates — the WB11 runoff writeback path to dense array authority

Per the proposed **ADR-0023** (revive + ratify in this package): move the hot path from logical-map
authority + indexed mirror to **dense array authority keyed by `SymbolId`**. The first rung is the
PERFARCH03-validated surface:

- production WB11 runoff: `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_runoff_reconciliation.rs`
  + `Wb11HydrologyKernel` (`src/hydrology/03_kernel_support_00_support_helpers.rs`);
- the kernel writes its outputs to **dense `SymbolId` slots** instead of building a
  `BoundarySymbol`-keyed logical `KernelWritebackPayload`;
- `HillslopeWritebackSurface` remains the logical contract/publication surface — materialized at
  boundaries, **not** the daily mutable kernel-loop store;
- finite/domain guard semantics, fail-closed behavior, and diagnostic message-id classes stay
  **equivalent** to the current logical writeback path (resolved on failure/publication, not on the
  success hot path).

## The single-phase boundary trade-off (a design call for Codex, not a dictate)

A single migrated phase still hands off to unmigrated downstream phases that read logical. So the first
rung carries a **transitional materialize boundary** at its output edge — the same 108 µs/OFE-day
one-shot cost PERFARCH03 isolated. Two honest consequences the package must confront, **not** paper over:

- **The first-rung H2637 endpoint delta may be modest or even slightly negative** if the transitional
  boundary offsets a single phase's physics win. That is expected and is **not** a NO-GO — the win
  compounds as adjacent phases migrate and the internal boundaries collapse (the boundary moves outward
  toward I/O, where it already exists).
- Therefore the **rung granularity is itself a design decision**: migrate one phase (cleanest identity
  unit, but a transitional boundary partly masks the win) vs a **contiguous cluster** of the per-OFE-day
  hydrology chain (real endpoint signal because internal boundaries vanish, but larger blast radius).
  **Codex chooses the granularity** that produces an honest, interpretable endpoint signal while keeping
  identity tractable. Whatever the choice, the transitional boundary is **measured separately** and named
  as retired-on-future-rungs, never silently folded into the floor.

## Honest-measurement discipline (non-negotiable — carried from PERFARCH03 + the perf-arc lessons)

- **Bit-identity per flipped flow.** Every migrated branch is `to_bits()`-identical to the current
  logical path (the PERFARCH03 543+8 fixture is the seed; extend to every branch the rung touches —
  warm-rain is validated, but the production phase also has snow/frost/melt branches that this rung must
  either migrate-and-validate or explicitly leave logical with a named boundary).
- **Real H2637 endpoint timing + RSS after the rung**, same machine/fixture as PERFIDX06 (73.12×). The
  endpoint is the gate — the prototype branch floor proved it's *worth* doing; the endpoint proves it's
  *working*. Report the net, the transitional-boundary cost separately, and the projection-to-budget as
  boundaries collapse.
- **No normal-path logical+array dual-write** (the PERFIDX05 ceiling) and **no daily full `BTreeMap`
  export at the kernel seam** (the PERFIDX03 seam). Dense is the single authority on the hot path.
- **No logical payload construction / `from_logical_payload` inside a migrated hot loop.** Logical
  resolves only on failure and publication paths.
- **Determinism preserved**; the existing determinism + conservation gates stay green.

## The decision (the deliverable)

1. **ADR-0023 ratified** (array-authoritative hot-path state), committed to `docs/decisions/0023-*.md`
   (number is reserved/free), superseding the hot-path-authority portion of ADR-0022 while keeping its
   registry/`SymbolId`/sorted-order foundation.
2. **WB11 runoff migrated** to dense array authority in production, bit-identical, gates green.
3. **H2637 endpoint re-measured** after the rung (vs 73.12×), with the transitional-boundary cost broken
   out and a credible projection of the full migration's trajectory to ≤10×/≤5× as boundaries collapse.
4. **CONTINUE / REDIRECT call:** CONTINUE (the pattern is identity-clean and the trajectory projects to
   budget → scaffold the next rung) or REDIRECT (a named obstacle the single-phase prototype didn't
   surface — e.g. a guard/transfer-helper authority that resists dense ownership — with the specific
   redirect). A modest first-rung endpoint delta is **not** a REDIRECT; a broken identity or an
   un-retireable boundary is.

## Scope

In scope: revive + ratify ADR-0023; migrate the WB11 runoff writeback path to dense `SymbolId` authority
in production; per-branch bit-identity; H2637 endpoint timing + RSS; the transitional-boundary accounting;
the CONTINUE/REDIRECT decision + (if CONTINUE) the next-rung outline.

Out of scope:

- **No broad multi-phase migration** beyond the chosen first-rung granularity — each subsequent rung is
  its own package, gated on this rung's endpoint + identity.
- No science-contract changes, no HBP/parquet schema changes, no irrigation activation (ADR-0023
  Non-Decisions), no clean-room/physics changes — this is a state-representation flip, byte-identical
  results.
- No removal of logical surfaces from public/reporting/diagnostic boundaries.

## Acceptance Criteria

- ADR-0023 ratified + committed (`docs/decisions/0023-*.md`); ADR-0022 foundation preserved.
- WB11 runoff writeback migrated to dense array authority in production; **every branch the rung touches
  is `to_bits()`-identical** to the current logical path (warm-rain seeded by PERFARCH03's 543+8 fixture;
  other touched branches validated or explicitly left logical with a named boundary).
- No normal-path dual-write; no daily `BTreeMap` export at the kernel seam; no logical-payload
  construction in migrated hot loops (perf or static evidence).
- H2637 endpoint timing + RSS recorded same-machine vs PERFIDX06, transitional boundary broken out,
  trajectory projection stated.
- Workspace Rust gates (fmt, check, clippy `-D warnings`, test) + determinism/conservation gates green on
  the touched scopes; markdown-doc lint green.
- CONTINUE/REDIRECT decision with numbers, not assertions.

## Deliverables

- `docs/decisions/0023-array-authoritative-hot-path-state.md` (ratified from the PERFARCH02 proposed text)
- `artifacts/perfmig01-migration.md` (what was flipped; the dense authority shape; the boundary placement)
- `artifacts/perfmig01-bit-identity.md` (every touched branch validated identical; the fixture set)
- `artifacts/perfmig01-endpoint-timing.md` (H2637 s + ratio + RSS vs PERFIDX06; transitional boundary
  broken out; trajectory projection)
- `artifacts/perfmig01-logical-free-proof.md` (no dual-write / no seam export / no hot-loop logical payload)
- `artifacts/perfmig01_disposition.md` (CONTINUE + next-rung outline / REDIRECT + the named obstacle)

## Execution Result

PERFMIG01 ratified ADR-0023 and migrated the production WB11 warm-rain runoff
writeback branch to id-backed dense authority. The migrated branch emits a
543-state + 8-flux `IndexedKernelWritebackPayload` and returns an empty logical
payload on the success path; active snow, frost, irrigation, and MOFE hourly
carry branches remain explicitly logical boundaries.

Focused tests proved exact materialized map equality and exact `f64::to_bits()`
identity against the legacy logical payload. The H2637 no-UI endpoint rerun
completed with rc `0` at `669.97s`, `228144 KB` RSS, versus PERFIDX06
`666.82s`, `228508 KB`. Outputs remained semantically identical (`.hbp` and
`wat.parquet` byte-identical; `pass.parquet` Arrow-equal with metadata ignored;
`loss.json` and `plot.parquet` differed only by isolated runfile `run_name`).

The transition-boundary harness measured production id-backed writeback apply at
`107.531649 us/payload`, projected to `25.373275s` across the full H2637 OFE-day
count. The endpoint regression is therefore a named, retireable single-rung
boundary-offset result, not a redirect. Verdict: `CONTINUE`; next rung should
migrate a contiguous WB11-consumer cluster so the compatibility materialization
boundary moves outward.

## Dependencies

- `docs/work-packages/20260618-perfarch03-full-array-native-floor-prototype-001/` — the GO floor
  (0.959423 µs/OFE-day; 543+8 identity fixture; boundary 108 µs; the constraints this rung inherits)
- `docs/work-packages/20260618-perfarch02-array-authoritative-hot-path-state-redesign-001/artifacts/perfarch02-proposed-adr.md`
  — the ADR-0023 text to ratify; the 49.9× writeback/guard prototype signal
- `docs/work-packages/20260618-perfidx06-high-ofe-target-assessment-001/artifacts/{perfidx06-bottleneck-analysis,perfidx06-legacy-ratio}.md`
  — the 73.12× endpoint, the budgets, the same-machine timing protocol
- `docs/decisions/0022-indexed-runtime-surface-representation.md` — the registry/`SymbolId` foundation ADR-0023 preserves
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_runoff_reconciliation.rs`
  + `src/hydrology/03_kernel_support_00_support_helpers.rs` (`Wb11HydrologyKernel`) — the migration surface
- `AGENTS.md`; `crates/AGENTS.md`; `docs/numerics/README.md`; `docs/work-packages/AGENTS.md`

## Subagent Requirement

None required for the migration + measurement. If the rung granularity decision needs a survey of which
downstream phases read the WB11 runoff outputs (to place the transitional boundary), a read-only Explore
pass over `kernel_phases_mod/` is sufficient.

## Autonomy

Execute end-to-end: ratify ADR-0023, migrate the WB11 runoff writeback path to dense array authority,
validate per-branch identity, re-measure the H2637 endpoint + RSS, account for the transitional boundary,
and make the CONTINUE/REDIRECT call. **The endpoint number + the identity proof are the deliverables.** A
modest first-rung endpoint delta is expected and acceptable if the pattern is identity-clean and the
trajectory projects to budget; do not over-claim H2637 closure from one rung, and do not treat a
transitional-boundary offset as a NO-GO.
