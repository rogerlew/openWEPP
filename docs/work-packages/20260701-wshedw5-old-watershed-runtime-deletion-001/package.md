# WSHED-W5 Old Watershed Runtime Deletion

Status: `QUEUED-HANDOFF-AUTHORED`

Date opened: `2026-07-01`

Package type: implementation and mechanical-deletion package; watershed runtime
architecture rung W5.

## Objective

Delete the old watershed compatibility runtime after W4DC01 moved the public
watershed CLI onto frame-native typed dispatch and typed publication. The
target is full retirement, not another wrapper: remove the watershed-specific
`WatershedWritebackSurface` routing path, obsolete watershed kernel-request /
writeback protocol, compatibility projection helpers, and tests whose only
purpose is to preserve those old surfaces. Backfill protected coverage on the
typed public route before closing.

## Rationale

The watershed performance strategy is a ground-up runtime rewrite followed by
full deletion of the replaced runtime. The hillslope performance work showed
that carrying compatibility runtimes too long keeps cost and ambiguity alive.
WSHED-W4 produced the typed frame/publication handoff and WSHED-W4DC01 lifted
the production routing hold by adding `execute_watershed_dispatch_with_frame`
and typed routed-state publication. WSHED-W5 is the deletion rung that makes the
typed watershed runtime the only production watershed route.

## Dependencies

- `docs/architecture/watershed-runtime-architecture-specification.md`
- `docs/decisions/0032-watershed-runtime-ratification.md`
- `docs/work-packages/20260701-wshedw4-typed-watershed-network-frame-001/`
- `docs/work-packages/20260701-wshedw4dc01-typed-routing-kernel-writeback-closure-001/`
- `docs/work-packages/20260701-wshedw3-bounded-worker-pool-001/`
- `tests/fixtures/watershed/carnivorous-adobo/README.md`

## Included Scope

- Delete watershed-specific old runtime types and helpers that are no longer on
  the public typed route, including the `WatershedWritebackSurface` execution
  path, watershed kernel request/response/writeback protocol, and compatibility
  projection/publication harvest helpers.
- Delete or migrate obsolete tests that assert old watershed map keys,
  writeback-surface values, or compatibility-only dispatch behavior.
- Backfill coverage on the typed public route, direct kernel helpers, typed
  frame publication, and protected watershed CLI behavior.
- Add source guards proving the public watershed CLI and orchestrator
  production dispatch no longer expose or call the old watershed runtime.
- Record a deletion manifest that distinguishes deleted watershed runtime code
  from out-of-scope generic or hillslope compatibility surfaces.
- Preserve W2/W3 supervisor, pass inventory, worker-pool, stale-artifact, and
  fail-closed behavior.
- Update W5 artifacts, `docs/ROADMAP.md`, and `docs/work-packages/README.md`.

## Excluded Scope

- Do not delete hillslope compatibility/runtime surfaces, generic
  `BoundarySymbol` / `BoundaryValue` infrastructure still used outside the
  watershed old runtime, or direct-runtime compatibility seams governed by
  non-watershed packages.
- Do not change routing, impoundment, sediment, erosion, runoff-partition,
  water-balance, latest-event, or output-schema semantics for deletion
  convenience.
- Do not adopt larger 1,000+ hillslope fixtures; WSHED-W6 owns large-fixture
  scaling.
- Do not normalize fixture data.
- Do not introduce a public old-runtime selector, compatibility adapter, or
  fallback route to preserve deleted behavior.

## Intended Write Set

- `crates/openwepp-watershed-orchestrator/src/**`
- `crates/openwepp-kernel-contract/src/**` only for deleting watershed-specific
  old-runtime request/symbol surfaces proven unused outside W5 scope.
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `tests/integration/**` for deleting obsolete old-surface tests and adding
  typed replacement/source-guard tests.
- `docs/specifications/science-contracts/**` only if execution discovers a
  contract-authority change is required.
- `docs/work-packages/20260701-wshedw5-old-watershed-runtime-deletion-001/**`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`

Any production edit outside this write set requires package amendment before
implementation.

## Deletion Targets

Execution must inventory and disposition these watershed-specific old surfaces:

- `WatershedWritebackSurface`
- `WatershedKernelExecutionReport`
- `WatershedKernelStepReport`
- old `WatershedKernelRequest` / `WatershedKernel` protocol if no longer needed
  outside obsolete watershed compatibility tests
- `execute_watershed_dispatch_with_kernel`
- `execute_watershed_dispatch_with_gate_and_kernel`
- `compatibility_writeback_surface`
- compatibility publication harvest helpers
- watershed runtime-surface builders/seeders that exist only to populate the
  old writeback surface
- tests that assert old watershed symbol-map spelling instead of public typed
  behavior

The deletion manifest must also name any symbol that remains, explain why it is
not part of the watershed old runtime, and cite the consumer that still owns it.

## Science and Conservation Authority

W5 is intended as behavior-preserving deletion after typed production cutover.
It must not change process physics. If deletion exposes a missing typed operand,
changed guard, changed unit lineage, changed publication meaning, or changed
kernel branch, execution must stop production edits for that surface, consult
the relevant canonical `SC-*` contract, and amend contract authority before any
semantic production change.

Protected output identity or contract-governed deltas are required for
watershed public outputs touched by deletion. One-sided bounds and exact
self-consistency checks are supporting evidence only.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to `comparator_suite_runner`, `rust_code_reviewer`,
`rust_qa_reviewer`, and `science_contract_reviewer` subagents.
`comparator_suite_runner` is required for heavy full-closure gates,
protected-output comparator runs, and release-style fixture runs when
available; expected output is compact metrics plus log/artifact paths.
`rust_code_reviewer`, `rust_qa_reviewer`, and `science_contract_reviewer` are
authorized for read-only review and verification with compact findings and
file/path references. Write access is read-only for subagents; parent
disposition is recorded in package artifacts.

## Phase Plan

1. Preparation and inventory:
   - read required authority and package documents;
   - inventory all watershed old-runtime symbols and classify each as delete,
     migrate, retained-out-of-scope, or hold-blocked;
   - record the pre-edit deletion inventory and source-guard baseline.
2. Contract and protection gate:
   - confirm the deletion is behavior-preserving for production typed routing;
   - amend contracts and add contract-derived tests first if execution discovers
     any semantic change;
   - identify replacement tests for every obsolete old-surface test removed.
3. Delete old runtime:
   - remove obsolete watershed writeback surface types, dispatch entrypoints,
     compatibility projection helpers, and runtime-surface seeders;
   - remove or migrate old-surface tests;
   - keep public CLI and orchestrator production dispatch on the typed frame.
4. Backfill and source guards:
   - add typed route/source-guard coverage proving the deleted surfaces cannot
     return to production watershed routing;
   - preserve W2/W3 supervisor and worker-pool behavior tests;
   - prove protected outputs remain identity-equivalent or record
     contract-governed deltas.
5. Review and closure:
   - run focused iteration gates and final Rust closure gates;
   - complete dual review, dual verification, deletion manifest, line-count
     governance, gate results, worker handoff, and final disposition;
   - update roadmap and work-package index state for W6.

## Exit Criteria

- No public watershed old-runtime selector exists.
- Public `openwepp-cli-watershed` routes only through typed
  `WatershedNetworkFrame` dispatch and typed publication.
- Production watershed code no longer exposes or uses
  `WatershedWritebackSurface`, `compatibility_writeback_surface`, or
  `execute_watershed_dispatch_with_kernel`.
- Obsolete watershed old-surface tests are deleted or migrated, with protected
  coverage restored on typed public behavior.
- Any remaining generic/hillslope compatibility symbols are explicitly
  out-of-scope and recorded in the deletion manifest.
- Protected watershed outputs are identity-equivalent on committed fixtures, or
  deltas are contract-governed and accepted in review.
- Full gates pass, or the package closes `EXECUTED-HOLD` with a named blocker
  and evidence that the blocker is outside the W5 envelope.

## Required Final Gates

Complete closure requires:

1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo nextest run --workspace --profile full`
4. `cargo deny check`
5. focused W5 source-guard, deletion-manifest, protected-output, and
   replacement-coverage gates
6. scoped docs lint for touched package/index docs
7. `git diff --check`

## Security and Safety

Do not introduce network dependencies, credential handling, broad path
discovery, silent dependency fallbacks, or shell interpolation. Preserve typed
fail-closed behavior for invalid plans, child failures, missing pass inventory,
missing latest-event payloads, stale generated artifacts, and watershed domain
violations. Do not canonicalize-and-proceed on process-domain violations unless
canonical `SC-*` authority explicitly permits bounded normalization.
