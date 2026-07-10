# Verification Agent B

Evidence label: Static/Ran.

Status: `EXECUTED-COMPLETE`

Verifier: `rust_code_reviewer` subagent
`019f4992-8753-7b31-8e20-3a2281839c5b`.

Scope:

- Read-only test and metric consistency verification.
- Used `git`, `rg`, `sed`, `jq`, `stat`, and `sha256sum`.
- Did not rerun cargo gates.

Findings:

- No findings.

Verification summary:

- Public variants in `crates/openwepp-runner/src/errors.rs` are covered by
  assertions in `tests/integration/cli01_runner_contract_derived_tests.rs`.
- No production code changed.
- Targeted metric file byte counts and SHA-256 hashes match the package
  artifacts.
- Targeted CRAP JSON has `13` unique target functions, all `<= 30`, max
  `20.0`.
- LCOV JSON supports `266/267` line and `390/395` region coverage.
- Heavy runner evidence is accurately represented for package closure:
  full coverage blocked by `laned_shadow_h2637`, workspace clippy/full
  nextest/deny passed.

Residual risk:

- Verification relied on recorded logs and current `/tmp` metric artifacts; it
  did not rerun focused or heavy gates.
