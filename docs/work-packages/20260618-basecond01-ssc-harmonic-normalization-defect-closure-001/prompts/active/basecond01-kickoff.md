# BASECOND01 Kickoff - Vertical `ssc` Harmonic Normalization

Close defect `BASECOND01-SSC-HARMONIC-NORMALIZATION` end-to-end.

## Autonomy

Execute through contract amendment, contract-derived tests, production
correction, validation, review, verification, and disposition without asking for
next steps unless a named boundary blocker is reached.

## Correction Authority Envelope

In scope:

- `SC-INFILE-SOIL-001` parser-to-runtime conductivity projection authority.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/soil.rs`.
- `tests/integration/parser_runtime_seam_integration/common.rs`.
- Package evidence artifacts, work-package index, and roadmap status.

Protected boundaries:

- Do not change WB19 lateral equation, withdrawal, active-layer selection,
  `drfc`, or `ksatadj`.
- Do not make hourly `wb19_lateral_ssh_####` harmonic.
- Do not loosen typed guards or add silent defaults.

Conversion rule:

If the in-scope defect is reproduced and the expected behavior is supported by
canonical contract authority plus pinned baseline provenance, land the
contract-first fix in this package. Do not relay another diagnostic step.

## Required Reading

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260618-basecond01-ssc-harmonic-normalization-defect-closure-001/package.md`

Conditional:

- `/home/workdir/openWEPP/docs/defect_closure_execplans.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md`
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/home/workdir/openWEPP/crates/AGENTS.md`

On-demand:

- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/home/workdir/openWEPP/docs/work-packages/20260618-stage2-base-conductivity-h2637-magnitude-001/artifacts/base-cond_disposition.md`
- `/home/workdir/openWEPP/docs/work-packages/20260618-stage2-base-conductivity-h2637-magnitude-001/artifacts/base-cond-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/input.for`

Required-reading budget:

- Local bytes total: `366583`.
- Disposition: `OK`.
- Authority map: `artifacts/required-reading-map.md`.

## Execution Requirements

- Contract-first sequence:
  1. contract amendment;
  2. contract-derived tests;
  3. pre-implementation gate;
  4. production code edit.
- Gate evidence non-deferral: a phase can close only when its current-scope
  gates have current evidence.
- Conservation/output acceptance: H2637 rerun evidence must report WAT/PASS
  checksums and aggregate magnitude deltas; one-sided checks alone are not
  enough.
- Subagent authorization: this prompt explicitly authorizes spawning/delegating
  to read-only review and verification subagents for bounded package diff,
  evidence, and gate-legitimacy review. Expected outputs are compact findings
  for the package review/verification artifacts. Write access is not authorized.
