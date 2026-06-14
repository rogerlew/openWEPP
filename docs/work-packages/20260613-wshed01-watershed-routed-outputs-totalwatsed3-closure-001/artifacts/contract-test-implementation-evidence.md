# Contract Test Implementation Evidence

Status: W-B executed

Evidence mode: Ran + Static

W-A authored no tests because the increment forbade production edits. W-B
implemented the required parser/CLI red tests.

W-B tests added:

- `tests/fixtures/infile/watershed_impoundment/strict_zero_impoundments.imp`
  with supported datver `99.1` and `jpond=0`.
- Parser assertions:
  - strict and compatibility success when
    `expected_structural_count=Some(0)`,
  - typed `IMP-E-007` mismatch when
    `expected_structural_count=Some(1)` and `jpond=0`,
  - bare strict parse still fails as `IMP-E-004`.
- Existing malformed-count, active-payload, and runtime-seed tests preserved.
- Watershed CLI regression:
  `watershed_cli_accepts_explicit_zero_impoundment_file_when_structure_has_none`
  proves the CLI accepts an explicit `jpond=0` input when structure has no
  impoundments and does not emit `CLIWAT-E-010`.

Red evidence:

- `cargo test --test infile_watershed_impoundment_parser_contract zero_impoundments`
  failed before production edits: three new assertions hit the old
  `DomainError { field: "jpond", allowed: ">= 1" }`.

Green evidence:

- `cargo test --test infile_watershed_impoundment_parser_contract`: `18`
  passed.
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract watershed_cli_accepts_explicit_zero_impoundment_file_when_structure_has_none`:
  `1` passed.
- `cargo test -p openwepp-watershed-orchestrator watershed_impoundment_runtime_seed`:
  `3` passed.

Required W-C tests:

- Assert all 14 watershed outputs exist.
- Assert `totalwatsed3.parquet` is not the one-row default writer surface for a
  real routed run.
- Assert reported depth columns match volume-derived depths.
- Add an anti-placeholder gate for required water-balance operands.
