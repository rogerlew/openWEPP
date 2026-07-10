# Behavior Equivalence

Evidence label: Static/Ran.

Status: `EXECUTED-PASS`

## Scope

Static:

- This package is runner error CQR. It does not own kernel numeric formulas or
  conservation-sensitive output aggregation.
- No production code was changed.

## Equivalence Claim

Static:

- Public error APIs are unchanged.
- Stable error codes are unchanged.
- Display wording is unchanged.
- Source-chain behavior is unchanged.
- No runtime, release, launch, sidecar, serialization, or output behavior
  changed.

## Evidence

Ran:

| Command | Result |
|---|---|
| `cargo nextest run --test cli01_runner_contract_derived_tests` | PASS, exit `0`; `13` tests passed |
| `cargo clippy --test cli01_runner_contract_derived_tests -- -D warnings` | PASS, exit `0` |
| `cargo clippy -p openwepp-runner --all-targets -- -D warnings` | PASS, exit `0` |

Disposition:

- Behavior/API/output identity is preserved for package scope.
- No comparator delta review is applicable because this package does not change
  simulation math, route ownership, serialization, or public output surfaces.
