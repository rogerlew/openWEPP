# 20260525-mofe09-hs-runtime-e-003-soil-runtime-fallback-and-h324-parity-rerun-001

## Status
- state: complete
- date: 2026-05-25
- timezone: UTC

## Objective
Resolve carved-letter `H324` runtime-seam blocker `HS-RUNTIME-E-003` by
contract-authorized soil runtime projection fallback for legacy `7778` soils,
then rerun MOFE single-hillslope semantic parity lane.

## Why This Package Exists
MOFE08 closed CLIGEN `5.323` compatibility but parity remains blocked at soil
runtime projection:
`HS-RUNTIME-E-003: primary soil layer missing required theta_r_rosetta (thetdr)`.
Legacy `7778` soils parse measured `fc/wp` but do not provide Rosetta theta
fields, so runtime projection must be version-compatible while preserving typed
fail-closed behavior.

## Scope
### Included
- Canonical soil contract amendment for runtime projection precedence.
- Contract-derived tests proving `7778` measured fallback behavior.
- Pre-implementation gate evidence (failing test before code change).
- Runtime seam implementation in hillslope orchestrator.
- `H324` lane rerun and semantic comparator rerun (or typed blocker capture).

### Explicitly Out of Scope
- Climate parser behavior (closed in MOFE08).
- Hydrology equation redesign or surrogate process-physics changes.
- Watershed MOFE routing redesign.

## Deliverables
1. Runtime fallback implementation report:
   - `artifacts/mofe09-runtime-fallback-implementation-report.md`
2. Runtime fallback test matrix:
   - `artifacts/mofe09-runtime-fallback-test-matrix.md`
3. H324 parity rerun report:
   - `artifacts/mofe09-h324-parity-rerun-report.md`
4. Contract implementation evidence:
   - `artifacts/mofe09-contract-implementation-evidence.md`
5. Contract-test implementation evidence:
   - `artifacts/mofe09-contract-test-implementation-evidence.md`
6. Pre-implementation contract gate:
   - `artifacts/mofe09-preimplementation-contract-gate.md`
7. Implementation/test evidence:
   - `artifacts/mofe09-implementation-and-test-evidence.md`
8. Kernel profile checklist:
   - `artifacts/mofe09-kernel-profile-compliance-checklist.md`
9. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/mofe09_disposition.md`
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
1. Amend canonical contracts for scoped runtime projection authority.
2. Implement contract-derived tests.
3. Record pre-implementation contract gate evidence.
4. Modify production parser/runtime code and execute parity rerun.

No production runtime seam behavior edits are permitted before steps 1-3
complete.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe08-cligen-5323-compat-and-h324-parity-rerun-001/artifacts/mofe08_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260525-mofe08-cligen-5323-compat-and-h324-parity-rerun-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`
- `/workdir/openWEPP/tools/legacy_comparison_suite/README.md`

## Intended Write Set
- `docs/work-packages/20260525-mofe09-hs-runtime-e-003-soil-runtime-fallback-and-h324-parity-rerun-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `tests/integration/parser_runtime_seam_integration.rs`
- `tests/fixtures/infile/soil/valid_7778.sol`

## Phase Plan
### Phase A - Contract Authority Alignment
- Amend canonical soil contract to define runtime export precedence for
  `thetdr`/`thetfc` from parser soil fields:
  - `thetdr := theta_r_rosetta` when present, else `wp_measured`.
  - `thetfc := fc_rosetta` when present, else `fc_measured`.
  - Missing/non-finite values remain typed hard failures.

### Phase B - Contract-Derived Tests
- Add tests proving `7778` soils (without Rosetta fields) build runtime surface
  from measured `wp/fc`.
- Preserve existing strict failures for truly missing required soil-state
  symbols.

### Phase C - Pre-Implementation Contract Gate
- Run targeted runtime seam tests and record expected failing behavior for new
  `7778` fallback expectation before production edits.

### Phase D - Runtime Implementation
- Implement fallback in `build_hillslope_runtime_surface_from_soil` with
  explicit Rosetta precedence and fail-closed guards.

### Phase E - MOFE Parity Rerun
- Re-run carved-letter `H324` lane and execute semantic comparator if candidate
  surface is produced.
- If blocked, capture the next typed blocker and reproducibility evidence.

### Phase F - Closeout
- Complete artifacts, dual review/verification, gate matrix, and disposition.

## Exit Criteria
- `HS-RUNTIME-E-003` no longer blocks valid `7778` measured-fc/wp soils.
- Contract-derived runtime seam tests cover fallback + precedence and pass.
- `H324` parity lane rerun executes through comparator or returns a new typed
  blocker after runtime-surface soil projection.

## Security Impact and Review Gate
- security_impact: none
- dedicated_security_review_required: no
- Rationale: runtime-input mapping + tests/docs only; no credential/network
  boundary changes.
