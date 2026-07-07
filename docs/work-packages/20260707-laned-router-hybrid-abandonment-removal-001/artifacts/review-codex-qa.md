# QA Review

Status: FINDINGS-DISPOSITIONED. Evidence mode: Static + Ran.

Reviewer: Codex `rust_qa_reviewer` subagent.

Ran:

- Static review of diff, strip inventory, SC rows, identity artifacts, and
  source/test symbol searches.
- `cargo nextest run --test laned_shadow_h2637 abandoned_implicit_selector_env_fails_closed_at_startup`

## Findings

### QA-M1 - Medium - Explicit Terminal-Deficit Fail-Closed Branch Was Unguarded

The live explicit router retained the `RoutingError::NegativeOutletBin`
public-path fail-closed branch, but the only terminal-deficit test had been
retired with the hybrid composition test set.

Disposition: accepted and fixed.

Fix:

- Added
  `material_terminal_bin_deficit_fails_closed_on_public_path` in
  `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`.
- The test constructs the retained front-arrival attribution class directly
  on explicit solver state and asserts the public `run_with_options` path
  returns `RoutingError::NegativeOutletBin`.

Verification:

- Ran: `cargo nextest run -p openwepp-hillslope-orchestrator material_terminal_bin_deficit_fails_closed_on_public_path --profile quick`
- Result: `1` test passed.

### QA-L1 - Low - `SC-OFEROUTE-001` Frontmatter Version Was Stale

`SC-OFEROUTE-001` frontmatter still had `contract_version: 36` while the
ADR-0037 removal changelog row is rev 37.

Disposition: accepted and fixed.

Fix:

- Updated `contract_version: 37`.

Verification:

- Ran: `cargo nextest run --test hphys0279_sc_unit_compliance_lint_contract --profile quick`
- Result: `9` tests passed.

## Non-Blocking Note

The reviewer suggested recording the active env directly in the identity
summaries. The package script is part of the committed artifact and records
the active-plain environment source (`OPENWEPP_LANED_ACTIVE=1`,
`OPENWEPP_LANED_SHADOW_PROFILE=1`, shadow/implicit unset). The JSON/MD
identity summaries plus script are treated as the evidence bundle.
