# Line Count Governance

Evidence label: Static/Ran.

Status: `EXECUTED-PASS`

Target file:
`crates/openwepp-runner/src/errors.rs`

Observed counts:

| File | Lines | Disposition |
|---|---:|---|
| `crates/openwepp-runner/src/errors.rs` | `549` | Below WARN/blocker thresholds |
| `tests/integration/cli01_runner_contract_derived_tests.rs` | `559` | Test file |

Disposition:

- Production target remains below the 2000-line WARN threshold and below the
  3000-line blocker.
