# openWEPP Plant/Landuse/Growth/Decomposition Architecture Fit Analysis (PL01)

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- openWEPP architecture requires typed ownership boundaries, deterministic orchestrator sequencing, and explicit canonical-symbol alias continuity.
- Management parser contract already codifies cropland-only executable policy and typed rejects for unsupported landuse/options.

Ran:
- Audited openWEPP management parser, integration tests, runtime seam adapters, symbol alias registry, and architecture docs.

## Current Fit Summary

1. Parser-contract layer fit: strong.
- Management parser accepts allowlisted datver branches (`95.7`, `98.4`, `2016.3`, `2017.1`) and enforces section/schedule closure with typed errors.
- Explicit typed reject for `landuse=2` (`UnsupportedLanduse` -> `MAN-E-004`) and explicit executable limit for perennial `mgtopt` (`1..3`).

2. Runtime seam layer fit: partial.
- Orchestrator runtime surface builders currently exist for soil, slope, and climate only.
- No management-to-runtime adapter exists for plant/landuse/growth/decomposition state surfaces.

3. Symbol continuity layer fit: partial-to-missing for PL01 domain.
- Canonical alias registry currently maps runoff/slope/soil/climate-oriented symbols.
- No alias entries exist for PL01 canonical symbols such as `lanuse`, `itype`, `imngmt`, `jdplt`, `jdharv`, `mgtopt`, `resmgt`, `vdmt`, `cancov`, `canhgt`, `lai`, `rmagt`, `rmogt`, `rtm`, `rtmass`.

## Architecture Constraints for PL01 Follow-on Implementation

| constraint | required by architecture | current state | implication |
|---|---|---|---|
| One owner per mutable runtime surface | `simulation-subsystem-kernel-architecture` | Distributed baseline ownership identified; no openWEPP runtime ownership split implemented for PL01 surfaces | Must define typed PL runtime-state crate/boundary before kernel wiring. |
| Deterministic scheduler order | architecture + baseline ordering | Baseline order (`decomp -> soil -> watbal`) established | openWEPP orchestrator phase graph must preserve this ordering contract for PL surfaces. |
| Typed seam adaptation | no silent fallback policy | No PL management runtime adapter in orchestrator | Add strict management-to-runtime projection with typed failures. |
| Canonical symbol alias continuity | architecture symbol policy | PL symbol aliases absent | Add PL alias table before exposing runtime kernel interfaces. |

## Recommended Boundary Pattern

1. Keep management parser as immutable input-state owner.
2. Add dedicated typed runtime adapter from parsed management schedule + initial scenarios to PL runtime state surfaces.
3. Split runtime ownership into explicit modules:
- landuse-management schedule state
- plant growth state
- residue/decomposition state
4. Extend canonical alias registry for full PL01 symbol bundle.
5. Add parser-to-runtime integration tests for PL surfaces before kernel implementation.

## Evidence Links

- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md:8`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md:23`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md:57`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md:62`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md:44`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md:48`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md:49`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs:9`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs:324`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs:383`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs:455`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs:559`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs:1082`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs:1279`
- `/home/workdir/openWEPP/tests/integration/infile_management_parser_contract.rs:68`
- `/home/workdir/openWEPP/tests/integration/infile_management_parser_contract.rs:238`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:514`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:785`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:1076`
- `/home/workdir/openWEPP/crates/openwepp-sim-contract/src/symbols.rs:255`
- `/home/workdir/openWEPP/crates/openwepp-sim-contract/src/symbols.rs:302`
