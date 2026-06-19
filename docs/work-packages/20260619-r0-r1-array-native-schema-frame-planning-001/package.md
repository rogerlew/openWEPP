# R0/R1 - Array-Native Schema and Frame Planning

Status: executed 2026-06-19. Disposition: `COMPLETE-PLANNING-ONLY`.

Package type: architecture planning / array-native runtime R0-R1 hold-limited
work package.

## Objective

Scaffold and execute the planning-only R0/R1 package allowed by
`docs/architecture/array-native-runtime-specification.md` while PERFDEEP07
remains in `HOLD`.

This package freezes the planning boundaries for:

- R0 runtime schema and direct-frame type-boundary decisions;
- R1 frame constructor and publication projection planning;
- PERFDEEP07 hold-lift constraints;
- proof methods required before direct-mode implementation can claim readiness.

It must not edit runtime Rust code, implement direct-frame execution, claim
runtime readiness, or activate any direct-frame path.

## Rationale

PERFDEEP07 remains blocked on the default-disabled P0 timing gate. The revised
array-native runtime specification allows only planning, schema, ledger,
fixture, shadow-scaffold, and non-activated adapter work until that blocker is
closed or explicitly superseded.

PERFDEEP06 already produced the working-set inventory, publication operand
ledger, direct-frame API plan, layout/allocation ledger, and no-hot-loop-map
proof. This package converts that evidence plus the post-PERFDEEP07
architecture revision into a right-sized R0/R1 execution plan that can guide
the next package without accidentally re-entering compatibility-edge
implementation.

## Scope

In scope:

- document the R0 direct-frame type-boundary decision;
- define the planning schema envelope for direct run/lane/day/publication
  frames;
- define R1 constructor and projection acceptance surfaces without code edits;
- bind the PERFDEEP06 publication operand ledger as the seed for future R6
  ledger promotion;
- define executable no-compatibility proof requirements for future direct-mode
  work;
- record the active PERFDEEP07 hold-lift blocker and the conditions to advance
  beyond planning-only R0/R1;
- update package artifacts and the work-package catalog.

Out of scope:

- any production Rust implementation;
- direct executor skeleton code;
- frame type introduction in `crates/`;
- output schema changes;
- canonical `SC-*` contract amendments;
- H2637 endpoint timing;
- default activation or opt-in activation of a direct-frame runtime;
- claiming that R0 runtime schema freeze is complete for implementation beyond
  this planning envelope.

## Required Reading

Core:

- `AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/package.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/decisions/0025-array-native-hillslope-day-frame.md`
- `docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-working-set-inventory.md`
- `docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-publication-operand-ledger.md`
- `docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-direct-frame-api-plan.md`
- `docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/disposition.md`
- `docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/gate-results.md`

Conditional:

- `docs/specifications/science-contracts/AGENTS.md` if execution discovers a
  canonical contract or guard-authority change is required. This package did
  not make one.
- `crates/AGENTS.md` before any Rust edit. This package did not edit Rust.
- `tests/AGENTS.md` before any test edit. This package did not edit tests.

On demand:

- `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/**`

## Dependencies

- `docs/architecture/array-native-runtime-specification.md` Revision 2 is the
  binding architecture authority.
- PERFDEEP06 artifacts are planning seed evidence.
- PERFDEEP07 disposition and gate results are the active hold-lift blocker.
- No canonical science contract changes are required for this planning-only
  package.

## Intended Write Set

- `docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

No Rust, tests, science contracts, or output schema files are writable in this
package.

## Phase Plan

1. Scaffold package directories, kickoff prompt, and required artifacts.
2. Populate required-reading and owned-file artifacts.
3. Execute R0 planning: direct-frame type-boundary decision and schema
   envelope.
4. Execute R1 planning: constructors/projections, publication ledger seed, and
   validation surfaces.
5. Record PERFDEEP07 hold-lift conditions and future no-compatibility proof
   method.
6. Record contract disposition, implementation/test evidence, line-count
   governance, gate results, review/verification, final disposition, and worker
   handoff.
7. Update package catalog and roadmap state.
8. Run scoped markdown lint and whitespace checks.

## Acceptance Criteria

- `artifacts/r0-runtime-schema-planning.md` records the schema envelope and
  states that implementation is blocked beyond planning-only R0/R1.
- `artifacts/direct-frame-type-boundary-decision.md` resolves the existing
  `HillslopeDayFrame` naming ambiguity.
- `artifacts/r1-frame-constructor-projection-plan.md` records constructor and
  projection planning without code edits.
- `artifacts/publication-ledger-promotion-plan.md` binds the PERFDEEP06 ledger
  as the seed for future promotion.
- `artifacts/no-compatibility-proof-plan.md` defines executable proof methods
  for future direct-mode packages.
- `artifacts/perfdeep07-hold-lift-disposition.md` records the active blocker.
- No production Rust, tests, or canonical contract files are edited.
- Scoped markdown lint passes for the package and touched catalog/roadmap docs.
- `git diff --check` passes.
- Review and verification artifacts disposition all findings and explicitly
  check gate legitimacy.

## Conservation / Output Acceptance

This package is planning-only and does not create, correct, or aggregate output
surfaces. It does touch future publication architecture, so the package must
record that future publication cutover requires operand lineage, anti-alias
fixtures, metadata parity, and independent operand reconstruction before any
runtime or output implementation can claim acceptance.

## Contract-First Rule

No contract amendment is intended. If execution discovers a required change to
physics, guard semantics, diagnostic attribution, output meaning, units, or
conservation authority, stop and re-scope before production edits.

## Security Impact Gate

No secrets, external network dependencies, user data, production endpoints, or
runtime activation changes are in scope.

## Subagent Authorization

Subagent authorization: none. This package does not authorize spawning or
delegating to subagents; review and verification artifacts are local static
review passes for the planning-only scope.

## Autonomy

Execute this planning-only package end-to-end without asking for next steps
unless a hard blocker prevents a truthful `COMPLETE-PLANNING-ONLY` or `HOLD`
disposition. Do not proceed into R2 or runtime implementation.
