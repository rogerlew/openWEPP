# ASSURE-04A Terminal Heavy Gate Runner

Status: **PASS**

Evidence class: **Ran**

Frozen base and HEAD: `81770ecb8f9e65702c7401852efa3d7f4682d15a`

Run date: 2026-07-15 UTC

## Verification B Amended Terminal Verdict

ASSURE-04A's Verification B amended terminal sequence passed all five required
gates on one stable source freeze. The fresh adjudicated CRAP acquisition
reported zero actionable rows overall and zero actionable rows in touched
files at threshold 30. The protected public surfaces and complete `usersum/`
aggregate remained byte-identical to the package's pre-implementation freeze.

The earlier post-review sequence remains preserved below as HOLD evidence. Its
failed CRAP bundle is preserved at
[`validation-evidence/adjudicated-crap-hold-20260715T160553Z/`](validation-evidence/adjudicated-crap-hold-20260715T160553Z/).
The pre-presence-remediation PASS bundle was copied byte-for-byte before this
acquisition to
[`validation-evidence/adjudicated-crap-pass-pre-presence-remediation-20260715T165752Z/`](validation-evidence/adjudicated-crap-pass-pre-presence-remediation-20260715T165752Z/).
The Verification B amended passing bundle is
[`validation-evidence/adjudicated-crap/`](validation-evidence/adjudicated-crap/).

## Prior HOLD Verdict

The post-review-remediation terminal sequence passed formatting, Clippy, the
full Nextest profile, and dependency policy. The required fresh adjudicated
CRAP gate failed with four actionable production rows above the closure
threshold of 30. ASSURE-04A therefore remains on HOLD.

All four actionable rows are in the touched, untracked production file
`crates/openwepp-assurance/src/v2.rs`. This evidence does not authorize an
adjudication or threshold exception. The complete heavy-gate sequence must be
restarted from a new freeze after source remediation.

## Prior HOLD Required Gate Results

| Order | Exact command | Start (UTC) | Finish (UTC) | Duration | Exit | Result |
| ---: | --- | --- | --- | ---: | ---: | --- |
| 1 | `cargo fmt --check` | `2026-07-15T15:53:02Z` | `2026-07-15T15:53:04Z` | 2.285 s | 0 | PASS |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | `2026-07-15T15:53:21Z` | `2026-07-15T15:53:45Z` | 24.363 s | 0 | PASS |
| 3 | `cargo nextest run --workspace --profile full` | `2026-07-15T15:54:05Z` | `2026-07-15T16:04:33Z` | 628.362 s | 0 | PASS |
| 4 | `cargo deny check` | `2026-07-15T16:05:19Z` | `2026-07-15T16:05:21Z` | 1.481 s | 0 | PASS |
| 5 | `bash tools/release/run_adjudicated_crap_gate.sh --base-ref 81770ecb8f9e65702c7401852efa3d7f4682d15a --output-dir docs/work-packages/20260715-assure04a-v2-source-identity-foundation-001/artifacts/validation-evidence/adjudicated-crap` | `2026-07-15T16:05:53Z` | `2026-07-15T16:41:16Z` | 2,123.558 s | 1 | FAIL |

`cargo deny check` reported all four checks as OK: advisories, bans, licenses,
and sources.

## Prior HOLD Full Nextest Evidence

| Field | Value |
| --- | --- |
| Run ID | `19876317-b4d6-49ce-b087-76ba36665aee` |
| Profile | `full` |
| Binaries | 184 |
| Executed | 1,985 |
| Passed | 1,985 |
| Failed | 0 |
| Skipped | 3 |
| Slow | 5 |
| Nextest-reported test time | 607.920 s |
| JUnit path | `target/nextest/full/junit.xml` |
| JUnit size | 470,474 bytes |
| JUnit SHA-256 | `4819761c23da1c6081208c4b9255b9014daec4ab8491a739a4f91788423262ea` |

The JUnit root identifies the same run ID and records 1,985 tests, zero
failures, and zero errors.

## Prior HOLD Adjudicated CRAP Evidence

Canonical report:
[`validation-evidence/adjudicated-crap/adjudicated-crap-report.md`](validation-evidence/adjudicated-crap/adjudicated-crap-report.md)

| Field | Value |
| --- | --- |
| Acquisition | Fresh; eligible for current-source closure |
| Threshold | CRAP strictly greater than 30 is raw debt; closure requires actionable rows at or below 30 |
| Production entries | 8,536 |
| Measurement inputs | 429 |
| Raw rows above threshold | 6 |
| Adjudicated rows | 2 |
| Actionable rows | 4 |
| Touched production files | 3 |
| Actionable rows in touched files | 4 |
| Actionable rows outside touched files | 0 |
| Debt status | FAIL |
| Production-source count | 223 |
| Production-source manifest before/after/final | `c3b47dcc792c749707ddb8ef5434579e56d556012361d7963247039ee165cc4e` |
| Workspace CRAP JSON SHA-256 | `1b6e23ac57be052b84442c083189d08bdc95dd8f03906c0befddb99b5d29a897` |
| LCOV SHA-256 | `ec12316f0484d0ebf26d19758e640ff4e3bd270165d002c31bda95d26d777b5c` |
| Adjudication registry SHA-256 | `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f` |
| Machine report SHA-256 | `03c7ec557ff56607a9c2957b1e5b1cf198c850df2620ed8693f24825e1935a46` |
| Human report SHA-256 | `97ba2e8e8de6830e3428be3626090b81c4be96e022c5c5742e669fda3175df93` |
| Run-status SHA-256 | `d0f8d8eb59940aba6195e201dd9b6be74014b7c2eb231bd8d57f7207bffdb8bd` |
| Evidence checksum manifest SHA-256 | `23347482903a7968ee5ebea57dfa7999b340aca0454c1da375d574fba116c789` |

The canonical checksum manifest was verified with `sha256sum -c`; all 16
listed artifacts passed. The production-source manifests before acquisition,
after acquisition, and at final disposition are byte-identical.

### Touched Production Files

| Status | Path | Actionable rows |
| --- | --- | ---: |
| `M` | `crates/openwepp-assurance/src/cli.rs` | 0 |
| `M` | `crates/openwepp-assurance/src/lib.rs` | 0 |
| `U` | `crates/openwepp-assurance/src/v2.rs` | 4 |

### Actionable Rows

| File | Function | Line | CC | Coverage | CRAP |
| --- | --- | ---: | ---: | ---: | ---: |
| `crates/openwepp-assurance/src/v2.rs` | `validate_schema_document` | 529 | 25 | 76.6667% | 32.9398 |
| `crates/openwepp-assurance/src/v2.rs` | `validate_dependency` | 1262 | 28 | 59.1549% | 81.4238 |
| `crates/openwepp-assurance/src/v2.rs` | `validate_result` | 1370 | 22 | 71.4286% | 33.2886 |
| `crates/openwepp-assurance/src/v2.rs` | `validate_research_object` | 1472 | 20 | 53.3333% | 60.6519 |

### Existing Adjudicated Rows

| File | Function | Line | CRAP | Adjudication |
| --- | --- | ---: | ---: | --- |
| `crates/openwepp-meteorology/src/error.rs` | `MeteorologyError::fmt` | 35 | 56 | `CQR-LOW-L08` |
| `crates/openwepp-sim-contract/src/symbols.rs` | `SymbolAliasRegistryError::fmt` | 67 | 90 | `CQR-LOW-L11` |

No adjudication was invalid or stale.

## Prior HOLD Source-Freeze Identity

The terminal freeze was captured after review disposition and remediation.
Package artifact paths matching `docs/work-packages/**/artifacts/**` were
excluded because the gate runner was authorized to write evidence there. All
source, tests, package specification, prompts, roadmap, queue, Cargo manifest,
and Cargo lockfile paths remained included.

| Representation | Count or size | SHA-256 |
| --- | ---: | --- |
| Non-artifact `git status --porcelain=v1 --untracked-files=all` | 27 rows | `30edf4798a0ced69a76d0cc2efad1e371dbfaf1f3f6a01ba15f1032d6ad21b72` |
| Binary full-index diff from HEAD | 37,975 bytes | `fa54fdfb7027c20f33d7613d3708dd4b444c04f4dd2fa33b37b81610039ff7ed` |
| Sorted changed-path list | 27 paths | `302a37b697b85b2e3c2c5171ce0f730f6f2036f1704c6953d9bb7baf633b02fb` |
| Sorted changed-file content manifest | 3,256 bytes | `3467409ac744d4afd87d4d7770e856331fdd3d83e3cf95c9d1a19ef10da8b6e2` |

The four representations were regenerated after the failed CRAP gate and
compared byte-for-byte with `cmp`; all were equal.

### Post-run Drift

After the HOLD was reported to the package owner, a final current-tree check
detected one newer source edit. The changed-path and status sets were unchanged,
but `crates/openwepp-assurance/src/v2.rs` changed from the frozen SHA-256
`9b82b865d47e0e2c5954437977a982ae28f03256c7dba3161e9e1eff2ef7bb12`
to `98a7bf064adeefe3809f5c1ea9343f37c21cdb01bf6b8f4561e1af0d2da1f6c2`.
The protected files and aggregate `usersum/` manifest remained unchanged.

The generated CRAP source manifests prove that the gate itself used identical
production bytes before acquisition, after acquisition, and at its final
disposition. The later edit means this failed run is historical evidence for
the stated freeze, not closure evidence for the newer working tree.

### Changed-File Manifest

```text
993d50b82d2a83e411cb831e08fac526406b9625c70488ed3ce3f591a02a153a  Cargo.lock
6968752676af113aac49ac847ae8d094afecd6fc57d9c572b22ed745501b047b  Cargo.toml
ef728218dc9c1723208848c1155722b5074075688315e4f48d2f045e09e174f9  assurance/README.md
7be9be7faaa5247ab1796e55a9cf8128955806996e58a6cde0f6706f72931d79  assurance/v2/README.md
e76d43e9ee337bf5678243a9b09b1f4c19eb5f2e8ea54a6af5ac485ab02324a8  assurance/v2/catalog.yaml
18a270516f1c5e221e1d9721e37bbb83d8aca69431952cf71336e7f35d30db13  assurance/v2/reports/linear-groundwater-reservoir-recurrence/manuscript.md
39a69a4fe723b26842becf719e3df8380985b478022c477fca32b46b58bea3bb  assurance/v2/reports/linear-groundwater-reservoir-recurrence/report.yaml
5fc3aa1834a41f277bd750373bd50c4223a5cf8503e25f3f16c13e509faed82d  assurance/v2/reports/linear-groundwater-reservoir-recurrence/results/h2637-ledger.json
41ada54b6ce96cc897bc7125ba737bab8194835488672903f717c2f350c6e483  assurance/v2/reports/linear-groundwater-reservoir-recurrence/results/two-day-recurrence.json
77a3c8b804a1f6f01c1b1ae9f2ea9cc341f91efa95d827530c14e7f29f92d8fe  assurance/v2/reports/linear-groundwater-reservoir-recurrence/supplement.md
7d15b4e56c2d519680ee906d2df1346a721a9dbcd2ec647fc7f3d787d2b6a520  assurance/v2/schemas/catalog.schema.json
70e09461fb223458c75726a7ce32038e84c62105e7b918bce0ffa68c937c5ba4  assurance/v2/schemas/report.schema.json
417efb4dbf2d9209cff3c41f52eca2637325c667dccc7c3588d14a0e8dc673a4  assurance/v2/schemas/result.schema.json
2e9309b78f25a93b4ba12c2b8911dc2b539a9d8263715e46c92206a0b974349d  crates/openwepp-assurance/Cargo.toml
22a5c043cec7fb4f5f86d2024bb61c1f842db718d9df45128e935c25891f5b68  crates/openwepp-assurance/src/cli.rs
b5e5ea4f372b56ddc80fedb482c1fe737126c26e87c8638a5c80103dfafd8620  crates/openwepp-assurance/src/lib.rs
9b82b865d47e0e2c5954437977a982ae28f03256c7dba3161e9e1eff2ef7bb12  crates/openwepp-assurance/src/v2.rs
09d062532eb091ce33c2e708e12c6719434fa8f07109a76c7c357932c2a4a636  docs/ROADMAP.md
80577fa1ccd88af607b07ab5abb7284c436ea8cc135bc0be37d0b98f2919d041  docs/planning/scientific-assurance-v2-implementation-roadmap.md
233401ec4fb1f3c5ec4f5da619a2beeef4710b84f9269fe7cbcbf43089d748c4  docs/work-packages/20260715-assure04a-v2-source-identity-foundation-001/package.md
d2fb868f808496589534fc2a9edc3dc80635494988de98124490dd2214f3aa2b  docs/work-packages/20260715-assure04a-v2-source-identity-foundation-001/prompts/README.md
ab76051397824e45122c33697859a5a80381efa59e4a3e7c537091b5d1e5062e  docs/work-packages/20260715-assure04a-v2-source-identity-foundation-001/prompts/active/20260715-codex-execute-assure04a_prompt.md
913ae586f70432c61cf1f25e47fca4285523f93320c7429f077b78120609cb39  docs/work-packages/20260715-assure04a-v2-source-identity-foundation-001/prompts/active/README.md
468a9682a96fe60cbda6a7a9d611e8a8a6fc3752ac86160f99a178621c772bde  docs/work-packages/20260715-assure04a-v2-source-identity-foundation-001/prompts/archived/README.md
8e8435509556b12466959665e63b99a429850675d882d566163dbd024d8f11ef  docs/work-packages/README.md
b26d4cdbe5bc30b518b56f7a789f6c4f7a4ef8831045d9ed4efcddd225387800  tests/integration/assurance_dossier_build_contract.rs
04713b0106e525a56ddba5b666614bad6f2d3324f0635143ddab84ba2ebbc037  tests/integration/assurance_v2_source_contract.rs
```

## Prior HOLD Protected-Surface Recheck

| Path | Before and after SHA-256 |
| --- | --- |
| `assurance/catalog.yaml` | `cb9cb601739b64f8cb497c0999ca268859946f2ce7e57532de8c08b9ed30801f` |
| `assurance/templates/catalog.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| `assurance/generated/wepppy-usersum.yaml` | `08b21cbe20ed059eb61f01f9965d8603779501bde90a728bc7a6c138a69258eb` |
| `usersum/assurance/README.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| Sorted SHA-256 manifest of all regular files under `usersum/` | `deb9f2c646aa5eb4ad9e427f8a7cec6ad51e3f9f6b47ffe29d14b4aed4bdcb7a` |

All protected values equal the package's pre-implementation freeze.

## Prior HOLD Toolchain

- Cargo: `1.92.0 (344c4567c 2025-10-21)`
- rustc: `1.92.0 (ded5c06cf 2025-12-08)`, LLVM `21.1.3`
- cargo-nextest: `0.9.138 (fc97e97bb 2026-06-21)`
- cargo-deny: `0.19.6`
- cargo-llvm-cov: `0.8.7`
- cargo-crap: `0.2.2`

## Prior HOLD Closure Requirement

Remediate the four actionable rows without changing the adjudication registry
or weakening the threshold. After remediation, capture a new source freeze and
restart all five required gates. Closure requires zero actionable rows and zero
actionable rows in touched files at the adjudicated CRAP threshold of 30.

## Prior Pre-presence PASS Required Gate Results

No result from the prior HOLD was reused as terminal evidence.

| Order | Exact command | Start (UTC) | Finish (UTC) | Duration | Exit | Result |
| ---: | --- | --- | --- | ---: | ---: | --- |
| 1 | `cargo fmt --check` | `2026-07-15T16:46:19Z` | `2026-07-15T16:46:21Z` | 2.189 s | 0 | PASS |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | `2026-07-15T16:47:03Z` | `2026-07-15T16:47:07Z` | 3.876 s | 0 | PASS |
| 3 | `cargo nextest run --workspace --profile full` | `2026-07-15T16:47:25Z` | `2026-07-15T16:57:09Z` | 584.071 s | 0 | PASS |
| 4 | `cargo deny check` | `2026-07-15T16:57:29Z` | `2026-07-15T16:57:31Z` | 2.073 s | 0 | PASS |
| 5 | `bash tools/release/run_adjudicated_crap_gate.sh --base-ref 81770ecb8f9e65702c7401852efa3d7f4682d15a --output-dir docs/work-packages/20260715-assure04a-v2-source-identity-foundation-001/artifacts/validation-evidence/adjudicated-crap` | `2026-07-15T16:57:52Z` | `2026-07-15T17:34:08Z` | 2,175.210 s | 0 | PASS |

`cargo deny check` reported advisories, bans, licenses, and sources as OK.

## Prior Pre-presence PASS Full Nextest Evidence

| Field | Value |
| --- | --- |
| Run ID | `363c4169-11bb-43bc-ae1b-acbe3bb07ad1` |
| Profile | `full` |
| Binaries | 184 |
| Executed | 1,985 |
| Passed | 1,985 |
| Failed | 0 |
| Skipped | 3 |
| Slow | 4 |
| Nextest-reported test time | 578.285 s |
| JUnit path | `target/nextest/full/junit.xml` |
| JUnit size | 470,471 bytes |
| JUnit SHA-256 | `a80b01bb1efcaeadc63c00185b5be0047ed0a4ef8de0bad1779b126160527826` |

The JUnit root identifies the same run ID and records 1,985 tests, zero
failures, and zero errors.

## Prior Pre-presence PASS Adjudicated CRAP Evidence

Canonical report:
[`validation-evidence/adjudicated-crap-pass-pre-presence-remediation-20260715T165752Z/adjudicated-crap-report.md`](validation-evidence/adjudicated-crap-pass-pre-presence-remediation-20260715T165752Z/adjudicated-crap-report.md)

| Field | Value |
| --- | --- |
| Acquisition | Fresh; eligible for current-source closure |
| Threshold | CRAP strictly greater than 30 is raw debt; closure requires zero actionable rows |
| Production entries | 8,568 |
| Raw rows above threshold | 2 |
| Adjudicated rows | 2 |
| Actionable rows | 0 |
| Touched production files | 3 |
| Actionable rows in touched files | 0 |
| Actionable rows outside touched files | 0 |
| Debt status | PASS |
| Production-source count | 223 |
| Production-source manifest before/after/final | `97339a2878ee0872eea0c126a5812f1c7ddc1ee6521d9a22a16f9662e60953dc` |
| Workspace CRAP JSON SHA-256 | `7b3ce64468f7561e875efd250afd04cdb4cddb8974ffc9f182a65f212706ce02` |
| LCOV SHA-256 | `e298c0b76f6862955f431f64bb812c5a26be11c6772d3cd38ee209a342755180` |
| Adjudication registry SHA-256 | `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f` |
| Machine report SHA-256 | `866996a65418298a1e3bda65dbaffa859c5df4ea00832c7eada91c1826798625` |
| Human report SHA-256 | `d179bec6b22759b2d3b28c3f353433e241b9c998089a4313753d197bc38dabfa` |
| Run-status SHA-256 | `828131d2fdb2071c84e71ab2e520f75aff35c5d708ee072f5c113ced1230eb6f` |
| Evidence checksum manifest SHA-256 | `747623530cf8ab4652ff22806b3b4360ab21097242e9bc0e38ad782a416ae99b` |

The checksum manifest was verified with `sha256sum -c`; all 16 artifacts
passed. The production-source manifests before acquisition, after acquisition,
and at final disposition are byte-identical.

### Terminal Touched Production Files

| Status | Path | Actionable rows |
| --- | --- | ---: |
| `M` | `crates/openwepp-assurance/src/cli.rs` | 0 |
| `M` | `crates/openwepp-assurance/src/lib.rs` | 0 |
| `U` | `crates/openwepp-assurance/src/v2.rs` | 0 |

### Terminal Adjudicated Rows

| File | Function | Line | CC | Coverage | CRAP | Adjudication |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| `crates/openwepp-meteorology/src/error.rs` | `MeteorologyError::fmt` | 35 | 7 | 0% | 56 | `CQR-LOW-L08` |
| `crates/openwepp-sim-contract/src/symbols.rs` | `SymbolAliasRegistryError::fmt` | 67 | 9 | 0% | 90 | `CQR-LOW-L11` |

There were no actionable rows and no invalid or stale adjudications.

## Prior Pre-presence PASS Source-Freeze Identity

The terminal freeze was captured after CRAP-driven decomposition. Package
artifact paths matching `docs/work-packages/**/artifacts/**` were excluded
because the gate runner was authorized to write evidence there. All source,
tests, package specification, prompts, roadmap, queue, Cargo manifest, and
Cargo lockfile paths remained included.

| Representation | Count or size | SHA-256 |
| --- | ---: | --- |
| Non-artifact `git status --porcelain=v1 --untracked-files=all` | 27 rows | `30edf4798a0ced69a76d0cc2efad1e371dbfaf1f3f6a01ba15f1032d6ad21b72` |
| Binary full-index diff from HEAD | 37,975 bytes | `fa54fdfb7027c20f33d7613d3708dd4b444c04f4dd2fa33b37b81610039ff7ed` |
| Sorted changed-path list | 27 paths | `302a37b697b85b2e3c2c5171ce0f730f6f2036f1704c6953d9bb7baf633b02fb` |
| Sorted changed-file content manifest | 3,256 bytes | `d36aec1d83a3351cf0861cc47412ce0336a74a74a8a62c37b18b5f2460564e26` |

All four representations were regenerated after gate 5 and compared
byte-for-byte with `cmp`; all were equal. The decomposed
`crates/openwepp-assurance/src/v2.rs` SHA-256 was
`422b62a30e4863122c51898914202d85b6214ab051188829991a787a1d635345`.

## Prior Pre-presence PASS Protected-Surface Recheck

| Path | Before and after SHA-256 |
| --- | --- |
| `assurance/catalog.yaml` | `cb9cb601739b64f8cb497c0999ca268859946f2ce7e57532de8c08b9ed30801f` |
| `assurance/templates/catalog.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| `assurance/generated/wepppy-usersum.yaml` | `08b21cbe20ed059eb61f01f9965d8603779501bde90a728bc7a6c138a69258eb` |
| `usersum/assurance/README.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| Sorted SHA-256 manifest of all regular files under `usersum/` | `deb9f2c646aa5eb4ad9e427f8a7cec6ad51e3f9f6b47ffe29d14b4aed4bdcb7a` |

All values equal the package's pre-implementation freeze.

## Prior Pre-presence PASS Toolchain

- Cargo: `1.92.0 (344c4567c 2025-10-21)`
- rustc: `1.92.0 (ded5c06cf 2025-12-08)`, LLVM `21.1.3`
- cargo-nextest: `0.9.138 (fc97e97bb 2026-06-21)`
- cargo-deny: `0.19.6`
- cargo-llvm-cov: `0.8.7`
- cargo-crap: `0.2.2`

## Verification B Amended PASS Required Gate Results

No result from the focused quick run or either earlier heavy sequence was
reused as terminal evidence.

| Order | Exact command | Start (UTC) | Finish (UTC) | Duration | Exit | Result |
| ---: | --- | --- | --- | ---: | ---: | --- |
| 1 | `cargo fmt --check` | `2026-07-15T17:57:19Z` | `2026-07-15T17:57:22Z` | 2.318 s | 0 | PASS |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | `2026-07-15T17:57:38Z` | `2026-07-15T17:57:43Z` | 4.200 s | 0 | PASS |
| 3 | `cargo nextest run --workspace --profile full` | `2026-07-15T17:58:05Z` | `2026-07-15T18:08:04Z` | 598.899 s | 0 | PASS |
| 4 | `cargo deny check` | `2026-07-15T18:08:34Z` | `2026-07-15T18:08:35Z` | 0.902 s | 0 | PASS |
| 5 | `bash tools/release/run_adjudicated_crap_gate.sh --base-ref 81770ecb8f9e65702c7401852efa3d7f4682d15a --output-dir docs/work-packages/20260715-assure04a-v2-source-identity-foundation-001/artifacts/validation-evidence/adjudicated-crap` | `2026-07-15T18:09:01Z` | `2026-07-15T18:45:27Z` | 2,185.931 s | 0 | PASS |

`cargo deny check` reported advisories, bans, licenses, and sources as OK.

The package owner also reported focused fmt, strict crate Clippy, and quick
Nextest evidence before dispatch: quick run
`3971cb34-0b18-451b-b52e-2db7c483888c`, 25/25 passed. That focused evidence is
context only; it is not one of this terminal sequence's five gate results.

## Verification B Amended Full Nextest Evidence

| Field | Value |
| --- | --- |
| Run ID | `8d011a3f-91ca-4814-b310-6b0fc65e6c7a` |
| Profile | `full` |
| Binaries | 184 |
| Executed | 1,986 |
| Passed | 1,986 |
| Failed | 0 |
| Skipped | 3 |
| Slow | 4 |
| Nextest-reported test time | 592.774 s |
| JUnit path | `target/nextest/full/junit.xml` |
| JUnit size | 470,674 bytes |
| JUnit SHA-256 | `86e802b20b7b23b217e532103526e588c4933cc73ea659bea3b924570acd1faa` |

The JUnit root identifies the same run ID and records 1,986 tests, zero
failures, and zero errors.

## Verification B Amended Adjudicated CRAP Evidence

Canonical report:
[`validation-evidence/adjudicated-crap/adjudicated-crap-report.md`](validation-evidence/adjudicated-crap/adjudicated-crap-report.md)

| Field | Value |
| --- | --- |
| Acquisition | Fresh; eligible for current-source closure |
| Threshold | CRAP strictly greater than 30 is raw debt; closure requires zero actionable rows |
| Production entries | 8,572 |
| Raw rows above threshold | 2 |
| Adjudicated rows | 2 |
| Actionable rows | 0 |
| Touched production files | 3 |
| Actionable rows in touched files | 0 |
| Actionable rows outside touched files | 0 |
| Debt status | PASS |
| Production-source count | 223 |
| Production-source manifest before/after/final | `9db9abcf7cb4bbd5ef7387bcada9831528d0f5f529ca7656584669739139831a` |
| Workspace CRAP JSON SHA-256 | `d073d6b9f400435681db34a604834bc6c9d7d802adc5c009c5bbfdfbf11d69eb` |
| LCOV SHA-256 | `1a55b7fa95672feb4e5a25fa2fd03004c37575bb5f3bde3034242855949afba1` |
| Adjudication registry SHA-256 | `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f` |
| Machine report SHA-256 | `75a422ba9b0c6a6bbd9cf5d9caa048d7a16a1607e5f3b6ef1130251e7219d6bf` |
| Human report SHA-256 | `f5b94ec1a618673a0e993994de386b43f6b6fbb6aa78ea703fa8b703e240520a` |
| Run-status SHA-256 | `ca283bb134e671d6837308d6030740bd9ff5b794fed3a0a78521ece8641955bb` |
| Evidence checksum manifest SHA-256 | `322a94e6ec92b50356caca435ea979e5dba308f3433e5c01fa93405d8becd952` |

The checksum manifest was verified with `sha256sum -c`; all 16 artifacts
passed. The production-source manifests before acquisition, after acquisition,
and at final disposition are byte-identical.

### Verification B Amended Touched Production Files

| Status | Path | Actionable rows |
| --- | --- | ---: |
| `M` | `crates/openwepp-assurance/src/cli.rs` | 0 |
| `M` | `crates/openwepp-assurance/src/lib.rs` | 0 |
| `U` | `crates/openwepp-assurance/src/v2.rs` | 0 |

### Verification B Amended Adjudicated Rows

| File | Function | Line | CC | Coverage | CRAP | Adjudication |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| `crates/openwepp-meteorology/src/error.rs` | `MeteorologyError::fmt` | 35 | 7 | 0% | 56 | `CQR-LOW-L08` |
| `crates/openwepp-sim-contract/src/symbols.rs` | `SymbolAliasRegistryError::fmt` | 67 | 9 | 0% | 90 | `CQR-LOW-L11` |

There were no actionable rows and no invalid or stale adjudications.

### Coverage Collection Is Not the Workflow

The CRAP gate's coverage acquisition supplies line/region execution data for
complexity-risk measurement. It is not the terminal test workflow. The
independent full Nextest run above is the workflow evidence: it has its own
command, run ID, count, timing, and JUnit checksum. Both gates passed on the
same frozen source bytes.

## Verification B Amended Source-Freeze Identity

The amended freeze was captured after Verification B remediation. Package
artifact paths matching `docs/work-packages/**/artifacts/**` were excluded
because the gate runner was authorized to write evidence there. All source,
tests, package specification, prompts, roadmap, queue, Cargo manifest, and
Cargo lockfile paths remained included.

| Representation | Count or size | SHA-256 |
| --- | ---: | --- |
| Non-artifact `git status --porcelain=v1 --untracked-files=all` | 27 rows | `30edf4798a0ced69a76d0cc2efad1e371dbfaf1f3f6a01ba15f1032d6ad21b72` |
| Binary full-index diff from HEAD | 41,074 bytes | `394df67b2cd3279c043916bcc67ee63b5878c0007820e543983ad20e77bde48c` |
| Sorted changed-path list | 27 paths | `302a37b697b85b2e3c2c5171ce0f730f6f2036f1704c6953d9bb7baf633b02fb` |
| Sorted changed-file content manifest | 3,256 bytes | `f2094bf9671c37275023dcfa887ee2466c85df0ac7be68f84f857b77216bcbbe` |

All four representations were regenerated after gate 5 and compared
byte-for-byte with `cmp`; all were equal.

### Post-gate Closure-document Drift

The representations above are the exact gate-time freeze, not a claim that
later package-closure documentation remained frozen. After the runner released
the source, the package owner advanced four governance records from remediation
to terminal-verification state and reconciled their current gate summaries.
At renewed Verification A intake, the status and changed-path sets remained
identical to the gate freeze, while only these four of the 27 non-artifact
content rows differed:

| Path | Gate-time SHA-256 | Verification-intake SHA-256 |
| --- | --- | --- |
| `docs/ROADMAP.md` | `aa67ed3761e8b94cc70e0044264e8de896c4aa1210cf13bbe1cf49bca2cc27d3` | `0461143314f5b981e71bc3c68d0fb0b64f71518b77755b4e507e5f761993dc2e` |
| `docs/planning/scientific-assurance-v2-implementation-roadmap.md` | `ed1576621fdcb0cc497f329960dfc085e3d8366a8dcc1b9b862f85e2c0c5346f` | `e9f7d16e3e4f1fbb83713f18b408ee69b1abd37c7959b184e56b02d60e3a1312` |
| `docs/work-packages/20260715-assure04a-v2-source-identity-foundation-001/package.md` | `36d26f85f65d0e3f1d8c21f682bb7b352e3ec818af99c1faa3ad822d650a5194` | `7047c024ecf699596ca16aaacae756a3ffbea7313c300580762dca4c65025cf4` |
| `docs/work-packages/README.md` | `37cea04396b742a4d528095428502748290b4c289e7e6b96b67e5a36fbd9a996` | `b76927c402699ab3a5991bcfc2332966b019bc6f360accc00e51448d6f79de84` |

The current path-sorted 27-file content manifest is
`d986d1349437bcad2d086ff5e62bcaa126b5c99bc51f9d0722bfc731b0dccb84`.
The current tracked `git diff --binary --full-index HEAD` is 41,109 bytes with
SHA-256
`21d18de629bd5eac50dd0d94ceccef384c01721b7787961a7027e760b59878b1`.
All other 23 non-artifact freeze paths, including production source, v2 source,
schemas, scientific prose, Cargo files, and both touched tests, remain exact.
All 223 production and 429 measurement-input rows also remain exact under the
fresh gate manifests. These four documentation changes are closure accounting;
they do not reuse heavy evidence after a production or test change.

### Final Post-verification Closure Snapshot

After renewed Verification A and B both passed, the package owner performed
the mechanical closure transition: marked the package complete, advanced
ASSURE-04B to `next` without authorizing it, updated the two roadmap/catalog
records, and moved the unchanged kickoff prompt from `active/` to `archived/`.
The final non-artifact snapshot is:

| Representation | Count or size | SHA-256 |
| --- | ---: | --- |
| Non-artifact `git status --porcelain=v1 --untracked-files=all` | 27 rows | `17345898cf8d30e0ffc4ad66a5a40371f8b7c8143560792aa6cef05baa1675d9` |
| Binary full-index diff from HEAD | 42,850 bytes | `2ba3a66830b1a65aa26bf5a579e9470c96600c636aff3a7af89fe8062abd1f1b` |
| Sorted changed-path list | 27 paths | `65e85c5fe3a6dfe8c9acd1c5a1526a9072444b43cf518f6b1d962049104ea0b5` |
| Sorted changed-file content manifest | 3,258 bytes | `95187b9e114d8c88142fd3a12568d064743560de0184ca3ea75dc00c1175089e` |

Relative to the gate-time freeze, the only final content changes are the six
closure-governance files below. The kickoff prompt moved locators with its
SHA-256 unchanged at
`ab76051397824e45122c33697859a5a80381efa59e4a3e7c537091b5d1e5062e`.

| Path | Gate-time SHA-256 | Final SHA-256 |
| --- | --- | --- |
| `docs/ROADMAP.md` | `aa67ed3761e8b94cc70e0044264e8de896c4aa1210cf13bbe1cf49bca2cc27d3` | `80dad3a94493711dcce000774c68ef1d9dda4670382466cc491c7e0fda39cb2a` |
| `docs/planning/scientific-assurance-v2-implementation-roadmap.md` | `ed1576621fdcb0cc497f329960dfc085e3d8366a8dcc1b9b862f85e2c0c5346f` | `6731f07733e7235070f882e2d8e01561f635b7dadbc0e6f91c8dd07fc1d9bf42` |
| `docs/work-packages/20260715-assure04a-v2-source-identity-foundation-001/package.md` | `36d26f85f65d0e3f1d8c21f682bb7b352e3ec818af99c1faa3ad822d650a5194` | `8625b2066615d1f9a2507d6f0c9f6ca6a9f0442308d07bc1c7a37835b6c62714` |
| `docs/work-packages/20260715-assure04a-v2-source-identity-foundation-001/prompts/README.md` | `2658ee1e09dc88d5d7e1a0f863d59e99a2bded8a66be621b6d71905e39e272a4` | `c878a52076f6a3deea561c5bf941954c2cb74825b8ab1f543ff5607898e88f1e` |
| `docs/work-packages/20260715-assure04a-v2-source-identity-foundation-001/prompts/active/README.md` | `eadb0e7eebe0dc12ff916f7ed539c88866243a214c4792b24c09fb0e11d82f43` | `c29a42006d2c0e627dd8f24df05296b5b4e67cef6d791421543c2519ef5cf0d6` |
| `docs/work-packages/README.md` | `37cea04396b742a4d528095428502748290b4c289e7e6b96b67e5a36fbd9a996` | `5deb5172f3140d53f568e428c6d4c87c314ca437cd775b013a3fd2e052161ac2` |

All executable source, v2 schemas/source/prose/results, Cargo files, and tests
remain byte-identical to the amended heavy-gate freeze. The 223 production and
429 measurement-input manifests, protected surfaces, and full `usersum`
aggregate remain exact. This final snapshot dispositions only the expected
post-verification documentation lifecycle transition.

### Verification B Amended Line Counts and Hashes

| Path | SHA-256 | Physical lines | Nonblank lines | Bytes |
| --- | --- | ---: | ---: | ---: |
| `crates/openwepp-assurance/src/v2.rs` | `886a5693d67ab88b0b0a6901260017eeca636aa7ccad1ad0faed7ccf24104b58` | 2,042 | 1,934 | 63,288 |
| `tests/integration/assurance_v2_source_contract.rs` | `d81ed537a7ab8441e406513a0569fe761292d6f892067285b8942f991e9aa4cc` | 709 | 646 | 27,360 |

Physical lines and bytes were measured with `wc -l -c`; nonblank lines were
counted with `awk 'NF'`. The counts and hashes were identical before and after
the complete gate sequence.

## Verification B Amended Protected-Surface Recheck

| Path | Before and after SHA-256 |
| --- | --- |
| `assurance/catalog.yaml` | `cb9cb601739b64f8cb497c0999ca268859946f2ce7e57532de8c08b9ed30801f` |
| `assurance/templates/catalog.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| `assurance/generated/wepppy-usersum.yaml` | `08b21cbe20ed059eb61f01f9965d8603779501bde90a728bc7a6c138a69258eb` |
| `usersum/assurance/README.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| Sorted SHA-256 manifest of all regular files under `usersum/` | `deb9f2c646aa5eb4ad9e427f8a7cec6ad51e3f9f6b47ffe29d14b4aed4bdcb7a` |

All values equal the package's pre-implementation freeze.

## Verification B Amended Toolchain

- Cargo: `1.92.0 (344c4567c 2025-10-21)`
- rustc: `1.92.0 (ded5c06cf 2025-12-08)`, LLVM `21.1.3`
- cargo-nextest: `0.9.138 (fc97e97bb 2026-06-21)`
- cargo-deny: `0.19.6`
- cargo-llvm-cov: `0.8.7`
- cargo-crap: `0.2.2`
