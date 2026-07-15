# ASSURE-02 Manuscript-First Scientific Assurance Architecture

Package ID: `20260714-assure02-manuscript-first-assurance-architecture-001`

Status: `EXECUTED-HOLD-USER-ACCEPTANCE`

Execution date: 2026-07-14

## Objective

Freeze a documentation-first v2 scientific-assurance architecture whose public
product reads as a conventional model-evaluation manuscript for hydrologists,
soil scientists, researchers, and practitioners. Demonstrate the architecture
with a hand-authored, nonpublic report prototype using retained evidence for a
bounded non-snow process kernel. Do not implement a v2 compiler or publish a v2
report in this package.

## Authority And Motivation

The binding closure contract is `docs/ROADMAP.md#assure-02-closure-contract`.
The v1 SNOTEL vertical slice proved deterministic engineering machinery but
failed the scientific communication objective: public lifecycle grades led the
page, the scientific argument was sparse, and a candidate could appear in the
public tree before scientific acceptance. ASSURE-02 replaces that publication
architecture without diminishing the underlying snow/frost model or its
evidence.

Verification and empirical corroboration remain asymmetric. Verification can
hard-gate a specified software or numerical claim. Environmental-model
validation remains claim-, quantity-, scale-, domain-, data-, and decision-
context specific; it is communicated as evidence and limitations rather than a
terminal universal fitness verdict.

## Package Class And Exemptions

This is a documentation-only architecture package. It changes no Rust source,
science-contract authority, v2 assurance compiler, generated/public assurance
content, or WEPPcloud vendor surface. Rust closure gates and CRAP measurement
are therefore `N/A`; scoped documentation, link, evidence, review, and
verification gates are required.

## Required Outcomes

1. An ADR and revised V&V strategy establish a manuscript-first public product
   and the report/supplement/machine/application boundary.
2. A v2 report standard defines a recognizable scientific structure, evidence
   rules, quantitative result and limitation expectations, accessibility, and
   claim-specific conclusions.
3. Lifecycle and source/build contracts define ownership, canonical source,
   staging-only drafts/candidates, review and approval, dependencies,
   deterministic builds, snapshots, supersession, and release transfer.
4. A recorded inventory selects one bounded non-snow pilot on evidence, not
   convenience or predetermined preference.
5. A hand-authored, nonpublic manuscript prototype uses real retained evidence,
   quantitative results, at least one useful table or figure, a reproducible
   method, limitations, and discussion.
6. A migration plan retires the v1 publication architecture while preserving
   its exact engineering and scientific provenance and supporting zero public
   reports.
7. A prospective implementation roadmap decomposes later work into independently
   closable packages with explicit consumers, gates, migration boundaries, and
   rollback.
8. Two independent reviews are dispositioned and independently verified.
   Final scientific-direction acceptance remains a user or named scientific-
   steward decision and is not self-issued by an agent.

## Pilot Decision Boundary

The linear groundwater-reservoir recurrence is the preferred candidate because
it appears to have published formulation authority, an analytical recurrence,
contract-derived vectors, domain guards, a real downstream consumer, and
run-level conservation evidence. It is selected only if the inventory confirms
those properties. The prototype may support formulation, code, numerical,
integration, and release-transfer verification claims justified by evidence;
it must not manufacture field validation or general watershed fitness claims.
Snow/frost is excluded from the pilot and remains the later flagship synthesis.

## Declared Write Set

- `docs/decisions/0038-manuscript-first-scientific-assurance-publication.md`
- `docs/decisions/README.md`
- `docs/governance/openwepp-verification-validation-strategy.md`
- `docs/governance/scientific-assurance-dossier-lifecycle.md`
- `docs/governance/scientific-assurance-v2-architecture.md`
- `docs/governance/scientific-assurance-v2-source-build-contract.md`
- `docs/governance/openwepp-release-procedure-draft.md`
- `docs/governance/README.md`
- `docs/standards/scientific-model-evaluation-report.md`
- `docs/standards/scientific-assurance-dossier.md`
- `docs/standards/usersum-authoring-style-guide.md`
- `docs/standards/README.md`
- `docs/planning/scientific-assurance-v2-migration-plan.md`
- `docs/planning/scientific-assurance-v2-implementation-roadmap.md`
- `docs/ROADMAP.md`
- `docs/README.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260714-assure02-manuscript-first-assurance-architecture-001/**`

Read-only evidence may be taken from science contracts, Rust implementation and
tests, prior work packages, local references, and retained run artifacts. Any
needed write outside this set requires explicit package amendment before edit.

## Explicit Exclusions

- No edits under `assurance/` or `usersum/assurance/`.
- No v2 compiler, schema, renderer, CLI, generated report, public report, or
  release snapshot implementation.
- No removal of the v1 candidate; ASSURE-03 owns that migration.
- No snow/frost scientific reassessment and no weakening of existing snow/frost
  evidence or claims.
- No empirical validation claim for the groundwater pilot unless admitted
  observations, methods, and results actually support that claim.
- No application fitness verdict and no WEPPcloud vendoring.

## Execution Plan

### Phase 1 — Intake And Scientific Reporting Research

1. Record applicable instructions and current authority.
2. Research primary or authoritative environmental-model evaluation,
   reproducibility, provenance, and scientific-reporting guidance.
3. Extract audience-facing requirements without importing a licensing-shaped
   universal validation verdict.

### Phase 2 — Evidence-Led Pilot Selection

1. Compare bounded non-snow candidates on scientific authority, independent
   referent, retained evidence, production relevance, confounding, and
   manuscript usefulness.
2. Select or reject the groundwater recurrence in a durable decision record.
3. Build a claim-evidence-method-result inventory for the selected pilot.

### Phase 3 — V2 Documentation Architecture

1. Author the ADR, architecture, V&V strategy, report standard, lifecycle, and
   source/build/dependency contracts as one coherent authority set.
2. Make the public report conventional and prose-led; keep the technical
   supplement subordinate and the machine bundle internal.
3. Define human ownership and approval separately from mechanical build and
   agent assistance.

### Phase 4 — Nonpublic Prototype And Migration Roadmap

1. Hand-author the prototype from real retained evidence before defining any
   future implementation schema.
2. Record exact evidence identities, equations, values, method, limitations,
   discussion, and reproduction path.
3. Author the v1 retirement plan and independently closable ASSURE-03 through
   ASSURE-05 implementation packages.

### Phase 5 — Review, Disposition, And Verification

1. Run scoped documentation, reference, cross-link, evidence-identity, and diff
   gates.
2. Dispatch two independent read-only reviewers:
   - Reviewer A: scientific communication, audience fit, manuscript integrity,
     claim discipline, and pilot credibility.
   - Reviewer B: architecture, ownership, reproducibility, dependency/build,
     migration, staging/publication, and rollback integrity.
3. Disposition every finding in a package artifact and amend accepted findings.
4. Dispatch independent verification A and B against the amended exact tree.
5. Record `EXECUTED-HOLD-USER-ACCEPTANCE` if all agent-executable gates pass but
   the required user/scientific-steward direction acceptance has not yet been
   explicitly issued. Gate Evidence Non-Deferral prohibits calling that hold
   complete.

## Subagent Authorization

The user instruction to scaffold and execute ASSURE-02 plus the repository's
standing work-package review rules explicitly authorize subagent delegation for
the two independent reviews and two independent verifications in this package.
Reviewers are read-only and return findings to the parent agent; the parent owns
all edits and the disposition. Coding-agent review must be labeled as such and
must not be represented as external scientific peer review.

## Gates

- Package scaffold and declared write set exist before substantive edits.
- Required-reading and research artifacts identify the actual sources used.
- Pilot selection is evidence-led and records rejected alternatives.
- Every quantitative statement in the prototype has units and a retained or
  reproducible source identity.
- Prototype conclusions do not exceed their formulation, code, numerical,
  integration, release, empirical, or application evidence.
- Draft/candidate publication to public `usersum` is prohibited by all v2
  authority documents.
- V1 retirement preserves provenance and permits zero published reports.
- `markdown-doc lint` and `markdown-doc validate` pass on the changed docs.
- `git diff --check` passes.
- Documentation link/reference checks pass or are dispositioned with exact
  scope.
- Review A and B findings are fully dispositioned.
- Verification A and B pass on the final tree or the package remains held.
- Rust/CRAP/line-count gates: `N/A` because no `.rs` file is in the write set.

## Closure Criteria

`EXECUTED-COMPLETE` requires every package gate plus explicit user or named
scientific-steward acceptance of the scientific communication direction.
Without that acceptance, the strongest truthful disposition is
`EXECUTED-HOLD-USER-ACCEPTANCE`; ASSURE-03 remains blocked and no v2 code is
authorized.

Agent-executable documentation, evidence, review, remediation, and verification
work passed on 2026-07-15 UTC. The package remains open only for explicit user
or named scientific-steward acceptance. Separately, openWEPP release assembly
remains prohibited on the named executable `ASSURE03-REL-001` conflict that an
accepted ASSURE-03 package must correct first.
