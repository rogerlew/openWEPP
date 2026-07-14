# Independent Review A: Asymmetric Scientific Assurance Reframe

Recommendation: **HOLD** pending correction and disposition of
`VVASYM-A-001`.

Static: I reviewed the package, kickoff prompt, required-reading map,
implementation evidence, canonical strategy, dossier standard, changed
navigation descriptions, gate results, and owned-file manifest. I did not read
any Reviewer B artifact.

Ran: I independently ran scoped Markdown, local-link, spelling-preview,
diff-integrity, changed-scope, and size checks.

## Finding

### High: `VVASYM-A-001` — The application-context “worksheet” is not yet a usable worksheet

The standard says every dossier includes a blank or example worksheet and that
completion is required before an application-fitness assessment
(`docs/standards/scientific-assurance-dossier.md:181` through `:185`). What
follows is a comprehensive prose list of fields (`:186` through `:202`), while
the starter outline supplies only an `Application context worksheet` heading
(`:247` through `:271`). There is no blank form, example, or side-by-side
comparison structure to complete.

The listed content is scientifically appropriate, but a hydrologist,
practitioner, agency reviewer, or dossier author must still invent how to align
the application requirement with the dossier's tested domain, identify the
specific difference or contradiction, and record its decision consequence.
That makes the most important handoff—from openWEPP evidence to the decision
owner's contextual judgment—less auditable and less usable than the navigation
claim of an “application-context worksheet” implies. `VVASYM-007` is therefore
not yet met.

Required remediation: add a compact, copyable blank template or worked example
to the standard. At minimum it should include:

1. a decision-ownership block for decision, organization, responsible role,
   date, consequence, reversibility, and required uncertainty tolerance;
2. a comparison table with columns equivalent to **decision factor**,
   **application facts or requirement**, **dossier evidence or tested range**,
   **difference / extrapolation / unknown / contradiction**, and **decision
   consequence, mitigation, or evidence needed**;
3. rows covering quantity/units/scale/configuration, climate/soil/topography/
   management/disturbance/topology, forcing and parameters, local observations
   and data roles, exclusions, verification gaps, and empirical contradictions;
   and
4. a separate decision-owner record for institutional decision, conditions,
   rationale, evidence-snapshot identity, author, and required approval.

Keep those entries free-text or institution-owned; do not create a third
openWEPP status ladder. Retain the privacy warning and the rule that completing
the worksheet is optional for reading but required before recording an
application assessment.

## Scientific-User And Decision-Owner Assessment

Static: Apart from the worksheet-format gap, the reframe equips rather than
adjudicates. The strategy identifies separate owners for software-verification
acceptance, empirical corroboration, and application fitness
(`docs/governance/openwepp-verification-validation-strategy.md:25` through
`:42`). It repeatedly states that an openWEPP release does not authorize a
watershed use, and the dossier requires the application-fitness statement to be
authored under the named decision owner's institutional terminology and approval
process.

Static: Public language is transparent and scientifically honest. Verification
uses `PASS` / `FAIL` / `BLOCKED` / `NOT_RUN` only for predeclared obligations on
an exact realization. Empirical evidence uses bounded, dated corroboration,
mixed, contradicted, insufficient, or unevaluated characterizations. The two are
not recombined into `SUPPORTED`, `VALIDATED`, or another developer-issued use
verdict. A well-founded contradiction can narrow or reject a bounded claim;
successful comparisons remain partial and cannot average contrary evidence into
a general pass.

Static: The scientific safeguards remain intact: calibration/evaluation
separation, forcing reconstruction versus operational lanes, uncertainty and
natural variability, scale and regime boundaries, comparator-as-flag posture,
visible negative and superseded evidence, independent review, and fail-closed
treatment of mandatory verification failure and material empirical
contradiction.

Static: Audit and review controls remain proportionate. Claim-bearing evidence
is content-bound, but a manually authored Markdown, JSON, or YAML manifest is
sufficient; no schema, database, service, provenance export, report generator,
or V&V crate is required. Review depth scales with consequence of misuse. The
combined strategy and standard are `4,949` words, but progressive disclosure
keeps the public evidence summary ahead of technical audit detail and avoids a
third project status ladder.

## Exit-Criterion Audit

| Criterion | Status | Reviewer A evidence |
| --- | --- | --- |
| `VVASYM-001` | `PASS` | Strategy lines 25-42 separate the three questions and assign maintainers/release authority, the scientific program/domain reviewers, and the application decision owner. |
| `VVASYM-003` | `PASS` | Strategy lines 120-132 make successful corroboration partial and revisable while allowing well-founded contradiction to establish a bounded nonuse domain or narrow a claim. |
| `VVASYM-004` | `PASS` | Application fitness remains with the named hydrologist, practitioner, agency, or other decision owner; no release or empirical status silently authorizes use. |
| `VVASYM-005` | `PASS` | Verification and empirical vocabularies are distinct in both canonical documents; combined developer-issued support language is prohibited. |
| `VVASYM-007` | `FAIL` | The evidence summary and content-bound manifest pass, but `VVASYM-A-001` shows that the promised practical worksheet is only a content list and heading. |
| `VVASYM-009` | `PASS` | Calibration separation, uncertainty, scale/regime limits, comparator posture, negative evidence, review, contradiction, and known-invalid fail-closed controls remain explicit. |
| `VVASYM-010` | `PASS` | Ran: 13 Markdown files validated with 0 errors/warnings; 60 local links resolved with 0 missing; spelling preview found no proposal in current canonical/package files; `git diff --check` passed. |
| `VVASYM-011` | `NOT RUN` | Dual review, finding disposition, accepted-fix verification, and terminal closure remain pending. |
| `VVASYM-012` | `PASS` | Ran/Static: all 28 changed paths are Markdown, with 0 touched `.rs` or executable surfaces and no new scientific or application verdict. |

## Gate Non-Deferral And Recommendation

Static: The package remains `ACTIVE`; its gate artifact records independent
review and accepted-fix verification as `NOT RUN` and explicitly denies closure
on the initial documentation gates alone. That is the correct non-deferral
posture. The documentation-only exemption from Rust and CRAP closure gates is
legitimate.

The asymmetric assurance philosophy, status separation, contradiction posture,
user agency, and proportional audit kernel are sound. The missing fillable or
example worksheet is nevertheless central to the promised decision-owner
handoff and is closure-blocking. Reviewer A recommends **HOLD** until
`VVASYM-A-001` is fixed and independently verified.
