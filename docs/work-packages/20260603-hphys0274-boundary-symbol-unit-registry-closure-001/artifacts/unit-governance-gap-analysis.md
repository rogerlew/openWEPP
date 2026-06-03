# Unit Governance Gap Analysis

Status: completed
Evidence mode: static

Static: HPHYS0274 closes the first registry implementation gap and leaves the
remaining HPHYS0275-HPHYS0279 unit-governance remediation queue explicit.

Ran: not-run.

## Closed Gap

- Machine-readable unit registry now exists in
  `crates/openwepp-sim-contract/src/units.rs`.
- Registry validator rejects missing/ambiguous unit rows.
- Mandatory local gate exists at `tools/release/check_unit_registry.sh`.
- High-risk hydrology, snow/freeze, ET, climate, soil, percolation, and WAT
  publication symbols have initial registry coverage.

## Residual HOLD Gaps

- Full repository symbol coverage is incomplete outside the HPHYS0274 touched
  scope.
- Automated source scanning of every runtime producer and every `SC-*` alias
  table is not complete; HPHYS0279 owns full contract-lint/source-scan
  enforcement.
- Most dimensional runtime aliases still travel as `BoundaryValue::scalar`;
  HPHYS0275 must add or apply typed dimensional boundary values.
- Raw dimensional conversion literals remain outside registry enforcement;
  HPHYS0276 must add named conversion helpers and guards.
- Hourly radiation still needs physical flux guard enforcement under HPHYS0277.
- WAT output metadata is not yet generated from the registry; HPHYS0278 must
  align writer metadata with registry authority.
- Contract unit-section linting remains queued under HPHYS0279.

These are explicit continuation gaps and do not imply HPHYS0274 silently
covered all runtime symbols.
