# implementation test evidence

Status: M-C executed-hold; no implementation tests added

Evidence mode: Ran + Static

## M-C ran

- Fresh H1-H36 CLI batch with `target/debug/openwepp-cli-hill --policy compat --legacy-sidecar-discovery`
  - PASS; 36/36 exit code `0`.
- Local owcmp H1-H36 semantic batch without comparator subagent
  - PASS execution; semantic FAIL due structural per-OFE WAT publication mismatch.
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
  - PASS execution; semantic FAIL due structural row-key/per-OFE WAT publication mismatch assigned to M-C.
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
