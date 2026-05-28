# WSHEDIMPL39 Contract Implementation Evidence

Status: complete  
Evidence mode: static  
Date: 2026-05-28

## Static
- Updated canonical contract gap rows for WSHEDIMPL39 closure scope:
  - `SC-ROUTE-001` v41:
    - added runfile applicability validator authority binding
      (`REF-ROUTE-RUNFILE-APPLICABILITY`),
    - upgraded `INV-ROUTE-013` from governance-only posture to runtime
      fail-closed applicability declaration requirement,
    - `GAP-ROUTE-005` -> `closed`.
  - `SC-SYSTEM-001` v62:
    - ratified concrete ARCH22 alias mappings for active watershed integration
      boundaries,
    - `GAP-SYSTEM-002` -> `closed`,
    - `GAP-SYSTEM-001` -> `promotable-with-risk`.
  - `SC-IMPOUND-001` v12:
    - closed cross-contract maturity ambiguity row `GAP-IMPOUND-003`.
- Updated `docs/specifications/science-contracts/index.md` notes for
  `SC-ROUTE-001`, `SC-SYSTEM-001`, and `SC-IMPOUND-001` with WSHEDIMPL39
  traceability.
- Updated watershed runfile contract authority in
  `docs/contracts/openwepp-watershed-runfile-contract.md` with required
  `inputs.applicability` selectors and typed `CLIWAT-E-040` fail-closed
  semantics.

## Ran
- not-applicable
