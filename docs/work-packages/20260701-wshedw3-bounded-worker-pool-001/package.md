# WSHED-W3 Bounded Worker Pool

Status: `EXECUTED-COMPLETE-WSHED-W3`

Date opened: `2026-07-01`

Package type: implementation package; watershed runtime architecture rung W3.

## Objective

Implement bounded worker-pool execution for watershed hillslope jobs behind the
public `openwepp-cli-watershed --jobs N` interface. W3 extends the W2
`WatershedRunPlan`, `HillslopeJob`, and `PassInventory` path so multiple
single-threaded hillslope subprocesses can run concurrently while routing and
publication remain deterministic and gated by a sorted pass inventory.

## Rationale

W2 completed the serial supervisor skeleton: the public watershed CLI now builds
a run plan, executes generated hillslope jobs for `--jobs 1`, isolates
per-hillslope artifacts, validates pass inventory before routing, and fails
closed on missing latest-event payloads. W3 is the CPU-scaling rung. It should
remove the temporary `--jobs > 1` rejection, add bounded fanout, prove
`--jobs 1` and `--jobs N` output identity, and record an auditable scaling curve
on committed fixtures.

The canonical W3 performance fixture is the committed 32-hillslope
`tests/fixtures/watershed/carnivorous-adobo/` fixture adopted by
WSHED-FIXTURE01. Arboreal-dendrite or `/wc1` measurements may be recorded only
as contextual engineering-budget evidence and must not be the sole ratification
surface.

## Included Scope

- Add a bounded `HillslopeWorkerPool` or equivalent supervisor component for
  generated hillslope jobs.
- Accept explicit positive `--jobs N` values greater than `1` in
  `openwepp-cli-watershed`.
- Preserve omitted `--jobs` and `--jobs 1` as the deterministic serial
  baseline.
- Run up to `N` hillslope child processes concurrently with `std::process`
  command construction using explicit argument vectors.
- Preserve W2 per-job isolated output, log, timing, and freshness artifacts.
- Preserve deterministic pass inventory order independent of child completion
  order.
- Preserve fail-closed behavior: child failures, stale/missing pass artifacts,
  schema-invalid payloads, wrong hillslope ids, wrong class counts, and missing
  latest-event payloads must prevent routing.
- Define and test worker-pool failure policy for in-flight jobs and pending
  jobs.
- Record per-job timing distribution, route-stage timing, wall/user/system/RSS
  summaries, job-count labels, CPU inventory, and sidecar/input-discovery mode.
- Prove output identity between `--jobs 1` and selected `--jobs N` runs on the
  committed fixture.
- Record consumer-path proof showing the real public runner uses the worker
  pool for the W3 claim.

## Excluded Scope

- No typed `WatershedNetworkFrame` production routing cutover; W4 owns that.
- No deletion of the old watershed runtime; W5 owns full deletion.
- No large 1,000+ hillslope fixture adoption; W6 owns large-scaling fixtures.
- No watershed routing physics, hillslope physics, erosion, sediment equation,
  or output schema changes for performance.
- No automatic host-wide CPU default; ADR-0032 requires explicit `--jobs N`.
- No use of `/wc1`, wepppy, or scratch paths as the sole persistent gate.
- No comparison that treats discovery-on and discovery-off timing as the same
  benchmark surface.

## Intended Write Set

- `crates/openwepp-runner/src/watershed_supervisor.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/src/lib.rs` if public exports change.
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `tests/integration/**` for focused W3 fixture/identity tests if needed.
- `tests/fixtures/watershed/carnivorous-adobo/**` only for committed metadata,
  generated runfile bindings, or manifest updates required by W3 gates.
- `docs/work-packages/20260701-wshedw3-bounded-worker-pool-001/**`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`

Any production edit outside this write set requires package amendment before
implementation.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to `comparator_suite_runner`, `rust_code_reviewer`, and
`rust_qa_reviewer` subagents. `comparator_suite_runner` is authorized and
required for heavy final closure gates, comparator runs, and scaling runs when
available; expected output is compact metrics plus log/artifact paths.
`rust_code_reviewer` and `rust_qa_reviewer` are authorized for read-only review
and verification; expected outputs are compact findings with file/path
references. Write access is read-only for subagents, with parent disposition
recorded in `artifacts/review-disposition.md` and
`artifacts/verification.md`.

## Phase Plan

1. Preparation:
   - read required authorities and this package;
   - inspect W2 supervisor implementation, CLI parsing, W2 focused tests, and
     carnivorous-adobo fixture metadata;
   - define the worker-pool API, failure policy, and timing record shape.
2. Implementation:
   - add bounded worker-pool execution for generated hillslope jobs;
   - remove temporary `--jobs > 1` rejection while preserving invalid/zero
     rejection;
   - keep `--jobs 1` using the deterministic baseline path or the same worker
     API with concurrency `1`;
   - stop launching new jobs after first hard failure, handle in-flight jobs
     according to the documented policy, and do not route on any failure;
   - collect job status/timing without using completion order for routing.
3. Tests and evidence:
   - add focused tests for `--jobs N` acceptance and `--jobs 0`/invalid
     rejection;
   - prove `--jobs 1` and selected `--jobs N` outputs are identical;
   - prove child failure and missing/stale pass failure paths fail closed before
     routing;
   - prove per-job artifacts stay isolated under concurrent execution;
   - prove the committed fixture path is used for canonical scaling evidence.
4. Performance characterization:
   - run at least three clean repeats for canonical job counts on the committed
     carnivorous-adobo fixture, hardware permitting: `1`, `2`, `4`, `8`, `16`,
     and `32`;
   - record wall, user, system, max RSS, job count, CPU inventory,
     sidecar/input-discovery mode, per-job duration distribution, route-stage
     duration, and output identity;
   - label any arboreal-dendrite, `/wc1`, or legacy comparison as contextual
     cross-scope evidence unless a legacy-equivalent committed fixture surface
     is introduced.
5. Review and closure:
   - run focused local iteration gates;
   - run or delegate required final closure gates;
   - complete dual review, verification, line-count governance, scaling
     evidence, consumer-path proof, and final disposition.

## Exit Criteria

- `openwepp-cli-watershed --jobs N` accepts positive `N > 1`; omitted `--jobs`
  and explicit `--jobs 1` still behave as the deterministic baseline.
- `--jobs 0`, negative values, and invalid values fail closed.
- The real public runner uses bounded worker-pool execution for generated
  hillslope jobs when `--jobs N > 1`.
- Child completion order cannot change pass inventory order, routing input
  order, output row order, warnings, or checksums.
- `--jobs 1` and selected `--jobs N` routed outputs are identical on the
  committed fixture, or any difference has explicit contract-governed
  disposition.
- Child failures and pass inventory failures prevent routing and publication.
- Per-job output/log/timing/freshness artifacts remain isolated under
  concurrent execution.
- Canonical scaling evidence is recorded on committed fixture inputs with
  sidecar/input-discovery mode and job count labels.
- Consumer-path proof names producer source, worker-pool state, runner handoff,
  pass inventory, downstream routing/publication consumer, output/API surface,
  and negative proof that old shell-loop orchestration is not used for the W3
  claim.
- Focused tests and final closure gates pass, or the package closes
  `EXECUTED-HOLD` with a named blocker.

## Required Final Gates

Because W3 edits production Rust, final closure must run and record:

1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo nextest run --workspace --profile full`
4. `cargo deny check`
5. focused W3 identity/worker-pool/failure/fixture/scaling gates.

If a gate cannot run, the package must close `EXECUTED-HOLD` with the exact
blocker unless a canonical decision explicitly authorizes a narrower closure.

## Security and Safety

The W3 worker pool supervises subprocesses and concurrent filesystem writes.
Use `std::process::Command` with explicit argument arrays; no shell
interpolation. Do not introduce network dependencies, credential handling,
silent dependency fallbacks, or broad path discovery. Bound concurrency to the
explicit `--jobs` value and preserve fail-closed behavior for invalid plans,
child failures, and artifacts.
