# Independent Review B: Test And Governance

Evidence class: **Static + Ran**

Recommendation: **PASS**. `B-01` was corrected and independently reverified;
there are no unresolved Review B findings.

This was an independent read-only review of the terminal working-tree diff
against frozen base `25bcb17f4a62924976a19381e974a36612ed4845`. I reviewed
the package, applicable root/crate/test/work-package instructions, production
and test changes, profile selection, and user-facing governance documents. I
did not read Reviewer A's artifact. The only repository file written by this
reviewer is this artifact.

## Resolved Finding

### B-01 — RESOLVED — Medium: converter error handling was scheduling-sensitive

Ran: an independent
`cargo nextest run --workspace --profile assurance-editorial` selected the
intended 65 tests across seven binaries, but completed `64 PASS / 1 FAIL` (run
ID `15cf12c1-85dd-4bf0-a758-bb958f824078`). The failure was
`v2::normalization::tests::controlled_converter_errors_are_typed`: the
nonzero converter error did not contain its required `denied` standard-error
diagnostic. Five immediate isolated reruns passed, demonstrating that the
contract is intermittent rather than consistently wrong.

Static: `run_converter` writes all input before calling `wait_with_output` and
returns immediately if that write fails
(`crates/openwepp-assurance/src/v2/normalization.rs:514-531`). A converter that
exits quickly can therefore close its input before the parent writes. Depending
on scheduling, the caller sees either the intended nonzero status and captured
standard error or an earlier broken-pipe I/O error; the child was also not
explicitly reaped on the early-write-error path. The observed profile failure
under concurrent workspace load was consistent with this race. The same
write-before-read arrangement could deadlock with a streaming converter and an
input/output pair larger than pipe capacity.

Disposition — fixed and verified: the corrected implementation drains standard
output and standard error concurrently while writing input, always waits for
the child, gives nonzero status and standard error precedence over a broken
pipe, and surfaces an input-write error only when the child succeeds. The
contract now repeats the immediate nonzero-exit case 32 times and sends 2 MiB
through a streaming `cat` converter. An intermediate corrected revision failed
warnings-as-errors Clippy on its pipe destructuring; that was also corrected.
Independent final verification passed package Clippy and the complete 65-test
editorial profile. `B-01` is resolved without weakening its diagnostic
assertion or masking the production race.

## Boundary And Coverage Assessment

Static: the public API now fixes the production executable to `uk2us`; converter
and transaction controls are private module seams used only by unit contracts.
The operation is limited to `en-US`, one named pre-review `DRAFT`, manuscript
and supplement bytes, and the packet/descriptor/catalog digest cascade. It
rejects review-authorized or non-DRAFT sources, malformed packet cardinality,
non-idempotent or failed converters, active-source drift, staged-generation
drift, and retained recovery state.

Static: the held tree snapshot binds file digests and file/directory modes.
Candidate reads are checked directly against that snapshot, the mechanically
expected staged tree is checked before exchange, the parent directory is
synced around exchange and cleanup, and pre-commit/exchange/sync/validation
failures restore or preserve the old generation. Post-validation cleanup
failure is correctly distinguished: the valid new generation remains active,
the typed error carries the committed deterministic receipt, and retained
recovery state blocks another operation pending explicit disposition.

Static + Ran: the former assembly test's thread, polling loop, 8 MiB padding,
spin wait, and wall-clock deadline are removed. A private synchronous hook now
mutates a real identified result after installation; the real post-install
input verifier detects drift, restores exact prior selected staging bytes, and
removes `.next`, `.previous`, and `.restore` artifacts. The replacement unit
contract is selected by the editorial profile.

Static: the final profile boundary is proportionate. It includes the complete
`openwepp-assurance` crate (therefore normalization, confinement, lifecycle,
publication-transition, and deterministic assembly unit invariants), plus v2
source, planner, assembly, normalization, and report integration contracts. It
excludes the long publication integration suite because normalization is
pre-review DRAFT-only. Documentation sends lifecycle, approval, authority,
schema/builder, mixed, unclear, and publication changes to ordinary full gates;
no scientific review, publication, release, or root-renewal requirement is
weakened.

## Ran Evidence

| Check | Result |
| --- | --- |
| `cargo nextest list --workspace --profile assurance-editorial` | `65` tests, seven binaries; private assembly rollback and normalization negative contracts selected; publication integration excluded |
| `cargo nextest run --workspace --profile assurance-editorial` | **FAIL**, `64/65`; run ID `15cf12c1-85dd-4bf0-a758-bb958f824078`; `B-01` |
| Five isolated reruns of `controlled_converter_errors_are_typed` | `5/5 PASS`; confirms scheduling sensitivity rather than closure |
| Corrected `cargo clippy -p openwepp-assurance --all-targets -- -D warnings` | PASS |
| Corrected `cargo nextest run --workspace --profile assurance-editorial` | PASS, `65/65` across seven binaries; run ID `e8559173-6314-4db5-b37c-8e9e00a9e059` |
| Real built CLI `normalize --report linear-groundwater-reservoir-recurrence --language en-US --check` | PASS; deterministic no-change receipt; `assurance/v2` aggregate hash unchanged at `75d91bdd910fb42a80e089868949901ce3ecf96f3f320ccbcdb0b5a98685e145` |
| Real built CLI `validate --report linear-groundwater-reservoir-recurrence` | PASS; selected lifecycle remains `DRAFT`, source root `08e2b5e3b6444067db7204f790a6670af2d6f16bf1b733879cbc3e95d235dfa6` |
| `cargo fmt --check` | PASS |
| `git diff --check` | PASS |
| Timing/polling mechanism search over changed assembly source/test | PASS; no old test name, implementation-padding path, spawn/spin/deadline mechanism, or 8 MiB padding remains |
| `markdown-doc lint` and `markdown-doc validate` over nine implementation/package Markdown files | PASS; zero errors and warnings |
| `uk2us` preview over the same nine Markdown files | PASS; no proposed changes |

The corrected Phase 5 candidate identity initially reviewed included
normalization SHA-256
`89a83b4351581b08719cda4d8e42a7d1e75e9ef9bc0534077128b42c41388a1f`,
assembly SHA-256
`5df95bf39777bb83e8fc9559fdd4a0e96499597820443e4748420ca48b9357f5`,
confinement SHA-256
`eb7f06490b3c832760e009b6c974a2484c509eb0aee6129c37471c680d79d189`,
and Nextest configuration SHA-256
`211cee6937b8e300b8fcf60c96bdd9e8ef8a7072943c3e82e5695c71c6828046`.

## Line-Count Governance

| Rust file | Frozen base | Reviewed | Disposition |
| --- | ---: | ---: | --- |
| `crates/openwepp-assurance/src/v2.rs` | 2,821 | 2,841 | `WARN`: below 3,000; this package adds only module wiring and delegates all normalization behavior to a new module |
| `crates/openwepp-assurance/src/v2/assembly.rs` | 1,747 | 1,948 | PASS |
| `crates/openwepp-assurance/src/v2/confined.rs` | 1,293 | 1,380 | PASS |
| `crates/openwepp-assurance/src/v2/normalization.rs` | new | 1,551 | PASS; current CRAP-remediated candidate (initial Phase 5 review: 1,440) |
| `crates/openwepp-assurance/src/cli.rs` | 661 | 798 | PASS |
| `tests/integration/assurance_v2_assembly_contract.rs` | 809 | 759 | PASS |
| `tests/integration/assurance_v2_normalization_contract.rs` | new | 417 | PASS |

Decomposition rationale: `v2.rs` is a preexisting typed source/validation
hub, while this change keeps its substantive logic in `v2/normalization.rs`.
Split intent: the openwepp-assurance owner should extract a coherent remaining
catalog/schema or report-validation domain before `v2.rs` reaches 3,000 lines;
new normalization logic must remain in its owned module. No file reaches the
3,000-line closure blocker.

## Gate Non-Deferral And Recommendation

The package remains `ACTIVE` and correctly leaves heavy closure, fresh CRAP,
finding disposition, and dual terminal verification pending. Those sequenced
gates are not waived by this review. The only Review B finding was in the
declared implementation/test envelope, was corrected in place, and now passes
independent targeted and profile-level verification. Review B recommends
**PASS** for the corrected Phase 5 candidate, with no unresolved findings;
remaining sequenced package gates must still run before closure.

## CRAP-Remediation Re-Review

Evidence class: **Static + Ran**

Recommendation: **PASS** for current normalization SHA-256
`eb4f51a0f2258ca32c819960db98f07f1adf2523e224b9a170e9372a2ecbd57b`.
There are no unresolved Review B findings on this candidate.

### Comparison Limitation

The requested exact comparison against the previously reviewed normalization
SHA-256
`89a83b4351581b08719cda4d8e42a7d1e75e9ef9bc0534077128b42c41388a1f`
could not be performed. `normalization.rs` is an untracked new file, and its
prior bytes were not retained as a Git object or separate snapshot. This
review therefore does **not** claim bytewise pre/post equivalence. At the
producer's direction, I expanded the re-review to the full current production
module and its unit contracts against the package requirements and the prior
`B-01` requirements.

### Full-Current Static Assessment

Static: the current decomposition preserves a narrow public surface and keeps
converter selection and transaction fault seams private. Preparation still
holds one exclusive transaction lock, validates the existing report, captures
the complete v2 file/directory digest-and-mode snapshot, reads every candidate
input against that snapshot, requires pre-review `DRAFT`, and rejects any
selected or unrelated tree drift before installation.

Static: content conversion remains exact and idempotence-checked. Packet
cardinality and path/digest bindings are validated before rebinding; candidate
construction mechanically updates manuscript/supplement identities, packet,
descriptor, and catalog without reformatting unrelated content. The resulting
replacement map is checked against a mechanically derived whole-tree snapshot
before exchange.

Static: application still verifies inputs immediately before staging,
preserves file and directory modes, syncs the staged tree and parent directory,
validates the installed generation through a reopened repository, restores the
old generation on every rollback-capable path, and leaves a valid new
generation plus a deterministic receipt and blocking recovery state on
post-validation cleanup failure. Splitting preparation, candidate rebinding,
change construction, and tree cloning into named helpers did not expose new
authority or bypass paths in the full current source.

Static: `B-01` remains correctly resolved. Standard output and standard error
are drained concurrently with input writing; the child is waited; a nonzero
status and its standard error take precedence over a broken pipe; and an input
write error is surfaced only for an otherwise successful child. The final
test-only non-UTF-8 converter consumes its input before successfully emitting
an invalid byte. That makes the fixture test the intended encoding contract;
it does not weaken the separate repeated early-nonzero or 2 MiB streaming
contracts.

No new defect was found in the full current normalization implementation or
its selected contracts. The file is 1,551 lines, below the package's 2,000-line
warning threshold.

### Fresh Current-Hash Evidence

| Check | Result |
| --- | --- |
| `sha256sum crates/openwepp-assurance/src/v2/normalization.rs` | `eb4f51a0f2258ca32c819960db98f07f1adf2523e224b9a170e9372a2ecbd57b` |
| `cargo fmt --check` | PASS |
| `cargo clippy -p openwepp-assurance --all-targets -- -D warnings` | PASS |
| Exact `controlled_converter_errors_are_typed` Nextest selection | PASS; run ID `ada112d0-90d9-47f0-a96a-5c11bfafd55e` |
| `cargo nextest run --workspace --profile assurance-editorial` | PASS, `65/65` across seven binaries; run ID `83ceabc6-c697-4f0b-bc80-5b384cd3db5c` |
| Real CLI normalization check and selected-report validation | PASS; no-change receipt retained selected source root `08e2b5e3b6444067db7204f790a6670af2d6f16bf1b733879cbc3e95d235dfa6`; lifecycle remained `DRAFT` |
| `git diff --check` | PASS |

The editorial run reported two tests above Nextest's 60-second slow threshold
while another normalization gate sequence was active in the same worktree; all
tests completed successfully. That overlapping run is functional evidence, not
an isolated performance comparison with the unavailable prior source.

### Re-Review Disposition

Review B recommends **PASS** for the complete current candidate. This
disposition is based on full-current static review and fresh current-hash
execution, not on an unavailable exact decomposition diff. It does not waive
fresh adjudicated CRAP evidence or the remaining sequenced closure and terminal
verification gates.
