# 20260523-wb19-lateral-drainage-physics-equivalence-port-001

## Status
- state: completed
- date: 2026-05-23
- timezone: UTC

## Objective
Replace WB11 fraction-only lateral/drainage surrogate behavior with
legacy-equivalent subsurface and drainage physics authority under contract-first
sequencing in the monolithic openWEPP scientific hydrology/erosion model.

## Why This Package Exists
PL15R reversal retained PL08 hold and the PL09 parity-recovery addendum created
`WB19-lateral-drainage-physics-equivalence-port` after WB18. Static evidence
showed WB11 lateral/drainage behavior is currently scalar split surrogate logic
and not full legacy-physics authority.

This package is contract-first and physics-authority-bound: canonical science
contracts must explicitly encode lateral/drainage physics equations, symbols,
units/guards, and layer-domain flux semantics before production kernel code
edits. Contract-derived tests and pre-implementation contract-gate evidence are
mandatory before kernel implementation changes.

## Scope
### Included
- Implement canonical lateral/drainage physics-authority amendments in
  `SC-SUBHYD-001` and `SC-WATBAL-001`, including explicit equation forms,
  state symbols, units, domain guards, and subsurface/drainage branch behavior.
- Derive lateral/drainage physics migration authority from pinned legacy
  baseline `/workdir/wepp-forest_260430_baseline` (commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) and record equation-level
  provenance in WB19 artifacts.
- Amend companion contracts/index references where lateral/drainage physics
  authority or alias continuity requires cross-contract updates.
- Implement WB19 contract-derived tests from amended canonical
  lateral/drainage authority.
- Record pre-implementation contract-gate evidence proving contract +
  contract-test completion before production kernel edits.
- Replace WB11 lateral/drainage surrogate production behavior with
  equation-driven, layer-aware lateral/drainage physics behavior aligned to
  canonical authority.
- Produce hydraulic vector and branch-response parity evidence for
  lateral/drainage behavior.
- Preserve ARCH15/ARCH21 typed-seam closure posture as non-regression.

### Explicitly Out of Scope
- WB20 forward water-balance solver lane execution.
- CLI10 executable driver implementation.
- Tier-A hold-lift disposition updates beyond WB19 scope.

## Deliverables
1. WB19 process-contract authority implementation evidence:
   - `artifacts/wb19-contract-implementation-evidence.md`
2. WB19 lateral/drainage physics authority and guard map note:
   - `artifacts/wb19-lateral-drainage-physics-authority-and-guard-map.md`
3. WB19 contract-derived test implementation evidence:
   - `artifacts/wb19-contract-test-implementation-evidence.md`
4. Pre-implementation contract gate evidence:
   - `artifacts/wb19-preimplementation-contract-gate.md`
5. Implementation and test evidence:
   - `artifacts/wb19-implementation-and-test-evidence.md`
6. Lateral/drainage hydraulic vector parity evidence:
   - `artifacts/wb19-lateral-drainage-hydraulic-vector-parity-evidence.md`
7. Lateral/drainage branch-response evidence:
   - `artifacts/wb19-lateral-drainage-branch-response-evidence.md`
8. Legacy lateral/drainage physics provenance map:
   - `artifacts/wb19-legacy-lateral-drainage-physics-provenance-map.md`
9. Typed-seam non-regression evidence:
   - `artifacts/wb19-typed-seam-non-regression-evidence.md`
10. Kernel profile compliance checklist:
   - `artifacts/wb19-kernel-profile-compliance-checklist.md`
11. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/wb19_disposition.md`
12. Dual review/verification artifacts:
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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15r-tier-a-delta-recloseout-and-hold-lift-rerun-001/artifacts/pl15r-pl08-hold-lift-decision-record.md`
- `/workdir/openWEPP/docs/work-packages/20260523-pl15r-tier-a-delta-recloseout-and-hold-lift-rerun-001/artifacts/pl15r_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb18-percolation-physics-equivalence-port-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb18-percolation-physics-equivalence-port-001/artifacts/wb18_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb11-et-perc-lateral-drain-kernels-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260523-wb11-et-perc-lateral-drain-kernels-001/artifacts/wb11_disposition.md`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`
- `/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs`
- `/workdir/wepp-forest_260430_baseline` (legacy lateral/drainage migration authority)
- `/workdir/openWEPP/docs/work-packages/20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/artifacts/arch15_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260522-arch21-architecture-review-re-closeout-001/artifacts/crf-closure-evidence-matrix.md`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `tests/integration/**`
- `docs/work-packages/20260523-wb19-lateral-drainage-physics-equivalence-port-001/**`
- `docs/work-packages/README.md`
- `references to /workdir/wepp-forest_260430_baseline lateral/drainage authority in WB19 artifacts`

## Phase Plan
### Phase 0 - Intake
- Confirm PL15R reversal posture, WB19 queue objective, and current WB11
  lateral/drainage surrogate behavior baseline.

### Phase 1 - Canonical Physics-Contract Implementation
- Implement required lateral/drainage physics authority in canonical `SC-*`
  contracts, including equation forms, symbol/alias continuity, subsurface/
  drainage branch semantics, units, and guard invariants.
- Publish WB19 lateral/drainage physics provenance mapping from baseline legacy
  authority to canonical contract equations before kernel code edits.

### Phase 2 - Contract-Test Implementation + Gate (Pre-Implementation)
- Implement contract-derived lateral/drainage tests from amended canonical
  authority.
- Execute and record pre-implementation contract-gate evidence before
  production kernel code edits.

### Phase 3 - Kernel Implementation
- Implement lateral/drainage production behavior to match canonical
  lateral/drainage physics-contract authority and remove surrogate-only logic
  from production claim basis.

### Phase 4 - Verification
- Run targeted kernel/integration tests and required repository gates.

### Phase 5 - Disposition
- Publish WB19 contract/contract-test/implementation evidence and closure
  posture.

## Exit Criteria
- `WB19` queue objective is evidence-backed.
- Canonical science contracts explicitly contain lateral/drainage physics
  authority (equations, symbols, units, guards, branch and flux semantics) in
  `SC-SUBHYD-001` and `SC-WATBAL-001`; work-package notes alone do not satisfy
  closure.
- Legacy lateral/drainage migration provenance is explicit and reproducible
  against pinned `/workdir/wepp-forest_260430_baseline` authority in the WB19
  provenance-map artifact.
- Production lateral/drainage behavior is equation-driven and layer-aware and
  no longer governed by WB11 surrogate scalar fraction-split reduction.
- Contract-derived WB19 tests are implemented and executed.
- Pre-implementation contract-gate evidence exists and shows contract-test
  implementation completed before production kernel code edits.
- Lateral/drainage hydraulic vector and branch-response evidence are produced.
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
- Rationale: lateral/drainage kernel physics-authority and contract/test
  implementation.

## Execution Fallback (Policy-Filter Safe)
If an execution agent receives a false-positive policy block while running
WB19, continue with a minimal scoped prompt and micro-steps:

1. Scope:
   - local repository flat-file reads/edits only;
   - science-contract and Rust kernel migration work only.
2. Run in short phases (separate prompts):
   - phase A: contract-only updates (`SC-SUBHYD-001`, `SC-WATBAL-001`, index);
   - phase B: contract-derived tests + pre-implementation gate artifact;
   - phase C: production code edits;
   - phase D: verification + disposition artifacts.
3. Avoid large multi-file prompt payloads; reference file paths and specific
   headings/functions instead.
4. Preserve all WB19 exit criteria and evidence requirements; this fallback is
   prompt-shape mitigation only, not a scope change.
