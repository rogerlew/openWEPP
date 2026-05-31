# AUTH04 Implementation And Test Evidence

Status: completed  
Evidence mode: Static + Ran

## Scope
- Implement authority-stack lane integration in release automation and CI.

## Static

1. Release-gate script integration:
   - `tools/release/run_release_candidate_gates.sh`
   - Added lane flags:
     - `--skip-authority-required`
     - `--run-authority-periodic`
     - `--run-authority-manual`
     - `--authority-registry`
     - `--authority-report`
   - Added registry-derived lane execution by `gate_lane` + `failure_class`.
   - Added machine-readable lane result report output:
     - `<release_dir>/authority_suite_results.md`
2. Workflow integration:
   - `.github/workflows/release-gates.yml`
   - Added:
     - weekly schedule trigger,
     - dispatch inputs for periodic/manual lanes,
     - routing of workflow trigger state into script lane flags.
3. Operator docs:
   - `tools/release/README.md`
   - `docs/governance/openwepp-release-procedure-draft.md`

## Ran

1. `cargo test --test auth03_level4_constitutive_gate_contract --test auth04_release_gate_authority_stack_contract`
   - initial run exposed one AUTH04 assertion mismatch.
2. `cargo test --test auth04_release_gate_authority_stack_contract`
   - pass after README wording alignment (`4 passed`).
3. `bash tools/release/run_release_candidate_gates.sh --release-tag 260531auth04 --release-dir /tmp/openwepp_auth04_release --skip-stability --run-authority-periodic --run-authority-manual`
   - pass.
   - Produced authority report with required lane hard-fail pass:
     - `cas_l4_soil_fc_minus33_001`
     - `cas_l4_soil_wp_minus1500_001`
     - `cas_l4_watbal_relax_to_fc_001`
   - Periodic/manual lanes were routable and reported as `not-configured` (no
     suites assigned yet).
