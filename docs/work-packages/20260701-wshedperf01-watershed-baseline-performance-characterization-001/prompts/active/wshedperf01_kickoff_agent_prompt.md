# WSHEDPERF01 Kickoff Agent Prompt

Scope: local repository engineering/performance characterization task;
flat-file reads/edits and local benchmark commands only; no external
connectivity or production service actions.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in
`docs/work-packages/20260701-wshedperf01-watershed-baseline-performance-characterization-001/package.md`
sequentially through disposition.

Required reading (read before canonical timing):

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/20260701-wshedperf01-watershed-baseline-performance-characterization-001/package.md`
- `docs/work-packages/20260701-wshedperf01-watershed-baseline-performance-characterization-001/artifacts/required-reading-map.md`
- `docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/package.md`

Conditional:

- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` when discovering
  exact CLI args or output behavior.
- `docs/work-packages/20260616-perf-high-ofe-hillslope-characterization-001/package.md`
  when extracting hillslope timing lessons.
- `docs/work-packages/20260616-perfarch01-indexed-runtime-surface-design-001/package.md`
  when extracting architecture/profiling lessons.
- `docs/work-packages/20260616-perfopt01-runtime-surface-map-churn-001/package.md`
  when extracting optimization measurement patterns.
- `docs/work-packages/20260630-direct-publication-streaming-sink-001/package.md`
  when extracting current direct/RSS endpoint context.
- `docs/ROADMAP.md` when checking current performance-track state.

On-demand:

- `docs/decisions/0004-subprocess-hillslope-orchestration.md` if defining
  end-to-end pipeline/concurrency boundaries.
- Tests or helper files discovered with `rg "openwepp-cli-watershed|arboreal-dendrite"`
  if command invocation is unclear.

Required-reading budget: core `55971` bytes at scaffold time, maximum listed
core plus conditional context `217228` bytes, `OK`; map:
`docs/work-packages/20260701-wshedperf01-watershed-baseline-performance-characterization-001/artifacts/required-reading-map.md`.

Files:

- `docs/work-packages/20260701-wshedperf01-watershed-baseline-performance-characterization-001/**`
- `docs/work-packages/README.md` active/held pointer only, if status needs update

Task: execute the package objective end-to-end for the declared scope. Build the
release openWEPP watershed CLI, run the arboreal-dendrite baseline measurements
against legacy where runnable, measure openWEPP routed-stage performance, collect
profiling/coarse attribution, and write the architecture handoff.

Constraints:

- No production source edits.
- No branch creation or switching.
- Baseline provenance defaults to
  `/workdir/wepp-forest_260430_baseline/release/wepp_260430`.
- Do not silently substitute a current legacy binary for the pinned baseline;
  record pinned-baseline failure first, then label any current legacy run as
  secondary context.
- Do not compare non-equivalent scopes without naming the mismatch.
- Use release binaries for canonical openWEPP timing.
- Keep benchmark output roots isolated under `/tmp/wshedperf01_<timestamp>/`.

Subagent requirement: REQUIRED: spawn or continue as `comparator_suite_runner`
for all heavy benchmark/profiling runs and comparator-style timing loops; do NOT
run those loops on a premium parent model unless the subagent is unavailable, in
which case record command-level evidence. This prompt explicitly authorizes
subagent spawning/delegation to `comparator_suite_runner` for watershed
benchmark/profiling execution and compact evidence summarization; outputs:
updated package artifacts plus compact metrics and log paths; write access:
bounded to the package directory and the active/held pointer in
`docs/work-packages/README.md`.

Autonomy: execute package phases end-to-end and update required artifacts without
requesting additional user direction unless hard-blocked.

Outputs: update package artifacts and final disposition for all completed
phases, then return a compact summary with artifact paths and any blocked timing
surfaces.
