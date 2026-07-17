# Test And Gate Authority Foundation

Package ID: `20260717-test-gate-authority-001`

Status: `EXECUTED-COMPLETE`

Execution date: 2026-07-17

## Objective

Establish one canonical, mechanically specific authority for openWEPP test
selection and gate timing. Adopt campaign-scoped, risk-based execution through
an ADR; define the vocabulary, architecture, dependency and risk mechanics,
gate receipts, coverage/CRAP cadence, assurance currency, CI lanes, and
transition rules; and leave repository/tooling alignment to a separately
authorized implementation package.

## Rationale

Current policy is fragmented. Root and work-package instructions require full
workspace Nextest and fresh full-workspace CRAP for every implementation
package, while the local-CI standard says to match cost to risk. Release-scale
automation also runs on ordinary pull requests and pushes. The result is repeat
execution of unrelated suites and assurance work during active science
campaigns. A single authority must resolve policy before tooling is changed.

## Package Class

This is a documentation-only governance package. It changes no Rust source,
test, fixture, CI workflow, Nextest profile, assurance source, generated output,
or release tool. Rust, full-workspace, coverage, and CRAP gates are `N/A`.
Scoped documentation, references, dual review, finding disposition, and dual
verification are required.

## Declared Write Set

- `docs/decisions/0039-campaign-scoped-risk-based-testing-and-assurance-gates.md`
- `docs/decisions/README.md`
- `docs/standards/testing-and-gate-strategy.md`
- `docs/standards/README.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260717-test-gate-authority-001/**`

Read-only inspection may cover current instructions, standards, decisions,
tests, Nextest configuration, CI workflows, release scripts, assurance
architecture, and retained timing evidence. Writes outside the declared set
require package amendment before editing.

## Explicit Exclusions

- No edits to `AGENTS.md`, nested `AGENTS.md`, ADR-0021, or the correctness
  authority model; the follow-up alignment package owns those changes.
- No gate planner, impact map, receipt schema, CLI, workflow, Nextest, coverage,
  CRAP, or assurance implementation.
- No weakening of science-contract obligations, coverage thresholds, CRAP
  threshold 30, fail-closed behavior, consumer-path proof, conservation proof,
  external-authority requirements, or release qualification.
- No declaration that deferred evidence passed or that an assurance report is
  current for a realization it has not assessed.

## Required Outcomes

1. ADR-0039 adopts one canonical testing-and-gate standard and states exactly
   which earlier frequency rules it supersedes without changing their quality
   thresholds.
2. The standard defines terms, authority boundaries, test families, lifecycle
   levels, deterministic impact selection, risk escalation, gate planning and
   execution, receipts and reuse, coverage/CRAP cadence, assurance currency,
   CI lanes, performance budgets, failure semantics, and anti-evasion rules.
3. The standard distinguishes a valid campaign increment from campaign closure
   and release qualification. Deferred campaign obligations remain visible and
   cannot be waived or represented as passed.
4. The standard defines enough implementation interfaces and machine-readable
   fields for the follow-up package to proceed without reopening policy.
5. A transition inventory names every known policy and automation surface that
   the follow-up implementation must align.
6. Two independent coding-agent reviews are fully dispositioned, followed by
   two independent terminal verifications of the amended exact tree.

## Execution Plan

### Phase 1 — Intake And Scaffold

Record applicable instructions, current authority fragments, retained timing
evidence, and external primary references. Freeze the documentation-only write
set and explicit subagent authorization before substantive drafting.

### Phase 2 — Authority Drafting

Author ADR-0039 and `docs/standards/testing-and-gate-strategy.md` as a coherent
pair. The ADR records the decision and supersession boundary. The standard is
the sole living operational authority. Catalog both documents.

### Phase 3 — Transition Handoff

Record the exact guidance, scripts, workflows, profiles, registries, and
assurance surfaces that remain out of alignment. Define implementation package
boundaries without changing enforcement in this package.

### Phase 4 — Review And Remediation

Dispatch two independent read-only reviewers. Reviewer A evaluates testing
philosophy, scientific correctness protection, lifecycle/risk mechanics, and
whether the policy could miss consequential regressions. Reviewer B evaluates
architecture, deterministic selection, receipts, assurance deferral, CI and
transition feasibility, ambiguity, and anti-evasion. Disposition every finding
as `accepted`, `rejected`, `deferred`, or `follow-up`; fix all accepted findings.

### Phase 5 — Terminal Verification And Disposition

Run scoped documentation and reference checks. Dispatch two independent
read-only verifiers against the amended exact tree. Close only if both verify
the ADR/standard consistency, finding disposition, catalog discovery, package
scope, and implementation handoff.

### Phase 6 — Additional Adversarial Review

At the user's request, dispatch two fresh reviewers after the first closure
candidate. Reviewer C attacks scientific/governance completeness, omission
risk, and lifecycle failure modes. Reviewer D attacks schema determinism,
implementability, identity/reuse safety, and migration/cutover mechanics.
Disposition every new finding and repeat terminal verification if authority
bytes change.

## Subagent Authorization

The user explicitly requested dual agent review and then a second dual-review
round. This package explicitly authorizes subagent spawning/delegation to two
independent first-round coding-agent reviewers, two independent first-round
terminal verifiers, two fresh adversarial round-2 reviewers, and two renewed
round-2 terminal verifiers. All subagents are
read-only except for their one explicitly named package artifact. The parent
owns all authority edits and finding disposition. Coding-agent review is
internal engineering review, not external scientific peer review.

## Gates

- `tools/agents/find-agents --for` records the applicable instruction chain.
- ADR-0039 and the standard agree on status, precedence, supersession, gate
  levels, CRAP cadence, assurance currency, and transition posture.
- Every normative term used for closure is defined.
- All external practice claims have primary or authoritative references.
- Catalog links resolve and changed Markdown passes scoped lint/validation.
- `git diff --check` passes.
- American-English normalization is previewed and only safe prose changes are
  applied.
- Both independent reviews are present and every finding is dispositioned.
- Both terminal verifications pass the amended exact tree.
- Rust, full Nextest, cargo-deny, coverage, CRAP, and `.rs` line-count gates are
  `N/A`: no implementation file is in the write set.
- Security impact: `N/A`; no executable, dependency, credential, workflow, or
  release behavior changes.

## Closure Criteria

`EXECUTED-COMPLETE` requires all documentation gates, dual review,
dispositioned findings, dual terminal verification, and a bounded implementation
handoff. Completion establishes policy authority; it does not claim that current
repository instructions or automation already conform. The follow-up alignment
package remains required before the new mechanics are enforceable.
