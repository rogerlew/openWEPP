# Asymmetric Scientific Assurance Reframe

Status: `COMPLETE`

Date: `2026-07-14`

Execution mode: `package-end-to-end`

## Objective

Reframe openWEPP assurance around three distinct questions and decision owners:
binary acceptance of specified software-verification obligations, nonterminal
empirical corroboration over observed domains, and contextual application
fitness decided by the hydrologist, practitioner, agency, or other responsible
decision owner. Remove the remaining licensing-shaped implication that
openWEPP can issue a terminal scientific fit-for-use verdict for open natural
systems.

## Rationale

The completed scientist-facing inversion correctly made public evidence
dossiers the primary product. It still routed verification, empirical evidence,
and application fitness through a combined developer-issued disposition and a
terminal "release qualification" phase. That structure imported the outcome of
a regulated nuclear licensing process without its controlled operating
envelope, identified regulator, or authority to adjudicate each environmental
application.

User direction on 2026-07-14 authorizes this follow-on. The revised strategy
must preserve nuclear-style hard gates for propositions openWEPP can close,
while treating natural-system validation as continuously revisable
corroboration and equipping, rather than replacing, the user's application
fitness judgment.

## Included Scope

- State the asymmetry between verification acceptance and empirical
  corroboration explicitly.
- Separate software-verification status, empirical corroboration status, and
  decision-owner application fitness.
- Replace combined openWEPP use dispositions with evidence characterizations
  that cannot be mistaken for site-specific authorization.
- Replace terminal scientific release qualification with a verified software
  release carrying an immutable, as-of corroboration profile.
- Add decision ownership and an application-context worksheet to the dossier
  standard.
- Preserve bounded domains, uncertainty, negative evidence, independent review,
  calibration separation, comparator posture, and content-bound manifests.
- Add Oreskes, Shrader-Frechette, and Belitz (1994) to the canonical bibliography
  and bound the role of nuclear precedents in the research basis.
- Update navigation, package evidence, dual independent review and verification,
  finding disposition, and final closure.

## Excluded Scope

- Changing executable release gates, science contracts, correctness authority,
  runtime behavior, datasets, fixtures, or tests.
- Performing an application fitness assessment for any user, watershed, or
  decision.
- Reclassifying existing empirical evidence or publishing a new corroboration
  status.
- Selecting or admitting a new observational dataset.
- Implementing schemas, services, evidence databases, V&V crates, or report
  generators.
- Rewriting the completed prior package's historical review or disposition
  artifacts.

## Deliverables

- Revised `docs/governance/openwepp-verification-validation-strategy.md`.
- Revised `docs/standards/scientific-assurance-dossier.md`.
- Updated bibliography entries and navigation descriptions.
- Package-local reading map, implementation evidence, gates, two independent
  reviews, finding disposition, two independent verifications, handoff, and
  final disposition.

## Dependencies

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/20260714-vv-strategy-scientist-facing-inversion-001/`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/decisions/0028-observed-data-admission-authority.md`
- `docs/specifications/correctness-authority-model.md`
- bibliography entries `R-114` through `R-125`

## Intended Write Set

- `docs/governance/openwepp-verification-validation-strategy.md`
- `docs/governance/README.md`
- `docs/standards/scientific-assurance-dossier.md`
- `docs/standards/README.md`
- `docs/README.md`
- `references/annotated_bibliography.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260714-vv-asymmetric-assurance-reframe-001/**`

## Subagent Authorization And Requirement

Subagent authorization: this package explicitly authorizes spawning/delegating
to two independent reviewer/verifier subagents. Reviewer A examines scientific
user agency, application-context usability, audience language, and whether the
dossier equips rather than adjudicates the user's decision. Reviewer B examines
the epistemic V&V asymmetry, verification-gate precision, Oreskes/EPA/nuclear
source characterization, release semantics, and preservation of audit controls.
Expected outputs are package-local `artifacts/review-a.md`,
`artifacts/review-b.md`, `artifacts/verification-a.md`, and
`artifacts/verification-b.md`. Each reviewer may write only its assigned
artifact and must not read the other reviewer's artifact before initial review.

Subagent requirement: dual independent review and verification are required.
No heavy batch, comparator, release, or Rust closure run is in scope.

## Phase Plan

1. Freeze authority, terminology, decision ownership, and the package reading
   map.
2. Revise the strategy, dossier standard, bibliography, and indexes.
3. Run spelling, Markdown, local-path, status, terminology, scope, and diff
   checks.
4. Obtain two independent reviews, disposition every finding, and remediate all
   accepted findings.
5. Obtain dual accepted-fix verification and record terminal disposition.

## Exit Criteria

| ID | Criterion |
| --- | --- |
| `VVASYM-001` | The strategy defines software verification, empirical corroboration, and application fitness as separate layers with named decision owners. |
| `VVASYM-002` | Verification evidence may be quantitative, but exact obligations become binary release gates only after requirements and tolerances are declared. |
| `VVASYM-003` | Empirical agreement is partial and nonterminal; contradiction may exclude a use or domain, while successful corroboration does not establish whole-model fitness. |
| `VVASYM-004` | openWEPP provides application decision support but does not issue a site-specific fitness verdict unless it is itself the explicitly named decision owner. |
| `VVASYM-005` | Verification and corroboration use distinct status vocabularies; a combined developer-issued `SUPPORTED` disposition no longer conflates them with application authorization. |
| `VVASYM-006` | A release is a verified software realization plus an immutable as-of corroboration snapshot, not a terminal scientific qualification event. |
| `VVASYM-007` | The dossier standard leads with an evidence summary, retains the content-bound audit manifest, and adds a practical application-context worksheet and optional decision-owner assessment. |
| `VVASYM-008` | The research basis accurately bounds nuclear precedent and incorporates the open-natural-system corroboration argument with canonical bibliography provenance. |
| `VVASYM-009` | Calibration separation, uncertainty, scale, comparator-as-flag, negative evidence, review, and fail-closed known-invalid exclusions remain intact. |
| `VVASYM-010` | Navigation, spelling, Markdown, local-path, terminology, diff, and scope checks pass. |
| `VVASYM-011` | Two independent reviews and verifications complete; every finding is dispositioned and every accepted finding is fixed. |
| `VVASYM-012` | Documentation-only scope, no new scientific verdict, no runtime/security impact, and zero touched `.rs` files are recorded truthfully. |

Statuses are `PASS`, `FAIL`, `BLOCKED`, or `NOT RUN`. Any state other than
`PASS` blocks `COMPLETE` disposition.

## Security Impact

Security impact is low. The package changes tracked Markdown only, performs no
external publication, handles no credentials or restricted payloads, and
changes no executable behavior. Application-context guidance must not encourage
publication of private site data, credentials, or restricted dataset locations.

## Truthfulness

Evidence artifacts label read/reasoned evidence as `Static:` and command
evidence as `Ran:`. This package changes how evidence and decision ownership are
described; it does not change the strength of any existing evidence or decide
fitness for an application.

## Progress

- [x] (2026-07-14) User authorized the asymmetric assurance reframe.
- [x] (2026-07-14) Prior package closure and applicable instruction chains were
  inspected without modifying historical artifacts.
- [x] (2026-07-14) Revised canonical strategy, standard, bibliography, and
  navigation around asymmetric decision ownership.
- [x] (2026-07-14) Initial documentation, spelling, link, terminology, scope,
  and diff gates passed.
- [x] (2026-07-14) Two independent reviews returned `HOLD`; all three findings
  were accepted and remediated without deferral.
- [x] (2026-07-14) Both independent accepted-fix verifications and terminal
  gates passed; package disposition is `COMPLETE`.

## Surprises And Discoveries

- The current strategy already separates evidence dimensions but recombines
  them in `SUPPORTED` dispositions and a terminal release-qualification phase.
  The defect is decision ownership, not missing evidence taxonomy.
- Independent review showed that a list of worksheet fields is not a usable
  worksheet and that negative empirical status is not automatically safe: an
  unverified mismatch cannot establish model contradiction.

## Decision Log

- Decision: Create a dependent follow-on package instead of reopening the
  completed scientist-facing inversion package.
  Rationale: The earlier package truthfully closed its stated objective; this
  follow-on materially changes disposition and release semantics while
  preserving the earlier review record.
  Date/author: 2026-07-14, Codex.
- Decision: Do not define an openWEPP application-fitness status vocabulary.
  Rationale: The decision belongs to the user's institutional context; adding a
  third project ladder would recreate the audience and cognitive-load problem
  this reframe is intended to solve.
  Date/author: 2026-07-14, Codex.

## Outcomes And Retrospective

The reframe now matches the actual epistemic and decision structure of
openWEPP: verification closes specified software propositions; empirical
corroboration remains dated, bounded, and revisable; and application fitness
belongs to the person or institution responsible for the decision. The public
dossier is the handoff between those layers, not a licensing surrogate.

Independent review materially improved the result. It converted the worksheet
from a field list into a usable comparison form, prevented an unverified
implementation mismatch from being mislabeled as model contradiction, and
removed an overstatement of certainty from the nuclear analogy. All three
findings were fixed and independently verified in this package.
