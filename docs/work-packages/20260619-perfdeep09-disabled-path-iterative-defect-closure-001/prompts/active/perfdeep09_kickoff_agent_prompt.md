# PERFDEEP09 Kickoff Agent Prompt

Scope: local repository science-contract/kernel performance task; flat-file
reads/edits only; no external connectivity.

In `/home/workdir/openWEPP`, execute
`docs/work-packages/20260619-perfdeep09-disabled-path-iterative-defect-closure-001/`
end to end.

Execution mode: package-end-to-end.

Close defect `PERFDEEP09-DISABLED-PATH-R2-BLOCKER` end to end. Diagnose
internally until the mechanism is owned or a branch condition is met. If the
mechanism is owned and authority-supported, implement the remediation, validate,
and complete review/disposition in this package. Do not request a new package
for intermediate diagnostic steps.

Objective: clear the R2+ blocker by making the default-disabled H2637 endpoint
pass the P0 timing gate with all PERFDEEP opt-ins off. Required final gate:
three clean H2637 default-disabled no-UI runs with median `<= 676.67 s`,
protected output identity, zero-cost-disabled proof, and full closure gates.

Required reading:

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/defect_closure_execplans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/ROADMAP.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep09-disabled-path-iterative-defect-closure-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep08-disabled-path-hard-isolation-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep08-disabled-path-hard-isolation-001/artifacts/disposition.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep08-disabled-path-hard-isolation-001/artifacts/perfdeep08-r2-blocker-disposition.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep08-disabled-path-hard-isolation-001/artifacts/perfdeep08-rejected-candidates-ledger.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep08-disabled-path-hard-isolation-001/artifacts/worker-handoff.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/perfdeep07-disabled-path-baseline.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/package.md`
- `/home/workdir/openWEPP/docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/perfdeep07-hold-lift-disposition.md`
- `/home/workdir/openWEPP/docs/architecture/array-native-runtime-specification.md`
- `/home/workdir/openWEPP/docs/decisions/0025-array-native-hillslope-day-frame.md`

Conditional:

- `/home/workdir/openWEPP/crates/AGENTS.md` before Rust edits
- `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md`
  before kernel/runtime-authority edits
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
  if canonical contracts must change
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
  if canonical contracts must change
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
  if canonical contracts must change
- `/home/workdir/openWEPP/tests/AGENTS.md` before root test edits
- `/home/workdir/openWEPP/tools/owcmp/AGENTS.md` before owcmp edits or
  comparator tooling changes

On-demand:

- source files implicated by profiling or micro-benchmark evidence under
  `/home/workdir/openWEPP/crates/openwepp-runner/src/hillslope/`
- source files implicated by profiling or micro-benchmark evidence under
  `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/`
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib_mod/core_types/`
  only for disabled-path data-shape guards or tests

Required-reading budget: `265417`, `OK`; map:
`artifacts/required-reading-map.md`.

Execution order:

1. Populate required-reading and owned-file artifacts, including byte budget.
2. Establish a no-edit same-machine H2637 default-disabled control baseline.
3. Profile or micro-benchmark the retained default-disabled path and attribute
   top costs to named mechanisms before production edits.
4. Remediate the attributed in-envelope mechanism.
5. Run focused tests, protected identity checks, and a screening H2637 timing
   run for each candidate.
6. Retain improved candidates, revert slower candidates unless they are a
   documented prerequisite for a same-package follow-on, and continue the loop
   while the final gate remains failed.
7. Run the final three-run H2637 default-disabled endpoint gate. Required
   median: `<= 676.67 s`.
8. Run full closure gates only when the P0 gate passes:
   `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo test --workspace`, `cargo deny check`, scoped docs lint, and
   `git diff --check`.
9. Complete line-count governance, dual review, finding disposition, dual
   verification, roadmap/catalog updates, and worker handoff.

Constraints: contract-first sequencing if authority changes become necessary;
canonical `SC-*` authority; typed guards; no silent defaults; no
canonicalize-and-proceed for domain violations; no process-physics changes; no
output meaning, unit, schema, or metadata changes; no direct-frame hydrology,
direct executor, R2+ runtime schema, publication cutover, or default opt-in
activation.

Conservation/output acceptance: preserve protected output identity. If execution
discovers a need to change publication operands, units, metadata meaning, or
conservation math, stop and amend the package under the conservation/publication
acceptance rules before production edits.

Subagent requirement: REQUIRED where available for heavy batch/closure/
comparator work. This prompt explicitly authorizes subagent spawning/delegation
to profiler/benchmark runner, comparator runner, reviewer, and verifier roles
for H2637 endpoint runs, protected identity checks, heavy Rust closure gates,
package artifact review, line-count-governance review, and gate-legitimacy
verification. Outputs: compact metrics, log paths, and findings recorded in
package artifacts. Write access: package artifacts only unless the package is
explicitly amended. If subagents are unavailable, record command-level evidence
before running locally.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked. Do not stop
after a single diagnostic run or a single failed candidate. Continue the
attribution/remediation loop until the R2+ blocker is cleared or a legitimate
DC boundary is proven.
