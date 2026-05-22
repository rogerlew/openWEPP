# Parser-to-Simulation Seam Ownership Contract

Evidence mode: `Static`
Status: `complete`

## Scope
ARCH17 closes the parser-to-runtime seam for representative hillslope and watershed inputs with explicit adapter ownership and typed failure surfaces.

## Ownership Boundary

| seam_id | parser producer | adapter owner | runtime consumer | runtime symbols | contract policy |
|---|---|---|---|---|---|
| `HS-SEAM-001` | `openwepp_input_contract::parsers::soil::parse_soil` | `openwepp-hillslope-orchestrator::runtime_inputs::build_hillslope_runtime_surface_from_soil` | `HillslopePhaseScheduler::execute_with_kernel` | `solthk`, `dg`, `thetdr`, `thetfc` | Missing/invalid runtime fields fail via `HS-RUNTIME-E-*`; no defaults. |
| `WS-SEAM-001` | `openwepp_input_contract::parsers::chaninp::parse_chaninp_from_str/path` | `openwepp-watershed-orchestrator::runtime_inputs::build_watershed_runtime_surface_from_chaninp` | `execute_watershed_dispatch_with_kernel` | `ipeak`, `nchan`, `dtchr`, `ntchr`, `nchnum`, `cbase` | Non-`ParsedBranch` outcomes and invalid values fail via `WS-RUNTIME-E-*`; no compat fallback promotion. |

## Enforced Invariants
1. Runtime adapter surfaces are strict and typed; missing required parser fields are explicit errors.
2. Compatibility parser branches are not silently elevated to runtime authority.
3. Canonical WEPP symbol continuity is preserved at adapter boundaries (`solthk`, `dg`, `thetdr`, `thetfc`, plus channel symbols carried by parser variable names).
4. Runtime ownership remains in orchestrator writeback surfaces; parser and adapter code never mutate kernel-owned state directly.

## Root-Masking Closure
Direct crate dependencies were added from both orchestrator crates to `openwepp-input-contract`, removing root-crate-only dependency visibility for the implemented seam.

## Evidence
- [DIRECT] `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:10-183`
- [DIRECT] `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs:10-170`
- [DIRECT] `crates/openwepp-hillslope-orchestrator/Cargo.toml:18-22`
- [DIRECT] `crates/openwepp-watershed-orchestrator/Cargo.toml:18-22`
- [DIRECT] `tests/integration/workspace_integration_ownership_acceptance.rs:7-27`
