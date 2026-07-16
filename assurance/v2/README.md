# Scientific Assurance V2 Internal Sources

Status: ASSURE-04D review and external publication mechanics implemented; zero public reports

This tree holds canonical scientific-assurance source, not generated reader
documentation. The manuscript and supplement carry the scientific argument in
reader-first prose. Strict YAML records identify authorship, accountable review
roles, agent assistance, claims, methods, dependencies, results, value
bindings, tables, result-bearing figures, references, research objects, review
state, and publication state. They do not generate conclusions or substitute
lifecycle labels for evidence. The current production-domain draft discloses
its agent author and blocks review while its human report lead and scientific
approver are unassigned.

The current groundwater report is the first production-domain v2 source. It is
a manuscript-first software-verification study with preregistered methods,
fresh execution evidence, reproducible arithmetic, and explicit scientific
claim limits. `DRAFT` records its lifecycle, not a negative assessment of the
groundwater process. It has no public route, review lock, export permission,
release snapshot, or vendoring authority until accountable humans review and
approve the exact source.

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
    inputs/*.json
    procedures/*.py
    evidence/*.json
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

## Normalize Draft Prose

American English is the report language. Before a DRAFT enters review, check
its authored manuscript and supplement with the canonical converter:

```bash
cargo run --quiet -p openwepp-assurance -- normalize \
  --report linear-groundwater-reservoir-recurrence \
  --language en-US --check
```

When the check reports drift, apply the exact converter output and mechanically
rebind its content identities:

```bash
cargo run --quiet -p openwepp-assurance -- normalize \
  --report linear-groundwater-reservoir-recurrence \
  --language en-US --apply
```

The explicit maintenance command invokes the installed `uk2us` executable
without a shell. It changes only converter-produced manuscript or supplement
bytes, then updates the disclosed agent packet, report descriptor, and catalog
digests in one locked `assurance/v2` tree transaction while preserving source
permission modes. It emits a deterministic JSON receipt to standard output. It
refuses `IN_REVIEW` or `APPROVED` sources, review-entry-authorized drafts,
unsupported languages, stale inputs, ambiguous identity edges, non-idempotent
converter output, and incomplete recovery. Pre-commit and post-install
validation failures restore the old generation. If cleanup fails after the new
generation has validated, the command instead reports a distinct committed-
cleanup error containing the committed receipt and leaves the valid new
generation active. Any retained `.v2.normalize.next` directory requires
explicit operator disposition and blocks every later check or apply operation.

For an exact normalization produced by this command, use the proportional
editorial gate:

```bash
cargo nextest run --workspace --profile assurance-editorial
cargo run --quiet -p openwepp-assurance -- validate \
  --report linear-groundwater-reservoir-recurrence
```

The fast path does not classify arbitrary edits as editorial. Mixed prose,
claim, method, result, figure, lifecycle, authority, schema, or builder changes
use the ordinary package, impact-review, and full-gate process. A new content
root remains a new content root even when the scientific-impact disposition is
editorial-only. Publication integration is intentionally outside the fast
profile because the operation accepts only a pre-review DRAFT; lifecycle,
approval, builder, or publication changes must run the ordinary full gates.

The complete authoring order is:

1. Draft the manuscript and supplement while the report remains `DRAFT`.
2. Run `normalize --check`, then `normalize --apply` only when drift is reported.
3. Run `validate --report <id>` against the normalized source.
4. Run `build --report <id>` and `check --report <id>` in a disposable staging
   root using the commands below.
5. Freeze the exact source and staged-output roots for review entry.

Normalization is preparation for validation and review, not evidence that
either has occurred.

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
commands build the internal source into a future
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

The staging result is build evidence, not a reviewed or published
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
competence, perform scientific review, or generate approval. The current report
is intentionally `DRAFT` in the production trust domain; its missing human
accountability blocks review entry and publication.

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
