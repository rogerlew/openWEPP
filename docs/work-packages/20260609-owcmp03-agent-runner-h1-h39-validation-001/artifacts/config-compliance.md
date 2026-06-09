# Config Compliance

Evidence mode: Ran

Checks to record:

- Static: `.codex/config.toml` registers `comparator_suite_runner` with
  `config_file = "agents/comparator_suite_runner.toml"`.
- Static: `.codex/agents/comparator_suite_runner.toml` uses model
  `gpt-5.3-codex-spark`.
- Static: active runner examples name:
  - `tools/owcmp/owcmp wat semantic ...`
  - `tools/owcmp/owcmp pl14s run ...`
  - `tools/owcmp/owcmp summarize --input <report.json> --output-root <dir>`
  - `tools/owcmp/owcmp batch h1-h39-semantic ...`
- Static: active runner examples do not name `tools/legacy_comparison_suite`.
- Static: runner output contract requires compact metrics and log paths.
- Ran: `cargo test --test owcmp_agent_config_contract`
  Result: pass, `2` tests passed.
- Ran: `rg -n "tools/legacy_comparison_suite" .codex || true`
  Result: no output.
