# Parser-to-Runtime Integration Evidence

Evidence mode: `Ran`
Status: `complete`

## End-to-End Integration Tests

### `parser_to_hillslope_runtime_surface_closure`
- parses `valid_9002.sol` (`parse_soil`)
- adapts parser output through `build_hillslope_runtime_surface_from_soil`
- executes `HillslopePhaseScheduler::execute_with_kernel`
- kernel probe asserts parser-derived runtime symbols are present in every phase request:
  - `solthk=0.25`
  - `dg=0.1`
  - `thetdr=0.05`
  - `thetfc=0.31`

### `parser_to_watershed_runtime_surface_closure`
- parses `strict_valid.chaninp` (`parse_chaninp_from_str`)
- adapts parser output through `build_watershed_runtime_surface_from_chaninp`
- executes `execute_watershed_dispatch_with_kernel`
- kernel probe asserts parser-derived runtime symbols are present in every dispatch request:
  - state: `ipeak=3`, `nchan=2`, `dtchr=600`, `ntchr=144`, `nchnum=2`
  - flux: `cbase=0.000001`

## Acceptance Test for Root-Masking Risk
`workspace_integration_ownership_acceptance` verifies:
1. both orchestrator crate manifests declare direct `openwepp-input-contract` dependencies;
2. root crate remains non-reexport aggregator (no `pub use openwepp_*` masking path).

## Command Evidence
- `cargo test --test parser_runtime_seam_integration --test workspace_integration_ownership_acceptance`
  - result: 4/4 tests passed
- `cargo test --workspace`
  - result: full workspace pass, including new seam tests and adapter unit tests.

## Evidence
- [DIRECT] `tests/integration/parser_runtime_seam_integration.rs:76-131`
- [DIRECT] `tests/integration/workspace_integration_ownership_acceptance.rs:7-27`
- [DIRECT] command output logs captured in ARCH17 execution transcript.
