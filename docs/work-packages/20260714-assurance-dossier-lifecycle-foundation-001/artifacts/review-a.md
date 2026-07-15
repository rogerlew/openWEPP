# Independent Review A: Scientific User And Governance

Review A disposition: **FAIL**. Hold package closure until the four
closure-blocking findings below are corrected and dispositioned.

Candidate reviewed:

- `FROZEN_BASE`:
  `00d985b1c0de77f1ea664df23a6f4999c4dad0cc`;
- dossier source-root SHA-256:
  `907ff5a9d1b50b869e78773d3a5448b67ade55eff25110ff216890642e9b1d28`;
  and
- dossier lifecycle and empirical status: `CANDIDATE` /
  `INSUFFICIENT_EVIDENCE`.

Static: I reviewed the package, lifecycle contract, V&V strategy, dossier
standard, usersum style guide, SNOTEL source records and public pages, ADR-0028,
the retained observation and activation evidence, the ownership/evidence
artifacts, and the agent-assisted-analysis record. I did not read Reviewer B's
review or verification output.

Ran: I independently executed `openwepp-assurance validate --all` and
`plan --all`, checked the retained evidence SHA-256 identities, linted the ten
Reviewer A governance/public/agent-record Markdown files, resolved their 28
local links, previewed `uk2us`, and ran `git diff --check` on those files.

## Findings

### High: `ASSURE-A-001` — The review record cannot represent the required scientific review

The lifecycle contract requires disclosed role separation, prohibits a
conclusion-bearing author from independently approving their own work, and
says the review record names the reviewer, role, findings, and disposition
(`docs/governance/scientific-assurance-dossier-lifecycle.md:31` through `:49`
and `:187` through `:195`). The implemented strict record has only one free-text
`reviewer`, one date, two digests, and a list of unstructured finding strings
(`assurance/schemas/review.schema.json:7` through `:17`). It has no reviewer
role or expertise, independence disclosure, conclusion-author identity,
structured finding disposition, residual disagreement, or review-history
record.

This is not only a missing report field. A conforming `approved` record can be
accepted without evidence that its reviewer is scientifically qualified or
independent and without every finding being dispositioned. The single record
also cannot preserve a scientific approval while separately recording a later
tooling/editorial approval. That matters because the reviewed source root binds
templates, compiler sources, Cargo manifests, and `Cargo.lock`, while the
trigger matrix says a nonsemantic template/renderer change does not require
independent rereview. In practice any such byte change invalidates the sole
lock, but the data model cannot distinguish rendered-diff approval from renewed
scientific assessment.

Required resolution: make review identity, role/expertise, independence and
authorship separation, findings, dispositions, and retained review history
explicit and auditable. Reconcile the trigger matrix with the lock model so a
mechanical change receives the declared level of review without erasing or
impersonating scientific approval.

Severity: `HIGH`. Closure classification: `CLOSURE-BLOCKING`. Affected exit
criteria: `ASSURE-LIFE-001`, `ASSURE-LIFE-002`, `ASSURE-LIFE-003`,
`ASSURE-LIFE-004`, `ASSURE-BUILD-004`, and `ASSURE-GOV-001`.

### High: `ASSURE-A-002` — The dossier-level verification `PASS` is broader than its demonstrated obligations

The dossier's first screen labels software verification `PASS` for a question
covering daily SWE, snow depth, derived density, and seasonal response
(`usersum/assurance/dossiers/snow-snotel-swe-depth-density.md:6` through
`:21`). Its verification profile then narrows that pass to default-selector
behavior, retention of a legacy rollback path, selected-model trace, and
phase-partition conservation (`:27` through `:32`). Those are useful individual
obligations, but they are not a complete verification profile for the published
SWE/depth/density result surface.

The dossier standard defines `PASS` for an exact realization and requires the
verification profile relevant to empirical interpretation to cover
requirements, code, solution, integration, the real consumer, and output
lineage. The current source has one aggregate verification enum and no
obligation records for those layers. It also says an exact current-release
identity is available only in a future release snapshot. A reader therefore
encounters an unqualified top-level pass before learning that only a narrow
historical activation/conservation subset passed.

Required resolution: enumerate the material verification obligations, exact
realization and dates, evidence pointers, quantitative results/tolerances, and
per-obligation statuses. Do not present an aggregate dossier-level `PASS` until
every mandatory obligation for the interpreted and published quantities is
`PASS`; alternatively label the currently passing selector and conservation
checks as individual evidence without implying full verification acceptance.

Severity: `HIGH`. Closure classification: `CLOSURE-BLOCKING`. Affected exit
criteria: `ASSURE-LIFE-003`, `ASSURE-PILOT-002`, `ASSURE-PILOT-003`, and
`ASSURE-GOV-001`.

### High: `ASSURE-A-003` — The model narrative converts an insufficient SNOTEL record into a downstream “improved” claim

The dossier correctly says the five-climate evidence is
`INSUFFICIENT_EVIDENCE`, does not establish general snow accuracy, and leaves
application-specific runoff and erosion adequacy unknown. The linked why
narrative nevertheless says runoff, erosion, and watershed outputs changed
“because the snow inputs feeding them improved”
(`usersum/snow-frost-modeling-and-validation.md:318` through `:322`).

“Improved” is a favorable scientific characterization, not merely a statement
that a default changed. It is not supported by the candidate dossier and it
extends the SNOTEL interpretation into runoff, erosion, and watershed behavior,
which the package expressly excludes. This makes the why and what layers
disagree at the exact navigation path intended to prevent claim transfer.

Required resolution: describe the downstream discontinuity neutrally as the
effect of changed snow-process defaults, or bind any improvement claim to a
separate, independently reviewed downstream assessment. Do not infer improved
runoff, erosion, or watershed inputs from the SNOTEL activation decision.

Severity: `HIGH`. Closure classification: `CLOSURE-BLOCKING`. Affected exit
criteria: `ASSURE-PILOT-002`, `ASSURE-PILOT-003`, `ASSURE-PILOT-004`, and
`ASSURE-GOV-001`.

### Medium: `ASSURE-A-004` — Agent-assisted authoring is disclosed honestly but is neither complete nor lock-bound

The agent record commendably identifies the agent family, unavailable serving
identifier and settings, bounded questions, lower-status decision, and the fact
that private reasoning is not evidence. It remains incomplete, however. It
lists only “principal” input digests rather than the required complete set of
repository-relative input paths and SHA-256 identities, points to byte-count
reading maps instead of content identities, and has no specific retained-output
path/digest or completed reviewer findings, disposition, identity, and approved
candidate-root digest
(`artifacts/agent-assisted-analysis.md:3` and `:35` through `:75`).

Ran: `plan --all` reproduced source root
`907ff5a9d1b50b869e78773d3a5448b67ade55eff25110ff216890642e9b1d28`
and did not include `agent-assisted-analysis.md`. Consequently a changed agent
task, input inventory, retained output, or review packet does not invalidate the
dossier lock or produce `REVIEW_REQUIRED`, contrary to the contract's statement
that agent-assisted analysis is an authored input under the same review/lock
rules and that changed input invalidates approval
(`docs/governance/scientific-assurance-dossier-lifecycle.md:47` through `:49`
and `:197` through `:211`).

Required resolution: complete and content-identify the packet, bind its
conclusion-bearing prompt/input/output identities into the reviewed dependency
set, and mechanically enforce invalidation; or define and implement an equally
explicit separate approval dependency that cannot drift without
`REVIEW_REQUIRED`. Finalize the packet after accepted fixes rather than merely
adding an unbound terminal digest.

Severity: `MEDIUM`. Closure classification: `CLOSURE-BLOCKING`. Affected exit
criteria: `ASSURE-LIFE-004`, `ASSURE-BUILD-004`, `ASSURE-GOV-001`, and
`ASSURE-CLOSE-004`.

## Scientific-User Assessment

Static: apart from the findings, the why/how/what/application route is strong.
The catalog, dossier, method, narrative, limitations section, and worksheet are
mutually discoverable; the dossier's opening states the tested quantities,
tested domain, lower empirical characterization, unknowns, and decision-owner
boundary without requiring work-package vocabulary. Ran: all 28 local links in
the reviewed route resolve.

Static: the SNOTEL empirical classification itself is appropriately
conservative. The sources disclose retrospective model and threshold selection,
the absence of a calibration/evaluation split, mixed forcing products,
point-versus-hillslope support mismatch, incomplete raw acquisition replay,
unpropagated uncertainty, and absent external hydrologist review. Direct
observations, observation-only characterization, comparative diagnostics,
implementation/conservation verification, unavailable provenance, and review
gaps remain visibly distinct. The `INSUFFICIENT_EVIDENCE` characterization is
supported; it does not become application fitness.

Static: the lifecycle states, immutable-history posture, accountable why/how/
what/so-what roles, rereview triggers for scientific changes, evidence as-of
date, currency judgment, and release snapshot are well separated in prose. The
defects are in making that governance executable and internally consistent,
not in the stated philosophy.

## Reviewer A Exit-Criterion Audit

| Criterion | Status | Reviewer A evidence |
| --- | --- | --- |
| `ASSURE-LIFE-001` | `FAIL` | `ASSURE-A-001`: the contract states role separation, but the canonical review record cannot disclose or preserve it. |
| `ASSURE-LIFE-002` | `FAIL` | `ASSURE-A-001`: lifecycle states are clear, but renderer/tool lock behavior conflicts with the independent-rereview trigger matrix. |
| `ASSURE-LIFE-003` | `FAIL` | `ASSURE-A-001` and `ASSURE-A-002`: scientific approval and mechanical approval are not representable separately, and a narrow verification subset is exposed as a dossier-level pass. |
| `ASSURE-LIFE-004` | `FAIL` | `ASSURE-A-001` and `ASSURE-A-004`: review and agent-authoring identities are not fully represented in the mechanically checked lifecycle. |
| `ASSURE-BUILD-004` | `FAIL` | `ASSURE-A-001` and `ASSURE-A-004`: ordinary bound source changes invalidate the lock, but required review metadata and agent-authoring inputs can drift outside it. |
| `ASSURE-PILOT-001` | `PASS` | The inventory accurately distinguishes empirical, comparative, verification, review, tracked, and unavailable evidence and forcing posture. |
| `ASSURE-PILOT-002` | `FAIL` | The empirical status is appropriately insufficient, but `ASSURE-A-002` and `ASSURE-A-003` leave the complete public status/claim posture nonconforming. |
| `ASSURE-PILOT-003` | `FAIL` | Navigation is usable, but its first-screen verification pass and linked downstream-improvement statement can mislead a scientific user. |
| `ASSURE-PILOT-004` | `FAIL` | Generated-page mechanics pass, but the linked public narrative and dossier disagree about whether the snow evidence supports an improvement claim. |
| `ASSURE-GOV-001` | `FAIL` | All four findings are contradictions between governing requirements and the record, lock, or public language that implements them. |
| `ASSURE-CLOSE-004` | `NOT RUN` | This initial review contains four undispositioned closure findings; accepted-fix verification remains pending. |

Reviewer B-scoped build, release, and security criteria are not dispositioned
here.

## Executed Checks And Gate Non-Deferral

- Ran: `cargo run --quiet -p openwepp-assurance -- validate --all` returned
  `validation: PASS`.
- Ran: `plan --all` reproduced the assigned source root and `review=pending`;
  it content-identified the retained SNOTEL evidence but not the agent packet.
- Ran: all six tracked evidence identities in the pilot manifest matched the
  files inspected.
- Ran: targeted Markdown lint returned zero errors and warnings; 28 local links
  resolved with zero missing targets; `git diff --check` returned zero.
- Ran: spelling preview proposed only the deliberate `non-agricultural`
  terminology and the published article title “Hydrological modelling”; no
  rewrite is warranted.

Static: the candidate gate record correctly leaves full closure and dual review
at `NOT RUN`; it does not claim package completion. These four findings are all
closure-blocking, not follow-up items. Review A therefore returns **FAIL**.
