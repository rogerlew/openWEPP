# MOFE02 Pre-Implementation Contract Gate

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Contract authority audit completed before production edits.
- Canonical parity invariants already exist and are explicit in:
  - `docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md` (`SLP-E-007`, cross-file constraints)
  - `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md` (`SOL-E-007`, hillslope `ntemp == nofe` constraint)
  - `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md` (`MAN-E-007`, topology-count closure)
- No canonical `SC-*` amendment was required for MOFE02 because authority and typed mismatch taxonomy were already present.
- Contract-derived tests were authored before production runner edits in:
  - `tests/integration/cli03_runner_contract_derived_tests.rs`

## Ran
- `cargo test -p openwepp --test cli03_runner_contract_derived_tests mofe02 -- --nocapture`
- Pre-implementation result: `FAILED` (4/4 MOFE02 tests failed) against baseline behavior, confirming missing production parity gate and missing hillslope soil-topology wiring.
