# CLI03 Implementation And Test Evidence

Status: completed
Evidence mode: Static + Ran

## Static
Implemented CLI03 production behavior in runner/CLI code:
- `crates/openwepp-runner/src/lib.rs`
  - `.run` execution config parser now enforces:
    - schema id `openwepp-hillslope-runfile-v1`,
    - required non-empty `run_name`,
    - metric-only `unit_system = "metric"`,
    - required `[inputs]` core paths (`soil`, `management`, `slope`, `climate`) with readable-file checks,
    - required `[outputs]` paths (`pass`, `loss`) and optional parquet paths (`wat`, `soil`, `plot`, `ebe`, `element`),
    - output contract extension/path invariants via `openwepp-hillslope-output` crate.
  - non-legacy mode now uses `.run` sidecar overrides only:
    - `wepp_ui` parsed only when `.run` requests it,
    - `pmetpara` parsed only when explicitly configured.
  - legacy mode preserves sidecar discovery authority:
    - discovered legacy sidecars remain authoritative,
    - `.run` sidecar override values are ignored for runtime sidecar binding decisions.
  - output emission switched to `.run` output surfaces:
    - required `pass` (`.hbp`) and `loss` (`.json`) outputs,
    - optional parquet-path outputs written when configured,
    - required-output missing checks map to `CLIHILL-E-013`.
  - manifest output checksum assembly now delegates to dedicated output crate helper (`assemble_output_checksums`) with deterministic key ordering.
- `crates/openwepp-runner/src/bin/open_wepp_runner.rs`
  - added `--legacy-sidecar-discovery` CLI parsing + forwarding.
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
  - added `--legacy-sidecar-discovery` CLI parsing + forwarding.
- `crates/openwepp-runner/Cargo.toml`
  - runner now depends on `openwepp-hillslope-output` for output contract/wiring boundary.
- `tests/integration/cli03_runner_contract_derived_tests.rs`
  - CLI03 contract-derived suite includes pass cases for:
    - required/optional output emission + manifest coverage,
    - legacy discovery precedence behavior.

Implemented CLI03-aligned Python consumer boundary behavior:
- `open_wepp_runner/open_wepp_runner.py`
  - `make_hillslope_run` emits TOML runfiles for
    `openwepp-hillslope-runfile-v1` with metric unit declaration and required
    `pass`/`loss` outputs,
  - `run_hillslope` verifies required outputs from runfile `[outputs]`
    bindings (`pass`, `loss`),
  - legacy ASCII pass-family runfile generation is explicitly rejected
    (`OPEN_RUNNER-E-026`).
- `tests/python/test_open_wepp_runner_api.py`
  - updated to validate TOML runfile content, required `.hbp/.json` outputs,
    and legacy pass-family rejection.

## Ran
- Command:
  - `cargo check -p openwepp-runner --all-targets`
- Observed:
  - pass.

- Command:
  - `cargo test -p openwepp-hillslope-output`
- Observed:
  - pass (`11 passed; 0 failed`).

- Command:
  - `cargo test --test cli03_runner_contract_derived_tests`
- Observed:
  - pass (`9 passed; 0 failed`).

- Commands:
  - `cargo build -p openwepp-runner --bins`
  - `PYTHONPATH=. pytest -q tests/python/test_open_wepp_runner_api.py`
- Observed:
  - pass (`5 passed`).
