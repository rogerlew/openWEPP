# 20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001

## Status
- state: complete
- date: 2026-05-25
- timezone: UTC

## Objective
Close the carved-letter MOFE parser compatibility blockers identified by MOFE06
by implementing compatibility-mode acceptance for legacy carved-letter slope and
soil encodings required to run single-hillslope semantic parity.

## Why This Package Exists
MOFE06 selected `H324` as a low-closure MOFE candidate but could not produce an
openWEPP candidate surface because `openwepp-cli-hill` failed with typed parser
errors on carved-letter `p324.slp` and `p324.sol` compatibility forms.

## Scope
### Included
- Canonical contract authority updates for parser compatibility behavior where
  needed.
- Contract-derived parser tests for carved-letter compatibility forms.
- Pre-implementation gate evidence capturing failing tests before parser edits.
- Production parser compatibility implementation for slope and soil intake.
- Validation evidence and disposition.

### Explicitly Out of Scope
- Hydrology/erosion kernel physics changes.
- Watershed routing changes.
- Broad non-carved-letter parser redesign beyond scoped compatibility behavior.

## Deliverables
1. Parser compatibility implementation report:
   - `artifacts/mofe07-parser-compatibility-implementation-report.md`
2. Contract-derived compatibility test matrix:
   - `artifacts/mofe07-parser-compatibility-test-matrix.md`
3. Contract implementation evidence:
   - `artifacts/mofe07-contract-implementation-evidence.md`
4. Contract-test implementation evidence:
   - `artifacts/mofe07-contract-test-implementation-evidence.md`
5. Pre-implementation contract gate:
   - `artifacts/mofe07-preimplementation-contract-gate.md`
6. Implementation/test evidence:
   - `artifacts/mofe07-implementation-and-test-evidence.md`
7. Kernel profile checklist:
   - `artifacts/mofe07-kernel-profile-compliance-checklist.md`
8. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/mofe07_disposition.md`
   - `artifacts/worker-handoff.md`
9. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
10. Dual verification artifacts:
    - `artifacts/verification_agent_a.md`
    - `artifacts/verification_agent_b.md`

## Autonomous Execution Intent (Required)
This package is execution-ready and must proceed end-to-end without user
intervention unless hard-blocked by contradictory canonical requirements or
unresolvable environment failures.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:` sections.

## Contract-First Sequence (Required)
1. Amend canonical contracts for scoped parser compatibility authority.
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Modify production parser code.

No production parser behavior edits are permitted before steps 1-3 complete.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe06-single-hillslope-semantic-parity-carved-letter-001/artifacts/mofe06_disposition.md`
- `/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/slope.rs`
- `/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/soil.rs`
- `/workdir/openWEPP/tests/integration/infile_slope_parser_contract.rs`
- `/workdir/openWEPP/tests/integration/infile_soil_parser_contract.rs`

## Intended Write Set
- `docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `tests/fixtures/infile/slope/**` (new compat fixture(s))
- `tests/fixtures/infile/soil/**` (new compat fixture(s))
- `tests/integration/infile_slope_parser_contract.rs`
- `tests/integration/infile_soil_parser_contract.rs`
- `crates/openwepp-input-contract/src/parsers/slope.rs`
- `crates/openwepp-input-contract/src/parsers/soil.rs`

## Phase Plan
### Phase A - Contract Authority Alignment
- Encode carved-letter compatibility authority for:
  - legacy shared OFE-metadata slope form in compatibility mode,
  - quoted legacy soil OFE header identifiers in compatibility mode.

### Phase B - Contract-Derived Tests
- Add parser tests/fixtures proving scoped compatibility acceptance in
  compatibility mode and preserved strict rejection posture where required.

### Phase C - Pre-Implementation Contract Gate
- Run targeted parser contract tests and capture expected pre-implementation
  failures for new compatibility tests.

### Phase D - Production Parser Implementation
- Implement compatibility-mode parser behavior in slope and soil parsers.

### Phase E - Validation and Disposition
- Run required gates and targeted parser tests.
- Complete review/verification artifacts and publish disposition.

## Exit Criteria
- Scoped parser compatibility authority is explicit in canonical contracts.
- Contract-derived tests cover new compatibility paths and guard posture.
- Pre-implementation failing-gate evidence is captured.
- Production parser changes satisfy tests and gates.

## Security Impact and Review Gate
- security_impact: none
- dedicated_security_review_required: no
- Rationale: parser compatibility logic only; no credential/network/security
  boundary changes.
