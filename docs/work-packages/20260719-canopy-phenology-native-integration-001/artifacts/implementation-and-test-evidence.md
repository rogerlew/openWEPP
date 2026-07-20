# Implementation And Test Evidence

Evidence mode: `Ran`

Status: `focused implementation gates pass; terminal campaign pending`

Implemented surfaces:

- strict native YAML phenology authority and parser projection;
- standalone typed GSI-to-canopy realization with uninitialized cold start;
- continuous per-lane direct-runtime GSI state;
- typed growth override, daily allocation/litter ledger, and same-day residue
  handoff; and
- exact consumed-value observations for frost residue depth and erosion canopy.

Ran on 2026-07-20:

```text
cargo test -p openwepp-hillslope-orchestrator --lib r7b_constructor_type_size_layout_is_bounded
cargo test -p openwepp-runner --lib
cargo test -p openwepp-plant-phenology
cargo test -p openwepp-management-schema -p openwepp-input-contract -p openwepp-landuse-migrate
cargo test --test infile_management_parser_contract --test infile_management_yaml_contract
cargo test --test testgate_align_authority_contract production_
cargo clippy -p openwepp-plant-phenology -p openwepp-management-schema -p openwepp-input-contract -p openwepp-landuse-migrate -p openwepp-hillslope-orchestrator -p openwepp-runner --all-targets -- -D warnings
```

Results: all listed commands passed. The runner result was 132/132 tests; the
plant package result was 12 unit, 6 canopy-contract, 1 restart, and 0 doctest
failures. Management/input/migration and the two integration targets passed.
The terminal TESTGATE receipt remains required after verification and contract
promotion.
