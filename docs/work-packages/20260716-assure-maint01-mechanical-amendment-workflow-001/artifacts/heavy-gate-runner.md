# ASSURE-MAINT-01 Heavy Gate Runner

Evidence class: Ran

Current disposition: **PASS**. The final terminal-B-fixed run passed all five
required commands, including fresh adjudicated CRAP with zero actionable rows.
The coverage acquisition's ignored libtest failures are disclosed under Runs 4
and 5 and are not characterized as test passes; the current authoritative full
nextest workflow passed all 2,072 tests.

## Run 1 — Initial Closure Attempt

Disposition: **FAIL**. Three of five required commands passed. Workspace Clippy
failed one `too_many_lines` lint, and the fresh adjudicated CRAP gate found 68
actionable rows. This evidence does not close the package.

### Gate Results

| Gate | Result | Elapsed | Evidence |
| --- | --- | ---: | --- |
| `cargo fmt --check` | PASS | 2.37 s | Exit 0; no output. |
| `cargo clippy --workspace --all-targets -- -D warnings` | FAIL | 18.38 s | Exit 101; `rendered_tables_figures_references_objects_and_links_are_real_consumers` in `tests/integration/assurance_v2_assembly_contract.rs:90` has 110 lines against the 100-line `clippy::too_many_lines` ceiling. |
| `cargo nextest run --workspace --profile full` | PASS | 595.804 s | Run ID `b2c7d34c-ff80-4bd0-a091-90c28194e593`; 2,066 passed, 5 skipped, 24 slow. |
| `cargo deny check` | PASS | 6.68 s | Exit 0: advisories, bans, licenses, and sources all `ok`. |
| `bash tools/release/run_adjudicated_crap_gate.sh --base-ref 15763d7f6d5d4125333d9b7583424c714f5f5ea4` | FAIL | 36 min 22 s | Fresh acquisition, exit 1; raw 70, adjudicated 2, actionable 68. All 68 actionable rows are in touched files; zero are outside touched files. |

### Fresh CRAP Evidence

- Acquisition started: `2026-07-17T05:02:12Z`.
- Acquisition finished: `2026-07-17T05:38:34Z`.
- Production entries assessed: 9,700.
- Threshold: CRAP strictly greater than 30.
- Raw over-threshold rows: 70.
- Adjudicated rows: 2.
- Actionable rows: 68.
- Actionable rows in touched files: 68.
- Actionable rows outside touched files: 0.
- Invalid adjudications: 0.
- Production source manifest: 234 sources,
  `309b2b8688ee755b21a4df9a281c31b2bf6e1200d58fd7164762a20edd12a907`.
- CRAP JSON SHA-256:
  `306012cb4fa8827cbbeb00e6ef0628c582916b9f621b37fb1a7ab36152f28e3c`.
- LCOV SHA-256:
  `36c45dad24a11fcc31940c77717a4c6dbdaae6a31fb560af9ba79de20b84da6f`.
- Adjudication registry SHA-256:
  `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`.
- Generated raw evidence:
  `target/adjudicated-crap/adjudicated-crap-report.{json,md}`.

Touched production files reported by the gate:

- `crates/openwepp-assurance/src/cli.rs`
- `crates/openwepp-assurance/src/lib.rs`
- `crates/openwepp-assurance/src/v2.rs`
- `crates/openwepp-assurance/src/v2/amendment.rs`
- `crates/openwepp-assurance/src/v2/amendment_support.rs`
- `crates/openwepp-assurance/src/v2/assembly.rs`
- `crates/openwepp-assurance/src/v2/confined.rs`
- `crates/openwepp-assurance/src/v2/fixture.rs`
- `crates/openwepp-assurance/src/v2/identity.rs`
- `crates/openwepp-assurance/src/v2/lifecycle.rs`
- `crates/openwepp-assurance/src/v2/normalization.rs`
- `crates/openwepp-assurance/src/v2/planner.rs`
- `crates/openwepp-assurance/src/v2/publication.rs`
- `crates/openwepp-assurance/src/v2/transaction.rs`

### Runner Statement

I ran the five delegated commands exactly once in the required order. I made
no production-source changes and did not attempt to fix either failure. The
fresh CRAP acquisition verified that production source and the Git index did
not change during measurement.

## Run 2 — Post-Fix Closure Rerun

Disposition: **FAIL**. Formatting, Clippy, the full nextest suite, and dependency
policy passed. Fresh adjudicated CRAP remained nonzero at 25 actionable rows, so
the package's zero-actionable closure criterion is not met.

### Gate Results

| Gate | Result | Elapsed | Evidence |
| --- | --- | ---: | --- |
| `cargo fmt --check` | PASS | 2.28 s | Exit 0; no output. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | 6.61 s | Exit 0; `openwepp-assurance` and the workspace root completed with warnings denied. |
| `cargo nextest run --workspace --profile full` | PASS | 595.943 s | Run ID `a73f33cb-9574-43e6-a8f9-1afe0c47ae68`; 2,069 passed, 5 skipped, 24 slow. |
| `cargo deny check` | PASS | 2.71 s | Exit 0: advisories, bans, licenses, and sources all `ok`. |
| `bash tools/release/run_adjudicated_crap_gate.sh --base-ref 15763d7f6d5d4125333d9b7583424c714f5f5ea4` | FAIL | 42 min 40 s | Fresh acquisition, exit 1; raw 27, adjudicated 2, actionable 25. All 25 actionable rows are in touched files; zero are outside touched files. |

### Fresh CRAP Evidence

- Acquisition started: `2026-07-17T06:20:26Z`.
- Acquisition finished: `2026-07-17T07:03:06Z`.
- Production entries assessed: 9,700.
- Threshold: CRAP strictly greater than 30.
- Raw over-threshold rows: 27.
- Adjudicated rows: 2.
- Actionable rows: 25.
- Actionable rows in touched files: 25.
- Actionable rows outside touched files: 0.
- Invalid adjudications: 0.
- Production source manifest: 234 sources,
  `b4a557a53d9ea870b1444e1e74170ed5dd971c556a1aa3dff08806efb4216a4d`.
- CRAP JSON SHA-256:
  `f54b8884eebebbbcc9d03e7eeceb7a89f7272b95e153a6ca3c9810db6a441980`.
- LCOV SHA-256:
  `cee6610dc996b5b58913f5797021fc07c5bda05b51fb97f346ad0443d2f7a25e`.
- Adjudication registry SHA-256:
  `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`.
- Generated report JSON SHA-256:
  `5bf822949f1da8aeccd3c54e4fa95eea2486cae3c94024c07939a0d933301360`.
- Generated report Markdown SHA-256:
  `2de0cb3289821b87a1a04d2ccdeea3920c792e5f3237f3d364395faafaeadb26`.
- Generated checksum-manifest SHA-256:
  `44fb1854bd6cf16bc30694b373bdfb20fe60608de767a94fdecdf91041ba0f51`.
- Generated raw evidence:
  `target/adjudicated-crap/adjudicated-crap-report.{json,md}`.

Touched production files reported by the gate were unchanged from Run 1:

- `crates/openwepp-assurance/src/cli.rs`
- `crates/openwepp-assurance/src/lib.rs`
- `crates/openwepp-assurance/src/v2.rs`
- `crates/openwepp-assurance/src/v2/amendment.rs`
- `crates/openwepp-assurance/src/v2/amendment_support.rs`
- `crates/openwepp-assurance/src/v2/assembly.rs`
- `crates/openwepp-assurance/src/v2/confined.rs`
- `crates/openwepp-assurance/src/v2/fixture.rs`
- `crates/openwepp-assurance/src/v2/identity.rs`
- `crates/openwepp-assurance/src/v2/lifecycle.rs`
- `crates/openwepp-assurance/src/v2/normalization.rs`
- `crates/openwepp-assurance/src/v2/planner.rs`
- `crates/openwepp-assurance/src/v2/publication.rs`
- `crates/openwepp-assurance/src/v2/transaction.rs`

### Remaining Actionable Distribution

| File | Actionable rows |
| --- | ---: |
| `crates/openwepp-assurance/src/v2/amendment.rs` | 17 |
| `crates/openwepp-assurance/src/v2/amendment_support.rs` | 1 |
| `crates/openwepp-assurance/src/v2/identity.rs` | 4 |
| `crates/openwepp-assurance/src/v2/publication.rs` | 2 |
| `crates/openwepp-assurance/src/v2/transaction.rs` | 1 |

The largest remaining rows are `prepare_migration` (CRAP 506),
`prepare_layered_completion` and `calculate_roots` (272 each),
`recover_amendment` (181.603), and `amend_principal_at_generation` (121.706).
The raw/actionable reduction from Run 1 is 70/68 to 27/25; it is material but
does not satisfy the binary closure gate.

### Runner Statement

I ran the five delegated commands exactly once in the required order against
the post-fix source state. I made no production, source, or test changes and did
not attempt to fix the remaining CRAP failures. The fresh CRAP acquisition
verified that production source and the Git index did not change during
measurement.

## Run 3 — Superseded Pre-Gap Attempt

Disposition: **INTERRUPTED / NOT CLOSURE EVIDENCE**. This attempt began before
the finite implementation-contract rebind gap was closed. It was deliberately
superseded during full nextest, and no surviving process remained afterward.

| Gate | Result | Elapsed | Evidence |
| --- | --- | ---: | --- |
| `cargo fmt --check` | PASS | 2.40 s | Exit 0; no output. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | 9.56 s | Exit 0. |
| `cargo nextest run --workspace --profile full` | INTERRUPTED | Not complete | Run ID `4d8f5476-d3a3-4230-82e0-98fa53700149`; started 2,070 tests across 190 binaries with 5 skipped, but produced no final summary. |
| `cargo deny check` | NOT RUN | — | Superseded before this gate. |
| Fresh adjudicated CRAP | NOT RUN | — | Superseded before this gate. |

## Run 4 — Final Post-Refactor Closure

Disposition: **PASS**. All five required commands exited zero. Fresh
adjudicated CRAP found two raw rows, both covered by existing adjudications, and
zero actionable rows. No touched `openwepp-assurance` production file contains
an actionable function above CRAP 30.

### Gate Results

| Gate | Result | Elapsed | Evidence |
| --- | --- | ---: | --- |
| `cargo fmt --check` | PASS | 2.37 s | Exit 0; no output. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | 6.16 s | Exit 0; warnings denied across all workspace targets. |
| `cargo nextest run --workspace --profile full` | PASS | 601.083 s | Run ID `5d7b7bc1-13cb-4e9f-95ef-e2e59d64f355`; 2,071 passed, 5 skipped, 24 slow. |
| `cargo deny check` | PASS | 0.80 s | Exit 0: advisories, bans, licenses, and sources all `ok`. |
| `bash tools/release/run_adjudicated_crap_gate.sh --base-ref 15763d7f6d5d4125333d9b7583424c714f5f5ea4` | PASS | 41 min 31 s | Fresh acquisition, exit 0; raw 2, adjudicated 2, actionable 0. Touched actionable and untouched actionable counts are both zero. |

### Fresh CRAP Evidence

- Acquisition started: `2026-07-17T08:12:13Z`.
- Acquisition finished: `2026-07-17T08:53:44Z`.
- Production entries assessed: 9,700.
- Threshold: CRAP strictly greater than 30.
- Raw over-threshold rows: 2.
- Adjudicated rows: 2.
- Actionable rows: 0.
- Actionable rows in touched files: 0.
- Actionable rows outside touched files: 0.
- Invalid adjudications: 0.
- Production source manifest: 234 sources,
  `41191e288047bed7f597f775b31371574a84e128f9289c50f28d4c393be5f85f`.
- CRAP JSON SHA-256:
  `d10fab275a3e12047b24e6d50d07b2be8d9c56e5e1aaebbab3bdbedbf88c34e7`.
- LCOV SHA-256:
  `f383b9612310e8d16a1ca80199fcd6ec7b8915e11554b507ac34cd9e2772fc62`.
- Adjudication registry SHA-256:
  `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`.
- Generated report JSON SHA-256:
  `ac823a577f5024e2d22637f1275bdd16ef582bf282e49ba58e9e73527fd5688a`.
- Generated report Markdown SHA-256:
  `01b9728017d6534b5fc5a873bffc0ac31bc68ad0fa9f3c16965c6af65981325e`.
- Generated checksum-manifest SHA-256:
  `5f53ca731a70c050ef4e39dd16ba6ad70947f485c91305a115549ba986e0682d`.
- Coverage log SHA-256:
  `6b27964e5c4d562f689dd07ad0af5ea9fb8a5920913f0accc65d99495c2a8162`.
- Cargo CRAP log SHA-256:
  `7c6d8d64a85b0bfff3ba08aa4babdf60e9cc24791f88443e7bf9f273cddf1492`.
- Run-status SHA-256:
  `0e8fdcf05242b00b54d27964931540dbdcc27b62364f944e321cb60b55af208a`.

The two raw rows are the existing `CQR-LOW-L08` adjudication for
`MeteorologyError::fmt` and `CQR-LOW-L11` for
`SymbolAliasRegistryError::fmt`. Neither is in the touched assurance files.

### Touched openwepp-assurance Production Files

| Status | Path | Actionable rows |
| --- | --- | ---: |
| `M` | `crates/openwepp-assurance/src/cli.rs` | 0 |
| `M` | `crates/openwepp-assurance/src/lib.rs` | 0 |
| `M` | `crates/openwepp-assurance/src/v2.rs` | 0 |
| `U` | `crates/openwepp-assurance/src/v2/amendment.rs` | 0 |
| `U` | `crates/openwepp-assurance/src/v2/amendment_support.rs` | 0 |
| `M` | `crates/openwepp-assurance/src/v2/assembly.rs` | 0 |
| `M` | `crates/openwepp-assurance/src/v2/confined.rs` | 0 |
| `U` | `crates/openwepp-assurance/src/v2/fixture.rs` | 0 |
| `U` | `crates/openwepp-assurance/src/v2/identity.rs` | 0 |
| `M` | `crates/openwepp-assurance/src/v2/lifecycle.rs` | 0 |
| `M` | `crates/openwepp-assurance/src/v2/normalization.rs` | 0 |
| `M` | `crates/openwepp-assurance/src/v2/planner.rs` | 0 |
| `M` | `crates/openwepp-assurance/src/v2/publication.rs` | 0 |
| `U` | `crates/openwepp-assurance/src/v2/transaction.rs` | 0 |

`M` and `U` are the gate's modified and untracked touched-file classifications
against the frozen base; they are not closure dispositions.

### Coverage Validator Versus Test Workflow

The fresh CRAP script invokes `cargo llvm-cov --workspace --ignore-run-fail
--lcov`. The instrumented libtest acquisition reported four failures across two
targets and cargo test exit 101, but `--ignore-run-fail` intentionally retained
the coverage report and allowed the established CRAP procedure to adjudicate
it. Therefore the coverage acquisition is **not** represented as a test pass.

The observed instrumented failures were:

- `h2637_active_fails_closed_without_routing_coefficients`: expected fail-closed
  error but received a `HillslopeRunReport` at
  `tests/integration/laned_shadow_h2637.rs:451`.
- `h2637_active_and_shadow_are_mutually_exclusive`: expected fail-closed error
  but received a `HillslopeRunReport` at
  `tests/integration/laned_shadow_h2637.rs:486`.
- `h2637_active_and_disable_are_mutually_exclusive`: expected fail-closed error
  but received a `HillslopeRunReport` at
  `tests/integration/laned_shadow_h2637.rs:517`.
- `r3c_lane_transfer_span_projects_multilane_topology`: assertion failed at
  `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime/direct_runtime_r3c_r4b.rs:779`,
  with left `2` and right `1`.

The coverage log ended with failed targets
`-p openwepp --test laned_shadow_h2637` and
`-p openwepp-hillslope-orchestrator --lib`. The three H2637 failures are
consistent with process-global selector environment interference under
concurrent libtest execution. The authoritative process-isolated workflow is
the separately executed full nextest gate above, which passed all 2,071 tests.

### Runner Statement

I restarted the entire ladder after the rebind gap closed and ran the five
required commands in order. I made no production, source, test, schema,
assurance-data, or general-documentation changes. The fresh CRAP acquisition
verified that production source and the Git index remained stable during
measurement. All required command gates passed, and actionable CRAP is zero.

## Superseded Terminal-Fix Attempt

An immediately preceding attempt ran `cargo fmt --check` successfully in 2.34
seconds and `cargo clippy --workspace --all-targets -- -D warnings`
successfully in 1.27 seconds. It was superseded before nextest after all three
terminal-B findings were fixed. No remaining gate ran, so the partial attempt
is not closure evidence.

## Run 5 — Final Terminal-B-Fixed Closure

Disposition: **PASS**. All five required commands exited zero. The full nextest
workflow passed 2,072 tests. Fresh adjudicated CRAP found two raw rows, both
covered by existing adjudications, and zero actionable rows in touched or
untouched production files.

### Gate Results

| Gate | Result | Elapsed | Evidence |
| --- | --- | ---: | --- |
| `cargo fmt --check` | PASS | 2.36 s | Exit 0; no output. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | 0.45 s | Exit 0; warnings denied across all workspace targets. |
| `cargo nextest run --workspace --profile full` | PASS | 609.998 s | Run ID `959f93c0-a975-472d-8ee9-a8e8bb6d29e0`; 2,072 passed, 5 skipped, 24 slow. |
| `cargo deny check` | PASS | 0.99 s | Exit 0: advisories, bans, licenses, and sources all `ok`. |
| `bash tools/release/run_adjudicated_crap_gate.sh --base-ref 15763d7f6d5d4125333d9b7583424c714f5f5ea4` | PASS | 42 min 05 s | Fresh acquisition, exit 0; raw 2, adjudicated 2, actionable 0. Touched actionable and untouched actionable counts are both zero. |

### Fresh CRAP Evidence

- Acquisition started: `2026-07-17T09:49:48Z`.
- Acquisition finished: `2026-07-17T10:31:53Z`.
- Production entries assessed: 9,702.
- Threshold: CRAP strictly greater than 30.
- Raw over-threshold rows: 2.
- Adjudicated rows: 2.
- Actionable rows: 0.
- Actionable rows in touched files: 0.
- Actionable rows outside touched files: 0.
- Invalid adjudications: 0.
- Touched production files: 14, all in `crates/openwepp-assurance` and all
  with zero actionable rows; the exact paths are unchanged from Run 4.
- Production source manifest: 234 sources,
  `7227650f30319b95c279367c384bd8bed2af40840a124b37041fa19270b41784`.
- CRAP JSON SHA-256:
  `8ee407538f967bf63bbc9ca9c664cfd7fea1ff4fdb79714becb67d28ac0e04d9`.
- LCOV SHA-256:
  `04d1001c4f4e4a51cf2e21343d13e679402173729df997245b311112421b9ca9`.
- Adjudication registry SHA-256:
  `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`.
- Generated report JSON SHA-256:
  `7a6a0709b81b9897b7811ccaed5305650d89fa0a0bdd20ead9dbfa50a5197cba`.
- Generated report Markdown SHA-256:
  `1b2ecdbbd671ca4b217eec403b0be92816781cf4768c01ff4e41d2342725f30e`.
- Generated checksum-manifest SHA-256:
  `bb826d228f7d4ed2d09ee321ad0264e91cf9485de50bde891ea6a519ab37f03b`.
- Coverage log SHA-256:
  `e3a00655952bd3fc2409697a02df796211349fd5c2ebeab8afaed6598879744d`.
- Cargo CRAP log SHA-256:
  `7c6d8d64a85b0bfff3ba08aa4babdf60e9cc24791f88443e7bf9f273cddf1492`.
- Run-status SHA-256:
  `17e296fef40738cc383ba1686e865cd2ad9fb289aabd40d71d792ab77424c8b7`.

The two raw rows remain the existing `CQR-LOW-L08` adjudication for
`MeteorologyError::fmt` and `CQR-LOW-L11` for
`SymbolAliasRegistryError::fmt`. Neither is in the touched assurance files.

### Coverage Validator Versus Test Workflow

As in Run 4, the CRAP script's `cargo llvm-cov --workspace --ignore-run-fail
--lcov` acquisition retained LCOV after instrumented cargo test exited 101.
That acquisition is **not** classified as a test pass. It reported the same
three process-global H2637 selector failures at
`tests/integration/laned_shadow_h2637.rs:451`, `:486`, and `:517`, plus
`r3c_lane_transfer_span_projects_multilane_topology` at
`crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime/direct_runtime_r3c_r4b.rs:780`
with left `2` and right `1`. The failed targets were
`-p openwepp --test laned_shadow_h2637` and
`-p openwepp-hillslope-orchestrator --lib`.

The independent, process-isolated full nextest workflow is the test evidence
for this run. It passed all 2,072 tests before coverage acquisition. The CRAP
command's PASS disposition is limited to fresh coverage retention, raw-row
deduplication/adjudication, production-source stability, and zero actionable
CRAP.

### Runner Statement

I restarted the entire ladder after all terminal-B findings were fixed and ran
the five required commands in order. I made no production-source changes. The
fresh CRAP acquisition verified that production source and the Git index
remained stable during measurement. All required command gates passed, and
actionable CRAP is zero.
