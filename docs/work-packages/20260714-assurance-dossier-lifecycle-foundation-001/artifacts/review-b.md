# Review B: Build, Release, And Security

Status: `FAIL`; independent initial review complete. Seven closure-blocking
findings remain open on the reviewed candidate. No finding is classified as a
deferrable follow-up.

## Review Basis

Static: this review used the Reviewer B scope in `package.md` and did not read
Review A or its output. Implementation access was read-only. The only
repository file written by this reviewer is this artifact.

Ran: the reviewed `FROZEN_BASE` resolves to
`00d985b1c0de77f1ea664df23a6f4999c4dad0cc`. A fresh
`openwepp-assurance plan --all` reported dossier source root
`907ff5a9d1b50b869e78773d3a5448b67ade55eff25110ff216890642e9b1d28`,
the identity assigned for this review. The committed dossier SHA-256 was
`7d01631aaf76946a0226e0b85e04e3f737d33a5df2702ae263a24bd4fac2b4ff`.

## Closure-Blocking Findings

### B-001: Existing Snapshots Accept A Symlinked File Root

- Severity: `HIGH`
- Exit criteria: `ASSURE-BUILD-005`, `ASSURE-REL-001`, `ASSURE-SEC-001`,
  `ASSURE-TEST-001`
- Disposition class: closure-blocking

Static: `validate_snapshot_layout` compares only the names of the snapshot
root entries, then calls `read_dir` on `target/files` without first rejecting a
symlink at that entry. The later per-file reads follow the same symlink. The
descendant check therefore does not enforce the contract's rejection of every
nonregular snapshot entry.

Ran: created a valid snapshot, moved its exact `files/` tree outside the
snapshot directory, replaced `files` with a symlink to that external tree, and
rebuilt under the same ID. The second command exited `0` and reported
`snapshot: confirmed`; `stat` identified `files` as a symbolic link. This
disproves the retained claim that snapshot symlinks are rejected and means an
ID does not establish a self-contained immutable directory.

Required closure: reject symlinks at the snapshot target, `manifest.json`,
`files/`, every descendant, and relevant snapshot-root ancestors using
no-follow metadata/opens. Add a negative test for the exact root-entry case and
rerun snapshot, release-consumer, and security gates.

### B-002: The Public Narrative Is Outside The DAG And Snapshot Identity

- Severity: `HIGH`
- Exit criteria: `ASSURE-LIFE-002`, `ASSURE-LIFE-004`,
  `ASSURE-BUILD-002`, `ASSURE-BUILD-003`, `ASSURE-BUILD-004`,
  `ASSURE-BUILD-005`, `ASSURE-REL-001`, `ASSURE-TEST-001`
- Disposition class: closure-blocking

Static: the narrative is a public why-record and an exported wepppy document,
and the lifecycle matrix requires rebuild/snapshot treatment for narrative
changes. However, it is absent from `plan_input_paths`, graph node construction,
the reviewed source root, and the snapshot's generated-file set. The plan calls
it neither an input nor a node even though catalog loading requires that it
exist. A targeted plan therefore is not a complete transitive-input report for
the published public slice.

Ran: after creating a snapshot, changed only
`usersum/snow-frost-modeling-and-validation.md` and rebuilt under the same
snapshot ID. The command exited `0` and reported the existing snapshot as
confirmed. The changed public record was not content-bound by the source root,
output digests, or snapshot manifest.

Required closure: give every public handoff document, including hand-authored
narratives, an explicit typed DAG role and release identity. Make narrative
bytes appear in plans and snapshots, and make claim-bearing narrative changes
invalidate the applicable review lock. Test existence drift, byte drift,
same-ID conflict, and targeted/all input completeness.

### B-003: The Review Lock Cannot Represent The Required Approval Record

- Severity: `HIGH`
- Exit criteria: `ASSURE-LIFE-001`, `ASSURE-LIFE-002`,
  `ASSURE-LIFE-004`, `ASSURE-BUILD-004`, `ASSURE-TEST-001`
- Disposition class: closure-blocking

Static: the lifecycle contract requires a review record to name the reviewer,
role, date, findings, and disposition, and it prohibits a conclusion-bearing
author from independently approving their own work. The strict `Review` type
and JSON schema have no reviewer-role, finding-disposition, author/separation,
or unresolved-finding fields. `enforce_review_lock` requires only an approved
state, reviewer string, date, and matching digests. Consequently, a published
lock cannot preserve the required approval semantics or fail closed on
self-approval/unresolved findings.

Required closure: extend and bind the typed review record to the contract's
role, disposition, separation, and unresolved-finding semantics. Add approved,
self-review, unresolved-finding, payload-change, and historical
superseded/withdrawn lock tests.

### B-004: Public Rendering Does Not Fail Closed On Links Or Secrets

- Severity: `HIGH`
- Exit criteria: `ASSURE-PILOT-004`, `ASSURE-SEC-001`,
  `ASSURE-TEST-001`
- Disposition class: closure-blocking

Static: source-controlled strings are interpolated directly into Markdown and
the export fragment. Markdown validation checks only unresolved template
tokens, a banner, and four substrings (`/home/`, `/workdir/`, `AKIA`, and
`BEGIN PRIVATE KEY`). It does not parse or constrain generated links, escape
Markdown control syntax, recognize common token families, or scan the export
fragment at all. The positive link test explicitly skips HTTP(S) links rather
than enforcing the contract's vendored-tree-only rule.

Ran: changed a dossier title in a temporary fixture to contain
`[internal](/admin/private)` and a GitHub-style `ghp_...` token. `build --all`
exited `0`; both strings appeared in generated Markdown, and the token also
appeared in the wepppy YAML export. The current committed outputs themselves
passed a broader disclosure scan; the finding is the missing fail-closed
control.

Required closure: define typed public-text and link contracts, escape or reject
Markdown-active source fields by context, require generated links to remain in
the vendored usersum tree, and scan every public/export output with a reviewed
secret/private-path policy. Add malicious title, list, table, URI, root-link,
private-path, token, and export tests.

### B-005: Versioned JSON Schemas Are Not Mechanically Enforced Or Bounded

- Severity: `MEDIUM`
- Exit criteria: `ASSURE-LIFE-004`, `ASSURE-BUILD-001`,
  `ASSURE-BUILD-002`, `ASSURE-SEC-001`, `ASSURE-TEST-001`
- Disposition class: closure-blocking

Static: `validate_schema_documents` reads each JSON schema without the
2 MiB source bound and verifies only that it parses and has a string `$id`.
It does not enforce the schema's version, expected ID, required structure, or
congruence with the strict Rust record. Typed deserialization is useful but
does not validate the separately published schema documents.

Ran: changed `dossier.schema.json` so `schema_version` required `2` while the
dossier and Rust type remained version `1`. `validate --all` still exited `0`
with `validation: PASS`.

Required closure: mechanically validate the canonical schema identities and
their records, or generate both schema and parser contract from one typed
authority. Apply the source-size/resource bound to JSON schemas and add schema
drift/version/size negative tests.

### B-006: A Draft Dossier Can Be Included In A Release Snapshot

- Severity: `MEDIUM`
- Exit criteria: `ASSURE-LIFE-002`, `ASSURE-BUILD-005`,
  `ASSURE-REL-001`, `ASSURE-TEST-001`
- Disposition class: closure-blocking

Static: the lifecycle contract says `DRAFT` permits no release snapshot.
`validate_build_options` restricts snapshots to `--all` but does not restrict
lifecycle state, while the review-lock check deliberately returns success for
states that do not require a publication lock.

Ran: changed the temporary pilot catalog and dossier from `candidate` to
`draft`, then executed `build --all --snapshot draft-release`. The command
exited `0` and created the snapshot.

Required closure: define and enforce the snapshot-eligible lifecycle set before
any output or snapshot write. Add negative coverage for `DRAFT` and explicit
tests for the intended treatment of every other lifecycle state.

### B-007: The Exported Lifecycle Status Is Not Wepppy-Compatible

- Severity: `HIGH`
- Exit criteria: `ASSURE-XREPO-001`, `ASSURE-REL-001`,
  `ASSURE-TEST-001`
- Disposition class: closure-blocking

Static: openWEPP lowercases dossier lifecycle values and currently exports
`status: candidate` for the method and dossier. The actual downstream wepppy
contract accepts only `active`, `deprecated`, or `draft` and rejects any other
status. The handoff says to preserve candidate status, so its stated merge and
validation commands cannot succeed without an unrecorded downstream contract
change or a defined semantic mapping. The same mismatch applies to future
`published`, `superseded`, and `withdrawn` values.

Required closure: either emit a documented compatible status mapping while
preserving assurance lifecycle in a separately accepted field, or amend the
wepppy handoff to require and test a downstream contract extension. Validate a
transformed fragment against the real downstream parser in a read-only fixture.

## Passing Evidence And Bounded Checks

Ran: all of the following passed on the reviewed candidate:

- `cargo fmt --check`;
- `cargo clippy -p openwepp-assurance --all-targets -- -D warnings`;
- `cargo nextest run -p openwepp-assurance` (`2` passed);
- `cargo nextest run --test assurance_dossier_build_contract` (`6` passed);
- `bash tools/release/check_assurance_dossier_exports.sh`;
- `bash -n` for both assurance-touched release scripts;
- separate all and targeted builds into clean temporary roots followed by a
  byte comparison; and
- current generated-output scans for private paths, common credential markers,
  and nonlocal Markdown links.

Static: nextest is used only as the test executor; the Rust compiler owns the
typed graph. The normal Rust source and crate manifest contain no command,
network-client, template-execution, nextest-scheduler, plugin, or agent surface.
Generated files have declared producers and source banners. The release-candidate
script invokes the real drift hook and records a snapshot-manifest digest, but
that consumer cannot close `ASSURE-REL-001` while B-001, B-002, B-006, and B-007
remain.

Static and Ran: `/home/workdir/wepppy` remained read-only. Its HEAD remained
`b2b6d62c3472c324263c55597c7ee5ccc9545942`, and its preexisting tracked-diff
SHA-256 remained
`ee6cdc17fde5bb7c709fab6a4ad166ddd60daddb3be8709bacf5c9405e9dc70d`.
No downstream deployment or consumption is claimed.

## Reviewer B Exit-Criterion Disposition

| Exit criterion | State | Reviewer B basis |
| --- | --- | --- |
| `ASSURE-LIFE-001` | `FAIL` | Required reviewer role and separation cannot be represented. |
| `ASSURE-LIFE-002` | `FAIL` | Draft snapshots are allowed; historical review-lock semantics are incomplete. |
| `ASSURE-LIFE-004` | `FAIL` | Narrative and review identities are incomplete; schema authority can drift undetected. |
| `ASSURE-BUILD-001` | `FAIL` | Happy-path commands pass, but schema validation can report false success. |
| `ASSURE-BUILD-002` | `FAIL` | The public narrative is absent from the typed DAG and schema drift is accepted. |
| `ASSURE-BUILD-003` | `FAIL` | Current builds are byte-identical, but the reported transitive input set omits the narrative. |
| `ASSURE-BUILD-004` | `FAIL` | The review record and narrative invalidation contract are incomplete. |
| `ASSURE-BUILD-005` | `FAIL` | Symlinked snapshots confirm, narrative bytes are unbound, and draft snapshots succeed. |
| `ASSURE-PILOT-004` | `FAIL` | Public link/private-disclosure behavior is not fail-closed. |
| `ASSURE-XREPO-001` | `FAIL` | Export lifecycle statuses are rejected by the real wepppy consumer contract. |
| `ASSURE-REL-001` | `FAIL` | The real hook exists, but its snapshot and downstream artifact do not meet their contracts. |
| `ASSURE-TEST-001` | `FAIL` | Bounded nextest use passes, but required negative/consumer cases are missing and reproduced as failures. |
| `ASSURE-SEC-001` | `FAIL` | Snapshot containment, publication sanitization, and JSON resource/schema controls remain open. |
| `ASSURE-CLOSE-004` | `FAIL` | Seven accepted-or-rejected dispositions and relevant remediation verification are pending. |

## Final Assessment

`FAIL`. The candidate establishes a useful deterministic happy path and a real
release-hook integration, but it is not ready for package closure or release.
All seven findings are in-envelope and closure-blocking; none should be deferred
to a later package. After disposition and remediation, Reviewer B should verify
the exact reproductions above, the focused gates, the real release consumer,
the downstream contract fixture, and the terminal source identity.
