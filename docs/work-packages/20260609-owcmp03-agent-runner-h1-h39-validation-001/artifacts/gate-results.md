# Gate Results

Evidence mode: Ran

Focused gates run from `/workdir/openWEPP`:

| Command | Result |
|---|---|
| `.venv/bin/python -m py_compile tools/owcmp/owcmp tools/owcmp/*.py` | pass |
| `cargo fmt --check` | pass |
| `cargo test --test owcmp_agent_config_contract` | pass, `2` tests |
| `cargo test --test owcmp_cli_contract` | pass, `10` tests |
| `git diff --check` | pass |
| `rg -n "tools/legacy_comparison_suite" .codex || true` | pass, no output |
| `tools/owcmp/owcmp batch h1-h39-semantic --baseline-dir /tmp/hphys0303_adr0016_1780691036/reports/hillslope/fixed_baseline_partitions --candidate-dir /tmp/hphys0300_full_20260605T155527Z/hillslope_output --candidate-year-offset 2012 --start 1 --end 1 --output-root /tmp/owcmp03_h1_smoke` | pass command execution; semantic pass count `0/1`, structural row/key failures `0` |
| `tools/owcmp/owcmp batch h1-h39-semantic --baseline-dir /tmp/hphys0303_adr0016_1780691036/reports/hillslope/fixed_baseline_partitions --candidate-dir /tmp/hphys0300_full_20260605T155527Z/hillslope_output --candidate-year-offset 2012 --output-root docs/work-packages/20260609-owcmp03-agent-runner-h1-h39-validation-001/artifacts/runner-h1-h39` | delegated runner pass; exit `0`, execution verdict `PASS`, semantic pass count `0/39`, structural row/key failures `0` |

Final focused gate chain rerun after the `.venv` trampoline update:

- Ran: `cargo fmt --check && .venv/bin/python -m py_compile tools/owcmp/owcmp tools/owcmp/*.py && cargo test --test owcmp_agent_config_contract && cargo test --test owcmp_cli_contract && git diff --check`
  Result: pass.
- Ran: command-log inspection for
  `docs/work-packages/20260609-owcmp03-agent-runner-h1-h39-validation-001/artifacts/runner-h1-h39/command-log.json`
  Result: `39` entries; first command interpreter
  `/home/workdir/openWEPP/.venv/bin/python`; H39 exit `0`.

Generated-noise cleanup:

- Ran: `find tools/owcmp -type d -name __pycache__ -print`
  Result: `tools/owcmp/__pycache__` existed after Python compilation.
- Ran: `rm -rf tools/owcmp/__pycache__ && find tools/owcmp -type d -name __pycache__ -print`
  Result: no output.

Post-review failure-path gate:

- Ran: `cargo test --test owcmp_cli_contract`
  Result: pass, `10` tests, including missing-input and command-failure artifact
  contract tests.
- Ran: `cargo test --test owcmp_agent_config_contract`
  Result: pass, `2` tests.
- Ran: `cargo fmt --check`, `.venv/bin/python -m py_compile tools/owcmp/owcmp tools/owcmp/*.py`, and `git diff --check`.
  Result: pass.
