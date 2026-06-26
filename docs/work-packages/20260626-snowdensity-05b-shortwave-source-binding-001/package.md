# SNOWDENSITY-05B Shortwave Source Binding

Status: complete.

Package type: contract/source-binding package.

Primary contract: `SC-SNOWFREEZE-001`.

Closure: COMPLETE-05B-SHORTWAVE-SOURCE-BINDING.

Objective: bind the shortwave/radiation source and transformation lineage that
future `coe_shortwave_albedo_v1` melt code must consume, without implementing
the opt-in melt path. The package decides what radiation source openWEPP owns,
how units move into `winter.hourly.rad_mj_m2_####`, and how the source remains
shared with ET and other hydrology consumers.

No production runtime code, constants, parser surfaces, output schemas, or
defaults are changed by SNOWDENSITY-05B.

## Decision Boundaries

- The canonical openWEPP acceptance point is the existing daily climate
  `rad`/`radly` field in `Ly d^-1`.
- Upstream gridded product selection and spatialization remain outside
  openWEPP ownership. Orchestration may normalize a gridded product into the
  climate `rad` field and carry provenance, but the engine must not fetch,
  select, spatialize, or tune gridded radiation products.
- The transformation path is exactly `SC-CLIMATE-001#INV-CLIMATE-013`:
  `radly -> radmj = radly * 0.04184 -> sunmap -> radcur/hr_tmp ->
  winter.hourly.rad_mj_m2_####`.
- ET and snowmelt must consume the same daily radiation authority
  (`rad`/`RA`/`radiation_ly`). A snow-only radiation scalar, fitted multiplier,
  clipping path, or unit double-conversion is invalid.
- Albedo state, albedo constants, and production opt-in melt implementation are
  deferred to SNOWDENSITY-05C and SNOWDENSITY-05D.

## Required Reading Completed

Static:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/planning/snow-frost-fidelity-strategy.md` sections 2, 4, 5, 7, and 10
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `docs/specifications/wepp-input-files/specs/climate-file.spec.md`
- `tests/AGENTS.md`
- Runtime climate source lineage in
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/03_climate.rs`,
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`,
  and `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests/climate.rs`

## Write Set

- `Cargo.toml`: registered the new integration test target.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`:
  bumped to v77 and added the 05B source/provenance authority.
- `tests/integration/snowdensity05a_melt_contract_guard.rs`: updated the
  header-version marker after the contract moved from v76 to v77.
- `tests/integration/snowdensity05b_shortwave_source_contract.rs`: added the
  contract/package guard.
- `docs/planning/snow-frost-fidelity-strategy.md`: marked 05B complete and
  identified 05C as the next gate.
- `docs/work-packages/README.md`: updated active/completed package status.
- Package artifacts under this directory.

## Evidence

Ran:

- Before amendment, `cargo test --test snowdensity05b_shortwave_source_contract`
  failed as expected because the contract was still v76 and this package was not
  scaffolded.

Static:

- `SC-SNOWFREEZE-001` v77 contains `INV-SNOWFREEZE-053`,
  `OBL-SNOWFREEZE-P-028`, `winter_shortwave_daily_radly`, and the
  SNOWDENSITY-05B Shortwave Source Binding Addendum.
- `SC-CLIMATE-001#INV-CLIMATE-013` already binds the single-conversion
  `radly -> radmj -> sunmap/radcur/hr_tmp -> winter.hourly.rad_mj_m2_####`
  lineage.
- Runtime source inspection shows openWEPP already consumes daily climate
  `rad` as Langleys/day and publishes hourly winter radiation in
  `MJ m^-2 h^-1`; no source selector or snow-only radiation scalar was added.

## Exit Criteria Disposition

- Typed source/provenance ledger exists: satisfied by
  `artifacts/source-provenance-ledger.md`.
- Unit/anti-alias evidence exists: satisfied by
  `artifacts/anti-alias-evidence.md` plus the contract guard.
- ET/snow shared authority is proven at the contract level: satisfied by
  `SC-SNOWFREEZE-001#INV-SNOWFREEZE-053`, `SC-CLIMATE-001#INV-CLIMATE-013`,
  and `SC-EVAP-001#INV-EVAP-021` references.
- Runtime implementation not claimed: deferred to 05D after 05C.
- Albedo state not claimed: deferred to 05C.

## HOLD Boundaries Checked

No HOLD was required because the required openWEPP-owned source is the existing
normalized climate seam. Upstream gridded product selection, acquisition,
spatialization, and provenance capture remain orchestration responsibilities;
they are not reimplemented in openWEPP. If a future package requires engine-side
provider selection or a separate snow radiation column, it must close `HOLD` or
amend the contract first.

## Subagent Authorization

Subagent authorization: not used. This package performed local static review
and verification only because the 05B write set is contract/test/artifact
limited and no production runtime code was changed.

## Security Impact

No external network access was used. No copyrighted PDF content was copied into
the repository. No secrets, tokens, generated credentials, or external data
products were added.
