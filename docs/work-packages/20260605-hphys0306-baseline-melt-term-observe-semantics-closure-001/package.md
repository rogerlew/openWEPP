# HPHYS0306 Baseline Melt-Term Observe Semantics Closure

## Status

HOLD after execution: branch-active comparison reclassified the HPHYS0305
missing-`amelt` blocker into eight melt-call mask holds and one H39 first-2013
same-hour `cmelt`/`snodpt` hold. No production physics edit is authorized.

## Objective

Execute the HPHYS0305 required continuation by making fixed-baseline
branch-active/inactive melt-term observe semantics explicit, reclassifying the
nine H1/H7/H39 target windows on that domain, and preserving the production-edit
`HOLD` unless a source-owned divergence is identified from paired same-unit
evidence.

## Rationale

HPHYS0305 proved fixed-comparator observe identity and added paired term/state
surfaces, but every target-window ledger row stopped at
`paired-surface-gap:amelt`. Follow-up inspection shows the apparent gap is
dominated by openWEPP inactive-hour trace publication while fixed-baseline
`melt.for` terms only have authority when the melt routine is called. HPHYS0306
must close that comparison-domain ambiguity before any snow producer, forcing,
WB13, WB17, WB18, WB19, or WB12 edit can be considered.

## Included Scope

- Amend canonical `SC-*` authority for branch-active/inactive melt-term observe
  semantics.
- Add contract-derived tests requiring complete branch-active paired ledgers or
  typed `HOLD` classification.
- Reclassify HPHYS0305 fixed-baseline/openWEPP paired traces using:
  - fixed-baseline melt-call presence as the authoritative baseline active
    domain,
  - openWEPP `snow_hourly_melt_branch_active` as the candidate active domain,
  - same-unit comparisons only on the active comparison domain.
- Detect and classify melt-call mask divergence before comparing forcing,
  snow-state, or melt-term magnitudes.
- Preserve fixed-comparator identity and HPHYS0305 observe evidence; no
  production physics edits are authorized by this package.
- Complete dual review, finding disposition, dual verification, gate results,
  disposition, and worker handoff.

## Excluded Scope

- Changing production snow, winter, runoff, ET, storage, percolation, lateral,
  routing, or WB13 publication physics.
- Zero-imputing inactive fixed-baseline melt terms.
- Treating inactive-hour openWEPP trace publication as baseline term-state
  authority.
- Downstream compensation in WB13/WB17/WB18/WB19/WB12.

## Dependencies

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts/disposition.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts/worker-handoff.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts/paired-melt-term-state-ledger.json`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts/baseline-observe-identity.json`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/artifacts/openwepp-trace-field-audit.json`
- `/home/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/home/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/home/workdir/openWEPP/docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md`
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/wepp-forest_260430_baseline` at ADR-0016/HPHYS0303 fixed
  comparator commit `47ac4c32faeea81bb99081f955a14c38b815ef4d`.

## Intended Write Set

- `docs/work-packages/20260605-hphys0306-baseline-melt-term-observe-semantics-closure-001/**`
- `docs/work-packages/README.md`
- Canonical `SC-*` contract files for branch-active observe semantics.
- Contract tests under `tests/integration/`.
- Package-local diagnostic scripts and evidence artifacts only.

## Phase Plan

1. **Contract authority**: amend canonical SC contracts with branch-active
   melt-term observe semantics, aliases, and no-zero-imputation guard.
2. **Contract-derived tests**: add tests for package/prompt requirements,
   runner branch-domain behavior, ledger semantics, and no remote actions.
3. **Pre-implementation gate**: run contract and anti-evasion gates before
   diagnostic execution.
4. **Branch-domain reclassification**: build the HPHYS0306 paired ledger from
   HPHYS0305 fixed-baseline observe and openWEPP trace evidence.
5. **Review and disposition**: complete dual independent review, disposition
   every finding, dual verification, final disposition, and worker handoff.

## Progress

- [x] Execute contract authority phase.
- [x] Execute contract-derived tests.
- [x] Execute pre-implementation gate.
- [x] Execute branch-domain reclassification.
- [x] Complete review, disposition, verification, and handoff.

## Execution Outcome

HPHYS0306 converted the HPHYS0305 all-row `paired-surface-gap:amelt` blocker
into a branch-active classification:

- eight target windows route `melt-call-mask` /
  `branch-active-mask-hold`;
- H39 first-2013 has closed branch-active masks and routes
  `same-hour-multi-source:cmelt,snodpt` / `same-hour-multi-source-hold`;
- no row authorizes production physics edits or downstream compensation.

## Exit Criteria

- Canonical SC authority states branch-active/inactive melt-term observe
  semantics and prohibits inactive-hour zero imputation.
- The HPHYS0306 ledger exists for all nine H1/H7/H39 target windows.
- Each row has a branch-active mask status, a first-source classification, a
  route, and an explicit production-edit authorization flag.
- Production edits remain blocked unless paired same-unit active-domain
  evidence identifies a source-owned correction target.
- Dual review and verification artifacts have no undispositioned findings.

## Security Impact Gate

No credentials, network actions, branch creation, or remote pushes are in
scope. Work is limited to local flat-file reads/edits, package-local diagnostic
execution, local tests, and local artifact generation.
