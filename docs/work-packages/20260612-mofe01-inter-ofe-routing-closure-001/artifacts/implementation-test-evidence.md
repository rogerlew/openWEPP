# implementation test evidence

Status: M-G executed; erosion `qin`/sediment coupling boundary and manifest
provenance tests pass.

Evidence mode: Ran + Static

## M-G ran

M-G made no process-physics math change. It added manifest/report provenance for
the current water-transfer-only EROD14 `qin` source and contract-derived tests
that prevent treating that source as sediment-coupled closure.

- `cargo fmt --check`
  - PASS.
- Focused M-G contract coverage
  - PASS:
    `cargo test --test mofe01_inter_ofe_route_contract -- --nocapture`.
- Focused M-G manifest coverage
  - PASS:
    `cargo test --test cli03_runner_contract_derived_tests cli03_mofe03 -- --nocapture`.
- Semantic comparisons
  - NOT APPLICABLE: M-G is a contract/manifest decision increment, not a WAT
    value-acceptance increment. No comparator subagent was used.
- Final full gates
  - PASS:
    `cargo clippy --workspace --all-targets -- -D warnings`.
  - PASS:
    `cargo test --workspace`.
  - PASS:
    `cargo deny check`.
  - PASS:
    `bash tools/release/check_authority_suite_antievasion.sh`.
  - PASS:
    `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`.

Detailed evidence: `m-g-erosion-qin-coupling-decision-evidence.md`.

## M-F-REDO2 ran

M-F-REDO2 closed the remaining public `QOFE` local-depth publication gate while
keeping conservation identities on raw transfer/runoff operands.

- `cargo fmt --check`
  - PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - PASS.
- `cargo test --workspace`
  - PASS.
- `cargo deny check`
  - PASS; `advisories ok, bans ok, licenses ok, sources ok`.
- `bash tools/release/check_authority_suite_antievasion.sh`
  - PASS.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`
  - PASS.
- Focused M-F-REDO2 coverage
  - PASS:
    `cargo test --test cli03_runner_contract_derived_tests cli03_mf_multiofe_publication_emits_public_per_ofe_wat_rows -- --nocapture`.
  - PASS:
    `cargo test -p openwepp-runner --lib mofe01_me4_redo_internal_wb13_records -- --nocapture`.
  - PASS:
    `cargo test --test mofe04_publication_contract_authority_closure_contract -- --nocapture`.
  - PASS:
    `cargo test --test mofe01_per_ofe_state_contract -- --nocapture`.
- Required local H smoke without comparator subagent
  - PASS runtime execution: H1/H6/H9/H11 exited zero under
    `/tmp/openwepp_mofe01_mfredo2_current`.
  - PASS public `QOFE` local-depth normalization: candidate `QOFE/Q` ratios
    match the legacy-clean geometry ladder on H1/H6/H9/H11.
  - PASS no downstream alias: active downstream `QOFE == Q` rows are zero.
  - PASS anti-clone: active `QOFE`, public `Q`, and hydrology-vector
    all-clone active-day counts are zero.
  - PASS identities: per-element residual maxima are at or below
    `2.56e-13` mm; transfer residuals close at `0.0`.
- Local semantic comparisons without comparator subagent
  - PASS command execution for H1/H6/H9/H11 with
    `tools/owcmp/semantic_wat.py --candidate-year-offset 1999`.
  - INVESTIGATION FAIL semantic value pass: row keys align, but broader
    routed hydrology/storage/ET value families still fail. This is classified
    outside the M-F-REDO2 publication-normalization gate because the
    publication-ratio invariant matches legacy and independent conservation
    evidence closes.
- Single-OFE anchors
  - PASS:
    `/tmp/openwepp_mofe01_mfredo2_single_anchor` reports
    H8/H15/H19/H20/H22/H23/H28 byte-identical to
    `/tmp/openwepp_mofe01_mfredo_clone_single_final/output` for `.hbp`,
    `.loss.json`, `.plot.parquet`, and `.wat.parquet` (28/28 PASS).
  - NON-ACCEPTANCE:
    `/tmp/openwepp_mofe01_mfredo2_single_final` used H-sidecar runfiles and
    changed the sidecar mode; it is recorded only as setup error evidence.
- Markdown lint
  - PASS:
    `markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-WATBAL-001.md --path docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --format json`
    scanned 38 files, 0 errors, 0 warnings.
  - NON-SUBSTANTIVE:
    `wctl doc-lint` scanned 0 staged files.
- `git diff --check`
  - PASS.

Detailed evidence: `m-f-per-ofe-wat-publication-evidence.md`.

## M-F-REDO-CLONE ran

M-F-REDO-CLONE fixed the stale multi-step infiltration lineage defect that left
local runoff cloned after M-F-REDO.

- `cargo fmt --check`
  - PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - PASS.
- `cargo test --workspace`
  - PASS.
- `cargo deny check`
  - PASS; `advisories ok, bans ok, licenses ok, sources ok`.
- `bash tools/release/check_authority_suite_antievasion.sh`
  - PASS.
- `cargo test --test auth11_required_suite_obligation_guards_contract`
  - PASS.
- Focused M-F-REDO-CLONE coverage
  - PASS:
    `cargo test --test wb14_infiltration_hyetograph_kernel_contract mofe01_mfredo_clone`.
  - PASS:
    `cargo test --test wb12_reconciliation_kernel_contract`.
  - PASS:
    `cargo test -p openwepp-runner --lib mofe01_me4_redo_internal_wb13_records`.
  - PASS:
    `cargo test --test cli03_runner_contract_derived_tests cli03_mf_multiofe_publication_emits_public_per_ofe_wat_rows`.
  - PASS:
    `cargo test --test mofe04_publication_contract_authority_closure_contract`.
  - PASS:
    `cargo test --test mofe01_per_ofe_state_contract mofe01_me4_redo_current_architecture_requires_non_tautological_internal_wb13_checks`.
- Required local H smoke without comparator subagent
  - PASS runtime execution: H1/H6/H9/H11 exited zero under
    `/tmp/openwepp_mofe01_mfredo_clone_current`.
  - PASS local runoff anti-clone: all-identical active local-runoff days are
    zero for H1/H6/H9/H11.
  - PASS full-vector anti-clone: clone days are zero for H1/H6/H9/H11.
  - PASS identities: per-element residual maxima are at or below
    `2.56e-13` mm; transfer and aggregate residuals close at `0.0`.
- Local semantic comparisons without comparator subagent
  - PASS command execution for H1/H6/H9/H11 with
    `tools/owcmp/semantic_wat.py --candidate-year-offset 1999`.
  - INVESTIGATION FAIL semantic acceptance: row keys align, but value
    comparisons remain false pending M-F-REDO2 `QOFE` publication closure.
- Single-OFE anchors
  - PASS:
    `/tmp/openwepp_mofe01_mfredo_clone_single_final` reports
    H8/H15/H19/H20/H22/H23/H28 byte-identical to M-E2 outputs for `.hbp`,
    `.loss.json`, `.plot.parquet`, and `.wat.parquet` (28/28 PASS).
- `wctl doc-lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001`
  - NON-SUBSTANTIVE; command exited zero but reported 0 files validated.
- `git diff --check`
  - PASS.

Detailed evidence: `m-f-per-ofe-wat-publication-evidence.md`.

## M-F-REDO ran

M-F-REDO fixed the M-F clone and zero surface-handoff defects, then held on the
remaining public `QOFE` geometry-scaling acceptance gate.

- `cargo fmt --check`
  - PASS.
- `git diff --check`
  - PASS.
- `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --path docs/specifications/science-contracts/contracts/SC-WATBAL-001.md --path docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md --format plain`
  - PASS; 38 files validated, 0 errors, 0 warnings.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - PASS.
- `cargo test --workspace`
  - PASS.
- `cargo deny check`
  - PASS; `advisories ok, bans ok, licenses ok, sources ok`.
- `cargo build -p openwepp-runner --bin openwepp-cli-hill`
  - PASS.
- Focused/publication coverage inside the full workspace run
  - PASS:
    `cli03_mf_multiofe_publication_emits_public_per_ofe_wat_rows`.
  - PASS:
    `mofe04_publication_contract_authority_closure_contract`.
- Required local H smoke without comparator subagent
  - PASS runtime execution: H1/H6/H9/H11 exited zero under
    `/tmp/openwepp_mofe01_mfredo_final`.
  - PASS row cardinality/provenance: each smoke output has
    `day_count * contributor_ofe_count` public WAT rows.
  - PASS active surface handoff: downstream `UpStrmQ` rows are nonzero and
    active-surface residuals close at `0.0` mm.
  - PASS active lateral handoff: downstream `SubRIn` rows are nonzero and
    lateral residuals close at `0.0` mm.
  - PASS anti-clone audit: no active surface day is an all-clone day, and
    daily `Q`/`SoilWaterTotal` distinct counts reach the OFE count.
  - FAIL acceptance: candidate H1/H6/H9/H11 still report
    `max_abs_qofe_minus_q=0.0`.
- Pinned legacy-clean direct audit
  - FAIL blocker confirmed: legacy H1/H6/H9/H11 have max
    `abs(QOFE-Q)` values of `362.13991`, `177.51694`, `185.89531`, and
    `84.64425` mm.
  - Static source authority:
    `/workdir/wepp-forest_260430_baseline/src/watbal.for` writes public `Q`
    with `efflen/totlen` and public `QOFE` with `efflen/slplen`.
- Local semantic comparisons without comparator subagent
  - PASS command execution for H1/H6/H9/H11 with
    `tools/owcmp/semantic_wat.py --candidate-year-offset 1999`.
  - FAIL semantic acceptance: row keys align, but value comparisons still fail
    for public WAT families including `Q`, `QOFE`, `UpStrmQ`, and `SubRIn`.
- Single-OFE anchors
  - PASS:
    `/tmp/openwepp_mofe01_mfredo_single_final/single-ofe-anchor-cmp.tsv`
    reports H8/H15/H19/H20/H22/H23/H28 byte-identical to M-E2 outputs for
    `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet` (28/28 PASS).

Detailed evidence: `m-f-per-ofe-wat-publication-evidence.md`.

## M-F ran

M-F wired public WAT/WB13 publication to internal per-OFE records, then held on
the surface `UpStrmQ` acceptance gate.

- `cargo fmt --check`
  - PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - PASS.
- `cargo test --workspace`
  - PASS.
- `cargo deny check`
  - PASS; `advisories ok, bans ok, licenses ok, sources ok`.
- `git diff --check`
  - PASS.
- Focused/publication coverage inside the full workspace run
  - PASS:
    `cli03_mf_multiofe_publication_emits_public_per_ofe_wat_rows`.
  - PASS:
    `watershed_cli_mf_accepts_valid_per_ofe_publication_metadata`.
  - PASS:
    `mofe01_mf_current_architecture_requires_public_per_ofe_wat_publication`.
- Required local H smoke without comparator subagent
  - PASS runtime execution: H1/H6/H9/H11 exited zero under
    `/tmp/openwepp_mofe01_mf`.
  - PASS row cardinality/provenance: H1/H6/H9/H11 row counts equal
    `day_count * contributor_ofe_count`.
  - FAIL acceptance: all four smoke runs report `max_upstrmq=0.0` and zero
    downstream nonzero `UpStrmQ` rows.
  - FAIL semantic comparison: row keys align, but `UpStrmQ`, `SubRIn`, and
    `QOFE` value comparisons remain failing.

Detailed evidence: `m-f-per-ofe-wat-publication-evidence.md`.

## M-E4-REDO ran

M-E4-REDO rebuilt internal per-OFE WB13 identity validation so acceptance
evidence compares independently sourced storage and transfer operands. It did
not flip public WAT publication.

- `cargo fmt --check`
  - PASS.
- `cargo test -p openwepp-runner mofe01_me4_redo -- --nocapture`
  - PASS; 4 focused M-E4-REDO tests passed.
- `cargo test -p openwepp-runner mofe01 -- --nocapture`
  - PASS; 12 runner per-OFE tests passed.
- `cargo test --test mofe01_per_ofe_state_contract -- --nocapture`
  - PASS; all five contract-derived tests passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - PASS.
- `cargo test --workspace`
  - PASS.
- `cargo deny check`
  - PASS; `advisories ok, bans ok, licenses ok, sources ok`.
- `bash tools/release/check_authority_suite_antievasion.sh`
  - PASS.
- `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture`
  - PASS; 2 authority obligation guard tests passed.
- `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --path docs/specifications/science-contracts/contracts/SC-WATBAL-001.md --format plain`
  - PASS; 36 files validated, 0 errors, 0 warnings.
- Required local H smoke without comparator subagent
  - PASS runtime execution: H1/H6/H9/H11 exited zero under
    `/tmp/openwepp_mofe01_me4_redo`.
  - PASS manifest audit:
    `/tmp/openwepp_mofe01_me4_redo/m-e4-redo-internal-wb13-audit.json`
    reports internal record counts equal to `row_count * contributor_ofe_count`
    and nonzero-at-noise per-element residual maxima under `1.5e-13` mm.
  - PASS local comparison command execution for H1/H6/H9/H11.
  - FAIL semantic comparison as expected for unchanged aggregate WAT
    publication: each smoke surface has `semantic_pass_count=0/1`; focus
    columns have zero failures and max diff `0.0`.
- Single-OFE anchors
  - PASS: `/tmp/openwepp_mofe01_me4_redo_single_anchors/single-ofe-anchor-cmp.tsv`
    reports H8/H15/H19/H20/H22/H23/H28 byte-identical to M-E2 outputs for
    `.hbp`, `.loss.json`, `.plot.parquet`, and `.wat.parquet` (28/28 PASS).

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
