# Lightweight work-package governance

Status: COMPLETE. Base: 47e9239ea. Owner: repository owner; executor: Codex.

## Objective and scope

The owner authorized scaffolding and executing the full structural replacement
on 2026-09-09, retaining work-packages and their scope/cadence. Each package has
one maintained record, package.md. Reduce repeated onboarding, assurance passes,
and administrative artifacts; preserve science and consequential correctness.
This explicit replacement applies to this package at intake: one record and two
independent reviewers who verify their own assigned fixes, without fresh terminal
verifiers. Historical packages and failed results are not rewritten.

Scope: root/CLAUDE and task-owning AGENTS; common package roles and specialized
procedures; execution/preparation/prompt/mechanical/validation standards; active
templates and routing documentation; narrowly coupled administrative tests or
live policy bindings if required for consistency. No production Rust, canonical
SC equations, solver implementation, fixture cohorts, historical evidence,
branches, push, or deployment. Package scopes are not merged or widened for cost.

## Authority and acceptance

User direction supersedes prior universal artifact and four-assignment rules.
Canonical science, typed guards, A0/A1/A3, independent conservation reconstruction,
real consumers, anti-evasion, and critical/release correctness remain binding.
Sources: docs/work-packages/AGENTS.md; docs/standards/AGENTS.md;
docs/standards/testing-and-gate-strategy.md; applicable instruction chains found
with tools/agents/find-agents. Reading selection changes no scientific authority.

Acceptance: common rules and prospective entrypoints consistently require one
record; editorial work has no mandatory independent review, bounded work one,
critical work two with fix verification; no universal fresh verifiers or runner
delegation. Scientific readings follow affected obligations even in single-file
contracts. No mandatory routine reading metrics, identity triplets, placeholder
artifacts, prompt archives, or file-length-triggered unrelated refactors.
Prospective bounded inherited-lint policy distinguishes usable results from
integration/release readiness. Evidence identity, truthful failures and legitimate
acceptance remain. Historical requirements/results remain interpretable.

## Plan and validation

1. Scaffold this record; replace common policy and align active procedures/routes.
2. Inspect exact diff, conflicting prospective wording and references. Run diff
   hygiene, affected documentation/reference checks, and relevant existing
   administrative contract tests. Reconcile only affected administrative bindings
   if checks expose stale expectations; preserve scientific/security assertions.
3. Obtain independent correctness/authority and QA/usability reviews; correct
   material findings and ask the same reviewers to verify affected fixes.
4. Reconcile final diff and update this record and current locator. No historical
   package migration campaign or new governance executable/telemetry framework.

This is consequential governance with two independent perspectives, but no runtime
behavior change: full numerical regression is not selected solely for prose edits.
Any executable/test-coverage impact discovered is assessed before editing it.
This package explicitly authorizes subagent spawning/delegation for two independent
reviewers, each read-only except their assigned Review A/B section below. They
inspect primary diff/evidence and report concise findings; executor owns corrections.

## Current state and evidence

Ran: initial git status clean; coordinated replacement implemented across common
roles, preparation/validation/science procedures, mechanical/CQR templates and
current routes. Both independent reviews and their accepted-fix checks PASS.
No production/test Rust edits. Final diff hygiene passes.

Validation environment: cargo is absent from the default shell (exit 127);
the existing offline Nix development shell supplies it. Initial focused run:
`nix develop --offline --command cargo nextest run --test advisory_linter_authority_contract
--test cqr_quality_evidence_handoff_contract --test solver_architecture_authority_contract
--test adr0017_comparator_distrust_ratification_contract` (cwd /workdir/openWEPP)
exited 100: 9 passed, 2 failed, 9 not run due to fail-fast. Log:
/tmp/openwepp-lightweight-governance-JJOPZK/contracts-initial-nix.log.
Failures: retained comparator protection was worded differently from a prose
assertion (explicit wording restored); live policy hash needed rebinding after
the authorized strategy change. Rebound only tools/release/authority-policy/impact-map.json
policy_sha256; all suite/required-case entries and
historical hashes stay unchanged.

Ran: corrected command with --no-fail-fast ran all 20 tests in 362.588 seconds:
19 PASS, including the CQR evidence self-test, and one FAIL for the inherited
missing roadmap ADR-0044/anti-accretion reference. Log:
/tmp/openwepp-lightweight-governance-JJOPZK/contracts-corrected.log.
Restored the docs/ROADMAP.md standing authority reference only, preserving all
queue/activation states. Ran `nix develop --offline --command cargo nextest run
--no-fail-fast --test solver_architecture_authority_contract` in /workdir/openWEPP:
exit 0, 5/5 PASS in 0.451 seconds; log:
/tmp/openwepp-lightweight-governance-JJOPZK/solver-final.log.
The prior 15 passes in the other three targets remain current: the roadmap-only
correction is outside their source/input dependencies. All 20 selected tests now
have passing current evidence; the failed attempts remain visible above.
No test assertion or protected behavior changed. Full numerical regression was
not selected or run; this package changes administrative prose and one live hash.

Ran: added local Markdown link checks and concrete repository reference checks,
no missing targets; final git diff --check PASS. Scoped searches found no remaining
generic mandatory handoff/reading-map/fresh-verifier or size-threshold closure
directives in the active owning routes. Source inspection confirms the CQR
validator and its specialized committed input schema are unchanged.
Actual token/quota savings are unmeasured; 5x is an objective, not acceptance
evidence inferred from document size. No extra measurement tooling is planned.

## Review A — authority and correctness

Reviewer: /root/governance_correctness, independent read-only review of base
47e9239ea plus assigned working diff. A1 found stale fixed-section/universal-dual
review wording in the DC guide. Executor corrected the opening and conversion/
kickoff wording without changing DC scientific acceptance or HOLD boundaries.

Final reviewer response:

> Static: Review A fix verification by `/root/governance_correctness` confirms A1 closed. The DC opening and conversion/kickoff wording now consistently apply flexible `package.md` sections and consequence-based independent review, preserving scientific acceptance and legitimate HOLD requirements. The restored comparator nonwaiver sentence preserves existing authority.
>
> Ran: scoped `git diff --check` passed. No runtime tests performed.
>
> Verdict: PASS for assigned scope; no unresolved Review A findings.

## Review B — QA and execution usability

Reviewer: /root/governance_qa, independent read-only review of base 47e9239ea plus
assigned working diff and actual CQR validator fields. B1-B3 found stale file-size
requirements, a separate MTE coverage ledger and conflicting kickoff rules.
Executor aligned the Rust/CQR/MTE standards and standards AGENTS, preserving
all exclusion evidence fields and independent exclusion adjudication.

Final reviewer response:

> Static: Review B fix verification by `/root/governance_qa`; inspected the current corrections to B1–B3 and the additional work-package/science-contract README routing changes. Ran: read-only Git diff inspection; no runtime tests.
>
> B1 resolved: the Rust standard treats file length as a maintenance signal while preserving explicitly owned size targets; CQR acceptance no longer requires line-count disposition.
>
> B2 resolved: MTE classification entries now reside in `package.md`; all six evidence requirements, including both independent exclusion-review dispositions, remain intact.
>
> B3 resolved: standards instructions permit reference-only kickoffs and route review requirements through the common guide.
>
> The README changes remove obsolete generic ceremony and align disposition routing with the single record. The inspected diff preserves historical package entries and canonical contract promotion requirements.
>
> Verdict: PASS for the assigned QA/usability scope and accepted fixes. No outstanding Review B findings. This verdict does not claim runtime/admin-test execution or replace the executor’s final validation evidence.

## Disposition

COMPLETE: one maintained record per package, zero/one/two consequence-based
independent reviews with reviewer-owned fix verification, affected-authority
reading, proportional evidence, optional reading metrics and no mandatory
file-length refactoring. Scientific authority, critical correctness, independent
conservation, real-consumer and historical-failure requirements are preserved.

Final scope: 38 files (36 tracked modifications, this record and the generic
package template). The only non-Markdown change is the live policy hash; no
historical package, production source, test assertion, fixture, Cargo or Nextest
configuration changed. Reviews remain valid after final record updates and the
bounded roadmap reference restoration; no scientific claim or acceptance drift.
The owner subsequently authorized committing and pushing this completed change
on the current branch. No branch change or deployment is included.
Next action: use the new guide/template for subsequent authorized work-packages;
no further work is required to complete this replacement.
