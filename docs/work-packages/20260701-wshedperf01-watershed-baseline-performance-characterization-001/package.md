# WSHEDPERF01 Watershed Baseline Performance Characterization

Status: `EXECUTING-DISPATCHED`

Date opened: `2026-07-01`

Dispatched runner: `comparator_suite_runner` agent
`019f1e3c-8794-7da3-87e7-bd21878c4c56` (`Ramanujan`).

## Objective

Establish the starting performance baseline for the arboreal-dendrite watershed
run by measuring legacy WEPP watershed walltime against current openWEPP
watershed execution surfaces, then produce a profiling-backed architecture
handoff for CPU-scalable watershed execution.

The package is characterization-only. It must not implement performance
optimizations or production behavior changes.

## Rationale

Legacy WEPP watershed execution is single-threaded. openWEPP hillslope execution
is currently single-threaded per hillslope but can be run as multiple concurrent
instances, and watershed-level orchestration can use multiprocessing. Before
choosing an architecture for watershed performance, the project needs a measured
arboreal-dendrite baseline and a clear separation between:

- legacy full watershed binary timing,
- openWEPP routed watershed stage timing from existing hillslope pass artifacts,
- any practical openWEPP end-to-end watershed pipeline timing that includes
  hillslope generation/concurrency.

The immediate goal is not to prove parity or speedup. The goal is to know where
we are starting, what is being timed, and where CPU-scalable work should focus.

## Included Scope

- Read prior watershed routing/output and hillslope performance evidence needed
  to understand the current runtime surface.
- Build release-mode openWEPP watershed tooling before timing.
- Measure arboreal-dendrite legacy watershed walltime against the pinned
  baseline legacy binary when it is runnable:
  `/workdir/wepp-forest_260430_baseline/release/wepp_260430`.
- If the pinned baseline binary cannot run the current arboreal-dendrite
  substrate, record the exact failure and optionally time a current legacy
  binary only as secondary context, not as canonical baseline replacement.
- Measure current openWEPP watershed CLI walltime for the routed watershed stage
  using existing arboreal-dendrite hillslope HBP/pass artifacts.
- If practical without production edits, measure an openWEPP end-to-end pipeline
  variant that includes hillslope instance execution/concurrency; otherwise
  record the missing command surface as an architecture input.
- Capture walltime, user CPU, system CPU, max RSS, command line, exit status,
  output counts, binary hashes, git SHA, CPU inventory, and repeat count.
- Produce first-pass profiling attribution for openWEPP watershed execution,
  enough to distinguish parse/intake, HBP/pass ingestion, routing, and output
  writing costs where current tooling permits.
- Produce an architecture handoff describing current ratios, bottlenecks,
  CPU-scaling hypotheses, and the recommended follow-on work package boundary.

## Excluded Scope

- No production Rust edits.
- No physics, routing, erosion, sediment, or output-schema changes.
- No broad parity/comparator adjudication beyond recording whether the timed
  runs completed and produced expected output surfaces.
- No final architecture decision or ADR.
- No full workspace closure loop unless a local doc/tooling edit unexpectedly
  requires it.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to `comparator_suite_runner` subagents for benchmark/profiling runs and compact
evidence summarization; expected outputs are package artifacts under
`docs/work-packages/20260701-wshedperf01-watershed-baseline-performance-characterization-001/artifacts/`;
write access is bounded to this package directory and, if needed, the single
active/held pointer in `docs/work-packages/README.md`.

The parent agent must not spend premium foreground cycles running the long
legacy/openWEPP timing loops when the `comparator_suite_runner` subagent is
available.

## Required Inputs

- Root instructions: `AGENTS.md`.
- Work-package playbook: `docs/work-packages/AGENTS.md`.
- Prompt standard:
  `docs/standards/prompt-wording-guidance.md`.
- This package and active prompt.
- Arboreal-dendrite substrate:
  `/wc1/runs/ar/arboreal-dendrite/wepp/`.
- Prior watershed closure package:
  `docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/`.
- Hillslope performance and architecture evidence:
  - `docs/work-packages/20260616-perf-high-ofe-hillslope-characterization-001/`.
  - `docs/work-packages/20260616-perfarch01-indexed-runtime-surface-design-001/`.
  - `docs/work-packages/20260616-perfopt01-runtime-surface-map-churn-001/`.
  - `docs/work-packages/20260630-direct-publication-streaming-sink-001/`.
- Watershed CLI:
  `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`.
- Legacy baseline binary:
  `/workdir/wepp-forest_260430_baseline/release/wepp_260430`.

## Intended Write Set

- `docs/work-packages/20260701-wshedperf01-watershed-baseline-performance-characterization-001/**`.
- `docs/work-packages/README.md` active/held pointer only.

No source files are in the intended write set.

## Deliverables

- `artifacts/required-reading-map.md`: exact required-reading map, local byte
  total, and threshold disposition.
- `artifacts/environment-and-input-inventory.md`: machine, CPU, git, binary,
  substrate, command-surface, and run-input inventory.
- `artifacts/baseline-command-log.md`: every timed command, output directory,
  exit status, and raw timing source.
- `artifacts/watershed-baseline-timing.md`: canonical table of legacy and
  openWEPP timing results, with repeat counts and scope labels.
- `artifacts/watershed-profile-attribution.md`: profiling or coarse attribution
  evidence for openWEPP watershed execution.
- `artifacts/watershed-perf-architecture-handoff.md`: recommended architecture
  direction and next package boundary.
- `artifacts/gate-results.md`: package gate table with `PASS`, `FAIL`,
  `BLOCKED`, or `NOT RUN`.
- `artifacts/disposition.md`: final package disposition.
- `artifacts/worker-handoff.md`: compact worker summary and follow-on actions.

## Phase Plan

### Phase A: Orientation and Inventory

1. Read the required inputs.
2. Fill `artifacts/required-reading-map.md`.
3. Record local git SHA/status, CPU inventory, legacy/openWEPP binary locations,
   binary hashes, arboreal-dendrite input layout, and expected output surfaces in
   `artifacts/environment-and-input-inventory.md`.
4. Identify the exact legacy and openWEPP command surfaces before timing.

### Phase B: Build and Smoke Check

1. Build the release-mode watershed CLI with:
   `cargo build --release -p openwepp-runner --bin openwepp-cli-watershed`.
2. Run a non-timed smoke check or one initial timed run only after output roots
   are isolated under `/tmp/wshedperf01_<timestamp>/`.
3. Confirm the openWEPP watershed CLI emits the expected watershed parquet
   surfaces for the routed-stage run, or record the fail-closed error.

### Phase C: Baseline Timing

1. Time the pinned legacy watershed binary on arboreal-dendrite if runnable.
2. Time openWEPP routed watershed stage with release binary and existing HBP/pass
   artifacts.
3. Use at least three repeats for each practical short-running surface. For a
   long legacy run, one successful canonical run is acceptable if the artifact
   explains the cost and records that repeat count limitation.
4. Use `/usr/bin/time -v` or an equivalent timing source that records elapsed
   time, user CPU, system CPU, and max RSS.
5. Keep debug `cargo run` timing out of the canonical table.

### Phase D: Attribution and Architecture Handoff

1. Use `perf stat`, `perf record`, `hyperfine`, or coarse command-level
   attribution where locally available. If unavailable, record why and fall back
   to the timing evidence.
2. Identify whether the current openWEPP watershed cost is dominated by process
   dispatch, file/intake parse, HBP/pass loading, routing math, or parquet/output
   writing.
3. Write the architecture handoff, including:
   - current legacy/openWEPP ratio by measured scope,
   - which measured scope is comparable and which is not,
   - likely CPU-scalable decomposition,
   - lessons to carry over from hillslope performance work,
   - next work package recommendation.

### Phase E: Review and Disposition

1. Complete gate table.
2. Review artifact truthfulness: every metric must be labeled as `Ran:` or
   `Static:` and every skipped run must say `NOT RUN` or `BLOCKED`.
3. Write final disposition and worker handoff.

## Exit Criteria

- `artifacts/environment-and-input-inventory.md` identifies the exact machine,
  CPU count, git state, binaries, hashes, and arboreal-dendrite input/output
  roots used for timing.
- `artifacts/baseline-command-log.md` contains every canonical timed command and
  raw timing source.
- `artifacts/watershed-baseline-timing.md` includes at least:
  - pinned legacy result or explicit pinned-legacy blocker,
  - openWEPP routed-stage result,
  - scope labels and repeat counts,
  - walltime, user CPU, system CPU, max RSS,
  - ratio calculations only where scopes are comparable.
- `artifacts/watershed-profile-attribution.md` contains either profiling output
  or a clear `BLOCKED`/`NOT RUN` reason with fallback timing interpretation.
- `artifacts/watershed-perf-architecture-handoff.md` recommends the next
  architectural direction and defines a bounded follow-on package.
- No production files are edited.
- Any unsupported legacy/current command surface is recorded as evidence, not
  silently replaced.

## Security and Safety Gate

No network, secrets, credentials, production services, or branch changes are in
scope. All benchmark output roots must be local scratch paths, preferably under
`/tmp/wshedperf01_<timestamp>/`, and source/substrate inputs must not be mutated.

## Truthfulness Requirements

- Start reviews/audits with `Static:` and/or `Ran:` evidence labels.
- A successful build is not a benchmark.
- A successful benchmark is not parity validation.
- Ratio claims must name the timed scope; do not compare full legacy watershed
  time against openWEPP routed-stage time without labeling the mismatch.
