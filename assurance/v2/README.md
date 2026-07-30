# Scientific Assurance V2 Internal Sources

Status: production draft sources present; zero public reports

This tree holds canonical scientific-assurance source, not generated reader
documentation. The manuscript and supplement carry the scientific argument in
reader-first prose. Strict YAML records identify authorship, accountable review
roles, agent assistance, claims, methods, dependencies, results, value
bindings, tables, result-bearing figures, references, research objects, review
state, and publication state. They do not generate conclusions or substitute
lifecycle labels for evidence.

Three production-domain sources are currently held as internal drafts:

- `linear-groundwater-reservoir-recurrence` is a manuscript-first
  software-verification study with preregistered methods, fresh execution
  evidence, reproducible arithmetic, and explicit claim limits.
- `snow-and-frozen-soil-process-evaluation` is a retrospective scientific
  synthesis of identified precipitation-phase, seasonal snowpack,
  frozen-soil, production-path, and conservation evidence. Its manuscript
  keeps the four claim envelopes separate and does not turn the diagnostic snow
  profile into a universal accuracy grade.
- `native-forest-canopy-phenology-evaluation` evaluates the daily native-forest
  canopy formulation, temperate calibration transfer, litter and residue
  readiness, downstream consumers, Southern Hemisphere robustness, and the
  retained tropical dry-forest limitation.

`DRAFT` and `IN_REVIEW` record lifecycle, not scientific merit. Both sources
disclose agent assistance and have no public route, approval lock, export
permission, release snapshot, or vendoring authority. The snow/frost source has
an accountable human report lead and is in independent review; the groundwater
source remains a draft with human accountability unassigned.

## Layout

```text
assurance/v2/
  catalog.yaml
  identity.lock.json                 # generated active generation
  principals.yaml
  schemas/
    catalog.schema.json
    principals.schema.json
    report.schema.json
    result.schema.json
    identity-lock.schema.json
    review-lock.schema.json
    review-event.schema.json
    transaction-receipt.schema.json
  reports/<report-id>/
    report.yaml
    manuscript.md
    supplement.md
    inputs/*.json
    procedures/*.py
    evidence/*.json
    results/*.json
    review-events/<event-id>.json    # immutable human-authority inputs
    review.lock.json                 # generated current layer roots
  transactions/<receipt-id>.json    # generated transition receipts
```

Authored YAML declares logical paths and semantic values, never derived file
digests or calculated roots. `identity.lock.json` binds every admitted source,
review event, and generated review lock. Each report lock separates science,
communication, attribution, governance, finding-ledger, approval, realization,
and release-transfer identities. Receipts chain the retained migration genesis
to the active generation. Hashes prove byte consistency; Git review and named
human authority establish provenance and decisions.

Every admitted file is a confined, regular, non-symlink path. External evidence
uses immutable identities. Restricted evidence discloses its restriction and
review role without exposing a protected local path or content digest.

## Validate

From the repository root:

```bash
cargo run --quiet -p openwepp-assurance -- validate --all
cargo run --quiet -p openwepp-assurance -- validate \
  --report <report-id>
```

Validation is deterministic and offline. It checks schema and contract
versions, content identities, logical-reference closure, units, unused
declarations, restrictions, and draft lifecycle consistency. It does not plan,
render, approve, publish, or scientifically reevaluate a report.

## Inspect And Amend

Inspect the current layered identity before deciding what kind of change is
needed:

```bash
target/release/openwepp-assurance inspect --report <report-id> --format human
```

Bounded attribution, principal-version, report-role, lifecycle, and American-
English normalization changes use typed `amend` or `lifecycle` commands with
exactly one of `--check` and `--apply`. The command parses structured input,
calculates complete consumers and invalidation, validates an isolated candidate,
atomically exchanges the generation, and writes one deterministic receipt. No
operator or agent edits hashes or copies roots.

Admit a complete new production-domain DRAFT only through:

```bash
target/release/openwepp-assurance amend admit-report \
  --report <report-id> \
  --path assurance/v2/reports/<report-id>/report.yaml \
  --check
```

Review the complete candidate receipt, then repeat with `--apply` and the
current optional `--if-generation` guard. Admission derives catalog metadata
from the report, validates every declared regular source in an isolated
candidate, generates the initial empty-event review lock and successor
identity, and writes one receipt. Conflicting IDs/paths, non-DRAFT state,
preexisting review locks, stale generations, symlinks, or invalid builds fail
closed. Repeating an exact admitted binding is a no-op.

Changed transactions emit schema-version 2 receipts with old and new
per-report projection roots. Historical schema-version 1 receipts remain valid
archive members.

When one DRAFT report-owned source set or already-declared external
`local_content` dependency has changed, use the source-adoption transaction:

```bash
target/release/openwepp-assurance amend adopt-report-source \
  --report <report-id> \
  --path <declared-relative-path> \
  --check
```

Apply the same command with `--apply` only after reviewing the complete
candidate. Selecting the exact conventional manifest path of a `DRAFT` report
adopts every drifted internal source already owned by that report in one
transaction. Unrelated report drift still fails closed. Dependency adoption
accepts exactly one drifted external dependency, refuses internal or undeclared
paths, returns an `IN_REVIEW` report to `DRAFT`, clears its review authority,
and invalidates its active review events. Neither form creates a human decision
or edits a source. This is a `scientific-full` transition; run the
package-selected implementation and full gates rather than the focused
report-data receipt runner.

After an applied report-data-only transaction, run its receipt-authorized gate:

```bash
.venv/bin/python tools/local_ci/run_assurance_amendment.py \
  --receipt assurance/v2/transactions/<receipt-id>.json
```

A current focused receipt is the complete local proof for that exact bounded
change. It does not authorize scientific prose changes, schema/builder changes,
approval, publication, release, export, or vendoring.

## Normalize Draft Prose

American English is the report language. Before a DRAFT enters review, check
its authored manuscript and supplement with the canonical converter:

```bash
target/release/openwepp-assurance amend normalize \
  --report <report-id> \
  --language en-US --check
```

When the check reports drift, apply the exact converter output and mechanically
rebind its content identities:

```bash
target/release/openwepp-assurance amend normalize \
  --report <report-id> \
  --language en-US --apply
```

The explicit maintenance command invokes the installed `uk2us` executable
without a shell. It changes only converter-produced manuscript or supplement
bytes, regenerates dependent reader blocks and locks, and commits one confined
transaction while preserving source permission modes. It emits a deterministic
receipt. It
refuses `IN_REVIEW` or `APPROVED` sources, review-entry-authorized drafts,
unsupported languages, stale inputs, ambiguous identity edges, non-idempotent
converter output, and incomplete recovery. Pre-commit and post-install
validation failures restore the old generation. If cleanup fails after the new
generation has validated, the command returns the committed receipt, leaves the
valid new generation active, and retains the old generation for explicit typed
recovery. Any retained recovery directory blocks every later check or apply
operation.

For an exact normalization produced by this command, use the receipt runner
shown above. `normalize` remains a one-cycle compatibility alias.

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
  --report <report-id>
cargo run --quiet -p openwepp-assurance -- plan --all --format json
```

Planning is deterministic, offline, and read-only. It emits the same typed
graph as human-readable text or JSON. Each node is `current` when its generated
identity and prerequisites are current, `stale` when observed bytes differ from
the active lock, `blocked` when required local content is unavailable, or
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
cp -a usersum/. "$stage/usersum/"
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

## Synchronize Tracked Human-Review Drafts

Accountable humans review resolved reports, not YAML descriptors and unresolved
Markdown directives. Build the assurance binary, then synchronize the full
admitted catalog into the committed review-only lane:

```bash
cargo build -p openwepp-assurance
.venv/bin/python tools/local_ci/render_assurance_review_drafts.py \
  --root . \
  --binary target/debug/openwepp-assurance \
  --apply
.venv/bin/python tools/local_ci/render_assurance_review_drafts.py \
  --root . \
  --binary target/debug/openwepp-assurance \
  --check
```

The command invokes the real `build --all` and `check --all` consumers in an
owned temporary root, then compares exact paths and bytes. `--apply` replaces
only `usersum/assurance/review-drafts/`; `--check` writes nothing and rejects
missing, extra, drifted, symlinked, or special files. The review index and every
report state explicitly say `DRAFT`.

Tracked review drafts are durable review inputs. They are not the approved
public catalog, a publication snapshot, a release transfer, an export, or a
vendored report. Only the governed human-review and release workflow may write
approved reports to `usersum/assurance/reports/`.

The `retained_svg` figure variant binds an identified public-safe SVG and
Markdown ancillary object. Assembly removes the source XML declaration,
external DOCTYPE, and non-rendering metadata, injects accessible title,
description, and image role, converts only Matplotlib's exact inert default
line-cap/join style into equivalent inherited presentation attributes, and
parses the resulting SVG before staging it. All other style elements, CSS
escapes, unsupported presentation declarations, scripts, event handlers,
external references, nonfragment links, unsafe CSS, and malformed XML fail
closed. Generated `linear_magnitude_bars` figures retain their existing
contract.

The staging result is build evidence, not a reviewed or published
scientific report. It grants no review lock, public route, snapshot, export,
release, or vendoring authority.

## Review And Approval Boundary

The authored lifecycle is `DRAFT` → `IN_REVIEW` → `APPROVED`. There is no
authored `PUBLISHED` state. Immutable events record supplied review entry,
findings, dispositions, approvals, withdrawal, supersession, and release
transfer. Generated locks bind those events to the applicable science,
communication, governance, attribution, finding-ledger, realization, and exact
predecessor identities. Regeneration can invalidate authority; it cannot create
or carry a human decision to different reviewed bytes.

The registry in `principals.yaml` gives stable identities, kinds, authorities,
and eligible roles. The software checks structure, declared roles, distinct
principals, and specified conflicts. It cannot authenticate a person, assess
competence, perform scientific review, or generate approval. A `DRAFT` with
missing human accountability cannot enter review. An `IN_REVIEW` source may
have an accountable report lead but remains blocked from approval and
publication until the required independent humans approve its exact locked
root.

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
