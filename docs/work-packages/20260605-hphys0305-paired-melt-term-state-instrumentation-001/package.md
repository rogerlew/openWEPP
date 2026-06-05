# HPHYS0305 Paired Melt-Term State Instrumentation

## Status

HOLD after execution: paired fixed-baseline `amelt` surfaces are incomplete for
all nine required target windows, so production physics edits are not
authorized.

## Objective

Execute ADR-0016 Required Continuation Order step 2 after HPHYS0304: implement
paired fixed-baseline/openWEPP instrumentation for melt-term, forcing, and snow
state surfaces needed to identify the first divergent source of the H1/H7/H39
snow/`RM` target-window residuals.

## Rationale

HPHYS0302 proved that aggregate `RM`, `Snow-Water`, raw `hrmlt`, and routed
`wmelt` output surfaces are not sufficient term-level authority. HPHYS0304
reruns the semantic suite against the fixed comparator, but production edits
still require paired baseline/openWEPP term-state evidence. This package must
collect that evidence before any snow, melt, forcing, WB13, WB17, WB18, WB19,
or WB12 production change is considered.

## Included Scope

- Amend canonical `SC-*` contracts first if new term/state surfaces or aliases
  are needed.
- Add contract-derived tests for paired instrumentation semantics before
  production/runtime instrumentation edits.
- Instrument baseline fixed comparator observe surfaces for:
  `amelt`, `bmelt`, `cmelt`, `dmelt`, `hrrain`, `hrtemp`, `tdpt`, `hrad`,
  `cloudC`, `vwind`, `snodpt`, and `densgt`.
- Instrument openWEPP trace surfaces with explicit canonical alias mappings for
  the same symbols.
- Run the paired instrumentation on the nine H1/H7/H39 target windows and
  classify the first divergent source per window.
- Run the full H1..H39 semantic suite after instrumentation-only changes when
  feasible, preserving HPHYS0304 fixed-baseline comparator identity.
- Complete dual review, finding disposition, dual verification, gate results,
  disposition, and worker handoff.

## Excluded Scope

- Production physics corrections before paired term/state evidence and
  contract-derived tests identify an authorized edit target.
- Downstream compensation in WB13/WB17/WB18/WB19/WB12.
- Silent defaults, clamping, canonicalize-and-proceed, or heuristic process
  physics.
- Treating aggregate daily WAT residuals as term-level producer authority.

## Dependencies

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0304-fixed-comparator-semantic-rerun-continuation-001/artifacts/disposition.md`
- `/home/workdir/openWEPP/docs/work-packages/20260605-hphys0304-fixed-comparator-semantic-rerun-continuation-001/artifacts/snow-rm-window-reclassification.json`
- `/home/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/home/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/home/workdir/openWEPP/docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md`
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/wepp-forest_260430_baseline` at fixed comparator commit
  `47ac4c32faeea81bb99081f955a14c38b815ef4d`.
  This fixed comparator is the ADR-0016/HPHYS0303 ratified `wepp_260430`
  comparator for negative-melt carried-state authority.

## Intended Write Set

- `docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/**`
- Canonical `SC-*` contract files only if instrumentation terms/aliases need
  contract authority.
- Baseline observe/instrumentation patches in the fixed comparator worktree
  only as local evidence unless separately authorized.
- openWEPP instrumentation and tests only after contract/test/gate completion.

## Phase Plan

1. **Contract authority**: amend canonical SC contracts with any missing
   paired term/state symbol definitions, units, aliases, and provenance.
2. **Contract-derived tests**: add tests that fail without paired term/state
   instrumentation and enforce no downstream compensation.
3. **Pre-implementation gate**: run contract gates and record truth-labeled
   evidence before production/runtime edits.
4. **Paired instrumentation**: implement fixed-baseline and openWEPP
   instrumentation for the listed term/state surfaces.
5. **Target-window execution**: run instrumentation for all nine target windows
   and identify first divergent sources.
6. **Full-suite context**: rerun H1..H39 semantic metrics when feasible.
7. **Review and disposition**: complete dual independent review, disposition
   every finding, dual verification, final disposition, and worker handoff.

## Progress

- [x] (2026-06-05T21:03:04Z) Scaffolded as the queued ADR-0016 Required
  Continuation Order step 2 package by HPHYS0304.
- [x] Execute contract authority phase.
- [x] Execute contract-derived tests.
- [x] Execute pre-implementation gate.
- [x] Execute paired instrumentation.
- [x] Execute target-window classification and full-suite context.
- [x] Complete review, disposition, verification, and handoff.

## Execution Outcome

HPHYS0305 completed contract-first instrumentation and targeted H1/H7/H39
execution, but it remains in `HOLD` because the fixed-baseline observe stream
does not provide complete `amelt` paired surfaces for the nine required target
windows. The strict package rule treats that as `paired-surface-gap` and routes
all rows to `surface-gap-hold`; no production physics correction or downstream
compensation is authorized.

## Exit Criteria

- Paired baseline/openWEPP term-state evidence exists for all nine H1/H7/H39
  target windows.
- First divergent source classification is explicit for each window or a
  concrete blocker is recorded.
- Any production edit recommendation is tied to canonical SC authority,
  contract-derived tests, and paired term/state evidence.
- No downstream compensation is authorized without source-owned evidence.
- Dual review and verification artifacts have no undispositioned findings.

## Security Impact Gate

No credentials, network actions, branch creation in openWEPP, or remote pushes
are in scope. Work is limited to local flat-file reads/edits, local comparator
instrumentation, local artifact generation, and local tests.
