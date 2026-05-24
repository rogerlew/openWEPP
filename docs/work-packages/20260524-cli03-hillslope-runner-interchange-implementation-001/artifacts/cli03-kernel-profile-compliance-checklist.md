# CLI03 Kernel Profile Compliance Checklist

Status: completed
Evidence mode: Static + Ran

## Static
Profile applicability decision:
- CLI03 changes runtime projection semantics that control kernel-branch
  execution (hillslope runner/CLI `.run` parse and sidecar/output authority),
  so kernel-profile compliance is required.

Checklist:
- [x] Canonical authority updates captured in canonical contract surfaces:
  - `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` (v12)
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` (v26)
  - `docs/specifications/science-contracts/index.md` registry update.
- [x] Runtime projection contract authority for CLI03 is explicit in canonical
      runner contracts/specs:
  - `docs/contracts/openwepp-hillslope-runfile-contract.md`
  - `docs/contracts/openwepp-runner-contract.md`
  - `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
- [x] Guard/error mapping is explicit and implemented in production code:
  - `CLIHILL-E-010` parse/contract boundary hard-fails,
  - `CLIHILL-E-013` required output existence hard-fail,
  - output contract validation via `OHOUT-E-001/002` mapping.
- [x] Contract-first sequencing evidence is present:
  - contract-test implementation artifact,
  - pre-implementation gate artifact,
  - implementation/test evidence artifact.
- [x] Test-vector obligations are reflected in implemented tests:
  - `tests/integration/cli03_runner_contract_derived_tests.rs`
  - `crates/openwepp-hillslope-output` unit tests.

## Ran
- `cargo test --test cli03_runner_contract_derived_tests`
  - pass (`9 passed; 0 failed`).
- `cargo test -p openwepp-hillslope-output`
  - pass (`11 passed; 0 failed`).
- Required repository gates:
  - `cargo fmt --check`: pass
  - `cargo clippy --workspace --all-targets -- -D warnings`: pass
  - `cargo test --workspace --quiet`: pass
  - `cargo deny check`: pass (non-fatal allowlist warnings only)

Compliance decision:
- CLI03 satisfies kernel-profile compliance obligations for its runtime
  projection scope.
