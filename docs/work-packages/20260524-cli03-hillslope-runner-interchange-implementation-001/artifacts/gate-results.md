# CLI03 Gate Results

Status: completed
Evidence mode: Static + Ran

## Static
- Pre-implementation contract gate completed and recorded in:
  - `artifacts/cli03-preimplementation-contract-gate.md`
- Post-implementation gate objective: prove CLI03 implementation closure and
  repository gate compliance on current tree.

## Ran
Targeted CLI03 gates:
- `cargo test -p openwepp-hillslope-output`
  - pass (`11 passed; 0 failed`).
- `cargo test --test cli03_runner_contract_derived_tests`
  - pass (`9 passed; 0 failed`).

Additional targeted integration gate:
- `cargo test --test cli01_runner_hillslope_integration -- --test-threads=1`
  - pass (`5 passed; 0 failed`).

Python consumer-boundary gate:
- `cargo build -p openwepp-runner --bins`
  - pass.
- `PYTHONPATH=. pytest -q tests/python/test_open_wepp_runner_api.py`
  - pass (`5 passed`).

Required repository closeout gates:
- `cargo fmt --check`
  - pass.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - pass.
- `cargo test --workspace --quiet`
  - pass.
- `cargo deny check`
  - pass (`advisories ok, bans ok, licenses ok, sources ok`).
  - non-fatal warnings observed: `license-not-encountered` for unmatched
    allowlist entries in `deny.toml`.
