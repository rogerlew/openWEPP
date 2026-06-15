# CQR18 Quality Plan Report

Static: scoped quality target is CRAP/cyclomatic-complexity burn-down for the
current target function in
`crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`.

Static: protected boundaries are public API, HBP binary schema, CRC checks,
payload bounds, event-kind behavior, state snapshot registry validation,
required-state obligations, parser compatibility, and typed HBP error IDs.

Status: closed.

Execution plan:

- Capture before LCOV and CRAP for the live target.
- Add focused characterization before production refactor because the target
  combines CRC, event, and state validation branches.
- Decompose only `validate_payload` behavior into private helpers.
- Re-run after LCOV and CRAP; target and extracted helpers must be CRAP
  `<= 30`.
- Run full required package gates and record results.

Risk controls:

- Public API parity report required.
- Behavior equivalence report required.
- Dual review and dual verification artifacts required.
- Final `cargo test --workspace` and HBP parser contract evidence required.

Current metric disposition:

- Before target CRAP: `456.4060356652947`.
- After `validate_payload` CRAP: `9.0`.
- After max target-file helper CRAP: `13.041259765625`.
