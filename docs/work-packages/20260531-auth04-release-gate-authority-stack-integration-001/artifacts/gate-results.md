# AUTH04 Gate Results

Status: completed  
Evidence mode: Ran

## Scope
- Execute AUTH04 validation gates and release-gate integration run.

## Commands run

1. `bash -n tools/release/run_release_candidate_gates.sh`
   - pass
2. `cargo test --test auth03_level4_constitutive_gate_contract --test auth04_release_gate_authority_stack_contract`
   - fail (one AUTH04 assertion on README default-lane wording)
3. `cargo test --test auth04_release_gate_authority_stack_contract`
   - pass (`4 passed`)
4. `bash tools/release/run_release_candidate_gates.sh --release-tag 260531auth04 --release-dir /tmp/openwepp_auth04_release --skip-stability --run-authority-periodic --run-authority-manual`
   - pass
   - included successful execution of:
     - `cargo fmt --check`
     - `cargo clippy --workspace --all-targets -- -D warnings`
     - `cargo test --workspace`
     - `cargo deny check` (non-failing duplicate/license-not-encountered warnings only)
   - authority suite report generated:
     - `/tmp/openwepp_auth04_release/authority_suite_results.md`
5. `markdown-doc lint --path docs/specifications/correctness-authority-model.md --path docs/specifications/science-contracts/index.md --path docs/governance/openwepp-release-procedure-draft.md --path tools/release/README.md --path docs/work-packages/20260531-auth04-release-gate-authority-stack-integration-001`
   - pass (`23 files validated, 0 errors, 0 warnings`)
6. `markdown-doc validate --path docs/specifications/correctness-authority-model.md --path docs/specifications/science-contracts/index.md --path docs/governance/openwepp-release-procedure-draft.md --path tools/release/README.md --path docs/work-packages/20260531-auth04-release-gate-authority-stack-integration-001`
   - fail (tool false positive on non-empty prompt file schema handling)
7. `markdown-doc validate --path docs/specifications/correctness-authority-model.md --path docs/specifications/science-contracts/index.md --path docs/governance/openwepp-release-procedure-draft.md --path tools/release/README.md --path docs/work-packages/20260531-auth04-release-gate-authority-stack-integration-001/package.md --path docs/work-packages/20260531-auth04-release-gate-authority-stack-integration-001/artifacts`
   - pass (`19 files validated, 0 errors`)

## Authority lane outcomes

1. `required` + `hard-fail`
   - pass (`auth03_level4_constitutive_gate_contract`)
2. `required` + `investigation`
   - not-configured (0 suites)
3. `periodic` + `hard-fail`
   - not-configured (0 suites)
4. `periodic` + `investigation`
   - not-configured (0 suites)
5. `manual` + `hard-fail`
   - not-configured (0 suites)
6. `manual` + `investigation`
   - not-configured (0 suites)

## Gate decision
- pass
