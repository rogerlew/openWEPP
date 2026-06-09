# Implementation Test Evidence

Evidence mode: Ran

Implementation summary:

- Added `tools/owcmp/batch_h1_h39.py`.
- Updated `tools/owcmp/owcmp` to dispatch
  `owcmp batch h1-h39-semantic`.
- Updated `tools/owcmp/README.md` and `tools/owcmp/specification.md` with the
  H1-H39 batch command and compact artifact contract.
- Updated `.codex/config.toml` and
  `.codex/agents/comparator_suite_runner.toml` so agents discover `owcmp`.
- Added `tests/integration/owcmp_agent_config_contract.rs`.
- Extended `tests/integration/owcmp_cli_contract.rs` with a two-hillslope `.dat`
  fixture batch test.

Focused tests run from `/workdir/openWEPP`:

- Ran: `.venv/bin/python -m py_compile tools/owcmp/owcmp tools/owcmp/*.py`
  Result: pass.
- Ran: `cargo fmt --check`
  Result: pass after applying `cargo fmt` to the new Rust test file.
- Ran: `cargo test --test owcmp_agent_config_contract`
  Result: pass, `2` tests passed.
- Ran: `cargo test --test owcmp_cli_contract`
  Result: pass, `10` tests passed after adding failure-path artifact tests.
- Ran: `git diff --check`
  Result: pass.
- Ran: `rg -n "tools/legacy_comparison_suite" .codex || true`
  Result: no active `.codex` references.
- Ran: `tools/owcmp/owcmp batch h1-h39-semantic --baseline-dir /tmp/hphys0303_adr0016_1780691036/reports/hillslope/fixed_baseline_partitions --candidate-dir /tmp/hphys0300_full_20260605T155527Z/hillslope_output --candidate-year-offset 2012 --start 1 --end 1 --output-root /tmp/owcmp03_h1_smoke`
  Result: command execution pass; semantic pass count `0/1`; structural
  row/key failures `0`. This proved direct CLI parquet support through
  `.venv/bin/python`.

Line-count governance:

- Ran: `wc -l tests/integration/owcmp_cli_contract.rs tests/integration/owcmp_agent_config_contract.rs tools/owcmp/batch_h1_h39.py tools/owcmp/owcmp`
  Result: touched `.rs` files are `635` and `48` lines. No `.rs` file reaches
  the `2000` line warning threshold.
