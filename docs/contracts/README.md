# Interface Contracts

Pinned cross-component contracts. Changes require coordinated updates across producers and consumers.

| Contract | Authoritative source | Notes |
|---|---|---|
| `.run` input contract | [openwepp-hillslope-runfile-contract.md](openwepp-hillslope-runfile-contract.md) | Declarative schema-versioned hillslope `.run` contract (`openwepp-hillslope-runfile-v1`) with explicit metric-only units (`unit_system = "metric"`), required core inputs, optional sidecar overrides, required `pass`/`loss` outputs, and configurable optional parquet outputs. |
| HBP (hillslope binary pass) | wepp-palimpsest `docs/contracts/hillslope-binary-pass-format.md` | openWEPP consumes and produces HBP shards per the upstream specification. Magic, header, day directory, and footer must match. |
| Parquet hillslope-trajectory schema | wepppy / wepppyo3 interchange | openWEPP emits configured optional simulation-driven parquet artifacts via existing consumer-side schemas (for example `H.wat.parquet`, `H.soil.parquet`, `H.plot.parquet`, `H.ebe.parquet`, `H.element.parquet`); output serialization/validation implementation is organized in dedicated crate `crates/openwepp-hillslope-output/` for CLI03. |
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
- hard-fail on missing/invalid required paths.

## Parquet schema source-of-truth

Schemas are owned by wepppy / wepppyo3. openWEPP organizes output contract
and serializer implementation in `crates/openwepp-hillslope-output/`, while
schema evolution remains coordinated through the wepppy repo.

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
