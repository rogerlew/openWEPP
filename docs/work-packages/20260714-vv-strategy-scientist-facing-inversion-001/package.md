# Scientist-Facing V&V Strategy Inversion

Status: `COMPLETE`

Date: `2026-07-14`

Execution mode: `package-end-to-end`

## Objective

Refactor openWEPP's verification and validation strategy so its primary outcome
is transparent, useful scientific evidence for hydrologists, soil scientists,
researchers, and practitioners. Establish a concise companion standard for
public scientific assurance dossiers, and make generalized evidence tooling a
later response to demonstrated campaign needs rather than a prerequisite for
communicating current evidence and gaps.

## Rationale

The initial strategy correctly distinguished verification, validation,
comparative evidence, uncertainty, and use qualification. It nevertheless
combined philosophy, scientific program design, and a future software subsystem
in one long governance document. That ordering emphasized schemas, provenance
machinery, and automation before the audience-facing evidence products that
motivated the strategy.

User direction on 2026-07-14 authorizes this revision after accepting the
scientist-facing inversion: intended use, scientific questions, observational
evidence, limitations, and human communication must lead; infrastructure must
serve those products.

## Included Scope

- Rewrite the active V&V strategy around bounded scientific claims and public
  evidence dossiers.
- Preserve the essential separation of verification, validation, comparative
  evidence, uncertainty, and use qualification.
- Place an honest current-state baseline and named existing SNOTEL example
  before future subsystem architecture.
- Invert the adoption roadmap so public baseline dossiers and scientific
  validation plans precede generalized tooling.
- Add a concise standard defining the content, layers, status language,
  visuals, limitations, and minimal audit kernel of a scientific assurance
  dossier.
- Update documentation indexes and complete documentation validation, dual
  independent review, finding disposition, and final package disposition.

## Excluded Scope

- Selecting or admitting new observational datasets.
- Executing new verification or empirical-validation campaigns.
- Claiming that openWEPP or any public quantity is newly validated.
- Implementing a V&V crate, schema, evidence graph, freshness engine, CLI, or
  report generator.
- Changing science contracts, process physics, tests, runtime code, release
  gates, or existing evidence verdicts.

## Deliverables

- `docs/governance/openwepp-verification-validation-strategy.md`
- `docs/standards/scientific-assurance-dossier.md`
- Updated governance, standards, documentation, and work-package indexes.
- Package-local reading map, gate evidence, two independent reviews, finding
  disposition, worker handoff, and final disposition.

## Dependencies

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/governance/openwepp-verification-validation-strategy.md`
- `docs/decisions/0028-observed-data-admission-authority.md`
- `docs/specifications/correctness-authority-model.md`

## Intended Write Set

- `docs/governance/openwepp-verification-validation-strategy.md`
- `docs/governance/README.md`
- `docs/standards/scientific-assurance-dossier.md`
- `docs/standards/README.md`
- `docs/README.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260714-vv-strategy-scientist-facing-inversion-001/**`

## Subagent Authorization And Requirement

Subagent authorization: this package explicitly authorizes spawning/delegating
to two independent reviewer/verifier subagents for scientist-audience fit,
scientific V&V integrity, governance proportionality, cross-reference
integrity, and closure verification. Expected outputs are package-local
`artifacts/review-a.md`, `artifacts/review-b.md`, `artifacts/verification-a.md`,
and `artifacts/verification-b.md`. Each reviewer has read-only access outside
its single bounded artifact path and must not edit the strategy, standard, or
the other reviewer's artifact.

Subagent requirement: dual independent review and verification are required.
No heavy batch, comparator, release, or Rust closure execution is in scope.

## Phase Plan

1. Freeze the audience, authority, package scope, and required-reading map.
2. Rewrite the strategy and author the dossier standard.
3. Update navigation and run spelling, Markdown, link/path, and diff checks.
4. Obtain two independent reviews, disposition every finding, and remediate all
   accepted findings.
5. Reverify affected surfaces and record the final package disposition.

## Exit Criteria

| ID | Criterion |
| --- | --- |
| `VVINV-001` | The strategy names scientific users and the questions its transparency program must answer. |
| `VVINV-002` | Public baseline dossiers are the first adoption deliverable, including honest `NOT_ASSESSED` and insufficient-evidence states. |
| `VVINV-003` | Verification remains required for interpreting validation evidence but is presented as a visible evidence dimension rather than the audience-facing center of the report. |
| `VVINV-004` | The strategy identifies current strengths, gaps, and the existing SNOTEL observed-data example without overstating empirical support. |
| `VVINV-005` | Scientific plans and campaigns precede generalized evidence infrastructure in the roadmap. |
| `VVINV-006` | The dossier standard supplies a practical human-first structure, scientific visuals, status language, limitations, and a minimal reproducibility/audit kernel. |
| `VVINV-007` | Planned schemas, crates, query engines, and provenance standards are not active prerequisites for publishing evidence or gaps. |
| `VVINV-008` | Documentation indexes, spelling normalization, Markdown lint, and changed-path integrity checks pass. |
| `VVINV-009` | Two independent reviews and verifications are complete, and every finding is dispositioned with accepted findings fixed. |
| `VVINV-010` | The package truthfully records documentation-only scope, no new scientific verdict, no runtime/security impact, and no touched `.rs` files. |

Statuses are `PASS`, `FAIL`, `BLOCKED`, or `NOT RUN`. Any state other than
`PASS` blocks `COMPLETE` disposition.

## Security Impact

Security impact is low. This package edits tracked Markdown only, performs no
external publication, handles no secrets or restricted datasets, and changes
no executable behavior. Review must confirm that reproduction guidance does
not embed credentials, private locations, or unsupported data-access claims.

## Truthfulness

Evidence artifacts label read/reasoned evidence as `Static:` and command
evidence as `Ran:`. Documentation claims must distinguish existing repository
evidence from future plans. This package cannot strengthen any scientific
acceptance verdict.

## Progress

- [x] (2026-07-14) User authorized the scientist-facing inversion.
- [x] (2026-07-14) Applicable root, standards, and work-package instructions
  identified before substantive edits.
- [x] (2026-07-14) Rewrote the strategy and authored the dossier standard.
- [x] (2026-07-14) Updated indexes; initial spelling, Markdown, path, and diff
  gates pass.
- [x] (2026-07-14) Completed two independent initial reviews; accepted and
  remediated all findings while retaining `Pending Review` status.
- [x] (2026-07-14) Both reviewers verified accepted fixes; synchronized all
  three activation surfaces and received dual terminal activation confirmation.
- [x] (2026-07-14) Terminal 20-file Markdown, spelling, path, diff, status,
  scope, and line-count gates pass; package disposition is complete.

## Surprises And Discoveries

- The repository already contains a concrete observed-data example in ADR-0028:
  SNOTEL SWE, snow depth, and density across five climates. The initial strategy
  referenced that ADR but did not surface the example for scientific readers.
- The preferred `wctl doc-lint` wrapper resolves into the neighboring wepppy
  environment and cannot start here because `typer` is missing. The installed
  underlying `markdown-doc lint` command works directly and is the recorded
  validation surface for this package.

## Decision Log

- Decision: Keep one active philosophy/strategy document and add one concise
  dossier standard; do not create a speculative subsystem specification.
  Rationale: This makes the human product operational now while allowing real
  campaigns to discover stable automation requirements.
  Date/author: 2026-07-14, Codex.
- Decision: Reduce the strategy itself from `4930` to `2270` words and move
  dossier mechanics into the standard.
  Rationale: A nominal file split would not solve the audience problem if the
  governing strategy remained a near-500-line procedural document.
  Date/author: 2026-07-14, Codex.
- Decision: Require a manually authorable content-identity manifest from the
  first dossier while continuing to defer a generalized evidence platform.
  Rationale: Auditability requires immutable binding of claim-bearing evidence;
  it does not require a crate, database, schema service, or provenance export.
  Date/author: 2026-07-14, Codex.

## Outcomes And Retrospective

The strategy now makes scientist-facing evidence the primary product and cuts
the governing document from `4930` to `2315` words. The companion standard
provides an immediately usable dossier structure without requiring a V&V crate,
database, schema service, or evidence graph. Independent review strengthened the
minimal audit kernel: every dossier begins `NOT_ASSESSED`, every baseline is
reviewed, historical positive claims cannot silently expand, and a lightweight
manifest content-binds claim-bearing evidence. No scientific verdict or runtime
surface changed.
