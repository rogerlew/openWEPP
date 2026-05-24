# CLI02 Runner/Interchange Authority and Guard Map

Status: complete
Evidence mode: Static

## Static

| Surface | Authority | Guard / Hard-Fail Requirement |
|---|---|---|
| runner ownership boundary | `docs/contracts/openwepp-runner-contract.md` | `open_wepp_runner` is openWEPP-only; legacy WEPP orchestration belongs to `wepppy/wepp_runner`; no cross-family fallback |
| `.run` schema contract | `docs/contracts/openwepp-hillslope-runfile-contract.md` | required schema id `openwepp-hillslope-runfile-v1`; invalid schema/version is hard-fail |
| metric-only discoverability | `docs/contracts/openwepp-hillslope-runfile-contract.md` + `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md` | `unit_system` must be `metric`; non-metric selection is hard-fail |
| required core inputs | `docs/contracts/openwepp-hillslope-runfile-contract.md` | required `[inputs]` keys (`soil`, `management`, `slope`, `climate`); unresolved required paths hard-fail |
| default sidecar controls | `docs/contracts/openwepp-hillslope-runfile-contract.md` | non-legacy mode accepts optional `.run` sidecar controls (`wepp_ui`, `pmetpara`, `snow`, `frost`) |
| legacy sidecar discovery precedence | `docs/contracts/openwepp-hillslope-runfile-contract.md` + `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md` | in `--legacy-sidecar-discovery`, discovered `snow.txt`, `frost.txt`, `wepp_ui.txt`, `pmetpara.txt` are authoritative; `.run` sidecar overrides are ignored |
| snow/frost execution semantics | `docs/contracts/openwepp-hillslope-runfile-contract.md` | presence of `snow.txt`/`frost.txt` does not toggle routine execution; they are overrides only |
| required outputs | `docs/contracts/openwepp-hillslope-runfile-contract.md` + `docs/contracts/openwepp-runner-contract.md` | `outputs.pass` (`.hbp`) and `outputs.loss` (`.json`) are required; missing required outputs hard-fail (`CLIHILL-E-013`, `OPEN_RUNNER-E-018`) |
| optional outputs | `docs/contracts/openwepp-hillslope-runfile-contract.md` | optional parquet outputs: `outputs.wat`, `outputs.soil`, `outputs.plot`, `outputs.ebe`, `outputs.element` |
| manifest path policy | `docs/contracts/openwepp-hillslope-runfile-contract.md` + `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md` | run manifest emission remains required; manifest path is launcher-managed, not `.run` output configuration |
| output organization boundary | `docs/contracts/openwepp-runner-contract.md` + `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md` | output serialization/validation logic must be crate-organized in `crates/openwepp-hillslope-output/` with crate-owned tests |

## Ran
- None for CLI02 planning scope (docs/governance only).
