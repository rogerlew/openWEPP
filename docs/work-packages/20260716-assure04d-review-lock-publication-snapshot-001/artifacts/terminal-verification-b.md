# ASSURE-04D Terminal Verification B

Status: **PASS**

Verified: `2026-07-16T09:19:35Z`

Evidence class: Static + Ran

This is independent engineering terminal verification B. It verifies package
mechanics and evidence; it is not scientific, reproduction/publication,
release-owner, or named-human approval. No production, test, authority,
protected, public, catalog, roadmap, queue, prompt, or package-state file was
edited. This artifact is the verifier's only repository write.

## Verdict

The current implementation, actual consumers, review disposition, preserved
HOLD chronology, third heavy closure, fresh adjudicated-CRAP bundle, protected
surface, line counts, and declared write set satisfy ASSURE-04D's technical
acceptance criteria. No undispositioned finding or deferred current-scope gate
was found.

This PASS is one half of the required dual terminal verification. It does not
alone authorize package closeout or prompt archival; the parent may perform
those administrative steps only after terminal verification A also records
PASS.

## Heavy-Gate And CRAP Reconstruction

The complete heavy report preserves all three sequences rather than replacing
earlier failures:

| Sequence | Reconstructed disposition |
| --- | --- |
| First | HOLD at workspace/all-target strict Clippy; the runner correctly stopped before full Nextest, deny, and CRAP. |
| Second | HOLD after format, strict Clippy, full Nextest 2,043/2,043, and deny passed; fresh CRAP contained 9 raw, 2 established adjudications, and 7 actionable touched-file rows. |
| Third | PASS in the required order: format, strict Clippy, full Nextest 2,046/2,046, deny, and fresh adjudicated CRAP with 2 raw, 2 established adjudications, and 0 actionable rows. |

The third full-profile JUnit remains present at SHA-256
`fe2c4a83d620a81794685e7593a37b0abd1369e6d763c406d40bcbb7c30b847e`.
Its root records run ID `9438c097-eccb-4959-88df-fb860cc64fdb`, 2,046 tests,
zero failures, and zero errors. The terminal verifier did not rerun the full
workspace or CRAP acquisition.

Both evidence bundles were checked independently. All 16 entries in the fresh
canonical `sha256sums.txt` passed. The archived HOLD-02 manifest intentionally
retains its original canonical directory prefix; replaying the same manifest
with that prefix remapped in-memory to `adjudicated-crap-hold-02/` passed all
16 entries. The preserved report remains FAIL with 7 actionable rows, while
the canonical report is current-source closure-eligible PASS with zero
actionable rows. Both bundles contain the same unmodified adjudication registry
at SHA-256
`10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`;
no new adjudication hid the remediation.

All 437 current measurement inputs matched the final source manifest. Its 228
production sources remain represented by SHA-256
`16e5bcb05297d5ca73ff1617242d019ee54063bf29a4dfa12b3f4c34fe30cf02`.
Independent extraction from the canonical `workspace-crap.json` produced:

| Touched production file | Entries | Maximum CRAP | Maximum function |
| --- | ---: | ---: | --- |
| `cli.rs` | 76 | 30 | `publish_selected` |
| `lib.rs` | 0 | N/A | no measured function and no actionable row |
| `v2.rs` | 194 | 23 | `validate_report_sections` |
| `v2/assembly.rs` | 134 | 25 | `render_directive` |
| `v2/confined.rs` | 172 | 30 | `remove_regular_if_exists_platform` |
| `v2/lifecycle.rs` | 24 | 23.957831631357006 | `validate_date` |
| `v2/publication.rs` | 178 | 26.57365194382251 | `read_prior_public` |

CRAP debt is strictly greater than 30, so every touched-file maximum is at or
below the package gate and the workspace actionable set is empty.

## Implementation And Actual Consumer Audit

The current call chain is complete:

1. The CLI has distinct `publish` and `publish-test-fixture` commands and
   requires explicit staging, public, immutable-artifact, commit, and
   configuration inputs. Production and test entry points select different
   typed trust domains.
2. Publication holds descriptor capabilities for source, staging, public, and
   snapshot roots; validates pairwise ancestry and ambient identity; captures
   exact staging/source bytes; and replays them immediately before commit.
3. The public generation contains an exact machine-owned `catalog.json`,
   reader-facing README, report, supplement, research objects, figures, and
   links. Named publication preserves only receipt-verified peers; all mode
   makes the selected catalog/report set exact.
4. WEPPcloud's current usersum route imports
   `cmarkgfm.github_flavored_markdown_to_html`. Renewed rendering through that
   exact installed function passed the retained README, report, and supplement
   at 1,017/1,193, 15,545/18,111, and 5,649/6,777 Markdown/HTML bytes. Each
   retained `TEST ONLY`, and no raw script or unresolved assembly token appeared.
5. Snapshot and receipt names are hashes of their exact canonical bytes. The
   retained manifest and receipt independently hash to their path identities
   `e5348b835da39192a1d5c257cb44fdff5fef0a2edb11b44c20b273440e4ea647`
   and
   `5b10bd8f50dbf0283d0519e75c4078c82ed3b7e04e6db75f206e226da7a120b1`.
6. `verify_v2_release_snapshot` verifies complete manifest membership, file
   sizes and hashes, release identity, receipt maps, builder identity, public
   digest, and production trust domain. It then opens each captured source as a
   strict v2 repository, revalidates roles/lifecycle/approvals/transfer, rebuilds
   every layered root and public byte, and compares the complete reconstructed
   source/public payload. A merely self-hashed container is not authority.
7. `check_assurance_release_transition.sh` invokes that production verifier
   before release-directory creation and binds commit to checkout `HEAD` and
   configuration to `openwepp-release-default-v1`.
   `materialize_assurance_v2_release.sh` copies the content-addressed artifacts,
   verifies the copies again, writes the discovery sidecar, and checks
   `SHA256SUMS`. `run_release_candidate_gates.sh` calls both consumers.

The renewed 69-test focused run executes this real preflight/materializer path,
including copied-artifact reverification and discovery/checksum assertions. The
retained selected test also proved that the actual release driver rejects the
test-only snapshot before creating a release directory.

## Confinement, Atomicity, Durability, And Isolation

Static inspection confirms descriptor-relative Unix `openat`/`mkdirat` reads
and writes with no-follow and regular-file checks. Multiply linked inputs,
symlinks, special files, path escape, root replacement, lexical aliases, and
mounted ancestry are rejected. Public and snapshot roots are locked in stable
identity order.

Prepared files are synced, prepared trees are synced bottom-up, and parent
directories are synced before Linux no-replace rename. Immutable snapshot and
receipt conflicts compare exact bytes and cannot replace an existing identity.
The sole public commit is a complete directory no-replace rename or atomic
exchange. All injected precommit failures preserve the old public generation;
post-commit cleanup and diagnostic sync are deliberately best effort so a
successful exchange cannot be misreported as a precommit failure. Concurrent
writers serialize, and the reader contract observes a complete old or complete
new report byte stream.

Production verification requires production source, catalog, manifest, and
receipt domains and rejects fixture markers or `TEST ONLY` bytes. Test-only
publication uses separate entry points and visibly marked retained artifacts.
No production verifier path can promote the synthetic evidence.

## Acceptance Matrix

| Package acceptance requirement | Terminal-B result |
| --- | --- |
| Canonical subject/root completeness, stability, mutation sensitivity, and self-reference avoidance | PASS — layered-root implementation and source/publication contracts cover every classified leaf and dependency. |
| Fail-before-mutation lifecycle, root, dependency, finding, approval, withdrawal/supersession, and unauthorized-fixture negatives | PASS — focused suites execute the complete negative matrices and precommit invariants. |
| Exact named human roles over one root; builder grants no approval | PASS — lifecycle validation requires the exact distinct role set, competence, independence, ledger binding, and supplied dates; only synthetic test data exercises positive approval. |
| Named/all exact checked-staging promotion and peer preservation | PASS — shared per-report path, named-peer, all-mode, staging-drift, and byte-equality contracts pass. |
| Approved-only reader catalog, exact cleanup, and resolved portable links | PASS — retained catalog is typed and reader-facing; parser-derived link checks and named/all catalog ownership contracts pass. |
| Actual usersum rendering and accessible/portable report surfaces | PASS — renewed with WEPPcloud's installed `cmarkgfm` function; renderer proof and focused accessibility/link contracts pass. |
| Snapshot/source/public reconstruction, immutable retry/conflict behavior, and release binding | PASS — content identities, complete authority replay, retry/race/conflict, multi-report, and forged-authority negatives pass. |
| Synthetic test-only confinement and release rejection | PASS — retained test-only tree regenerated byte-for-byte and the actual release driver rejected it before output creation. |
| Failed publication leaves prior public/catalog/unrelated bytes complete | PASS — three fault boundaries, receipt conflict/retry, serialized writers, and distinct-generation reader contracts pass. |
| Offline deterministic descriptor confinement and hostile-root/path/special-file rejection | PASS — implementation and focused negatives confirm the required properties; no wall-clock or environment-derived scientific/release authority exists. |
| Canonical groundwater remains nonpublic `DRAFT` | PASS — `report.yaml` remains `DRAFT`, `test_only`, and `fixture_only`, with empty approvals and null subject/finding/approval/transfer/public identities; it is absent from tracked public surfaces. |
| Protected bytes and aggregate usersum identity | PASS — protected diff is empty; all four named hashes equal intake and aggregate `usersum/**` remains `deb9f2c646aa5eb4ad9e427f8a7cec6ad51e3f9f6b47ffe29d14b4aed4bdcb7a`. |
| Required focused, quick, full, formatting, Clippy, deny, docs, CRAP, review, and disposition evidence | PASS — current focused/static renewal and frozen heavy artifacts provide direct evidence; no failed sequence is reused as closure. |
| No undispositioned finding, technical deferral, CRAP excess, or 3,000-line file | PASS — disposition accepts and closes every finding; current heavy closure is PASS; actionable CRAP is zero; governed line maxima remain below 3,000. |

## Terminal-B Renewed Commands

| Command or check | Result |
| --- | --- |
| `cargo fmt --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| `cargo deny check` | PASS — advisories, bans, licenses, and sources |
| `cargo nextest run --profile quick -p openwepp-assurance --lib` | PASS — 17/17, run `e5d037f5-e74a-42ff-bb37-0d3067d362e4` |
| Five focused assurance integration suites | PASS — 69/69, run `97abfe01-7fb0-49bd-b813-d175904b0baf` |
| Retained selected publication plus `diff -qr` | PASS — 1/1 with 24 skipped, run `1c6670e1-e864-4eae-b3a4-41b181adcf0c`; regenerated tree byte-identical |
| Actual `cmarkgfm` rendering of retained README/report/supplement | PASS |
| Canonical and archived CRAP checksums and current source-manifest replay | PASS |
| Protected diff, four named hashes, aggregate usersum hash | PASS |
| Three release scripts under `bash -n`; `git diff --check` | PASS |

No new lint suppression, CRAP adjudication, ignored test, coverage exclusion, or
semantic bypass was introduced. The only scoped lint allowance found is the
preexisting `assembly.rs` `too_many_arguments` allowance already present at the
frozen base.

## Governance And Prompt State

Every current changed or untracked path is inside the package's declared write
set; the third heavy freeze independently recorded zero out-of-set paths and a
stable Git index. Governed production Rust line counts remain: `cli.rs` 661,
`error.rs` 72, `lib.rs` 24, `v2.rs` 2,821, `assembly.rs` 1,747,
`confined.rs` 1,293, `lifecycle.rs` 349, `planner.rs` 1,182, and
`publication.rs` 2,982. No nonexempt file reaches the 3,000-line block.

Pre-closeout governance state is consistent: ASSURE-04D remains active in the
package, work-package catalog, and roadmaps; ASSURE-05 remains queued and
blocked; final disposition and worker handoff remain administrative follow-up.
Those placeholders do not defer a technical 04D gate or authorize ASSURE-05.

The active execution prompt remains byte-present only at
`prompts/active/20260716-codex-execute-assure04d_prompt.md`, with SHA-256
`d19fe5f728dc0a138c20f393f184ac329798e3f4eae2e46e50a4996d42cf221a`.
No archived copy exists. Package Phase 6 requires its byte-for-byte move only
after both terminal verifiers pass; terminal B therefore leaves it untouched.
