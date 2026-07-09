# Consumer Path Plan

Status: implemented and tested.

Proof chain:

1. Fixture YAML:
   `tests/fixtures/infile/management/canonical_forest_nonzero_ow_lanuse_1.man.yaml`.
2. Shared schema/parser:
   `crates/openwepp-management-schema/src/lib.rs`.
3. Input-contract adapter:
   `crates/openwepp-input-contract/src/parsers/management.rs`
   `parse_management_document_from_path`.
4. Real runner intake:
   `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
   calls `parse_management_document_from_path` for management input paths.
5. Normalized runtime model:
   YAML converts into existing `ManagementParseOutput`.
6. Existing PL projection:
   `build_hillslope_pl_runtime_surfaces_from_management`.
7. Runtime proof:
   `tests/integration/infile_management_yaml_contract.rs` asserts YAML-derived
   route coefficients project to `ofe1_route_*` and slotted PL schedule route
   symbols.

Negative proof posture:

- The YAML fixture is parsed directly by extension dispatch.
- No source `.man`, migration report, or optional sidecar is read by the test.
- Route coefficients must be present in the YAML plant record; missing
  coefficients fail schema validation.

Executed evidence:

- `cargo test --test infile_management_yaml_contract`
- `cargo clippy --test infile_management_yaml_contract -- -D warnings`
