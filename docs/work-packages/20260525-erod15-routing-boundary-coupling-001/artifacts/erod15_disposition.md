# Erod15 disposition

Status: complete
Evidence mode: mixed

## Static
- Phase A (intake/entry confirmation): complete.
- Phase B (canonical contract amendments): complete.
- Phase C (contract tests + gate): complete.
- Phase D (production runtime implementation): complete.
- Phase E (verification + disposition): complete.
- Scope amendment requirements satisfied:
  - `crates/openwepp-runner/**` included.
  - HBP pass serialization docs included as dependencies.
  - `crates/openwepp-watershed-output/**` added for watershed writer boundary
    parity with hillslope pattern.

## Finding Disposition Matrix
- `F-001` (HBP contributor payload zero-seeding):
  - disposition: accepted
  - severity: high
  - closure class: closed
  - closure evidence:
    - `crates/openwepp-input-contract/src/parsers/hbp.rs`
    - `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
  - notes: contributor route symbols are now seeded from parsed latest-event
    payload values; zero vectors are only used when payload family is absent.
- `F-002` (empty watershed parquet emissions):
  - disposition: accepted
  - severity: high
  - closure class: closed
  - closure evidence:
    - `crates/openwepp-watershed-output/src/writers.rs`
  - notes: writer now hard-fails with typed guard `OWSOUT-E-004` instead of
    emitting schema-valid empty parquet placeholders.
- `F-003` (clippy gate failure on required lane):
  - disposition: accepted
  - severity: high
  - closure class: closed
  - closure evidence:
    - `cargo clippy -p openwepp-watershed-output -p openwepp-runner --bin openwepp-cli-watershed -- -D warnings` -> PASS
    - `cargo clippy --workspace --all-targets -- -D warnings` -> PASS
- `F-004` (`--output-dir` semantic ambiguity):
  - disposition: accepted
  - severity: medium
  - closure class: closed
  - closure evidence:
    - `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
    - `docs/contracts/openwepp-watershed-runfile-contract.md`
  - notes: relative output paths now resolve against `--output-dir`; relative
    input paths continue to resolve against the `.run` file location.
- `F-005` (watershed behavior test depth insufficient):
  - disposition: accepted
  - severity: medium
  - closure class: closed
  - closure evidence:
    - `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
  - notes: behavior-level CLI process tests now cover both WS10 domain-guard
    rejection and typed writer-guard rejection (`CLIWAT-E-034` + `OWSOUT-E-004`).
- `F-006` (`nchan` fallback clamp):
  - disposition: accepted
  - severity: low
  - closure class: deferred follow-up
  - required action: replace silent clamp with typed domain failure or explicit
    documented compatibility rationale.
  - owner surface: `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`

## Ran
- `cargo fmt --check` -> PASS.
- `cargo clippy --workspace --all-targets -- -D warnings` -> PASS.
- `cargo test --workspace` -> PASS.
- `cargo deny check` -> PASS (warnings only; no failing policy classes).
- `cargo clippy -p openwepp-watershed-output -p openwepp-runner --bin openwepp-cli-watershed -- -D warnings` -> PASS.
- `cargo test -p openwepp-runner --test watershed_cli_behavior_contract` -> PASS.
- `cargo test --test cli03_runner_contract_derived_tests` -> PASS.

## Final Disposition
- GO for package closure.
- EROD16 entry signal: GO.
