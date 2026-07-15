# Scientific Model-Evaluation Report Standard

Status: active v2 standard

Audience: scientific authors, reviewers, maintainers, and publication builders

## Purpose

An openWEPP scientific model-evaluation report presents a bounded study in the
form expected by hydrologists, soil scientists, researchers, and practitioners.
It explains the scientific question, formulation, data or referent, methods,
quantitative results, interpretation, limitations, and conclusion. It lets a
reader understand the study before descending into internal traceability.

The report is not a compliance scorecard, issue tracker, test manifest, or
application license. It may report strong, mixed, negative, or narrowly limited
results, but it must contain an actual study. The mere absence of an evaluation
belongs in the portfolio gap record or model narrative, not in a status-first
public report.

This standard is governed by the
[v2 architecture](../governance/scientific-assurance-v2-architecture.md),
[lifecycle contract](../governance/scientific-assurance-dossier-lifecycle.md),
and [source/build contract](../governance/scientific-assurance-v2-source-build-contract.md).

## Report Scope

One report addresses one coherent scientific question or a tightly related set
whose quantities, scales, domains, referents, and conclusions can be understood
together. Split a report when combining claims would hide materially different:

- processes or quantities of interest;
- spatial or temporal scales;
- observation or benchmark datasets;
- calibration and evaluation partitions;
- forcing regimes;
- software realizations;
- methods, metrics, or uncertainty treatments; or
- conclusions and application limitations.

The title names the process or quantity and evaluation question. It does not use
an aggregate lifecycle grade such as “candidate,” “supported,” or “insufficient
evidence” as the scientific headline.

## Required Manuscript Structure

The main report uses this recognizable sequence, adapted when the study type
requires it:

1. **Title and authorship** — specific title; accountable human author or report
   lead; contributors and disclosed agent assistance.
2. **Key findings** — one to three complete, quantitative where useful,
   claim-bounded statements.
3. **Plain-language summary** — purpose, principal result, and practical limit
   without internal governance jargon.
4. **Abstract** — question, method, principal quantitative results, and bounded
   conclusion.
5. **Introduction** — scientific motivation, prior knowledge, gap, and study
   objective.
6. **Model formulation or conceptual basis** — equations, assumptions, process
   boundaries, scale, and relation to established literature and openWEPP
   authority.
7. **Data, referents, and methods** — assessed software, datasets or analytical
   referents, admission criteria, partitions, forcing, metrics, uncertainty,
   comparators, and reproduction method.
8. **Results** — inspectable quantitative findings with the smallest sufficient
   tables and figures, including contrary or negative results.
9. **Discussion** — meaning, mechanisms, prior-knowledge comparison,
   sensitivity, uncertainty, and alternative explanations.
10. **Limitations** — material boundaries and missing evidence stated in domain
    language.
11. **Conclusions** — direct answer to the study question without expanding the
    claim envelope.
12. **Open research and reproduction** — data, software, configuration,
    retained outputs, procedures, access, and licensing.
13. **References** — scientific literature plus formal dataset and software
    citations.
14. **About this report** — assessed realization, report version, approval date,
    reviewers, supersession, and release transfer. This metadata is visible but
    does not lead the report.

The main report retains the interpretation and limitations needed to understand
the conclusion. Detailed branch matrices, full parameter tables, extended
methods, dependency manifests, and additional diagnostics may move to the
technical supplement.

## Study-Type Requirements

### Formulation or conceptual evaluation

Identify the scientific theory, literature, assumptions, alternatives, and
openWEPP contract. State whether evidence establishes plausibility, authority,
or comparative preference; do not imply that conceptual support proves the
implementation or predictive accuracy.

### Code verification

Name the specified equations or logic and show how the implementation was
tested against an independent analytical solution, manufactured solution,
hand calculation, property, or separately implemented oracle. Regression to the
same producer output is not independent evidence.

### Numerical-solution verification

Describe the numerical method, resolution variables, convergence or stability
study, expected order or property, acceptance tolerance, and observed result.
Conservation closure alone does not quantify discretization error.

### Integration and consumer verification

Trace the claim-bearing quantity through the real production producer,
serialization or boundary, consumer, and public result. Producer-only,
skeleton, or shadow evidence cannot close a downstream claim.

### Empirical model evaluation

Define observations, representativeness, quality controls, censoring, sample
counts, spatial and temporal alignment, calibration/evaluation separation,
forcing, uncertainty, metrics, diagnostics, and prior thresholds. Use graphical
and quantitative evidence selected for the quantity; do not rely on one
universal goodness-of-fit score.

If observations influenced parameter selection, mechanism design, threshold
choice, or debugging, disclose that role. An evaluation set must be independent
of those choices to support an independent corroboration claim.

### Comparative evaluation

State why the comparator is scientifically relevant and whether it is an
independent referent, alternate implementation, prior release, or diagnostic
flag. Under ADR-0017, legacy agreement alone is not truth. Report where models
agree and disagree and what the comparison can establish.

### Release-transfer evaluation

Bind the study to an exact software realization, configuration, dependencies,
and output identities. A static unchanged-path check is useful currency evidence
but does not replace a fresh release reproduction when the conclusion requires
one.

## Claim Envelope

Each material conclusion makes these elements recoverable from the report or
its supplement:

| Element | Required question |
| --- | --- |
| Process and quantity | What behavior or output was evaluated, in what units? |
| Scale | At what temporal and spatial support? |
| Domain | Which climate, soil, topography, management, state, or numerical regime? |
| Realization | Which software, configuration, parameterization, and dependency versions? |
| Referent | Analytical solution, theory, observation, experiment, comparator, or invariant? |
| Method | How were values aligned, calculated, compared, and reviewed? |
| Result | What quantitative and qualitative evidence was observed? |
| Uncertainty | Which measurement, forcing, parameter, numerical, and model-form uncertainties matter? |
| Limitation | What cannot be inferred or transferred? |
| Application boundary | Who must decide whether this evidence is adequate for a named use? |

No report-level aggregate may erase these dimensions. Verification, empirical
corroboration, comparison, release transfer, and application judgment remain
separate in prose and tables.

## Quantitative Reporting

Every claim-bearing value has:

- a stable result identity and retained source;
- quantity name, units, sign convention, temporal/spatial support, and
  aggregation rule;
- sample or realization count where applicable;
- precision justified by the evidence rather than raw floating-point output;
- uncertainty or an explicit explanation of why only numerical reconstruction
  error is reported;
- method and software identity; and
- enough intermediate operands for an independent reconstruction when
  conservation or transformation is claimed.

Report absolute and relative measures where either alone can mislead. Pair
summary metrics with diagnostics that reveal bias, regimes, residual structure,
timing, or outliers. Do not hide failed cases in an aggregate.

## Tables And Figures

Tables and figures answer scientific questions; they are not decoration.

- Every table has a title, labeled columns, units, and definitions needed to
  read it.
- Every result-bearing figure has a caption stating the quantity, domain,
  method, and scientific point.
- Source rows or figure data and the generation procedure are retained and
  identified.
- Color is not the sole carrier of meaning; text alternatives and source tables
  are available.
- Axes, transformations, censoring, aggregation, uncertainty intervals, and
  sample counts are visible.
- Conceptual diagrams are labeled as explanatory and do not masquerade as
  quantitative results.

## Limitations And Negative Evidence

Limitations receive a normal scientific section, not a lifecycle banner. State
the limitations most likely to change interpretation or transfer, including:

- untested processes, regimes, quantities, or scales;
- observational quality and representativeness;
- calibration leakage or parameter non-identifiability;
- forcing and input uncertainty;
- numerical resolution and tolerance limits;
- implementation or release-transfer gaps;
- contrary, failed, or sensitivity cases; and
- consequences of extrapolation.

A negative result remains visible and can be the principal finding. An absent
study is not inflated into a negative result.

## Reproduction And Availability

The report and supplement identify:

- exact openWEPP realization and configuration;
- admitted datasets, versions, access conditions, and licenses;
- input and processed-data identities;
- analysis and figure procedures;
- retained result objects and expected hashes or semantic checks;
- commands and environment needed to reproduce the study;
- known nondeterminism, computational cost, and platform limits; and
- restrictions that prevent public redistribution.

“Available on request” is not sufficient for project-owned public evidence.
External restrictions are disclosed and handled through an approved review path.
Temporary run directories are not preservation repositories.

Each published report links a version-bound public research-object manifest.
All safely redistributable project-owned claim-bearing data, table rows, figure
data, analysis/figure procedures, software and configuration identities, and
reproduction material listed above must resolve there. Protected evidence,
review locks, and build internals remain in the internal machine bundle; the
public manifest names the restriction and review path without exposing protected
content. Publication review fails if a required safe object is absent or stale.

## Review Standard

Before publication, every report receives:

1. scientific review of question, formulation, data, method, uncertainty,
   interpretation, limitations, and conclusion;
2. independent reproduction or reconstruction of material quantitative claims;
3. publication review for audience fit, cross-references, accessibility, and
   open-research completeness; and
4. named human approval bound to the reviewed source root.

Review records state competence, independence, charge, findings, disposition,
and exact reviewed identity. Coding-agent review is disclosed as internal and
is never labeled external peer review.

## Prohibited Patterns

- Leading with `CANDIDATE`, `PASS`, `INSUFFICIENT_EVIDENCE`, test counts, or an
  aggregate status instead of the scientific question and findings.
- Calling the model or subsystem “validated” without the complete claim
  envelope.
- Publishing an empty evidence inventory as a scientific report.
- Treating code verification, conservation, legacy agreement, or reviewer count
  as empirical accuracy.
- Hiding interpretation or material limitations exclusively in a supplement or
  machine manifest.
- Hand-copying claim-bearing numbers into multiple independent sources.
- Allowing a builder or agent to select methods, interpret results, approve
  science, or decide application fitness.

## Minimum Useful Publication Test

A report is publishable only when an informed domain reader can answer, from the
main report:

1. Why was this study needed?
2. What exact process, quantity, scale, domain, and realization were assessed?
3. What referent and method were used, and why are they appropriate?
4. What quantitative results were observed?
5. What do the results mean in relation to prior knowledge?
6. What important evidence contradicts or limits the conclusion?
7. What may and may not be inferred for another application?
8. Where can the study be reproduced or challenged?

If the reader must decode internal governance vocabulary to answer these
questions, the report is not ready.
