# ASSURE-04C Independent Heavy Gate Runner

Status: **PASS -- complete independent heavy sequence closed**

Evidence class: **Ran**

Frozen base and HEAD: `e704f0202278ebb86c6a8c667caf73d599be04ab`

Run date: 2026-07-16 UTC

## Current PASS Verdict

The complete independent Phase 5 restart passed formatting, strict
workspace/all-target Clippy, full Nextest, dependency policy, and a fresh
adjudicated CRAP gate. Nextest passed all 2,011 executed tests with three
skipped. Fresh CRAP produced two exact existing adjudications outside the
touched files and zero actionable rows. Every measurable touched-production
maximum is at or below 30, including the amended `error.rs`; `lib.rs` has no
measurable function row. This heavy sequence passes.

The prior strict-Clippy HOLD remains below as non-closable chronology. None of
its partial results was reused in this restart.

## Current PASS Gate Results

| Order | Exact command | Start (UTC) | Finish (UTC) | Duration | Exit | Result |
| ---: | --- | --- | --- | ---: | ---: | --- |
| 1 | `cargo fmt --check` | `2026-07-16T01:49:28Z` | `2026-07-16T01:49:30Z` | 2.477 s | 0 | PASS |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | `2026-07-16T01:49:51Z` | `2026-07-16T01:49:53Z` | 1.243 s | 0 | PASS |
| 3 | `cargo nextest run --workspace --profile full` | `2026-07-16T01:50:10Z` | `2026-07-16T02:00:39Z` | 628.658 s | 0 | PASS |
| 4 | `cargo deny check` | `2026-07-16T02:01:15Z` | `2026-07-16T02:01:18Z` | 2.978 s | 0 | PASS |
| 5 | `bash tools/release/run_adjudicated_crap_gate.sh --base-ref e704f0202278ebb86c6a8c667caf73d599be04ab --output-dir docs/work-packages/20260715-assure04c-deterministic-manuscript-assembly-001/artifacts/validation-evidence/adjudicated-crap` | `2026-07-16T02:01:50Z` | `2026-07-16T02:38:08Z` | 2,177.445 s | 0 | PASS |

`cargo deny check` reported advisories, bans, licenses, and sources as OK.

## Current PASS Full Nextest Evidence

| Field | Value |
| --- | --- |
| Run ID | `2344d4b1-ec78-40e0-8d5c-5474cdb438ee` |
| Profile | `full` |
| Binaries | 186 |
| Executed | 2,011 |
| Passed | 2,011 |
| Failed | 0 |
| Skipped | 3 |
| Slow | 5 |
| Nextest-reported test time | 627.295 s |
| JUnit path | `target/nextest/full/junit.xml` |
| JUnit size | 476,259 bytes |
| JUnit SHA-256 | `5b1b417542ebe3363e512626b5a139d7ca9815789a12e0016338ad4c1369768f` |

The JUnit root identifies the same run ID and records 2,011 tests, zero
failures, and zero errors.

## Current PASS Adjudicated CRAP Evidence

Canonical report:
[`validation-evidence/adjudicated-crap/adjudicated-crap-report.md`](validation-evidence/adjudicated-crap/adjudicated-crap-report.md)

| Field | Value |
| --- | --- |
| Acquisition | Fresh; eligible for current-source closure |
| Threshold | CRAP strictly greater than 30 is raw debt |
| Production entries | 8,948 |
| Raw rows above threshold | 2 |
| Adjudicated rows | 2 |
| Actionable rows | 0 |
| Touched production files | 7 |
| Actionable rows in touched files | 0 |
| Actionable rows outside touched files | 0 |
| Debt status | PASS |
| Production-source count | 226 |
| Production-source manifest before/after/final | `ed4213f8be4d1921740658865f4f3ec12cc1804b4c8d7e64ff16d9d7ae9c5d5e` |
| Workspace CRAP JSON SHA-256 | `f24ed50bf5754912e5f1a16ce635d9cccf67f161cf05c1bae65cbb1e828e35d3` |
| LCOV SHA-256 | `0f39a4cc95a527c67fd3ad4d7a3d6721925a1d282e8fb661361a3313c177585f` |
| Adjudication registry SHA-256 | `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f` |
| Machine report SHA-256 | `26a378b8adba38cb36392341e5c5ced2b4fbd129961e46882913bbd932ea7f16` |
| Human report SHA-256 | `6f9d99ad2797812ed385ee47eaccd8d6bc9e73a0a9efb3b42d5f3f7183e9a80b` |
| Run-status SHA-256 | `e50b0618d7d24d0154cc76d2e8790eb517798faae941122ddaa8d069f0d30a9f` |
| Evidence checksum manifest SHA-256 | `0562bb794ee38272289500fc8d7e83002f800eed9284397762d3c46e10964c4f` |

The canonical checksum manifest was verified with `sha256sum -c`; all 16
artifacts passed. The source manifests before acquisition, after acquisition,
and at final disposition are byte-identical.

### Current PASS Touched-Production Closure Rows

The maxima below come from the fresh canonical `workspace-crap.json`. No
touched file contains an adjudicated exception or a raw row above 30.

| Status | Touched production file | Maximum function | Line | CC | Coverage | Maximum CRAP | Disposition |
| --- | --- | --- | ---: | ---: | ---: | ---: | --- |
| `M` | `crates/openwepp-assurance/src/cli.rs` | `parse_options` | 166 | 19 | 100.0000% | 19.0000 | PASS |
| `M` | `crates/openwepp-assurance/src/error.rs` | `AssuranceError::source` | 65 | 4 | 0.0000% | 20.0000 | PASS |
| `M` | `crates/openwepp-assurance/src/lib.rs` | No function entry | N/A | N/A | N/A | N/A | PASS: no raw row above 30 |
| `M` | `crates/openwepp-assurance/src/v2.rs` | `validate_report_structure` | 1,274 | 30 | 100.0000% | 30.0000 | PASS: at threshold |
| `U` | `crates/openwepp-assurance/src/v2/assembly.rs` | `render_directive` | 746 | 25 | 100.0000% | 25.0000 | PASS |
| `M` | `crates/openwepp-assurance/src/v2/confined.rs` | `remove_directory_contents` | 393 | 12 | 71.4286% | 15.3586 | PASS |
| `M` | `crates/openwepp-assurance/src/v2/planner.rs` | `add_report_nodes` | 283 | 15 | 89.2857% | 15.2767 | PASS |

The two raw rows are the existing exact adjudications `CQR-LOW-L08` and
`CQR-LOW-L11`; neither is in a touched file. No adjudication is invalid or
stale. Coverage collection is metric evidence; the independent full Nextest
run above is the terminal workflow evidence on the same stable snapshot.

## Current PASS Worktree, Implementation, And Index Freeze

The restart freeze began at `2026-07-16T01:49:03Z` and excludes only the
authorized heavy report and canonical CRAP directory. Relative to the prior
HOLD freeze, only
`tests/integration/assurance_v2_assembly_contract.rs` changed in the
implementation subset. Parent chronology updates changed `package.md` and
`gate-results.md`; retained staging and the Git index remained identical.

| Representation | Count or size | Before and after SHA-256 |
| --- | ---: | --- |
| Non-runner worktree status | 69 rows | `00799a409ec38579441d87c90040743f7db1ee7af34cf5c30686bcb71f5018b1` |
| Non-runner binary full-index diff | 145,156 bytes | `f66bc90f5b061111e2b1dd80fcf8d6e767d317f3d63bb04890c05e6ce9dfad11` |
| Non-runner changed-path list | 69 paths | `ae0041009d742ca6dd643c5a354407d8373ff27ffadd8be6f1091a5e478a6348` |
| Non-runner content manifest | 11,967 bytes | `815ab41d12f6566b0e24b3335ecc46722c0a00ff2c9b43a3ab89eb2df9af47a7` |
| Implementation/source path list | 23 paths | `615b55265ad72b3f5e2ceffb87be7cd3f734883418327420a64918438ed6c1b0` |
| Implementation/source content manifest | 2,712 bytes | `42fbf6b5646e74c2b0fd9d803fde30dd98f96488a4f2f0b99be51ac09924d179` |
| Protected preexisting artifact path list | 41 files | `7acaf16fe4f1b9de37908b6dbe79633b6267f80cd500376c9eec79bb4b0ebebf` |
| Protected preexisting artifact content manifest | 8,595 bytes | `de82daa5ec2aa9493e9ce54596e1a17f5f7c45aa33b49c9a1d556f5a935ed3be` |
| Sorted `git ls-files --stage` index listing | repository index | `5cfe0880d88c29f99d0695848a06b0df4aaa83b0d61d6ada2253fefcc8a78d86` |

All representations were regenerated after gate 5 and compared byte-for-byte
with `cmp`; all were equal. Protected public surfaces, the aggregate `usersum`
identity, retained staging, and the recorded line counts also remained equal.
No out-of-bounds write occurred.

## Current PASS Protected Surfaces And Line Counts

| Protected surface | Before and after SHA-256 |
| --- | --- |
| `assurance/catalog.yaml` | `cb9cb601739b64f8cb497c0999ca268859946f2ce7e57532de8c08b9ed30801f` |
| `assurance/templates/catalog.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| `assurance/generated/wepppy-usersum.yaml` | `08b21cbe20ed059eb61f01f9965d8603779501bde90a728bc7a6c138a69258eb` |
| `usersum/assurance/README.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| Sorted SHA-256 manifest of every regular file below `usersum/` | `deb9f2c646aa5eb4ad9e427f8a7cec6ad51e3f9f6b47ffe29d14b4aed4bdcb7a` |

| Rust file | Physical lines | Nonblank lines | Bytes | Disposition |
| --- | ---: | ---: | ---: | --- |
| `crates/openwepp-assurance/src/lib.rs` | 21 | 18 | 710 | PASS |
| `crates/openwepp-assurance/src/error.rs` | 72 | 66 | 2,037 | PASS; amended typed recovery errors included |
| `crates/openwepp-assurance/src/cli.rs` | 352 | 325 | 11,456 | PASS |
| `crates/openwepp-assurance/src/v2.rs` | 2,436 | 2,306 | 75,663 | WARN; below 3,000-line block |
| `crates/openwepp-assurance/src/v2/assembly.rs` | 1,732 | 1,646 | 58,878 | PASS |
| `crates/openwepp-assurance/src/v2/confined.rs` | 889 | 807 | 31,435 | PASS |
| `crates/openwepp-assurance/src/v2/planner.rs` | 1,182 | 1,124 | 36,124 | PASS |

Both exact snapshots compared equal after gate 5. `v2.rs` is the only line-count
warning, and no touched Rust file reaches the 3,000-line closure block.

## Prior HOLD Verdict

The independent Phase 5 sequence stopped at the first required failure.
Formatting passed. Workspace/all-target Clippy failed on
`clippy::similar_names` in
`tests/integration/assurance_v2_assembly_contract.rs:155`. Full Nextest,
dependency policy, and fresh adjudicated CRAP were not run. This evidence cannot
close ASSURE-04C.

The runner made no source, test, authority, source-manifest, roadmap, queue,
retained-staging, public, protected, or Git-index edit. The only repository
write is this authorized evidence artifact.

## Prior HOLD Gate Results

| Order | Exact command | Start (UTC) | Finish (UTC) | Duration | Exit | Result |
| ---: | --- | --- | --- | ---: | ---: | --- |
| 1 | `cargo fmt --check` | `2026-07-16T01:43:49Z` | `2026-07-16T01:43:51Z` | 2.495 s | 0 | PASS |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | `2026-07-16T01:44:13Z` | `2026-07-16T01:44:31Z` | 17.789 s | 101 | FAIL |
| 3 | `cargo nextest run --workspace --profile full` | -- | -- | -- | -- | NOT RUN: stop-on-first-failure |
| 4 | `cargo deny check` | -- | -- | -- | -- | NOT RUN: stop-on-first-failure |
| 5 | `bash tools/release/run_adjudicated_crap_gate.sh --base-ref e704f0202278ebb86c6a8c667caf73d599be04ab --output-dir docs/work-packages/20260715-assure04c-deterministic-manuscript-assembly-001/artifacts/validation-evidence/adjudicated-crap` | -- | -- | -- | -- | NOT RUN: stop-on-first-failure |

Because gate 3 did not run, there is no current full-profile run ID, test count,
skip count, or JUnit evidence. Because gate 5 did not run, no current raw,
adjudicated, actionable, or touched-production CRAP count exists, and no
per-touched-file threshold claim is made.

## Prior HOLD Blocking Diagnostic

Clippy reported that the `stage` binding at
`tests/integration/assurance_v2_assembly_contract.rs:155` is too similar to the
`stale` binding at line 154:

```text
error: binding's name is too similar to existing binding
   --> tests/integration/assurance_v2_assembly_contract.rs:155:9
    |
154 |     let stale = fixture("assure04c-stale");
155 |     let stage = prepared_stage("assure04c-stale-stage");
    |
    = note: `-D clippy::similar-names` implied by `-D warnings`
```

The remediation must rename one binding without changing the test's semantics.
That source change invalidates this freeze, so the complete five-gate sequence
must restart from a new current snapshot.

## Prior HOLD Worktree, Implementation, And Index Freeze

The freeze excludes only the authorized paths
`artifacts/heavy-gate-runner.md` and
`artifacts/validation-evidence/adjudicated-crap/**`. All preexisting package
artifacts, including retained staging, remain inside an explicit protected
manifest.

| Representation | Count or size | Before and after SHA-256 |
| --- | ---: | --- |
| Non-runner worktree status | 69 rows | `00799a409ec38579441d87c90040743f7db1ee7af34cf5c30686bcb71f5018b1` |
| Non-runner binary full-index diff | 145,156 bytes | `f66bc90f5b061111e2b1dd80fcf8d6e767d317f3d63bb04890c05e6ce9dfad11` |
| Non-runner changed-path list | 69 paths | `ae0041009d742ca6dd643c5a354407d8373ff27ffadd8be6f1091a5e478a6348` |
| Non-runner content manifest | 11,967 bytes | `966a8be8572efc615078d2e44dbe046d1ac15fa654c42d33a0782d8c739aea95` |
| Implementation/source path list | 23 paths | `615b55265ad72b3f5e2ceffb87be7cd3f734883418327420a64918438ed6c1b0` |
| Implementation/source content manifest | 2,712 bytes | `fcebd37245771245b1e6347848e54cd3f71e08292e8190679021efea5428aa1f` |
| Protected preexisting artifact path list | 41 files | `7acaf16fe4f1b9de37908b6dbe79633b6267f80cd500376c9eec79bb4b0ebebf` |
| Protected preexisting artifact content manifest | 8,595 bytes | `d118ca778b83f7654f8e51c09ea21a2de4d0eec88cd3c86f767ee68818aa035c` |
| Sorted `git ls-files --stage` index listing | repository index | `5cfe0880d88c29f99d0695848a06b0df4aaa83b0d61d6ada2253fefcc8a78d86` |

Every representation was regenerated after the Clippy failure and compared
byte-for-byte with `cmp`; all were equal. HEAD also remained exactly the frozen
base.

## Prior HOLD Protected Surfaces

| Surface | Before and after SHA-256 |
| --- | --- |
| `assurance/catalog.yaml` | `cb9cb601739b64f8cb497c0999ca268859946f2ce7e57532de8c08b9ed30801f` |
| `assurance/templates/catalog.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| `assurance/generated/wepppy-usersum.yaml` | `08b21cbe20ed059eb61f01f9965d8603779501bde90a728bc7a6c138a69258eb` |
| `usersum/assurance/README.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| Sorted SHA-256 manifest of every regular file below `usersum/` | `deb9f2c646aa5eb4d9e427f8a7cec6ad51e3f9f6b47ffe29d14b4aed4bdcb7a` |

## Prior HOLD Touched-Production Line Counts

| Rust file | Physical lines | Nonblank lines | Bytes | Disposition |
| --- | ---: | ---: | ---: | --- |
| `crates/openwepp-assurance/src/lib.rs` | 21 | 18 | 710 | PASS |
| `crates/openwepp-assurance/src/error.rs` | 72 | 66 | 2,037 | PASS; amended typed recovery errors included |
| `crates/openwepp-assurance/src/cli.rs` | 352 | 325 | 11,456 | PASS |
| `crates/openwepp-assurance/src/v2.rs` | 2,436 | 2,306 | 75,663 | WARN; below 3,000-line block |
| `crates/openwepp-assurance/src/v2/assembly.rs` | 1,732 | 1,646 | 58,878 | PASS |
| `crates/openwepp-assurance/src/v2/confined.rs` | 889 | 807 | 31,435 | PASS |
| `crates/openwepp-assurance/src/v2/planner.rs` | 1,182 | 1,124 | 36,124 | PASS |

The exact line-count snapshot compared equal after both executed gates. No
nonexempt touched Rust file reaches 3,000 lines.

## Prior HOLD Toolchain

- Cargo: `1.92.0 (344c4567c 2025-10-21)`
- rustc: `1.92.0 (ded5c06cf 2025-12-08)`, LLVM `21.1.3`
- cargo-nextest: `0.9.138 (fc97e97bb 2026-06-21)`
- cargo-deny: `0.19.6`
- cargo-llvm-cov: `0.8.7`
- cargo-crap: `0.2.2`
