# Ownership And Lifecycle Matrix

Status: `complete`; terminal drift and dual verification pass.

Static: the authoritative matrices are in
`docs/governance/scientific-assurance-dossier-lifecycle.md`. This artifact is a
navigation summary, not replacement authority.

| Layer | Source owner | Generated/public consumer | Decision boundary |
| --- | --- | --- | --- |
| Why | Domain science steward | Hand-authored usersum narrative | No evidence status assignment |
| How | Evaluation method owner | Generated method page | Choices frozen prospectively or labeled retrospective |
| What | Dossier steward and scientific assessment owner | Generated dossier and catalog | Independent review required for publication |
| So what | Named application decision owner | Copyable worksheet | openWEPP never issues site fitness |
| Agent-assisted proposal | Disclosed agent plus accepting dossier steward | Canonical authoring record and dossier audit section | Independent procedural approval binds accepted outputs; no scientific authority |
| Publication mechanics | Assurance tooling maintainer | Generated pages, export, and snapshot | Separate publication review cannot impersonate scientific review |

Lifecycle is `DRAFT -> CANDIDATE -> PUBLISHED -> SUPERSEDED`, with terminal
`WITHDRAWN` from any nonterminal state. Published bytes are locked; material
changes create rereview and normally a new version. Each release records a new
immutable snapshot even when current dossier bytes are unchanged.

The scientific approval binds conclusion-bearing sources, the hand-authored
narrative, and any agent-assisted authoring record. The distinct publication
approval additionally binds schemas, templates, compiler sources, and output
paths. A material scientific edit invalidates both; a purely mechanical
renderer edit invalidates publication approval without erasing the retained
scientific history. Drafts are not snapshot-eligible. Candidate snapshots
preserve their pending review state rather than implying publication.

The canonical trigger matrix covers code, configuration, dataset,
transformation, metric, tolerance, partition, template, interpretation,
limitation, reviewer, narrative, public navigation, and release changes. It
separates rebuild, impact assessment, independent rereview, versioning, and
snapshot renewal.
