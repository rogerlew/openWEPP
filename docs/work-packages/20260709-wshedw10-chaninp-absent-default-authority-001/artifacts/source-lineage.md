# Source Lineage

Status: `EXECUTED-COMPLETE`
Evidence: `Static`

## Pinned Legacy Baseline

- `/workdir/wepp-forest_260430_baseline/src/wshinp.for`
- `/workdir/wepp-forest_260430_baseline/src/cchrt.inc`
- `/workdir/wepp-forest_260430_baseline/src/pmxchr.inc`

## openWEPP Surfaces

- `docs/specifications/wepp-input-files/specs/chaninp.spec.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `crates/openwepp-input-contract/src/parsers/chaninp.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`

## Initial Finding

Static: legacy `wshinp.for` absent/open-error branch initializes `cbase=0`,
sets `ichout=0` and `nchnum=0`, then continues into shared timestep
normalization. Existing openWEPP parser defaulted compatibility output is typed,
but the watershed CLI currently bypasses it with a `None` branch.

## WSHED-W10 Ratification

Static: pinned legacy directly supports the no-output default branch:
`ichout=0`, `nchnum=0`, empty selected channel list, no channel output, and
`cbase=0`. Static search did not find a fresh `dtchr` assignment outside the
`chan.inp` read path before the shared clamp block, so WSHED-W10 ratifies the
deterministic openWEPP compatibility interpretation in canonical contracts:
`dtchr_input_s=60`, `dtchr_norm_s=60`, and `ntchr=1440`.

Static: `SC-INFILE-CHANINP-001` v0.1.2 now owns the parser default branch and
`SC-SYSTEM-001` rev 88 owns watershed runtime consumption of that typed parser
state. The end-user `chaninp` specification documents the same values and marks
`CHANINP-GAP-002` as `RATIFIED-WSHED-W10`.

Static: production surfaces changed:
- Parser default branch now emits `ichout=0`.
- `WatershedNetworkFrame` accepts `ParsedBranch`, `DefaultedCompat`, and
  `OpenErrorCollapsedCompat` as runtime-ready when options are present, and
  uses `dtchr_norm_s` for routing globals.
- `openwepp-cli-watershed` always parses the configured `chan.inp` path or
  `run_dir/chan.inp` when unconfigured, surfaces parser warnings, and no longer
  supplies hidden `dtchr=3600` / `ntchr=24` fallback globals.
