# SNOWDENSITY-05C Albedo State Core

Status: complete.

Package type: contract/albedo-state-core implementation package.

Primary contract: `SC-SNOWFREEZE-001`.

Closure: COMPLETE-05C-ALBEDO-STATE-CORE.

Objective: ratify and implement the standalone opt-in snow-albedo state core
required before `coe_shortwave_albedo_v1` can be wired into production melt.
The implementation owns Brock-2000 style temperature/age albedo state,
fresh-snow reset, bounded albedo domain, model id/provenance, and fail-closed
state requirements for the future opt-in melt path.

No routed-melt acceptance or `coe_shortwave_albedo_v1` production wiring is
performed by SNOWDENSITY-05C. `legacy_coe` remains the default and does not
require, mutate, or consume the albedo state.

## Decision Boundaries

- Accepted albedo state id is `brock2000_temperature_age_v1`.
- `Ta` is the accumulated non-negative positive-temperature age since the
  latest material fresh-snow reset.
- Fresh-snow reset threshold is `0.001 m` water equivalent.
- Albedo is bounded to `[0, 0.85]`.
- Missing opt-in albedo state is a hard error only for active
  `coe_shortwave_albedo_v1` snowpack without a fresh-snow reset.
- 05C does not change radiation source binding, melt coefficients,
  routed-melt publication, parser surfaces, output schemas, or defaults.

## Required Reading Completed

Static:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/planning/snow-frost-fidelity-strategy.md` sections 2, 4, 5, 7, and 10
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `references/copyrighted/brock2000.pdf`
- SNOWDENSITY-05A and 05B package handoffs
- Existing CoE melt implementation in
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`

## Write Set

- `Cargo.toml`: registered `snowdensity05c_albedo_state_core`.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`:
  bumped to v78 and added `INV-SNOWFREEZE-054`,
  `OBL-SNOWFREEZE-P-029`, albedo-state variables, and the 05C addendum.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/08_snow_albedo.rs`:
  added the typed albedo-state core.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/mod.rs`: included
  the new support module.
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`: exported the public
  albedo-state API for 05D.
- `tests/integration/snowdensity05c_albedo_state_core.rs`: added contract,
  package, and behavior guards.
- `tests/integration/snowdensity05a_melt_contract_guard.rs` and
  `tests/integration/snowdensity05b_shortwave_source_contract.rs`: updated
  contract-version markers from v77 to v78.
- `docs/planning/snow-frost-fidelity-strategy.md`: marked 05C complete and
  kept 05D as the next melt-modernization gate.
- `docs/work-packages/README.md`: added this package to the execution log.
- Package artifacts under this directory.

## Evidence

Ran:

- Before implementation,
  `cargo test --test snowdensity05c_albedo_state_core` failed on unresolved
  albedo-state API imports.
- After implementation and artifact closure, focused 05C tests pass.

Static:

- `SC-SNOWFREEZE-001` v78 contains the authority and fail-closed contract.
- The new Rust core is standalone and is not called by the routed melt path.
- `legacy_coe` returns an inactive albedo outcome without requiring state.

## Exit Criteria Disposition

- Albedo bounds: satisfied by contract and focused integration test.
- Monotonic age decay: satisfied by focused integration test.
- Fresh-snow reset: satisfied by focused integration test.
- Missing-state fail-closed behavior: satisfied by focused integration test.
- No effect on `legacy_coe`: satisfied by API behavior and no runtime wiring.
- Routed-melt acceptance: explicitly deferred to 05D.

## Subagent Authorization

Subagent authorization: not used. This package performed local static review,
implementation, and verification because the write set is narrow and does not
default-activate production physics.

## Security Impact

No external network access was used. No copyrighted source text was copied into
the repository. No secrets, tokens, generated credentials, or external data
products were added.
