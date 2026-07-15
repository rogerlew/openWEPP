# Dependency And Build Proof

Status: `complete`; re-freeze identities, focused proof, the complete heavy
sequence, and both independent accepted-fix verifications pass.

## Compiler Boundary

Static: `openwepp-assurance` is a bounded, offline compiler. The catalog can
name only dossier source records, fixed template roles, and generated paths.
It has no command, environment-expansion, network, plugin, or agent field.
Strict `serde` records reject unknown YAML fields, while compiler-owned graph
construction assigns the node kinds and dependencies.

Static: the graph contains these typed layers:

```text
catalog/schema/tool
        |
method/dossier/evidence manifest/evidence assets/narrative
interpretation/limitations/agent-authoring + tracked inputs + accepted outputs
review history
        |
template + generated method/dossier/catalog/worksheet/export + snapshot
```

Every node fingerprint hashes length-framed domain, contract/schema versions,
node kind, stable ID, repository-relative path, ordered dependency IDs and
fingerprints, then the source byte length and raw bytes when applicable. Source
files are streamed. The tool aggregate includes the root and crate Cargo
manifests, `Cargo.lock`, and every Rust source below
`crates/openwepp-assurance/src/`. Modification times do not participate.

Opening performs discovery, content capture, a second parse under the frozen
set, and a final identity check. Every operation rechecks both all bytes and
the compiler-source path inventory before and after reading. A file edited or
new compiler source added after `Assurance::open` produces `Drift` before a
snapshot can be created.

## Candidate Identity

Ran: `plan --all` reports separate scientific and publication roots, two
scope-specific review-payload digests, and a fingerprint for every transitive
source, schema, template, evidence asset, narrative, authoring record, review
record, and output node. The frozen candidate identities are:

| Identity | SHA-256 |
| --- | --- |
| Scientific root | `bb4b8b5f6188613e22ca9a7bec301bd7d6a94f8ef5e3e2ed83f98ad532d45e8c` |
| Publication root | `9d3432db6eee33201c03d50ac9666bc050d46d4a0519170d05f05132ed5c32e8` |
| Scientific review payload | `17013e3a6ec7ce97a795a4b3a024b3f5db305a8064a6b24342c155c28c54f6f1` |
| Publication review payload | `ab11f85bddd81fc8457d5413cb47cd9969511ce6feb139deca36cae462cc58b8` |
| Agent accepted-output root | `01aa0936d0dce5c859440f56a9bd0eca87976462a524696307840103a9fae9ed` |

The dossier-level scientific and publication approvals remain intentionally
pending because package review is not represented as external hydrologist or
publication approval. The separately required agent-assisted-authoring review
is approved over the exact accepted-output root by an independent procedural
reviewer; that is not a favorable scientific characterization.

## Selection And Output Sets

Ran: both `plan --all` and
`plan --dossier snow-snotel-swe-depth-density` reported the same five outputs,
as expected while the catalog contains one dossier:

- `assurance/generated/wepppy-usersum.yaml`;
- `usersum/assurance/README.md`;
- `usersum/assurance/application-context-worksheet.md`;
- `usersum/assurance/dossiers/snow-snotel-swe-depth-density.md`; and
- `usersum/assurance/methods/snow-snotel-evaluation-v1.md`.

The shared index and export depend on all catalog entries. The selected method
and dossier depend only on the selected dossier's declared sources plus shared
schema, template, and compiler identities. The complete ordered inputs and
their SHA-256 values are emitted by `plan`; authoring-only inputs and accepted
outputs are first-class nodes rather than incidental overlap with dossier
sources. Missing dependencies and cycles are rejected before planning or
building.

## Determinism And Drift

Ran: two `build --all` operations into separately allocated clean `/tmp`
directories were compared with `diff -qr`; the command exited zero with no
differences. Their output SHA-256 values were identical:

| Output | SHA-256 |
| --- | --- |
| wepppy export | `828762f7ef5672a7e50b0e56184aac2d5a40530a3507b788cf4a58714612ee2a` |
| public catalog | `bfe57c9c65fcba174a543e0f5bc287124a292215aaf3f17f0148a58460d8b26e` |
| application worksheet | `866774bf82baaaff90f63e8050cad8b9f3127f490b74fe24fca7fa6e7f269352` |
| SNOTEL dossier | `6d2dea9f676d996b7b1ddf8b6737cc61d80fbbf06ba473250fd8800842fdfbfd` |
| evaluation method | `15bd161a6b63515533fdb6aea651260fedb1556e81ca549530e56a4217dc5e82` |

Ran: `cargo run -p openwepp-assurance -- check --all` and
`bash tools/release/check_assurance_dossier_exports.sh` both passed against the
committed public files. The integration contract separately mutates a generated
file and asserts `AssuranceError::Drift`.

## Review Invalidation

Static: the scientific root binds the dossier, method, evidence,
interpretation, limitations, narrative, and canonical agent-assisted authoring
record. The publication root additionally binds every template, exact
compiler-bound schema, compiler source, Cargo manifest, lockfile, output-path
contract, and the repository-relative review-record path without circularly
hashing the review bytes. The dossier output node depends on all contributors
to the publication root that it renders.
Separate scientific and publication histories have noncircular, prefix-bound
payload digests. Editing, removing, or reordering earlier semantic history
invalidates later approval; the current publication approval must be the
terminal entry. `PUBLISHED`, `SUPERSEDED`, and `WITHDRAWN` require both current
root and complete-history payload matches; published agent-assisted work also
requires its own accepted-output-root approval.

Ran: the review-lock integration cases prove:

1. exact scientific, publication, payload, and authoring locks permit a
   published build;
2. a bound source edit returns `REVIEW_REQUIRED`;
3. a review-history edit fails validation before a stale approval can be used;
4. self-review and approval over an unresolved closure blocker fail; and
5. superseded and withdrawn records require and accept matching historical
   locks.

No test or compiler path promotes a scientific status in response to a hash or
test result.

## Immutable Snapshot

Ran: explicit temporary snapshots were created and confirmed by identical
second builds. Their manifests record catalog, contract, compiler, dossier
version/lifecycle, empirical status, scientific root, publication root, and
every public-file identity, including the hand-authored narrative.

Ran: snapshot tests prove same-ID/same-content confirmation; narrative change
under the same ID conflicts; `DRAFT`, unsafe IDs, and targeted snapshots fail;
and symlinks at the snapshot-root ancestor, `manifest.json`, `files/`, and a
descendant public file fail. Existing root entries and the recursive public
file set must exactly match the manifest before bounded reads. Component-wise
root creation produces no write through a symlink ancestor, and exclusive
retry staging preserves an unknown colliding directory and sentinel.

## Negative And Consumer Proof

Ran: the 18-test `assurance_dossier_build_contract` integration target
passes. Its negative cases cover unknown source fields, duplicate output paths,
canonical path grammar, source/output/snapshot symlinks, unsafe snapshot IDs,
stale or orphan generated output, separate and historical review locks,
authoring-lock invalidation, unique authoring dependencies, self-review,
unsafe pending participants, unresolved findings, schema mutation/size,
public links/fragments/absolute paths/secret families, draft snapshots,
open-to-operation drift, and all five lifecycle export mappings.

Static and Ran: `production_builder_has_no_execution_network_or_agent_surface`
scans the production Rust source and manifest for subprocess, network, async,
template-execution, nextest-as-DAG, and agent surfaces. It also requires the
release script to invoke the drift consumer, build an explicit snapshot, and
record its digest. `bash -n` passes for both changed release scripts.

Static: all four generated Markdown files carry the compiler source banner.
The real-public-slice integration test follows the local why/how/what/worksheet
links and validates the export record fields, so proof reaches committed public
consumers rather than stopping at a producer.

Ran: the transformed five-record export passed the real read-only wepppy
`_parse_docs_manifest` parser at the recorded downstream HEAD. This proves the
`draft` / `active` / `deprecated` status mapping is syntactically consumable;
it does not claim downstream vendoring or deployment.

Ran: two re-freeze clean builds were byte-identical, the targeted and all-plan
output sets were identical for the one-dossier catalog, and two builds using
snapshot ID `package-proof` confirmed identical content. The terminal snapshot
manifest SHA-256 was
`68059305c87af056c6c7d81dd21de104670270ccdce9afd21d7f4ccf2aab44a8`.
