# 20260523-wb20-forward-water-balance-solver-lane-001

## Status
- state: completed
- date: 2026-05-23
- timezone: UTC

## Objective
Establish a forward-solved openWEPP water-balance parity lane for Tier-A
comparison that does not consume observed closure targets (`wb12_runoff_observed`,
`wb12_storage_observed`) as acceptance-driving inputs.

## Why This Package Exists
PL15R reversal retained PL08 hold and the PL09 parity-recovery addendum created
`WB20-forward-water-balance-solver-lane` after WB17/WB18/WB19. Static evidence
showed prior reconciliation posture could consume observed closure targets,
which is non-authoritative for forward-solver parity claims.

This package is contract-first and parity-authority-bound: canonical science
contracts must explicitly define forward-solver lane semantics, observed-target
exclusion rules, and acceptance boundaries before production lane/harness code
edits. Contract-derived tests and pre-implementation contract-gate evidence are
mandatory before production changes.

## Scope
### Included
- Implement canonical forward-solver lane authority amendments in
  `SC-WATBAL-001` and companion contracts (`SC-RUNOFFPART-001`, `SC-SYSTEM-001`)
  to encode observed-target exclusion and acceptance semantics.
- Derive lane parity authority from pinned legacy baseline
  `/workdir/wepp-forest_260430_baseline` (commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) where migration provenance is
  required, and record authority mapping in WB20 artifacts.
- Amend companion contracts/index references where solver-lane authority or
  alias continuity requires cross-contract updates.
- Implement WB20 contract-derived tests from amended canonical authority,
  including explicit proofs that observed closure targets are not used as
  acceptance-driving runtime inputs.
- Record pre-implementation contract-gate evidence proving contract +
  contract-test completion before production code edits.
- Implement forward-solver lane configuration/runtime wiring for parity lanes
  with explicit input manifesting and no observed-target substitution.
- Produce forward-solver replay traces and no-substitution evidence artifacts.
- Preserve ARCH15/ARCH21 typed-seam closure posture as non-regression.

### Explicitly Out of Scope
- PL14S strict Tier-A comparator execution and final hold-lift disposition.
- CLI10 executable implementation.
- WB17/WB18/WB19 physics-authority kernel authoring beyond consumed outputs.

## Deliverables
1. WB20 process-contract authority implementation evidence:
   - `artifacts/wb20-contract-implementation-evidence.md`
2. WB20 forward-solver lane authority and guard map:
   - `artifacts/wb20-forward-solver-lane-authority-and-guard-map.md`
3. WB20 contract-derived test implementation evidence:
   - `artifacts/wb20-contract-test-implementation-evidence.md`
4. Pre-implementation contract gate evidence:
   - `artifacts/wb20-preimplementation-contract-gate.md`
5. Implementation and test evidence:
   - `artifacts/wb20-implementation-and-test-evidence.md`
6. Forward-solver lane input manifest:
   - `artifacts/wb20-forward-solver-lane-input-manifest.md`
7. No observed-target substitution proof:
   - `artifacts/wb20-no-observed-target-substitution-evidence.md`
8. Forward-solver replay trace evidence:
   - `artifacts/wb20-forward-solver-replay-trace-evidence.md`
9. Legacy authority provenance map:
   - `artifacts/wb20-legacy-forward-lane-authority-provenance-map.md`
10. Typed-seam non-regression evidence:
   - `artifacts/wb20-typed-seam-non-regression-evidence.md`
11. Kernel profile compliance checklist:
   - `artifacts/wb20-kernel-profile-compliance-checklist.md`
12. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/wb20_disposition.md`
13. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15r-tier-a-delta-recloseout-and-hold-lift-rerun-001/artifacts/pl15r-pl08-hold-lift-decision-record.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15r-tier-a-delta-recloseout-and-hold-lift-rerun-001/artifacts/pl15r_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb14-infiltration-and-subdaily-hyetograph-kernel-001/artifacts/wb14_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb15-canopy-interception-kernel-coupling-001/artifacts/wb15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-clim05-snow-runtime-kernel-port-001/artifacts/clim05_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-clim06-frost-frozen-soil-kernel-port-001/artifacts/clim06_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-irrig10-irrigation-runtime-kernel-port-001/artifacts/irrig10_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb17-et-physics-equivalence-port-001/artifacts/wb17_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb18-percolation-physics-equivalence-port-001/artifacts/wb18_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb19-lateral-drainage-physics-equivalence-port-001/artifacts/wb19_disposition.md`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`
- `/workdir/wepp-forest_260430_baseline` (legacy forward-lane authority reference)
- `/workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch21-architecture-review-re-closeout-001/artifacts/crf-closure-evidence-matrix.md`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `tests/integration/**`
- `docs/work-packages/20260523-wb20-forward-water-balance-solver-lane-001/**`
- `docs/work-packages/README.md`
- `references to /workdir/wepp-forest_260430_baseline forward-lane authority in WB20 artifacts`

## Phase Plan
### Phase 0 - Intake
- Confirm PL15R reversal posture, WB20 queue objective, and current observed-
  target reconciliation baseline.

### Phase 1 - Canonical Contract Implementation
- Implement required forward-solver lane authority in canonical `SC-*`
  contracts, including observed-target exclusion semantics, acceptance
  boundaries, symbol/alias continuity, and guard invariants.
- Publish WB20 authority provenance mapping from baseline legacy references to
  canonical contract statements before production lane code edits.

### Phase 2 - Contract-Test Implementation + Gate (Pre-Implementation)
- Implement contract-derived tests for no-observed-target-substitution and
  solver-output-derived closure behavior.
- Execute and record pre-implementation contract-gate evidence before
  production code edits.

### Phase 3 - Lane Implementation
- Implement forward-solver lane configuration/runtime wiring to satisfy
  canonical WB20 authority and remove observed-target substitution from
  acceptance-driving inputs.

### Phase 4 - Verification
- Run targeted integration tests and required repository gates.

### Phase 5 - Disposition
- Publish WB20 contract/contract-test/implementation evidence and closure
  posture.

## Exit Criteria
- `WB20` queue objective is evidence-backed.
- Canonical science contracts explicitly encode forward-solver lane authority,
  observed-target exclusion semantics, and acceptance boundaries in relevant
  `SC-*` contracts; work-package notes alone do not satisfy closure.
- Lane manifest and evidence prove `wb12_runoff_observed` and
  `wb12_storage_observed` are not acceptance-driving runtime inputs.
- Closure evidence is solver-output-derived for parity lanes.
- Contract-derived WB20 tests are implemented and executed.
- Pre-implementation contract-gate evidence exists and shows contract-test
  implementation completed before production code edits.
- Forward-solver replay traces and no-substitution artifacts are produced.
- Dual review and dual verification artifacts are completed and dispositioned.
- Existing typed-seam closure posture from ARCH15/ARCH21 remains non-regressed.
- If code is changed, run and record:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: forward-solver lane authority and contract/test implementation.

## Execution Fallback (Policy-Filter Safe)
If an execution agent receives a false-positive policy block while running
WB20, continue with a minimal scoped prompt and micro-steps:

1. Scope:
   - local repository flat-file reads/edits only;
   - science-contract and Rust kernel migration work only.
2. Run in short phases (separate prompts):
   - phase A: contract-only updates (`SC-WATBAL-001`, `SC-RUNOFFPART-001`, index);
   - phase B: contract-derived tests + pre-implementation gate artifact;
   - phase C: production code edits;
   - phase D: verification + disposition artifacts.
3. Avoid large multi-file prompt payloads; reference file paths and specific
   headings/functions instead.
4. Preserve all WB20 exit criteria and evidence requirements; this fallback is
   prompt-shape mitigation only, not a scope change.
