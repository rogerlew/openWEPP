## Milestone 0 Summary
- Package: 20260811-coupled-c3-forest-vegetation-state-machine-implementation-001
- Milestone: 0
- Frozen base: 06f7d8041f7d957a803a52db87fb5957461f84df
- Commands run: 7
- PASS: 6
- FAIL: 1
- Effective required PASS: 6
- Effective required FAIL: 0
- Verdict: PASS

- Passed (required + selected artifact pass check):
  - bash tools/release/check_science_contract_admission.sh --base-ref 06f7d8041f7d957a803a52db87fb5957461f84df --worktree (0)
  - bash tools/release/check_authority_suite_antievasion.sh (0)
  - bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md (0)
  - bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-BIOGEOCHEM-001.md (0)
  - sha256sum --check docs/work-packages/20260811-coupled-c3-forest-vegetation-model-stack-authority-001/artifacts/openwepp_c3_woody_v1_definition.json against expected OPENWEPP_C3_WOODY_V1 sha (0)
  - sha256sum --check selected registry file crates/openwepp-vegetation/model-registry/openwepp_c3_woody_v1_definition.json against expected OPENWEPP_C3_WOODY_V1 sha (0)

- Non-gated historical entry:
  - test -f docs/work-packages/20260811-coupled-c3-forest-vegetation-state-machine-implementation-001/artifacts/openwepp_c3_woody_v1_definition.json && verify OPENWEPP_C3_WOODY_V1 SHA-256 (1)

- Notes:
  - Failure at index 5 is from an unselected package-scoped artifact path; embedding strategy requires only `crates/openwepp-vegetation/model-registry/openwepp_c3_woody_v1_definition.json`, which passes byte equality.

- Log paths:
  - /home/workdir/openWEPP/docs/work-packages/20260811-coupled-c3-forest-vegetation-state-machine-implementation-001/artifacts/comparator-gate-logs/milestone-0-command-log.jsonl
  - /home/workdir/openWEPP/docs/work-packages/20260811-coupled-c3-forest-vegetation-state-machine-implementation-001/artifacts/comparator-gate-logs/milestone-0-command-log.json
  - /home/workdir/openWEPP/docs/work-packages/20260811-coupled-c3-forest-vegetation-state-machine-implementation-001/artifacts/comparator-gate-logs/milestone-0-summary.json
