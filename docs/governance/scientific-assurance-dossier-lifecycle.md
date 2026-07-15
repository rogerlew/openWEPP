# Scientific Assurance Dossier Lifecycle And Build Contract

Status: Active

Contract version: 1

Effective date: 2026-07-14

Owner: openWEPP maintainers

## Purpose And Boundary

This contract governs the records that let a hydrologist, soil scientist,
researcher, or practitioner inspect openWEPP evidence without learning the
repository's internal process. It connects four questions while preserving the
different authority behind each answer:

| Question | Public record | Accountable owner |
| --- | --- | --- |
| Why does the model behave this way? | Model-science narrative | Domain science steward |
| How was it evaluated? | Versioned evaluation method | Evaluation method owner |
| What does the evidence show? | Scientific assurance dossier | Dossier steward and scientific assessment owner |
| What does that mean for this application? | Application-context worksheet and decision record | Named user or institutional decision owner |

The build system proves that declared inputs and generated pages agree. It
does not decide whether evidence is scientifically current, whether a reviewer
has appropriate expertise, or whether openWEPP is fit for a particular use.
Software verification acceptance, empirical characterization, and application
fitness remain separate decisions.

## Ownership And Separation

One person may hold more than one role when that is disclosed, except that a
conclusion-bearing author may not independently approve their own work.

| Record or decision | Accountable owner | Required separation |
| --- | --- | --- |
| Model-science narrative | Domain science steward | Explains intended science; does not assign an evidence status. |
| Evaluation method | Evaluation method owner | Freezes prospective choices before execution; labels retrospective choices. |
| Evidence manifest and dossier source | Dossier steward | Preserves absent, restricted, failed, and contrary evidence. |
| Empirical characterization | Scientific assessment owner | Requires independent scientific review at the declared consequence level. |
| Templates, compiler, and generated pages | Assurance tooling maintainer | Reports structural validity and drift only. |
| Release snapshot inclusion | Release authority | Accepts exact software and evidence identities, not site fitness. |
| wepppy vendoring and discovery | wepppy documentation owner | Owns downstream sync, manifest merge, roles, navigation, rendering, and search. |
| Application-fitness decision | Named decision owner | Applies only to the named application, purpose, and conditions. |

Package reviewers and coding agents are not described as external scientific
reviewers. Agent-assisted analysis is an authored input subject to the same
review and lock rules as human-authored analysis.

## Canonical Records And Stable Identity

The tracked assurance source root is `assurance/`. It contains the catalog,
compiler-bound versioned schemas, templates, methods, dossier sources,
interpretation, limitations, evidence manifests, agent-assisted authoring
records, review histories, and the generated wepppy export fragment. The
public generated root is `usersum/assurance/`. A declared model-science
narrative remains hand-authored under `usersum/`, but it is a typed catalog
input, a review-root input, a dependency-graph node, and a snapshot file.

Stable dossier and method IDs are lowercase ASCII kebab-case. They are never
derived from titles or paths and are never reused. Versions use three numeric
components. A material conclusion change creates a new version; a title edit
alone does not change identity. A published version is never silently rewritten.

Source files are edited by their accountable owners. Generated Markdown and
the export fragment carry a source banner and must not be hand-edited. The
catalog assigns exactly one producer to each generated output. Existing science
contracts, tests, observed-data admission decisions, and work packages remain
authority or evidence; a dossier links and content-identifies them rather than
copying their authority.

Every public dossier links to its narrative, method, limitations, application
worksheet, and catalog. The narrative links back to the method and dossier.
Public pages link only within the vendored `usersum/` tree. Audit paths that are
meaningful only in a source checkout are printed as text, not rendered as
public hyperlinks.

## Lifecycle

| State | Meaning | Permitted transition |
| --- | --- | --- |
| `DRAFT` | Authoring is incomplete; no publication or release snapshot. | `CANDIDATE`, `WITHDRAWN` |
| `CANDIDATE` | Structurally valid and publicly inspectable, but review or publication approval remains open. | `DRAFT`, `PUBLISHED`, `WITHDRAWN` |
| `PUBLISHED` | Independent scientific and publication approvals match their separate roots. | `SUPERSEDED`, `WITHDRAWN` |
| `SUPERSEDED` | Preserved historical record replaced by a named newer dossier version. | none |
| `WITHDRAWN` | Preserved record whose rationale and affected conclusions remain visible. | none |

`PUBLISHED` requires two current, independently authored approval records. The
scientific approval binds the conclusion-bearing source root: dossier, method,
evidence manifest, interpretation, limitations, narrative, and agent-assisted
authoring record. The publication approval binds that material plus templates,
schemas, compiler sources, Cargo manifests, and the lockfile. Each approval
also binds its reviewer roles, expertise, independence basis, findings,
dispositions, and residual disagreements through a noncircular, ordered-history
payload digest. Every approved entry binds the complete semantic history prefix
through that entry. A terminal publication approval must be the last history
entry, so later review activity requires renewed publication review. Malformed
or altered history fails validation; a valid history whose current roots no
longer match produces `REVIEW_REQUIRED`. `DRAFT` dossiers cannot enter release
snapshots; structurally valid `CANDIDATE` dossiers may be snapshotted with
their pending review state visible. The tool never changes a lifecycle or
evidence status automatically.

When a dossier declares agent-assisted authoring, its packet must itself carry
an independent approval bound to the accepted-output root before `PUBLISHED`.
That packet approval does not replace either dossier-level approval.

Four dates or identities must not be collapsed:

- the evidence as-of date says what evidence was considered;
- the review lock says which conclusion-bearing bytes were approved;
- the generated digest says which public bytes match those sources; and
- the release snapshot says which immutable dossier set accompanied a release.

None of these proves scientific currency. Currency is an explicit steward and
reviewer judgment after checking new science, observations, software changes,
and intended uses.

## Material-Change Trigger Matrix

`R` means mechanical rebuild, `I` evidence-impact assessment, `X` independent
rereview, `V` new dossier version or supersession, and `S` new release snapshot.
An empty entry means no scientific effect, although ordinary editorial review
may still apply.

| Change | R | I | X | V | S | Required treatment |
| --- | :---: | :---: | :---: | :---: | :---: | --- |
| Model code affecting the assessed result path | yes | yes | yes | usually | yes | Recheck verification and empirical transferability. |
| Declared model configuration | yes | yes | yes | usually | yes | Treat changed defaults and parameters as a new assessed realization. |
| Dataset bytes or admission posture | yes | yes | yes | yes | yes | Preserve the prior evidence set. |
| Transformation code or transformed bytes | yes | yes | yes | yes | yes | Reproduce and reassess affected results. |
| Metric definition, units, or aggregation | yes | yes | yes | yes | yes | Do not compare old and new scores as identical measures. |
| Interpretation criterion or tolerance | yes | yes | yes | yes | yes | Label post hoc changes and bias review. |
| Calibration/evaluation partition | yes | yes | yes | yes | yes | Reassess leakage and independence. |
| Template or renderer with no semantic change | yes |  |  |  | yes | Review rendered diff; source characterization is unchanged. |
| Interpretation or evidence-summary prose | yes | yes | yes | if material | yes | Any conclusion-bearing edit invalidates the review lock. |
| Limitation, exclusion, or nonuse domain | yes | yes | yes | if material | yes | Narrowing information is never suppressed for convenience. |
| Reviewer identity, approval, or unresolved finding | yes | yes | yes | if conclusion changes | yes | A new lock and independent record are required. |
| Narrative rationale only | yes | impact check | if claim changes | if claim changes | yes | Keep why and what consistent without duplicating results. |
| Public title, navigation key, or audience metadata | yes |  |  |  | yes | wepppy owns final downstream placement and access. |
| New release with unchanged current dossier bytes |  | currency check | only if currency changed |  | yes | Record a new immutable release snapshot. |

The dossier steward records the impact decision. When uncertain whether a
change alters an empirical conclusion, treat it as conclusion-bearing and
require rereview.

## Frozen Command Contract

Run from the repository root:

```text
cargo run -p openwepp-assurance -- validate (--dossier <stable-id> | --all)
cargo run -p openwepp-assurance -- plan (--dossier <stable-id> | --all)
cargo run -p openwepp-assurance -- build (--dossier <stable-id> | --all)
    [--output-root <path>]
    [--snapshot <path-safe-id> --snapshot-root <path>]
cargo run -p openwepp-assurance -- check (--dossier <stable-id> | --all)
```

Exactly one selector is required. `--snapshot` and `--snapshot-root` are a
pair and are accepted only by `build --all`. `--output-root` redirects generated
public and export paths beneath a test or staging root without changing their
repository-relative names. Normal commands are offline and invoke no shell,
network client, plugin, or agent.

`validate` checks compiler-bound schemas and strict typed deserialization, IDs,
paths, content identities, complete rendered public documents, links, graph
integrity, lifecycle fields, and review locks. `plan` performs the same
read-only source and rendered-document validation, then prints the ordered
transitive inputs, SHA-256 identities, output set, scientific and publication
root digests, approval-history-payload digests, and review implications without
promoting lifecycle state. `build`
renders only the selected dossier plus affected shared
catalog/export outputs. `check` builds in a fresh temporary directory and
compares every selected committed generated file without modifying tracked
content; it also requires the two generated roots to contain exactly the full
catalog-declared output inventory, with no orphan, missing, symlink, or special
entry.

## Typed Dependency And Fingerprint Contract

The catalog and strict Rust types define the only allowed node kinds: catalog,
schema, compiler tool, method, dossier, evidence manifest, evidence asset,
narrative, interpretation, limitations, agent-assisted authoring record,
tracked authoring input, accepted authoring output, review, template, public
output, and export. Unknown fields or enum values fail. The format has no
command, environment, URL-fetch, plugin, or agent node.

Every generated node has one producer and a complete ordered dependency set.
Duplicate IDs, missing dependencies, cycles, output collisions, absolute paths,
`..` traversal, unsafe symlink escape, undeclared outputs, and paths outside the
approved source or generated roots fail closed. External evidence may be named
with an access posture and stable identity, but is never fetched.

A node fingerprint is SHA-256 over length-framed domain, contract version,
schema version, node kind, stable ID, and repository-relative node path;
ordered dependencies contribute their stable IDs and fingerprints. A source
node then contributes its byte length and raw bytes through a bounded-memory
stream. The scientific root and larger publication root are reported separately
as described above. Schema documents have exact
compiler-bound path, byte digest, dialect, identifier, and version identities;
arbitrary JSON with a plausible `$id` is not accepted. The evidence manifest
in turn records the exact software,
configuration, dataset, retained output, and review-evidence identities used by
the assessment. Filesystem modification times are never identities.

Opening the repository discovers the complete local input set, captures every
content identity, reparses under that frozen set, and verifies that neither an
input nor the path set changed. Every validate, plan, build, check, and snapshot
path checks those identities before and after its read phase. A caller must
reopen after any input edit; stale in-memory state cannot be rendered or
snapshotted.

Identical frozen inputs and tool version produce byte-identical output. Shared
catalog and export entries sort by stable ID. Targeted operations report the
complete transitive input set and may update only the selected dossier's outputs
and declared shared outputs.

## Review Lock And Optional Agent Assistance

A review history names conclusion authors and retains separate scientific and
publication approval entries. Every approval records scope, reviewer names,
roles, expertise, independence basis, date, structured findings, dispositions,
resolution state, residual disagreements, and the applicable root digest. A
second digest binds the complete ordered semantic history prefix while
excluding the derived digest fields. Editing, removing, or reordering an
earlier entry therefore invalidates every affected later approval. Self-approval
and approval with an unresolved closure-blocking finding fail. A `CANDIDATE`
may retain pending entries. `PUBLISHED` requires
matching current scientific and publication approvals; `SUPERSEDED` and
`WITHDRAWN` retain the approvals that applied to the historical version.

When an agent helps inventory, compare, summarize, or draft conclusion-bearing
content, retain a review packet with:

- the bounded question and procedure version;
- complete task instruction;
- repository-relative input paths and SHA-256 digests;
- available agent, model, and tool identity;
- date and available nondeterministic settings;
- retained output path and digest;
- accepted edits or extraction decisions; and
- reviewer findings, disposition, identity, and approved candidate-root digest.

The canonical packet is cataloged beside the dossier, validated on every
build, and included in both review roots and the dependency graph. It binds the
complete content-identified input list, every accepted output and digest, an
accepted-output root, and an independent review disposition. A package-local
artifact may summarize or point to that canonical record but is not a shadow
substitute. The record supports traceability and procedural repetition, not
byte-identical agent output. Private reasoning is neither required nor treated
as evidence. Changed tracked input or accepted output invalidates the packet;
normal builds never rerun the agent.

## Immutable Release Snapshots

A snapshot is created only by explicit `build --all --snapshot ...`. Its
manifest records the snapshot ID, catalog digest, tool version and source
digest, contract version, selected dossier versions and lifecycle states,
scientific and publication root digests, and every public file digest. The
public file set includes generated pages, the export fragment, and each
declared hand-authored narrative; changing a narrative therefore conflicts
with reuse of the same snapshot ID. Snapshot IDs accept only lowercase ASCII
letters, digits, period, underscore, and hyphen and may not begin with a period.

The snapshot directory is immutable by ID. Rebuilding byte-identical content
under the same ID confirms it; different content under that ID fails. Snapshot
roots and existing layouts are inspected without following symlinks. New
snapshots use exclusively created retry staging directories and never delete a
preexisting collision owned by another process. A release
candidate first validates committed output, then creates the named snapshot in
its evidence directory and records the manifest digest. Rollback selects a
previous software release and its recorded snapshot; it does not mutate a newer
snapshot or claim that the older evidence is scientifically current.

## Cross-Repository Boundary

openWEPP owns the assurance source, generated usersum pages, stable document
IDs, titles, source-relative paths, minimum roles, categories, audience tags,
statuses, and navigation keys in its export fragment. wepppy owns vendor
registration, synchronization policy, final manifest merge, authorization,
navigation placement, rendering, and search indexing. An openWEPP build proves
the handoff artifact exists and is internally consistent; it does not prove
that wepppy has consumed or deployed it.

Generating a handoff does not authorize or imply readiness for vendoring.
Downstream vendoring requires a separately authorized wepppy package at a
declared release gate. The current gate is deferred until the openWEPP beta
release campaign in WEPPcloud: do not begin vendoring while the WEPPcloud
documentation surface or dossier set is still being developed, but complete
and verify vendoring before that beta release is issued.

The export uses only downstream-supported `status` values: `draft` for
`DRAFT`, `active` for `CANDIDATE` and `PUBLISHED`, and `deprecated` for
`SUPERSEDED` and `WITHDRAWN`. The exact assurance lifecycle is retained in the
separate `assurance_lifecycle` field; downstream code may ignore that extension
until its own contract adopts it.

## Prohibited Interpretations

- Matching hashes prove congruence, not scientific adequacy.
- A `PUBLISHED` dossier is reviewed evidence as of a date, not a permanent
  certificate or an application authorization.
- Nextest executes compiler tests; it is not the evidence graph or scheduler.
- Green CI, conservation, legacy agreement, test count, and code coverage do
  not establish empirical corroboration.
- `INSUFFICIENT_EVIDENCE`, `NOT_EVALUATED`, mixed evidence, contradiction, and
  unavailable evidence are valid public outcomes.
- No release event turns openWEPP into a terminally validated model of an open
  natural system.
