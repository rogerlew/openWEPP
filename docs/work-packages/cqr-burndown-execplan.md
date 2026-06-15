# Using PLANS.md for multi-hour problem solving

Codex and the `gpt-5.2-codex` model (recommended) can be used to implement complex tasks that take significant time to research, design, and implement. The approach described here is one way to prompt the model to implement these tasks and to steer it towards successful completion of a project.

These plans are thorough design documents, and "living documents". As a user of Codex, you can use these documents to verify the approach that Codex will take before it begins a long implementation process. The particular `PLANS.md` included below is very similar to one that has enabled Codex to work for more than seven hours from a single prompt.

We enable Codex to use these documents by first updating `AGENTS.md` to describe when to use `PLANS.md`, and then of course, to add the `PLANS.md` file to our repository.

## openWEPP Work-Package Addendum

Every openWEPP work-package ExecPlan must require dual independent reviews
before final disposition. The plan must require every review finding to be
explicitly dispositioned as `accepted`, `rejected`, `deferred`, or `follow-up`
with rationale. Accepted findings must be fixed and verified. Rejected findings
must explain why no change is required. Deferred or follow-up findings must be
linked from the disposition and worker-handoff artifacts. Package closure is
blocked while any review finding is undispositioned.

Every openWEPP work-package ExecPlan must also enforce the **Gate Evidence
Non-Deferral Rule** (canonical statement: `docs/work-packages/AGENTS.md`).
A package, phase, or staged increment may be marked complete only
when every required current-scope exit criterion has direct evidence in the
current artifact set. If required evidence depends on a later phase/increment,
the current phase is `HOLD` / `executed-hold` with that dependency named as the
blocker. Plans may narrow or move a gate only before implementation begins and
with explicit artifact/review justification; after execution starts, an unmet
current gate cannot be rebranded as "next increment scope." Reviews and
verifications must check this rule explicitly.

When an ExecPlan expects delegated reviewers, verifiers, comparator runners, or
other role agents, it must explicitly authorize subagent spawning/delegation in
both the package body and kickoff prompt. The authorization must name the
role(s), scope, expected compact outputs/artifacts, and read/write limits.
Phrases such as "dispatch `<role>`" or references to agent config files are not
sufficient unless paired with explicit authorization wording.

ExecPlans must also require explicit `.rs` file line-count governance checks in
review artifacts and checklists: files at or above 2000 lines are `WARN` and
must carry a decomposition rationale plus follow-on split intent; files at or
above 3000 lines require refactor before closure unless a generated/fixture
exception is explicitly approved with owner and sunset plan. Review artifacts
must record exception disposition, and package closure is blocked if any 3000+
non-exempt file remains undispositioned.

Mechanical refactor packages should additionally follow
`docs/standards/mechanical-refactor-authoring-guide.md` for explicit seam
definition, tool usage, refactor patterns, and compile/test closure flow.

ADR-0017 makes comparator agreement a flag rather than a target for all
comparator/ledger work packages. ExecPlans that classify legacy-comparator
residuals must require like-for-like unit and lineage-stage proof before any
`OPENWEPP-DEFECTIVE` verdict, must include `HARNESS-SURFACE-MISMATCH` as a peer
verdict for unit or surface-pairing defects, must prohibit waiving independent
correctness authority for openWEPP-defect labels, and must keep `HOLD` findings
owned by a named follow-on gate rather than unscoped.

ADR-0018 adds a required Defect-Closure ExecPlan (DC-ExecPlan) subtype for
closing observed invariant violations, fail-closed events on valid input, and
conservation residuals. A DC-ExecPlan must declare a Correction Authority
Envelope, diagnose internally to a named mechanism, and, when the root cause is
inside that envelope and the corrected behavior is backed by canonical `SC-*`
authority, pinned-baseline provenance, or a contract-authorized physical
invariant, proceed through contract amendment, contract-derived tests,
pre-implementation gate, production edit, validation, dual review, and
disposition in the same package. It may close in `HOLD` only at a declared
boundary, and its handoff's first actionable item must be "close defect `<id>`",
not a next inspection step. Reviews for DC-ExecPlans must check `HOLD`
legitimacy, envelope adequacy, and protected-boundary integrity. Authoring
details live in `docs/defect_closure_execplans.md`.

## CQR Top-30 CRAP Burn-Down ExecPlan

This ExecPlan is a living document. The sections `Progress`,
`Surprises & Discoveries`, `Decision Log`, and `Outcomes & Retrospective` must
be kept up to date as work proceeds.

Maintain this plan in accordance with the requirements in this file, the
work-package governance in `docs/work-packages/AGENTS.md`, the mechanical
refactor requirements in
`docs/standards/mechanical-refactor-authoring-guide.md`, and the code-quality
refactor requirements in
`docs/standards/code-quality-refactor-authoring-guide.md`.

### Purpose / Big Picture

The repo has a CRAP-metric burn-down backlog from a top-30 production-function
snapshot. CRAP combines cyclomatic complexity and test coverage, so high values
identify functions that are both hard to reason about and weakly exercised. The
goal of this ExecPlan is to continue the CQR series after CQR07 and CQR08 by
closing the remaining 28 ranked rows as separate behavior-preserving
code-quality refactor work packages.

After this plan is complete, each listed production function will have had its
own CQR package with scoped characterization, mechanical decomposition,
before/after CRAP and coverage evidence, full Rust closure gates, dual review,
dual verification, and final disposition. Progress is visible by checking off
the tracker below and by reading the corresponding package directories under
`docs/work-packages/`.

### Progress

- [x] (2026-06-15) Closed rank 1 as CQR07:
  `crates/openwepp-runner/src/watershed_wat.rs`, original CRAP `4830`.
- [x] (2026-06-15) Closed rank 2 as CQR08:
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs`,
  original CRAP `4290`.
- [x] (2026-06-15) Closed CQR09, rank 3, original CRAP `1497`, CC
  `79`, Cov `39%`:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/07_decomposition_equations.rs`.
  Package:
  `docs/work-packages/20260615-cqr09-decomposition-equations-complexity-001/`.
  Pushed commit: `e72588fb5b57ebd81c871fc9aa8ebf7a893e4afe` on branch `main`.
  Final target: `build_annual_decomposition_control`, CRAP
  `9.179748500041095`.
- [x] (2026-06-15) Closed CQR10, rank 4, original CRAP `1482`, CC
  `38`, Cov `0%`:
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`.
  Package:
  `docs/work-packages/20260615-cqr10-irrigation-fixeddate-runtime-001/`.
  Pushed commit: `0a2e469d69ee7883511710c7b6be739df0502581` on branch `main`.
  Final target: `seed_hillslope_runtime_surface_from_irrigation_fixeddate`,
  CRAP `4.0`.
- [x] (2026-06-15) Closed CQR11, rank 5, original CRAP `1406`, CC
  `37`, Cov `0%`:
  `crates/openwepp-input-contract/src/parsers/management.rs`.
  Package:
  `docs/work-packages/20260615-cqr11-management-parser-complexity-001/`.
  Pushed commit: `cb3947d9a9373b344a70dd72323a911e256abe95` on branch `main`.
  Final target: `parse_yearly_perennial`, CRAP `4.0`.
- [x] (2026-06-15) Closed CQR12, rank 6, original CRAP `1122`, CC
  `33`, Cov `0%`:
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`.
  Package:
  `docs/work-packages/20260615-cqr12-irrigation-depletion-runtime-001/`.
  Pushed commit: `96e39f68a0847e98c50399ad99ded88f5b4f2528` on branch `main`.
  Final target:
  `seed_hillslope_runtime_surface_from_irrigation_depletion`, CRAP `2.0`.
- [x] (2026-06-15) Closed CQR13, rank 7, original CRAP `964`, CC
  `65`, Cov `40%`:
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs`.
  Package:
  `docs/work-packages/20260615-cqr13-runtime-core-types-complexity-001/`.
  Pushed commit: `5e7a6c44f16c3f0962f509c64e0f5b661568e467` on branch `main`.
  Final target: `HillslopeRuntimeInputError::soil_core_code`, CRAP
  `14.0478515625`.
- [x] (2026-06-15) Closed CQR14, rank 8, original CRAP `650`, CC
  `25`, Cov `0%`:
  `crates/openwepp-runner/src/release.rs`.
  Package:
  `docs/work-packages/20260615-cqr14-runner-release-complexity-001/`.
  Pushed commit: `3b8b4f8b9c77b3a5b79d25ccdc0a834dc4229487` on branch `main`.
  Final target: `lint_release_directory`, CRAP `4.0`.
- [x] (2026-06-15) Closed CQR15, rank 9, original CRAP `581`, CC
  `94`, Cov `62%`:
  `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`.
  Package:
  `docs/work-packages/20260615-cqr15-scheduler-seed-runtime-complexity-001/`.
  Pushed commit: `b5dccb5d1d0a8f52a1030de95edc39894b0b893b` on branch `main`.
  Final target: `seed_wb11_runtime_surface_inputs`, CRAP `15.0`.
- [x] (2026-06-15) Closed CQR16, rank 10, original CRAP `506`, CC
  `22`, Cov `0%`:
  `crates/openwepp-sim-contract/src/units_mod/registries.rs`.
  Package:
  `docs/work-packages/20260615-cqr16-unit-registries-complexity-001/`.
  Pushed commit: `642a4c6bbdc9bb5b6388230eddad73066e7b3d85` on branch `main`.
  Final target: `BoundaryUnitRegistryError::fmt`, CRAP `6.0`.
- [x] (2026-06-15) Closed CQR17, rank 11, original CRAP `466`, CC `37`,
  Cov `32%`:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod19.rs`.
  Package:
  `docs/work-packages/20260615-cqr17-hydrology-erod19-complexity-001/`.
  Pushed commit: `a412b2939b24f246464b7c8a1379556acee2e158` on branch `main`.
  Final target: `Wb11HydrologyKernel::erod19_xcrit_classification`, CRAP
  `2.0`.
- [x] (2026-06-15) Closed CQR18, rank 12, original CRAP `456`, CC
  `80`, Cov `61%`:
  `crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`.
  Package:
  `docs/work-packages/20260615-cqr18-hbp-payload-validator-complexity-001/`.
  Pushed commit: `2a3cbd37cae178757a6ca3815a079631896b5452` on branch
  `main`.
  Final target: `validate_payload`, CRAP `9.0`.
- [x] (2026-06-15) Closed CQR19, rank 13, original CRAP `420`, CC
  `20`, Cov `0%`:
  `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/types.rs`.
  Package:
  `docs/work-packages/20260615-cqr19-watershed-runtime-types-complexity-001/`.
  Pushed commit: `bb6eade8193f19cd61fcc09a285001d41f245745` on branch
  `main`.
  Final target: `WatershedClimateRuntimeInputError::fmt`, CRAP `6.0`.
- [ ] CQR20, rank 14, original CRAP `384`, CC `29`, Cov `25%`:
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/05_projection_helpers.rs`.
- [ ] CQR21, rank 15, original CRAP `380`, CC `19`, Cov `0%`:
  `crates/openwepp-climate-runtime-adapter/src/lib.rs`.
- [ ] CQR22, rank 16, original CRAP `369`, CC `29`, Cov `26%`:
  `crates/openwepp-input-contract/src/parsers/soil.rs`.
- [ ] CQR23, rank 17, original CRAP `352`, CC `79`, Cov `65%`:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod19.rs`.
- [ ] CQR24, rank 18, original CRAP `317`, CC `58`, Cov `57%`:
  `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`.
- [ ] CQR25, rank 19, original CRAP `305`, CC `113`, Cov `75%`:
  `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`.
- [ ] CQR26, rank 20, original CRAP `300`, CC `122`, Cov `77%`:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs`.
- [ ] CQR27, rank 21, original CRAP `291`, CC `35`, Cov `41%`:
  `crates/openwepp-input-contract/src/parsers/management.rs`.
- [ ] CQR28, rank 22, original CRAP `282`, CC `91`, Cov `72%`:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_plant_percolation.rs`.
- [ ] CQR29, rank 23, original CRAP `272`, CC `16`, Cov `0%`:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs`.
- [ ] CQR30, rank 24, original CRAP `265`, CC `81`, Cov `70%`:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod13.rs`.
- [ ] CQR31, rank 25, original CRAP `252`, CC `76`, Cov `69%`:
  `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`.
- [ ] CQR32, rank 26, original CRAP `240`, CC `15`, Cov `0%`:
  `crates/openwepp-input-contract/src/parsers/climate.rs`.
- [ ] CQR33, rank 27, original CRAP `240`, CC `15`, Cov `0%`:
  `crates/openwepp-input-contract/src/parsers/watershed_structure.rs`.
- [ ] CQR34, rank 28, original CRAP `240`, CC `15`, Cov `0%`:
  `crates/openwepp-summary-accumulator/src/lib.rs`.
- [ ] CQR35, rank 29, original CRAP `239`, CC `64`, Cov `65%`:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs`.
- [ ] CQR36, rank 30, original CRAP `220`, CC `73`, Cov `70%`:
  `crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs`.

### Surprises & Discoveries

- Observation: the top-30 list is a snapshot rather than a live metric report.
  Ranks 1 and 2 have already been closed by CQR07 and CQR08, so each future
  package must re-run current LCOV and `cargo-crap` before choosing the exact
  target function inside the listed file.
  Evidence: CQR07 and CQR08 are recorded as complete in
  `docs/work-packages/README.md`.

- Observation: the same file appears multiple times in the remaining backlog.
  Evidence: `04_snow_frost_irrigation.rs`, `management.rs`,
  `hydrology_phase_erod19.rs`, `scheduler_seed_and_runtime.rs`, and
  `hydrology_phase_lateral_drainage.rs` each appear in more than one unchecked
  row.

- Observation: CQR09 live metrics matched the rank-3 snapshot target, but the
  same target file still contains pre-existing out-of-scope CRAP rows above
  `30`.
  Evidence: CQR09 after metrics reduced `build_annual_decomposition_control`
  to CRAP `9.179748500041095`; `build_perennial_decomposition_control` and
  `compute_equation_decomposition_seed_surface` remain above `30` in
  `docs/work-packages/20260615-cqr09-decomposition-equations-complexity-001/artifacts/crap_after.json`.

- Observation: CQR10 live metrics matched the rank-4 fixed-date irrigation
  target, and the duplicate-file CQR12 depletion row remains live and out of
  scope.
  Evidence: CQR10 after metrics reduced
  `seed_hillslope_runtime_surface_from_irrigation_fixeddate` to CRAP `4.0`;
  `seed_hillslope_runtime_surface_from_irrigation_depletion` remains CRAP
  `1122.0` in
  `docs/work-packages/20260615-cqr10-irrigation-fixeddate-runtime-001/artifacts/crap_after.json`.

- Observation: CQR11 live metrics matched the rank-5 management perennial
  parser target, and focused characterization exposed the exact stable error
  IDs for perennial invalid option and arity paths.
  Evidence: CQR11 after metrics reduced `parse_yearly_perennial` to CRAP
  `4.0`; the characterization suite records `MAN-E-004` for invalid
  `mgtopt` and `MAN-E-002` for perennial row arity errors. Out-of-scope
  management parser rows remain above `30` in
  `docs/work-packages/20260615-cqr11-management-parser-complexity-001/artifacts/crap_after.json`.

- Observation: CQR12 live metrics matched the rank-6 depletion irrigation
  runtime target; focused characterization corrected a fixture-order assumption
  around the sprinkler period fields before production refactor.
  Evidence: CQR12 after metrics reduced
  `seed_hillslope_runtime_surface_from_irrigation_depletion` to CRAP `2.0`;
  the package records the first sprinkler fixture period as `aprati=1.0` and
  `deplev=0.50`. The pre-existing frost `too_many_lines` suppression remains
  outside CQR12 scope in
  `docs/work-packages/20260615-cqr12-irrigation-depletion-runtime-001/artifacts/cqr12_disposition.md`.

- Observation: CQR13 live metrics proved the rank-7 runtime core type row was
  already closed by prior decomposition, so no production refactor was needed.
  Evidence: CQR13 before and after metrics both show the highest
  `00_core_types.rs` row as `HillslopeRuntimeInputError::soil_core_code` at
  CRAP `14.0478515625`, with `HillslopeRuntimeInputError::code` and
  `HillslopeRuntimeInputError::fmt` both at CRAP `9.0`, in
  `docs/work-packages/20260615-cqr13-runtime-core-types-complexity-001/artifacts/crap_after.json`.

### Decision Log

- Decision: preserve the top-30 ranking as the canonical tracker and assign
  CQR09 through CQR36 to ranks 3 through 30.
  Rationale: the user asked for the remaining 28 entries to be completed as
  their own CQR work packages. Keeping one checklist item per ranked row avoids
  bundling duplicate-file entries into one broader package.
  Date/Author: 2026-06-15 / Codex.

- Decision: each CQR package must re-measure current CRAP before implementation
  and may only close the ranked row when the current target function is
  identified and reduced to CRAP `<= 30`.
  Rationale: prior packages can alter later rankings, coverage, and helper
  names. A stale snapshot is useful for queue order but not sufficient evidence
  for package closure.
  Date/Author: 2026-06-15 / Codex.

- Decision: duplicate-file rows remain separate work packages.
  Rationale: this honors the user request and the code-quality guide's
  preference for one module and one quality dimension per package. If an earlier
  package in a duplicate file fully closes a later ranked row, the later package
  still needs an explicit short package or catalog disposition proving that the
  row is already closed by current metrics.
  Date/Author: 2026-06-15 / Codex.

- Decision: every CQR package in this tracker must be committed and pushed
  immediately after package closure.
  Rationale: the user explicitly requested commit-and-push after each completed
  CQR. Requiring the push before checking off the row keeps the tracker aligned
  with remote repository state rather than local-only completion.
  Date/Author: 2026-06-15 / Codex.

### Outcomes & Retrospective

CQR09 and CQR10 are complete-with-warnings and pushed to `origin/main`. CQR07
and CQR08 remain complete and serve as the precedent for package shape, metric
evidence, focused characterization, closure gates, review, verification, and
disposition.

### Context and Orientation

`CRAP` is a metric reported by `cargo-crap`. It rises when a function has high
cyclomatic complexity, low test coverage, or both. In this plan, "rank" means
the original position in the user-provided top-30 snapshot. "CQR" means a
code-quality refactor work package: a behavior-preserving package that improves
one quality dimension, records before/after evidence, and does not change
science, public API, typed guard semantics, parser compatibility, or output
formulas.

Each package directory must live under `docs/work-packages/` and use the
standard name shape `YYYYMMDD-cqrNN-<module-slug>-001`. Each package must update
`docs/work-packages/README.md` when it is scaffolded and again when it closes.

The completed precedents are:

- `docs/work-packages/20260615-cqr07-watershed-wat-complexity-001/`
- `docs/work-packages/20260615-cqr08-runtime-core-types-display-001/`

Future packages should follow those layouts unless a narrower or stricter local
playbook applies.

### Plan of Work

Execute the tracker from CQR09 through CQR36 in order unless the current live
metrics prove a lower-ranked item has become a better next target and a decision
log entry records the reordering. Do not combine two checklist rows into one
implementation package. For each row, scaffold a new CQR package before
production edits. The package objective should name the file, original rank,
original CRAP/CC/coverage values from this plan, and the current target function
found by re-running metrics.

For every package, read the nearest applicable `AGENTS.md` files before edits.
For kernel-affecting files, read
`docs/specifications/science-contracts/AGENTS.md` before production changes.
For parser/runtime publication files, explicitly preserve typed error behavior,
compatibility modes, guard IDs, aliases, symbols, units, and public APIs.

Each package must use the CQR sequence:

1. scaffold `docs/work-packages/YYYYMMDD-cqrNN-<slug>-001/` with `package.md`,
   `prompts/active/`, `prompts/archived/`, and `artifacts/`;
2. record current line counts, suppression census, before LCOV, before CRAP, and
   exact target function;
3. add characterization coverage before decomposition when coverage is weak or
   the target behavior lacks an exact focused test;
4. decompose the target function into private helpers while preserving statement
   order, float expression grouping, short-circuit behavior, typed errors, and
   public API;
5. run focused tests during iteration;
6. re-run LCOV and `cargo-crap`;
7. run the mandatory closure gates;
8. complete evidence artifacts, dual reviews, dual verification,
   line-count-governance checklist, disposition, and worker handoff;
9. commit the package write set with a terse CQR-specific message and push the
   current branch to `origin`;
10. update this checklist item from `[ ]` to `[x]` only after the package closes
   and the pushed commit SHA is known.

### Concrete Steps

Run commands from `/home/workdir/openWEPP`.

Before starting the next package, confirm the worktree:

    git status --short --branch

Generate current coverage and CRAP evidence for the whole workspace or for the
package-defined scope:

    cargo llvm-cov clean --workspace
    cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path <package>/artifacts/lcov_before.info
    cargo crap --workspace --lcov <package>/artifacts/lcov_before.info --min 0 --format json --output <package>/artifacts/crap_before.json

Extract the current rows for the target file and identify the target function:

    jq -r '.entries[] | select(.file | endswith("<repo-relative-file>")) | [.function, .line, .cyclomatic, (.coverage // "null"), .crap] | @tsv' <package>/artifacts/crap_before.json | sort -k5,5nr

Run focused tests for the touched crate before production edits. The exact
command depends on the file, but use a crate-specific filter when available,
then finish with the full closure loop.

Run final closure gates for each package:

    cargo fmt --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo test --workspace
    cargo deny check
    markdown-doc lint --path docs/work-packages/README.md --path <package> --format json
    git diff --check

After the package disposition is complete or complete-with-warnings and all
required gates have passed, commit and push before marking the tracker row
complete:

    git status --short --branch
    git add <package-write-set>
    git commit -m "Refactor <module> complexity"
    git push origin "$(git branch --show-current)"
    git log -1 --oneline
    git status --short --branch

Do not create or switch branches unless explicitly requested. If the current
branch is `main`, commit and push `main`.

When a package closes, update this ExecPlan's `Progress` checklist with the
completion date, package directory, pushed commit SHA, pushed branch, and final
maximum target CRAP.

### Validation and Acceptance

Each individual CQR package is accepted only when:

1. the current target function and any newly extracted helpers have CRAP
   `<= 30`;
2. target-file coverage is not regressed relative to the package baseline;
3. focused characterization passes before and after the production refactor;
4. `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo test --workspace`, and `cargo deny check` all pass with exit `0`;
5. package markdown lint and `git diff --check` pass;
6. dual review and dual verification artifacts explicitly check the Gate
   Evidence Non-Deferral Rule;
7. no `.rs` touched file at or above 2000 lines lacks a WARN disposition, and no
   non-exempt touched file at or above 3000 lines remains unresolved;
8. the package disposition is complete or complete-with-warnings with explicit
   WARN holds.
9. the package commit has been pushed to `origin`, and the checklist row records
   the pushed commit SHA.

The full ExecPlan is accepted when all CQR09 through CQR36 checklist items are
checked and `docs/work-packages/README.md` records each package as complete or
complete-with-warnings.

### Idempotence and Recovery

This plan is safe to resume. A package may be restarted from its package
directory and artifacts. If current metrics show a ranked row is already below
CRAP `30`, create or update the corresponding package with a current-metric
closure disposition instead of deleting the checklist row. If a gate fails,
first attempt a behavior-preserving correction inside the package scope. If the
failure requires changing science, public API, guard semantics, parser
compatibility, or output formulas, stop and mark the package `HOLD` with a
defect-shaped follow-up.

Do not use `git reset --hard` or revert unrelated user work. Stage only the
package write set when committing. If `git push` fails, leave the checklist row
unchecked, record the blocker in the package handoff and this plan's
`Surprises & Discoveries`, and do not mark the CQR complete until a later push
succeeds.

### Artifacts and Notes

Every CQR package must include at least:

- required-reading map;
- quality plan report;
- public API surface parity report;
- function-length or CRAP before/after report, as appropriate;
- raw `lcov_before.info`, `lcov_after.info`, `crap_before.json`, and
  `crap_after.json` when CRAP is the active metric;
- coverage closure report;
- numeric-equivalence report;
- implementation and test evidence;
- line-count governance checklist;
- owned-file manifest;
- gate-results table with explicit exit codes;
- dual review artifacts;
- dual verification artifacts;
- disposition;
- worker handoff.

### Interfaces and Dependencies

Use existing Rust APIs and local helper patterns in the target module. New
helpers should be private unless a package explicitly authorizes a public API
change, which normal CQR packages should not do. Do not add dependencies for
these refactors.

Required tools are the existing workspace toolchain:

- `cargo fmt`;
- `cargo clippy`;
- `cargo test`;
- `cargo deny`;
- `cargo llvm-cov`;
- `cargo crap`;
- `markdown-doc`;
- `jq`;
- `rg`.

If a required tool is unavailable, record the exact command failure in the
package and stop at a blocker unless the repo provides an equivalent canonical
fallback.

## `AGENTS.md`

[`AGENTS.md`](https://github.com/openai/agents.md) is a simple format for guiding coding agents such as Codex. We describe a term that users can use as a shorthand and a simple rule for when to use planning documents. Here, we call it an "ExecPlan". Note that this is an arbitrary term, Codex has not been trained on it. This shorthand can then be used when prompting Codex to direct it to a particular definition of a plan.

Here's an `AGENTS.md` section instructing an agent about when to use a plan:

```md
# ExecPlans

When writing complex features or significant refactors, use an ExecPlan (as described in .agent/PLANS.md) from design to implementation.
```

## `PLANS.md`

Below is the entire document. The prompting in this document was carefully chosen to provide significant amounts of feedback to users and to guide the model to implement precisely what a plan specifies. Users may find that they benefit from customizing the file to meet their needs, or to add or remove required sections.

~~~md
# Codex Execution Plans (ExecPlans):

This document describes the requirements for an execution plan ("ExecPlan"), a design document that a coding agent can follow to deliver a working feature or system change. Treat the reader as a complete beginner to this repository: they have only the current working tree and the single ExecPlan file you provide. There is no memory of prior plans and no external context.

## How to use ExecPlans and PLANS.md

When authoring an executable specification (ExecPlan), follow PLANS.md _to the letter_. If it is not in your context, refresh your memory by reading the entire PLANS.md file. Be thorough in reading (and re-reading) source material to produce an accurate specification. When creating a spec, start from the skeleton and flesh it out as you do your research.

When implementing an executable specification (ExecPlan), do not prompt the user for "next steps"; simply proceed to the next milestone. Keep all sections up to date, add or split entries in the list at every stopping point to affirmatively state the progress made and next steps. Resolve ambiguities autonomously, and commit frequently.

When discussing an executable specification (ExecPlan), record decisions in a log in the spec for posterity; it should be unambiguously clear why any change to the specification was made. ExecPlans are living documents, and it should always be possible to restart from _only_ the ExecPlan and no other work.

When researching a design with challenging requirements or significant unknowns, use milestones to implement proof of concepts, "toy implementations", etc., that allow validating whether the user's proposal is feasible. Read the source code of libraries by finding or acquiring them, research deeply, and include prototypes to guide a fuller implementation.

## Requirements

NON-NEGOTIABLE REQUIREMENTS:

* Every ExecPlan must be fully self-contained. Self-contained means that in its current form it contains all knowledge and instructions needed for a novice to succeed.
* Every ExecPlan is a living document. Contributors are required to revise it as progress is made, as discoveries occur, and as design decisions are finalized. Each revision must remain fully self-contained.
* Every ExecPlan must enable a complete novice to implement the feature end-to-end without prior knowledge of this repo.
* Every ExecPlan must produce a demonstrably working behavior, not merely code changes to "meet a definition".
* Every ExecPlan must define every term of art in plain language or do not use it.

Purpose and intent come first. Begin by explaining, in a few sentences, why the work matters from a user's perspective: what someone can do after this change that they could not do before, and how to see it working. Then guide the reader through the exact steps to achieve that outcome, including what to edit, what to run, and what they should observe.

The agent executing your plan can list files, read files, search, run the project, and run tests. It does not know any prior context and cannot infer what you meant from earlier milestones. Repeat any assumption you rely on. Do not point to external blogs or docs; if knowledge is required, embed it in the plan itself in your own words. If an ExecPlan builds upon a prior ExecPlan and that file is checked in, incorporate it by reference. If it is not, you must include all relevant context from that plan.

## Formatting

Format and envelope are simple and strict. Each ExecPlan must be one single fenced code block labeled as `md` that begins and ends with triple backticks. Do not nest additional triple-backtick code fences inside; when you need to show commands, transcripts, diffs, or code, present them as indented blocks within that single fence. Use indentation for clarity rather than code fences inside an ExecPlan to avoid prematurely closing the ExecPlan's code fence. Use two newlines after every heading, use # and ## and so on, and correct syntax for ordered and unordered lists.

When writing an ExecPlan to a Markdown (.md) file where the content of the file *is only* the single ExecPlan, you should omit the triple backticks.

Write in plain prose. Prefer sentences over lists. Avoid checklists, tables, and long enumerations unless brevity would obscure meaning. Checklists are permitted only in the `Progress` section, where they are mandatory. Narrative sections must remain prose-first.

## Guidelines

Self-containment and plain language are paramount. If you introduce a phrase that is not ordinary English ("daemon", "middleware", "RPC gateway", "filter graph"), define it immediately and remind the reader how it manifests in this repository (for example, by naming the files or commands where it appears). Do not say "as defined previously" or "according to the architecture doc." Include the needed explanation here, even if you repeat yourself.

Avoid common failure modes. Do not rely on undefined jargon. Do not describe "the letter of a feature" so narrowly that the resulting code compiles but does nothing meaningful. Do not outsource key decisions to the reader. When ambiguity exists, resolve it in the plan itself and explain why you chose that path. Err on the side of over-explaining user-visible effects and under-specifying incidental implementation details.

Anchor the plan with observable outcomes. State what the user can do after implementation, the commands to run, and the outputs they should see. Acceptance should be phrased as behavior a human can verify ("after starting the server, navigating to [http://localhost:8080/health](http://localhost:8080/health) returns HTTP 200 with body OK") rather than internal attributes ("added a HealthCheck struct"). If a change is internal, explain how its impact can still be demonstrated (for example, by running tests that fail before and pass after, and by showing a scenario that uses the new behavior).

Specify repository context explicitly. Name files with full repository-relative paths, name functions and modules precisely, and describe where new files should be created. If touching multiple areas, include a short orientation paragraph that explains how those parts fit together so a novice can navigate confidently. When running commands, show the working directory and exact command line. When outcomes depend on environment, state the assumptions and provide alternatives when reasonable.

Be idempotent and safe. Write the steps so they can be run multiple times without causing damage or drift. If a step can fail halfway, include how to retry or adapt. If a migration or destructive operation is necessary, spell out backups or safe fallbacks. Prefer additive, testable changes that can be validated as you go.

Validation is not optional. Include instructions to run tests, to start the system if applicable, and to observe it doing something useful. Describe comprehensive testing for any new features or capabilities. Include expected outputs and error messages so a novice can tell success from failure. Where possible, show how to prove that the change is effective beyond compilation (for example, through a small end-to-end scenario, a CLI invocation, or an HTTP request/response transcript). State the exact test commands appropriate to the project’s toolchain and how to interpret their results.

Capture evidence. When your steps produce terminal output, short diffs, or logs, include them inside the single fenced block as indented examples. Keep them concise and focused on what proves success. If you need to include a patch, prefer file-scoped diffs or small excerpts that a reader can recreate by following your instructions rather than pasting large blobs.

## Milestones

Milestones are narrative, not bureaucracy. If you break the work into milestones, introduce each with a brief paragraph that describes the scope, what will exist at the end of the milestone that did not exist before, the commands to run, and the acceptance you expect to observe. Keep it readable as a story: goal, work, result, proof. Progress and milestones are distinct: milestones tell the story, progress tracks granular work. Both must exist. Never abbreviate a milestone merely for the sake of brevity, do not leave out details that could be crucial to a future implementation.

Each milestone must be independently verifiable and incrementally implement the overall goal of the execution plan.

## Living plans and design decisions

* ExecPlans are living documents. As you make key design decisions, update the plan to record both the decision and the thinking behind it. Record all decisions in the `Decision Log` section.
* ExecPlans must contain and maintain a `Progress` section, a `Surprises & Discoveries` section, a `Decision Log`, and an `Outcomes & Retrospective` section. These are not optional.
* When you discover optimizer behavior, performance tradeoffs, unexpected bugs, or inverse/unapply semantics that shaped your approach, capture those observations in the `Surprises & Discoveries` section with short evidence snippets (test output is ideal).
* If you change course mid-implementation, document why in the `Decision Log` and reflect the implications in `Progress`. Plans are guides for the next contributor as much as checklists for you.
* At completion of a major task or the full plan, write an `Outcomes & Retrospective` entry summarizing what was achieved, what remains, and lessons learned.

# Prototyping milestones and parallel implementations

It is acceptable—-and often encouraged—-to include explicit prototyping milestones when they de-risk a larger change. Examples: adding a low-level operator to a dependency to validate feasibility, or exploring two composition orders while measuring optimizer effects. Keep prototypes additive and testable. Clearly label the scope as “prototyping”; describe how to run and observe results; and state the criteria for promoting or discarding the prototype.

Prefer additive code changes followed by subtractions that keep tests passing. Parallel implementations (e.g., keeping an adapter alongside an older path during migration) are fine when they reduce risk or enable tests to continue passing during a large migration. Describe how to validate both paths and how to retire one safely with tests. When working with multiple new libraries or feature areas, consider creating spikes that evaluate the feasibility of these features _independently_ of one another, proving that the external library performs as expected and implements the features we need in isolation.

## Skeleton of a Good ExecPlan

    # <Short, action-oriented description>

    This ExecPlan is a living document. The sections `Progress`, `Surprises & Discoveries`, `Decision Log`, and `Outcomes & Retrospective` must be kept up to date as work proceeds.

    If PLANS.md file is checked into the repo, reference the path to that file here from the repository root and note that this document must be maintained in accordance with PLANS.md.

    ## Purpose / Big Picture

    Explain in a few sentences what someone gains after this change and how they can see it working. State the user-visible behavior you will enable.

    ## Progress

    Use a list with checkboxes to summarize granular steps. Every stopping point must be documented here, even if it requires splitting a partially completed task into two (“done” vs. “remaining”). This section must always reflect the actual current state of the work.

    - [x] (2025-10-01 13:00Z) Example completed step.
    - [ ] Example incomplete step.
    - [ ] Example partially completed step (completed: X; remaining: Y).

    Use timestamps to measure rates of progress.

    ## Surprises & Discoveries

    Document unexpected behaviors, bugs, optimizations, or insights discovered during implementation. Provide concise evidence.

    - Observation: …
      Evidence: …

    ## Decision Log

    Record every decision made while working on the plan in the format:

    - Decision: …
      Rationale: …
      Date/Author: …

    ## Outcomes & Retrospective

    Summarize outcomes, gaps, and lessons learned at major milestones or at completion. Compare the result against the original purpose.

    ## Context and Orientation

    Describe the current state relevant to this task as if the reader knows nothing. Name the key files and modules by full path. Define any non-obvious term you will use. Do not refer to prior plans.

    ## Plan of Work

    Describe, in prose, the sequence of edits and additions. For each edit, name the file and location (function, module) and what to insert or change. Keep it concrete and minimal.

    ## Concrete Steps

    State the exact commands to run and where to run them (working directory). When a command generates output, show a short expected transcript so the reader can compare. This section must be updated as work proceeds.

    ## Validation and Acceptance

    Describe how to start or exercise the system and what to observe. Phrase acceptance as behavior, with specific inputs and outputs. If tests are involved, say "run <project’s test command> and expect <N> passed; the new test <name> fails before the change and passes after>".

    ## Idempotence and Recovery

    If steps can be repeated safely, say so. If a step is risky, provide a safe retry or rollback path. Keep the environment clean after completion.

    ## Artifacts and Notes

    Include the most important transcripts, diffs, or snippets as indented examples. Keep them concise and focused on what proves success.

    ## Interfaces and Dependencies

    Be prescriptive. Name the libraries, modules, and services to use and why. Specify the types, traits/interfaces, and function signatures that must exist at the end of the milestone. Prefer stable names and paths such as `crate::module::function` or `package.submodule.Interface`. E.g.:

    In crates/foo/planner.rs, define:

        pub trait Planner {
            fn plan(&self, observed: &Observed) -> Vec<Action>;
        }

If you follow the guidance above, a single, stateless agent -- or a human novice -- can read your ExecPlan from top to bottom and produce a working, observable result. That is the bar: SELF-CONTAINED, SELF-SUFFICIENT, NOVICE-GUIDING, OUTCOME-FOCUSED.

When you revise a plan, you must ensure your changes are comprehensively reflected across all sections, including the living document sections, and you must write a note at the bottom of the plan describing the change and the reason why. ExecPlans must describe not just the what but the why for almost everything.
~~~
