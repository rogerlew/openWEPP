# Gate Results

Evidence mode: Ran.

Closure gates:

| Gate | Result |
| --- | --- |
| `cargo build -p openwepp-runner --bin openwepp-snowbench` | PASS |
| `cargo test -p openwepp-runner snowbench::tests` | PASS, 4 tests |
| `cargo test --test snowfrost_fidelity_g0_pysnobal_bridge_contract` | PASS, 2 tests |
| `.venv/bin/python -m py_compile tools/snowfreeze_observed/pysnobal_compare.py` | PASS |
| Five-site `openwepp-snowbench export-pysnobal` to `target/snowfrost_fidelity_g1` | PASS |
| G1 site-sane PySnobal harness summary | PASS, `PROCEED-SNOWFROST-FIDELITY-G1-SANE-SITE-LANES` |
| Focused Morris failed-lane reproduction | PASS as expected failed-lane evidence, `HOLD-PYSNOBAL-SANITY-FAILURE` for strict single-lane route |
| Focused Morris January 1980 window | PASS, proves window control and local window sanity |
| `cargo fmt --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| `cargo test --workspace` | PASS |
| `cargo deny check` | PASS, advisories/bans/licenses/sources ok |
| `git diff --check` | PASS |
| `! rg -n "qwet\|Qwet\|frzftp" crates` | PASS |

Notes:

- Workspace tests are long-running and include expected `running for over 60
  seconds` messages for existing integration tests; all completed successfully.
- `cargo test --workspace` was a true workflow gate, not a compile-only check.
- No production `qwet`, `Qwet`, or `frzftp` coupling was introduced.
