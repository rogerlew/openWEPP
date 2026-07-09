# Target Selection

Ran: initial CQR nightly measurement completed before package scaffolding.

Commands:

- `cargo llvm-cov clean --workspace` exited `0`.
- `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-nightly.lcov` exited `0`.
- `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-crap.json` exited `0`.
- Follow-up baseline check `cargo nextest run -p openwepp --test laned_shadow_h2637` exited `0` with `8 passed, 2 skipped`.

Artifacts:

- `/tmp/openwepp-cqr-nightly.lcov` (`4036104` bytes).
- `/tmp/openwepp-cqr-nightly-crap.json` (`2688894` bytes).
- `/tmp/openwepp-cqr-nightly-module-rank.tsv` (`2262` bytes).
- Command logs were written under `/home/workdir/openWEPP/artifacts/`.

Notes:

- The llvm-cov run recorded `error: 1 target failed: -p openwepp --test laned_shadow_h2637`
  because the required command uses `--ignore-run-fail`; a focused nextest rerun
  of the same target passed. The target-selection measurement is accepted as
  usable, and the focused rerun is retained as baseline-context evidence.
- `cargo crap` reported warning-only missing LCOV entries for 149 source files.

Selection method:

- Eligible targets are production Rust source files under `crates/`.
- Test files, fixtures, generated output, `target/`, and docs-only paths are
  excluded.
- File-level modules are used because each nightly package has one concrete
  `target_module_path`.
- Ranking uses total excess CRAP, then function count above `30`, then maximum
  CRAP, after de-duplicating identical cargo-crap rows.

Selected 10 production source modules:

| Rank | Target module path | Total excess | Functions above 30 | Max CRAP |
|---:|---|---:|---:|---:|
| 1 | `crates/openwepp-kernel-contract/src/lib_mod/core_types/01_typed_symbol_surfaces.rs` | 2915.6690906368485 | 3 | 2833.422607238448 |
| 2 | `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/chaninp.rs` | 1593.634505414379 | 18 | 306.0 |
| 3 | `crates/openwepp-runner/src/bin/openwepp-snowbench.rs` | 1176.0 | 2 | 930.0 |
| 4 | `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/02_ws20_segment_routing.rs` | 892.0 | 10 | 306.0 |
| 5 | `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/helpers.rs` | 525.8809022887735 | 2 | 547.2389753179175 |
| 6 | `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs` | 504.9508413968741 | 7 | 296.51689035535117 |
| 7 | `crates/openwepp-input-contract/src/parsers/management.rs` | 482.1830023762919 | 6 | 203.62194460172833 |
| 8 | `crates/openwepp-runner/src/errors.rs` | 402.76867779158215 | 6 | 192.89940656693898 |
| 9 | `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/01_ws22_ws23_ws26_detachment.rs` | 386.0 | 4 | 272.0 |
| 10 | `crates/openwepp-runner/src/hillslope/laned_shadow.rs` | 374.0 | 3 | 210.0 |

Exclusions before top 10 finalization:

| Excluded path | Reason |
|---|---|
| `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` | Non-production test file under `crates/*/tests`. |

This package row:

| Target module path | Total excess | Functions above 30 | Max CRAP |
|---|---:|---:|---:|
| `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/chaninp.rs` | 1593.634505414379 | 18 | 306.0 |
