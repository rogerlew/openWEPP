# 20260601-hphys0241-mofe-hourly-carry-arrays-routing-continuity-001

## Status
- state: queued
- date: 2026-06-01
- timezone: America/Los_Angeles
- decision: HOLD_PENDING_EXECUTION

## Objective
Close HPHYS0237 Dispatch Group C by implementing baseline-authoritative MOFE
hourly carry-array runtime surfaces and routing-continuity handoffs for
upstream/runon/lateral carry terms in hourly lane mode.

## Why This Package Exists
HPHYS0237 identified missing hourly upstream/runon carry arrays
(`ui_SUrunf`, `ui_SCrunf`, `ui_LfUrf`, `ui_LfCrf`) as a remaining
not-migrated gap. HPHYS0239 left the HPHYS stream in `HOLD` until the
remaining Dispatch-Group-B/C/D work is completed. This package follows the
HPHYS0240 runoff-carryover authority package and makes MOFE hourly routing
surfaces explicit rather than relying on daily aggregate or implicit state.

## Scope
### Included
- Contract amendments in:
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  - `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
  - `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- Contract-derived tests in:
  - `tests/integration/wb11_hydrology_kernel_contract.rs`
  - `tests/integration/cli03_runner_contract_derived_tests.rs`
  - `tests/integration/mofe04_publication_contract_authority_closure_contract.rs`
  - `tests/integration/mofe05_watershed_contributor_metadata_contract_authority_closure_contract.rs`
- Production implementation in:
  - `crates/openwepp-hillslope-orchestrator/src/constants.rs`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
  - `crates/openwepp-runner/src/hillslope/mod.rs`
  - `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- Workspace validation gates:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`

### Explicitly Out of Scope
- HPHYS0240 hourly runoff carryover closure.
- HPHYS0242 WB14/WB12 cadence and infiltration/ET/runoff/storage observation
  ordering closure.
- Erosion physics changes beyond preserving existing MOFE Wave-2/WB13/WAT
  boundary-carry semantics.

## Closure Measures (Required)
1. `MEASURE-HP241-001`: canonical contracts define MOFE hourly carry-array
   symbols, units, producer/consumer ownership, and fail-closed guard posture.
2. `MEASURE-HP241-002`: runner/orchestrator surfaces explicitly publish and
   consume hourly carry arrays without implicit daily aggregate substitution.
3. `MEASURE-HP241-003`: contract-derived tests prove multi-OFE hourly carry
   continuity and reject missing/non-finite/out-of-domain carry payloads.
4. `MEASURE-HP241-004`: watershed contributor metadata and MOFE publication
   policy remain consistent with explicit hourly carry surfaces.
5. `MEASURE-HP241-005`: required workspace gates pass and are recorded with
   truthful evidence labels.

## Deliverables
1. `artifacts/hphys0241-contract-implementation-evidence.md`
2. `artifacts/hphys0241-contract-test-implementation-evidence.md`
3. `artifacts/hphys0241-preimplementation-contract-gate.md`
4. `artifacts/hphys0241-implementation-and-test-evidence.md`
5. `artifacts/hphys0241-kernel-profile-compliance-checklist.md`
6. `artifacts/owned-file-manifest.md`
7. `artifacts/gate-results.md`
8. `artifacts/hphys0241_disposition.md`
9. `artifacts/worker-handoff.md`
10. `artifacts/review_agent_a.md`
11. `artifacts/review_agent_b.md`
12. `artifacts/verification_agent_a.md`
13. `artifacts/verification_agent_b.md`

## Mandatory Sequence (Required)
1. Amend canonical contracts for MOFE hourly carry-array authority.
2. Add/adjust contract-derived tests.
3. Record pre-implementation contract gate.
4. Modify production code.
5. Run required workspace gates.
6. Publish disposition and worker handoff.

## Autonomous Execution Intent (Required)
Execute phases end-to-end through disposition without requesting additional
user direction unless hard-blocked.

## Truthfulness Labeling Requirement
Each artifact must label evidence class (`Static:` vs `Ran:`).

## Physics Authority Requirements
- Canonical `SC-*` contracts are the only authority for new/changed process
  physics; package-local notes are evidence, not authority replacement.
- Physics/equation authority defaults to `/workdir/wepp-forest_260430_baseline`
  at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Do not invent or approximate MOFE hourly carry-array or routing-continuity
  physics; every equation, constant, guard, and invariant must trace to
  canonical contract text plus provenance citations.
- Preserve legacy WEPP variable naming continuity for `ui_SUrunf`,
  `ui_SCrunf`, `ui_LfUrf`, and `ui_LfCrf`; record explicit alias mappings where
  openWEPP boundary names differ.
- If baseline-authoritative carry-array closure is not completed, keep
  disposition in `HOLD` and open a follow-on package rather than merging
  placeholder/proxy physics.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0237-hourly-bulk-routine-inventory-and-dispatch-001/artifacts/hphys0237-hourly-routine-inventory.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0239-wb19-wb12-hourly-ordering-handoff-closure-001/artifacts/hphys0239_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0240-hourly-runoff-carryover-authority-closure-001/package.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- `/workdir/wepp-forest_260430_baseline/src/route.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0241-mofe-hourly-carry-arrays-routing-continuity-001/**`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `tests/integration/wb11_hydrology_kernel_contract.rs`
- `tests/integration/cli03_runner_contract_derived_tests.rs`
- `tests/integration/mofe04_publication_contract_authority_closure_contract.rs`
- `tests/integration/mofe05_watershed_contributor_metadata_contract_authority_closure_contract.rs`
- `crates/openwepp-hillslope-orchestrator/src/constants.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`

## Phase Plan
### Phase A - Contract authority amendment
- Amend canonical contracts for `ui_SUrunf`, `ui_SCrunf`, `ui_LfUrf`,
  `ui_LfCrf`, producer/consumer ownership, units, and guard posture.

### Phase B - Contract-derived tests + pre-implementation gate
- Add tests for multi-OFE hourly carry continuity and malformed carry payload
  rejection; record the pre-implementation gate before production edits.

### Phase C - Production implementation + gates
- Implement explicit runtime surfaces and runner/orchestrator seeding/publication
  paths, then run required workspace gates.

### Phase D - Disposition + handoff
- Publish evidence, reviews, verification, disposition, and handoff to
  HPHYS0242.

## Exit Criteria
- `MEASURE-HP241-001..005` satisfied and evidenced.
- Disposition explicitly records whether Dispatch Group C is closed and whether
  HPHYS stream remains `HOLD` for Group D.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel/runner/tests/docs edits only; no credentials/network writes.
