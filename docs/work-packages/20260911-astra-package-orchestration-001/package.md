# Astra work-package orchestration

Status: COMPLETE. Source base: `dafe138fe7022d59ba8ff7d685308eb434bba486`.

## Objective and scope

Owner direction: Astra orchestrates work-packages efficiently, delegates detailed
execution, and maintains oversight. Codex Astra or ChatGPT Pro science/technical
reviews between packages set direction for subsequent work.

Change active execution/role guidance, matching templates and local model defaults.
Preserve scientific authority, independent acceptance reviews, validation strength,
explicit owner bounds, historical records and existing commit/push permissions.
The dirty B01 record, candidate patch and log are pre-existing and excluded.
No B01 recovery, simulation, production Rust or historical migration is included.

## Execution and acceptance

Deliverable: coherent prospective governance and configuration implementing that
division of responsibility, with source custody owned by the orchestrator.
Selected checks: terminal diff/whitespace, TOML parsing and role-file resolution,
changed-document link checks, targeted stale-policy search and two independent
reviews (correctness/authority and QA/evidence). This changes agent execution
authority, not numerical code or test-selection policy; runtime correctness
campaigns are not applicable. Authentic-run allowance: zero.

## Current state and evidence

Implemented Astra/medium as the configured primary and an explicit Terra/medium
implementer role. The common guide and orchestration procedure assign detailed
execution to implementers and integration, independent reviews, custody and
disposition to Astra. The direction/adviser role and templates now operate at
package boundaries. Generic 2-cycle/60-minute boundaries are internal reassessment;
explicit existing owner stops and consumption remain hard limits. Source retention
requires durable storage and verified recovery before unique-source cleanup.

Ran from `/workdir/openWEPP`: `git diff --check` PASS. Inline
`.venv/bin/python` checks with stdlib `tomllib` parsed the primary config and all
8 role files, resolved every `config_file`, and asserted Astra/medium primary and
Terra/medium implementer: PASS. An inline stdlib `pathlib`/`re` scan of changed/new
governance Markdown resolved 58 local references across 13 documents: PASS. Search of
active guidance/config/templates for old primary routing, generic hard-stop and
token-pilot instructions found one stale implementer phrase; GOV-COR-001 corrected
it. No Rust/runtime validation was run; no executable kernel, dependency, test
selection, scientific authority or protected runtime data path changed.

OpenAI Docs was used for configuration guidance: the
[configuration reference](https://learn.chatgpt.com/docs/config-file/config-reference)
and [Astra migration guidance](https://developers.openai.com/api/docs/guides/latest-model/gpt-6-astra)
support the model field/role configuration and retaining the existing reasoning
effort. Fresh-session loading and actual host enforcement are NOT RUN/UNOBSERVED;
editing config does not switch a running session. No efficiency improvement is
claimed before a package executes under the new arrangement.

Terminal scope is 16 governance/configuration/record files: root and common guide,
three standards, three role procedures, two templates, config and two agent roles,
this record, catalog and locator. B01 remains excluded; its retained candidate and
log hashes still match `1b7f8a7b48f02d0aa117969081295325e7af0e11de0d914af6306c739df338bb`
and `3aa57f914935585123d35722a1206838e6cf0d69245818340d9c0402ac137fa2`.

## Review and corrections

Both reviewers are independent of the author and each other, with read-only
assignments covering the 16 scoped files at this source base plus the working diff.
Their reviews do not include B01. No additional verifier wave was used.

Correctness/authority: `/root/governance_correctness`, configured
`rust_code_reviewer` (Sol/high). Initial finding GOV-COR-001 (medium, blocking):
the implementer procedure retained "active checkpoint bounds," which could
recreate the superseded automatic handoff. Accepted and changed to "assigned
package/deliverable envelope and explicit owner limits," retaining the assigned
management return boundary. Same-reviewer fix verification passed. Its independent
TOML, links and whitespace checks passed; it found the two-review classification
and omission of Rust/runtime campaigns appropriate.

Original final correctness verdict after same-reviewer fix verification:

> Static: `GOV-COR-001` is resolved. [role-implementation.md](/workdir/openWEPP/docs/work-packages/role-implementation.md:16) now binds execution to the assigned package/deliverable envelope and explicit owner limits, while retaining the internal assigned return boundary.
>
> Ran: targeted stale-policy search and scoped `git diff --check` passed.
>
> Final correctness verdict: no blocking findings remain. Approved within the assigned correctness/authority scope.

The first follow-up stalled; Astra interrupted it and returned only the affected
lines to the same reviewer. No replacement reviewer or expanded review was used.

QA/evidence: `/root/governance_qa`, configured `rust_qa_reviewer` (Terra/medium).
Independently ran whitespace, TOML/role resolution and scoped link checks: PASS.
The initially missing locator entry was resolved and verified in that review.
Original terminal scoped verdict:

> QA pass: No remaining QA/maintainability, role-configuration, stale-active-policy, custody/recoverability, or reference-integrity blocker in the final scoped cut. The orchestration/implementer/adviser division is coherent and preserves explicit owner limits, independent reviews, and remote-handoff responsibility.

Same QA review's affected-fix verification:

> Static: GOV-COR-001 correction verified.
>
> `docs/work-packages/role-implementation.md:15-19` now correctly binds implementation to the assigned package/deliverable envelope and explicit owner limits, while retaining the assigned return boundary. This aligns with `bounded-agent-execution.md:18-49` and the common guide’s internal-checkpoint rule.
>
> No remaining QA consistency finding; scoped `git diff --check` passes.

## Disposition

COMPLETE: governance/configuration deliverable and both independent reviews pass.
The one blocking wording defect is fixed and verified by its reviewer. Terminal
diff/whitespace, TOML/role resolution, local references and stale-policy checks pass.
Preserve this 16-file change in a scoped local commit; no remote push is part of
this execution. Existing B01 modifications and evidence are outside that commit.
Fresh-session routing and delivered-package efficiency remain unmeasured. B01's
explicit stops remain unchanged; this package does not authorize its recovery or
execution. No scientific, production or performance acceptance follows.
