# Pre-Implementation Contract Gate

Status: completed
Evidence mode: ran

Ran before contract edits:
- `tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-EVAP-001.md`: fail, 11 SC unit-compliance findings.

Gate result: red as expected. The package is authorized to amend canonical `SC-EVAP-001` documentation rows so the contract declares registry-covered WAT `Ep`, `Es`, and `Er` output units and aliases.
