# Watershed Runtime Architecture Specification

Status: **Draft, Revision 4** - proposed design authority for watershed
performance work. Revision 1 incorporated the hillslope performance lesson that
partial compatibility-runtime refactors were not aggressive enough: watershed
performance work is specified as a ground-up runtime rewrite with full deletion
of the existing watershed runtime after replacement, not as incremental
hardening of the current runtime. Revision 2 dispositions dual-review findings
by tightening pass-payload validation, benchmark truthfulness, `--jobs`
authority, consumer-path proof, deletion-test coverage, and implementation gate
requirements. Revision 3 adds the fixture ladder and auditability rule:
arboreal-dendrite remains a tiny smoke/baseline fixture, carnivorous-adobo is
the preferred near-term watershed-development fixture, larger 1,000+
hillslope fixtures are required after runtime progress, and any adopted gate
fixture must be committed to this repository for future auditability. Revision
4 dispositions Claude static verification by adding the sidecar-discovery
measurement axis, recording the roadmap activation requirement, naming
`chan_out`, and making the latest-event `NoEvent` decision an explicit
contract-first follow-up. This document is not ratified yet; it should be
promoted by a follow-on ADR after implementation package review.

Audience: contributors working on watershed CLI, watershed orchestration,
hillslope fanout, HBP/pass handoff, output publication, and performance
packages.

Owner: architecture authority; implementation by Codex work packages.

Related authority:

- [ADR-0004](../decisions/0004-subprocess-hillslope-orchestration.md):
  subprocess-per-hillslope remains the process boundary.
- [array-native-runtime-specification.md](array-native-runtime-specification.md):
  hillslope runtime representation authority and performance precedent.
- [watershed-dispatch-scheduler-graph.md](watershed-dispatch-scheduler-graph.md):
  deterministic channel/impoundment dispatch graph.
- WSHEDPERF01:
  `../work-packages/20260701-wshedperf01-watershed-baseline-performance-characterization-001/`.

Last updated: 2026-07-01.

---

## 0. Summary

The watershed performance bottleneck is not the routed channel stage on the
arboreal-dendrite fixture. WSHEDPERF01 measured current openWEPP routed-stage
execution from existing pass artifacts at about `0.07-0.08 s`, while the full
openWEPP command chain repeated at `1:01.06-1:02.38` (`avg 1:01.62`). The
architecture work therefore starts with **CPU-scalable hillslope fanout and
deterministic artifact handoff**, not with micro-optimizing watershed routing.

The architecture program is explicitly a **ground-up watershed runtime rewrite
and retirement of the existing runtime**. The current CLI body, shared output
path assumptions, path-dependent pass discovery, `WatershedWritebackSurface`,
symbol-keyed watershed request/writeback maps, and tests that exist only to
assert those obsolete surfaces are scaffolding. They may exist temporarily for
comparison and migration, but the destination state deletes them from production
rather than preserving them as a compatibility runtime.

The target runtime shape is:

```text
watershed request
  -> typed watershed run plan
  -> bounded subprocess worker pool for hillslope jobs
  -> typed pass inventory and freshness validation
  -> typed watershed network frame
  -> deterministic channel/impoundment dispatch
  -> typed watershed publication projection
  -> legacy-compatible parquet/HBP-sidecar outputs
```

The runtime must not remain:

```text
shell loop over ad hoc runfiles
  -> shared mutable output directory
  -> path-dependent HBP discovery
  -> symbol-keyed BTreeMap watershed state
  -> single row seed from writeback maps
```

The first implementation goal is for `openwepp` to use additional CPUs through
the new runtime, by running multiple single-threaded hillslope subprocesses
concurrently under a typed supervisor. Each hillslope process remains
single-threaded. The watershed supervisor controls parallelism with a bounded
job count, validates all produced pass artifacts, and routes only after the pass
inventory is complete and deterministic.

The second implementation goal is full deletion of the existing watershed
runtime once the new runtime is validated. The typed network frame is required
for maintainability, testability, and later large-watershed scaling. WSHEDPERF01
shows it is not the first walltime lever on arboreal-dendrite, but it remains a
current-scope architecture requirement because preserving the existing
symbol/writeback runtime would repeat the hillslope compatibility-runtime
mistake.

---

## 1. Motivation and Evidence

### 1.1 WSHEDPERF01 Baseline

WSHEDPERF01 recorded these arboreal-dendrite timings:

| Surface | Scope | Repeats | Wall | User | System | Max RSS | Evidence |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| pinned legacy WEPP | full legacy watershed | 1 | `0:07.86` | `6.04` | `1.81` | `2877312 KB` | `watershed-baseline-timing.md` |
| openWEPP watershed CLI | routed stage from existing HBP/pass | 3 repeat + 1 canonical | `0:00.07-0:00.08` | `0.07` | `0.00` | `8448 KB` | `watershed-baseline-timing.md` |
| openWEPP full command chain | 36 hillslope CLIs + watershed routing | 3 stability repeats + 1 profile run | `1:01.06-1:02.38` (`avg 1:01.62`); profile `1:02.07` | `avg 60.69` | `avg 0.60` | `16896 KB` | `watershed-baseline-timing.md` |

The full openWEPP chain now has three stability repeats. It is sufficient for a
draft architecture direction and scoped engineering budget. It still is not a
legacy-equivalent speedup/parity claim: WSHEDPERF01 explicitly records that the
pinned legacy full watershed run and current openWEPP full command chain are
not equivalent scopes unless a fresh legacy-equivalent hillslope generation plus
routing surface is introduced. Cross-scope comparisons must be labeled as
contextual engineering budget only.

The accepted WSHEDPERF01 full-chain runs used `--legacy-sidecar-discovery`.
That mode is part of the measured baseline scope. Future canonical benchmark
runs that remove legacy sidecar discovery are a different scope unless the
performance record labels the sidecar/input-discovery mode and justifies the
comparison.

### 1.2 Architectural Signal

The routed-stage command is too small to explain the end-to-end gap. Its
`perf stat` sample recorded about `80.46 msec task-clock`, `338,400,048`
instructions, and low memory. The full chain's repeated user time averages
about `60.69 s`, so current end-to-end cost is dominated by hillslope execution
and command fanout. Arboreal-dendrite is still useful as a tiny smoke/baseline
fixture, but it is not enough by itself to drive watershed runtime architecture.

This means:

1. Process-level parallelism is the primary performance lever.
2. Shared-output path conventions and runfile materialization must be made
   deterministic before performance numbers are trusted.
3. Routed-stage typed cleanup is still necessary, but it is not the first
   walltime package.
4. Fixture strategy must include more than arboreal-dendrite: a small
   multi-hillslope development fixture first, then larger stress fixtures once
   the new runtime is stable.

### 1.3 Existing Runtime Constraints

Current `openwepp-cli-watershed`:

- parses watershed structure, channel, impoundment, slope, and optional
  `chan.inp`;
- validates topology;
- loads HBP/pass latest-event payloads for contributor hillslopes;
- seeds `WatershedWritebackSurface` maps keyed by `BoundarySymbol`;
- runs deterministic channel/impoundment dispatch through
  `execute_watershed_dispatch_with_kernel`;
- writes watershed interchange outputs from a row seed projected from the final
  writeback surface.

That is acceptable as a compatibility scaffold, but it is not the target
runtime. The target is a typed job plan and typed network frame, with
symbol-keyed maps retained only at compatibility, replay, and diagnostic edges.

### 1.4 Hillslope Performance Lesson

The hillslope performance program showed that partial refactors can preserve the
old runtime's costs while adding new representation-management costs. The
watershed program must not repeat that pattern by making the current
watershed CLI/runtime cheaper one edge at a time.

For watershed work:

- replacement is the objective, not compatibility-runtime optimization;
- old-runtime tests whose only purpose is to assert obsolete surfaces may be
  deleted during the full-deletion package;
- tests that protect science contracts, topology failures, output schemas,
  conservation-sensitive operands, or user-facing behavior must be backfilled
  against the new runtime before closure;
- deletion can precede backfill for tests tied solely to removed internal
  surfaces, as happened during hillslope compatibility-runtime deletion, but the
  package must record which tests were deleted and which contract/user-facing
  assertions replaced them;
- deletion packages must include a pre-deletion test classification, protected
  assertion inventory, and net-coverage restoration plan. The hillslope
  deletion caveat remains a warning: wholesale deletion is valid only when
  removed internal-surface tests do not silently retire contract/user-facing
  coverage;
- old runtime selection must not survive as a public mode after the deletion
  package.

---

## 2. Core Architecture Thesis

1. **Ground-up runtime replacement is the target.**
   Implementation packages should build the new watershed runtime directly and
   delete the old production runtime after validation. Do not optimize the old
   shell-loop/path-discovery/symbol-writeback runtime into a long-lived
   compatibility path.

2. **Watershed scaling is process-level first.**
   A watershed run owns a bounded pool of single-threaded hillslope subprocesses.
   The parent process supervises, logs, and validates; it does not parallelize
   one hillslope internally.

3. **The supervisor owns a typed run plan.**
   Path resolution, runfile selection, output roots, pass filenames, manifest
   filenames, expected outputs, and retry/failure policy are computed before any
   child process starts.

4. **Child processes never share write targets.**
   Every hillslope job writes to an isolated scratch/output directory. The
   supervisor moves or indexes completed artifacts through a typed pass
   inventory. A shared `runfiles/output` directory is not a production
   concurrency surface.

5. **Completion order is not publication order.**
   Jobs may finish in any order. The pass inventory and watershed routing input
   order are sorted by hillslope id and topology, so `--jobs 1` and `--jobs N`
   produce identical routed outputs.

6. **Routing remains deterministic and gated.**
   The channel/impoundment graph keeps stable topological dispatch. Routing
   starts only after required hillslope pass artifacts validate, unless a future
   package explicitly proves safe streaming routing for a narrower graph class.

7. **Typed network state replaces symbol maps; it is not optional cleanup.**
   `BoundarySymbol`/`BoundaryValue` maps are adapter state. Production channel
   routing should read typed vectors/structs for channel controls, impoundment
   controls, hillslope contributions, routed fluxes, and sediment payloads. The
   old symbol/writeback runtime must be deleted from production after the new
   runtime is accepted.

8. **Legacy compatibility is an edge, not an executor.**
   Legacy sidecar discovery, HBP aliases, and legacy-shaped diagnostics may
   remain for intake/replay/comparator support, but benchmark and production
   runs must be able to use canonical paths without discovery warnings.

9. **Old-surface tests are not sacred.**
   Tests coupled to deleted runtime internals should be removed with the old
   runtime and replaced with tests for typed plan construction, worker-pool
   determinism, pass inventory validation, typed routing, and publication
   contracts. Keeping obsolete tests alive by wrapping old surfaces is a defect.

---

## 3. Target Components

### 3.1 `WatershedRunPlan`

`WatershedRunPlan` is the immutable plan built from the watershed run request.
It contains:

- run id and scratch/output roots;
- sidecar policy and compatibility policy;
- CPU/job count;
- sorted `HillslopeJob` list;
- watershed structure/channel/impoundment/slope paths;
- expected pass inventory;
- expected watershed output contract;
- failure policy.

The plan is built once, logged once, and reused by timing/profiling evidence.
It replaces ad hoc shell loops and package-local runfile mutations.

### 3.2 `HillslopeJob`

Each `HillslopeJob` owns:

- hillslope id;
- input runfile path or generated runfile document;
- read-only input root;
- isolated output root;
- expected pass file;
- expected manifest file;
- stdout/stderr log paths;
- timeout/resource policy if configured;
- final status and timing record.

The child command is constructed with `std::process::Command` and explicit
arguments. Shell interpolation is not part of the production path.

### 3.3 `HillslopeWorkerPool`

The worker pool schedules `HillslopeJob` values with bounded concurrency:

- `--jobs N` is the public CLI control.
- Default policy is not ratified by this draft. Until ADR ratification,
  implementation packages must require explicit `--jobs` for performance runs
  and use `--jobs 1` as the deterministic functional baseline. Candidate
  defaults such as `min(available_parallelism, hillslope_count)` require ADR
  selection before becoming production behavior.
- `--jobs 1` is the deterministic serial baseline.
- `--jobs N` must preserve identical outputs relative to `--jobs 1`.
- On hard failure, the supervisor stops launching new jobs, waits for or
  terminates in-flight jobs according to policy, and does not route.

Controller threads are acceptable. Compute remains in child processes, so each
hillslope process keeps its single-threaded kernel/runtime contract.

### 3.4 `PassInventory`

`PassInventory` is the only routing handoff surface. It records, per hillslope:

- pass file path and size;
- pass parser metadata (`hillslope_id`, `nofe`, particle class count);
- optional manifest metadata and checksums;
- latest event payload state and fields used by watershed routing;
- freshness/provenance relation to the `HillslopeJob`;
- validation status.

The inventory rejects missing, empty, stale, wrong-id, wrong-class-count,
schema-invalid, or semantically ambiguous pass artifacts before routing. Latest
event state must be typed as either:

- `EventPayload`, with all runoff/sediment fields required for the selected
  watershed routing policy; or
- `NoEvent`, with explicit parser evidence that the pass file represents a
  valid no-runoff/no-sediment event state for that contributor.

The new runtime must not silently synthesize zero runoff, zero duration, zero
detachment, zero deposition, or zero sediment concentrations just because a
latest-event payload is absent. Zero values are allowed only as produced values
inside a validated `EventPayload` or as the typed consequence of a validated
`NoEvent` state. Reusing existing pass files is an explicit run-plan mode, never
a silent fallback.

### 3.5 `WatershedNetworkFrame`

`WatershedNetworkFrame` is the typed replacement for production
`WatershedWritebackSurface` use. It stores:

- topology nodes and stable dispatch order;
- channel controls and per-channel geometry;
- impoundment controls and coefficient tables;
- hillslope runoff/sediment contribution vectors by hillslope id;
- routed channel state/flux arrays by channel id;
- publication accumulators and diagnostics.

The frame may expose compatibility projection methods, but channel/impoundment
kernels do not perform symbol lookup during production routing.

### 3.6 `WatershedPublicationFrame`

`WatershedPublicationFrame` is the output projection edge for:

- `ebe_pw0.parquet`;
- `chan.out.parquet` (`chan_out` output/runfile field);
- `chanwb.parquet`;
- `chnwb.parquet`;
- `soil_pw0.parquet`;
- `totalwatsed3.parquet`;
- loss outputs.

Publication keeps schema compatibility. Operand lineage, units, aliases, and
anti-alias validation remain required for conservation-sensitive surfaces.

### 3.7 Deleted Runtime Surface

The following surfaces are not destination architecture:

- production execution through ad hoc package-local shell loops;
- production routing through `WatershedWritebackSurface`;
- channel/impoundment kernels reading `BTreeMap<BoundarySymbol, BoundaryValue>`;
- public runtime modes that select the old watershed executor;
- tests whose assertions are only about old map keys, old path-discovery
  behavior, shared output directories, or other removed internal surfaces.

Deletion packages must distinguish obsolete-internal tests from protected tests.
Protected tests are backfilled or migrated; obsolete-internal tests are deleted
with an explicit manifest. A deletion package cannot close if it reduces net
protected coverage without either restoring that coverage at the new stable
runtime surface or recording an `EXECUTED-HOLD` blocker with a concrete
coverage-restoration package.

---

## 4. Execution Model

### 4.1 Serial Baseline

`--jobs 1` executes:

```text
build run plan
  -> run hillslope jobs in sorted hillslope id order
  -> validate pass inventory
  -> build typed network frame
  -> route channel/impoundment graph
  -> publish outputs
```

This mode is the deterministic baseline for functional comparison and
debugging. It should replace the package-local shell loop used by WSHEDPERF01.

### 4.2 Parallel Fanout

`--jobs N` executes:

```text
build run plan
  -> enqueue sorted hillslope jobs
  -> run up to N child processes concurrently
  -> collect job timing and status
  -> sort completed artifacts by hillslope id
  -> validate pass inventory
  -> route and publish identically to --jobs 1
```

Parallel completion order is never allowed to affect pass inventory order,
routing input order, output row order, warnings, or checksums.

### 4.3 Failure Behavior

Failures are typed and fail closed:

- bad plan: no child process starts;
- child exit failure: route stage is skipped;
- invalid pass artifact: route stage is skipped;
- topology validation failure: route stage is skipped;
- output contract failure: run exits non-zero after preserving logs and scratch
  paths for diagnosis.

The supervisor must write enough per-job evidence to identify which hillslope
failed without scanning shared directories.

### 4.4 Reuse Mode

Existing pass files may be routed without rerunning hillslopes only when the
run plan explicitly selects reuse mode. Reuse mode must record:

- pass root;
- expected hillslope id set;
- validation results;
- whether outputs are fresh enough for the requested benchmark/production
  policy.

Reuse mode is useful for routed-stage profiling. It is not a substitute for
end-to-end watershed performance evidence.

### 4.5 Consumer-Path Proof

Any package that claims the new runtime is active, direct, ready, or production
default must prove the real downstream consumer reads the new path. Required
evidence includes:

- producer source: `WatershedRunPlan`, `HillslopeJob`, `PassInventory`,
  `WatershedNetworkFrame`, or `WatershedPublicationFrame`;
- runner handoff: the public CLI/API entrypoint that selects the new runtime;
- downstream consumer: the routing/publication call site that consumes the new
  typed structures;
- negative proof: the old shell-loop/shared-output/symbol-writeback path is not
  used for that claim;
- output proof: produced watershed outputs are generated through the new path.

Producer-only scaffolding, counters, or shadow data cannot close a production
runtime claim.

---

## 5. Performance Contract

### 5.1 Measurement Surfaces

Every watershed performance package must label timing scope:

- `legacy-full-watershed`;
- `openwepp-routed-stage-from-existing-pass`;
- `openwepp-end-to-end-jobs-1`;
- `openwepp-end-to-end-jobs-N`.

Every watershed performance package must also label sidecar/input-discovery
mode:

- `legacy-sidecar-discovery-on`;
- `canonical-sidecar-discovery-off`;
- `strict-committed-fixture`.

WSHEDPERF01 full-chain evidence is `legacy-sidecar-discovery-on` because the
validated command used `--legacy-sidecar-discovery`. The canonical performance
target is `canonical-sidecar-discovery-off` or `strict-committed-fixture` after
fixture adoption, so the first CPU-scaling package must not compare
discovery-on and discovery-off timings as the same measurement surface.

Ratios are valid only when the compared scopes are named and justified. Direct
speedup/parity language is allowed only for equivalent scopes. Cross-scope
legacy comparisons are contextual engineering-budget evidence, not parity
evidence.

### 5.2 Initial Viability Gate

The first CPU-scaling package should measure `--jobs 1,2,4,8,16,24,36` on
arboreal-dendrite where hardware permits and report:

- median of at least three clean repeats;
- wall, user, system, max RSS;
- job count and CPU inventory;
- sidecar/input-discovery mode;
- per-job duration distribution;
- route-stage duration;
- output identity between `--jobs 1` and `--jobs N`;
- contextual comparison against the pinned legacy full watershed baseline,
  explicitly labeled as cross-scope unless a legacy-equivalent openWEPP surface
  is introduced.

The engineering target is simple: at a useful job count on the available host,
median openWEPP end-to-end walltime should fall below the pinned legacy full
watershed walltime. Until the scopes are made equivalent, this is a cross-scope
operator budget, not a speedup/parity claim. If the budget is not met, the
package must identify the remaining bottleneck by evidence.

### 5.3 Secondary Runtime Cleanup Gate

After process fanout is stable, typed network-frame work should use separate
gates:

- no regression in routed-stage walltime;
- output identity or documented contract-governed deltas;
- no `BoundarySymbol` lookup in production channel/impoundment kernel loops;
- no legacy sidecar discovery warnings in canonical benchmark mode, with
  discovery mode labeled separately from WSHEDPERF01;
- retained replay/comparator adapters remain available at edges.

### 5.4 Implementation Closure Gates

Any W2-W5 package that edits production Rust or deletes runtime/test surfaces
inherits the root closure loop unless a package-local, pre-authorized hold
boundary states otherwise:

1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo nextest run --workspace --profile full`
4. `cargo deny check`
5. package-specific identity, consumer-path, pass-inventory, and deletion-test
   manifest gates.

Fast local loops may use narrower package gates, but final closure must either
run these gates or close in `EXECUTED-HOLD` with the exact blocker.

### 5.5 Fixture Ladder and Auditability

Watershed runtime work needs a fixture ladder, not a single small case:

- **Smoke/baseline tier:** arboreal-dendrite remains the historical tiny
  benchmark and smoke fixture for comparing with WSHEDPERF01 evidence.
- **Near-term development tier:** `/wc1/runs/ca/carnivorous-adobo/wepp` is the
  preferred next development candidate because it is still small enough for fast
  local iteration while exercising a 32-hillslope watershed.
- **Large-scaling tier:** after W2/W3 make progress, add one or more larger
  watershed fixtures. Watersheds with more than 1,000 hillslopes are common, so
  the scaling gate must eventually include at least one fixture in that class or
  an explicitly justified representative reduction.

Paths under `/wc1/runs/...` are exploratory substrates only. A fixture is
**adopted** when it becomes part of a work-package gate, ratification
requirement, recurring benchmark, regression test, or release-readiness claim.
Every adopted watershed fixture must be committed to this repository, preferably
under a dedicated `tests/fixtures/watershed/` subtree, with enough provenance for
future audit:

- source substrate path and date captured;
- expected hillslope count and topology summary;
- input/runfile files needed to reproduce the gate;
- expected output manifest or checksum set when outputs are part of the gate;
- fixture README naming the adopting package and intended scope.

Do not make `/wc1`-only, operator-local, or scratch-only data the sole evidence
for a persistent gate. If a fixture is too large to commit, the package must
hold or first define a committed reduced fixture plus a separate external-scale
exploration note; it cannot ratify the gate on uncommitted data alone.

---

## 6. Migration Sequence

### W1 - Baseline Refresh / Independent Confirmation

WSHEDPERF01 already records three stability repeats plus one profile run for
the current full openWEPP command chain. W1 is therefore satisfied for draft
architecture orientation. Re-run W1 only when code changes, machine changes, or
ADR ratification requires independent confirmation of the baseline. This
package does not change code; it hardens or refreshes the baseline and command
surface.

### W1A - Adopt Carnivorous-Adobo Development Fixture

Before using carnivorous-adobo as a repeated development or acceptance gate,
commit an auditable fixture derived from `/wc1/runs/ca/carnivorous-adobo/wepp`
into the repository. This is a fixture-adoption package, not runtime
optimization.

Acceptance:

- committed fixture contains all required inputs/runfiles for the intended
  watershed gate;
- fixture metadata records `32` hillslopes, source substrate path, and capture
  provenance;
- fixture README defines whether it is smoke, development, performance, or
  output-contract scope;
- package evidence proves tests/benchmarks read the committed fixture path, not
  `/wc1` directly.

### W2 - New Runtime Skeleton and Supervisor Plan

Implement `WatershedRunPlan`, `HillslopeJob`, and `PassInventory` without
parallelism in the new runtime path. The public command still executes
serially, but it no longer depends on shell loops, shared output directories, or
ad hoc path rewrites. This package should avoid adding abstraction to the old
runtime except where needed to preserve comparator access.

Acceptance:

- `--jobs 1` outputs match the validated serial command;
- per-job logs and timings are written;
- pass inventory validates before routing;
- latest-event payload handling is resolved contract-first, then implemented as
  fail-closed `EventPayload` or explicitly typed valid `NoEvent`;
- existing routed-stage reuse mode remains available;
- consumer-path proof shows the public runner reads the new plan/inventory path
  and does not use the old shell/shared-output path for the claim.

### W3 - Bounded Worker Pool

Add `--jobs N` and run multiple hillslope child processes concurrently.

Acceptance:

- `--jobs 1` and `--jobs N` routed outputs are identical;
- failures fail closed before routing;
- scaling curve is recorded on arboreal-dendrite;
- scaling evidence labels sidecar/input-discovery mode and does not compare
  discovery-on and discovery-off timings as the same surface;
- median `--jobs N` end-to-end walltime is compared to pinned legacy as a
  labeled cross-scope engineering budget unless a legacy-equivalent scope has
  been introduced;
- consumer-path proof shows the public runner uses the worker pool and old
  serial shell-loop orchestration is not used for the claim.

### W4 - Typed Watershed Network Frame

Replace production routing reads/writes through `WatershedWritebackSurface` with
typed channel, impoundment, contribution, and publication frame fields.

Acceptance:

- no symbol lookup in production routing loops;
- protected outputs remain identity-equivalent or deltas are contract-governed;
- routed-stage timing does not regress materially;
- consumer-path proof shows routing/publication consumes `WatershedNetworkFrame`
  data and does not construct/read `WatershedWritebackSurface` for production
  routing.

### W5 - Full Runtime Deletion

Delete the old watershed production runtime and obsolete internal tests. The
new runtime becomes the only production executor. Compatibility code may remain
only as an explicit edge adapter for input/output/replay/comparator support.

Acceptance:

- no public CLI/API mode selects the old runtime;
- no production routing path constructs `WatershedWritebackSurface`;
- tests tied only to old internal surfaces are deleted with a manifest;
- protected science, topology, output, and user-facing assertions are backfilled
  against the new runtime;
- net protected coverage is restored or the package closes `EXECUTED-HOLD` with
  a named restoration package;
- `--jobs 1` and selected `--jobs N` outputs remain deterministic and validated;
- consumer-path proof shows no production runtime path selects the deleted
  runtime.

### W6 - Publication and Large-Watershed Scaling

Make watershed publication stream from typed projection state and measure larger
fixtures. Consider topological parallel routing only if routed-stage timing
becomes material on larger networks.

---

## 7. Non-Goals

- Do not replace subprocess-per-hillslope with in-process hillslope linkage in
  this architecture revision.
- Do not introduce PyO3/FFI bindings.
- Do not change hillslope physics, watershed routing physics, erosion/sediment
  authority, or output schemas for performance reasons.
- Do not use compatibility sidecar discovery as a canonical benchmark path.
- Do not claim legacy parity or speedup from routed-stage-only measurements.
- Do not preserve the old watershed runtime as a permanent compatibility mode.
- Do not keep tests alive by wrapping or reintroducing removed runtime internals.

---

## 8. Open Questions

1. Should the supervisor live in `openwepp-cli-watershed` or a separate
   `openwepp-cli-watershed-run` entrypoint while routed-stage reuse remains
   available?
2. What is the canonical scratch-directory retention policy for failed
   subprocess jobs?
3. Should `--jobs` default to all logical CPUs, physical cores, or a
   wepppy-supplied value?
4. Which artifact hashes are required before existing pass reuse is considered
   fresh enough for production?
5. Which 1,000+ hillslope fixture should become the large-scaling gate after the
   new runtime's worker-pool path is stable on committed small fixtures?
6. Which science contract defines when a pass with no latest-event payload is a
   valid `NoEvent` state rather than a hard error?
7. Which sidecar/input-discovery mode is the ratified canonical benchmark mode
   once committed watershed fixtures are in place?

---

## 9. Ratification Requirements

Before this document becomes binding architecture authority:

1. WSHEDPERF01 repeat evidence is accepted as the current baseline, or a
   follow-on benchmark package refreshes it after relevant code or host changes.
2. An ADR selects the public entrypoint, default `--jobs` policy, and canonical
   sidecar/input-discovery benchmark mode.
3. A W2 implementation package proves serial supervisor output identity.
4. A W3 implementation package proves parallel output determinism and records a
   scaling curve.
5. Carnivorous-adobo or any successor adopted fixture is committed to the repo
   with provenance before it is used as a persistent gate.
6. W2-W5 packages include consumer-path proof for every production-readiness
   claim.
7. A W5 deletion package removes the old watershed runtime from production or
   records the exact blocker that prevents deletion.
8. W2-W5 packages run or explicitly hold on the required Rust closure loop.
9. Work-package evidence confirms no production source changes were made merely
   to fit the benchmark harness.
10. `docs/ROADMAP.md` carries the active WSHED-ADR/W2-W6 planning queue until
    the rungs close and move to the work-package execution log.
