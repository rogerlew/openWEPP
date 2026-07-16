# Scientific Assurance V2 Internal Sources

Status: ASSURE-04C deterministic staging assembly implemented; nonpublic

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
  schemas/
    catalog.schema.json
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

ASSURE-04D owns review locks, promotion, public catalogs, snapshots, and release
transfer.
Public scientific communication remains under `usersum/` and continues to
contain zero assurance reports until an approved report is promoted. Vendoring
remains deferred until the openWEPP beta release campaign.
