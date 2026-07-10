# Contract Disposition

Status: `EXECUTED`

Evidence mode: `Static` (contract reads + amendment authoring) plus `Ran`
(binding-exposure lint; reference verification commands recorded in
`authority-matrix.md`).

Authority basis: `artifacts/authority-matrix.md` (all rows `RATIFIABLE`; the
two `DECISION-REQUIRED` items resolved inside the amendment as recorded
below).

## Per-contract decisions

| Contract | Decision | Content |
|---|---|---|
| `SC-ROUTE-001` | **AMENDED** (v50 → v51; v52 in the Codex post-hoc cycle, see `codex-review-disposition.md`) | Eight new authority anchors (`REF-ROUTE-CH13-GEOMCARRY`, `REF-ROUTE-CREAMS-CH3-QS`, `REF-ROUTE-CREAMS-CH3-WIDEN`, `REF-ROUTE-ARS77-SAMEGRID`, `REF-ROUTE-HECRAS-QUS`, `REF-ROUTE-CH14-TIMESTEP`, `REF-ROUTE-GULLY-STATE`, `REF-ROUTE-JIMF2023-CARRY`); six new invariants `INV-ROUTE-015..020` with guard-map rows; `INV-ROUTE-005(e)` made conditional on interval-lane activation; `BEI-ROUTE-007`; `TOL-ROUTE-006..008`; three new allowed degenerate states + four new invalid states; the W11A Channel-Interval Sediment Sequencing Addendum (activation rule, operand table, sequencing steps, widening clock; eleven test-vector obligations as of v52); `GAP-ROUTE-012/013` (+ `GAP-ROUTE-014` in v52); revision rows v51/v52. |
| `SC-SED-001` | **NO AMENDMENT** | The interval lane changes only the channel-side consumer. Producer semantics (`hourly_sediment_mass_kg`, `INV-SED-013/014`, per-quantum enrichment `INV-SED-017`) are untouched. The class-fraction timing rule the channel consumes is already governed by `GAP-SED-008` in its E.4-narrowed interchange scope, whose consumer clause ("must not treat the uniform split as enriched timing") `INV-ROUTE-019` now cites explicitly. Amending SC-SED-001 would duplicate authority across contracts. |
| `SC-SYSTEM-001` | **NO AMENDMENT** | `INV-SYSTEM-009` (channel sediment continuity, no untracked mass across handoffs) already binds the system level and is satisfied — not modified — by the per-interval closure chain (`INV-ROUTE-019/020` are strictly tighter). No system-level payload, manifest, or ordering surface changes: the interval lane consumes existing routed water state and the already-carried per-hour inlet sediment array. |
| `SC-INFILE-HBP-001` | **NO AMENDMENT** (package exclusion holds) | Authority did **not** prove true per-hour enriched class state is mandatory: the ratified interval lane consumes the existing minor-1 `V_h`/`S_h` total-mass surfaces with the day-level class blend (GAP-SED-008 interchange scope). No schema change; the per-class-hourly channel remains a future additive extension decided with its own driver (per ADR-0036 D2). |

## Resolution of the two authority-matrix `DECISION-REQUIRED` items

| Item | Resolution | Where |
|---|---|---|
| Sub-threshold trickle constant | Resolved **without a new constant**: the zero-flow interval floor reuses the existing routed-closure floor `q1(it) <= 1e-12 m^3 s^-1` from `INV-ROUTE-007`; a sediment-specific flow threshold is declared invalid. | `INV-ROUTE-020(a)/(b)` |
| End-of-grid storage sediment disposition | On the ratified quasi-steady lane the question is **moot by construction** (each interval closes without a storage term; zero storage-attributable mass); **deposit-at-grid-end** is ratified for the recorded unsteady **fallback lane only** (lineage-consistent with the REF-ROUTE-JIMF2023-CARRY defect posture; divergent HEC-RAS suspended-carry precedent recorded with the revisit condition). | `INV-ROUTE-020(c)` + `GAP-ROUTE-013` |

## Registry

`docs/specifications/science-contracts/index.md` `SC-ROUTE-001` row
`last_reviewed` updated `2026-05-28` → `2026-07-10` (lifecycle metadata only).

## Validation

Ran `.venv/bin/python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`:
`PASS ... 7 binding exposure row(s) fully consolidated`.

## Non-goals honored

- No Rust production implementation (package exclusion).
- No surrogate/provisional physics: every ratified rule carries a named
  external or lineage anchor; the one deliberate refinement beyond legacy
  source-intent (per-interval quasi-steady sequence) is labeled as such in
  `INV-ROUTE-016` with its recorded fallback, mirroring the ADR-0036 D1
  pattern.
- No impoundment sediment routing changes (Chapter 14 cited as precedent
  only).
