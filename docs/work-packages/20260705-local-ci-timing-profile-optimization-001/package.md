# LOCAL CI TIMING AND PROFILE OPTIMIZATION

Status: `EXECUTED-COMPLETE`

## Objective

Install a local-CI workflow that reduces repeated full-suite burden without
weakening merge/release evidence:

1. make gate-selection guidance discoverable to agents,
2. add persistent local nextest timing diagnostics,
3. empirically test whether capped nextest fixture groups can be raised on the
   `forest` development machine,
4. add profile/documentation support for sane focused gates, and
5. document which expensive families are release/domain gates rather than
   every-review gates.

## Rationale

The full nextest suite is now a frequent 9-minute-class tax in review loops.
The suite remains the merge/release closure gate for Rust implementation work,
but running it after every narrow review response is not the local-CI
optimization path. The repo already has nextest groups and `quick`/`full`/`frost`
profiles; this package makes that scheduling measurable, raises only empirically
safe caps, and records guidance so agents discover the intended tiering.

## Scope

Included:

- `tools/local_ci/` timing utility and documentation.
- `.config/nextest.toml` profile/group tuning.
- Agent-facing guidance in root/test/work-package docs and standards.
- Work-package evidence recording the empirical concurrency sweep.

Excluded:

- GitHub CI design. The optimization target is the local agent workflow.
- Any kernel, contract, or fixture behavior change.
- Deleting or weakening full-suite release/merge gates for kernel
  implementation packages.

## Review Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only review subagents for local-CI tooling,
nextest-profile, and documentation-soundness review; expected outputs are
review findings with severity, evidence, and recommended disposition; write
access is read-only.

## Write Set

- `.config/nextest.toml`
- `AGENTS.md`
- `tests/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/README.md`
- `docs/standards/local-ci-gate-selection.md`
- `docs/dev-guide/01-orientation.md`
- `docs/dev-guide/07-contributing.md`
- `tools/local_ci/`
- this package directory and `docs/work-packages/README.md`

## Execution Plan

1. Audit existing nextest profiles and JUnit timing evidence.
2. Add a persistent local timing tool that records JUnit-derived slow-test data
   under `target/local-ci-history/`.
3. Sweep capped fixture groups using temporary nextest configs on `forest`.
4. Change committed caps/profiles only where the sweep supports it.
5. Install concise guidance for agents and contributors.
6. Run scoped verification and record evidence.

## Exit Criteria

| Criterion | Required evidence |
|---|---|
| Timing tool works | `py_compile` and successful summary parse of an existing JUnit file |
| Concurrency changes are empirical | `artifacts/empirical-concurrency.md` records commands, caps, wall times, and decisions |
| Guidance is discoverable | root/test/work-package/standard/dev-guide docs link the local-CI standard |
| Profile changes are syntactically valid | targeted nextest sweep runs through temporary configs; final `.config/nextest.toml` parsed by a focused nextest command |
| No kernel/code behavior changed | write set remains tooling/docs/profile only |
| Whitespace gate | `git diff --check` |

## Closure Notes

This package does not waive full-suite closure for Rust implementation work.
It changes local iteration and review-response expectations: use targeted and
profile-appropriate gates first, then run `full` at branch-head merge/release
checkpoints or whenever the package explicitly requires it.

Implementation, local verification, and dual read-only subagent review are
complete. Bernoulli's four findings were accepted and fixed; Bernoulli re-check
and Locke second review reported no remaining merge-blocking findings.
