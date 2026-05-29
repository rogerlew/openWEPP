# HILLSTAB07 Disposition

Status: complete  
Evidence mode: mixed (`Static` + `Ran`)

## Decision
- decision: HOLD
- date: 2026-05-29
- reason: WB16 contract authority and compatibility-seed observability gaps are
  closed, but full baseline-authoritative `ealpha` producer-chain migration is
  still open.

## Objective Closure
- Closed in this package:
  1. Canonical WB16 `m`/`ealpha` provenance authority gaps in
     `SC-RUNOFFPART-001` and `SC-WATBAL-001`.
  2. Silent compatibility-seed behavior for WB16 `ealpha` (runner now publishes
     explicit manifest provenance and warning `SIMPIPE-W-003`).
  3. Contract-derived test coverage for provenance publication path.
- Not closed in this package:
  - Full canonical `ealpha` producer migration
    (`frcfac -> rdat(alpha) -> alphay -> eplane`) into production runtime.

## Hold Register
- `GAP-RUNOFFPART-005`: open (non-promotable).
- `GAP-WATBAL-005`: open (non-promotable).

## Closure Statement
- HILLSTAB07 is execution-complete and correctly dispositioned `HOLD` pending a
  follow-on package that ports the full baseline-authoritative `ealpha`
  producer chain without heuristic substitutions.
