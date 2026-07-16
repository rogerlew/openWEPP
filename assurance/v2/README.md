# Scientific Assurance V2 Internal Sources

Status: ASSURE-04D review and external publication mechanics implemented; zero public reports

This tree holds canonical scientific-assurance source, not generated reader
documentation. The manuscript and supplement carry the scientific argument in
reader-first prose. Strict YAML records identify authorship, accountable review
roles, agent assistance, claims, methods, dependencies, results, value
bindings, tables, result-bearing figures, references, research objects, review
state, and publication state. They do not generate conclusions or substitute
lifecycle labels for evidence. The current architecture fixture discloses its
agent author and blocks review while its human report lead and scientific
approver are unassigned.

The current groundwater report is a positive architecture fixture derived from
the accepted ASSURE-02 manuscript prototype. Its `DRAFT` and `fixture_only`
fields are governance controls, not a reader-facing assessment of the science.
It has no public route, review lock, export permission, release snapshot, or
vendoring authority.

## Layout

```text
assurance/v2/
  catalog.yaml
  principals.yaml
  schemas/
    catalog.schema.json
    principals.schema.json
    report.schema.json
    result.schema.json
  reports/<report-id>/
    report.yaml
    manuscript.md
    supplement.md
    results/*.json
```

Every local claim-bearing file is a confined, regular, non-symlink repository
path bound by SHA-256. External evidence uses immutable identities. Restricted
evidence must disclose its restriction and review role without exposing a local
protected path or content digest.

## Validate

From the repository root:

```bash
cargo run --quiet -p openwepp-assurance -- validate --all
cargo run --quiet -p openwepp-assurance -- validate \
  --report linear-groundwater-reservoir-recurrence
```

Validation is deterministic and offline. It checks schema and contract
versions, content identities, logical-reference closure, units, unused
declarations, restrictions, and draft lifecycle consistency. It does not plan,
render, approve, publish, or scientifically reevaluate a report.

## Plan

From the repository root:

```bash
cargo run --quiet -p openwepp-assurance -- plan --all
cargo run --quiet -p openwepp-assurance -- plan \
  --report linear-groundwater-reservoir-recurrence
cargo run --quiet -p openwepp-assurance -- plan --all --format json
```

Planning is deterministic, offline, and read-only. It emits the same typed
graph as human-readable text or JSON. Each node is `current` when its declared
identity and prerequisites are current, `stale` when observed bytes differ from
the declared SHA-256, `blocked` when required local content is unavailable, or
`selected` when a changed prerequisite transitively affects it. These words
describe build impact, not scientific merit or fitness for an application.

The planner orders dependencies before consumers, rejects cycles and missing
or unused logical records, ignores file modification time, and plans a named
report without traversing unrelated reports. It does not write files, update
hashes, render prose, decide whether scientific rereview is necessary, or
approve a report.

## Assemble And Check A Disposable Consumer

Assembly always requires a caller-selected disposable staging root. These
commands build the internal fixture into a future
`usersum/assurance/reports/` shape and then verify every expected byte and local
link without writing during the check:

```bash
stage="$(mktemp -d)"
mkdir -p "$stage/usersum"
cp usersum/hillslope-hydrology-and-sediment-physics.md "$stage/usersum/"
cargo run --quiet -p openwepp-assurance -- build --all \
  --staging-root "$stage"
cargo run --quiet -p openwepp-assurance -- check --all \
  --staging-root "$stage"
```

`build --report <id>` and `check --report <id>` use the same per-report
assembler as `--all`. The assembler copies authored literal prose, resolves
only the small typed directive vocabulary, renders declared values/tables/SVG
figures from identified result objects, copies public-safe research objects,
and emits portable links. Unit disagreement, unsupported display precision,
unused content, noncurrent inputs, unresolved links, output drift, unsafe
staging paths, or symlink traversal fail closed.

The staging result is architecture evidence, not a reviewed or published
scientific report. It grants no review lock, public route, snapshot, export,
release, or vendoring authority.

## Review And Approval Boundary

The authored lifecycle is `DRAFT` → `IN_REVIEW` → `APPROVED`. There is no
authored `PUBLISHED` state. A domain-separated subject root binds the scientific
source and exact staged output; a finding-ledger root then binds review
findings; an approval-lock root binds three exact-ledger approval declarations;
and a release-transfer root binds the approved lock to an independently
supplied commit and release configuration. A verified immutable receipt is the
only authority for the derived public state.

The registry in `principals.yaml` gives stable identities, kinds, authorities,
and eligible roles. The software checks structure, declared roles, distinct
principals, and specified conflicts. It cannot authenticate a person, assess
competence, perform scientific review, or generate approval. The current
fixture is intentionally `DRAFT` and `test_only`; its missing human
accountability blocks publication.

## Publish To Explicit External Roots

Production publication is available only for a source already carrying valid
production-domain approvals and release transfer. It consumes exact checked
04C staging bytes and requires three pairwise unrelated external roots:

```bash
cargo run --quiet -p openwepp-assurance -- publish --report <report-id> \
  --staging-root /absolute/stage \
  --usersum-root /absolute/public-usersum \
  --publication-snapshot-root /absolute/snapshots \
  --release-commit <full-40-character-git-object-id> \
  --release-configuration <stable-configuration-id>
```

The `publish-test-fixture` command is a separate synthetic-only entry point.
Its source, report, catalog, snapshot, and receipt must all remain in the
`test_only` trust domain and visibly carry `TEST ONLY — NOT SCIENTIFICALLY
APPROVED`. The production release verifier rejects these artifacts.

Publication atomically replaces the complete owned `assurance/` generation in
the external usersum-shaped root. It also installs a content-addressed snapshot
and receipt with no-replace semantics. Repeating identical publication confirms
the existing immutable artifacts; conflicting bytes fail closed. The tracked
repository `usersum`, export, release, and vendor trees are rejected as
destinations.

Release verification receives the expected release identity independently:

```bash
cargo run --quiet -p openwepp-assurance -- verify-release --all \
  --snapshot-dir /absolute/snapshots/<snapshot-id> \
  --receipt /absolute/snapshots/receipts/<receipt-id>.json \
  --release-commit <full-40-character-git-object-id> \
  --release-configuration <stable-configuration-id>
```

The openWEPP release-candidate preflight can consume the same complete artifact
set. It retains the existing zero-report path when no v2 artifacts are
supplied. Public scientific communication under tracked `usersum/` continues
to contain zero assurance reports. WEPPcloud discovery and vendoring remain
deferred until the beta release campaign.
