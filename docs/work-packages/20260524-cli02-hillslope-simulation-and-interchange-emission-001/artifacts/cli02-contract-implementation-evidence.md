# CLI02 Contract Implementation Evidence

Status: complete
Evidence mode: Static

## Static
- Added canonical schema-versioned hillslope `.run` contract:
  - `docs/contracts/openwepp-hillslope-runfile-contract.md`
  - schema id `openwepp-hillslope-runfile-v1`.
- Ratified explicit metric-only run contract posture:
  - required top-level `unit_system = "metric"`.
- Ratified simplified `[outputs]` contract shape:
  - required `pass` (`.hbp`) and `loss` (`.json`),
  - optional parquet outputs `wat`, `soil`, `plot`, `ebe`, `element`.
- Ratified sidecar semantics and precedence:
  - `snow`/`frost` are override parameters (not routine toggles),
  - `--legacy-sidecar-discovery` discovers optional sidecars from run dir,
  - discovered legacy sidecars are authoritative in legacy mode,
  - `.run` sidecar override keys are ignored in legacy mode.
- Ratified runner/CLI authority updates:
  - `docs/contracts/openwepp-runner-contract.md`
  - `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
  - `docs/specifications/subsystems/runner/README.md`
  - `docs/contracts/README.md`.
- Ratified output organization requirement for CLI03 implementation:
  - output serialization/validation moves to dedicated crate
    `crates/openwepp-hillslope-output/` with crate-owned contracts and tests.
- Ratified manifest-path policy:
  - run manifest remains required,
  - manifest path is launcher-managed and is not a `.run` `[outputs]` key.
- Ratified legacy scope separation:
  - `open_wepp_runner` is openWEPP-only,
  - legacy WEPP orchestration remains in `wepppy/wepp_runner`.

## Ran
- None for CLI02 planning scope (docs/governance only).
