# CLI03 Runner Interchange Authority And Guard Map

Status: completed
Evidence mode: Static

## Authority To Implementation Map

| Authority requirement | Implementation surface | Guard behavior | Error surface |
| --- | --- | --- | --- |
| `.run` schema is `openwepp-hillslope-runfile-v1` | `crates/openwepp-runner/src/lib.rs` (`parse_runfile_execution_config`) | reject schema mismatch | `CLIHILL-E-010` |
| `unit_system` is metric-only | `crates/openwepp-runner/src/lib.rs` (`parse_runfile_execution_config`) | reject non-metric unit system | `CLIHILL-E-010` |
| required core input bindings must exist/read | `crates/openwepp-runner/src/lib.rs` (`resolve_required_runfile_path`, required input file checks) | hard-fail when missing/unreadable | `CLIHILL-E-010` |
| required outputs are `pass` (`.hbp`) and `loss` (`.json`) | `crates/openwepp-hillslope-output/src/contracts.rs`, consumed by runner | required path + extension checks | `OHOUT-E-001`/`OHOUT-E-002` mapped to `CLIHILL-E-010` |
| optional outputs are emitted only when configured and must be `.parquet` | `crates/openwepp-hillslope-output/src/contracts.rs` + `writers.rs` | reject optional extension mismatch; emit only configured paths | `OHOUT-E-002` mapped to `CLIHILL-E-010` |
| required outputs must be present after execution | `crates/openwepp-runner/src/lib.rs` post-write required output check | hard-fail if required files missing | `CLIHILL-E-013` |
| legacy sidecar discovery precedence | `crates/openwepp-runner/src/lib.rs` (`if request.legacy_sidecar_discovery { ... }`) | discovered sidecars authoritative; `.run` sidecar overrides ignored for authority | sidecar typed errors (`CLIHILL-E-007/008/009`) |
| checksum/manifest output determinism | `crates/openwepp-hillslope-output/src/manifest.rs` + runner manifest assembly | duplicate/invalid checksum entries rejected; sorted checksum map assembly | `CLIHILL-E-015/016/017` on manifest/IO failure |

## Notes
- This map covers CLI03 runtime projection authority (runner/CLI branch behavior)
  and does not replace canonical contract authority documents.
- Canonical source documents for these guards remain:
  - `docs/contracts/openwepp-hillslope-runfile-contract.md`
  - `docs/contracts/openwepp-runner-contract.md`
  - `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`
