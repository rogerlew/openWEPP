# ASSURE-04D Terminal Verification A

Status: PASS — independent terminal engineering verification

Verified: 2026-07-16 UTC

Evidence class: Static + Ran

## Scope And Boundary

This verifier read the complete current package and frozen publication
contract; both independent review histories and their disposition; both
preserved heavy HOLDs and the third heavy PASS; the current and archived CRAP
bundles; retained synthetic publication; renderer proof; release preflight and
materializer; protected, write-set, line-count, roadmap, catalog, prompt, and
acceptance evidence; and the current implementation and exact consumer tests.

This is an engineering verification only. It does not authenticate a person,
assess scientific competence, approve a scientific result, grant reproduction
or publication approval, authorize a release, or authorize ASSURE-05.

## Heavy Chronology And Evidence Integrity

Both earlier Phase 5 HOLDs remain intact and are not relabeled:

1. the first sequence stopped at eight strict-Clippy diagnostics in the
   publication integration test; full, deny, and CRAP were not run; and
2. the complete restarted sequence passed format, strict Clippy, 2,043 full
   tests, and deny, then stopped because fresh CRAP reported 9 raw, 2
   adjudicated, and 7 actionable rows.

The archived second-HOLD directory contains 17 files. After resolving its
intentionally retained canonical path prefix at verification time, all 16
entries in its unchanged checksum manifest passed. Its before, after, and final
source manifests compare byte-for-byte equal, and `run-status.json` remains
fresh `FAIL`, exit 1. No historical failure evidence was overwritten by the
third run.

The third sequence is a new frozen-tree restart, not a reuse of passing results
from either HOLD:

| Required gate | Terminal result |
| --- | --- |
| `cargo fmt --check` | PASS |
| Workspace/all-target strict Clippy | PASS |
| Full workspace Nextest | PASS — 2,046/2,046, 0 failures; run `9438c097-eccb-4959-88df-fb860cc64fdb` |
| `cargo deny check` | PASS — advisories, bans, licenses, and sources |
| Fresh adjudicated CRAP | PASS — raw 2, adjudicated 2, actionable 0, touched files 7 |

The retained full JUnit file declares 2,046 tests and zero failures/errors and
has the recorded SHA-256
`fe2c4a83d620a81794685e7593a37b0abd1369e6d763c406d40bcbb7c30b847e`.
The current canonical CRAP bundle contains 17 files; all 16 checksum-manifest
entries pass. Its three production-source manifests compare exactly. All 228
listed production sources and all 437 measurement inputs still match the final
manifest, HEAD remains the frozen base, and the NUL-form Git-index identity is
`424cff7bd0c6acac537e816a5bf449768152331dc21c602248656cd14fae3a0b`.

The current CRAP report is fresh, closure-eligible, and reports both acquisition
and debt status `PASS`. The only raw rows above 30 are the existing exact
`CQR-LOW-L08` and `CQR-LOW-L11` adjudications outside the touched files. The
registry is byte-identical to the canonical registry at SHA-256
`10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`;
there are no invalid adjudications.

| Touched production file | Entries | Maximum CRAP | Maximum function |
| --- | ---: | ---: | --- |
| `cli.rs` | 76 | 30.0000 | `publish_selected` |
| `lib.rs` | 0 | N/A | no measured function row; no actionable row |
| `v2.rs` | 194 | 23.0000 | `validate_report_sections` |
| `v2/assembly.rs` | 134 | 25.0000 | `render_directive` |
| `v2/confined.rs` | 172 | 30.0000 | `remove_regular_if_exists_platform` |
| `v2/lifecycle.rs` | 24 | 23.9578 | `validate_date` |
| `v2/publication.rs` | 178 | 26.5737 | `read_prior_public` |

Every touched maximum is therefore at or below the adjudicated closure limit
of 30, and the workspace actionable set is empty.

## Authority, Isolation, And Exact Consumers

Static implementation review confirms the required layered authority:

- the subject root binds admitted source, schemas, catalog, principals, tools,
  inputs, generated staging, and narrative identities while named transition
  leaves enter later layers;
- the finding ledger binds the subject, charge, producers, maintainer, and
  complete stable findings;
- every approval must bind the calculated finding ledger; exactly three
  distinct declared human roles, competence and independence declarations, and
  conflict rules are required before the approval lock is accepted;
- the transfer binds the approval lock to an independently supplied commit and
  configuration;
- the snapshot binds the exact source/public payload and release identity; and
- the immutable receipt binds every prior root and exact public tree and is the
  sole mechanical authority for derived `PUBLISHED` state.

Production and synthetic entry points remain separate in both the Rust API and
CLI. Production publication accepts only `production`; fixture publication
accepts only `test_only`; the production verifier always requests production
authority, rejects an empty catalog and any fixture marker, reconstructs every
source repository and layered root, and requires exact source, catalog, public,
manifest, receipt, builder, and release agreement.

The real downstream chain is executable rather than producer-only:

1. `publish`/`publish-test-fixture` bind every explicit root and release option
   and dispatch to the distinct repository entry point;
2. publication consumes descriptor-held checked staging bytes, prepares a
   complete generation, installs or confirms no-replace snapshot and receipt,
   replays source and held-root authority, then commits only through atomic
   whole-generation exchange;
3. `verify-release` reconstructs production authority rather than accepting a
   self-hashed container;
4. `check_assurance_release_transition.sh` invokes that verifier before the
   release runner creates `RELEASE_DIR`;
5. `run_release_candidate_gates.sh` invokes the same materializer; and
6. `materialize_assurance_v2_release.sh` verifies before copying, verifies the
   copied snapshot and receipt again, emits the receipt-discovery sidecar, and
   checksums the copied authority.

The current publication suite executes this chain, including mechanically
approved production reconstruction, two-report replay, the actual preflight,
materializer, copied-byte comparison, discovery sidecar, and checksum check.
It also executes synthetic rejection before release-directory creation,
forged/empty authority negatives, lifecycle/root/role/bound-byte negatives,
receipt conflicts and retry, fault boundaries, complete old/new reads,
confinement, special-file rejection, and named/all exact-set behavior.

## Retained Consumer And Public Boundary

The retained synthetic evidence reconstructs exactly:

- snapshot ID
  `e5348b835da39192a1d5c257cb44fdff5fef0a2edb11b44c20b273440e4ea647`
  equals the manifest digest;
- receipt ID
  `5b10bd8f50dbf0283d0519e75c4078c82ed3b7e04e6db75f206e226da7a120b1`
  equals the receipt digest;
- all 34 manifest payload entries match path, bytes, and SHA-256;
- retained public bytes equal snapshot `public/` bytes; and
- the independently reconstructed public-tree digest is
  `0547ffeb3e3c843ed727a3791d2c3443057b2be2181f674cc31379ba98259aef`.

Manifest, receipt, catalog, README, report, and supplement retain the explicit
`TEST ONLY — NOT SCIENTIFICALLY APPROVED` boundary. A direct current-tree
production `verify-release` invocation rejects the retained snapshot with a
trust-domain error.

The actual WEPPcloud usersum consumer still imports cmarkgfm. Re-rendering the
retained README, report, and supplement with that installed function reproduced
the recorded byte/link/table/image counts exactly; every output retained the
test marker, with no unresolved assembly braces or raw script element. This
proves the claimed Markdown renderer only, not WEPPcloud discovery, vendoring,
scientific review, or release acceptance.

The canonical groundwater report remains `DRAFT`, `test_only`, and
`fixture_only`, with null human lead/scientific approver and no approval or
transfer root. The tracked v1-retired catalog remains zero-report. Direct
current-tree validation reports one internal DRAFT fixture and zero public
reports. The synthetic and mechanically approved production fixtures exercised
by tests are disposable engineering fixtures, not approval records.

## Independent Current-Tree Renewal

This verifier did not rerun full Nextest or CRAP. Proportionate current-tree
execution passed:

- `cargo fmt --check`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo deny check`;
- assurance library tests, 17/17, run
  `b2589818-625c-4d9f-81bd-d63bfea5ad3e`;
- publication and downstream-consumer contracts, 25/25, run
  `83a76716-8f4a-44b3-bc14-5776be6032a9`;
- release-transition validation and zero-report release preflights;
- all three release-script syntax checks; and
- `git diff --check`.

Protected hashes equal intake for the legacy catalog, catalog template,
generated export, and tracked usersum README. The complete sorted `usersum/**`
hash stream remains
`deb9f2c646aa5eb4ad9e427f8a7cec6ad51e3f9f6b47ffe29d14b4aed4bdcb7a`,
and the protected diff is empty. The 132-path complete status inventory has
zero paths outside the amended write set.

Current governed line counts match the third heavy freeze. No production Rust
file reaches 3,000 lines. `v2.rs` at 2,821 and `v2/publication.rs` at 2,982
retain their required WARN dispositions; the other governed files are below
2,000.

## Acceptance, Non-Deferral, And Prompt State

Every substantive ASSURE-04D acceptance row has current direct evidence:
layered root and lifecycle integrity; fail-before-mutation negatives; exact
named/all staging and catalog behavior; renderer/link/accessibility
compatibility; immutable snapshot/receipt reconstruction; release identity and
materialization; test-only isolation; transaction/confinement behavior;
canonical DRAFT/zero-public state; protected/write-set/line closure; dual review
and accepted-finding remediation; and the complete third heavy sequence. No
review or heavy finding is undispositioned or deferred to ASSURE-05.

The roadmap and package catalog correctly keep ASSURE-04D active and ASSURE-05
queued and blocked pending this package's terminal closeout. Administrative
final-disposition, handoff, queue, and roadmap edits are intentionally not
prerequisites to an independent verifier's decision; they occur only after both
terminal verifiers return PASS.

The active execution prompt is still byte-present at 2,902 bytes with SHA-256
`d19fe5f728dc0a138c20f393f184ac329798e3f4eae2e46e50a4996d42cf221a`.
No same-named archived copy exists. Package Phase 6 and prompt-directory
governance require that exact byte sequence to move to `prompts/archived/` only
after both terminal PASSes; leaving it active during this verification is
correct and is not a closure deferral.

## Verdict

**PASS.** Terminal verification A found no engineering closure blocker. This
decision makes no scientific, reproduction, publication, release-owner, or
human-approval judgment.
