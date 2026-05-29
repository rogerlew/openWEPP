# 20260529-wshedimpl43-hbp-binary-only-ascii-pass-removal-001

## Status
- state: queued
- date: 2026-05-29
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute WSHEDIMPL43 to remove ASCII pass support completely and replace it with
binary HBP-only publication/ingestion across hillslope and watershed CLI
boundaries, then rerun `/wc1/runs/un/unpalatable-rind` to successful watershed
parquet emission.

## Why This Package Exists
WSHEDIMPL42 closed the WB14 runoff guard blocker but disposition remained HOLD
because watershed intake failed with `CLIWAT-E-017` / `HBP-E-002` after
hillslope outputs were emitted as ASCII pass text under `.hbp` names.
ASCII pass support must be fully removed: no option, no fallback, no cutover.

## Scope
### Included
- Canonical contract/spec updates to enforce binary-only HBP naming and strict
  parse policy (no `.pass.dat` compatibility derivation).
- Contract-derived tests that assert `.pass.dat` rejection and binary HBP
  publication/consumption continuity.
- Production implementation that replaces ASCII pass publication with binary HBP
  shard emission from hillslope runtime outputs.
- Watershed CLI intake policy migration to strict `.hbp` path policy.
- Release metadata/schema updates removing legacy ASCII pass capability fields.
- Closure rerun on `/wc1/runs/un/unpalatable-rind` demonstrating successful
  watershed execution and parquet emission.

### Explicitly Out of Scope
- Physics/process changes unrelated to pass-format boundary behavior.
- New compatibility layers for legacy ASCII `.pass.dat` workflows.
- Silent fallback/defaulting for invalid pass format or naming.

## Deliverables
1. `artifacts/wshedimpl43-ascii-pass-removal-gap-matrix.md`
2. `artifacts/wshedimpl43-contract-implementation-evidence.md`
3. `artifacts/wshedimpl43-contract-test-implementation-evidence.md`
4. `artifacts/wshedimpl43-preimplementation-contract-gate.md`
5. `artifacts/wshedimpl43-implementation-and-test-evidence.md`
6. `artifacts/wshedimpl43-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl43_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Implement required canonical contract/index/spec amendments for binary-only
   HBP policy and release-boundary authority.
2. Implement contract-derived tests for strict naming and binary publication.
3. Record pre-implementation contract-gate evidence before production edits.
4. Apply production code changes for binary-only HBP publication/intake.

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority lives in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Binary HBP authority remains anchored to:
  - `/workdir/wepp-forest/docs/contracts/hillslope-binary-pass-format.md`
  - `/workdir/wepp-forest/docs/contracts/watershed-hillslope-pass-reader-contract.md`
- No ASCII pass fallback behavior is permitted in production surfaces.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/wepp-input-files/specs/hbp-file.spec.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/work-packages/20260529-wshedimpl42-wb14-runoff-guard-unpalatable-rind-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260529-wshedimpl42-wb14-runoff-guard-unpalatable-rind-closure-001/artifacts/wshedimpl42_disposition.md`
- `/wc1/runs/un/unpalatable-rind`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260529-wshedimpl43-hbp-binary-only-ascii-pass-removal-001/**`
- `docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/wepp-input-files/specs/hbp-file.spec.md`
- `crates/openwepp-input-contract/src/parsers/hbp.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-runner/src/release.rs`
- `tests/integration/infile_hbp_parser_contract.rs`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL43 authorization from WSHEDIMPL42 HOLD follow-on requirement.
- Freeze scope to binary-only HBP boundary closure.

### Phase B - Contract/spec authority updates
- Amend canonical HBP parser/system contracts and spec text to prohibit
  `.pass.dat` compatibility derivation.
- Update science-contract index notes for WSHEDIMPL43 scope.

### Phase C - Contract-derived test updates
- Add/adjust tests for strict `.hbp` naming-only behavior and no compat warning
  branch.
- Add/adjust tests asserting binary HBP publication path (not ASCII text).

### Phase D - Pre-implementation contract gate
- Record evidence that contract/spec updates and test vectors exist before
  production edits.

### Phase E - Production implementation
- Replace ASCII pass write path with binary HBP writer in hillslope CLI
  execution.
- Remove `.pass.dat` compatibility path from parser and watershed CLI intake.
- Remove release metadata legacy ASCII pass family declaration/validation.

### Phase F - Validation and watershed closure rerun
- Execute:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
- Rerun `/wc1/runs/un/unpalatable-rind` watershed and verify parquet outputs.

### Phase G - Dual review, dual verification, disposition
- Complete review and verification artifacts.
- Publish GO/HOLD disposition with explicit closure status.

## Exit Criteria
- All ASCII pass support paths are removed from production parser/runner/release
  boundaries.
- Hillslope CLI emits binary-valid `.hbp` that watershed CLI ingests in strict
  mode.
- Contract-first sequence is satisfied and evidenced.
- Unpalatable-rind watershed run completes with parquet outputs.
- Validation gates are truthfully recorded.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local parser/runner/spec/test updates only; no auth/credential or
  external service surface changes.
