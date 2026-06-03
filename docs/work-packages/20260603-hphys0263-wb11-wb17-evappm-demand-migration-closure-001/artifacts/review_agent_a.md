# Review Agent A

Status: completed-local

Evidence mode: static

Static: Local code-focused review only. Independent sub-agent dispatch is not
claimed because the HPHYS0263 user instruction did not explicitly request
sub-agents.

## Scope

- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/03_climate.rs`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`

## Findings

- PASS: PMET branch selection follows the legacy contract: `iflget == 1`
  retains Priestley-Taylor `evap`; `iflget != 1` selects migrated EVAPPM PMET
  demand.
- PASS: The migrated demand path publishes explicit diagnostics instead of
  hiding branch behavior behind aggregate `Ep`.
- PASS: Required runtime scalars fail through WB11 seed errors rather than
  silent defaults.
- PASS: Runfile-sidecar default discovery now finds `pmetpara.txt` when the
  run file does not explicitly name it.
- PASS: Climate and management projection additions are minimal and necessary
  for migrated PMET inputs.

## Required Follow-Up

- Migrate pinned `evappm.for:391-454` before claiming full EVAPPM process
  closure.
