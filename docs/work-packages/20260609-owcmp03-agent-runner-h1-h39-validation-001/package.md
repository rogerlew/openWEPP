# OWCMP03 Agent Runner H1-H39 Validation

Status: complete

Objective: make `tools/owcmp` the discoverable comparator surface for Codex
batch agents, add a compact H1-H39 semantic WAT batch command, and validate that
`comparator_suite_runner` can run the H1-H39 suite without relying on
`tools/legacy_comparison_suite`.

This ExecPlan is a living document. Keep `Progress`, `Surprises & Discoveries`,
`Decision Log`, and `Outcomes & Retrospective` current while executing. Follow
`docs/codex_exec_plans.md` and `docs/work-packages/AGENTS.md`.

## Purpose / Big Picture

OWCMP01 created the comparator CLI and OWCMP02 removed the old
`tools/legacy_comparison_suite` directory. The next risk is operational: agents
and heavy comparison runners must naturally find `owcmp`, produce compact
metrics, and avoid loading raw per-hillslope reports into premium context. After
this package, an agent can run one H1-H39 semantic WAT batch command and return a
small summary artifact instead of pasting 39 reports or hand-maintained scripts.

## Progress

- [x] 2026-06-09: Confirmed clean tree, root/package instructions, current
  `owcmp` CLI surface, and stale `comparator_suite_runner` legacy examples.
- [x] 2026-06-09: Scaffolded package prompts and evidence artifacts.
- [x] 2026-06-09: Added `owcmp batch h1-h39-semantic` as a thin composition layer over the
  existing semantic WAT comparator.
- [x] 2026-06-09: Updated `.codex` runner config and `tools/owcmp` docs/spec for agent
  discoverability.
- [x] 2026-06-09: Added focused tests for batch command contract and runner-config compliance.
- [x] 2026-06-09: Ran local gates and delegated H1-H39 validation with
  `.codex/agents/comparator_suite_runner.toml`; second delegated run completed
  with exit `0`, execution verdict `PASS`, semantic pass count `0/39`, and
  structural row/key failures `0`.
- [x] 2026-06-09: Accepted and fixed review finding that failure summaries did
  not fully satisfy the documented artifact contract; added failure-path tests.
- [x] 2026-06-09: Completed dual reviews, finding disposition, dual verification, line-count
  governance, worker handoff, and final disposition.

## Surprises & Discoveries

- Observation: `.codex/agents/comparator_suite_runner.toml` still documented
  `tools/legacy_comparison_suite` examples after OWCMP02 removed that active
  directory.
  Evidence: `rg legacy_comparison_suite .codex tools/owcmp` found active hits in
  the runner config.

- Observation: The first scaffold patch was accidentally applied from the
  conversation cwd under `/workdir/wepppy`, not `/workdir/openWEPP`.
  Evidence: `/workdir/wepppy/docs/work-packages/20260609-owcmp03-agent-runner-h1-h39-validation-001`
  existed and `/workdir/openWEPP/...` had no files. The stray directory was
  removed before recreating the scaffold in openWEPP.

- Observation: Direct `tools/owcmp/owcmp` execution used `/usr/bin/python3`,
  while `pyarrow` was available in `.venv`.
  Evidence: the first delegated H1-H39 run failed at H1 with
  `RuntimeError: parquet input requires pyarrow`; `.venv/bin/python -c 'import pyarrow'`
  reported version `24.0.0`.

## Decision Log

- Decision: Add an `owcmp batch h1-h39-semantic` command instead of preserving or
  reviving a package-local HPHYS script.
  Rationale: The batch command keeps orchestration and compact aggregation in the
  supported tool surface while reusing the already-tested `owcmp wat semantic`
  comparator.
  Date/Author: 2026-06-09 / Codex.

- Decision: Treat OWCMP03 as tooling/agent-compliance work, not a kernel or
  science-contract package.
  Rationale: The write set is `.codex`, `tools/owcmp`, integration tests, and
  package docs. It does not alter hydrology, erosion, runtime publication,
  formulas, thresholds, or canonical `SC-*` contracts.
  Date/Author: 2026-06-09 / Codex.

- Decision: Make direct `tools/owcmp/owcmp` invocations re-exec through
  `.venv/bin/python` when that repo-local environment exists.
  Rationale: Agent-facing commands are documented as direct CLI invocations, and
  parquet support is installed in the repo-local Python environment. Without the
  trampoline, the H1-H39 runner misses `pyarrow` and fails before comparator
  metrics.
  Date/Author: 2026-06-09 / Codex.

## Outcomes & Retrospective

Complete. OWCMP03 exposes the H1-H39 batch through `tools/owcmp/owcmp`, routes
raw per-H reports/logs to files, enforces active runner discoverability with
focused tests, fixed the direct-invocation `.venv` dependency gap, and completed
delegated full-suite validation through `comparator_suite_runner`.

## Scope

Included:

- `.codex/config.toml` and `.codex/agents/comparator_suite_runner.toml`
  compliance with the `owcmp` path and compact-output contract.
- `tools/owcmp` batch command, README, and specification updates.
- Focused integration tests proving command discovery, compact summary
  generation, and config compliance.
- End-to-end H1-H39 semantic WAT validation delegated to the comparator runner.
- Work-package evidence, reviews, verification, and disposition.

Excluded:

- Kernel/runtime physics changes.
- New comparator acceptance semantics. ADR-0017 remains authoritative:
  comparator output is an investigation flag, not a target.
- `owcmp observe normalize`; it remains a future observability package.
- Full manifest schema validation beyond existing OWCMP01 behavior.

## Intended Write Set

- `tools/owcmp/owcmp`
- `tools/owcmp/batch_h1_h39.py`
- `tools/owcmp/README.md`
- `tools/owcmp/specification.md`
- `.codex/config.toml`
- `.codex/agents/comparator_suite_runner.toml`
- `tests/integration/owcmp_cli_contract.rs`
- `tests/integration/owcmp_agent_config_contract.rs`
- `Cargo.toml`
- `docs/work-packages/README.md`
- `docs/work-packages/20260609-owcmp03-agent-runner-h1-h39-validation-001/**`

## Plan of Work

First, document required reading and scaffold the package artifacts so the work
can be audited independently. Second, implement `owcmp batch h1-h39-semantic` as
a wrapper that runs `owcmp wat semantic` for H1 through H39 or a supplied
subrange, writes per-hillslope reports and logs under `artifacts/`, and emits
`summary.json` plus `summary.md`. Third, update the Codex comparator runner
configuration to name `tools/owcmp/owcmp` and the compact batch command. Fourth,
add integration tests that prove the batch command can run on tiny fixture WAT
files and that `.codex` no longer points active agents at
`tools/legacy_comparison_suite`. Finally, run local gates and delegate the real
H1-H39 run to the comparator runner.

## Concrete Steps

Run commands from `/workdir/openWEPP`.

1. Implement the batch helper and dispatcher.
2. Update docs/config and tests.
3. Run:

       .venv/bin/python -m py_compile tools/owcmp/owcmp tools/owcmp/*.py
       cargo test --test owcmp_cli_contract
       cargo test --test owcmp_agent_config_contract
       git diff --check

4. If H1-H39 fixture inputs exist, run through the comparator runner:

       tools/owcmp/owcmp batch h1-h39-semantic \
         --baseline-dir /tmp/hphys0303_adr0016_1780691036/reports/hillslope/fixed_baseline_partitions \
         --candidate-dir /tmp/hphys0300_full_20260605T155527Z/hillslope_output \
         --candidate-year-offset 2012 \
         --output-root docs/work-packages/20260609-owcmp03-agent-runner-h1-h39-validation-001/artifacts/runner-h1-h39

## Validation and Acceptance

Acceptance requires:

- `comparator_suite_runner.toml` references `tools/owcmp/owcmp`, not
  `tools/legacy_comparison_suite`.
- `tools/owcmp/owcmp batch h1-h39-semantic` emits compact `summary.json` and
  `summary.md` and leaves raw per-hillslope reports/logs on disk.
- Focused tests pass.
- The delegated H1-H39 run produces command-level evidence and compact metrics,
  or records a hard blocker with path/exit-code evidence.
- Reviews and verification artifacts have no undispositioned findings.

## Idempotence and Recovery

The batch command overwrites only files under its declared `--output-root`.
Rerunning the same command is safe because per-hillslope reports, logs, command
logs, and summaries are deterministic artifacts for the same inputs. If a
required input path is missing, the command must fail closed and report missing
paths rather than inventing substitute fixtures.

## Security and Kernel Impact

Security impact: none expected. The package edits local tooling and agent config
only and does not add network access, secrets, auth boundaries, or production
runtime behavior.

Kernel impact: none. No `SC-*` contract, Rust production kernel, runtime
publication, formula, threshold, or parameterization default is in scope.
