# PERFDEEP01 — HillslopeDayFrame Scaffold (array-native Stage 0)

Status: scaffolded 2026-06-18 (Codex-ready). **Stage 0** of the array-native re-architecture
([ADR-0025](../../decisions/0025-array-native-hillslope-day-frame.md), ratified 2026-06-18;
design authority [`architecture/array-native-runtime-specification.md`](../../architecture/array-native-runtime-specification.md)).

Package type: **Foundation — build the frame + the identity round-trip + the migration ledgers. No phase
migrated, no perf win expected.** Its entire value is de-risking Stage 1: prove the typed frame is a
bit-identical representation, and front-load the three surfaces the Codex review flagged (publication
operands, dynamic guards, contract transition) so the hydrology island has **no hidden seams**.

## Why this exists — Stage 0 of the ratified program

Two incremental rungs (PERFMIG01/02) proved partial symbol/phase migration is dominated by
dual-representation bookkeeping. ADR-0025 adopts the comprehensive fix: replace the symbol-keyed
`BTreeMap<BoundarySymbol, BoundaryValue>` hot path with a typed dense **HillslopeDayFrame** that all 14
phases mutate in place; seams vanish by construction; logical surfaces survive only at the I/O edge. This
package builds the frame and the scaffolding **without migrating any phase** — the maps stay authoritative,
the frame runs beside them in shadow, and every output stays byte-identical.

## Scope — what Stage 0 delivers

1. **The `HillslopeDayFrame` type + field schema.** A typed, owned, dense per-OFE per-day struct. Every
   current `BoundarySymbol` scalar → a named **unit-typed field** (reuse the `BoundaryValue` unit newtypes
   — `WaterDepthMeters`, `TemperatureCelsius`, … — they are zero-cost and preserve dimensional safety; raw
   `f64` only where no unit type exists). Index-suffixed symbol families (`mofe_*_carry_0001..0024`,
   per-layer `wb14_*_0001..`, frost fine layers) → **fixed-width arrays / struct-of-arrays**, not N fields.
   Variable-length climate forcing → a borrowed read-only slice, not copied into the frame.
2. **Seed + flush, with bit-identical round-trip.** `seed`: logical `HillslopeWritebackSurface` →
   `HillslopeDayFrame`. `flush`: `HillslopeDayFrame` → logical surface. **Fixture: seed a frame from a real
   H2637 OFE-day state/flux surface, flush it back, assert `f64::to_bits()` equality on every symbol** (the
   frame is a faithful representation, losslessly). This is the load-bearing identity gate of Stage 0.
3. **Publication operand-lineage ledger (closes review Finding 2 — HIGH).** The WAT/PASS parquet rows are
   typed structs but are *assembled* from many runtime-surface symbol reads per OFE-day
   (`build_simulation_owned_wb13_row_for_ofe` and helpers read `prcp`, `tmax`, `wb13_profile_*`, `thetfc`,
   `dg`, `wb11_nsl`, frost/snow/runoff terms, …), plus the ~5 HBP scalars (`peakro`, `watdur`,
   `total_detachment_kg`, `total_deposition_kg`, `sediment_concentration_kg_m3_0001`) and manifest
   provenance. **Deliverable: an artifact enumerating every runtime-surface symbol the output/publication
   path reads, mapped to its `HillslopeDayFrame` field (or projection),** so no publication operand is
   stranded when the logical hot path is later deleted. The frame's I/O-edge typed capture is built/validated
   against this ledger.
4. **Two-tier guard catalogue + policy (closes review Finding 1 — HIGH).** Many writeback bounds are
   **runtime-derived**, not const (`maximum: Some(theta.iter().sum())`, `Some(carryover_total)`,
   `Some(layer_ul)`, `minimum: Some(soil_water_after - tol)` across erod19/plant_percolation/
   infiltration_evap/runoff_reconciliation). **Deliverable: an artifact cataloguing every writeback guard
   site as static-bound (→ compile-time field-invariant schema) or runtime-derived-bound (→ inline check at
   the write site), defining the two-tier guard policy** that preserves finite/domain/closure semantics,
   message-id classes, and diagnostic attribution exactly. No guard implementation is migrated in Stage 0 —
   this catalogues the policy the phase migrations will follow.
5. **Kernel-contract transition compatibility map (closes review Finding 3 — MED).** The kernel contract
   (`openwepp-kernel-contract`) is shared (hillslope + watershed) and map/payload-centric. **Deliverable: an
   artifact specifying how the frame coexists with the existing `HillslopeKernel` /
   `KernelWritebackPayload` contract during migration** (shadow beside the maps; the per-phase cutover
   interface; whether the contract is adapter-bridged or versioned-broken at cutover) so Stage 1+ is a
   planned interface change, not an ad-hoc multi-crate break.
6. **Shadow-run mechanism.** The harness that runs the frame **beside** the authoritative maps and asserts
   `to_bits()` agreement, for use by every later stage's per-phase identity gate. Decide the mechanism (a
   compile-time feature flag running both paths vs a test-harness-only differential — spec §12 fork 5) and
   document it.

**Explicitly NOT in scope (Stage 0):** migrating any of the 14 phases to the frame; making the frame
authoritative on the production hot path; deleting any logical surface; any expected perf change. The frame
is **shadow/test-only or feature-flagged off** — the production binary's behavior and endpoint are
unchanged.

## Honest-measurement discipline (carried from the perf arc)

- **Bit-identity is the Stage-0 gate, not speed.** The round-trip fixture (§2) + H2637 output identity
  (`.hbp`/`wat`/`pass` unchanged) are the acceptance. There is no perf claim in Stage 0.
- **No production hot-path regression.** Because the frame is shadow/flagged-off, the H2637 endpoint must
  stay flat vs PERFMIG01 (669.97 s) — confirm it, don't assume it. If the scaffold leaks onto the hot path
  and regresses, that's a Stage-0 defect.
- **The ledgers must be complete, not representative.** A partial publication-operand or guard catalogue
  re-creates the PERFMIG02 trap (a missed operand = a stranded logical read at deletion time). Completeness
  is the deliverable; `log()` any symbol deliberately deferred with the reason.

## Acceptance Criteria

- `HillslopeDayFrame` type + field schema defined; every current scalar symbol and array family has a typed
  home; climate forcing borrowed, not copied.
- `seed`/`flush` implemented; **round-trip `to_bits()` identity fixture green on a real H2637 OFE-day
  surface** (every symbol).
- Publication operand-lineage ledger artifact complete (every output/publication runtime-surface read →
  frame field/projection).
- Two-tier guard catalogue artifact complete (every writeback guard site classified static vs
  runtime-derived; the policy stated).
- Kernel-contract transition compatibility map artifact complete.
- Shadow-run mechanism implemented + documented.
- **H2637 `.hbp`/`wat.parquet` byte-identical, `pass.parquet` Arrow-equal; endpoint flat vs 669.97 s**
  (frame is shadow/flagged-off — no production behavior change).
- Workspace Rust gates (fmt, check, clippy `-D warnings`, test) + `cargo deny` + determinism + markdown
  lint green.

## Deliverables

- the `HillslopeDayFrame` type + schema + seed/flush + shadow harness (production-adjacent Rust, frame
  shadow/flagged-off)
- `artifacts/perfdeep01-frame-schema.md` (the field schema: symbol→field mapping, the SoA array families)
- `artifacts/perfdeep01-roundtrip-identity.md` (the seed/flush `to_bits()` round-trip evidence)
- `artifacts/perfdeep01-publication-operand-ledger.md` (Finding-2 closure: every output read → frame field)
- `artifacts/perfdeep01-guard-tier-catalogue.md` (Finding-1 closure: static vs runtime-derived guard policy)
- `artifacts/perfdeep01-contract-transition-map.md` (Finding-3 closure: frame↔contract coexistence + cutover)
- `artifacts/perfdeep01-endpoint-rss.md` (H2637 unchanged + endpoint flat confirmation)
- `artifacts/perfdeep01_disposition.md` (Stage-0 complete → Stage-1 hydrology-island go, or blockers found)

## Execution Result

(pending Codex execution)

## Dependencies

- [`docs/architecture/array-native-runtime-specification.md`](../../architecture/array-native-runtime-specification.md) — **binding design authority** (frame design §4, I/O edge §5, identity §7, stages §8, forks §12)
- [`docs/decisions/0025-array-native-hillslope-day-frame.md`](../../decisions/0025-array-native-hillslope-day-frame.md) — the ratifying ADR (required gates)
- `docs/work-packages/20260618-perfarch03-full-array-native-floor-prototype-001/` — the 543+8 `to_bits` fixture seed; the floor
- `docs/work-packages/20260618-perfmig02-...-001/artifacts/review-claude-independent.md` — why incrementalism died (the trap Stage 0 front-loads against)
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/{00_symbol_registry_and_indexed_surfaces,02_boundary_values_and_kernel_requests}.rs` — `BoundaryValue` unit types, the contract to coexist with
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` (`HillslopeWritebackSurface` 252-256; `OfeLanePersistentState`) — the surface to seed/flush
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs` (`build_simulation_owned_wb13_row_for_ofe`, `build_hbp_output`, `require_runtime_surface_scalar`) — the publication-operand surface to ledger
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/**` + `support_helpers_mod/` — the writeback guard sites to classify
- `AGENTS.md`; `crates/AGENTS.md`; `docs/numerics/README.md`; `docs/work-packages/AGENTS.md`

## Subagent Requirement

Recommended: one `Explore` read-only pass to enumerate (a) the complete output/publication runtime-surface
read set (for the operand ledger) and (b) every writeback guard site with its bound expression (for the
guard catalogue), since both ledgers must be **complete**, not sampled.

## Autonomy

Execute end-to-end: define the frame + schema, implement seed/flush with the round-trip identity fixture,
produce the three ledgers (publication operands, guard tiers, contract transition), build the shadow-run
mechanism, and confirm H2637 output identity + flat endpoint. **Stage 0 has no perf deliverable — the gate
is bit-identity + complete ledgers.** Do not migrate a phase or touch the production hot path's authority;
that is Stage 1 (PERFDEEP02, the hydrology island). A complete, faithful frame + complete ledgers is the
deliverable; an incomplete ledger is a Stage-0 failure, not a Stage-1 problem.
