# PL Runtime Canonical Symbol Alias Requirements

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Canonical WEPP symbols remain authoritative for science-contract continuity.

Ran:
- Audited current alias registry contents and confirmed PL symbol coverage is missing and must be added in PL04.

## Required Canonical Symbol Set

| domain | canonical symbols |
|---|---|
| schedule controls | `lanuse`, `itype`, `imngmt`, `jdplt`, `jdharv`, `jdstop`, `resmgt`, `mgtopt`, `gday`, `gend`, `rw` |
| growth state | `vdmt`, `tlive`, `cancov`, `canhgt`, `lai`, `rtmass`, `rtd`, `sumgdd`, `hia`, `vdmx`, `isenes`, `ncount` |
| decomposition/residue state | `rmagt`, `rmogt`, `rilrm`, `rigrm`, `smrm`, `rtm`, `iresd`, `iroot`, `senvin`, `fenvin`, `benvin` |

## Alias Pattern Requirements

1. Scalar controls:
- direct aliases required for non-indexed controls (`lanuse`, `imngmt`, `nowcrp`-adjacent controls).

2. Slot-indexed schedule aliases:
- use deterministic index templates compatible with registry token policy, for example:
  - `{symbol}_{idx4}` for crop-slot indexed controls (`itype_{idx4}`, `jdplt_{idx4}`, `jdharv_{idx4}`).

3. Residue/root partition aliases:
- use deterministic slot indexing for residue/root pool arrays (`rmogt_{idx4}`, `rtm_{idx4}`, `iresd_{idx4}`, `iroot_{idx4}`).

4. OFE-scoped expansion:
- where runtime requires OFE-scoped PL variants, compose with existing OFE pattern style (`ofe{ofe}_{symbol}` and `ofe{ofe}_{symbol}_{idx4}`).

## Gap Statement

- Current `canonical_wepp_registry()` contains slope/soil/climate/runoff-focused mappings and does not yet include the PL canonical symbol set above.
- PL04 must extend registry entries and tests before PL05/PL06 kernel scaffolding claims alias continuity closure.

## Evidence Links

- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md:62`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md:64`
- `/home/workdir/openWEPP/crates/openwepp-sim-contract/src/symbols.rs:255`
- `/home/workdir/openWEPP/crates/openwepp-sim-contract/src/symbols.rs:302`
