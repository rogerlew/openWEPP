# WSHED-W2 Serial Watershed Supervisor Skeleton

Status: `EXECUTED-COMPLETE-WSHED-W2`

Date opened: `2026-07-01`

Date closed: `2026-07-01`

Package type: implementation package; watershed runtime architecture rung W2.

## Outcome

W2 completed. The public `openwepp-cli-watershed` path now supports the serial
`--jobs 1` supervisor skeleton with `WatershedRunPlan`, `HillslopeJob`, and
`PassInventory`. Generated hillslope jobs run serially with isolated
per-hillslope output/log/timing paths, validated pass inventory gates routing,
missing latest-event payloads fail closed, and routed-stage reuse remains
available only through explicit `use_existing_pass_file = true` runfile blocks.

Focused W2 coverage proves invalid job values, explicit reuse gating,
ambiguous-block rejection, relative output-dir generated mode, stale generated
artifact cleanup/fail-closed behavior, generated public CLI handoff, and
latest-event payload fail-closed behavior. Final closure gates passed; see
`artifacts/closure/summary.md`.

## Objective

Implement the serial watershed supervisor skeleton for the new watershed
runtime. W2 introduces `WatershedRunPlan`, `HillslopeJob`, and `PassInventory`
in the public watershed runner path for `--jobs 1`, without parallel fanout. The
new serial path must stop depending on package-local shell loops, shared output
directories, and ad hoc path rewrites, while preserving routed-stage reuse mode
for explicit profiling/replay/comparator work.

## Rationale

ADR-0032 ratified `openwepp-cli-watershed` as the public watershed runtime
entrypoint and `--jobs 1` as the deterministic default. WSHED-FIXTURE01 adopted
the committed 32-hillslope carnivorous-adobo input/runfile fixture under
`tests/fixtures/watershed/carnivorous-adobo/`. W2 is the first implementation
rung that turns the architecture into a real runner path by supervising
single-threaded hillslope subprocesses serially, validating pass artifacts, and
then invoking existing watershed routing.

## Included Scope

- Add the new serial runtime data model and orchestration path:
  `WatershedRunPlan`, `HillslopeJob`, and `PassInventory`.
- Add public CLI handling for `--jobs`, with omitted `--jobs` equivalent to
  `--jobs 1`; reject `--jobs 0` and invalid values.
- Execute hillslope jobs serially in deterministic order with isolated per-job
  output/log paths.
- Validate the pass inventory before watershed routing.
- Resolve latest-event payload handling contract-first:
  - default to fail-closed hard error for missing latest-event payloads unless
    existing contract authority explicitly admits a valid `NoEvent` state;
  - do not synthesize zero runoff/sediment fields from absent payloads.
- Preserve explicit routed-stage reuse mode for existing-pass profiling/replay.
- Add tests and evidence proving the real public runner reads the new
  plan/inventory path for the W2 claim.
- Record consumer-path, line-count, gate, review, verification, and disposition
  artifacts.

## Excluded Scope

- No parallel execution; W3 owns `--jobs N` worker-pool concurrency.
- No typed `WatershedNetworkFrame` production routing cutover; W4 owns that.
- No deletion of the old watershed runtime; W5 owns full deletion.
- No large 1,000+ hillslope fixture adoption; W6 owns large-scaling fixtures.
- No watershed routing physics, hillslope physics, erosion, sediment equation,
  or output schema changes for performance.
- No silent use of `/wc1`, wepppy, or scratch paths as persistent gates.

## Intended Write Set

- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/src/**` for watershed supervisor modules only.
- `crates/openwepp-runner/tests/**`
- `tests/integration/**` for focused W2 contract tests.
- `tests/fixtures/watershed/carnivorous-adobo/**` only for metadata or manifest
  updates required by W2 gates.
- `docs/work-packages/20260701-wshedw2-serial-watershed-supervisor-skeleton-001/**`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`

Any production edit outside this write set requires package amendment before
implementation.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to `comparator_suite_runner`, `rust_code_reviewer`, and
`rust_qa_reviewer` subagents. `comparator_suite_runner` is authorized for heavy
final gate execution and comparator/fixture runs; expected output is compact
metrics plus log/artifact paths. `rust_code_reviewer` and `rust_qa_reviewer`
are authorized for read-only review and verification; expected outputs are
compact findings with file/path references. Write access is read-only for
subagents, with parent disposition recorded in `artifacts/review-disposition.md`
and `artifacts/verification.md`.

## Phase Plan

1. Preparation:
   - read required authorities and this package;
   - inspect current `openwepp-cli-watershed`, `openwepp-cli-hill`,
     `open_wepp_runner`, `launch.rs`, watershed CLI tests, and the committed
     carnivorous-adobo fixture;
   - define the exact W2 command surface and serial control/baseline surfaces.
2. Contract and design:
   - document `WatershedRunPlan`, `HillslopeJob`, and `PassInventory` fields;
   - decide whether absent latest-event payload is a hard error or an admitted
     `NoEvent` state under existing science/format authority;
   - if `NoEvent` authority is missing, implement hard-error fail-closed
     handling and record the deferred contract question.
3. Implementation:
   - add serial plan construction and deterministic hillslope job execution;
   - use isolated output/log paths;
   - build and validate pass inventory before routing;
   - route only after inventory validation succeeds;
   - preserve explicit routed-stage reuse mode.
4. Tests and evidence:
   - add focused tests for `--jobs` parsing/default/rejection;
   - prove committed fixture paths are used where W2 gates use fixtures;
   - prove missing/stale/ambiguous pass payloads fail closed;
   - prove routed-stage reuse remains explicit;
   - write consumer-path proof showing the public runner uses the new
     plan/inventory path and not a shell-loop/shared-output path.
5. Review and closure:
   - run focused local iteration gates;
   - run or delegate required final closure gates;
   - complete dual review, verification, line-count governance, and final
     disposition.

## Exit Criteria

- `openwepp-cli-watershed` accepts `--jobs`; omitted `--jobs` equals `--jobs 1`,
  and invalid/zero values fail closed.
- `--jobs 1` uses `WatershedRunPlan`, `HillslopeJob`, and `PassInventory` in the
  real public runner path.
- Hillslope jobs execute serially in deterministic order and write isolated
  outputs/logs.
- Pass inventory validation runs before watershed routing and fails closed on
  missing, stale, wrong-id, wrong-class-count, schema-invalid, or semantically
  ambiguous artifacts.
- Missing latest-event payloads do not synthesize zeros; they either fail closed
  or are typed as valid `NoEvent` with explicit authority.
- Existing routed-stage reuse mode remains available and explicit.
- Consumer-path proof names producer source, in-memory state/frame object,
  runner handoff, downstream routing/publication consumer, output/API surface,
  and negative proof that the old shell-loop/shared-output path is not used for
  the W2 claim.
- Focused tests and final closure gates pass, or the package closes
  `EXECUTED-HOLD` with a named blocker.

## Required Final Gates

Because W2 edits production Rust, final closure must run and record:

1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo nextest run --workspace --profile full`
4. `cargo deny check`
5. focused W2 identity/consumer-path/pass-inventory/fixture gates.

If a gate cannot run, the package must close `EXECUTED-HOLD` with the exact
blocker unless a canonical decision explicitly authorizes a narrower closure.

## Security and Safety

The W2 runner supervises subprocesses and writes outputs/logs. Use
`std::process::Command` with explicit argument arrays; no shell interpolation.
Do not introduce network dependencies, credential handling, silent dependency
fallbacks, or broad path discovery. Preserve fail-closed behavior for invalid
plans and artifacts.
