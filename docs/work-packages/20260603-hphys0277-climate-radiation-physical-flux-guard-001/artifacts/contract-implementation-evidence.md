# Contract Implementation Evidence

Status: completed
Evidence mode: static

Static: HPHYS0277 contract authority was implemented before production code
edits.

Ran: not-run; this artifact records document edits and source-level evidence.

## Contract Amendments

- `SC-CLIMATE-001` `contract_version` advanced from `16` to `17`.
- `INV-CLIMATE-013` now requires finite hourly radiation to fail closed when
  `hradmj` exceeds the `radcur.for`-derived physical hourly extraterrestrial
  bound.
- `OBL-CLIMATE-P-009` records the production guard obligation for
  `winter.hourly.rad_mj_m2_####` / `snow.hourly.radmj_####` publication.
- `TOL-CLIMATE-005` limits tolerance to explicit floating-point roundoff only.
- The SIMIMPL28 deterministic requirement and verification-vector sections now
  include the HPHYS0277 high-flux typed error requirement.
- Revision history records the HPHYS0277 amendment as version `17`.

## Governance Amendments

- `docs/specifications/science-contracts/index.md` now notes that HPHYS0277
  extended `INV-CLIMATE-013` with the `radcur.for` physical flux bound.
- `docs/specifications/units/boundary-symbol-unit-registry.md` now records the
  HPHYS0277 radiation guard as implemented for first-wave SIMIMPL28 winter
  hourly radiation.

## Contract-First Sequence

Static: contract and governance text was edited before production runtime guard
implementation, matching the required sequence in `package.md`.
