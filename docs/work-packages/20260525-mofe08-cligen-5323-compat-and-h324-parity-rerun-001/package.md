# 20260525-mofe08-cligen-5323-compat-and-h324-parity-rerun-001

## Status
- state: complete
- date: 2026-05-25
- timezone: UTC

## Objective
Implement CLIGEN `5.323` compatibility in openWEPP climate intake by treating
all climate `datver` values `>=5.3` and `<5.4` as canonical `5.3`, then rerun
carved-letter MOFE single-hillslope parity lane (`H324`) to confirm slope/soil
blocker closure and record new parity posture.

## Why This Package Exists
MOFE07 closed carved-letter slope/soil parser blockers, but `openwepp-cli-hill`
for `H324` remains blocked at climate parse (`unsupported datver '5.323'`).
The user also requires CLIGEN guidance codifying minor-version bump policy for
format-breaking changes in `/workdir/jimf-cligen532` and openWEPP cross
references to that guidance.

## Scope
### Included
- Canonical climate parser contract updates for CLIGEN 5.3x compatibility.
- Contract-derived tests for `datver=5.323` acceptance + canonicalization.
- Pre-implementation gate evidence capturing failing tests before parser edits.
- Production parser implementation of `>=5.3,<5.4 -> 5.3` behavior.
- Carved-letter `H324` MOFE candidate rerun and semantic parity comparator rerun
  (or typed blocker capture if comparator still blocked).
- Guidance update in `/workdir/jimf-cligen532` requiring minor-version bumps for
  format-breaking changes, plus openWEPP cross-reference.

### Explicitly Out of Scope
- Hydrology/erosion process-physics changes.
- Watershed routing or erosion model redesign.
- Non-CLIGEN climate format expansions beyond scoped `5.3x` policy.

## Deliverables
1. CLIGEN compatibility implementation report:
   - `artifacts/mofe08-cligen-compat-implementation-report.md`
2. CLIGEN compatibility test matrix:
   - `artifacts/mofe08-cligen-compat-test-matrix.md`
3. MOFE parity rerun report:
   - `artifacts/mofe08-h324-parity-rerun-report.md`
4. Contract implementation evidence:
   - `artifacts/mofe08-contract-implementation-evidence.md`
5. Contract-test implementation evidence:
   - `artifacts/mofe08-contract-test-implementation-evidence.md`
6. Pre-implementation contract gate:
   - `artifacts/mofe08-preimplementation-contract-gate.md`
7. Implementation/test evidence:
   - `artifacts/mofe08-implementation-and-test-evidence.md`
8. Kernel profile checklist:
   - `artifacts/mofe08-kernel-profile-compliance-checklist.md`
9. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/mofe08_disposition.md`
   - `artifacts/worker-handoff.md`
10. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
11. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Autonomous Execution Intent (Required)
This package is execution-ready and must proceed end-to-end without user
intervention unless hard-blocked by contradictory canonical requirements or
unresolvable environment failures.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:` sections.

## Contract-First Sequence (Required)
1. Amend canonical contracts for scoped climate compatibility authority.
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Modify production parser/runtime code and execute parity rerun.

No production parser behavior edits are permitted before steps 1-3 complete.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe06-single-hillslope-semantic-parity-carved-letter-001/artifacts/mofe06_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/artifacts/mofe07_disposition.md`
- `/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/climate.rs`
- `/workdir/openWEPP/tests/integration/infile_climate_parser_contract.rs`
- `/workdir/openWEPP/tools/legacy_comparison_suite/README.md`
- `/workdir/jimf-cligen532/README.md`

## Intended Write Set
- `docs/work-packages/20260525-mofe08-cligen-5323-compat-and-h324-parity-rerun-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`
- `docs/specifications/wepp-input-files/specs/climate-file.spec.md`
- `crates/openwepp-input-contract/src/parsers/climate.rs`
- `tests/integration/infile_climate_parser_contract.rs`
- `tests/fixtures/infile/climate/**` (new compat fixture(s))
- `/workdir/jimf-cligen532/README.md`

## Phase Plan
### Phase A - Contract Authority Alignment
- Encode canonical `5.3x` datver compatibility policy in `SC-INFILE-CLIMATE-001`:
  accepted input domain `5.3 <= datver < 5.4`, canonicalized to `5.3` in parser
  output lineage.

### Phase B - Contract-Derived Tests
- Add tests/fixtures for `datver=5.323` acceptance and canonicalization.
- Keep strict rejection for `datver >= 5.4` and `<5.3` non-allowlisted variants.

### Phase C - Pre-Implementation Contract Gate
- Run targeted climate parser tests and capture expected failures for new
  compatibility expectations before parser edits.

### Phase D - Parser Implementation
- Implement parser policy to treat `datver >=5.3 && <5.4` as canonical `5.3`.

### Phase E - MOFE Parity Rerun
- Re-run `openwepp-cli-hill` on carved-letter `H324` lane with compat policy.
- Execute semantic WAT comparator if candidate surface is emitted; otherwise
  capture typed blocker evidence.

### Phase F - Cross-Repo Guidance + Disposition
- Update `/workdir/jimf-cligen532/README.md` with minor-version-break policy.
- Cross-reference guidance in openWEPP contract/spec artifacts.
- Complete artifacts and publish disposition.

## Exit Criteria
- `datver=5.323` parses successfully and canonicalizes to `5.3`.
- Contract-derived tests cover policy and pass.
- MOFE `H324` parity lane rerun is executed through comparator or blocked with
  typed, reproducible post-climate evidence.
- `jimf-cligen532` guidance update is authored and cross-referenced in openWEPP.

## Security Impact and Review Gate
- security_impact: none
- dedicated_security_review_required: no
- Rationale: parser compatibility + docs changes only; no credential/network
  boundary changes.
