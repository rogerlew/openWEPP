# Scientific Assurance V2 Architecture

Status: proposed — ASSURE-02 acceptance gate

Owner: openWEPP maintainers and the designated scientific-assurance steward

## Purpose

The architecture exists to make openWEPP's scientific basis and evaluated
behavior understandable, inspectable, reproducible, and challengeable by
hydrologists, soil scientists, researchers, and practitioners. The reader should
encounter the science first and be able to follow every important result back to
its method, data, software realization, and review.

The architecture does not manufacture confidence by grading incomplete records.
It connects a recognizable scientific report to progressively more detailed
evidence.

## Four Distinct Records

| Record | Primary question | Audience | Publication posture |
| --- | --- | --- | --- |
| Scientific model-evaluation report | What was evaluated, why, how, what did it show, and what does it not show? | Hydrologists, soil scientists, researchers, practitioners | Public only after approval |
| Technical assurance supplement | Exactly which methods, inputs, metrics, identities, branches, and reproduction steps support the report? | Scientific reviewers, reproducers, advanced users | Public with its report |
| Machine assurance bundle | Which source objects, dependencies, values, figures, reviews, and snapshots produced the approved publication? | Build tools, maintainers, audit agents | Internal; protected evidence, locks, and build records stay here |
| Application assessment | Is this evidence adequate for one named site, decision, accuracy need, and consequence of error? | Named user or institution | Owned outside the report; public only by its owner |

These records must not collapse into one page. In particular, machine lifecycle
state and aggregate test status do not lead the scientific report, while the
report cannot hide claim-bearing methods or limitations solely in machine data.

### Public research-object surface

Each published report has a mandatory, version-bound public research-object
surface. It is a supporting publication surface rather than a fifth assessment
record. It contains all safely redistributable project-owned claim-bearing
data, table rows, figure data, analysis and figure procedures, software and
configuration identities, and reproduction instructions required by the
report. The report and supplement link to it directly.

The data or method steward owns the scientific objects; the report lead decides
which objects support each claim; the reproduction/publication reviewer checks
completeness; and the assurance build maintainer publishes only the approved
manifest. Protected or externally restricted evidence remains in the internal
bundle with its restriction and review path. Public omission is permitted only
for a recorded restriction, never merely because the machine bundle exists.

## Relationship To Existing Documentation

### Model-science narratives

Canonical `usersum` narratives explain why a process is represented, how the
formulation works, how it interacts with other processes, and what users should
understand before interpreting outputs. They are durable model documentation,
not frozen evaluation results.

A narrative links to every approved report that materially evaluates its
process. A report links back to the relevant narrative and identifies the
applicable science contracts by stable ID and version. The supplement or release
snapshot supplies an inspectable portable contract reference; generated
`usersum` pages never contain a relative link into unavailable contributor
documentation. Claim-bearing result tables live in the report and its retained
result objects, not duplicated in the narrative.

### Science contracts

`SC-*` contracts remain normative process and implementation authority. A
report translates the relevant formulation and boundaries into scientific prose
and cites the contract version used. Evidence does not amend a contract; a
scientific finding that challenges authority opens separate contract work.

### Public catalogs

The `usersum` documentation catalog owns discovery. It lists only approved,
published reports and links each to related model narratives. Drafts, review
candidates, withdrawn reports, internal bundles, and empty placeholders are not
public catalog entries.

The assurance catalog may contain zero reports. In that state it explains that
reports are under development and routes readers to model narratives and
ordinary limitations without publishing a failure grade.

## Reader Path

The intended path is:

```text
usersum model narrative
        |
        +--> approved scientific report
                 |
                 +--> technical supplement
                 +--> required public research objects and reproduction
                 +--> portable science-contract references

named application decision
        ^
        |
        +-- report evidence + local context supplied by decision owner
```

A reader can stop after the report and still understand the scientific
conclusion. A reviewer can descend into the supplement and retained objects. A
machine can verify identities without becoming the author of the conclusion.

## Ownership

| Responsibility | Accountable role | Required independence |
| --- | --- | --- |
| Model rationale and science-contract consistency | Process science owner | Domain review proportionate to change |
| Report question, methods, interpretation, and conclusion | Report lead / scientific assessment owner | Cannot self-approve scientific publication |
| Dataset admission and method implementation | Data/method steward | Reproduction reviewer must not be the sole producer |
| Public research-object completeness | Report lead and data/method steward | Independent reproduction/publication review required |
| Dependency graph, deterministic build, and snapshots | Assurance build maintainer | Cannot approve scientific conclusions by build success |
| Scientific review | Scientific reviewer | Independent of report authorship; domain competence recorded |
| Reproduction and publication review | Reproduction/publication reviewer | Independent execution or reconstruction required for claimed results |
| Public inclusion and release transfer | Assurance steward and release owner | Both scientific approval and exact build/release identity required |
| Application fitness | Named user or institution | Never delegated to the report builder or generic project status |

One person may hold multiple maintenance roles in a small project, but the
following incompatibilities are hard boundaries:

- a report lead or material claim, dataset, method, result, table, or figure
  producer cannot be the sole scientific approver;
- a report lead, material dataset/method/result producer, or assurance build
  maintainer cannot be the sole reproduction approver for that material; and
- no producer or build maintainer may be the sole authority waiving review
  after a material dependency change.

If the required independent person is unavailable, the report remains in
review; resource scarcity does not convert self-review into independence.
Coding-agent review is labeled as internal review, not external peer review.

## Evidence And Conclusion Boundaries

Every material conclusion names its process, quantity, temporal and spatial
scale, tested domain, software realization, evidence type, and limitation. The
report keeps these evidence dimensions separate:

- formulation and conceptual basis;
- code and numerical verification;
- integration and downstream-consumer verification;
- empirical corroboration against observations;
- comparative evidence against another implementation;
- transfer to the current release realization; and
- application assessment.

Evidence can be strong in one dimension and absent in another. A code-
verification report is useful when it answers a real implementation question;
it does not need an “insufficient empirical evidence” headline for a claim it
did not make.

## Build Boundary

The future build system may:

- validate structure and typed identities;
- resolve approved values, tables, and figures from retained result objects;
- plan one or all affected report builds from declared dependencies;
- detect missing, changed, stale, or unused dependencies;
- enforce staging/public separation and review locks;
- render deterministic report and supplement outputs; and
- create immutable release snapshots.

It may not select datasets, calibration partitions, metrics, thresholds, or
interpretations; invoke an agent or network service during an ordinary build;
rewrite scientific prose; approve a review; or issue an application verdict.

`cargo nextest` is appropriate for testing compiler units, integration behavior,
reproduction harnesses, and report-specific executable checks. Dependency
planning and incremental rebuild decisions belong to the assurance builder,
because nextest does not model manuscript/result/figure/review publication
dependencies.

## Acceptance Boundary

This architecture is not active until ASSURE-02 records both independent
reviews, their disposition, dual verification, and explicit user or named
scientific-steward acceptance. Later packages implement migration and tooling;
they do not reopen the scientific communication direction silently.

Acceptance is one atomic documentation transition: ADR-0038 becomes accepted,
the v2 report standard becomes active, and the v1 dossier standard becomes
finally retired. Until that disposition is recorded, v2 remains proposed, v1
is frozen under a no-new-public-authoring moratorium, and no assurance report
may enter the public tree.
