# Scientific Assurance Dossier Standard

Status: `Active`

Audience: dossier authors, scientific reviewers, maintainers, and agents

Purpose: make openWEPP verification and empirical evidence understandable and
auditable, while preserving application decisions for hydrologists, soil
scientists, researchers, practitioners, and other named decision owners

Governing strategy:
[openWEPP verification and validation](../governance/openwepp-verification-validation-strategy.md)

Lifecycle, ownership, source/generated boundaries, review locks, and release
snapshots:
[dossier lifecycle and build contract](../governance/scientific-assurance-dossier-lifecycle.md)

## Core Rule

A scientific assurance dossier answers a bounded **evidence question**. It is
not a test dashboard, work-package summary, promotional model card, dump of
provenance records, or developer-issued authorization for an environmental
application.

The first page must tell a scientific reader what was assessed, what the
evidence says, where it applies, where it fails or remains unknown, and what
questions remain for an application. Detailed verification and reproduction
evidence remains linked and inspectable without dominating that explanation.

A dossier may and should be published with `NOT_EVALUATED`,
`INSUFFICIENT_EVIDENCE`, `MIXED_EVIDENCE`, or
`CONTRADICTED_WITHIN_TESTED_DOMAIN` empirical evidence. Publishing an honest
gap or contradiction is a successful transparency outcome.

## Unit And Status Separation

One dossier covers one bounded quantity of interest and assessment domain, or a
tightly related set for which the same data and conclusions apply. Split a
dossier when its quantities, scales, regimes, datasets, or evidence conclusions
differ enough that one summary would conceal those differences.

Report required software-verification obligations with these statuses:

| Verification status | Meaning |
| --- | --- |
| `PASS` | The exact realization met the predeclared requirement and tolerance. |
| `FAIL` | The realization violated the requirement or tolerance. |
| `BLOCKED` | Required evidence could not be obtained or adjudicated. |
| `NOT_RUN` | The obligation has not been executed for the exact realization. |

Only `PASS` closes a mandatory verification obligation. Quantitative evidence
such as a residual, convergence rate, or numerical-error estimate remains
visible behind the status.

The dossier also reports one mechanical aggregate over its mandatory
obligations. It is `FAIL` if any mandatory obligation fails, otherwise
`BLOCKED` if any is blocked, otherwise `NOT_RUN` if any is unrun, and only
`PASS` when every mandatory obligation passes. This aggregate is a concise
closure signal, not a whole-model grade. Every row still names the exact
software realization, execution date, requirement, tolerance, result, and
evidence identity; a selector or conservation check cannot stand in for
solution verification, release-result lineage, or independent reproduction.

Report empirical evidence with one of these separate characterizations:

| Empirical status | Meaning |
| --- | --- |
| `CORROBORATED_WITHIN_TESTED_DOMAIN` | Results agree with the stated referents to the degree described, within the tested conditions only. |
| `MIXED_EVIDENCE` | Material successes and failures prevent one directional characterization. |
| `CONTRADICTED_WITHIN_TESTED_DOMAIN` | Evidence conflicts with the bounded representational claim or establishes a nonuse domain. |
| `INSUFFICIENT_EVIDENCE` | The design, data, or uncertainty cannot sustain a characterization. |
| `NOT_EVALUATED` | No claim-bearing empirical assessment has been completed. |

`CORROBORATED_WITHIN_TESTED_DOMAIN`, `MIXED_EVIDENCE`, and
`CONTRADICTED_WITHIN_TESTED_DOMAIN` require every verification obligation
material to the result surface to be `PASS`. If one is `FAIL`, `BLOCKED`, or
`NOT_RUN`, preserve that verification status and classify the affected
empirical assessment as `NOT_EVALUATED` or `INSUFFICIENT_EVIDENCE`. Retain any
observed mismatch as negative implementation evidence; it cannot yet
corroborate or contradict the model's representational claim.

Do not combine these statuses into `SUPPORTED`, `VALIDATED`, or another
whole-model use disposition. State the quantity, version, scale, tested domain,
evidence date, and limitations.

## Three-Layer Structure

### Layer 1: Evidence Summary

The opening section is written for scientific users and practitioners. It
contains:

- a plain-language quantity, assessment purpose, and decisions the evidence may
  inform;
- the quantities, units, spatial and temporal scales, and assessed version;
- the verification and empirical statuses, kept separate, with their dates;
- a short statement of what was corroborated, contradicted, mixed, insufficient,
  or not evaluated;
- the named observational or exact-reference basis and its regime coverage;
- the most important performance result, bias, failure, and uncertainty;
- known exclusions and the most important application-context questions;
- an explicit statement that application fitness belongs to the named decision
  owner; and
- prominent links to the scientific evidence and reproducibility layer.

Do not begin with claim identifiers, build hashes, test counts, or internal
governance vocabulary. Those details belong in the audit layer unless they
change the scientific interpretation.

### Layer 2: Scientific Evidence

The evidence body explains the assessment in enough detail for a domain expert
to evaluate it. It contains:

1. **Assessment envelope:** intended decision context, quantities, processes,
   scales, regimes,
   interpretation criteria, and exclusions. The envelope bounds an evidence
   statement; it is not an application authorization.
2. **Data coverage:** named datasets, observed variables, locations or systems,
   periods, event counts, regime coverage, uncertainty, representativeness,
   transformations, and calibration/evaluation role.
3. **Method:** forcing lane, parameter source, model configuration, comparison
   basis, metrics, stratification, and handling of missing or near-zero values.
4. **Results:** figures and tables showing central behavior, residuals,
   uncertainty, variability, extremes, and regime-specific successes and
   failures.
5. **Verification profile:** a concise account of requirements, code, solution,
   integration, real-consumer, and output-lineage evidence relevant to
   interpreting the results.
6. **Uncertainty and applicability:** material uncertainty sources, supported
   interpolation, guarded extrapolation, and regions outside the evidence.
7. **Limitations and negative evidence:** known defects, failed cases, missing
   processes, conflicting evidence, and unexecuted obligations.
8. **Review and characterization:** reviewer expertise and independence,
   findings, finding disposition, residual disagreement, and the reasoning for
   the published verification and empirical statuses.

Verification, empirical corroboration, comparative evidence, software QA,
uncertainty, and independent review remain visibly separate. A favorable result
in one dimension cannot silently compensate for a failed mandatory obligation
or contradiction in another, and no combination silently becomes application
fitness.

### Layer 3: Reproducibility And Audit

The audit layer contains or links to the minimum information needed to reproduce
and challenge the assessment:

- source commit and dirty-state status;
- executable, dependency, schema, and configuration identities;
- complete inputs and parameters;
- dataset origin, rights or access posture, collection and quality-control
  notes, transformations, and partition assignment;
- exact commands, environment assumptions, metric definitions, units, and
  aggregation bases;
- retained outputs, logs, figures, and material failed or superseded runs;
- links from each conclusion and limitation to supporting evidence; and
- dossier version, supersession history, reviewer record, and unresolved
  findings.

Every dossier has a tracked evidence manifest. For each claim-bearing input,
parameter set, transformation source or configuration, output, log, figure,
review, and material failed or superseded artifact, the manifest records:

- its scientific or verification role;
- a stable repository-relative path or external location and access posture;
- a SHA-256 digest, or a named equivalent content identity with justification;
- whether it is available, restricted, external, or unavailable; and
- the source, executable, and assessment identity under which it was produced
  or used.

The dossier records the manifest path and digest so its statuses and conclusions
are bound to that evidence set. A material unavailable or unidentifiable asset
remains an explicit evidence limitation and cannot silently support a favorable
characterization.

The manifest may be a manually authored Markdown table, JSON file, or YAML file.
The active public vertical slice uses the bounded `openwepp-assurance` compiler
to validate and render the canonical YAML form. This does not require or justify
a database, service, generalized provenance export, workflow engine, or
automated scientific adjudicator.

## Figures And Tables

Use the smallest set of visuals that exposes performance and failure modes.
Select plots for the quantity and use rather than following a universal chart
checklist. A claim-bearing empirical dossier normally needs more than one
aggregate metric and should consider:

- predicted-versus-observed values with a visible one-to-one reference;
- signed residuals against observed magnitude or another relevant driver;
- distributions, exceedances, or event extremes;
- time series, timing, seasonal, or annual behavior when temporal response
  matters;
- results stratified by climate, soil, management, slope, scale, event size, or
  another claim-defining regime; and
- uncertainty or replicate-observation variability where available.

Every figure and table states the quantity, units, sample size, aggregation,
data role, and relevant uncertainty. Show excluded and failed cases or account
for them explicitly. Avoid truncated axes, undisclosed filtering, pooled results
that hide regimes, and color-only encodings that obstruct accessibility.

## Application Context Worksheet

Every dossier includes a blank or example worksheet that helps a user compare a
specific application with the evidence. Completing it is optional for reading
the dossier but required before recording an application-fitness assessment.
Use or adapt this copyable blank template.

| Decision ownership | Entry |
| --- | --- |
| Decision and purpose | |
| Organization and responsible person or role | |
| Assessment date | |
| Consequence of error and reversibility | |
| Required accuracy or uncertainty tolerance | |
| Dossier and evidence-snapshot identity | |
| Target openWEPP version and configuration | |

| Decision factor | Application facts or requirement | Dossier evidence or tested range | Difference, extrapolation, unknown, or contradiction | Decision consequence, mitigation, or evidence needed |
| --- | --- | --- | --- | --- |
| Quantity, units, aggregation, and spatial/temporal scale | | | | |
| Climate, seasonality, event magnitude, and extremes | | | | |
| Soils and topography | | | | |
| Management and disturbance | | | | |
| Hillslope, channel, and watershed topology | | | | |
| Forcing and parameter sources, resolution, quality, and uncertainty | | | | |
| Local observations, calibration/model-selection role, and held-out checks | | | | |
| Known exclusions, missing processes, and extrapolations | | | | |
| Verification failures, blocks, or unrun obligations | | | | |
| Mixed or contradicted empirical evidence | | | | |

| Decision-owner record | Entry |
| --- | --- |
| Institutional decision and conditions | |
| Rationale and material evidence | |
| Mitigation or additional evidence required | |
| Author and date | |
| Required institutional review or approval | |

An optional application assessment uses the decision owner's institutional
terminology and approval process, not another openWEPP status vocabulary. It
applies only to the named decision and does not transfer to another site,
purpose, configuration, or decision owner. Private site data, credentials, and
restricted dataset locations must not be placed in a public dossier.

## Authoring Workflow

1. Define the quantity, decision context, and assessment envelope. Initialize a
   new empirical assessment as `NOT_EVALUATED`; do not select a favorable
   characterization as an execution target.
2. Inventory existing evidence and classify it without strengthening its
   meaning. CI, conservation, and legacy comparison are not empirical
   corroboration.
3. Name the observational datasets or exact referents. Record `NOT_EVALUATED`
   or `INSUFFICIENT_EVIDENCE` when no suitable referent exists.
4. Freeze evaluation roles, metrics, criteria, exclusions, and required
   verification before new conclusion-bearing execution. For retrospective
   evidence, record which choices were or were not prespecified and the resulting
   bias risk rather than inventing retrospective preregistration.
5. Build the evidence and audit layers. Assign each verification status from its
   declared requirement and each provisional empirical characterization from
   the results and limitations; do not combine them.
6. Obtain scientific review and independent evidence verification proportional
   to the consequence of misuse for every dossier, including a baseline
   inventory.
7. Disposition every finding, finalize the evidence-derived statuses, and
   publish with unresolved limitations and contradictions visible.
8. Supersede rather than erase the dossier when evidence, code, configuration,
   outputs, or the assessment envelope changes materially.

A baseline dossier need not rerun historical work or pretend that its plan was
prospectively registered. Review must still verify its evidence classification,
content identities, provenance, limitations, and public summary. Mapping
historical evidence to a new decision context or envelope is a new assessment.
A prior
favorable characterization may be carried forward only when its independently
reviewed claim, version, quantity, scale, regimes, criteria, and limitations are
demonstrably identical and its evidence remains current. Otherwise the dossier
stays `NOT_EVALUATED` or `INSUFFICIENT_EVIDENCE` until the new empirical
assessment is reviewed. Any application-fitness assessment remains a separate
record authored by its decision owner.

## Starter Outline

Use this heading structure unless the scientific question requires a clearer
organization:

```text
# <Quantity and assessed domain>: scientific assurance dossier

Status, assessed version, assessment date, dossier version

## Evidence summary
## Verification status and profile
## Empirical corroboration status
## Assessment envelope
## Observational or exact-reference data
## Methods and evaluation design
## Results
## Uncertainty, applicability, and extrapolation
## Limitations, failures, contradictions, and unevaluated regions
## Application considerations
## Application context worksheet
## Optional decision-owner assessment
## Independent review and finding disposition
## Reproduce and audit
## Supersession history
```

The headings are not evidence. Omit empty boilerplate only when the evidence
summary explicitly identifies why the section is not applicable. Missing or
expensive evidence is not automatically not applicable.

## Quality Check

Before publication, reviewers confirm:

- the first page answers the practical scientific question without requiring
  internal repository knowledge;
- verification and empirical statuses are visibly separate and no broader than
  the version, quantity, scale, data, and regime evidence;
- no openWEPP-authored status is presented as site-specific application fitness;
- calibration and evaluation data are separated and disclosed;
- uncertainty, variability, failures, exclusions, and extrapolation are not
  hidden by aggregate results;
- verification evidence covers the production path and published quantity used
  in the empirical comparison;
- plots, metrics, prose, and audit artifacts agree;
- every material finding is dispositioned and unresolved findings remain
  visible; and
- reproduction pointers resolve without exposing credentials or restricted
  data.

If a mandatory verification obligation is `FAIL`, `BLOCKED`, or `NOT_RUN`, hold
verification acceptance, preserve that status, and use `NOT_EVALUATED` or
`INSUFFICIENT_EVIDENCE` for the affected empirical assessment. Publish any
mismatch as negative implementation evidence, not as corroboration or
contradiction. If a verified comparison supplies well-founded contradictory
evidence, use `CONTRADICTED_WITHIN_TESTED_DOMAIN` and narrow or reject the
affected claim. Other failed publication checks hold a favorable empirical
characterization until corrected. The dossier may still publish the gap or
negative evidence.
