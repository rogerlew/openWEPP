# CQR Nightly Batch 02, Target 08 — Topology

Package: `20260709-cqr-nightly-b02-08-topology-001`
Status: `EXECUTED-COMPLETE-CQR-NIGHTLY`
ExecPlan: `docs/work-packages/cqr-nightly-burndown-execplan.md`
Nightly batch: `02`
Target module: `crates/openwepp-topology/src/lib.rs`
Target rank: `8` of `10`
Quality dimension: `CRAP/cyclomatic-complexity`

## Objective

Reduce every eligible production function in
`crates/openwepp-topology/src/lib.rs` with CRAP above `30` to `<= 30`, or
record an ADR-0021-style disposition when a row is not safely reducible as
behavior-preserving CQR. Preserve runtime behavior and output identity for all
existing valid inputs.

## Scope

In scope:

- characterization tests required to make the target safe to refactor;
- behavior-preserving helper extraction or control-flow simplification inside
  `crates/openwepp-topology/src/lib.rs`;
- package artifacts and prompt material;
- focused tests that prove existing topology parser and validation behavior.

Out of scope:

- public topology API changes;
- fixture grammar, message ID, typed error, or validation-policy changes;
- routing behavior or public output semantic changes;
- science-formula, threshold, tolerance, or contract-authority changes;
- opportunistic cleanup outside the target module.

## Intended Write Set

- `crates/openwepp-topology/src/lib.rs`
- `tests/integration/topology_graph_validation_gate.rs`
- `docs/work-packages/20260709-cqr-nightly-b02-08-topology-001/**`
- `docs/work-packages/README.md` after closure if catalog update is needed

Do not edit unrelated dirty files. If a declared write-set path is already dirty
from unrelated work, stop before implementation and record a global/process hold.

## Required Reading

Core:

- `AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/20260709-cqr-nightly-b02-08-topology-001/package.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/20260709-cqr-nightly-b02-08-topology-001/artifacts/required-reading-map.md`

Conditional:

- `docs/specifications/science-contracts/AGENTS.md` and nearest relevant
  `SC-*` contract only if the change touches contract authority,
  conservation-sensitive outputs, or contract-derived tests.
- `docs/standards/local-ci-gate-selection.md` if focused iteration gates need to
  be narrowed before the final closure loop.

On-demand:

- `crates/openwepp-topology/src/lib.rs`
- `tests/integration/topology_graph_validation_gate.rs`
- `tests/integration/wshedw5_typed_watershed_runtime_contract.rs`
- `tests/fixtures/topology/*`

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to review, verification, and comparator/closure-runner
subagents for behavior-preserving CQR review, target metric verification,
focused/full gate execution, and output-identity checks. Expected outputs are
package-local `artifacts/review_agent_a.md`, `artifacts/review_agent_b.md`,
`artifacts/verification_agent_a.md`, `artifacts/verification_agent_b.md`,
compact metrics, command logs, and artifact paths. Write access is read-only
unless a subagent is explicitly assigned a bounded implementation fix in
`crates/openwepp-topology/src/lib.rs`,
`tests/integration/topology_graph_validation_gate.rs`, or package-local
artifacts.

Subagent requirement: this package requires spawning `comparator_suite_runner`
for heavy batch/closure/comparator runs, including full-workspace CRAP/coverage
after implementation, `cargo nextest run --workspace --profile full`,
comparator suites, and population/fixture batches. Do not run those heavy gates
locally on the parent model unless the subagent is unavailable; if unavailable,
record command-level evidence before running locally.

## Required Gates

Commit scaffold before edits; use test-first characterization, delegated heavy
metrics/gates, dual review/verification, finding disposition, line-count
governance, and completion or hold commit before target 09.
