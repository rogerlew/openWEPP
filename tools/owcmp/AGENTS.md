# tools/owcmp AGENTS.md
> Local playbook for the openWEPP comparison CLI.

## Purpose

`tools/owcmp` is the active comparison-tooling surface for openWEPP. It owns
semantic WAT comparison, PL14S replay orchestration, compact summaries, and
agent-safe batch execution. Keep this directory focused on comparison tooling
and evidence generation; it is not a production hydrology or erosion kernel.

## Local Rules

- Read `tools/owcmp/specification.md` before changing command behavior, report
  schemas, suite orchestration, or artifact layout.
- Keep `tools/owcmp/README.md` aligned with user/agent-facing commands.
- Use direct CLI invocations (`tools/owcmp/owcmp ...`) in docs and agent config.
  The wrapper re-execs through `.venv/bin/python` when available so parquet
  support and repo-local Python dependencies are available.
- Preserve fail-closed behavior. Missing inputs, duplicate row keys, unsupported
  source classes, and failed subprocesses must emit explicit errors plus compact
  artifact evidence.
- Do not add compatibility paths back under `tools/legacy_comparison_suite`.
  That directory was retired; historical references belong only in archival
  work-package evidence.
- ADR-0017 remains binding: comparator agreement is an investigation flag, not
  an acceptance oracle or physics target.

## Adding or Modifying Commands

Update all of these surfaces together:

- `tools/owcmp/owcmp` dispatcher.
- A focused implementation module, usually one command or suite per file.
- `tools/owcmp/specification.md` for normative behavior, schemas, and artifact
  contracts.
- `tools/owcmp/README.md` for the runnable command surface.
- `tools/owcmp/suites/` for reusable suite manifests when the command or cohort
  should be discoverable by agents.
- Focused tests in `tests/integration/owcmp_cli_contract.rs` or a new
  integration test when the surface is large enough to justify separation.

Command output should be compact. Large per-row reports, per-hillslope JSON, and
logs belong on disk under the declared output root. Chat or parent-agent output
should include only verdicts, pass counts, key metrics, and artifact paths.

## Adding a Suite

When adding a new suite, define these first:

- Suite name and CLI shape, for example `owcmp batch <suite-name>` or
  `owcmp <lane> run`.
- Input authority: baseline/candidate paths, required identity evidence, source
  class, tolerance config, and any expected row/span policy.
- Output layout: `summary.json`, `summary.md`, command log, raw reports, and
  stdout/stderr logs.
- Summary schema keys and failure-path keys. Failure summaries should preserve
  the same stable handoff keys as successful summaries, using explicit
  `NOT_RUN`, empty lists, or null values where appropriate.
- Promotability posture: investigation-only vs promotable evidence. Do not label
  evidence promotable when identity inputs or policy checks are missing.

Prefer composing existing commands instead of duplicating comparison math. For
example, the H1-H39 batch composes the semantic WAT comparator and aggregates
its reports.

For validation cohorts that do not yet define a complete baseline-vs-candidate
comparison pair, add an `owcmp-suite-manifest-v1` `cohort-inventory` manifest
under `tools/owcmp/suites/`. It should declare the run root, expected surfaces,
artifact policy, and `comparator_suite_runner` return contract. It should pass
`tools/owcmp/owcmp env --manifest <path>` before being referenced in prompts.

## Artifact Contract

For batch or suite commands, write artifacts under the declared `--output-root`.
Use this default shape unless the spec says otherwise:

```text
<output-root>/
  summary.json
  summary.md
  command-log.json
  logs/
  reports/
  raw/
```

Failure paths must still write compact artifacts wherever possible. Parent
agents should be able to diagnose a blocked run from `summary.json`,
`summary.md`, `command-log.json`, and log paths without reading raw reports into
model context.

## Tests and Validation

Focused validation for `owcmp` changes usually includes:

```bash
.venv/bin/python -m py_compile tools/owcmp/owcmp tools/owcmp/*.py
cargo fmt --check
cargo test --test owcmp_cli_contract
cargo test --test owcmp_agent_config_contract
git diff --check
```

Add or update tests for:

- Dispatcher discovery and usage strings.
- Success-path summary keys and artifact files.
- Failure-path summary keys and artifact files.
- Config/documentation markers when agent discoverability changes.
- Schema/provenance markers required by existing contracts.

For parquet lanes, ensure `.venv/bin/python -c 'import pyarrow'` succeeds before
running real parquet suites. Do not silently fall back to `.dat` fixtures for a
parquet validation claim.

## Agent Runner Integration

`comparator_suite_runner` is the intended executor for context-heavy suites. If
you add or retire a runner-facing suite:

- Update `.codex/agents/comparator_suite_runner.toml` examples.
- Keep returned metrics compact: command, exit code, execution verdict, pass
  count, structural failures, first divergent key, focus metrics, and artifact
  paths.
- Do not require runner agents to read canonical science contracts unless the
  package explicitly asks for that authority.

## Boundaries

- Do not change Rust kernel/runtime behavior from this directory.
- Do not change `SC-*` science contracts as an incidental comparator-tool edit.
- Do not introduce broad exception swallowing in command paths. If a broad
  boundary is unavoidable, keep it minimal, explain why, and emit explicit
  failure evidence.
- Do not commit generated `__pycache__` directories.
