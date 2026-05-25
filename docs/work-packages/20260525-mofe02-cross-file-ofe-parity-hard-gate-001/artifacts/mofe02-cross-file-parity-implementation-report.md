# MOFE02 Cross-File Parity Implementation Report

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
Implementation summary:
- `crates/openwepp-runner/src/hillslope/mod.rs`
  - Reordered intake parsing to resolve `slope` and `management` before `soil`.
  - Wired soil parser options with hillslope topology guard when authoritative counts align:
    - `expected_topology_count: Some(slope.ofe_count)` when `slope.ofe_count == management.topology_count`.
    - `topology_scope: Some(Hillslope)` for that path.
  - Added explicit triad parity validator before runtime surface merge:
    - `validate_hillslope_ofe_topology_parity(slope, management, soil)`.
- `crates/openwepp-runner/src/errors.rs`
  - Added typed runner error variant `OfeTopologyMismatch`.
  - Added stable error code `CLIHILL-E-019`.
  - Added deterministic mismatch-pair text in error message (`slope-management`, `slope-soil`, `management-soil`).
- `tests/integration/cli03_runner_contract_derived_tests.rs`
  - Added four MOFE02 contract-derived tests and fixture mutators for one/two-OFE slope and `ntemp`-controlled soil fixtures.
- `tests/fixtures/cli01/hillslope_run_dir/case.slp`
- `tests/fixtures/cli01/hillslope_run_dir_unknown/case.slp`
  - Normalized baseline CLI01 fixtures to one-OFE topology to keep unrelated runner conformance tests aligned with new hard parity gate.

## Ran
- Targeted MOFE02 tests passed.
- Full workspace gates passed (see `gate-results.md`).
