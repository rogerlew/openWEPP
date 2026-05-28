# 20260528-clim17-breakpoint-climate-baseline-parity-burndown-001

## Status
- state: complete
- date: 2026-05-28
- timezone: UTC
- decision: GO

## Objective
Execute CLIM17 to identify implementation gaps in openWEPP breakpoint climate
behavior versus `/workdir/wepp-forest_260430_baseline` and migrate the
breakpoint implementation to baseline-authoritative parity with contract-first
sequencing and dual-review gate requirements.

## Why This Package Exists
CLIM04 delivered the initial breakpoint runtime port, but this follow-on
package is required to reassess full breakpoint parity against the pinned
baseline authority and close any residual implementation drift. The user also
provided a concrete breakpoint-climate corpus anchor:
`/wc1/runs/un/unpalatable-rind`.

This package is implementation scoped. It must run contract-first: canonical
contract amendments, then contract-derived tests, then pre-implementation
contract gate evidence, then production code changes.

## Scope
### Included
- Baseline-to-openWEPP breakpoint parity assessment and gap matrix, anchored to:
  - `/workdir/wepp-forest_260430_baseline/src/brkpt.for`
  - `/workdir/wepp-forest_260430_baseline/src/stmget.for`
  - `/workdir/wepp-forest_260430_baseline/src/idat.for`
- Curated breakpoint comparator/input evidence using:
  - `/wc1/runs/un/unpalatable-rind`
- Canonical contract/index amendments required by discovered parity gaps:
  - `SC-CLIMATE-001`
  - `SC-INFILE-CLIMATE-001`
  - `SC-SYSTEM-001` (if breakpoint integration seam language requires update)
  - `SC-WATBAL-001` (if downstream forcing/closure seam language requires update)
  - `docs/specifications/science-contracts/index.md`
- Contract-derived tests that expose each discovered breakpoint gap.
- Production parser/runtime parity migration for discovered breakpoint gaps.
- Required validation gates plus dual review/dual verification evidence.

### Explicitly Out of Scope
- New climate algorithm features outside baseline breakpoint parity closure.
- Non-breakpoint climate branch redesign (`ibrkpt=0`) not required for
  discovered breakpoint parity gaps.
- Heuristic or surrogate process-physics substitutions in production paths.

## Deliverables
1. Breakpoint parity gap matrix:
   - `artifacts/clim17-breakpoint-gap-matrix.md`
2. Contract implementation evidence:
   - `artifacts/clim17-contract-implementation-evidence.md`
3. Contract-test implementation evidence:
   - `artifacts/clim17-contract-test-implementation-evidence.md`
4. Pre-implementation contract gate evidence:
   - `artifacts/clim17-preimplementation-contract-gate.md`
5. WC1 unpalatable-rind fixture evidence:
   - `artifacts/clim17-wc1-unpalatable-rind-fixture-evidence.md`
6. Implementation and test evidence:
   - `artifacts/clim17-implementation-and-test-evidence.md`
7. Kernel profile compliance checklist:
   - `artifacts/clim17-kernel-profile-compliance-checklist.md`
8. Package governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/clim17_disposition.md`
   - `artifacts/worker-handoff.md`
9. Dual review/verification artifacts (deal review gate requirements):
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract/index language for CLIM17 scope.
2. Implement contract-derived breakpoint parity tests.
3. Record pre-implementation contract-gate evidence.
4. Implement production parser/runtime breakpoint parity edits.

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority is in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Legacy parity source authority defaults to
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No heuristic/proxy process-physics substitutions are allowed in production
  parity closure claims.
- Variable naming continuity must be preserved with explicit alias mapping when
  runtime names differ from baseline/canonical symbols.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260522-clim01-wepp-forest-climate-model-discovery-and-specification-001/artifacts/climate-implementation-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260522-clim04-breakpoint-runtime-port-and-policy-reconciliation-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260522-clim04-breakpoint-runtime-port-and-policy-reconciliation-001/artifacts/clim04_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-clim07-climate-comparator-and-closure-evidence-001/artifacts/clim07_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260523-clim08-climate-governance-disposition-closeout-001/artifacts/clim08_disposition.md`
- `/workdir/wepp-forest_260430_baseline/src/brkpt.for`
- `/workdir/wepp-forest_260430_baseline/src/stmget.for`
- `/workdir/wepp-forest_260430_baseline/src/idat.for`
- `/wc1/runs/un/unpalatable-rind`

## Intended Write Set
- `docs/work-packages/20260528-clim17-breakpoint-climate-baseline-parity-burndown-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-input-contract/src/parsers/climate.rs`
- `crates/openwepp-climate-runtime-adapter/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `tests/integration/infile_climate_parser_contract.rs`
- `tests/integration/parser_runtime_seam_integration.rs`
- `tests/integration/clim07_climate_comparator_and_closure_contract.rs`
- `tests/fixtures/infile/climate/**`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm CLIM17 authorization and freeze scope to breakpoint (`ibrkpt=1`)
  baseline parity closure.
- Confirm `/wc1/runs/un/unpalatable-rind` corpus use for parity evidence.

### Phase B - Gap assessment and authority mapping
- Build explicit baseline-to-openWEPP breakpoint gap matrix from pinned
  baseline and current runtime/parser implementations.
- Classify each discovered gap as contract-only, test-only, runtime, or mixed.

### Phase C - Contract updates (first required gate)
- Amend canonical `SC-*` contracts and index language for all in-scope
  breakpoint parity gaps.

### Phase D - Contract-derived tests (second required gate)
- Implement breakpoint parity vectors for every discovered gap with explicit
  expected behavior assertions.

### Phase E - Pre-implementation contract gate (third required gate)
- Record gate evidence that contract and contract-derived tests are complete
  before production edits.

### Phase F - Production parity migration (fourth required gate)
- Implement parser/runtime breakpoint parity edits with typed fail-closed
  guards and no silent defaults/clamping.

### Phase G - Validation evidence run
- Execute required validation gates:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`

### Phase H - Dual review, dual verification, disposition
- Complete `review_agent_a.md` and `review_agent_b.md`.
- Complete `verification_agent_a.md` and `verification_agent_b.md`.
- Publish GO/HOLD disposition with explicit closure map for all CLIM17
  breakpoint gaps.

## Exit Criteria
- Baseline-authoritative breakpoint parity matrix is complete and
  dispositioned for all in-scope parser/runtime surfaces.
- Contract-first sequencing evidence is complete and truthful.
- Breakpoint gaps discovered by CLIM17 are either closed or explicitly retained
  with evidence-backed HOLD ownership.
- `/wc1/runs/un/unpalatable-rind` evidence is captured and traceable.
- Dual review and dual verification artifacts are complete.
- Required validation gates pass and are recorded truthfully.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local contracts/runtime/tests and package artifacts only; no
  network or credential surface changes.
