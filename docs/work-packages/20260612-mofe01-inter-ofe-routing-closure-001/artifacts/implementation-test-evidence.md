# implementation test evidence

Status: M-E4 complete for internal WB13 record increment; package active for M-E5

Evidence mode: Ran + Static

## M-E4 ran

M-E4 implemented internal per-OFE WB13 records from persisted lane state. It
did not flip public WAT publication.

- `cargo fmt --check`
  - PASS.
- `cargo test -p openwepp-runner mofe01_me4 -- --nocapture`
  - PASS; 3 focused M-E4 tests passed.
- `cargo test -p openwepp-runner mofe01 -- --nocapture`
  - PASS; 11 runner per-OFE tests passed.
- `cargo test --test mofe01_per_ofe_state_contract -- --nocapture`
  - PASS; all four contract-derived tests passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - PASS.
- `cargo test --workspace`
  - PASS.
- `cargo deny check`
  - PASS; `advisories ok, bans ok, licenses ok, sources ok`.
- `bash tools/release/check_authority_suite_antievasion.sh`
  - PASS.
- `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --format plain`
  - PASS; 35 files validated, 0 errors, 0 warnings.
- Required local H smoke without comparator subagent
  - PASS runtime execution: H1/H6/H9/H11 exited zero under
    `/tmp/openwepp_mofe01_me4_runtime_smoke`.
  - PASS manifest audit:
    `/tmp/openwepp_mofe01_me4_runtime_smoke/m-e4-internal-wb13-audit.json`
    reports internal record counts equal to `row_count * contributor_ofe_count`
    and all identity residual maxima at `0.0` mm.
  - PASS local owcmp command execution for H1/H6/H9/H11.
  - FAIL semantic comparison as expected for unchanged aggregate WAT
    publication: each smoke surface has `semantic_pass_count=0/1`; focus
    columns have zero failures and max diff `0.0`.
- Single-OFE anchors
  - PASS: H8/H15/H19/H20/H22/H23/H28 are byte-identical to M-E2 outputs for
    `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet` (28/28 pass).

Detailed evidence: `m-e4-internal-wb13-record-evidence.md`.

## M-E3 ran

M-E3 implemented persistent per-OFE dynamic state behind the sequential OFE
lane executor. It did not produce internal per-OFE WB13 records and did not
flip public WAT publication.

- `cargo fmt --check`
  - PASS.
- `cargo test -p openwepp-hillslope-orchestrator mofe01_me3 -- --nocapture`
  - PASS; 3 focused M-E3 tests passed.
- `cargo test -p openwepp-runner mofe01 -- --nocapture`
  - PASS; 8 runner per-OFE tests passed.
- `cargo test --test mofe01_per_ofe_state_contract -- --nocapture`
  - PASS; all four contract-derived tests passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - PASS.
- `cargo test --workspace`
  - PASS.
- `cargo deny check`
  - PASS; `advisories ok, bans ok, licenses ok, sources ok`.
- `bash tools/release/check_authority_suite_antievasion.sh`
  - PASS.
- `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --format plain`
  - PASS; 34 files validated, 0 errors, 0 warnings.
- Required local H smoke without comparator subagent
  - PASS runtime execution: H1/H6/H9/H11 exited zero under
    `/tmp/openwepp_mofe01_me3_runtime_h1`.
  - PASS manifest audit:
    `/tmp/openwepp_mofe01_me3_runtime_h1/m-e3-publication-audit.json`
    reports `smoke_pass=true` and `anchor_pass=true`.
  - PASS local owcmp command execution for H1/H6/H9/H11.
  - FAIL semantic comparison as expected for unchanged aggregate WAT
    publication: each smoke surface has `semantic_pass_count=0/1`; focus
    columns have zero failures and max diff `0.0`.
- Single-OFE anchors
  - PASS: H8/H15/H19/H20/H22/H23/H28 are byte-identical to M-E2 outputs for
    `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet` (28/28 pass).

Detailed evidence: `m-e3-dynamic-state-persistence-evidence.md`.

## M-E2 ran

M-E2 implemented the sequential OFE lane executor around the existing phase
graph. It did not wire that executor into the runner CLI path, persist dynamic
state, produce per-OFE WB13 records, or flip WAT publication.

- `cargo fmt --check`
  - PASS.
- `cargo test -p openwepp-hillslope-orchestrator mofe01_me2 -- --nocapture`
  - PASS; 6 focused M-E2 tests passed.
- `cargo test -p openwepp-runner mofe01_me1 -- --nocapture`
  - PASS; M-E1 runner tests remain green.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - PASS.
- `cargo test --test mofe01_per_ofe_state_contract -- --nocapture`
  - PASS; all four contract-derived tests passed.
- `cargo test -p openwepp-hillslope-orchestrator --lib writeback:: -- --nocapture`
  - PASS; existing writeback tests plus M-E2 tests passed: 10 total.
- `cargo deny check`
  - PASS; `advisories ok, bans ok, licenses ok, sources ok`.
- `bash tools/release/check_authority_suite_antievasion.sh`
  - PASS; authority suite anti-evasion checks passed.
- `cargo test --workspace`
  - PASS.
- `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --format plain`
  - PASS; 33 files validated, 0 errors, 0 warnings after final M-E2
    verification records.
- `cargo build -p openwepp-runner --bin openwepp-cli-hill`
  - PASS.
- Final local H1-H36 replay/comparison without comparator subagent
  - PASS runtime execution: `/tmp/openwepp_mofe01_me2_final/exit-codes.tsv`
    has 36/36 exit code `0`, with 36 manifests and 144 output files.
  - PASS local owcmp command execution:
    `/tmp/openwepp_mofe01_me2_final/owcmp/summary.json` reports
    `execution_verdict=PASS`.
  - FAIL semantic comparison as expected for the unchanged aggregate WAT
    publication boundary: `semantic_pass_count=0/36`,
    `structural_row_key_failures=350720`, first divergent H1 key
    `[1, 1, 2000]`, and focus columns all have zero failures and max diff
    `0.0`.
  - PASS no-publication-flip audit:
    `/tmp/openwepp_mofe01_me2_final/m-e2-publication-audit.json` reports
    36/36 matching manifests, aggregate policy unchanged, dynamic per-OFE
    flags false, and `per_ofe_record_count=0`.
  - PASS single-OFE anchor comparison:
    `/tmp/openwepp_mofe01_me2_final/single-ofe-anchor-cmp.tsv` has 28/28
    byte-identical output files for H8/H15/H19/H20/H22/H23/H28 against M-E1.

Detailed evidence: `m-e2-sequential-ofe-lane-executor-evidence.md`.

## M-E1 ran

M-E1 implemented the per-OFE data-model shadow-state layer and made the M-E0
structural red target green without flipping WAT publication.

- `cargo fmt --check`
  - PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - PASS.
- `cargo test -p openwepp-runner mofe01_me1 -- --nocapture`
  - PASS; 7 focused M-E1 unit tests passed.
- `cargo test --test mofe01_per_ofe_state_contract -- --nocapture`
  - PASS; all four contract-derived tests passed.
- `cargo test -p openwepp --test hphys0319_fixed_baseline_stmtim_observe_contract hphys0319_contract_authority_is_registered -- --nocapture`
  - PASS after removing stale exact WATBAL version pin.
- `cargo test -p openwepp --test hphys0320_stmtim_start_time_source_line_contract hphys0320_contract_authority_is_registered -- --nocapture`
  - PASS after the same version-pin repair.
- `cargo test --workspace`
  - PASS.
- `cargo deny check`
  - PASS; `advisories ok, bans ok, licenses ok, sources ok`.
- `bash tools/release/check_authority_suite_antievasion.sh`
  - PASS.
- `cargo build -p openwepp-runner --bin openwepp-cli-hill`
  - PASS.
- Fresh H1-H36 final CLI batch under `/tmp/openwepp_mofe01_me1_final`
  - PASS; 36/36 exit code `0`, 36 manifests, 144 output files.
- Local owcmp H1-H36 semantic batch without comparator subagent
  - PASS command execution.
  - FAIL semantic comparison as expected for M-E1:
    `semantic_pass_count=0/36`, `structural_row_key_failures=350720`,
    first divergent H1 key `[1, 1, 2000]`.
  - Focus columns `RM`, `Snow-Water`, `Total-Soil`, `SoilWaterTotal`, `Ep`,
    `Es`, `Dp`, `Q`, and `latqcc` all had zero failures and max diff `0.0`.
- No-publication-flip manifest audit
  - PASS; all 36 manifests preserve aggregate publication, dynamic per-OFE
    flags false, `per_ofe_record_count=0`, and static slice count equal to
    contributor count.
- Single-OFE anchor comparison
  - PASS; H8/H15/H19/H20/H22/H23/H28 byte-identical to M-C2 outputs for
    `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet`.

Detailed evidence: `m-e1-data-model-shadow-state-evidence.md`.

## M-E0 ran

M-E0 added no production implementation. It installed contract-derived tests and
then stopped at the intentional red architecture gate.

- `cargo fmt --check`
  - PASS.
- `cargo test --test mofe01_per_ofe_state_contract mofe01_me0_contract_authority_is_present -- --nocapture`
  - PASS; M-E0 authority-presence test passed.
- `cargo test --test mofe01_inter_ofe_route_contract -- --nocapture`
  - PASS; adjacent M-B authority smoke test passed after removing the stale
    fixed-date registry assertion.
- `cargo test --test mofe01_per_ofe_state_contract -- --nocapture`
  - FAIL as intended. The target ran 4 tests: one authority test passed and
    three structural red gates failed:
    `mofe01_me0_current_architecture_requires_structural_per_ofe_state_collection`,
    `mofe01_me0_current_architecture_requires_structural_transfer_payloads`,
    and
    `mofe01_me0_current_architecture_requires_publication_policy_manifest_gate`.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - PASS.
- `cargo deny check`
  - PASS; `advisories ok, bans ok, licenses ok, sources ok`.
- `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --path docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md --path docs/specifications/science-contracts/contracts/SC-WATBAL-001.md --path docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md --path docs/specifications/science-contracts/index.md --format plain`
  - PASS; final post-evidence run validated 35 files with 0 errors and
    0 warnings.

No comparator or heavy runtime-output comparison was run for M-E0. The
increment did not change runtime behavior, and the red test blocks promotion
until M-E1 adds real per-OFE dynamic state.

## M-D ran

- `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --format plain`
  - PASS; 30 files validated, 0 errors, 0 warnings.

No M-D production implementation or tests were added. The increment was
design-only and completed by producing the per-OFE state architecture artifact.
M-E0 later satisfied this boundary by adding the contract-derived red tests
before production implementation.

## M-C2 ran

- `cargo build -p openwepp-runner --bin openwepp-cli-hill`
  - PASS.
- Fresh H1-H36 CLI batch with `target/debug/openwepp-cli-hill --policy compat --legacy-sidecar-discovery`
  - PASS; 36/36 exit code `0` under `/tmp/openwepp_mofe01_mc2`.
- Direct parquet publication audit with `.venv/bin/python`
  - FAIL M-C2 red tests: all 29 multi-OFE surfaces still publish one `OFE=1`
    row/day, `UpStrmQ=0`, and `QOFE=Q`.
- Single-OFE anchor comparison to M-B
  - PASS; H8/H15/H19/H20/H22/H23/H28 byte-identical for `.hbp`,
    `.loss.json`, `.plot.parquet`, and `.wat.parquet`.
- Local owcmp H1-H36 semantic batch without comparator subagent
  - PASS command execution.
  - FAIL semantic comparison due structural per-OFE WAT row-key mismatch.
  - Ran locally under explicit operator direction because
    GPT-5.3-Codex-Spark weekly quota was exhausted.
- `cargo test --test wb11_hydrology_kernel_contract mofe01_mb -- --nocapture`
  - PASS.
- `cargo test -p openwepp-runner mofe01_mb_wb11_seed_purges_stale_daily_carryover_for_mofe_hourly_arrays -- --nocapture`
  - PASS.
- `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --format plain`
  - PASS; 28 files validated, 0 errors, 0 warnings.
- `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --path docs/work-packages/AGENTS.md --path docs/standards/kernel-work-package-preparation.md --path docs/codex_exec_plans.md --format plain`
  - PASS; 31 files validated, 0 errors, 0 warnings.

No M-C2 production implementation or tests were added. The increment is held at
the real per-OFE daily state architecture boundary.

## M-C ran

- Fresh H1-H36 CLI batch with `target/debug/openwepp-cli-hill --policy compat --legacy-sidecar-discovery`
  - PASS; 36/36 exit code `0`.
- Local owcmp H1-H36 semantic batch without comparator subagent
  - PASS command execution.
  - FAIL semantic comparison due structural per-OFE WAT publication mismatch.
  - Ran locally under explicit operator direction because
    GPT-5.3-Codex-Spark weekly quota was exhausted.
- Direct parquet publication audit
  - FAIL M-C red tests: all 29 multi-OFE surfaces still publish one `OFE=1`
    row/day, `UpStrmQ=0`, and `QOFE=Q`.
- Single-OFE anchor comparison to M-B
  - PASS; H8/H15/H19/H20/H22/H23/H28 byte-identical for `.hbp`,
    `.loss.json`, `.plot.parquet`, and `.wat.parquet`.

No M-C production implementation or tests were added. The increment is held at
the real per-OFE state boundary.

## M-B ran

- `cargo test --test mofe01_inter_ofe_route_contract --test wb11_hydrology_kernel_contract --test wb14_infiltration_hyetograph_kernel_contract mofe01_mb -- --nocapture`
  - PASS.
- `cargo test -p openwepp-runner mofe01_mb_wb11_seed_purges_stale_daily_carryover_for_mofe_hourly_arrays -- --nocapture`
  - PASS.
- `cargo test -p openwepp --test hphys0319_fixed_baseline_stmtim_observe_contract --test hphys0320_stmtim_start_time_source_line_contract`
  - PASS after updating stale `SC-WATBAL-001` contract-version assertions from 152 to 154.
- `cargo test --workspace`
  - PASS.
- Full H1-H36 CLI batch with `target/debug/openwepp-cli-hill --policy compat --legacy-sidecar-discovery`
  - PASS; 36/36 exit code `0`.
- Local owcmp H1-H36 semantic batch without comparator subagent
  - PASS command execution.
  - FAIL semantic comparison due structural row-key/per-OFE WAT publication mismatch assigned to M-C.
  - Ran locally under explicit operator direction because
    GPT-5.3-Codex-Spark weekly quota was exhausted.

## M-A ran

- `cargo build -p openwepp-runner --bin openwepp-cli-hill`
  - PASS.
- `cargo build -p openwepp-runner --bin open_wepp_runner`
  - PASS.
- Isolated H1-H36 batch with `target/debug/openwepp-cli-hill --policy compat --legacy-sidecar-discovery`
  - 7 single-OFE surfaces passed.
  - 29 multi-OFE surfaces failed before output publication.
- Legacy H1-H36 WAT parse
  - PASS; 271,808 rows parsed.

## Not run

Anti-evasion guards were not run as standalone commands because M-B did not edit external-authority suite posture, cohort fixture binding, or required-case binding. The `auth11_required_suite_obligation_guards_contract` target passed as part of `cargo test --workspace`.
