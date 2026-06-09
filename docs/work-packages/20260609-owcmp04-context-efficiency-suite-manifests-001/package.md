# OWCMP04 Context-Efficiency Suite Manifests

Status: complete

Objective: complete the post-OWCMP03 context-efficiency improvements by making
the known comparison cohorts declarative, adding an `owcmp` environment
preflight, documenting artifact retention, and giving work-package authors a
small reusable prompt snippet for delegating comparison execution to
`comparator_suite_runner`.

This ExecPlan is a living document. Keep `Progress`, `Surprises & Discoveries`,
`Decision Log`, and `Outcomes & Retrospective` current while executing. Follow
`docs/codex_exec_plans.md` and `docs/work-packages/AGENTS.md`.

## Purpose / Big Picture

The OWCMP01-03 sequence made `tools/owcmp` the active comparison surface and
proved a delegated H1-H39 run. The remaining context problem was operational:
future agents still had to infer which `/wc1` cohorts matter, how to preflight
them, what artifacts to return, and how much raw output to keep out of premium
context. After this package, agents can discover suites with
`tools/owcmp/owcmp manifest list`, preflight a named cohort with
`tools/owcmp/owcmp env --manifest <path>`, and follow a documented retention
policy that promotes compact evidence by default.

## Progress

- [x] 2026-06-09: Read root, work-package, `tools/owcmp`, tests, and ExecPlan
  instructions.
- [x] 2026-06-09: Added `owcmp-suite-manifest-v1` helper support and dispatcher
  entries for `manifest list`, `manifest show`, and `env`.
- [x] 2026-06-09: Added declarative manifests for the North Idaho H1-H39
  `ksflag=0`, Minnesota corn H1-H43 `ksflag=1`, and Washington Cascades MOFE
  H1-H36 `ksflag=0` cohorts.
- [x] 2026-06-09: Updated `tools/owcmp` docs, specification, local AGENTS
  guidance, comparator-runner config, reusable prompt guidance, and artifact
  retention policy.
- [x] 2026-06-09: Added focused CLI/config contract tests for suite discovery,
  env preflight, preflight-only inventory manifests, and runner discoverability.
- [x] 2026-06-09: Ran focused gates and all three real `/wc1` manifest
  preflights successfully.
- [x] 2026-06-09: Scaffolded OWCMP04 package artifacts after user request to
  complete the work as a formal package.
- [x] 2026-06-09: Completed package review, finding disposition, verification,
  line-count governance, worker handoff, final gates, and disposition.

## Surprises & Discoveries

- Observation: `/wc1/runs/un/unpalatable-rind` does not expose per-H
  `H*.wat.dat` files; it has H1-H39 plot files and an interchange parquet WAT
  file.
  Evidence: manifest preflight for `n-idaho-single-ofe-ksflag0` checks `39`
  `H{h}.plot.dat` files and
  `/wc1/runs/un/unpalatable-rind/wepp/output/interchange/H.wat.parquet`.

- Observation: The Minnesota corn run has `43` H surfaces and the WA Cascades
  MOFE run has `36` H surfaces, not H1-H39.
  Evidence: `owcmp env --manifest` reports `present: 43` for algebraic-radium
  WAT/plot outputs and `present: 36` for arboreal-dendrite WAT/plot outputs.

- Observation: Running Python compile/check commands creates
  `tools/owcmp/__pycache__`.
  Evidence: `find tools/owcmp -type d -name __pycache__ -print` found the
  directory during validation; it was removed before disposition.

## Decision Log

- Decision: Represent the three user-named cohorts as `cohort-inventory`
  manifests instead of executable comparison manifests.
  Rationale: The user supplied run roots and cohort identity, but not complete
  baseline-vs-candidate comparator pairings for every cohort. Preflight
  manifests are the honest, useful abstraction now and can later grow executable
  `args` when a package defines a complete comparison.
  Date/Author: 2026-06-09 / Codex.

- Decision: Add `owcmp env` as a fail-closed preflight with `pyarrow`,
  tolerance-config, suite-count, and optional manifest checks.
  Rationale: OWCMP03 showed parquet support was a real runner dependency.
  Failing before a delegated suite run is cheaper than discovering environment
  drift after loading comparison context.
  Date/Author: 2026-06-09 / Codex.

- Decision: Keep raw reports/logs local by default and promote only
  `summary.json`, `summary.md`, `command-log.json`, and concise disposition
  evidence unless audit needs require more.
  Rationale: The user's budget concern is mostly caused by large comparison
  outputs entering parent context. Compact artifact promotion preserves evidence
  without turning chat into the artifact store.
  Date/Author: 2026-06-09 / Codex.

## Outcomes & Retrospective

Complete. OWCMP04 makes the expensive comparison cohorts discoverable and
preflightable through `owcmp`, gives agents a documented low-context artifact
policy, and pins the new behavior with focused integration tests. No Rust kernel,
science contract, formula, threshold, or production runtime behavior changed.

## Scope

Included:

- `tools/owcmp` manifest/env helper and dispatcher updates.
- Suite manifests under `tools/owcmp/suites/`.
- `tools/owcmp` README, specification, local AGENTS guidance, artifact
  retention policy, and reusable prompt guidance.
- `.codex/agents/comparator_suite_runner.toml` discoverability examples.
- Focused integration tests and package evidence.

Excluded:

- Kernel/runtime physics changes.
- New science-contract authority.
- Claiming comparator pass/fail as an acceptance oracle. ADR-0017 remains
  binding.
- Executing full comparator suites for the Minnesota and WA cohorts; this
  package only makes those cohorts preflightable and discoverable.

## Intended Write Set

- `tools/owcmp/owcmp`
- `tools/owcmp/suite_manifest.py`
- `tools/owcmp/suites/**`
- `tools/owcmp/README.md`
- `tools/owcmp/specification.md`
- `tools/owcmp/AGENTS.md`
- `tools/owcmp/artifact-retention.md`
- `.codex/agents/comparator_suite_runner.toml`
- `docs/prompt_templates/owcmp-comparator-runner-guidance.md`
- `tests/integration/owcmp_cli_contract.rs`
- `tests/integration/owcmp_agent_config_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260609-owcmp04-context-efficiency-suite-manifests-001/**`

## Plan of Work

First, add a small manifest helper module and route it through the existing
`tools/owcmp/owcmp` dispatcher. Second, encode the three user-named validation
cohorts as declarative suite manifests that can be listed and preflighted
without reading historical chat. Third, document the low-context comparison
workflow in `tools/owcmp`, `.codex`, and a reusable prompt-template snippet.
Fourth, add focused contract tests for the new CLI/config surface. Finally, run
focused gates, anti-evasion guards for external-authority suite posture,
manifest preflights for all three real cohorts, reviews, verification, and
disposition.

## Concrete Steps

Run commands from `/workdir/openWEPP`.

1. Exercise the package-local Python and Rust contract gates:

       .venv/bin/python -m py_compile tools/owcmp/owcmp tools/owcmp/*.py
       cargo fmt --check
       cargo test --test owcmp_cli_contract
       cargo test --test owcmp_agent_config_contract
       git diff --check

2. Exercise the external-authority suite posture gates:

       bash tools/release/check_authority_suite_antievasion.sh
       cargo test --test auth11_required_suite_obligation_guards_contract

3. Exercise the manifest discovery and real cohort preflight:

       tools/owcmp/owcmp manifest list --json
       tools/owcmp/owcmp env --manifest tools/owcmp/suites/n-idaho-single-ofe-ksflag0.json --json
       tools/owcmp/owcmp env --manifest tools/owcmp/suites/minnesota-corn-ksflag1.json --json
       tools/owcmp/owcmp env --manifest tools/owcmp/suites/wa-cascades-mofe-ksflag0.json --json

4. Remove generated Python bytecode before final status:

       rm -rf tools/owcmp/__pycache__

## Validation and Acceptance

Acceptance requires:

- `owcmp manifest list --json` returns the three suite IDs.
- `owcmp env --manifest` passes for all three suite manifests on this host.
- `cohort-inventory` manifests fail closed if executed with
  `owcmp manifest run`.
- Focused CLI/config tests pass.
- External-authority anti-evasion gates pass.
- Review, finding disposition, verification, line-count governance, worker
  handoff, and final disposition artifacts are complete.

## Idempotence and Recovery

The manifest/env commands are read-only. They do not write under `/wc1`. The
tests create temporary fixtures under the system temp directory and remove them.
The only generated local noise expected from Python execution is
`tools/owcmp/__pycache__`, which is safe to remove. If a `/wc1` run root is not
mounted, `owcmp env --manifest` fails closed and reports the missing path rather
than substituting fixtures.

## Security and Kernel Impact

Security impact: none expected. The package adds local CLI preflight and docs; it
does not add network access, secrets, auth changes, or production endpoint
behavior.

Kernel impact: none. No `SC-*` contract, Rust production kernel, runtime
publication path, formula, threshold, or parameterization default changed.

