# Gate Results

Status: `EXECUTED-COMPLETE`
Evidence mode: `Ran`

| Gate | Result | Evidence |
| --- | --- | --- |
| `git status --short --branch` | PASS | Initial: `## main...origin/main`; current dirty set is scoped to M-T3. |
| Dependency order: Tier 1 | PASS | Tier 1 final disposition is `EXECUTED-HOLD-APPROXIMATION-ENVELOPE`; hold limited to unratified `Re^0.45`, not M-T3 behavior. |
| Dependency order: WSHED-W7R | PASS | W7R final disposition is `EXECUTED-COMPLETE-W7R-SEDIMENT-ACTIVE-PUBLICATION-CLOSURE`. |
| Dependency order: M-T2 baseflow export | PASS | M-T2 final disposition states M-T3 is unblocked on the groundwater/baseflow export leg. |
| `SC-ROUTE-001` contract-first authority | PASS | Rev 49 added all-hourly/no-hourly inlet rule and fail-closed partial/mixed authority. |
| `SC-ROUTE-001` profile/BEI authority | PASS | Rev 50 added Binding Exposure Index; `.venv/bin/python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`: PASS, 6 rows fully consolidated. |
| `SC-ROUTE-001` unit compliance | PASS | `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`: PASS. |
| Unit registry | PASS | `bash tools/release/check_unit_registry.sh`: 21 passed. |
| Focused HBP parser tests | PASS | `cargo nextest run --test infile_hbp_parser_contract`: 25 passed. |
| Focused watershed frame tests | PASS | `cargo nextest run --test wshedw5_typed_watershed_runtime_contract`: 18 passed. |
| Production CLI HBP hourly consumer proof | PASS | `cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract`: 1 passed. |
| Orchestrator package tests | PASS | `cargo nextest run -p openwepp-watershed-orchestrator`: 9 passed. |
| W7R production supervisor fixture | PASS | `cargo nextest run -p openwepp-runner --test watershed_cli_behavior_contract wshedw7r_p102_sediment_active_fixture_publishes_nonzero_sediment_and_jobs_identity`: 1 passed, 28 skipped. |
| Release runner build | PASS | `cargo build --release -p openwepp-runner --bins`: completed in 1m23s. |
| Release CLI real path | PASS | `target/release/openwepp-cli-watershed ... --output-dir /tmp/mt3_p102_release`: exit 0; HBP and watershed outputs present. |
| `cargo fmt --check` | PASS | Ran after formatting. |
| `cargo check --workspace` | PASS | Workspace check completed. |
| Delegated heavy-gate attempt | FAIL-THEN-FIXED | `comparator_suite_runner` found clippy failures in the new test helper (`format_push_string`, `cast_precision_loss`) before full gates completed. The helper was amended and the stray root log directory was removed. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Local post-fix rerun passed. |
| `cargo nextest run --workspace --profile full` | PASS | 1479 passed, 3 skipped, 4 slow; duration 572.628 s. |
| `cargo deny check` | PASS | advisories, bans, licenses, and sources all ok. |
| `git diff --check` | PASS | No whitespace errors. |
| Scoped Markdown/doc lint | PASS | `markdown-doc lint --path docs/work-packages/20260708-laned-router-watershed-hbp-hourly-water-sediment-consumption-001 --path docs/specifications/science-contracts/contracts/SC-ROUTE-001.md --path docs/ROADMAP.md --path docs/work-packages/README.md`: 25 files validated, 0 errors, 0 warnings. |
| `.rs` line-count disposition | PASS | Touched Rust files: `direct.rs` 1780 lines, `wshedw5_typed_watershed_runtime_contract.rs` 1126 lines, new `mt3_hbp_hourly_consumer_contract.rs` 628 lines. Read-only supporting files observed: `openwepp-cli-watershed.rs` 2263 lines WARN-existing; `watershed_cli_behavior_contract.rs` 2971 lines WARN-existing, not expanded by M-T3. No touched 3000+ Rust file. |
| Source-level anti-evasion guards | NOT RUN | No external-authority suite posture, cohort fixture, or required-case binding edited. |
| Full closure loop | PASS | fmt, clippy, full nextest, deny, diff check, markdown lint, contract BEI/unit checks, and unit registry passed after review remediation. |

Binary provenance:

| Binary | Size bytes | mtime epoch | SHA-256 |
| --- | ---: | ---: | --- |
| `target/release/openwepp-cli-hill` | 10673664 | 1783577550 | `37c08e0a8038f208e50e7b4a228fe3cc63c880b5641e1c44d8cb50838518f85a` |
| `target/release/openwepp-cli-watershed` | 9137904 | 1783577555 | `13b826d601e6884ee94680a9bd995bdb736bf173de9a5075e2fbf72e90c40b32` |
