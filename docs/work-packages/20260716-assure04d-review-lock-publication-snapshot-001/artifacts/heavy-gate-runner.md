# ASSURE-04D Independent Heavy Gate Runner

Status: **HOLD -- strict workspace Clippy failure**

Evidence class: **Ran**

Frozen base and HEAD: `ec396c458a5015c504011a75814ff13e274544a1`

Run date: 2026-07-16 UTC

## Verdict

The independent Phase 5 sequence stopped at its first required failure.
Formatting passed. Workspace/all-target Clippy failed with eight test-only
diagnostics in `tests/integration/assurance_v2_publication_contract.rs`. Full
Nextest, dependency policy, and fresh adjudicated CRAP were not run. This
evidence cannot close ASSURE-04D.

The runner made no source, test, authority, queue, synthetic-publication,
public, protected, or Git-index edit. The only repository write is this
authorized package artifact. This is an engineering gate disposition, not a
scientific, reproduction, publication, release-owner, or human approval.

## Gate Results

| Order | Exact command | Start (UTC) | Finish (UTC) | Duration | Exit | Result |
| ---: | --- | --- | --- | ---: | ---: | --- |
| 1 | `cargo fmt --check` | `2026-07-16T06:19:09Z` | `2026-07-16T06:19:11Z` | 2.345 s | 0 | PASS |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | `2026-07-16T06:19:38Z` | `2026-07-16T06:19:56Z` | 18.353 s | 101 | FAIL |
| 3 | `cargo nextest run --workspace --profile full` | -- | -- | -- | -- | NOT RUN: stop-on-first-failure |
| 4 | `cargo deny check` | -- | -- | -- | -- | NOT RUN: stop-on-first-failure |
| 5 | `bash tools/release/run_adjudicated_crap_gate.sh --base-ref ec396c458a5015c504011a75814ff13e274544a1 --output-dir docs/work-packages/20260716-assure04d-review-lock-publication-snapshot-001/artifacts/validation-evidence/adjudicated-crap` | -- | -- | -- | -- | NOT RUN: stop-on-first-failure |

Because gate 3 did not run, there is no current full-profile run ID, executed
count, skip count, or JUnit evidence. Because gate 5 did not run, no current
raw, adjudicated, actionable, or touched-production CRAP counts exist, and no
per-touched-file CRAP closure claim is made.

## Blocking Diagnostics

All diagnostics are in
`tests/integration/assurance_v2_publication_contract.rs`:

| Clippy lint | Location or function | Count |
| --- | --- | ---: |
| `needless_raw_string_hashes` | raw strings beginning at lines 279, 389, and 450 | 3 |
| `too_many_lines` | `enter_synthetic_review` at line 274; `reconstructed_production_snapshot_passes_and_forged_roots_fail` at line 636; `stale_roots_open_findings_conflicts_and_release_mismatch_fail_before_publication` at line 1,415; `authority_lifecycle_and_bound_byte_negative_matrix_is_fail_closed` at line 1,542 | 4 |
| `format_push_string` | formatted append beginning at line 1,902 | 1 |

Any remediation changes the frozen test snapshot, so the complete five-gate
sequence must restart from a new freeze. The runner's artifact-only authority
does not permit that remediation.

## Prior HOLD Chronology

The package's earlier engineering HOLDs remain preserved without modification:

- `early-design-audit-disposition.md` records the preimplementation design
  HOLDs and their accepted contract amendments.
- `review-agent-a.md`, `review-agent-b.md`, and `review-disposition.md` record
  both independent Phase 4 HOLD reviews, accepted remediations, and final
  review rechecks.
- `gate-results.md` retains the expected absent-API contract-first failure and
  the later focused/quick PASS evidence.

None of those partial or focused results was reused as Phase 5 heavy closure.

## Worktree, Write-Set, And Index Freeze

The freeze began at `2026-07-16T06:18:55Z` and excludes only
`artifacts/heavy-gate-runner.md` and
`artifacts/validation-evidence/adjudicated-crap/**`. Every preexisting package
artifact, including the retained synthetic public/snapshot evidence, is bound
by a separate protected manifest.

| Representation | Count or size | Before and after SHA-256 |
| --- | ---: | --- |
| Non-runner worktree status | 96 rows | `d359ea6454f6bd53635c9aa8d38be41dc209f1d8abfa0da01515822754c37305` |
| Non-runner binary full-index diff | 118,425 bytes | `7f271d00bffc744fd9c4dbabe7333e9a010d10607094f3af51f74c0c7c89a670` |
| Non-runner changed-path list | 96 paths | `8cb82e84297095efbdca3c909ea4b487e7169686ef42056f968e5e88557df2e0` |
| Non-runner content manifest | 22,352 bytes | `456766812cfc099e829678b86cbd25a3810bdf03c717c94064be06c6fe909318` |
| Implementation/source path list | 25 paths | `da32398930ee0bc573d39dcb0e37b2699cf46d40e2e84823b8215b05ce2af470` |
| Implementation/source content manifest | 2,626 bytes | `3968f0a037c106ad6537e13e8ccb5dde8677b75cb7356e326258c6bec5b36581` |
| Protected preexisting artifact path list | 64 files | `b613ffd6e75afc739237f99151a51a29b709b5f99d1befc31806a9a2c7a9208a` |
| Protected preexisting artifact content manifest | 18,741 bytes | `7a61670ed0782e509e6804456d451eeb8e67ec6423d3f1ae1627550b5e114d4a` |
| Sorted `git ls-files --stage` index listing | repository index | `04bad6f6fa4be03e0f5c5126fc14e93166fb242477d310d6ce4553d9d0d7f7f3` |

Every representation was regenerated after the Clippy failure and compared
byte-for-byte with `cmp`; all were equal. HEAD remained exactly the frozen
base.

The independently enumerated declared-write-set audit found zero out-of-set
paths. The specifically requested dependency/materializer rows are:

| Status | Declared path | Frozen SHA-256 |
| --- | --- | --- |
| `M` | `Cargo.lock` | `56ac6994017a53cb6593b86dcd93953d127b562a19e46501cbe23dcdf7f7b5bf` |
| `M` | `crates/openwepp-assurance/Cargo.toml` | `2eafc4b0c59b79085be1cb9d0d03a90816a07cdd6d448ee0cb3b758e95e6e518` |
| `U` | `tools/release/materialize_assurance_v2_release.sh` | `8d10ceabfd34e70ef51537a2ac5d208e33b267b9260bb43dda630065bb66c1b0` |

`git diff --check` passed. `bash -n` passed for
`check_assurance_release_transition.sh`,
`materialize_assurance_v2_release.sh`, and
`run_release_candidate_gates.sh` both before and after the executed gates.

## Protected Surfaces

| Protected surface | Before and after SHA-256 |
| --- | --- |
| `assurance/catalog.yaml` | `cb9cb601739b64f8cb497c0999ca268859946f2ce7e57532de8c08b9ed30801f` |
| `assurance/templates/catalog.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| `assurance/generated/wepppy-usersum.yaml` | `08b21cbe20ed059eb61f01f9965d8603779501bde90a728bc7a6c138a69258eb` |
| `usersum/assurance/README.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| Sorted SHA-256 manifest of every regular file below `usersum/` | `deb9f2c646aa5eb4ad9e427f8a7cec6ad51e3f9f6b47ffe29d14b4aed4bdcb7a` |

## Current Production-Rust Line Counts

| Rust file | Physical lines | Nonblank lines | Bytes | Disposition |
| --- | ---: | ---: | ---: | --- |
| `crates/openwepp-assurance/src/cli.rs` | 508 | 473 | 18,168 | PASS |
| `crates/openwepp-assurance/src/error.rs` | 72 | 66 | 2,037 | PASS |
| `crates/openwepp-assurance/src/lib.rs` | 24 | 21 | 983 | PASS |
| `crates/openwepp-assurance/src/v2.rs` | 2,984 | 2,840 | 94,932 | WARN; below 3,000-line block |
| `crates/openwepp-assurance/src/v2/assembly.rs` | 1,747 | 1,659 | 59,375 | PASS |
| `crates/openwepp-assurance/src/v2/confined.rs` | 1,262 | 1,147 | 44,658 | PASS |
| `crates/openwepp-assurance/src/v2/lifecycle.rs` | 146 | 136 | 4,659 | PASS |
| `crates/openwepp-assurance/src/v2/planner.rs` | 1,182 | 1,124 | 36,124 | PASS |
| `crates/openwepp-assurance/src/v2/publication.rs` | 2,903 | 2,781 | 103,842 | WARN; below 3,000-line block |

The exact line-count snapshot compared equal after both executed gates. No
nonexempt production Rust file reaches 3,000 lines.

## Toolchain

- Cargo: `1.92.0 (344c4567c 2025-10-21)`
- rustc: `1.92.0 (ded5c06cf 2025-12-08)`, LLVM `21.1.3`
- cargo-nextest: `0.9.138 (fc97e97bb 2026-06-21)`
- cargo-deny: `0.19.6`
- cargo-llvm-cov: `0.8.7`
- cargo-crap: `0.2.2`

---

## Second Heavy Run -- Current HOLD

Current status: **HOLD -- adjudicated CRAP closure failure**

Second-run freeze: `2026-07-16T06:40:00Z`

Frozen base and HEAD: `ec396c458a5015c504011a75814ff13e274544a1`

The first heavy-run strict-Clippy HOLD above is retained as historical
chronology. After the test-only Clippy remediation and renewed review evidence,
the independent Phase 5 sequence restarted from gate 1 on a new frozen
post-remediation tree. Gates 1 through 4 passed. Gate 5 failed and the sequence
stopped. No gate result from the first run was reused.

The current blocker is seven unadjudicated workspace CRAP rows, all in touched
production Rust files. Four of the seven touched production Rust files have a
measured maximum CRAP greater than 30. Therefore neither the zero-actionable-row
requirement nor the per-touched-file CRAP `<= 30` requirement is satisfied.

This is an engineering gate HOLD only. It is not scientific approval, human
approval, reproduction acceptance, publication approval, release-owner
approval, or authority to mutate public or protected bytes.

### Second-Run Gate Results

| Order | Exact command | Start (UTC) | Finish (UTC) | Duration | Exit | Result |
| ---: | --- | --- | --- | ---: | ---: | --- |
| 1 | `cargo fmt --check` | `2026-07-16T06:40:23Z` | `2026-07-16T06:40:26Z` | 2.437 s | 0 | PASS |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | `2026-07-16T06:41:02Z` | `2026-07-16T06:41:03Z` | 1.167 s | 0 | PASS |
| 3 | `cargo nextest run --workspace --profile full` | `2026-07-16T06:41:51Z` | `2026-07-16T06:51:30Z` | 579.315 s | 0 | PASS |
| 4 | `cargo deny check` | `2026-07-16T06:54:00Z` | `2026-07-16T06:54:01Z` | 0.975 s | 0 | PASS |
| 5 | `bash tools/release/run_adjudicated_crap_gate.sh --base-ref ec396c458a5015c504011a75814ff13e274544a1 --output-dir docs/work-packages/20260716-assure04d-review-lock-publication-snapshot-001/artifacts/validation-evidence/adjudicated-crap` | `2026-07-16T06:54:29Z` | `2026-07-16T07:30:17Z` | 2,148.762 s | 1 | FAIL |

The gate-5 terminal summary was exactly:

```text
adjudicated-crap: status=FAIL raw=9 adjudicated=2 actionable=7 touched_files=7
```

### Full-Profile Test Evidence

- Nextest run ID: `6b01827d-b022-4fb8-8a91-50745687a779`.
- Summary: 2,043 tests passed, 0 failed, 3 skipped, and 4 slow.
- JUnit: `target/nextest/full/junit.xml`.
- JUnit size: 483,339 bytes.
- JUnit SHA-256:
  `7c648b3d81f144483d9f6b8aeb31ad07ef7d5bf46ae80ce8cd760563bfb8cdc4`.
- Dependency policy: advisories, bans, licenses, and sources all passed.

### Adjudicated CRAP Failure

The fresh report is
`artifacts/validation-evidence/adjudicated-crap/adjudicated-crap-report.md`.
Its acquisition was current-source closure eligible, but its debt assessment
and status are both `FAIL`.

| Report field | Exact result |
| --- | ---: |
| Production entries assessed | 9,216 |
| Raw rows over 30 | 9 |
| Currently adjudicated rows | 2 |
| Actionable rows | 7 |
| Touched production files | 7 |
| Actionable rows in touched files | 7 |
| Actionable rows outside touched files | 0 |

The two adjudicated rows are the existing `CQR-LOW-L08` and `CQR-LOW-L11`
entries outside the touched files. No invalid or stale adjudication was
reported. The seven current actionable rows are:

| File | Function | Line | CC | Coverage | CRAP |
| --- | --- | ---: | ---: | ---: | ---: |
| `crates/openwepp-assurance/src/cli.rs` | `execute_publish` | 177 | 17 | 20% | 164.968 |
| `crates/openwepp-assurance/src/cli.rs` | `parse_options` | 222 | 31 | 95.5224% | 31.0863 |
| `crates/openwepp-assurance/src/v2.rs` | `validate_report_structure` | 1,525 | 33 | 92.5% | 33.4594 |
| `crates/openwepp-assurance/src/v2.rs` | `validate_review` | 2,308 | 26 | 71.0526% | 42.3974 |
| `crates/openwepp-assurance/src/v2/confined.rs` | `open_ambient_platform` | 237 | 14 | 50% | 38.5 |
| `crates/openwepp-assurance/src/v2/publication.rs` | `install_receipt` | 1,906 | 19 | 61.7021% | 39.2783 |
| `crates/openwepp-assurance/src/v2/publication.rs` | `verify_snapshot_content` | 2,234 | 27 | 69.2308% | 48.2362 |

The maximum measured CRAP by touched production file is:

| Touched production Rust file | Measured entries | Maximum CRAP | Disposition |
| --- | ---: | ---: | --- |
| `crates/openwepp-assurance/src/cli.rs` | 56 | 164.968 | FAIL |
| `crates/openwepp-assurance/src/lib.rs` | 0 | N/A | No measured function row; no actionable row |
| `crates/openwepp-assurance/src/v2.rs` | 202 | 42.3973611314 | FAIL |
| `crates/openwepp-assurance/src/v2/assembly.rs` | 134 | 25 | PASS |
| `crates/openwepp-assurance/src/v2/confined.rs` | 168 | 38.5 | FAIL |
| `crates/openwepp-assurance/src/v2/lifecycle.rs` | 8 | 23.9578316314 | PASS |
| `crates/openwepp-assurance/src/v2/publication.rs` | 164 | 48.2362312244 | FAIL |

The canonical CRAP directory contains 16 checksum-bound files. Running
`sha256sum -c` from the repository root passed for every file. The production
source manifests before coverage, after coverage, and at finalization compared
byte-for-byte equal: 228 sources under SHA-256
`a25f5fb5048612447ea50262cecd0520ce1bc548451a9f1358b50b84bc74035d`.

| Evidence identity | SHA-256 |
| --- | --- |
| Workspace CRAP JSON | `c511a02f1c46d034d0e415cc96130c24ec1c71130ff4606905b2b081bb0328a8` |
| Workspace LCOV | `03b305372ca9fd9e7e065083f25d4c708b4c3ffcfc3c85a5d4a44edba3dacf74` |
| Adjudication registry | `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f` |
| JSON report file | `ba4b19fbd8a6be0db85d963bacf7800a8ce91a0a4b4ca7b32f890dc5925f5215` |
| Markdown report file | `b9a0447cbdcecd145d041d1740c9d0f35b36ed64bb7c999f36660cf6768a848b` |
| `sha256sums.txt` | `4bf3f73914cd08768612a63c5b67dfdd03ca4fae50672e5db504993623317dc0` |

### Second-Run Freeze And Governance Rechecks

The second-run freeze excludes only this heavy-runner artifact and the
canonical adjudicated-CRAP directory. The prior heavy-runner artifact was
frozen before the restart under SHA-256
`c0cc4e6708c7b84f6d37dea4e94d44f716bcbf9cd50af3fc65782cb373884617`.

| Representation | Count or size | Before and final SHA-256 |
| --- | ---: | --- |
| Non-runner worktree status | 96 rows | `d359ea6454f6bd53635c9aa8d38be41dc209f1d8abfa0da01515822754c37305` |
| Non-runner binary full-index diff | 118,425 bytes | `7f271d00bffc744fd9c4dbabe7333e9a010d10607094f3af51f74c0c7c89a670` |
| Frozen path list | 96 paths | `8cb82e84297095efbdca3c909ea4b487e7169686ef42056f968e5e88557df2e0` |
| Frozen content manifest | 22,352 bytes | `252bded3217cdb3dbd18d480e80df2d9d19616e0705515b56d926a231632c67e` |
| Implementation/source path list | 25 paths | `da32398930ee0bc573d39dcb0e37b2699cf46d40e2e84823b8215b05ce2af470` |
| Implementation/source content manifest | 2,626 bytes | `5172b0e822d3463f174ba615677ba8b65674d045804639d88d418e6182f02486` |
| Protected preexisting artifact path list | 64 files | `b613ffd6e75afc739237f99151a51a29b709b5f99d1befc31806a9a2c7a9208a` |
| Protected preexisting artifact content manifest | 18,741 bytes | `bc39b797463d912889d748f70cb906ab71518a4221b5283552d1e4f025d432d5` |
| Sorted `git ls-files --stage` index listing | repository index | `04bad6f6fa4be03e0f5c5126fc14e93166fb242477d310d6ce4553d9d0d7f7f3` |

Every final representation compared byte-for-byte equal with its second-run
freeze. HEAD remained exactly the frozen base. The independently enumerated
write-set audit found zero out-of-set paths. The only second-run repository
writes are this authorized heavy-runner section and the canonical gate-5
evidence directory. No code, test, authority, package-state, other evidence,
public/protected byte, or Git-index edit was made by the runner.

The named protected-surface manifest remained
`38d607f9d58e87fbc513c37d8a1671a11dca0ac166161a7afe1be4d9b3cf5e26`.
Its surface hashes remain the values recorded in the first-run section. The
sorted manifest of every regular file under `usersum/` remained
`deb9f2c646aa5eb4ad9e427f8a7cec6ad51e3f9f6b47ffe29d14b4aed4bdcb7a`.

`git diff --check` passed. `bash -n` passed for
`check_assurance_release_transition.sh`,
`materialize_assurance_v2_release.sh`, and
`run_release_candidate_gates.sh`. The governed production-Rust line-count
snapshot remained byte-identical under SHA-256
`7fa7e22894d9407b47a171daea49d729ee8adbf5a2c675edeb442b86c23a2638`;
the exact counts and under-3,000 dispositions remain those recorded in the
first-run table.

### Second-Run Disposition

ASSURE-04D remains **HOLD**. Closure requires a new authorized remediation of
the seven actionable CRAP rows followed by a fresh freeze and a complete
restart of all five gates in exact order. The passing format, strict-Clippy,
full-Nextest, and dependency-policy results above are evidence from this
failed sequence, not reusable substitutes for that future complete run.

---

## Third Heavy Run -- Current PASS

Current status: **PASS -- complete engineering heavy closure sequence**

Third-run freeze: `2026-07-16T08:14:40Z`

Frozen base and HEAD: `ec396c458a5015c504011a75814ff13e274544a1`

The first strict-Clippy HOLD and second adjudicated-CRAP HOLD above are retained
exactly as historical chronology. Before this run, the second run's failing
canonical CRAP bundle was atomically renamed to
`artifacts/validation-evidence/adjudicated-crap-hold-02/`. The complete
five-gate sequence then restarted from gate 1 on the frozen post-remediation
tree. All five required gates passed in exact order.

This is an engineering gate PASS only. It is not scientific approval, human
approval, reproduction acceptance, publication approval, release-owner
approval, or authority to mutate public or protected bytes.

### Historical HOLD-02 Bundle

The archived second-run bundle contains 17 files: 16 files named by its
unchanged `sha256sums.txt` plus the checksum manifest itself. Every named file
passed checksum verification before and after the atomic rename. A sorted
relative-path SHA-256 manifest of all 17 files compared byte-for-byte equal
before and after the rename.

| HOLD-02 identity | Exact value |
| --- | --- |
| Archived result | `FAIL`: raw 9, adjudicated 2, actionable 7, touched files 7 |
| Relative-path bundle manifest SHA-256 | `25f20dc51729915004206cecd8be28bb434b63870c5fa18f86053e614868cc9e` |
| Archived `sha256sums.txt` SHA-256 | `4bf3f73914cd08768612a63c5b67dfdd03ca4fae50672e5db504993623317dc0` |
| Archived source-manifest SHA-256 | `a25f5fb5048612447ea50262cecd0520ce1bc548451a9f1358b50b84bc74035d` |

The archived bundle was included in the third-run protected artifact path and
content manifests. It remained byte-identical throughout all five gates.

### Third-Run Gate Results

| Order | Exact command | Start (UTC) | Finish (UTC) | Duration | Exit | Result |
| ---: | --- | --- | --- | ---: | ---: | --- |
| 1 | `cargo fmt --check` | `2026-07-16T08:15:43Z` | `2026-07-16T08:15:46Z` | 2.357 s | 0 | PASS |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | `2026-07-16T08:16:18Z` | `2026-07-16T08:16:20Z` | 1.189 s | 0 | PASS |
| 3 | `cargo nextest run --workspace --profile full` | `2026-07-16T08:16:56Z` | `2026-07-16T08:26:30Z` | 573.377 s | 0 | PASS |
| 4 | `cargo deny check` | `2026-07-16T08:27:08Z` | `2026-07-16T08:27:09Z` | 1.396 s | 0 | PASS |
| 5 | `bash tools/release/run_adjudicated_crap_gate.sh --base-ref ec396c458a5015c504011a75814ff13e274544a1 --output-dir docs/work-packages/20260716-assure04d-review-lock-publication-snapshot-001/artifacts/validation-evidence/adjudicated-crap` | `2026-07-16T08:27:39Z` | `2026-07-16T09:04:05Z` | 2,186.091 s | 0 | PASS |

The gate-5 terminal summary was exactly:

```text
adjudicated-crap: status=PASS raw=2 adjudicated=2 actionable=0 touched_files=7
```

### Full-Profile Test And Dependency Evidence

- Nextest run ID: `9438c097-eccb-4959-88df-fb860cc64fdb`.
- Summary: 2,046 tests passed, 0 failed, 3 skipped, and 4 slow.
- JUnit: `target/nextest/full/junit.xml`.
- JUnit size: 483,956 bytes.
- JUnit SHA-256:
  `fe2c4a83d620a81794685e7593a37b0abd1369e6d763c406d40bcbb7c30b847e`.
- Dependency policy: advisories, bans, licenses, and sources all passed.

### Fresh Adjudicated CRAP Closure

The fresh report is
`artifacts/validation-evidence/adjudicated-crap/adjudicated-crap-report.md`.
Its acquisition is current-source closure eligible. Its debt assessment and
status are both `PASS`.

| Report field | Exact result |
| --- | ---: |
| Production entries assessed | 9,262 |
| Raw rows over 30 | 2 |
| Currently adjudicated rows | 2 |
| Actionable workspace rows | 0 |
| Touched production files | 7 |
| Actionable rows in touched files | 0 |
| Actionable rows outside touched files | 0 |

The only raw rows above 30 are the existing exact `CQR-LOW-L08` and
`CQR-LOW-L11` adjudications outside the touched files. No invalid or stale
adjudication was reported.

The maximum measured CRAP by touched production file is:

| Touched production Rust file | Measured entries | Maximum CRAP | Maximum function | Disposition |
| --- | ---: | ---: | --- | --- |
| `crates/openwepp-assurance/src/cli.rs` | 76 | 30 | `publish_selected` | PASS |
| `crates/openwepp-assurance/src/lib.rs` | 0 | N/A | No measured function row | PASS; no actionable row |
| `crates/openwepp-assurance/src/v2.rs` | 194 | 23 | `validate_report_sections` | PASS |
| `crates/openwepp-assurance/src/v2/assembly.rs` | 134 | 25 | `render_directive` | PASS |
| `crates/openwepp-assurance/src/v2/confined.rs` | 172 | 30 | `remove_regular_if_exists_platform` | PASS |
| `crates/openwepp-assurance/src/v2/lifecycle.rs` | 24 | 23.9578316314 | `validate_date` | PASS |
| `crates/openwepp-assurance/src/v2/publication.rs` | 178 | 26.5736519438 | `read_prior_public` | PASS |

The gate defines raw debt as CRAP strictly greater than 30. Every measured
touched-file maximum is therefore at or below the required threshold, and the
workspace actionable set is empty.

The canonical directory contains 17 files. All 16 entries in
`sha256sums.txt` passed. The production source manifests before coverage,
after coverage, and at finalization compared byte-for-byte equal: 228 sources
under SHA-256
`16e5bcb05297d5ca73ff1617242d019ee54063bf29a4dfa12b3f4c34fe30cf02`.

| Current evidence identity | SHA-256 |
| --- | --- |
| Workspace CRAP JSON | `2b5b9bf05db3413c804ba6826a94aac926b52b397b831495653da1516d8fe5e8` |
| Workspace LCOV | `1cdaeb00115e6a4773a57abe4fc34b54f8cf5dd2731761aa8917a86862aa0656` |
| Adjudication registry | `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f` |
| JSON report file | `6d5547ed29f9f0ce35251faa8cc7010178597c3424d06adf394b90c84da23d04` |
| Markdown report file | `e5128a259ee24e0fecc982c20c9da6600b0e8451e134ce3ac904b50e36014ecd` |
| `sha256sums.txt` | `64d859f2ce0eddb635ab1fa31f0c2214731fb7d9ba4c631dede70d9d90bd164d` |

### Third-Run Freeze And Governance Rechecks

The third-run freeze excludes only this heavy-runner artifact and the fresh
canonical adjudicated-CRAP directory. The complete preexisting heavy report,
including both prior HOLD chronologies, was frozen under SHA-256
`29fe6f9947908c511b7d3824df9c54c9e1fd225ad9c3e9cf71a4dad67c626bdd`.

| Representation | Count or size | Before and final SHA-256 |
| --- | ---: | --- |
| Non-runner worktree status | 114 rows | `03a0761268a840a998c5a34219ee14701f252cb09113c4e1602235b742132f7f` |
| Non-runner binary full-index diff | 122,348 bytes | `59aba6c5f361b244c04caab2361f5d00c2f853d70ebb00e65bc78de9bb2246c9` |
| Frozen path list | 114 paths | `84756ee25ab4a09c8d2da3c7c09b6bd95624855dad96ac438bdc585f861a6f70` |
| Frozen content manifest | 26,228 bytes | `398f4e0bbff8ec8c7f593b867341bade563f12353c743fe4b1bef5c81a61e112` |
| Implementation/source path list | 28 paths | `2728b80fb2104dade304c50cbf2a4ae06ffbbec40e5a689967be0ca66c6ce83e` |
| Implementation/source content manifest | 2,933 bytes | `b052c1aec3d87d85595d9730a28fdb41370f39e76633646fb864b54c6329a492` |
| Protected preexisting artifact path list | 82 files | `4be3b8dd629884e189e136a0d6a78b47f7da6bf28f945e6f3baf366fcc9ca933` |
| Protected preexisting artifact content manifest | 22,617 bytes | `b9bcbdfa2316509d1343c3fb650c8b04b4594b5149fdb2866200949e64a7f26a` |
| Sorted `git ls-files --stage` index listing | repository index | `04bad6f6fa4be03e0f5c5126fc14e93166fb242477d310d6ce4553d9d0d7f7f3` |

Every final representation compared byte-for-byte equal with its third-run
freeze. HEAD remained exactly the frozen base. The independently enumerated
write-set audit found zero out-of-set paths. The runner made no code, test,
authority, package-state, other-evidence, public/protected-byte, or Git-index
edit. Runner writes are confined to this appended section, the atomic
historical CRAP archive rename, and the fresh canonical CRAP evidence bundle.

The named protected-surface manifest remained
`38d607f9d58e87fbc513c37d8a1671a11dca0ac166161a7afe1be4d9b3cf5e26`.
Its exact surface hashes remain the values recorded in the first-run section.
The sorted manifest of every regular file below `usersum/` remained
`deb9f2c646aa5eb4ad9e427f8a7cec6ad51e3f9f6b47ffe29d14b4aed4bdcb7a`.

The governed production-Rust line-count snapshot remained byte-identical under
SHA-256
`909d0baf683946eef6151b44f217b043f6c1befa0b34490df9ffc3959409b1c4`:

| Rust file | Physical lines | Nonblank lines | Bytes | Disposition |
| --- | ---: | ---: | ---: | --- |
| `crates/openwepp-assurance/src/cli.rs` | 661 | 613 | 21,710 | PASS |
| `crates/openwepp-assurance/src/error.rs` | 72 | 66 | 2,037 | PASS |
| `crates/openwepp-assurance/src/lib.rs` | 24 | 21 | 983 | PASS |
| `crates/openwepp-assurance/src/v2.rs` | 2,821 | 2,681 | 88,407 | WARN; below 3,000-line block |
| `crates/openwepp-assurance/src/v2/assembly.rs` | 1,747 | 1,659 | 59,375 | PASS |
| `crates/openwepp-assurance/src/v2/confined.rs` | 1,293 | 1,175 | 45,540 | PASS |
| `crates/openwepp-assurance/src/v2/lifecycle.rs` | 349 | 328 | 12,723 | PASS |
| `crates/openwepp-assurance/src/v2/planner.rs` | 1,182 | 1,124 | 36,124 | PASS |
| `crates/openwepp-assurance/src/v2/publication.rs` | 2,982 | 2,852 | 105,844 | WARN; below 3,000-line block |

`git diff --check` passed. `bash -n` passed for
`check_assurance_release_transition.sh`,
`materialize_assurance_v2_release.sh`, and
`run_release_candidate_gates.sh` before and after the gates.

### Third-Run Disposition

ASSURE-04D Phase 5 independent engineering heavy closure is **PASS**. All five
required gates passed in exact order on one frozen post-remediation tree; the
workspace actionable CRAP set is empty; every touched production Rust file is
at adjudicated CRAP 30 or below; and all freeze, write-set, protected-surface,
usersum, line-count, script-syntax, diff, and index checks pass.
