# ASSURE-04B Independent Heavy Gate Runner

Status: **PASS -- complete independent heavy sequence closed**

Evidence class: **Ran**

Frozen base and HEAD: `22fb7dfbafdb9e82a42afe0a5356b4c923a45232`

Run date: 2026-07-15 UTC

## Current Verdict

The complete independently rerun sequence passed formatting, strict workspace
Clippy, full Nextest, dependency policy, and a fresh adjudicated CRAP gate.
Nextest passed all 2,001 executed tests with three skipped. Fresh CRAP produced
two raw rows, both exact existing adjudications outside the touched files, and
zero actionable rows. Every measurable touched-production-file maximum is at
or below 30; `lib.rs` has no measurable function row. ASSURE-04B therefore
passes this heavy closure sequence.

Both earlier HOLD attempts remain documented below as non-closable chronology.
No partial gate result or CRAP bundle from either attempt was reused. The second
attempt's failed CRAP bundle is retained under a clearly non-current archive
path; the canonical bundle contains only this fresh terminal PASS run.

## Current PASS Gate Results

| Order | Exact command | Start (UTC) | Finish (UTC) | Duration | Exit | Result |
| ---: | --- | --- | --- | ---: | ---: | --- |
| 1 | `cargo fmt --check` | `2026-07-15T21:49:53Z` | `2026-07-15T21:49:55Z` | 2.182 s | 0 | PASS |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | `2026-07-15T21:50:17Z` | `2026-07-15T21:50:22Z` | 5.038 s | 0 | PASS |
| 3 | `cargo nextest run --workspace --profile full` | `2026-07-15T21:50:51Z` | `2026-07-15T22:00:34Z` | 583.243 s | 0 | PASS |
| 4 | `cargo deny check` | `2026-07-15T22:01:49Z` | `2026-07-15T22:01:50Z` | 1.131 s | 0 | PASS |
| 5 | `bash tools/release/run_adjudicated_crap_gate.sh --base-ref 22fb7dfbafdb9e82a42afe0a5356b4c923a45232 --output-dir docs/work-packages/20260715-assure04b-v2-dependency-planner-001/artifacts/validation-evidence/adjudicated-crap` | `2026-07-15T22:02:14Z` | `2026-07-15T22:37:45Z` | 2,131.308 s | 0 | PASS |

`cargo deny check` reported advisories, bans, licenses, and sources as OK.

## Current PASS Full Nextest Evidence

| Field | Value |
| --- | --- |
| Run ID | `d754b564-4e64-4d91-810d-81fd67b50fa4` |
| Profile | `full` |
| Binaries | 185 |
| Executed | 2,001 |
| Passed | 2,001 |
| Failed | 0 |
| Skipped | 3 |
| Slow | 4 |
| Nextest-reported test time | 582.012 s |
| JUnit path | `target/nextest/full/junit.xml` |
| JUnit size | 473,968 bytes |
| JUnit SHA-256 | `5f6ff05f7eb5311f6aa8ce4cf83190b91ba6b3e09d00a743a3ea0e9b79392689` |

The JUnit root identifies the same run ID and records 2,001 tests, zero
failures, and zero errors.

## Current PASS Adjudicated CRAP Evidence

Canonical report:
[`validation-evidence/adjudicated-crap/adjudicated-crap-report.md`](validation-evidence/adjudicated-crap/adjudicated-crap-report.md)

| Field | Value |
| --- | --- |
| Acquisition | Fresh; eligible for current-source closure |
| Threshold | CRAP strictly greater than 30 is raw debt |
| Production entries | 8,694 |
| Raw rows above threshold | 2 |
| Adjudicated rows | 2 |
| Actionable rows | 0 |
| Touched production files | 6 |
| Actionable rows in touched files | 0 |
| Actionable rows outside touched files | 0 |
| Debt status | PASS |
| Production-source count | 225 |
| Production-source manifest before/after/final | `0f5f3193c6886135a72b1a8cb670743de99e67c4cfac79ffcc937299079d45cc` |
| Workspace CRAP JSON SHA-256 | `4bf27cfb60a385c2c8c65ff9b136fc6a028d12014ff6004deace2069187c2056` |
| LCOV SHA-256 | `1fb6979da78043ba4e925e11eba8ce4cc3432ada08c85da1b27a0ce6f4a1b9a1` |
| Adjudication registry SHA-256 | `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f` |
| Machine report SHA-256 | `4f60135469902b2445185c3210c9e41ae475d0bebf6be062e6ab9fc35ee49405` |
| Human report SHA-256 | `b071681e6c2239ba7e42ba87caf2f3eeeb7247d69d3053bf60f813a4df4490f0` |
| Run-status SHA-256 | `f6ee5adcd4e50e7f2bfb99a0045597a523e7397a7401fb09259e2f8d43f07c4f` |
| Evidence checksum manifest SHA-256 | `14cf14ebc678b3ac5e4051aab1468cb5fb26cb8252cea97124e69783494c2bf1` |

The canonical checksum manifest was verified with `sha256sum -c`; all 16
artifacts passed. The production-source manifests before acquisition, after
acquisition, and at final disposition are byte-identical.

### Current PASS Touched-Production Closure Rows

These maxima are derived from the fresh canonical `workspace-crap.json`, not
from pre-run coverage diagnostics. No touched file contains an adjudicated
exception, and every measurable maximum passes the threshold.

| Status | Touched production file | Maximum function | Line | CC | Coverage | Maximum CRAP | Disposition |
| --- | --- | --- | ---: | ---: | ---: | ---: | --- |
| `M` | `crates/openwepp-assurance/src/cli.rs` | `parse_options` | 135 | 17 | 97.3684% | 17.0053 | PASS: at or below 30 |
| `M` | `crates/openwepp-assurance/src/engine.rs` | `create_snapshot` | 473 | 16 | 88.0952% | 16.4319 | PASS: at or below 30 |
| `M` | `crates/openwepp-assurance/src/lib.rs` | No function entry | N/A | N/A | N/A | N/A | PASS: no raw row above 30 |
| `M` | `crates/openwepp-assurance/src/v2.rs` | `validate_report_structure` | 1,104 | 26 | 100.0000% | 26.0000 | PASS: at or below 30 |
| `U` | `crates/openwepp-assurance/src/v2/confined.rs` | `read_regular_confined_platform` | 25 | 7 | 73.6842% | 7.8930 | PASS: at or below 30 |
| `U` | `crates/openwepp-assurance/src/v2/planner.rs` | `add_report_nodes` | 283 | 13 | 90.0000% | 13.1690 | PASS: at or below 30 |

The two raw rows are the existing exact adjudications `CQR-LOW-L08` and
`CQR-LOW-L11`; neither is in a touched file. No adjudication is invalid or
stale. The CRAP gate's coverage collection is metric evidence, not the
workflow; the independent full Nextest run above is the terminal workflow
evidence on the same stable source snapshot.

## Current PASS Source And Index Freeze

The fresh freeze excludes only
`docs/work-packages/20260715-assure04b-v2-dependency-planner-001/artifacts/**`.
Relative to the second HOLD attempt, accepted remediation changed only
`crates/openwepp-assurance/src/cli.rs` and
`tests/integration/assurance_v2_planner_contract.rs`; the Git index did not
change.

| Representation | Count or size | SHA-256 |
| --- | ---: | --- |
| Non-artifact status | 20 rows | `ddb1981e8710d009de7786f530128f6924606aaa7cefc97d7f4bf711e88e6d0d` |
| Non-artifact binary full-index diff | 32,887 bytes | `4c1721e41f358eae8cb6b14c7f070453e82620f32ed546e4f1912a1dadd36299` |
| Non-artifact changed-path list | 20 paths | `7d10d83382d1bb9a158be6eac194cb87389fe18d67045b7834b4c893a2240245` |
| Non-artifact content manifest | 2,310 bytes | `f7c5034e5596b56e50e01cbe9d48a098df48c5d8a1296ce8b4adf7d3d7011724` |
| Implementation path list | 11 paths | `8f59636f463f2643e4d670d93385bf02fabfa0c61f1bf00a402cdc93cf84105e` |
| Implementation content manifest | 1,125 bytes | `e41a24066333cc4b29ca3d2c34ee41269db4e0c8f7fd49bc16f9a724a7b8c9fd` |
| Sorted `git ls-files --stage` index listing | repository index | `80ae18484185fb741241b3d9e2fe110e7701dab06232a5602ffdde72c01ada6a` |

All seven representations were regenerated after the final gate and compared
byte-for-byte with `cmp`; all were equal. Protected files, the complete
`usersum/` aggregate, and recorded line counts also remained equal. No
out-of-bounds source, test, authority, governance, roadmap, prompt, protected,
or Git-index write occurred.

## Current PASS Line Counts

| Rust file | Physical lines | Nonblank lines | Bytes |
| --- | ---: | ---: | ---: |
| `crates/openwepp-assurance/src/v2.rs` | 2,064 | 1,950 | 64,411 |
| `crates/openwepp-assurance/src/v2/confined.rs` | 256 | 230 | 9,088 |
| `crates/openwepp-assurance/src/v2/planner.rs` | 1,114 | 1,058 | 34,304 |
| `crates/openwepp-assurance/src/engine.rs` | 622 | 582 | 20,776 |
| `crates/openwepp-assurance/src/cli.rs` | 285 | 261 | 9,099 |
| `crates/openwepp-assurance/src/lib.rs` | 20 | 17 | 632 |
| `tests/integration/assurance_v2_source_contract.rs` | 709 | 646 | 27,454 |
| `tests/integration/assurance_v2_planner_contract.rs` | 515 | 477 | 19,919 |

`v2.rs` remains a documented warning above 2,000 lines; no touched Rust file
reaches the 3,000-line closure block.

## Current PASS Protected Surfaces

| Path | Before and after SHA-256 |
| --- | --- |
| `assurance/catalog.yaml` | `cb9cb601739b64f8cb497c0999ca268859946f2ce7e57532de8c08b9ed30801f` |
| `assurance/templates/catalog.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| `assurance/generated/wepppy-usersum.yaml` | `08b21cbe20ed059eb61f01f9965d8603779501bde90a728bc7a6c138a69258eb` |
| `usersum/assurance/README.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| Sorted SHA-256 manifest of every regular file below `usersum/` | `deb9f2c646aa5eb4ad9e427f8a7cec6ad51e3f9f6b47ffe29d14b4aed4bdcb7a` |

## Current PASS Toolchain

- Cargo: `1.92.0 (344c4567c 2025-10-21)`
- rustc: `1.92.0 (ded5c06cf 2025-12-08)`, LLVM `21.1.3`
- cargo-nextest: `0.9.138 (fc97e97bb 2026-06-21)`
- cargo-deny: `0.19.6`
- cargo-llvm-cov: `0.8.7`
- cargo-crap: `0.2.2`

## First HOLD Verdict

The independent heavy sequence stopped at its first required failure.
Formatting passed. Workspace/all-targets Clippy failed on a test-only
`push_str(&format!(...))` construction. Full Nextest, cargo-deny, and fresh
adjudicated CRAP were not run. This evidence cannot close ASSURE-04B.

The diagnostic is `clippy::format_push_string` at
`tests/integration/assurance_v2_planner_contract.rs:385`. Clippy recommends a
formatted write directly into the existing `catalog` string.

## First HOLD Gate Results

| Order | Exact command | Start (UTC) | Finish (UTC) | Duration | Exit | Result |
| ---: | --- | --- | --- | ---: | ---: | --- |
| 1 | `cargo fmt --check` | `2026-07-15T20:41:09Z` | `2026-07-15T20:41:12Z` | 2.279 s | 0 | PASS |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | `2026-07-15T20:41:36Z` | `2026-07-15T20:41:50Z` | 14.746 s | 101 | FAIL |
| 3 | `cargo nextest run --workspace --profile full` | -- | -- | -- | -- | NOT RUN: stop-on-first-failure |
| 4 | `cargo deny check` | -- | -- | -- | -- | NOT RUN: stop-on-first-failure |
| 5 | `bash tools/release/run_adjudicated_crap_gate.sh --base-ref 22fb7dfbafdb9e82a42afe0a5356b4c923a45232 --output-dir docs/work-packages/20260715-assure04b-v2-dependency-planner-001/artifacts/validation-evidence/adjudicated-crap` | -- | -- | -- | -- | NOT RUN: stop-on-first-failure |

Because gate 3 did not run, there is no current terminal Nextest run ID, count,
or JUnit checksum. Because gate 5 did not run, raw, adjudicated, actionable, and
touched-production CRAP rows are unavailable. No CRAP closure claim is made.

## First HOLD Source And Index Freeze

The full freeze excludes only
`docs/work-packages/20260715-assure04b-v2-dependency-planner-001/artifacts/**`,
the runner's authorized evidence path. The implementation subset contains
changed Cargo manifests/lockfile, production Rust, and integration-test paths.

| Representation | Count or size | SHA-256 |
| --- | ---: | --- |
| Non-artifact status | 20 rows | `ddb1981e8710d009de7786f530128f6924606aaa7cefc97d7f4bf711e88e6d0d` |
| Non-artifact binary full-index diff | 31,085 bytes | `8c03ac0eebb86d8347d1b9dbfa4df963c2b9720f07f8d417cc2f54cb1217d93d` |
| Non-artifact changed-path list | 20 paths | `7d10d83382d1bb9a158be6eac194cb87389fe18d67045b7834b4c893a2240245` |
| Non-artifact content manifest | 2,310 bytes | `a1d1bb1007e59d9434b72765c846840c5a18e485fa625526cdbd6eb774d1018c` |
| Implementation path list | 11 paths | `8f59636f463f2643e4d670d93385bf02fabfa0c61f1bf00a402cdc93cf84105e` |
| Implementation content manifest | 1,125 bytes | `801c1888dd193bbb4b7e2eda2fc0050a4df6e2c227ffc674ca3045299cb7a35f` |
| Sorted `git ls-files --stage` index listing | repository index | `80ae18484185fb741241b3d9e2fe110e7701dab06232a5602ffdde72c01ada6a` |

All seven representations were regenerated after the Clippy failure and
compared byte-for-byte with `cmp`; all were equal. No source, test, governance,
roadmap, prompt, protected, or Git-index mutation occurred. The only runner
write is this authorized package artifact.

### Implementation Manifest

```text
57061417256f40dded448d38366611822e007385eb280277f715f5ae8ad71cf2  Cargo.lock
17fe5d1d0799bc4fafcad2eae2be42c355e525269dfd09c32038b62c48a1d0c6  Cargo.toml
23d2a8aef86ef26cd5823686c53b7fa37d2cd3bcfb774652512ef70347ed1864  crates/openwepp-assurance/Cargo.toml
7a1152d97a27f792bfded858b2db4136093c152c6353963f9a4467be8947c8fa  crates/openwepp-assurance/src/cli.rs
e123c8d226848501ea64cc31be63299b41628eecb7bda9d84ff2ebb96e8e8fc6  crates/openwepp-assurance/src/engine.rs
d5be8d6ab460e40ea13839e75d83d890a16998bae7d7c4bd6d2dec1775b343a9  crates/openwepp-assurance/src/lib.rs
c95831852492bf2811f6c5ab772619af991457a591a7f0047540bfb5b25a343e  crates/openwepp-assurance/src/v2.rs
eec626066412634d2493c26316524a8746e30ef1002c508c9a1b1a395ea49705  crates/openwepp-assurance/src/v2/confined.rs
aa3aa7eb35ec5c6dee4c09ee51c30acc00ca9473ed3b2629be5b9ff0791c0e18  crates/openwepp-assurance/src/v2/planner.rs
54137613dde7140e6a60b61bee26d717a4d9a51c44fdba2c4e826d9ef932f9ae  tests/integration/assurance_v2_planner_contract.rs
b22cc61f7b419b444e25b39b938dcf962c67d307fef629a0c2f1f28db70238e1  tests/integration/assurance_v2_source_contract.rs
```

## First HOLD Line Counts

| Rust file | Physical lines | Nonblank lines | Bytes |
| --- | ---: | ---: | ---: |
| `crates/openwepp-assurance/src/v2.rs` | 2,064 | 1,950 | 64,411 |
| `crates/openwepp-assurance/src/v2/confined.rs` | 256 | 230 | 9,088 |
| `crates/openwepp-assurance/src/v2/planner.rs` | 1,114 | 1,058 | 34,304 |
| `crates/openwepp-assurance/src/engine.rs` | 622 | 582 | 20,776 |
| `crates/openwepp-assurance/src/cli.rs` | 269 | 249 | 8,881 |
| `crates/openwepp-assurance/src/lib.rs` | 20 | 17 | 632 |
| `tests/integration/assurance_v2_source_contract.rs` | 709 | 646 | 27,454 |
| `tests/integration/assurance_v2_planner_contract.rs` | 497 | 461 | 19,135 |

Counts were identical before and after both executed gates. `v2.rs` remains a
documented warning above 2,000 lines; no touched Rust file reaches the
3,000-line closure block.

## First HOLD Protected Surfaces

| Path | Before and after SHA-256 |
| --- | --- |
| `assurance/catalog.yaml` | `cb9cb601739b64f8cb497c0999ca268859946f2ce7e57532de8c08b9ed30801f` |
| `assurance/templates/catalog.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| `assurance/generated/wepppy-usersum.yaml` | `08b21cbe20ed059eb61f01f9965d8603779501bde90a728bc7a6c138a69258eb` |
| `usersum/assurance/README.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| Sorted SHA-256 manifest of every regular file below `usersum/` | `deb9f2c646aa5eb4ad9e427f8a7cec6ad51e3f9f6b47ffe29d14b4aed4bdcb7a` |

## First HOLD Toolchain

- Cargo: `1.92.0 (344c4567c 2025-10-21)`
- rustc: `1.92.0 (ded5c06cf 2025-12-08)`, LLVM `21.1.3`
- cargo-nextest: `0.9.138 (fc97e97bb 2026-06-21)`
- cargo-deny: `0.19.6`
- cargo-llvm-cov: `0.8.7`
- cargo-crap: `0.2.2`

## First HOLD Required Follow-up

Remediate the cited test construction without changing planner semantics. That
source change invalidates this freeze, so the complete five-gate sequence must
restart from a new implementation and index snapshot. No partial result from
this held attempt may be combined with the remediated run for closure.

## Second HOLD Verdict

After the test-only Clippy remediation, formatting, strict workspace Clippy,
full Nextest, and dependency policy passed. Fresh adjudicated CRAP then failed
with one actionable touched-production row:
`crates/openwepp-assurance/src/cli.rs::execute`, CRAP 37.7074 at threshold 30.
That attempt remained on HOLD and its failed bundle is archived separately.
None of its partial results was reused for the current PASS sequence.

## Second HOLD Gate Results

| Order | Exact command | Start (UTC) | Finish (UTC) | Duration | Exit | Result |
| ---: | --- | --- | --- | ---: | ---: | --- |
| 1 | `cargo fmt --check` | `2026-07-15T20:48:11Z` | `2026-07-15T20:48:17Z` | 5.351 s | 0 | PASS |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | `2026-07-15T20:48:39Z` | `2026-07-15T20:48:47Z` | 7.904 s | 0 | PASS |
| 3 | `cargo nextest run --workspace --profile full` | `2026-07-15T20:49:12Z` | `2026-07-15T20:59:40Z` | 628.018 s | 0 | PASS |
| 4 | `cargo deny check` | `2026-07-15T21:00:04Z` | `2026-07-15T21:00:11Z` | 7.096 s | 0 | PASS |
| 5 | `bash tools/release/run_adjudicated_crap_gate.sh --base-ref 22fb7dfbafdb9e82a42afe0a5356b4c923a45232 --output-dir docs/work-packages/20260715-assure04b-v2-dependency-planner-001/artifacts/validation-evidence/adjudicated-crap` | `2026-07-15T21:00:36Z` | `2026-07-15T21:37:13Z` | 2,197.698 s | 1 | FAIL |

`cargo deny check` reported advisories, bans, licenses, and sources as OK.

## Second HOLD Full Nextest Evidence

| Field | Value |
| --- | --- |
| Run ID | `b43dee41-35dd-4e22-9680-0562f6d4c43c` |
| Profile | `full` |
| Binaries | 185 |
| Executed | 2,001 |
| Passed | 2,001 |
| Failed | 0 |
| Skipped | 3 |
| Slow | 4 |
| Nextest-reported test time | 624.176 s |
| JUnit path | `target/nextest/full/junit.xml` |
| JUnit size | 473,969 bytes |
| JUnit SHA-256 | `dcea47b04ccb34a14b033d4f18a2a687f77ddeaed89bdcec4e3648c1c4a1c4db` |

The JUnit root identifies the same run ID and records 2,001 tests, zero
failures, and zero errors.

## Second HOLD Adjudicated CRAP Evidence

Archived failed report:
[`validation-evidence/adjudicated-crap-hold-cli-execute-20260715T210036Z/adjudicated-crap-report.md`](validation-evidence/adjudicated-crap-hold-cli-execute-20260715T210036Z/adjudicated-crap-report.md)

| Field | Value |
| --- | --- |
| Acquisition | Fresh; eligible for current-source closure |
| Threshold | CRAP strictly greater than 30 is raw debt |
| Production entries | 8,686 |
| Measurement inputs | 432 |
| Raw rows above threshold | 3 |
| Adjudicated rows | 2 |
| Actionable rows | 1 |
| Touched production files | 6 |
| Actionable rows in touched files | 1 |
| Actionable rows outside touched files | 0 |
| Debt status | FAIL |
| Production-source count | 225 |
| Production-source manifest before/after/final | `ff4fb8cfb375dd478aa7158d3408e90d574e0aeb26ec93d2e63071660fe18ecb` |
| Workspace CRAP JSON SHA-256 | `e55b563823915742b1df42137eba39747568bd67bd7c7ade6e906714cd1c17fe` |
| LCOV SHA-256 | `05093316aedf4e63fb979fb63ac44243ba1a5072faf41d70a4af468c6b3bd956` |
| Adjudication registry SHA-256 | `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f` |
| Machine report SHA-256 | `3de9af31421a3b264e07d1c59f789602211fdd845d487ccb48c123edb950661d` |
| Human report SHA-256 | `af306f809c6b3576a5b4ddff03f340d060f910f9b53d2ce28f8873c37378d59c` |
| Run-status SHA-256 | `da4d336c15ed14b475dcae9e725361c0bf61c304efaa535f26ff76b82ae3fe50` |
| Evidence checksum manifest SHA-256 | `9aebe337554f8a0409a8b0780f786b2bfcc3cdf0212ddc26567d8fe18f2fe2b2` |

The canonical checksum manifest was verified with `sha256sum -c`; all 16
artifacts passed. The production-source manifests before acquisition, after
acquisition, and at final disposition are byte-identical.

### Second HOLD Touched-Production Closure Rows

The maximum measured CRAP row in each touched production Rust file was derived
from the fresh `workspace-crap.json`. No touched file contains an adjudicated
exception. `lib.rs` contains no measurable function entry.

| Status | Touched production file | Maximum function | Maximum CRAP | Threshold disposition |
| --- | --- | --- | ---: | --- |
| `M` | `crates/openwepp-assurance/src/cli.rs` | `execute` | 37.7074 | FAIL: above 30 |
| `M` | `crates/openwepp-assurance/src/engine.rs` | `create_snapshot` | 16.4319 | PASS |
| `M` | `crates/openwepp-assurance/src/lib.rs` | No function entry | N/A | PASS: no raw row above 30 |
| `M` | `crates/openwepp-assurance/src/v2.rs` | `validate_report_structure` | 26.0000 | PASS |
| `U` | `crates/openwepp-assurance/src/v2/confined.rs` | `read_regular_confined_platform` | 7.8930 | PASS |
| `U` | `crates/openwepp-assurance/src/v2/planner.rs` | `add_report_nodes` | 13.1690 | PASS |

The single actionable row is `execute` at line 61, CC 27, 75.5102% coverage,
and CRAP 37.7074. The two other raw rows are the existing exact adjudications
`CQR-LOW-L08` and `CQR-LOW-L11`; neither is in a touched file. No adjudication
was invalid or stale.

The CRAP gate's coverage collection is metric evidence, not the workflow. The
independent full Nextest run above is the terminal workflow evidence; both were
run on the same stable source snapshot.

## Second HOLD Source And Index Freeze

The Clippy remediation changed only
`tests/integration/assurance_v2_planner_contract.rs` relative to the first
attempt, from SHA-256 `54137613dde7140e6a60b61bee26d717a4d9a51c44fdba2c4e826d9ef932f9ae`
to `fbd0f21728d67e4e20df791b17a9fe3dca5e4fe1ca50bdabf0b0fde8a1900a40`.
The Git index did not change.

| Representation | Count or size | SHA-256 |
| --- | ---: | --- |
| Non-artifact status | 20 rows | `ddb1981e8710d009de7786f530128f6924606aaa7cefc97d7f4bf711e88e6d0d` |
| Non-artifact binary full-index diff | 31,085 bytes | `8c03ac0eebb86d8347d1b9dbfa4df963c2b9720f07f8d417cc2f54cb1217d93d` |
| Non-artifact changed-path list | 20 paths | `7d10d83382d1bb9a158be6eac194cb87389fe18d67045b7834b4c893a2240245` |
| Non-artifact content manifest | 2,310 bytes | `aa0b854d4d74bcdb76c84534b3bd472ec19e5a1f7e690932dc7416de3aec514d` |
| Implementation path list | 11 paths | `8f59636f463f2643e4d670d93385bf02fabfa0c61f1bf00a402cdc93cf84105e` |
| Implementation content manifest | 1,125 bytes | `14a68d6deba86eddde5b5791f2549c52bed53769e90a1838a6d8958acb3bd5cd` |
| Sorted `git ls-files --stage` index listing | repository index | `80ae18484185fb741241b3d9e2fe110e7701dab06232a5602ffdde72c01ada6a` |

All representations were regenerated after gate 5 and compared byte-for-byte
with `cmp`; all were equal. The protected files, complete `usersum/` aggregate,
and all recorded line counts also remained equal. No out-of-bounds source,
test, governance, roadmap, prompt, protected, or Git-index write occurred.

## Second HOLD Line Counts

| Rust file | Physical lines | Nonblank lines | Bytes |
| --- | ---: | ---: | ---: |
| `crates/openwepp-assurance/src/v2.rs` | 2,064 | 1,950 | 64,411 |
| `crates/openwepp-assurance/src/v2/confined.rs` | 256 | 230 | 9,088 |
| `crates/openwepp-assurance/src/v2/planner.rs` | 1,114 | 1,058 | 34,304 |
| `crates/openwepp-assurance/src/engine.rs` | 622 | 582 | 20,776 |
| `crates/openwepp-assurance/src/cli.rs` | 269 | 249 | 8,881 |
| `crates/openwepp-assurance/src/lib.rs` | 20 | 17 | 632 |
| `tests/integration/assurance_v2_source_contract.rs` | 709 | 646 | 27,454 |
| `tests/integration/assurance_v2_planner_contract.rs` | 500 | 464 | 19,201 |

## Second HOLD Protected Surfaces

| Path | Before and after SHA-256 |
| --- | --- |
| `assurance/catalog.yaml` | `cb9cb601739b64f8cb497c0999ca268859946f2ce7e57532de8c08b9ed30801f` |
| `assurance/templates/catalog.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| `assurance/generated/wepppy-usersum.yaml` | `08b21cbe20ed059eb61f01f9965d8603779501bde90a728bc7a6c138a69258eb` |
| `usersum/assurance/README.md` | `65115fe549cbee3107a120f2719b45c00ca2e63b49ebc5c6fc2d7ea350a3cb70` |
| Sorted SHA-256 manifest of every regular file below `usersum/` | `deb9f2c646aa5eb4ad9e427f8a7cec6ad51e3f9f6b47ffe29d14b4aed4bdcb7a` |

## Second HOLD Toolchain

- Cargo: `1.92.0 (344c4567c 2025-10-21)`
- rustc: `1.92.0 (ded5c06cf 2025-12-08)`, LLVM `21.1.3`
- cargo-nextest: `0.9.138 (fc97e97bb 2026-06-21)`
- cargo-deny: `0.19.6`
- cargo-llvm-cov: `0.8.7`
- cargo-crap: `0.2.2`

## Second HOLD Required Follow-up

Decompose `cli.rs::execute` and add bounded CLI branch coverage without
weakening the threshold or adjudicating ordinary hand-authored behavior. The
complete five-gate sequence must restart from a new source and index freeze.
