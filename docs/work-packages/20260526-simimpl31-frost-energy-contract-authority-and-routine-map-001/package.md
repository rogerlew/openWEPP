# 20260526-simimpl31-frost-energy-contract-authority-and-routine-map-001

## Status
- state: package-complete-with-hold
- date: 2026-05-26
- timezone: UTC
- decision: HOLD

## Objective
Close canonical `SC-SNOWFREEZE-001` frost process-authority gaps by mapping the
baseline frost routine chain (`winter` + `frostn` family +
`getFreezeCond`) to explicit openWEPP boundary/state aliases and invariants,
with downstream contract-derived test requirements for SIMIMPL32.

## Why This Package Exists
FROSTPLAN01 queued SIMIMPL31 as the required contract-first entry package for
frost process parity. SIMIMPL30 remains `HOLD`, with unresolved
`frost.hourly.*` process-family closure and comparator-lane admissibility
prerequisites.

SIMIMPL31 executes contract-first step 1 for frost-process scope so downstream
SIMIMPL32+ implementation packages proceed without routine-authority ambiguity.

## Scope
### Included
- Canonical contract amendments for:
  - `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  - `docs/specifications/science-contracts/index.md`
- Baseline frost routine-chain authority mapping for:
  - `winter.for`, `frostn.for`, `frsoil.for`, `frwatc.for`, `frzng.for`,
    `frznw.for`, `winthd.for`, `getfreezecond.for`
- Explicit mapping of routine-chain outputs/controls to openWEPP boundary alias
  families (`frost.runtime_*`, reserved `frost.hourly.*`, and companion winter
  hourly forcing/state families).
- Explicit downstream contract-derived test requirements for SIMIMPL32.
- Governance artifacts and handoff for SIMIMPL32/33/34/35 sequence.

### Explicitly Out of Scope
- Contract-derived test implementation in code (SIMIMPL32 scope).
- Production runtime/kernel code edits.
- Winter-hourly parity rerun/disposition execution (SIMIMPL35 scope).

## Deliverables
1. Contract amendment log:
   - `artifacts/simimpl31-contract-authority-amendment-log.md`
2. Frost routine authority map:
   - `artifacts/simimpl31-frost-routine-authority-map.md`
3. Cross-contract gap disposition:
   - `artifacts/simimpl31-cross-contract-gap-disposition.md`
4. Contract implementation evidence:
   - `artifacts/simimpl31-contract-implementation-evidence.md`
5. Contract-test implementation evidence:
   - `artifacts/simimpl31-contract-test-implementation-evidence.md`
6. Pre-implementation contract gate:
   - `artifacts/simimpl31-preimplementation-contract-gate.md`
7. Implementation/test evidence:
   - `artifacts/simimpl31-implementation-and-test-evidence.md`
8. Kernel profile checklist:
   - `artifacts/simimpl31-kernel-profile-compliance-checklist.md`
9. Governance artifacts:
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/simimpl31_disposition.md`
   - `artifacts/worker-handoff.md`
10. Dual review artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
11. Dual verification artifacts:
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
For downstream code-authoring packages in this wave:
1. implement required canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

SIMIMPL31 executes step 1 for frost process migration scope.

## Autonomous Execution Intent (Required)
This package is execution-ready and intended for end-to-end autonomous
completion through disposition without additional user direction unless
hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must include explicit `Static:` and/or `Ran:` sections.

## Provenance and Authority Posture
- Canonical authority remains in `SC-*` contract files.
- Legacy baseline migration authority defaults to:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No heuristic/proxy process-physics substitutions are permitted.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260526-frostplan01-frost-energy-solver-assessment-and-queue-001/artifacts/frost-energy-solver-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl30-winter-hourly-semantic-parity-rerun-and-disposition-001/artifacts/simimpl30_disposition.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/winter.for`
- `/workdir/wepp-forest_260430_baseline/src/frostn.for`
- `/workdir/wepp-forest_260430_baseline/src/frsoil.for`
- `/workdir/wepp-forest_260430_baseline/src/frwatc.for`
- `/workdir/wepp-forest_260430_baseline/src/frzng.for`
- `/workdir/wepp-forest_260430_baseline/src/frznw.for`
- `/workdir/wepp-forest_260430_baseline/src/winthd.for`
- `/workdir/wepp-forest_260430_baseline/src/getfreezecond.for`

## Intended Write Set
- `docs/work-packages/20260526-simimpl31-frost-energy-contract-authority-and-routine-map-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/index.md`

## Phase Plan
### Phase A - Intake and Authority Freeze
- Confirm FROSTPLAN01 queue objective and SIMIMPL30 carry-forward HOLD
  constraints.

### Phase B - Canonical Contract Authority Amendments
- Ratify explicit frost routine-chain authority mapping and downstream
  contract-derived test requirements in `SC-SNOWFREEZE-001`.

### Phase C - Cross-Contract Gap Reclassification
- Reclassify frost-process authority posture for migration scope with explicit
  companion contract ownership references.

### Phase D - Governance and Handoff
- Produce evidence, review, verification, and handoff artifacts.

### Phase E - Disposition
- Publish package disposition and queued follow-on requirements.

## Exit Criteria
- `SC-SNOWFREEZE-001` no longer carries routine-authority ambiguity for
  SIMIMPL31 migration scope.
- Downstream contract-derived test requirements are explicit for SIMIMPL32.
- Required governance artifacts are complete with truthful evidence labels.
- If non-doc files are changed, required gates are run and recorded:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- Rationale: contract-authoring package; no production runtime mutation.
