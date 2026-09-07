# Scaffold and execute: prospective Stage 3 mechanism experiments

## 1. Owner direction and actual objective

Scope: local openWEPP repository engineering, limited to source/document reads and edits, local builds, controlled process execution, and package evidence. No deployment, remote service changes, network actions, or push are required.

Execution mode: **package-end-to-end**. Scaffold and execute the package, not just a plan. Continue through the experiments, applicable validation, independent review/verification, and an architecture handoff.

**The actual goal is a tractable coupled Stage 3/native-vegetation/Lane-D evaluation architecture. These two experiments are bounded inputs to that goal, not a requirement to salvage old optimizations.** A well-supported negative result is useful. Do not turn either experiment into another open-ended recovery campaign.

This is explicit new owner authorization for prospective controlled experiments. The earlier `NO SUCCESSOR AUTHORIZED` disposition does not block this task. Preserve the earlier packages and their historical FAIL/HOLD results; add a forward reference rather than rewriting their conclusions.

Create or resume:

```text
docs/work-packages/YYYYMMDD-stage3-prospective-mechanism-experiments-001/
```

Use the actual local execution date in America/Los_Angeles. Do not duplicate an existing equivalent package. Update the catalog and applicable roadmap.

Implementation intent: **controlled mechanism evaluation + minimal measurement/coverage correction + re-architecture decision support**. This package authorizes new executable experimental implementations and their tests. It does not authorize production promotion, deployment, solver redesign, or declaring watershed throughput qualified. Retained production remains on HOLD.

### Non-negotiable distinction

We are asking:

> What changes when a newly frozen baseline receives exactly one reviewed mechanism change?

We are **not** asking:

> Can we reconstruct the exact source, Git index, allocator state, and executable used in an old failed experiment?

**Historical identity is not an admission requirement for the new experiment.** Freeze new identities. Treat recovered code as design material. Do not search caches, reverse old cleanup patches, recreate historical indexes, or repeat the historical GDB investigation. Existing historical evidence is sufficient context.

## 2. Required reading and accepted starting evidence

Core, before edits:

```text
AGENTS.md
docs/work-packages/AGENTS.md
docs/codex_exec_plans.md
docs/standards/prompt-wording-guidance.md
docs/standards/testing-and-gate-strategy.md
```

Read these sections of the previous packages, not their entire evidence trees:

```text
docs/work-packages/20260904-stage3-authentic-coverage-memory-attribution-001/
  artifacts/final-disposition.md
  artifacts/coverage-findings.md
  artifacts/memory-attribution.md
  artifacts/raw/historical_v31_gdb_take_return_summary.txt

docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/
  artifacts/performance-budget.md              # budgets and relevant mechanisms
  artifacts/tolerance-authority.md             # protected numerical boundaries
  artifacts/science-contracts/component-temperature-dependency-replay/contract_ref.md
```

Conditional: before kernel/runtime, authority, or test changes, read the science-contract AGENTS file, numerical-solver-architecture standard, applicable preparation rules, and the nearest instructions returned by `tools/agents/find-agents --for <exact paths>`. Resolve v61 authority through its real bindings; do not invent a contract name. Read v31's relevant LSE dependency-replay obligations and existing parity tests.

On demand: prior implementation patches/reviews and current source call sites. Do not require a fresh audit of every predecessor review or hash.

Record actual required-reading bytes, ranges, applicable threshold disposition, and exact write paths in `artifacts/required-reading-map.md` before implementation. Use progressive disclosure.

Accepted historical context at checkpoint `e89befa4678eadec039b3e7f7fe0a176af8e9dc5`:

- The historical v31 executable produced 200 completed sweeps of `48/14/10/24` and 1,800 of `54/14/16/24`, ordered as logical/identity-anchor/component-replay/complete. This is 30,800 actual replay probes. The fully centered `58/14/16/28` target was absent because lawful one-sided boundary stencils omit signed probes. Do not describe this as zero replay coverage.
- v61's historical memory cause remains unresolved. A reconstructed feed-forward implementation completed a real consumer run, but it was not historically identical. Neither fact decides a newly controlled treatment effect.
- The old `rss_kib` is a late `VmRSS` point sample, not peak or post-cleanup memory. The previous same-binary observer experiment added substantial timing overhead and was not a feed-forward treatment comparison.

Verify that the named evidence is present and resolve material contradictions; do not reproduce these investigations just to accept their context. Record actual checkout identity and differences from the checkpoint without resetting it.

## 3. Authority, delegation, and protected boundaries

For this openWEPP package, I explicitly authorize Codex to spawn/delegate to subagents whenever package governance requires review, verification, comparator execution, or parallel work.

**This prompt explicitly authorizes subagent spawning/delegation** to two independent reviewers, two independent verifiers, and `comparator_suite_runner`. Reviewers/verifiers are read-only except for their assigned artifacts. The comparator executes frozen commands and writes results, not source. Require that role for heavy runs when available. After one genuine service/tool failure, record the failure and use the repository-permitted local fallback; do not keep retrying an unavailable service. Independent review must not be relabeled parent self-review.

Run measurements serially, without concurrent builds, profilers, or other heavy jobs. Parallel source review is permitted outside measurement windows.

Allowed: implement each previously described mechanism against current source; add observation-only hooks, authentic runner tests, independent audit reconstruction, and prospective experimental authority needed for those exact changes. Correct in-scope harness defects. Contract-first sequencing applies to changes that require canonical authority, including experimental runtime changes.

Explicitly authorize a **new, separately identified, stencil-aware v31 experimental qualification protocol**. This may replace the fully centered existential requirement for the new experiment, not retroactively amend the old result. Likewise, new relative memory/timing experiments need not pass the old 64 MiB retention rule. Preserve scientific requirements and report existing engineering-budget comparisons separately. Do not merely comment out an old assertion and call its old test passing.

Forbidden: changes to equations, constitutive domains, numerical tolerances, stopping criteria, Jacobian arithmetic, adaptive cadence, event localization, physical processes, ownership, restart/publication semantics, exact-one transfers, or fail-closed guards. No surrogate physics, historical solver cascade, approximation, cross-evaluation cache, or hidden production selector.

Prospective experimental authority and tests must be reviewed before behavior changes. Resolve exact implementation paths before edits; new modules and extracted files need prospective entries, not blanket directory write permission.

Use the existing branch for retained package work. Detached local worktrees/source copies and separate target directories are explicitly authorized for isolated experiments. Do not create/switch named branches, alter unrelated changes, delete pre-existing `tmp/`, or push. Local commits for reproducible checkpoints are authorized. Preserve source and scripts before deleting scratch trees.

## 4. Experimental design: one baseline, two independent treatments

Freeze a coherent baseline `A`, including any common minimal harness changes, then create:

```text
F = A + feed-forward carrier treatment inspired by v61
R = A + component-temperature dependency replay inspired by v31
```

Measure `A versus F` and `A versus R` independently. Run F first. Failure of F does not block R. Do not measure R on top of F or add their speedups together. Combined treatment and interaction studies are out of scope.

Each treatment must be a reviewable source delta. Reuse available implementation material when useful, but reconcile it to A and test it as **new code**. Do not claim recovered historical identity. If implementing one mechanism would require a materially different architecture, record the exact dependency and classify that arm `DEFER_TO_REARCHITECTURE`; continue the other arm and the architecture handoff.

Freeze source snapshots, path/content manifests including deletions and new files, Cargo/Nix inputs, compiler/build flags, executable hashes, workload inputs, and commands. A commit plus a complete patch/source snapshot is useful; an opaque aggregate hash without its inputs is not sufficient. Exclude documentary output from executable-source identity so writing a result does not invalidate a binary.

Build outside measurement windows. Discover test executables from Cargo artifacts, not a guessed cache name. Each executable stays unchanged within a series. A source/toolchain/harness change begins a separately identified series. Preserve earlier samples.

### Workloads

Primary: the existing real one-OFE workload:

```text
hillslope::tests::stage3_laned_release_one_ofe_positive_baseline_profile
```

Resolve its current source under:

```text
crates/openwepp-runner/src/hillslope/tests03/stage3_runner_qualification.rs
```

Use the same authenticated fixture/owner construction and real `execute_hillslope_run_with_runtime_policy` consumer path. Add a separately named experimental entry point when necessary. Preserve forcing, topology, validators, and output semantics. Never manipulate state to manufacture a centered stencil.

The historical workload has 48 parent supports, 56 accepted publication supports, 20 direct trials, 32 split children, and 4 accepted microsteps. Confirm A's actual named counts. For an observation-only/mechanism-only treatment, retain exact control/result identity except the explicitly intended lower-level evaluation counts. A changed A caused by later source evolution must be documented, not forced to old constants.

Secondary: authenticated 10- and 19-OFE one-day cases. Use identical inputs within each comparison. These are bounded scale checks, not year/century or watershed qualification. No long-run climate campaign belongs in this package.

## 5. Implement and admit F: feed-forward carrier

Remove only the redundant same-invocation second carrier execution through the reviewed typed feed-forward handoff. Preserve every independent Full/Retry/Half1/Half2/Root, discovery/exact, batch, and canonical-final evaluation that the real execution requires. Do not collapse independent physical work.

Instrument actual provider invocations and carrier executions. Establish A's multiplicity directly. Compare stable invocation keys/multisets between A and F; map each deliberately removed execution to the unchanged returned physical/custody result. The old 400-to-200 observation is context, not a substitute for current counters.

Before timing, run the relevant typed ownership, stale/foreign/reuse rejection, error precedence, rollback, restart, full consumer, and scientific/output parity tests. Report build/check and execution separately. A runnable one-day candidate alone is insufficient admission.

If the candidate changes memory lifetimes, document which values move or remain live across which boundary. Keep this focused on the actual delta; no general allocation framework or lifetime-refactor campaign.

## 6. Implement and admit R: stencil-aware dependency replay

Use the shared canonical physics implementation for complete and replay evaluation. Preserve the existing finite-difference perturbations, branch choices, arithmetic, solver sequence, error precedence, and ownership rules. Keep cross-sweep/cross-map reuse and derivative redesign excluded.

The new coverage oracle must independently reconstruct expected logical probes from actual coordinate stencils and expected evaluator classification. A centered coordinate contributes two signed probes; a lawful one-sided coordinate contributes its actual one. Verify actual starts, completions, errors, and component-replay use against independently enumerated expectations. Do not copy the producer's counters into the expected result.

Require nonzero replay on the real primary workload. Distinguish replay executed from replay eligible. Account for identities, dropped records, and actual lifecycle cardinality. Do not require the old fully centered pattern or substitute a hardcoded `54/14/16/24` gate. Keep existing lawful centered/interior and boundary tests where available without making a new all-centered runner fixture an unrelated prerequisite.

Run forced-complete residual/Jacobian/full-solve, boundary/error/rollback, and real output parity before timing. The oracle's work and audit must be separate from the treatment's performance interval and memory trial. Match optional audit posture across A and R. Use bounded compact counters for timing and detailed traces only for separate correctness/attribution runs.

## 7. Measurement and bounded decision rules

Write and independently review `artifacts/experiment-protocol.md` before comparative measurements. The defaults below are prospective design choices for deciding engineering priority, not new scientific constants or historical qualification rules.

### Timing

For each viable comparison, use two warmup processes per executable followed by **12 fresh-process A/B pairs**, balanced AB/BA in a predeclared order. Pin the same permitted logical CPU/core type, record affinity/environment, and keep allocator, stack, compiler, output, and required-counter posture fixed. CPU 0 is preferred for continuity when available, not an excuse to block the experiment when a documented equivalent is available.

Run the frozen executable directly in its required environment after build. Record monotonic runner wall time and process CPU, scope boundaries, failures, source/binary/input identities, output evidence, and named workload counts. Measure one complete production-run interval; exclude fixture authoring and post-run oracle work, but do not exclude publication performed by the actual runner.

No GDB, high-frequency mapping scans, allocator tracer, or competing build during primary timing. Retain every sample. A correctness failure stops admission of that candidate, not collection of the other candidate. Do not discard valid slow or high-RSS runs. Freeze timeout and invalid-environment rules before execution; a timed-out run is not a speedup.

Report paired differences and ratios, median absolute seconds saved, and a descriptive paired-bootstrap uncertainty interval. Use **at least 5% median end-to-end wall-time improvement, with uncertainty supporting a gain and no material CPU regression**, as the default threshold for recommending a standalone salvage effort. Smaller gains may be architecture building blocks, not another optimization campaign. A target bucket improving while total runtime worsens is not a win.

Allow at most one extension to 24 total pairs if the uncertainty plausibly changes that priority decision. Use the combined dataset. Do not repeatedly rerun until significance or a preferred median appears.

### Memory

Run a **separate, matched A/B memory series of six fresh-process pairs** for each viable candidate. Prefer lightweight process status sampling and explicit phase-boundary mapping snapshots over repeatedly scanning `smaps_rollup`. Match observer code and settings across arms. Reuse the previous observer lessons; do not rerun a large same-binary observer campaign as a prerequisite.

Retain the original endpoint `VmRSS`, plus explicitly named pre-fixture, pre-run, end-run, post-validation, and post-drop/cleanup observations. Obtain child-process lifetime peak RSS via a reliable same-process or parent wait accounting surface, and distinguish it from a sampled active-run maximum. Collect private/anonymous/file mapping evidence at selected boundaries where it resolves a difference. Record the observed PID, readiness, sample gaps, units, and lifetime.

Explicitly drop report, audit, and output buffers in both arms before a post-drop reading. Deleting a directory is not proof that Rust locals or allocator-held pages were released. Current RSS, lifetime HWM, mapped private pages, and live requested heap bytes are different metrics. Do not infer a leak from one high endpoint or infer heap ownership from anonymous RSS alone.

In **three fresh processes per arm, execute ten complete one-day runs with teardown between them**, recording per-iteration lifecycle memory. This tests repeated allocation/lifetime behavior, not ten-day scientific continuity. Report any continuing post-drop growth and the observation horizon; do not claim long-run boundedness from ten repetitions.

For a candidate still worth considering, run **three matched pairs at each of 10 and 19 OFEs**. Report memory/time scaling without calling this the old qualification matrix.

Do not import the old 64 MiB endpoint retention rule as the new experiment's acceptance gate. Report the applicable existing workload-scaled engineering ceilings separately, including baseline failures. For the new decision, assess treatment deltas, growth/lifetime behavior, and whether any increase fits a known bounded working set. An unresolved meaningful memory increase makes a candidate provisional or not worth standalone salvage; it does not trigger an unlimited heap-forensics project.

Use at most one targeted allocation/lifetime experiment to resolve a decision-changing difference. Do not alter global allocator, stack, ASLR, or host memory settings to manufacture a pass. Any diagnostic counterfactual is a separate arm, never the baseline.

## 8. Stop rules that protect the larger objective

Complete the planned comparisons or supply a concrete, evidence-backed rejection/dependency for an arm. Do not stop merely at a scaffold, parser PASS, baseline run, or expected-red test while implementation/validation remains feasible in scope.

Conversely, do not keep rescuing an uncompetitive mechanism:

- At most one planned implementation plus one **performance-driven** refinement per mechanism. Correctness fixes are not evidence and must still be completed before a candidate can be measured or recommended.
- A substantial redesign needed to make a mechanism viable moves that mechanism into the architecture handoff; it does not silently enlarge this package.
- A correct candidate with no material end-to-end benefit is a useful negative result. Preserve it reproducibly and stop tuning it.
- A result still ambiguous after the declared sampling extension is `INCONCLUSIVE_WITH_BOUND`; report the measured interval and deprioritize standalone salvage. Do not call it equivalent or successful.
- Neither historical failure, one rejected arm, nor a frozen sample-limit disposition prohibits the larger re-architecture.

Use `HOLD` only for a genuine unresolved requirement of this package that prevents its promised evidence/disposition. Distinguish experimental rejection from inability to complete the package. Missing old source/index/binary custody is expressly not such a blocker.

## 9. Mandatory architecture handoff

Regardless of both experiment outcomes, produce `artifacts/rearchitecture-handoff.md`. This is a required deliverable, not an optional follow-up.

Use existing telemetry and only the minimal additional counters necessary to explain the current call graph. Report the multiplicative structure where directly observable:

```text
accepted physical supports
  -> provider/adaptive trials
  -> carrier evaluations
  -> LSE solves and nonlinear iterations
  -> Jacobian sweeps and residual/probe evaluations
  -> owner/receipt construction and publication
```

Separate authentic counts from inference. Distinguish exclusive cost from overlapping diagnostic timers; never sum nested buckets as independent work. If a bucket cannot be isolated cheaply, give a labeled bound rather than starting a new profiler campaign.

For A, F, and R, report complete-run cost, intended work eliminated, remaining dominant work, and achieved absolute—not only percentage—savings. Keep secondary-workload results separate from the primary workload. Do not add F and R speedups or infer century qualification.

Estimate the maximum end-to-end gain from eliminating each sufficiently attributed remaining category. Explain the assumptions and overlaps. Compare measured one-day costs with the existing engineering target as a feasibility gap, not a passed scaling projection.

Rank no more than three architectural directions, such as derivative/residual-system redesign, a stable numerical state/scratch representation with publication at accepted boundaries, or removal of repeated coupled evaluation through a newly justified solver/controller architecture. Do not implement them here.

For the preferred direction, provide actual source entry points, required authority changes, protected physics/custody invariants, comparator strategy, first decisive prototype, and a measurable early kill criterion. Explain which experimental mechanism should be reused, subsumed, or abandoned. A 5–20% local gain must not be portrayed as resolution of an orders-of-magnitude throughput gap.

The handoff must remain actionable even if neither mechanism merits production salvage. Do not conclude `NO SUCCESSOR` merely because these local optimizations fail. Separate technical recommendation from authorization to begin a larger production change.

## 10. Validation, reproducibility, and terminal delivery

Follow the repository's impact-based validation strategy. Execute affected contract/typed-guard, LSE/orchestrator/runner, consumer/output-parity, formatting, scoped Clippy, anti-evasion, and line-count gates as applicable to each actual source cut. Critical runtime changes require the corresponding full correctness strength, even in an experiment. Do not substitute `cargo check` for execution or use historical test totals as current results.

Document a per-field operand/source/units map for protected science comparisons. Independently reconstruct closure from produced outputs and compare deterministic scientific content. Separate volatile manifest paths/timestamps through an explicit allowlist; do not normalize arbitrary fields to hide changes. Preserve exact numerical/control/output identity for these behavior-preserving mechanisms, except intentional lower-level work-count changes.

Classify inherited failures and deliberate predecessor expected-red seams precisely; do not suppress them to manufacture release readiness. Use new experimental tests/authority for new claims. No candidate or feature switch enters the retained production execution path through this package.

Require dual independent protocol/implementation reviews and dual terminal verifications on immutable cuts. Use compact finding tables. Review evidence sufficiency, experimental isolation, and opportunity cost—not artifact presence alone. Fix actual blocking defects; do not create new unrelated acceptance obligations at closure.

Retain at least:

```text
package.md
prompts/active/kickoff.md
artifacts/required-reading-map.md
artifacts/experiment-protocol.md
artifacts/source-and-build-manifest.json
artifacts/treatment-results.md
artifacts/results.jsonl
artifacts/science-and-output-parity.md
artifacts/rearchitecture-handoff.md
artifacts/gate-results.md
artifacts/finding-disposition.md
artifacts/review-a.md, review-b.md
artifacts/verification-a.md, verification-b.md
artifacts/final-disposition.md
artifacts/reproduction/                 # scripts, complete patches/snapshots, manifests
artifacts/raw/                          # bounded raw evidence
```

Preserve enough actual source and build/run instructions to recreate each new candidate; `/tmp` paths or binary hashes alone are not a reproducibility kit. Snapshot before cleanup and verify patches apply to their named baseline. Do not commit generated executables or large build trees unless repository policy expressly requires them.

For each arm, choose a supported decision: `PROMISING_FOR_SALVAGE`, `USEFUL_ARCHITECTURE_BUILDING_BLOCK`, `REJECTED`, `DEFER_TO_REARCHITECTURE`, or `INCONCLUSIVE_WITH_BOUND`. None means production-qualified. If an experiment could not execute, state that explicitly and provide the concrete dependency; do not fabricate a negative measurement.

Final delivery must include the new source/experiment identities, a compact A/F/R result table with uncertainty and memory lifetimes, exact science/gate outcomes, independent decisions for F and R, and the preferred architecture direction with its first actionable experiment.

**Finish with an engineering decision and a route into the larger re-architecture—not another demand for unavailable historical custody and not an open-ended instruction to keep micro-optimizing.**


