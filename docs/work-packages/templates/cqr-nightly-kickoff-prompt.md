# CQR Nightly Kickoff Prompt Template

Copy this template to:

`docs/work-packages/{{package_id}}/prompts/active/{{date}}-codex-cqr-nightly-{{batch_ordinal}}-{{module_slug}}_prompt.md`

Then replace every `{{placeholder}}` before execution.

Scope: local repository code-quality refactor task; flat-file reads/edits only
inside `/home/workdir/openWEPP`; no external connectivity required.

Execution mode: package-end-to-end.

Phase plan: execute all phases in
`docs/work-packages/{{package_id}}/package.md` sequentially through
disposition.

Required reading:

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/{{package_id}}/package.md`
- `docs/work-packages/cqr-nightly-burndown-execplan.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`
- `docs/standards/code-quality-refactor-authoring-guide.md`
- `docs/decisions/0021-module-coverage-closure-thresholds.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/{{package_id}}/artifacts/required-reading-map.md`

Conditional:

- `docs/specifications/science-contracts/AGENTS.md` and the nearest relevant
  `SC-*` contract only if the target touches contract authority,
  conservation-sensitive outputs, or contract-derived tests.
- `docs/standards/testing-and-gate-strategy.md` for intent/diff reconciliation,
  lifecycle assignment, and escalation.
- `docs/standards/local-ci-gate-selection.md` for focused edit-loop feedback.

On-demand:

- `{{target_module_path}}`
- `{{focused_tests_or_contracts}}`
- adjacent modules imported by `{{target_module_path}}`

Required-reading budget: `{{local_bytes_total}}`,
`{{OK_WARN_OR_REQUIRES_JUSTIFICATION}}`; map:
`artifacts/required-reading-map.md`.

Files:

- `{{target_module_path}}`
- `{{test_paths}}`
- `docs/work-packages/{{package_id}}/**`

Task: execute the CQR nightly package for `{{target_module_path}}` end-to-end.
Reduce every eligible production function above CRAP `30` to `<= 30`, or record
an ADR-0021-style disposition and hold when behavior-preserving CQR cannot close
the target safely.

Quality evidence: consume the package-retained `CURRENT`
`quality-evidence-intake.json` bound to `{{quality_evidence_id}}`. Do not launch
coverage recollection from a locator or missing report. Recollection requires
both a retained typed `STALE`/`INVALID` intake receipt and the canonical
authorization bound to this explicit operator directive.

Eligibility gate: before implementation, preserve the raw CRAP rows and write
`artifacts/eligibility-classification.md`. Classify each row at exact
file/function/line granularity as `E-SCIENCE`, `E-PRODUCTION`, an `R-*`
retained exception, or an `X-*` denominator exclusion under ADR-0021. Default
to eligible. Do not use module names, file roles, wildcards, prior disposition,
or “hard to test” as exclusion authority. Both reviewers must accept each
`R-OBSERVABILITY`, `R-IRREDUCIBLE-CRAP`, or `X-*` row before it leaves the
actionable set. `R-INFRASTRUCTURE` may waive only the 75% floor, not CRAP above
30.

Constraints:

- Behavior-preserving CQR only.
- No science-formula, threshold, tolerance, contract-authority, serialization,
  fail-closed, or public-output semantic changes.
- Preserve floating-point statement order, expression grouping, accumulation
  order, and short-circuit behavior.
- No opportunistic cleanup outside the declared target.
- Parser grammar/cardinality, guards, error precedence, state/order/key logic,
  numerical boundaries, serialization/publication, and consumer handoffs are
  always eligible when hand-authored.
- Do not edit unrelated dirty files or active-package work.
- Package-required validation overrides ambient test-skip guidance.

Coverage closure: if characterization tests are added or materially changed,
record ADR-0021 tier assignment, line/region threshold status, per-function
region-floor disposition, and obligation-to-test binding in
`artifacts/coverage-closure.md` before decomposition closes.

Terminal CRAP closure: remeasure the owned target surface and require an empty
owned actionable set. Preserve unrelated workspace rows as observational debt.
Do not use raw `--fail-above`, an inline exception, or a package-local wildcard
as the adjudicated result.

Hold handling: local target holds roll back only current-package
production/test implementation edits to the scaffold baseline, preserve and
commit package hold evidence, and may continue to the next selected module.
Global/process holds stop the nightly batch. Do not revert unrelated user
changes.

Subagent requirement: REQUIRED: spawn `comparator_suite_runner` for all heavy
requirements selected from the declared objective, exact diff, and canonical
strategy, critical classification,
campaign/release boundary, or explicit conservative rollback, including
full-profile Nextest, explicit target metric measurement, comparator suites,
and population/fixture batches. Do NOT run those heavy gates on the
parent model unless the subagent is unavailable; if unavailable, record
command-level evidence before running locally. This prompt explicitly authorizes
subagent spawning/delegation to comparator/closure-runner, review, and
verification subagents for CQR metric checks, selected gate execution,
behavior-identity verification, review, and verification. Outputs:
`artifacts/review_agent_a.md`, `artifacts/review_agent_b.md`,
`artifacts/verification_agent_a.md`, `artifacts/verification_agent_b.md`, compact
metrics, command logs, and artifact paths. Write access: read-only unless a
subagent is explicitly assigned a bounded implementation fix in
`{{target_module_path}}` or package-local artifacts.

Autonomy: execute package phases end-to-end and update required artifacts without
requesting additional user direction unless hard-blocked.

Outputs: update all package artifacts, disposition every review finding, and
leave the package ready for its completion or hold commit.
