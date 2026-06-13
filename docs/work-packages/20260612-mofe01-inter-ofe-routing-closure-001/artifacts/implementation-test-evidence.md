# implementation test evidence

Status: M-E0 executed-hold; contract tests added

Evidence mode: Ran + Static

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
