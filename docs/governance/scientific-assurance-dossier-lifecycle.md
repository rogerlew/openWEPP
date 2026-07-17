# Scientific Assurance Report Lifecycle And Ownership Contract

Status: active v2 lifecycle contract

Filename note: the legacy `dossier` filename is retained as a stable governance
link. In v2, the public product is a scientific model-evaluation **report**; the
complete internal report/supplement/machine package may be called an assurance
bundle or dossier.

## Purpose

This contract defines who owns scientific assurance records, how they move from
working source to public report, how dependencies trigger rebuild or rereview,
and how an approved report is bound to an openWEPP release. It implements the
[v2 architecture](scientific-assurance-v2-architecture.md) and
[report standard](../standards/scientific-model-evaluation-report.md).

## Ownership Contract

| Record or decision | Accountable owner | Obligation |
| --- | --- | --- |
| Model-science narrative | Process science owner | Keep formulation, rationale, process interactions, and report cross-links current. |
| Scientific report and conclusion | Report lead / scientific assessment owner | Define the question, method, interpretation, and claim envelope; respond to review. |
| Technical supplement | Report lead with method/data steward | Preserve sufficient method, data, and reproduction detail. |
| Dataset admission and processing | Data steward | Record provenance, quality, representativeness, partitioning, restrictions, and transformations. |
| Analysis and figure procedure | Method steward | Make claim-bearing results independently reproducible or reconstructable. |
| Public research objects | Report lead with data/method steward | Publish every safe project-owned claim-bearing object and record restrictions for protected evidence. |
| Machine bundle and build | Assurance build maintainer | Validate identities, dependencies, deterministic outputs, locks, drift, and snapshots without scientific adjudication. |
| Scientific approval | Independent scientific reviewer plus assurance steward | Evaluate the science; bind approval to exact source and evidence identities. |
| Reproduction/publication approval | Independent reproduction/publication reviewer | Reproduce material results and review audience fit, accessibility, and availability. |
| Release transfer | Release owner and assurance steward | Prove the approved report applies to the exact release realization and snapshot it. |
| Application assessment | Named user or institution | Judge adequacy for a named site, decision, accuracy need, and consequence of error. |

Roles may be combined only when independence is preserved. A report lead or
material claim/data/method/result/figure producer cannot be the sole scientific
approver. A report lead, material data/method/result producer, or build
maintainer cannot be the sole reproduction approver. None may be the sole
authority waiving rereview after a material change. If an independent reviewer
is unavailable, the report remains in review. Agent review is internal review.

## Lifecycle States

| State | Meaning | Allowed output |
| --- | --- | --- |
| `DRAFT` | Scientific question, manuscript, method, or evidence is being developed. | Source and local staging only |
| `IN_REVIEW` | A frozen source root is undergoing scientific and reproduction/publication review. | Review staging only |
| `APPROVED` | Required findings are dispositioned and named human approval binds the exact root. | Approved staging; eligible for release transfer |
| `PUBLISHED` | An approved root has passed public build, catalog, accessibility, and release-snapshot gates. | Public `usersum` and immutable release snapshot |
| `SUPERSEDED` | A named newer report version replaces the scientific record. | Historical snapshot with prominent successor link; not current catalog entry |
| `WITHDRAWN` | A material defect prevents continued reliance. | Historical notice and reason; claim-bearing content removed from current navigation |

There is no publicly inspectable `CANDIDATE` state. Drafts and reviews never
enter the public `usersum` tree, release snapshot, export manifest, or WEPPcloud
vendor surface. A repository browser may see canonical source, but generated
public navigation and release materials must not present it as a report.

## Versioning

A report has one stable ID and an immutable semantic version shared by its
manuscript and supplement. Any change to published bytes creates a new version;
snapshots are never edited in place.

- Major: changes the scientific question, claim envelope, domain, formulation,
  or method enough that prior conclusions cannot be read as the same study.
- Minor: adds or materially updates evidence, results, limitations, or release
  coverage while retaining the study identity.
- Patch: corrects presentation or metadata without changing a scientific claim.

Even a patch receives the required impact review and a new reviewed root. A
release snapshot names the exact report version it transfers.

## Lifecycle Transitions

```text
DRAFT --> IN_REVIEW --> APPROVED --> PUBLISHED --> SUPERSEDED
   ^          |             |             |
   |          v             v             v
   +-------- DRAFT       DRAFT         WITHDRAWN
```

- Enter `IN_REVIEW` only after the manuscript, supplement, dependencies,
  result-bearing assets, and review charge form a content-identified root.
- Any accepted finding that changes a bound object creates a new review root.
- `APPROVED` requires scientific review, independent reproduction or
  reconstruction, publication/accessibility review, complete disposition, and
  named human approval.
- `PUBLISHED` additionally requires deterministic public rendering, catalog and
  cross-reference checks, complete public-safe research objects, and exact
  release transfer or a clearly identified standalone publication snapshot.
- Supersession preserves old snapshots and gives readers the replacement and
  reason. Withdrawal preserves audit history without leaving defective claims
  in current navigation.

## Source, Staging, And Public Surfaces

| Surface | Purpose | Authority |
| --- | --- | --- |
| `assurance/v2/` | Future canonical manuscript, supplement, dependency, method, result, figure, review, and publication source | Human-authored scientific source plus typed identities |
| Build staging outside `usersum` | Draft, review, and approved preview | Disposable output; never public authority |
| `usersum/assurance/reports/` | Future approved report and supplement output | Generated from an approved locked root only |
| Public research-object surface | Safe data, table/figure sources, procedures, software/configuration identities, and reproduction material | Version-bound to an approved report; completeness is a publication gate |
| Release assurance snapshot | Immutable source, safe evidence, output, review, and release binding | Release-specific audit record |
| WEPPcloud vendor tree | Downstream copy during beta release gate | Never authority; exact snapshot transfer required |

ASSURE-04 may refine the source and staging paths but must preserve their
separation. It may not build drafts directly into a tracked public tree and then
rely on a label to prevent readers from treating them as publications.

## Relationship To `usersum`

The `usersum` catalog is the public discovery owner. Only `PUBLISHED` reports
appear in its assurance list, navigation, search, or generated export. Each
entry states the report title, scientific question, assessed process/quantity,
assessed realization, publication date, and related model narrative in ordinary
language.

Every relevant model narrative links to its published reports. Each report
links back to the narrative and applicable science contracts. When no reports
are published, the catalog shows a neutral zero-report state and routes readers
to model documentation; it does not publish draft grades or imply that the
underlying science has not been studied.

## Review Contract

### Scientific review

The review charge covers:

- importance and clarity of the question;
- formulation and prior-knowledge accuracy;
- dataset or referent appropriateness;
- calibration/evaluation independence;
- method and metric suitability;
- uncertainty, sensitivity, and contrary evidence;
- whether results support each conclusion;
- limitations and transfer language; and
- usefulness to the named audiences.

### Reproduction and publication review

The second review independently runs or reconstructs material results, checks
identities and units, and evaluates manuscript structure, plain language,
tables/figures, accessibility, cross-references, and open-research availability.

### Approval record

Approvals bind the report, supplement, claims, methods, datasets, software,
results, figures, public research-object manifest, references, disclosed agent
packet, and science-authority versions. Reviewer identity, competence,
independence, charge, findings, disposition, date, and exact reviewed root are
retained.

## Dependency Impact And Currency

Every material change receives one recorded disposition:

| Change class | Minimum response |
| --- | --- |
| Bibliographic attribution only | Attribution root changes; scientific and reproduction approvals survive only when every root they bind is unchanged; steward, realization, and transfer authority are recalculated or invalidated |
| Deterministic DRAFT normalization with no protected-region change | Communication root changes before review; no approval exists to carry forward; the focused receipt proves only the bounded transformation |
| Result value, table, figure, or analysis code | New root; affected independent scientific and reproduction reviewers plus assurance steward approve a bounded impact disposition or repeat full review |
| Dataset, partition, quality control, forcing, parameter, or method | New evidence root; scientific and reproduction rereview plus assurance-steward approval |
| Science contract or model formulation | New root; process-owner assessment, independent scientific/reproduction review, and new report version or supersession as indicated |
| Software realization | New root; static impact check, fresh reproduction when material, affected independent review, and new release transfer |
| Builder/schema/template | New root; determinism and semantic-diff check plus independent publication/reproduction approval; scientific review if meaning changes |
| Review or approval record | New lock; never silently backdated |
| Application context | New application assessment owned by the decision owner; does not mutate the generic report |

The build planner detects changed dependencies but does not decide scientific
impact. File timestamps are never currency authority.

The implemented
[amendment and generated-identity workflow](../specifications/assurance-amendment-and-identity-workflow.md)
mechanizes bounded attribution, role, lifecycle, and deterministic normalization
changes. Its focused receipt is sufficient only when the typed operation proves
the declared root boundary and the lifecycle matrix supplies the required human
decision. It does not classify arbitrary prose or carry an approval to a changed
bound root. Layered roots and immutable events are now the implemented lifecycle
surface; the former monolithic roots remain migration history only.

Every impact decision binds old and new roots, changed identities, change
class, rationale, reviewer roles and independence attestations, required reruns,
and resulting lock. Unclear or mixed changes require full scientific,
reproduction, and publication rereview. A producer or build maintainer cannot
unilaterally declare no impact.

## Release Transfer

A report is not automatically current for every openWEPP release. Transfer to a
release records:

- exact release commit and configuration;
- science-contract and input-schema versions;
- report, supplement, dependency, evidence, and builder identities;
- static impact analysis since the assessed realization;
- required fresh reruns or independent reconstructions;
- accepted semantic differences;
- public output and catalog hashes; and
- the immutable snapshot identity.

If transfer is incomplete, the report stays out of that release's assurance
snapshot or is clearly identified as historical—not published with an internal
`BLOCKED` cell as though the release review were a scientific result.

## Supersession And Withdrawal

Published evidence is never silently rewritten. Material scientific changes
produce a new report version or a successor report. The previous snapshot
retains its exact bytes, dates, release scope, and successor link.

A withdrawal notice states the affected claims, reason, discovery date,
decision owner, and corrective path. Public catalogs remove the withdrawn report
from current recommendations but retain an audit link where appropriate.

## Agent Assistance

Agent-assisted research, drafting, analysis, or review follows the
[source/build contract](scientific-assurance-v2-source-build-contract.md).
The report discloses material assistance in its methods or contribution record.
Agent output cannot approve science, satisfy reviewer independence by itself, or
run during an ordinary build.

## V1 Transition

The v1 SNOTEL candidate and compiler remain historical until ASSURE-03. They are
not governed into legitimacy by this contract. ASSURE-03 must:

- remove v1 candidate navigation and generated public pages;
- preserve exact v1 source, generated bytes, reviews, and build provenance in a
  nonpublic historical record;
- repair model-narrative and catalog links to a neutral zero-report state;
- prevent release snapshot, export, or vendoring of the v1 candidate; and
- leave the underlying snow/frost science and evidence intact for later v2
  synthesis.

## Mechanical Gates

The future builder enforces structure, identity, dependency completeness,
review locks, lifecycle permissions, staging/public separation, deterministic
rendering, drift, catalog consumption, accessibility metadata, and snapshots.
These gates are necessary but cannot establish that the scientific method,
interpretation, or conclusion is sound.
