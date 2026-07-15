# Review B — Architecture, Lifecycle, And Build Integrity

Review class: internal coding-agent review; not external scientific peer review

Evidence class: Static + Ran

Verdict: **HOLD**

Static review covered the package and deliverable map; ADR-0038; the v2
architecture, lifecycle, source/build contract, report standard, V&V strategy,
migration and implementation roadmaps; the prototype and claim matrix; current
v1 `assurance/` and `usersum/assurance/`; release scripts, workflow, and tests;
and documentation/catalog surfaces.

Ran:

- `markdown-doc lint` and `markdown-doc validate` passed for the scoped
  canonical documents and ten package files.
- `git diff --check` passed.
- 140 changed-Markdown local-link occurrences resolved.

The reviewer made no workspace edits.

## Findings

### B-001 — High — Release-transition hold is prose-only

ADR-0038 forbids release snapshotting
(`docs/decisions/0038-manuscript-first-scientific-assurance-publication.md:63-77`),
and the runbook says release assembly is held
(`docs/governance/openwepp-release-procedure-draft.md:11-15,155-181`), but the
runbook still calls the automation authoritative and advertises commands that
unconditionally execute `openwepp-assurance build --all --snapshot`
(`tools/release/run_release_candidate_gates.sh:487-512`). PR/push CI invokes
that script and uploads the result as `openwepp-release-candidate-*`
(`.github/workflows/release-gates.yml:99-145`). The live catalog is `candidate`,
and the v1 compiler permits candidate snapshots. This is an actual conflicting
consumer, not a theoretical route.

Requested remediation: amend scope and split validation-only CI from release
assembly, or add an equivalent fail-closed transition guard. Normal PR
validation may omit snapshot creation but must not label or upload a release
candidate; release mode must fail while the v1 candidate or transition marker
exists. Add a negative test, update `tools/release/README.md` and the runbook,
and preserve ASSURE-03 as the package that installs the zero-report release
path. If this cannot be fixed in-package, remove the transitional-release-
safety closure claim and hold on a named technical blocker.

### B-002 — High — Review-lock renewal authority and independence are unclear

The source/build contract lets an undefined “impact owner” choose no scientific
impact or rereview
(`docs/governance/scientific-assurance-v2-source-build-contract.md:74-77`) and
says any bound change stays invalid only until a dependency-impact decision
says review need not repeat (`:125-137`). The lifecycle assigns reviews but
permits role combination except sole author/sole scientific approver
(`docs/governance/scientific-assurance-dossier-lifecycle.md:20-35,132-148`);
the architecture repeats that narrow incompatibility
(`docs/governance/scientific-assurance-v2-architecture.md:87-103`). A report or
method producer could therefore waive rereview or act as the sole reproduction
reviewer, after which the builder merely verifies the record.

Requested remediation: define a role-compatibility and impact-decision matrix.
A report lead, dataset/method/result producer, or build maintainer cannot be
the sole reproduction approver or sole material-change waiver authority.
Editorial-only changes may use a publication-review disposition; changes to
claims, methods, data, results, figures, software realization, or authority
require a new root and approval by affected independent scientific and
reproduction reviewers plus the assurance steward, or full rereview. Bind
signed decisions and independence attestations to old/new roots and make the
builder fail closed.

### B-003 — High — Synthetic fixture crosses the tracked public boundary

ASSURE-04D names public `usersum` as the consumer, uses a “synthetic approved
fixture,” then rolls back by withdrawing that generated public fixture
(`docs/planning/scientific-assurance-v2-implementation-roadmap.md:85-102`).
Public discovery is restricted to genuinely approved/published reports
(`docs/governance/scientific-assurance-v2-architecture.md:53-62` and
`docs/governance/scientific-assurance-dossier-lifecycle.md:37-51,89-101`). A
mechanically approved synthetic record is not a scientifically approved report.

Requested remediation: prove the publication/layout/catalog consumer in a
confined temporary fixture root with a zero-report tracked catalog and an
explicit negative assertion that tracked `usersum` remains unchanged. Implement
the production promotion path but do not execute it against tracked public
content until ASSURE-05 supplies a genuinely reviewed report. Mark fixture
snapshots test-only and prohibit them from release snapshots.

### B-004 — Medium — Authority switches before the acceptance gate

ADR-0038 remains proposed and becomes accepted only after human acceptance
(`docs/decisions/0038-manuscript-first-scientific-assurance-publication.md:3,76-77`),
but the existing v1 standard is already labeled retired and says new work must
use the proposed v2 standard (`docs/standards/scientific-assurance-dossier.md:3-13`),
while the standards index says standards are normative
(`docs/standards/README.md:16-26`). If ASSURE-02 closes as
`EXECUTED-HOLD-USER-ACCEPTANCE`, v1 would be retired while v2 remains
unaccepted.

Requested remediation: label v1 retirement proposed or frozen pending
acceptance and state a no-new-public-v1-authoring moratorium during the hold.
Atomically flip ADR-0038 to accepted, the v2 standard to active, and the v1
standard to retired only in the recorded human-acceptance disposition.

### B-005 — Medium — V1 retirement inventory omits active consumers

The manifest list
(`docs/planning/scientific-assurance-v2-migration-plan.md:17-36`) omits at least
`assurance/README.md`, which still declares v1 canonical and instructs rebuilding
public output (`assurance/README.md:3-24`), and does not explicitly classify
`usersum/README.md`, `.github/workflows/release-gates.yml`,
`tools/release/README.md`, `Cargo.toml`, `Cargo.lock`, or
`tests/integration/assurance_dossier_build_contract.rs`. Current test names and
assertions treat the candidate as the current public vertical slice. Broad
“links/scripts” wording is not enough for an exact inventory.

Requested remediation: add a path-by-path inventory with keep/update/remove
action, preservation identity, and zero-report negative proof for all source,
generated, navigation, compiler/workspace, test, release-script, workflow, and
dormant-export consumers. Distinguish retained v1 engineering code used for
empty-catalog migration from retired active/public data.

### B-006 — Medium — Active package is absent from the catalog

The package declares `docs/work-packages/README.md` in its write set
(`package.md:78-97`) and ROADMAP marks ASSURE-02 active, but the catalog's
current active/held section has no entry for this package.

Requested remediation: add an active catalog entry naming the package,
documentation-only scope, current review state, and user-acceptance hold
boundary; on closure, transition it to the exact terminal disposition.

## Positive Assessment

The four-record boundary, canonical source/staging/public separation, one/all
dependency planning, nextest boundary, agent packet, zero-report goal,
ASSURE-03/04A-D/05 sequencing, prototype claim discipline, and rollback
direction are otherwise coherent. Rust and CRAP gates remain not applicable to
the current documentation-only write set.
