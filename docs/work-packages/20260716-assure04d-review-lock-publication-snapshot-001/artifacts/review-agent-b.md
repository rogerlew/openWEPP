# ASSURE-04D Independent Review B

Status: HOLD — findings dispositioned and remediated before heavy closure

Evidence class: Static + Ran

The reviewer ran the then-current publication suite (10/10 PASS),
`git diff --check`, and protected-hash checks, then independently audited
transaction, confinement, release, and consumer behavior. No scientific-
approval judgment was made and no files were edited.

Findings:

1. Critical: release verification admitted a self-hashed empty production
   container and arbitrary syntactic root maps rather than replaying authority.
2. Critical: cleanup or sync errors after atomic exchange returned `Err` even
   though the new generation was public; prepared file/directory durability was
   not explicitly synced.
3. High: the descriptor-held checked-staging/root model was incomplete,
   ancestry asymmetric, ambient normalization weak, and receipt hard links
   admitted.
4. High: any README-only tree was treated as bootstrap; empty unknown
   directories were invisible; narrative bytes, canonical paths, and real
   Markdown links were not enforced.
5. Critical: release commit/configuration came only from caller arguments and
   verified v2 artifacts were absent from the assembled release directory.
6. High: fault/crash/reader, special-file, draft/in-review, receipt conflict,
   root replacement, and two-report named/all proofs were incomplete.
7. Minor: protected/write-set checks passed; heavy/CRAP remained pending; the
   2,135-line publication module required WARN.

Verdict at review: HOLD.

## Remediation Review

Status: HOLD — second review findings accepted for remediation

The reviewer ran the then-current publication suite (19/19 PASS; nextest run
`15819572-af13-4a90-b141-6a0b6b351088`) and `git diff --check`. The second
review retained HOLD because:

1. raw target-string search admitted fenced/code/comment Markdown examples;
2. an exact transaction-owned receipt preparation could not recover after a
   crash;
3. the reader test republished identical bytes and could not distinguish old,
   new, or mixed output; and
4. release persistence remained source-inspection evidence instead of an
   executed downstream consumer.

The reviewer also requested stronger multi-report production, receipt-conflict,
and special-file evidence. No files were edited.

## Final Remediation Audit — First Pass

Status: HOLD — Markdown parser mismatch

The reviewer cleared receipt retry/conflict, distinct old/new report reads,
multi-report production replay, public/immutable special-file rejection,
executed release materialization, held-root checks, protected/write-set status,
and current focused evidence. HOLD remained because the hand-written link
recognizer admitted a target inside a multiline raw HTML block and after a
non-closing fence marker, while the actual cmarkgfm consumer produced no link
in either case. Both behaviors were independently renderer-confirmed. No files
were edited.

## Final Remediation Audit — PASS

Status: PASS

The reviewer confirmed parser-derived `pulldown-cmark` link events close both
renderer-confirmed bypasses. Publication contracts passed 23/23 (nextest run
`5bdc1ba2-fc07-499c-bf11-d7c66b4b70e9`), the targeted parser unit passed, and
the focused, quick, protected, line, formatting, script, and diff evidence
remained current. No files were edited and no scientific-approval judgment was
made.

## Bounded Post-HOLD Renewal — 2026-07-16 UTC

Status: PASS — bounded renewal only; prior heavy HOLD preserved

Evidence class: Static + Ran

The reviewer inspected the post-HOLD remediation in
`tests/integration/assurance_v2_publication_contract.rs`. Frozen production
file byte and line counts, manifests, lockfile, and materializer hashes remained
unchanged. The remediation was test-only: helper extraction, splitting two
negative matrices into four tests, removal of needless raw-string delimiters,
and replacement of a temporary formatting allocation with `write!`. The
increase from 67 to 69 focused tests is explained exactly by the two test
splits. No lint suppression, lint-configuration change, conditional evasion,
or equivalent bypass was introduced.

The reviewer reran these commands on the remediated workspace:

```text
cargo fmt --check
git diff --check
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run --profile quick --test assurance_v2_source_contract --test assurance_v2_planner_contract --test assurance_v2_assembly_contract --test assurance_v2_publication_contract --test assurance_dossier_build_contract
```

Formatting, diff hygiene, and workspace strict Clippy passed. The focused
nextest command passed 69/69 with run ID
`9f0d5cca-afe2-46c2-b0f5-aba952cb8f88`.

The reviewer did not rerun the complete workspace quick suite. The current
package evidence for
`cargo nextest run --workspace --profile quick` records 1,958/1,958 passing,
34 profile skips, three slow tests, and run ID
`d99c842d-8397-45bc-85d9-1d316ff0b4c3`. That evidence is credible: the prior
1,956-test population plus the two semantics-preserving test splits accounts
exactly for the new total, while strict Clippy and the focused suite passed
independently.

Markdown parser/renderer adversarial coverage remains intact, including
ordinary and malformed fence closers, multiline raw HTML, inline and indented
code, comments, escapes, and images. Publication fail-closed assertions and all
reviewed publication contract semantics remain represented.

This bounded PASS does not replace or erase the first Phase 5 strict-Clippy
HOLD. That HOLD remains historical evidence of the interrupted heavy sequence.
The interrupted sequence did not run full nextest, `cargo deny check`, or the
adjudicated CRAP gate; those gates require the documented complete independent
restart and cannot be inferred from this renewal. No production or test file
was edited during this review; this section is the only persisted review
artifact change. No scientific-approval judgment was made.

## Post-CRAP-HOLD Production Renewal — 2026-07-16 UTC

Status: PASS — bounded production renewal only; both prior heavy HOLDs preserved

Evidence class: Static + Ran

The reviewer compared the current tree with the canonical second-heavy-HOLD
source manifest. Exactly five assurance production files changed during the
seven-row remediation: `cli.rs`, `v2.rs`, `v2/confined.rs`,
`v2/lifecycle.rs`, and `v2/publication.rs`. `lib.rs`, `v2/assembly.rs`,
dependencies, manifests, and external integration tests remained byte-identical
to the HOLD snapshot. The three new tests are module-local. The adjudication
registry remains byte-identical at SHA-256
`10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`.

The complete seven-row disposition was independently audited:

| Second-HOLD row | Renewal result |
| --- | --- |
| `execute_publish` | Explicit option construction and trust-domain/selection dispatch remain mandatory; production and fixture entry points are still separate. |
| `parse_options` | Selection, build, publication, release, verification, and remaining-option helpers preserve missing, duplicate, unknown, and command-specific option rejection. |
| `validate_report_structure` | `validate_report_sections` still invokes every content, dependency, unit, claim, method, result, binding, table, figure, reference, and research-object validator before review/publication consistency and unused-field rejection. |
| `validate_review` | The lifecycle move preserves producer/finding uniqueness, complete finding-disposition checks, state-specific roots, exact three-role approval, distinct principals, ledger binding, competence, independence, and date validation. |
| `open_ambient_platform` | Absolute normalized components are opened descriptor-relatively with `O_NOFOLLOW`; optional creation uses `mkdirat`, parent sync, and descriptor reopen. Relative paths, parent traversal, symlink components, and create/open races fail closed. |
| `install_receipt` | Existing destinations and transaction-named preparations are accepted only on exact bytes. Different bytes return `SnapshotConflict`; `RENAME_NOREPLACE` prevents overwrite, and an exact race converges while a differing race fails and removes only owned preparation. |
| `verify_snapshot_content` | Snapshot directory ID still hashes exact manifest bytes; format, domain, marker, and release headers are checked; every confined path, size, digest, kind, duplicate, and complete tree membership is verified; public bytes and public-tree digest reconstruct before production authority replay. |

The surrounding transaction remains fail closed. Source and staging bytes and
held root identities are replayed before commit. Snapshot and receipt install
precede the public transition. All three injected precommit boundaries leave
the prior generation byte-identical, and the complete generation changes only
through no-replace rename or atomic directory exchange. Production verification
requires production-domain source, manifest, receipt, and catalog authority and
rejects fixture markers or `TEST ONLY` public bytes. No lint suppression,
coverage exclusion, ignored test, conditional evasion, CRAP allow-list entry,
or semantic relaxation was found.

The reviewer ran:

```text
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run --profile quick -p openwepp-assurance --lib
cargo nextest run --profile quick --test assurance_v2_source_contract --test assurance_v2_planner_contract --test assurance_v2_assembly_contract --test assurance_v2_publication_contract --test assurance_dossier_build_contract
```

Formatting and strict Clippy passed. Assurance library tests passed 17/17 in
nextest run `c2c47b2d-cdd8-49d2-ab83-915eb94624d5`; focused contracts passed
69/69 in run `f46dbdea-7bb9-4c07-927d-d210feb6011c`.

The reviewer also acquired isolated focused coverage under
`/tmp/openwepp-assure04d-review-b-cov.AxAK5t`: library tests passed 17/17 in
run `017dd17e-fcb3-4406-a658-069ab92cd0f8`, and the same five integration
suites passed 69/69 in run `781a9eb1-79c7-4eb7-9d68-31e7a7b156ec`.
Independent `cargo crap` evaluation of that LCOV reported zero rows greater
than 30 in every governed touched file:

| Touched production file | Focused maximum CRAP | Rows greater than 30 |
| --- | ---: | ---: |
| `cli.rs` | 30.0000 | 0 |
| `lib.rs` | no measured function | 0 |
| `v2.rs` | 23.0000 | 0 |
| `v2/assembly.rs` | 25.0000 | 0 |
| `v2/confined.rs` | 30.0000 | 0 |
| `v2/lifecycle.rs` | 23.9578 | 0 |
| `v2/publication.rs` | 26.5737 | 0 |

All extracted helpers were present in the scoped report. This focused measure
is diagnostic only and does not replace fresh full-workspace adjudicated CRAP.

The retained synthetic publication test passed 1/1 with 24 skipped in run
`0b9622be-d7c3-4e6d-91e5-cc2396681870`; a fresh temporary public/snapshot/
receipt tree compared byte-for-byte equal with `synthetic-publication/`.
`git diff --check`, the protected-surface diff, all three release-script
`bash -n` checks, four named protected hashes, and the aggregate `usersum/**`
hash passed. Current production line counts are `cli.rs` 661, `v2.rs` 2,821,
`v2/confined.rs` 1,293, `v2/lifecycle.rs` 349, and `v2/publication.rs` 2,982;
no nonexempt Rust file reaches the 3,000-line block. The current status remains
inside the declared write set.

This PASS renews independent review only. It does not replace the first heavy
strict-Clippy HOLD or the second heavy adjudicated-CRAP HOLD, and it does not
reuse the second sequence's format, Clippy, full-nextest, or deny results as
closure. Phase 5 still requires a new freeze and a complete five-gate restart,
including fresh full-workspace adjudicated CRAP. No production, test,
authority, public, protected, or queue-state file was edited during this
review; this section is the only persisted artifact change. No scientific,
reproduction, publication, release-owner, or human approval judgment was made.
