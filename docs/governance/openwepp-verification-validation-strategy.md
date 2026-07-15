# openWEPP Verification and Validation Strategy

Status: `active-strategy`

Document type: asymmetric scientific assurance philosophy and adoption strategy

Delivery maturity: public dossier standard and bounded lifecycle compiler
active; dossier portfolio planned

Last reviewed: `2026-07-14`

Primary audiences: hydrologists, soil scientists, researchers, practitioners,
developers, reviewers, and release decision makers

Related authority:

- [scientific assurance dossier standard](../standards/scientific-assurance-dossier.md);
- [dossier lifecycle and build contract](scientific-assurance-dossier-lifecycle.md);
- [correctness authority model](../specifications/correctness-authority-model.md);
- [science-contract index](../specifications/science-contracts/index.md);
- [ADR-0017 comparator posture](../decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md);
- [ADR-0028 observed-data admission](../decisions/0028-observed-data-admission-authority.md); and
- [draft release procedure](openwepp-release-procedure-draft.md).

## Executive Position

openWEPP treats verification and validation (V&V) as an **asymmetric scientific
assurance program**. Some questions can close against declared requirements;
others remain conditional on observations, place, scale, purpose, and future
evidence. The program therefore keeps three decisions and their owners
separate:

1. **Software verification acceptance** asks whether an exact implementation,
   solution, interface, and publication path meet declared requirements and
   tolerances. Each required obligation resolves fail-closed for release;
   openWEPP maintainers and the named release authority own the decision.
2. **Empirical corroboration** asks how a verified calculation compares with
   defensible real-world referents over a bounded tested domain. The scientific
   program publishes graded, dated, revisable evidence, challenged by
   independent domain reviewers, rather than a terminal validity verdict.
3. **Application fitness** asks whether the available evidence is adequate for
   a particular watershed, decision, consequence, and uncertainty tolerance.
   The hydrologist, practitioner, agency, or other named decision owner owns
   this judgment; an openWEPP release does not issue that authorization.

The primary public product is a **scientific assurance dossier** for a bounded
quantity and assessment domain. It shows the tests, performance, applicability,
failures, gaps, and reproduction path, then helps a decision owner compare an
application with the tested domain. Generalized evidence infrastructure may
help later, but it is not a prerequisite for publishing current evidence.

Both verification and empirical corroboration are needed to interpret a
predictive result. A correctly implemented inadequate model is precisely wrong.
Agreement with observations is unreliable when the code path, numerical
solution, inputs, units, data partition, or output lineage are defective.
Verification makes empirical evidence interpretable; corroboration shows how
the verified model has behaved against reality so far. Neither decides whether
the model is fit for every application.

This strategy does not assert that openWEPP as a whole is validated, that every
current test has been reclassified, or that the planned public dossier portfolio
already exists.

## Transparency Outcome

A scientific user should be able to determine, without reading source code or
internal work-package logs:

1. which result, practical use, version, configuration, and scale were assessed;
2. which named observations or exact referents were used and which regimes they
   cover;
3. whether evaluation data were used for calibration or model selection;
4. how predictions compare with observations, including uncertainty,
   variability, biases, extremes, and failed regimes;
5. what remains untested and where use would be extrapolation;
6. which decisions openWEPP has made, which decision remains with the user, and
   which application facts must be considered; and
7. how the analysis and its verification basis can be reproduced and challenged.

Transparency includes publishing `NOT_EVALUATED`, `INSUFFICIENT_EVIDENCE`,
`MIXED_EVIDENCE`, and `CONTRADICTED_WITHIN_TESTED_DOMAIN` characterizations. A
visible gap or contradiction is more credible and useful than a green badge
whose scientific meaning is unclear.

Information is layered: a concise evidence summary for practitioners, a
scientific evidence body for researchers and reviewers, and a reproducibility
and verification annex for auditors, developers, and agents. An optional
application assessment is authored by its named decision owner, not inferred
from an openWEPP evidence status. The
[dossier standard](../standards/scientific-assurance-dossier.md) defines that
structure. All layers must report the same evidence.

## Governing Philosophy

### The decisions are asymmetric

The useful lesson from nuclear V&V is not that all evidence becomes binary.
Measurements, numerical errors, uncertainty analyses, and model comparisons
remain quantitative and contestable there too. The binary element is an
acceptance or authorization decision made by an identified authority against
declared requirements for a defined installation and operating envelope.

openWEPP adopts that discipline where the proposition is closable: exact
software requirements, numerical tolerances, conservation obligations,
interfaces, consumer paths, and release identity are hard-gated. It does not
import the licensing conclusion into an open natural system. Nuclear
authorization concerns an engineered installation, declared licensing basis,
and controlled operating envelope while explicitly managing residual
uncertainty. A watershed provides no comparable controlled site context, and
openWEPP is not the regulator or decision owner for each application.

### Claims are use-specific

No release, executable, crate, or process receives an unqualified "validated"
label. An empirical statement names the quantity, units, scale, processes,
exact software and configuration, tested regimes, evaluation design,
uncertainty treatment, and explicit exclusions or extrapolations. It describes
evidence within that envelope; it does not silently authorize a use outside it.

Evidence for annual hillslope soil loss does not establish event timing,
watershed sediment delivery, snow-dominated hydrology, or performance under an
untested management regime.

### Corroboration accumulates evidence, not truth

Models of open natural systems are purposeful simplifications of incompletely
known systems. Repeated agreement can corroborate behavior over tested
conditions; it cannot prove that the model is true or establish fitness for all
future applications. New observations may strengthen, narrow, or overturn an
assessment. Negative and superseded evidence remains discoverable rather than
being erased by a later favorable run.

The asymmetry also runs the other way: a well-founded contradiction can reject
or narrow a bounded representational claim, identify a nonuse domain, or block
a release purpose that depends on the claim. Successful comparisons remain
partial; known contrary evidence is not averaged into a general pass.

### Verification makes empirical evidence interpretable

Empirical investigation and verification may proceed iteratively, but a
corroboration status cannot be published as claim-bearing evidence until the
relevant algorithm, numerical solution, inputs, units, production consumer, and
output lineage are verified. The burden is proportional to the result being
interpreted, not every unrelated part of the repository.

Verification evidence is not inherently Boolean. Convergence rates, closure
residuals, numerical uncertainty, and comparator deltas are quantitative. A
verification obligation becomes a binary release gate only after its
requirement, metric, tolerance, applicable realization, and failure consequence
have been declared. An undeclared or unexecuted obligation is not a pass.

Dimensional consistency, conservation, boundedness, monotonic response, and
contract conformance can disprove an implementation or model. Alone, they do
not show that openWEPP predicts field behavior adequately.

### Capability and assessability are different

Greater model sophistication may improve process representation while making
the model harder to identify, observe, and test. Sparse field observations,
input uncertainty, natural variability, scale mismatch, and nonunique parameter
combinations can limit what can be inferred even from an advanced model. A
smaller corroboration envelope therefore does not by itself imply weak model
capability, and greater capability does not justify a broader evidence claim.

### Comparator agreement is a flag, not a target

ADR-0017 remains controlling. Agreement with pinned legacy WEPP is regression
or equivalence evidence. A difference is an investigation trigger. Neither
state establishes physical validity without independent authority and
like-for-like quantity lineage.

### Uncertainty and applicability are part of the result

Material measurement, sampling, natural, forcing, parameter, model-form,
numerical, and transformation uncertainty is reported where relevant. Unknown
uncertainty is recorded as unknown, not zero. Evidence applies only over its
represented regimes and scales; extrapolation and application fitness are
separate judgments owned by the application decision owner.

### Evidence is designed for challenge

The objective is not to persuade readers that openWEPP is generally correct or
to preempt their decision. It is to make the strongest defensible bounded
evidence statement easy to inspect, reproduce, dispute, revise, and apply with
appropriate caution. Failed cases, exclusions, and reviewer disagreements are
evidence, not presentation defects.

## Evidence And Public Language

Every dossier keeps the dimensions separate; a pass count, maturity score, or
aggregate badge cannot recombine them into a developer-issued use verdict. The
[dossier standard](../standards/scientific-assurance-dossier.md) defines the
statuses and their full meanings:

- required verification obligations use `PASS`, `FAIL`, `BLOCKED`, or
  `NOT_RUN`; only `PASS` closes an obligation for the exact realization;
- empirical assessments use `CORROBORATED_WITHIN_TESTED_DOMAIN`,
  `MIXED_EVIDENCE`, `CONTRADICTED_WITHIN_TESTED_DOMAIN`,
  `INSUFFICIENT_EVIDENCE`, or `NOT_EVALUATED`; and
- an optional application assessment belongs to its named decision owner and
  context, not to openWEPP's scientific status.

Use "verified against requirements" for software and "corroborated within the
tested domain" for empirical evidence, always with the named envelope and as-of
date. Do not use whole-model validity language or translate corroboration into
application fitness.

The minimum audit basis is exact software and configuration identity; dataset,
transformation, and calibration/evaluation provenance; metric definitions and
units; retained commands and results; visible failed evidence and limitations;
and independent review with finding disposition. A lightweight tracked manifest
content-identifies every claim-bearing input, transformation, output, figure,
log, review, and material failed or superseded artifact, and binds those assets
to the dossier version and its as-of evidence characterizations. The first
public vertical slice uses Markdown sources, strict schemas, and a bounded
deterministic compiler to remove recurring drift among the catalog, public
pages, review lock, release snapshot, and wepppy handoff. This compiler is
publication plumbing, not a generalized evidence platform or scientific
adjudicator.

## Scientific Program

### Empirical evaluation plans contain the science

Before conclusion-bearing execution, an empirical evaluation plan names the
decisions the evidence may inform; quantities and regimes; named datasets and
their uncertainty; calibration and held-out partitions; verification
obligations; metrics, stratifications, and interpretation criteria; and required
review. A schema or claim identifier cannot substitute for these specifics.

Field evaluation separates a **reconstruction lane**, using measured or
quality-controlled forcing to examine process response, from an **operational
lane**, using the forcing products and workflow expected in practice to examine
total predictive performance. Forcing-robust signatures and forcing-limited
absolute magnitudes remain distinct, consistent with ADR-0028.

Calibration and evaluation roles are declared before adjustment. Splits prevent
leakage across physical systems, event histories, climates, and managements;
random row splitting is not independent when rows share those structures.
Metrics and visuals are quantity-specific, stratified across the assessment
domain, and expose relevant bias, residuals, timing, extremes, conservation,
and uncertainty. An aggregate result cannot hide failed regimes.

Observation error, replicate and natural variability, and scale mismatch are
part of interpretation, not excuses for poor agreement. Evidence does not
transfer automatically between plot, hillslope, channel, and watershed scales;
interpolation and extrapolation are explicit.

### Verification is hierarchical

Verification progresses through requirements and design, code, numerical
solution, coupled production execution, and release realization. Evidence may
include analytical or manufactured solutions, independent calculations,
property tests, refinement and timestep studies, closure reconstruction,
consumer-path tests, and exact release identities.

Component evidence localizes errors; integrated evidence exposes coupling,
ordering, scale, and emergent behavior. Neither substitutes for the other. The
dossier summarizes the relevant profile and links to detailed contract, test,
closure, comparator, and release evidence. Passing that profile permits the
empirical results to be interpreted; it does not predetermine their
corroboration status.

## Governance Minimum

- Canonical `SC-*` contracts remain the authority for intended openWEPP physics;
  dossiers report evidence rather than replacing them or authorizing a use.
- Intended decision context, evaluation roles, metrics, and interpretation
  criteria are frozen before conclusion-bearing execution. Material post hoc
  changes reset the assessment and receive bias review.
- Failed cases, known defects, missing evidence, extrapolations, and limitations
  remain visible and linked to affected evidence statements.
- A failed mandatory verification obligation blocks verification acceptance. A
  material empirical contradiction on a verified result surface narrows or
  rejects the affected representational claim. Neither can be outweighed by
  favorable evidence in another dimension.
- Review independence and expertise scale with the consequence of misuse.
  Release verification requires independent evidence verification; published
  empirical characterizations require independent scientific review.
- Calibration against evaluation data, selective case removal, post hoc
  threshold changes, and tolerance relaxation block a favorable corroboration
  characterization unless the plan and assessment are reset and independently
  reassessed.
- Existing contract, authority-suite, work-package, defect, and release records
  are linked rather than copied into a conflicting authority ladder.
- Tooling is proportional. Absence of a generalized evidence platform cannot
  justify withholding an otherwise auditable dossier.

## Current openWEPP Baseline

openWEPP already has strong contract, integration, consumer-path, conservation,
fail-closed, release, and comparator controls. The completed integrated
validation campaign is principally **integration and release verification**
plus selected science-authority evidence. Its historical name does not make it
broad empirical corroboration across application regimes.

ADR-0028 records one concrete observed-data instance: a cross-climate SNOTEL
corpus of snow water equivalent, snow depth, and density across five climates,
used with a forcing-robust, decomposed rubric. It is a suitable starting point
for a public snow-process dossier after a current evidence and reproducibility
audit. It does not establish runoff, erosion, plant growth, routing, or
watershed-scale corroboration or application fitness.

The principal gaps are the absence of public dossiers for priority quantities,
insufficient named held-out cross-regime evidence for several public quantities,
limited systematic solution verification for some solvers, and fragmented
reproduction details. The immediate response is to publish this honest baseline
rather than wait for every gap or automation feature to close.

## Adoption Roadmap

### Phase 1: Publish the honest baseline

- Inventory priority public quantities and the decisions they may inform.
- Publish dossiers that separate verification status from corroborated, mixed,
  contradicted, insufficient, and unevaluated empirical states without
  strengthening existing evidence.
- Pilot the format with the existing SNOTEL snow evidence and one core
  hydrology or erosion quantity selected through dataset and applicability
  review.
- Establish mechanical lifecycle, ownership, cross-reference, build, and
  release-snapshot rules with the SNOTEL pilot; publish insufficient evidence
  when that is what the retained record supports.

Exit: a scientific user can find the current evidence, coverage, limitations,
practical meaning, and reproduction pointers without reading internal logs.

### Phase 2: Execute claim-driven scientific plans

- Author plans with named datasets, quantities, regimes, metrics, partitions,
  uncertainty treatment, criteria, and review.
- Strengthen analytical, manufactured-solution, refinement, and numerical
  evidence where the claims require it.
- Execute separate-effects, subsystem, and application campaigns in an order
  driven by process importance and public use.
- Publish figures, failures, reviewer findings, and updated evidence
  characterizations as campaigns close.

Exit: pilot characterizations rest on explicit scientific evidence rather than
legacy agreement or governance classification alone, without claiming to
decide application fitness.

### Phase 3: Standardize demonstrated recurring needs

- Evolve the versioned manifest only from fields demonstrated by real dossiers;
  do not turn the bounded baseline schema into an abstract portfolio ontology.
- Link existing tests, authority suites, release gates, and work packages rather
  than rewriting them.
- Automate report elements and freshness decisions where manual handling creates
  a demonstrated consistency or audit risk.

Exit: human and any machine views derive evidence statuses from one assessment
record, while a dossier remains reproducible without a service or database.

### Phase 4: Establish standing evidence stewardship

- Expand evidence across representative climates, soils, topographies,
  managements, scales, and extremes.
- Quantify sensitivity, uncertainty, and guarded extrapolation.
- Add portfolio-scale query, generalized impact analysis, or provenance export
  only when operating experience justifies them. Continue recording bounded
  immutable release snapshots under the lifecycle contract.
- Establish post-audit comparison with new field and operational observations.

Steady-state condition: each software release passes its declared verification
gates and carries an immutable, dated snapshot of current empirical
corroboration, contradiction, gaps, and exclusions. New evidence can supersede
the scientific snapshot without rewriting release history. No release event
turns the model into a terminally validated artifact.

A generalized tool is justified only when multiple real campaigns demonstrate
the same need, manual handling creates material audit risk, and the tool improves
the public evidence product without duplicating authority. The existing
`openwepp-assurance` crate remains intentionally limited to validation,
planning, rendering, drift detection, review locks, and snapshots. A database,
service, workflow engine, W3C PROV export, or RO-Crate export is a possible
later choice, not a strategy requirement.

## Release Basis, Evidence Snapshot, And Prohibited Shortcuts

A software release is an exact realization accepted against declared
verification obligations. Green CI is necessary but is not the whole
verification basis. Every required obligation must be `PASS`; `FAIL`, `BLOCKED`,
or `NOT_RUN` is closure-blocking. The release record preserves requirements,
tolerances, exact source and executable identity, results, unresolved defects,
independent evidence verification, and reproduction instructions.

The same release carries an immutable **as-of corroboration snapshot** for
priority public quantities. The snapshot preserves tested domains, empirical
characterizations, data pedigree, calibration separation, uncertainty,
applicability, comparative evidence, limitations, review, and public dossier
identity. `NOT_EVALUATED` and `INSUFFICIENT_EVIDENCE` remain visible and do not
become verification failures or silent scientific passes. A known contradiction
on a verified result surface blocks or narrows the affected scientific claim and
may block a release whose declared purpose depends on that claim.

Release means fit to execute under the software contract. It does not mean fit
for an unnamed watershed or environmental decision. That latter assessment is
made separately by the named application decision owner.

The following cannot close a verification obligation, establish empirical
corroboration, or decide application fitness:

- test count, coverage, complexity, lint, or a release badge standing in for
  scientific evidence;
- producer-derived expected values standing in for independent verification;
- legacy parity standing in for physical validity;
- conservation standing in for empirical performance, or empirical fit standing
  in for code and solution verification;
- calibration and evaluation on the same effective data;
- a favorable aggregate metric or plot hiding failed regimes or quantities;
- rerunning until green without retaining and explaining material failures;
- carrying evidence to a changed model, output, or use without impact review;
  or
- calling missing evidence not applicable merely because it is expensive.

## Research Basis

This strategy adapts practices from high-consequence and environmental modeling
without adopting a nuclear licensing framework or claiming that one level of
scrutiny can remove open-system uncertainty:

- [NRC Regulatory Guide 1.203](https://www.nrc.gov/docs/ML0535/ML053500170.pdf)
  provides a precedent for declared assessment requirements, graded rigor,
  configuration identity, applicability, uncertainty, and review. openWEPP uses
  that discipline for closable verification obligations; it does not treat a
  developer's empirical evidence characterization as the equivalent of a
  regulator's authorization for a defined facility and operating envelope.
- [NASA-STD-7009B](https://standards.nasa.gov/standard/NASA/NASA-STD-7009) and
  [ASME V&V
  20](https://www.asme.org/codes-standards/find-codes-standards/standard-for-verification-and-validation-in-computational-fluid-dynamics-and-heat-transfer)
  support distinct code and solution verification, quantity-specific empirical
  comparisons, uncertainty, data pedigree, permissible-use boundaries, and
  decision reporting.
- [Oreskes, Shrader-Frechette, and Belitz
  (1994)](https://doi.org/10.1126/science.263.5147.641) distinguish verification
  of closed mathematical components from the necessarily partial confirmation
  of models of open natural systems. This strategy uses **corroboration** for
  the latter so successful comparisons are not mistaken for proof or terminal
  application fitness.
- [EPA environmental-model
  guidance](https://www.epa.gov/sites/production/files/2015-04/documents/cred_guidance_0309.pdf)
  supports contextual model application, transparent evaluation, calibration
  independence, uncertainty analysis, peer review, and continuing post-audit.
- [Sandia's V&V and predictive-capability
  report](https://doi.org/10.2172/809603) supports hierarchical evidence,
  process-importance ranking, and numerical-error assessment.
- [Nearing's erosion-model evaluation
  method](https://doi.org/10.1002/1096-9837%28200008%2925%3A9%3C1035%3A%3AAID-ESP121%3E3.0.CO%3B2-B)
  interprets error relative to observed erosion variability.
- [Wang et al.'s multi-regime WEPP
  evaluation](https://doi.org/10.1016/j.iswcr.2022.10.004) demonstrates
  cross-regime datasets, calibrated and uncalibrated results, multiple temporal
  scales, and explicit limitations at event extremes.
- [W3C PROV-O](https://www.w3.org/TR/prov-o/) and the [RO-Crate
  specification](https://www.researchobject.org/ro-crate/specification.html)
  remain optional future provenance references.

Repository bibliography entries `R-114` through `R-125` record these sources,
their roles, and their rights posture.
