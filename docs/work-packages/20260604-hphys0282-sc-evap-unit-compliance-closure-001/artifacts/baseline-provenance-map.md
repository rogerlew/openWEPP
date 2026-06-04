# Baseline Provenance Map

Status: completed
Evidence mode: static

Static:
- This package does not change process physics and does not require new legacy equation migration.
- Registry authority is `crates/openwepp-sim-contract/src/units.rs`, where WAT output `Ep`, `Es`, and `Er` are registered as `mm` publication depths with `hillslope_wat.*` aliases.
- Contract authority is `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`; the required remediation is alias/unit documentation alignment, not a kernel behavior change.
