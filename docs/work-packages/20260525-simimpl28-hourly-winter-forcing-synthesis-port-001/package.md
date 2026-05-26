# 20260525-simimpl28-hourly-winter-forcing-synthesis-port-001

## Status
- state: complete
- date: 2026-05-25
- timezone: UTC
- decision: HOLD

## Objective
Port baseline-authoritative hourly winter forcing synthesis surfaces from
`/workdir/wepp-forest_260430_baseline` (`sunmap`, `radcur`, `hr_tmp`,
`stmtim`) into openWEPP runtime seams so hourly snow/frost consumers receive
deterministic typed forcing payloads.

## Why This Package Exists
SIMIMPL27 closed boundary/API alias authority for hourly winter migration and
queued SIMIMPL28 to implement runtime emission of required hourly forcing
families. Current runtime surfaces publish daily climate forcing only and do
not emit reserved `snow.hourly.*` / `winter.hourly.*` symbol families.

## Scope
### Included
- Canonical contract amendments required to author SIMIMPL28 forcing-synthesis
  authority and symbol emission requirements.
- Contract-derived tests for hourly forcing synthesis, reserved alias families,
  and typed failure posture.
- Pre-implementation contract gate evidence.
- Production runtime seam edits for hourly forcing synthesis and boundary
  emission.
- Runner integration updates needed to execute climate seeding with static
  context for winter forcing synthesis.
- Governance artifacts, dual review, and dual verification through disposition.

### Explicitly Out of Scope
- Snow energy-balance kernel migration (`snowd`, `melt`) and deep frost
  equation closure (SIMIMPL29 scope).
- Tier-A/MOFE semantic parity rerun/disposition (SIMIMPL30 scope).

## Deliverables
1. Contract implementation evidence:
   - `artifacts/simimpl28-contract-implementation-evidence.md`
2. Contract-test implementation evidence:
   - `artifacts/simimpl28-contract-test-implementation-evidence.md`
3. Pre-implementation contract gate:
   - `artifacts/simimpl28-preimplementation-contract-gate.md`
4. Runtime implementation and gate evidence:
   - `artifacts/simimpl28-implementation-and-test-evidence.md`
5. Kernel profile compliance checklist:
   - `artifacts/simimpl28-kernel-profile-compliance-checklist.md`
6. Hourly forcing port mapping report:
   - `artifacts/simimpl28-hourly-forcing-port-mapping.md`
7. Reserved hourly alias coverage report:
   - `artifacts/simimpl28-reserved-hourly-alias-coverage.md`
8. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl28_disposition.md`
   - `artifacts/worker-handoff.md`
9. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
10. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. implement required canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

## Autonomous Execution Intent (Required)
This package is execution-ready and intended for autonomous end-to-end
completion through disposition without additional user intervention unless
hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must include explicit `Static:` and/or `Ran:` sections.

## Provenance and Authority Posture
- Canonical authority remains in `SC-*` contract files.
- Legacy baseline migration authority defaults to:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No heuristic/proxy process-physics substitutions are permitted as final
  closure behavior.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260525-snowplan01-hourly-energy-balance-snow-assessment-and-queue-001/artifacts/snowplan01-snow-hourly-energy-balance-wp-queue.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/aspect.for`
- `/workdir/wepp-forest_260430_baseline/src/psolr.for`
- `/workdir/wepp-forest_260430_baseline/src/sunmap.for`
- `/workdir/wepp-forest_260430_baseline/src/radcur.for`
- `/workdir/wepp-forest_260430_baseline/src/hrtmp.for`
- `/workdir/wepp-forest_260430_baseline/src/hr_tmp.for`
- `/workdir/wepp-forest_260430_baseline/src/stmtim.for`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`

## Intended Write Set
- `docs/work-packages/20260525-simimpl28-hourly-winter-forcing-synthesis-port-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-climate-runtime-adapter/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`

## Phase Plan
### Phase A - Intake and Authority Freeze
- Confirm queue authorization and SIMIMPL27 handoff requirements.

### Phase B - Canonical Contract Authority Amendments
- Add explicit SIMIMPL28 hourly forcing synthesis authority and symbol
  emission requirements to canonical `SC-*` contracts.

### Phase C - Contract-Derived Tests
- Implement tests validating deterministic hourly forcing emission, reserved
  alias projection, and typed failure posture.

### Phase D - Pre-Implementation Contract Gate
- Record gate evidence demonstrating steps 1-3 are complete before production
  code edits.

### Phase E - Runtime Implementation and Validation
- Implement runtime seam synthesis and runner integration updates.
- Execute required validation gates and capture evidence.

### Phase F - Governance, Review, Verification, Disposition
- Complete required artifacts, dual review/verification, and final disposition.

## Exit Criteria
- Required canonical `SC-*` amendments for SIMIMPL28 are complete.
- Contract-derived tests exist and pass for SIMIMPL28 scope.
- Runtime seams emit deterministic hourly winter forcing symbol families for
  active winter coupling with typed failure posture.
- Required gates pass and are recorded:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: local runtime-seam and contract/test updates with no external
  connectivity or credential surface changes.
