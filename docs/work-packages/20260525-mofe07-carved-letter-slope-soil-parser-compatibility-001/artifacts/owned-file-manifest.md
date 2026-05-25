# Owned File Manifest

Status: complete
Evidence mode: mixed (Static + Ran)

Static:
- Manifest lists files authored/updated in MOFE07 scope.

Ran:
- Reconciled against current worktree edits.

Package and index:
- `docs/work-packages/README.md`
- `docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/package.md`
- `docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/prompts/README.md`
- `docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/prompts/active/README.md`
- `docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/prompts/active/mofe07_kickoff_agent_prompt.md`
- `docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/prompts/archived/README.md`
- `docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/artifacts/README.md`
- `docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/artifacts/mofe07-parser-compatibility-implementation-report.md`
- `docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/artifacts/mofe07-parser-compatibility-test-matrix.md`
- `docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/artifacts/mofe07-contract-implementation-evidence.md`
- `docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/artifacts/mofe07-contract-test-implementation-evidence.md`
- `docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/artifacts/mofe07-preimplementation-contract-gate.md`
- `docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/artifacts/mofe07-implementation-and-test-evidence.md`
- `docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/artifacts/mofe07-kernel-profile-compliance-checklist.md`
- `docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/artifacts/gate-results.md`
- `docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/artifacts/mofe07_disposition.md`
- `docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/artifacts/review_agent_a.md`
- `docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/artifacts/review_agent_b.md`
- `docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/artifacts/verification_agent_a.md`
- `docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/artifacts/verification_agent_b.md`

Contracts:
- `docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`

Code and tests:
- `crates/openwepp-input-contract/src/parsers/slope.rs`
- `crates/openwepp-input-contract/src/parsers/soil.rs`
- `tests/integration/infile_slope_parser_contract.rs`
- `tests/integration/infile_soil_parser_contract.rs`
- `tests/fixtures/infile/slope/compat_shared_geom_multi_ofe.slp`
- `tests/fixtures/infile/soil/compat_quoted_header_7778.sol`
- `tests/fixtures/infile/soil/compat_quoted_header_7778_per_ofe_restrictive.sol`
