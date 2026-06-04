# Diagnostic Evidence

Status: completed
Evidence mode: ran

Ran:
- `tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-EVAP-001.md`: failed with 11 findings.

Findings:
- `SCUNIT-E-009`: registry symbols `Ep` and `Er` are missing from `Variables and Units`.
- `SCUNIT-E-011`: registered aliases `hillslope_wat.Ep`, `hillslope_wat.Ep:mm`, `hillslope_wat.Es`, `hillslope_wat.Es:mm`, `hillslope_wat.Er`, and `hillslope_wat.Er:mm` are missing from `Symbol Alias Map`.
- `SCUNIT-E-004`: canonical `Es` row declares `m d^-1` while registry requires WAT output `mm`.
- `SCUNIT-E-008`: alias rows for `Er` and `Es` do not mention registry unit `mm`.

Interpretation: the defect is in `SC-EVAP-001` documentation coverage for final WAT publication depths. The executable registry already declares the WAT output symbols and units.
