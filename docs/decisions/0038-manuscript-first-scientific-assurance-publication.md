# ADR-0038: Manuscript-First Scientific Assurance Publication

Status: **Accepted**

Date: 2026-07-14 UTC

Deciders: openWEPP maintainer/user

Accepted: 2026-07-14 through the explicit instruction to scaffold and execute
ASSURE-03

## Context

openWEPP's first assurance vertical slice proved that typed manifests,
deterministic rendering, dependency checks, review locks, and release snapshots
could be built. Its public SNOTEL candidate nevertheless failed the primary
communication objective. It led with internal lifecycle and aggregate evidence
labels, exposed candidate material in the public tree, and presented too little
of the scientific argument a hydrologist or soil scientist would expect.

The failure was architectural, not a scientific reassessment of the snow/frost
model. A status-first record encouraged readers to treat incomplete dossier
machinery as the conclusion while obscuring extensive formulation, evaluation,
and implementation evidence elsewhere in the repository.

Environmental-model evidence also differs from a nuclear licensing decision.
Software and numerical verification claims can terminate in pass or fail
against specified requirements. Empirical support for an open natural system is
conditional on quantity, scale, domain, observations, forcing, uncertainty, and
application. The project should equip a decision owner rather than issue a
universal fitness verdict.

## Decision

1. The primary public assurance product is a **scientific model-evaluation
   report** written in a conventional manuscript structure. Scientific question,
   method, quantitative results, discussion, limitations, and conclusions lead.
2. A public technical supplement holds detailed methods, traceability, and
   reproduction information that would interrupt the manuscript. It is
   subordinate to, reviewed with, and versioned with the report.
3. An internal machine assurance bundle holds typed identities, dependencies,
   protected source data, build records, reviews, and snapshots. It supports the
   report but is not the public argument and cannot choose methods or
   conclusions. Each report also publishes a version-bound research-object
   surface containing all safely redistributable project-owned claim-bearing
   data, procedures, and reproduction material.
4. Model-science narratives explain the broader “why” and “how”; reports answer
   a bounded “what did this evaluation show?” Each cross-references the other
   without duplicating claim-bearing results.
5. Application fitness belongs to the named user or institution. A report may
   describe tested domains and limitations but does not authorize a site or
   decision.
6. Drafts and review candidates build only to staging. Public `usersum` and
   release snapshots contain approved reports only.
7. An evidence absence is recorded in the assurance portfolio or model
   narrative. It does not by itself justify a public report. Negative, mixed, or
   limited findings remain publishable when a real study and scientific argument
   exist.
8. Manuscripts are hand-authored before schemas or renderers. Later tooling may
   mechanize identities, values, tables, figures, dependencies, checks, and
   assembly; it may not generate scientific interpretation during an ordinary
   build.
9. Agent assistance is optional, disclosed, reproducible through a versioned
   procedure and retained inputs/outputs, and subordinate to accountable human
   authorship and independent review.

## Consequences

- The v1 SNOTEL candidate cannot be promoted, cited as openWEPP's snow/frost
  assessment, snapshotted for release, or vendored. ASSURE-03 removes it from
  active/public surfaces while preserving exact historical provenance.
- The public assurance catalog may truthfully contain zero reports until the
  first report completes scientific, reproduction, publication, and approval
  review.
- Existing v1 compiler and schema code remains historical engineering evidence;
  it is not the v2 architecture and will not constrain the v2 manuscript.
- A later compiler will be smaller: it plans dependencies, resolves retained
  values and figures, checks review locks and drift, builds staging/public
  outputs, and snapshots approved reports.
- `cargo nextest` may test compiler and reproduction behavior, but it is not the
  report dependency engine or scientific adjudicator.
- The user satisfied the terminal ASSURE-02 acceptance gate by explicitly
  directing ASSURE-03 execution. Acceptance does not approve a report,
  reassess snow/frost science, authorize vendoring, or declare a release ready.
