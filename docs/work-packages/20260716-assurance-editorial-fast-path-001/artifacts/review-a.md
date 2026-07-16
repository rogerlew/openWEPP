# Independent Review A: Assurance Editorial Fast Path

Evidence class: **Static + Ran**

Initial recommendation: **HOLD**

I independently reviewed the full current candidate against frozen base
`25bcb17f4a62924976a19381e974a36612ed4845`, the package contract, and the
applicable root, work-package, crate, and test instructions. The reviewed
normalization implementation had SHA-256
`953340f10e0e74231ef8f3d9fbc55162b2737649345367109271934339ca72ad`;
the normalization integration test had SHA-256
`6368b6d65acb78fcd00181040061ef72577177ccece67e9e71a96aa814ac00e4`.
I did not read Review B or edit production files.

## Findings

### EDIT-A-001 — High, blocking: the production API can bypass the canonical converter while issuing a false receipt

`V2NormalizationOptions::with_converter_for_test` and the fault enum/method are
public, exported production API; `#[doc(hidden)]` only hides documentation and
does not restrict callers
([normalization.rs](../../../../crates/openwepp-assurance/src/v2/normalization.rs):28,
[normalization.rs](../../../../crates/openwepp-assurance/src/v2/normalization.rs):55,
[lib.rs](../../../../crates/openwepp-assurance/src/lib.rs):19). The selected
executable supplies the bytes that are rebound into the assurance graph, but
the receipt unconditionally records `"uk2us"`
([normalization.rs](../../../../crates/openwepp-assurance/src/v2/normalization.rs):219).
Consequently, any library caller can run an arbitrary executable, commit its
output as a purported normalization, and obtain an audit receipt that falsely
attests to the canonical converter. The integration tests exercise exactly
this public bypass with a shell/sed substitute.

Required fix: keep the public options limited to language and mode. Move
converter and fault injection behind a crate-private, sealed test seam and
exercise it in unit/contract support that is not exported by the production
library. The CLI acceptance test should continue to prove the installed
canonical `uk2us` boundary. Receipts must identify the operation truthfully and
must never label substitute output as canonical.

### EDIT-A-002 — High, blocking: packet rebinding does not prove the `draft_outputs` evidence edge, and no-op checks skip it entirely

When prose changes, the code parses the agent packet as unconstrained generic
JSON and recursively accepts one matching `path`/`sha256` pair anywhere in the
document
([normalization.rs](../../../../crates/openwepp-assurance/src/v2/normalization.rs):268,
[normalization.rs](../../../../crates/openwepp-assurance/src/v2/normalization.rs):270).
It does not prove that the pair is in the top-level `draft_outputs` array, that
the array contains exactly the selected manuscript and supplement, or that
the packet's declared role/schema is the expected exact-output record. Normal
v2 validation treats this dependency's bytes as opaque content, so post-install
validation does not supply the missing semantic check.

The no-change branch returns before the packet is opened at all
([normalization.rs](../../../../crates/openwepp-assurance/src/v2/normalization.rs):254).
An already American-English report with a packet missing or misplacing its
exact-output edges can therefore receive a successful normalization receipt.
This contradicts the package's fail-closed requirement for missing or
ambiguous identity edges.

Required fix: add a normalization-specific typed packet contract that requires
and validates the exact `draft_outputs` mapping for both selected content
sources, including cardinality and duplicate rejection. Run it before the
no-op/check decision as well as before rebinding, and add negative tests for a
missing output, misplaced matching pair, duplicate pair, and unrelated extra
output.

### EDIT-A-003 — High, blocking: selected report inputs are reread without their validated identities and can absorb concurrent semantic edits

The operation validates the selected graph once, then rereads the manifest,
manuscript, supplement, packet, and catalog to construct the candidate without
checking those reads against the identities just validated
([normalization.rs](../../../../crates/openwepp-assurance/src/v2/normalization.rs):141,
[normalization.rs](../../../../crates/openwepp-assurance/src/v2/normalization.rs):233,
[normalization.rs](../../../../crates/openwepp-assurance/src/v2/normalization.rs):340).
Before exchange, both calls to `repository.verify_inputs()` cover only the
catalog/schema/principal inputs captured during `V2Repository::open`; report
inputs are local to `validate_sources` and are not retained
([normalization.rs](../../../../crates/openwepp-assurance/src/v2/normalization.rs):166,
[normalization.rs](../../../../crates/openwepp-assurance/src/v2/normalization.rs):558,
[v2.rs](../../../../crates/openwepp-assurance/src/v2.rs):682).

The root `flock` serializes cooperating normalizers but does not constrain an
editor, Git operation, or other non-locking writer. A report or manuscript edit
made after initial validation can therefore be incorporated and mechanically
rebound as if it were part of the converter-only operation. If the prose needs
no lexical change, the early return can instead claim a successful no-op while
the graph on disk is already stale. Receipt `old_sha256` values are also
computed from another later reread, so they need not identify the bytes sent to
the converter.

Required fix: construct the candidate from one identity-checked snapshot of
every selected input, retain those expected identities, and recheck the entire
old selected graph immediately before no-op success and before exchange. The
replacement transaction must not absorb bytes that were not in the validated
snapshot. Add deterministic mutation hooks for manifest/content drift between
validation, conversion, and exchange, and prove fail-closed behavior.

### EDIT-A-004 — High, blocking: cleanup failure can exchange a partially deleted old generation back into service

After the new generation validates, `finish_transaction` recursively deletes
the old generation at `.v2.normalize.next`. If that deletion errors after
removing only part of the tree, the caller invokes `restore_previous`, which
exchanges the now-partial old tree back over the complete validated tree
([normalization.rs](../../../../crates/openwepp-assurance/src/v2/normalization.rs):183,
[normalization.rs](../../../../crates/openwepp-assurance/src/v2/normalization.rs):582,
[normalization.rs](../../../../crates/openwepp-assurance/src/v2/normalization.rs):588).
An I/O error or a concurrent special-file insertion during cleanup can thus
turn cleanup failure into source corruption, contrary to whole-transaction
rollback.

Required fix: define the commit point and cleanup state so destructive cleanup
can never make the rollback generation ineligible for restoration. Once old-
generation deletion begins, do not exchange that generation back over the
validated current tree. Add a deterministic mid-cleanup failure contract that
proves the complete old or complete new generation remains available and that
recovery state is explicit.

### EDIT-A-005 — Medium, blocking: whole-tree cloning silently changes metadata outside the receipted change set

`clone_v2_tree` recreates every directory and file through generic staging
helpers rather than preserving source metadata
([normalization.rs](../../../../crates/openwepp-assurance/src/v2/normalization.rs):570).
Static inspection found all 21 current `assurance/v2` files are mode `0664` and
all 9 directories are mode `0775`; the helpers create files with `0644` and
directories with `0755`
([confined.rs](../../../../crates/openwepp-assurance/src/v2/confined.rs):463).
A successful one-line spelling normalization therefore silently removes group
write permission from the entire v2 tree while the receipt lists only the four
content/hash files. This is broader than the documented converter-produced
bytes plus mechanical rebinding and can disrupt the shared authoring surface.

Required fix: preserve the mode of every cloned regular file and directory, or
use a clone mechanism that does so while retaining confinement and special-file
rejection. Add a fixture with non-default modes and assert exact metadata as
well as byte preservation across success and rollback.

### EDIT-A-006 — Medium, blocking: mandatory negative and determinism contracts are absent

The package requires contracts for non-DRAFT refusal, converter failure
classes, and equivalent-input receipt determinism. The four normalization
tests cover a current no-op, British-spelling check, successful apply/build/
check/idempotence, a missing executable, and one post-install fault. They do
not exercise `IN_REVIEW`/`APPROVED` or review-lock refusal, converter nonzero
exit, converter non-UTF-8 output, non-idempotent output, equivalent fixtures
producing identical receipts, packet ambiguity/missing edges, or report-input
drift. The current success comparison usefully proves the rebound report and
packet bytes equal the reference, but it does not close these required failure
surfaces.

Required fix: add deterministic contracts for every listed rejection and for
equivalent-input receipt equality. Exercise CLI mode/selection isolation in
addition to private parser state, especially no mode, both modes, `--all`, and
foreign command options.

### EDIT-A-007 — Low, closure-governance: the touched 2,000-line file lacks the required decomposition record

Touched `crates/openwepp-assurance/src/v2.rs` is 2,842 lines (2,821 at the
frozen base), so the package's 2,000-line WARN applies. I found no recorded
decomposition rationale and split intent in the package artifacts. No touched
Rust file reaches the 3,000-line closure block.

Required fix: record why the thin 21-line repository delegation is retained in
the existing root module for this package and name an owned split intent for
the already-large schema/model/validation responsibilities. This finding does
not require an unrelated refactor in the current package, but the explicit
governance record is required before closure.

## Confirmed Surfaces And Executed Evidence

- Ran: `cargo fmt --check` passed.
- Ran: `cargo clippy -p openwepp-assurance --all-targets -- -D warnings`
  passed.
- Ran: focused normalization plus assembly integration contracts passed
  `12/12`.
- Ran: the initial editorial filter passed `81/81` in 111.47 seconds. After the
  final profile boundary excluded publication integration, the exact final
  `cargo nextest run --workspace --profile assurance-editorial` passed `56/56`
  in 8.946 seconds.
- Ran: the real production no-op CLI check emitted deterministic-looking JSON
  with equal old/new source roots and zero changes; SHA-256 comparison proved
  that catalog, descriptor, manuscript, supplement, and packet bytes were
  unchanged.
- Ran: `git diff --check` passed before this artifact was written.
- Static: the final editorial profile retains the complete assurance crate plus
  source, planner, assembly, normalization, and report-facing integration
  binaries. Excluding publication is consistent with the package's explicit
  pre-review DRAFT boundary; lifecycle, authority, schema, builder, mixed, and
  publication changes are documented as ordinary full-gate work.
- Static: the converter is invoked without a shell, with explicit `-`, piped
  standard input/output/error, nonzero-status rejection, and UTF-8 validation.
- Static: path/digest edits are span-scoped and avoid YAML/JSON reserialization;
  a `BTreeMap` gives deterministic changed-path order.
- Static: DRAFT lifecycle, draft review-state structure, and
  `review_entry_authorized == false` are checked before converter execution.
- Static: the whole-tree `RENAME_EXCHANGE` install provides one directory-entry
  cutover and post-install reopening/validation; findings EDIT-A-003 through
  EDIT-A-005 identify the remaining transaction-integrity gaps.

## Line-Count Governance

| Touched Rust file | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-assurance/src/v2.rs` | 2,842 | WARN; EDIT-A-007 requires rationale and split intent |
| `crates/openwepp-assurance/src/v2/assembly.rs` | 1,948 | Below threshold |
| `tests/integration/assurance_v2_assembly_contract.rs` | 759 | Below threshold |
| `crates/openwepp-assurance/src/cli.rs` | 766 | Below threshold |
| `crates/openwepp-assurance/src/v2/normalization.rs` | 601 | Below threshold |
| `tests/integration/assurance_v2_normalization_contract.rs` | 330 | Below threshold |
| `crates/openwepp-assurance/src/error.rs` | 72 | Below threshold |
| `crates/openwepp-assurance/src/lib.rs` | 25 | Below threshold |

## Recommendation

Keep the package on **HOLD**. EDIT-A-001 through EDIT-A-006 are current-scope
closure findings and must be dispositioned, fixed where accepted, and
independently reverified. EDIT-A-007 requires the package's explicit WARN
record. The final editorial profile boundary itself is proportionate and is
not a finding.

## Remediation Verification: Final Candidate

Evidence class: **Static + Ran**

Final recommendation: **PASS**

This section supersedes the initial HOLD recommendation for the remediated
implementation. The final normalization implementation reviewed here has
SHA-256
`89a83b4351581b08719cda4d8e42a7d1e75e9ef9bc0534077128b42c41388a1f`;
the confinement implementation has SHA-256
`eb7f06490b3c832760e009b6c974a2484c509eb0aee6129c37471c680d79d189`;
and the normalization integration contract has SHA-256
`13169d922fe3386598c3a4370d59d5f7639d3280b1dcc9a9ef2d4a885a93a093`.

### Finding Disposition Verification

- **EDIT-A-001 — resolved.** Public normalization options now expose only
  language and mode. Converter and deterministic-fault controls are private to
  `normalization.rs`; neither the substitute executable nor fault enum is
  exported from the crate. Production receipts can therefore identify the only
  production converter path as `uk2us`. Converter failure and fault injection
  remain testable through module-private controls.
- **EDIT-A-002 — resolved.** Packet validation runs before the no-op decision,
  requires schema version 1 and exactly the manuscript and supplement in the
  top-level `draft_outputs` array, rejects extra fields/outputs, and also
  requires each path/digest pair to occur exactly once in the complete packet.
  Digest replacement is restricted to the unique textual `draft_outputs`
  array rather than recursively editing unrelated packet records. Negative
  contracts cover missing, misplaced, duplicate, globally duplicated, and
  extra outputs.
- **EDIT-A-003 — resolved.** The transaction captures the complete v2 byte/mode
  snapshot under its lock. Manifest, content, packet, and catalog candidate
  reads must match that held snapshot; receipt old identities come directly
  from it. The selected graph and complete tree are rechecked before no-op or
  install, cloning checks every source entry, and both current and staged trees
  must match their expected snapshots before exchange. Deterministic selected-
  content, unrelated-tree, staged-tree, and transient-catalog drift cases fail
  closed.
- **EDIT-A-004 — resolved.** Rollback-capable faults restore the complete old
  generation and sync the parent. After the new generation validates, cleanup
  failure no longer exchanges a partly removed old generation back into
  service; it returns typed `CommittedCleanup` with valid deterministic receipt
  JSON and leaves the valid new generation active. Recovery-state detection now
  occurs immediately after lock acquisition, so retained
  `.v2.normalize.next` state blocks every later operation, including a no-op
  check. The cleanup-fault contract proves the committed graph validates,
  receipt roots differ, recovery state remains explicit, and retry fails
  closed.
- **EDIT-A-005 — resolved.** The held and staged snapshots include root,
  directory, and file permission modes. Clone and replacement paths preserve
  those modes through descriptor-relative no-follow operations, validate them
  before exchange, and the integration contract compares the complete mode
  map before and after apply. `fchmod` is followed by `sync_all`, so the newly
  stated durability boundary covers inode mode metadata as well as bytes and
  directory entries.
- **EDIT-A-006 — resolved.** Focused contracts now cover current no-op,
  read-only drift, apply/rebind/build/check/idempotence, equivalent receipt
  equality, valid review-lock and non-DRAFT refusal paths, stale packet
  semantics, missing/nonzero/non-UTF-8/non-idempotent converter behavior,
  large streaming input, source/staged drift, exchange/sync/post-install
  rollback, cleanup failure, recovery retry, mode preservation, and CLI mode,
  selection, and foreign-option isolation.
- **EDIT-A-007 — resolved.** `artifacts/line-count-governance.md` records every
  touched Rust file. `v2.rs` is 2,841 lines, remains below the 3,000-line block,
  and now has a concrete schema/model-versus-validation decomposition intent,
  scientific-assurance build-maintainer ownership, and a 3,000-line blocking
  sunset. All other touched Rust files are below 2,000 lines.

### Additional Remediation Audit

Review A identified two issues while verifying the first remediation handoff:
retained cleanup state could be bypassed by the no-op return and `fchmod`
metadata was not explicitly synced. Both are closed as described under
EDIT-A-004 and EDIT-A-005. Review B's converter-pipe concern is also closed:
stdout and stderr are drained concurrently while input is written, unavailable
pipes kill and reap the child, all capture threads are joined, and the focused
test repeatedly exercises nonzero stderr plus a 2 MiB streaming round trip.

No new implementation, security, transaction, CLI, packet-binding, or evidence
finding remains in Review A's scope.

### Final Executed Evidence

- Ran: final `cargo fmt --check` and targeted
  `cargo clippy -p openwepp-assurance --all-targets -- -D warnings` passed.
- Ran: the final normalization module selection passed `8/8`; the maintainer's
  combined normalization/transaction unit selection passed `10/10`.
- Ran: `assurance_v2_normalization_contract` passed `5/5`.
- Ran: the final `assurance-editorial` profile passed `65/65`.
- Ran: the real production normalization check emitted a no-change receipt
  with equal report roots, selected-report validation passed at lifecycle
  `DRAFT`, and before/after SHA-256 comparison proved the catalog, descriptor,
  manuscript, supplement, and packet were unchanged.
- Ran: final candidate `git diff --check` passed.
- Static: the proportional profile still includes the complete assurance crate
  plus source, planner, assembly, normalization, and report-facing integration
  contracts while keeping publication changes on the full-gate path.

Review A therefore recommends **PASS** for the remediated implementation. Full
package closure still requires the package's independent Review B, heavy root
gates and adjudicated CRAP evidence, finding disposition, and dual terminal
verification; those pending workflow artifacts are not unresolved Review A
implementation findings.

## CRAP-Remediation Re-review

Evidence class: **Static + Ran**

Final recommendation: **PASS**

The current normalization implementation reviewed here has SHA-256
`eb4f51a0f2258ca32c819960db98f07f1adf2523e224b9a170e9372a2ecbd57b`.
The first decomposed handoff had SHA-256
`7216fcb163437a1af2be58de9ae0e5a7eab9a7e4765ad26acf9582f09174a234`;
the current hash also includes the test-fixture correction dispositioned
below.

### Comparator Limitation

The previously reviewed `normalization.rs` was untracked, and no byte snapshot
of SHA-256
`89a83b4351581b08719cda4d8e42a7d1e75e9ef9bc0534077128b42c41388a1f`
is available in the repository. Review A therefore cannot claim a literal
pre/post textual comparison or bytewise proof that extraction alone preserved
behavior. This re-review instead performed a complete static audit of the
current file against the package contract, the accepted CRAP-remediation
decision, and every EDIT-A invariant, supplemented by focused execution.

### Static Preservation Audit

- The entry point retains the required ordering: validate controls, open and
  lock the confined source, reject retained recovery state, prepare a candidate
  from a held complete-tree snapshot, return a validated no-op receipt when
  appropriate, recheck drift, and only then apply the transaction.
- Preparation still validates the selected graph before and after conversion,
  derives the old root from the held tree, validates the agent packet even on a
  no-op, confines digest rebinding to the unique required packet outputs,
  rebinds the report and catalog, and computes deterministic changes from the
  held snapshot.
- Application still verifies all base inputs, installs the complete candidate,
  restores the old generation for rollback-capable faults, reopens and
  validates the committed graph, emits the deterministic committed receipt,
  and distinguishes post-commit cleanup failure through typed retained
  recovery state.
- Tree cloning still creates the staged hierarchy, verifies and clones every
  regular file, preserves file modes, restores directory modes from leaves to
  root, and validates the staged snapshot before exchange. The extracted
  helpers neither broaden path authority nor add a fallback path.
- EDIT-A-001 through EDIT-A-007 remain resolved: converter and fault controls
  are private; packet outputs are exact, globally unique, and narrowly
  rebound; the complete byte/mode snapshot governs all candidate reads and
  drift checks; rollback and committed cleanup states remain distinct;
  metadata is preserved and synced; negative and determinism contracts remain
  present; and `normalization.rs` remains below the line-count warning at
  1,551 lines.

No implementation, confinement, transaction, packet-binding, CLI, or
behavior-preservation defect was found in the decomposed production paths.

### Re-review Finding And Disposition

**RR-A-001 — resolved before disposition.** On the first decomposed handoff,
the isolated normalization and integration selections passed, but the complete
editorial profile failed
`v2::normalization::tests::controlled_converter_errors_are_typed`. The
non-UTF-8 shell fixture emitted output and exited without consuming stdin. A
valid scheduling outcome therefore allowed the production converter wrapper to
observe and return `BrokenPipe` before classifying the captured output, so the
test did not deterministically isolate its intended non-UTF-8 condition.

The fixture now consumes stdin before emitting invalid UTF-8. This preserves
the production wrapper's explicit stdin-write error semantics while making the
negative contract deterministic. The exact corrected test passed 20
consecutive nextest invocations, and the complete focused profile then passed.
No production change was required for this finding.

### Executed Evidence

- Ran: `cargo fmt --check` passed.
- Ran: `cargo clippy -p openwepp-assurance --all-targets -- -D warnings`
  passed.
- Ran: the normalization module selection passed `8/8`.
- Ran: `assurance_v2_normalization_contract` passed `5/5`.
- Ran: the corrected converter error contract passed `20/20` consecutive exact
  invocations.
- Ran: `cargo nextest run --workspace --profile assurance-editorial` passed
  `65/65`.

Review A recommends **PASS** for the current CRAP-remediated implementation.
This review did not rerun the coverage-derived adjudicated CRAP gate and makes
no independent claim about the new numeric scores; fresh CRAP evidence and the
remaining heavy closure belong to the package's heavy-gate and terminal
verification workflow.
