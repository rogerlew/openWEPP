# Interface Contracts

Pinned cross-component contracts. Changes require coordinated updates across producers and consumers.

| Contract | Authoritative source | Notes |
|---|---|---|
| `.run` input contract | [openwepp-hillslope-runfile-contract.md](openwepp-hillslope-runfile-contract.md) | Declarative schema-versioned hillslope `.run` contract (`openwepp-hillslope-runfile-v1`) with explicit metric-only units (`unit_system = "metric"`), required core inputs, optional sidecar overrides, required `pass`/`loss` outputs, and configurable optional parquet outputs. |
| watershed `.run` input contract | [openwepp-watershed-runfile-contract.md](openwepp-watershed-runfile-contract.md) | Declarative schema-versioned watershed `.run` contract (`openwepp-watershed-runfile-v1`) with required `pw0.*` legacy core file bindings, required `hillslopes_block` pass-shard bindings, optional sidecar overrides (`chaninp`, `tcr`), `--legacy-sidecar-discovery` parity mode, and required interchange parquet outputs (`ebe_pw0`, `chan.out`, `chanwb`, `chnwb`, `soil_pw0`, `loss_pw0.*`, `totalwatsed3`). |
| HBP (hillslope binary pass) | wepp-palimpsest `docs/contracts/hillslope-binary-pass-format.md` | openWEPP consumes and produces HBP shards per the upstream specification. Magic, header, day directory, and footer must match. |
| Parquet hillslope-trajectory schema | wepppy / wepppyo3 interchange | openWEPP emits configured optional simulation-driven parquet artifacts via existing consumer-side schemas (for example `H.wat.parquet`, `H.soil.parquet`, `H.plot.parquet`, `H.ebe.parquet`, `H.element.parquet`); CLI04 shared-boundary authority target is dedicated crate `crates/openwepp-output/` (CLI03 predecessor: `crates/openwepp-hillslope-output/`). |
| openWEPP runner boundary | [openwepp-runner-contract.md](openwepp-runner-contract.md) | openWEPP owns in-repo `open_wepp_runner` for openWEPP binaries only; legacy WEPP orchestration belongs to `wepppy/wepp_runner`; no silent fallback across engine families/contracts; CLI03 acceptance is interchange-first with crate-organized output implementation, not bootstrap legacy include-surface synthesis. |
| openWEPP binary release + sidecar | [openwepp-binary-release-contract.md](openwepp-binary-release-contract.md) | `openwepp_YYMMDD*` naming, mandatory sidecars, schema validation, and blocking release lint gate. |
| Routine interface v1 | [routine-interface-v1.md](routine-interface-v1.md) | Routine identity, lifecycle (`experimental/active/deprecated/retired`), replacement metadata, and resolver contract for routine selection. |
| WEPP soil file format | legacy WEPP / wepp-palimpsest | openWEPP parses; format pinned to existing producer compatibility. |
| WEPP management file format | legacy WEPP / wepp-palimpsest | Same. |
| WEPP climate (cligen) file format | legacy WEPP / wepp-palimpsest | Same. |
| Watershed structure file format | legacy WEPP / wepp-palimpsest | Same. |
| `.json` release manifest | wepp-palimpsest release tooling | openWEPP binaries ship with manifest sidecars per the upstream convention. |

## `.run` contract

The canonical hillslope `.run` contract is:

- schema-versioned (`openwepp-hillslope-runfile-v1`),
- metric-only (`unit_system = "metric"`),
- explicit for required core input surfaces,
- explicit for optional sidecar override surfaces (`wepp_ui`, `pmetpara`,
  `snow`, `frost`),
- explicit for required outputs (`pass` `.hbp`, `loss` `.json`),
- explicit for optional parquet output file paths
  (`wat`, `soil`, `plot`, `ebe`, `element`),
- explicit for `outputs.wat` metadata parity keys:
  - field metadata keys `units` and `description`,
  - dataset metadata keys
    `dataset_version`, `dataset_version_major`, `dataset_version_minor`,
    `schema_version`,
- hard-fail on missing/invalid required paths.

The canonical watershed `.run` contract is:

- schema-versioned (`openwepp-watershed-runfile-v1`),
- metric-only (`unit_system = "metric"`),
- explicit for required `pw0.*` core watershed inputs:
  `pw0_str`, `pw0_chn`, `pw0_imp`, `pw0_man`, `pw0_slp`, `pw0_cli`, `pw0_sol`,
- explicit for required hillslope pass bindings
  (`inputs.hillslopes_block[]` with `hillslope_id` + `pass_file`),
- explicit for optional sidecar overrides (`inputs.chaninp`, `inputs.tcr`),
- explicit for legacy discovery override mode
  (`--legacy-sidecar-discovery` ignores configured sidecar paths and probes
  run-directory legacy sidecars),
- explicit for required interchange parquet outputs:
  - `outputs.ebe_pw0` (`ebe_pw0.parquet`)
  - `outputs.chan_out` (`chan.out.parquet`)
  - `outputs.chanwb` (`chanwb.parquet`)
  - `outputs.chnwb` (`chnwb.parquet`)
  - `outputs.soil_pw0` (`soil_pw0.parquet`)
  - `outputs.totalwatsed3` (`totalwatsed3.parquet`)
  - `outputs.loss_hill` (`loss_pw0.hill.parquet`)
  - `outputs.loss_chn` (`loss_pw0.chn.parquet`)
  - `outputs.loss_out` (`loss_pw0.out.parquet`)
  - `outputs.loss_class_data` (`loss_pw0.class_data.parquet`)
  - `outputs.loss_all_years_hill` (`loss_pw0.all_years.hill.parquet`)
  - `outputs.loss_all_years_chn` (`loss_pw0.all_years.chn.parquet`)
  - `outputs.loss_all_years_out` (`loss_pw0.all_years.out.parquet`)
  - `outputs.loss_all_years_class_data` (`loss_pw0.all_years.class_data.parquet`)
- hard-fail on missing/invalid required paths.

## Parquet schema source-of-truth

Schemas are owned by wepppy / wepppyo3. openWEPP organizes output contract
and serializer implementation at shared-boundary targets
`crates/openwepp-hillslope-output/` and `crates/openwepp-watershed-output/`,
while schema evolution remains
coordinated through the wepppy repo.

CLI04 parquet dependency posture for new implementation work:
- required stack: `parquet` + `arrow-array` + `arrow-schema`;
- `arrow-schema` is a companion crate within `arrow-rs`, not an alternative;
- `arrow2` is prohibited for new implementation work in this package.

WAT authority exception for CLI04:
- default comparator baseline remains
  `/workdir/wepp-forest_260430_baseline` at
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`,
- WAT output semantics required for consumer closure (including optional
  `Interception` and `InterceptionStorage`) follow post-`wepp_260430`
  `wepp-forest`/WEPPpy lineage per stakeholder authority.

CLI03 output posture:
- required: `.run` `outputs.pass` (`.hbp`), `.run` `outputs.loss` (`.json`)
- optional parquet outputs from optional `.run` paths:
  `outputs.wat`, `outputs.soil`, `outputs.plot`, `outputs.ebe`,
  `outputs.element`

Additional output families are out of scope for this contract surface and must
be introduced through a dedicated subsystem contract/spec package.

## Failure posture

Contract mismatch is a hard error:

- unsupported runner flags for openWEPP-only runner boundaries,
- missing or invalid release sidecar,
- invalid binary name relative to release policy,
- mixed release-pair capability declarations,
- non-metric `.run` unit-system selection,
- missing required CLI03 run outputs (`outputs.pass`, `outputs.loss`).
