# CLI04 Kernel Profile Compliance Checklist

Status: completed
Evidence mode: Static + Ran

## Static
Profile applicability decision:
- CLI04 changes runtime projection and output-contract behavior for
  simulation-driven `outputs.wat` emission, so kernel-profile/runtime-contract
  compliance evidence is required.

Checklist:
- [x] Canonical runtime-contract authority updates are completed in runner
      contract/spec surfaces:
  - `docs/contracts/openwepp-hillslope-runfile-contract.md`
  - `docs/contracts/openwepp-runner-contract.md`
  - `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
  - `docs/contracts/README.md`
- [x] Required CLI04 invariants are explicitly encoded:
  - metadata parity (`units`, `description`, dataset version keys),
  - optional producer-authoritative `InterceptionStorage`,
  - dependency posture (`parquet` + `arrow-array` + `arrow-schema`, no new
    `arrow2`).
- [x] Guard/error mapping is implemented and fail-closed:
  - output-contract typed errors (`OHOUT-WAT-E-001`, `OHOUT-WAT-E-002`),
  - runner runtime-surface hard-fail mapping for `outputs.wat` emission.
- [x] Contract-first sequencing evidence is complete:
  - Phase A contract artifact,
  - Phase B contract-test artifact,
  - pre-implementation gate artifact,
  - Phase C implementation evidence.
- [x] Contract-derived and unit test obligations are implemented:
  - `tests/integration/cli04_runner_wat_parquet_contract_derived_tests.rs`
  - `crates/openwepp-hillslope-output/src/hillslope_wat.rs` unit tests.
- [x] No unresolved profile/procedure blockers remain for CLI04 package scope.

Notes:
- CLI04 does not alter kernel process equations or canonical SC algorithm text;
  authority deltas are limited to runner/output contract surfaces and verified
  runtime-output behavior.

## Ran
- `cargo test --test cli04_runner_wat_parquet_contract_derived_tests`
  - pass (`2 passed; 0 failed`).
- `cargo test -p openwepp-hillslope-output`
  - pass (`14 passed; 0 failed`).
- Required repository gates:
  - `cargo fmt --check`: pass
  - `cargo clippy --workspace --all-targets -- -D warnings`: pass
  - `cargo test --workspace`: pass
  - `cargo deny check`: pass

Compliance decision:
- CLI04 satisfies kernel-profile/runtime-contract compliance obligations for
  its output-contract implementation scope.
