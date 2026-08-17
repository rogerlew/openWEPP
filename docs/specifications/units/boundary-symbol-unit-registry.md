# Boundary Symbol Unit Registry

Status: Active
Last updated: 2026-06-04
Authority: `docs/specifications/unit-governance.md`

## Purpose

The boundary-symbol unit registry is the machine-readable unit authority for
runtime and publication symbols that cross openWEPP process seams. It prevents
unit inference from symbol names, suffixes, or downstream residuals.

## Canonical Implementation

- Registry API: `crates/openwepp-sim-contract/src/units.rs`
- Contract-derived test: `tests/integration/sim_contract_boundary_unit_registry.rs`
- Gate wrapper: `tools/release/check_unit_registry.sh`

Packages that add, change, or publish dimensional boundary symbols must add a
registry row or record an explicit HOLD gap before declaring closure.

## Registry Schema

Each registry row records:

- canonical symbol,
- runtime or API boundary aliases,
- unit label,
- dimension class,
- domain class,
- producer scope,
- consumer scope,
- owning `SC-*` contract,
- owning invariant or contract anchor,
- typed boundary requirement,
- scalar-exception reason when `BoundaryValue::scalar` remains allowed,
- publication aliases when the symbol is written to interchange outputs.

The validator rejects empty required fields, duplicate canonical symbols,
duplicate alias rows, ambiguous alias-to-canonical mappings, invalid template
tokens, dimensional symbols labeled `dimensionless`, and scalar exceptions
without a reason.

## Initial HPHYS0274 Coverage

The initial registry covers high-risk surfaces needed for water-balance and
winter unit governance:

- WAT publication depth columns: `P`, `RM`, `Q`, `Ep`, `Es`, `Er`, `Dp`,
  `UpStrmQ`, `SubRIn`, `latqcc`, `Total-Soil`, `frozwt`, `Snow-Water`,
  `QOFE`, `Tile`, `Irr`, `SoilWaterTotal`, `ProfileDepth`,
  `ProfilePorosityCap`, `ProfileFCStore`, `ProfileWPStore`, and
  `Interception` / `InterceptionStorage`.
- EVAPPM PMET runtime storage-return depth: `pmet.es_storage_return_m`.
- WAT publication area column: `Area`.
- Climate runtime aliases: `prcp`, `rad`, `tmax`, `tmin`, `tdpt`, `wind`
  direction, `vwind`, `stmdur`, `stmstr`, `timem_####`, `mxint`, `avrint`,
  and `intsty_####`, including hillslope-prefixed `hs##_...` watershed aliases
  for storm timing/intensity surfaces.
- Winter hourly aliases: `winter.hourly.rad_mj_m2_####`,
  `winter.hourly.air_temp_c_####`, `winter.hourly.dewpoint_c_####`,
  `winter.hourly.wind_m_s_####`, and
  `winter.hourly.cloud_fraction_####`.
- Snow runtime aliases: `snow.runtime_swe`, `snow.runtime_depth_m`,
  `snow.runtime_density_kg_m3`, `snow.runtime_settle_day_count`, and the
  high-risk `snow.hourly.*` depth/density/melt aliases.
- Soil runtime aliases: `dg`, `solthk`, `thetdr`, `thetfc`, `por`, `ssc`,
  `nsl`, and `sat` primary and OFE/layer templates.
- WB13 profile runtime aliases: `wb13_profile_depth_mm`,
  `wb13_profile_porosity_cap_mm`, `wb13_profile_fc_store_mm`,
  `wb13_profile_fc_tail_mm`, and `wb13_profile_wp_store_mm`.
- Persistent snow-free surface-liquid custody aliases:
  `surface_liquid.liquid_kg_m2_tile`,
  `surface_liquid.capacity_kg_m2_tile`,
  `surface_liquid.amount_kg_m2_ofe_ground`,
  `surface_liquid.condensation_kg_m2_ofe_ground`,
  `surface_liquid.parcel_mass_kg_m2_basis_ofe`,
  `surface_liquid.parcel_temperature_k`,
  `surface_liquid.parcel_enthalpy_j_m2_basis_ofe`,
  `surface_liquid.ofe_area_m2`, and `surface_liquid.interval_s`. These are
  candidate-only typed runtime seams owned by `SC-SURFACELIQUID-001`; none is a
  production publication alias.

`prcp` and WAT `P` are intentionally separate rows: runtime climate
precipitation is meters at the parser seam, while the WAT publication column is
millimeters.

`stmdur` and `timem_####` are elapsed seconds at the runtime seam. `stmstr`
remains an hour-of-day storm-start marker.

## Template Aliases

The validator supports:

- `{idx4}`: exactly four digits, used for hourly or layer suffixes such as
  `winter.hourly.rad_mj_m2_0001` and `dg_0001`.
- `{ofe}`: one or more digits, used for OFE-scoped symbols such as
  `ofe2_thetfc_0001` and numeric hillslope-prefixed aliases such as
  `hs21_timem_0001`.

Other template tokens are invalid until explicitly added to the registry API
and tests.

## Mandatory Gate

Run:

```bash
tools/release/check_unit_registry.sh
```

The wrapper runs the registry contract test and is the mandatory local gate for
packages that touch boundary-symbol units. The test validates the HPHYS0274
required-alias manifest, WAT schema metadata alignment, failure modes for
missing/ambiguous units, template-token validation, and duplicate publication
alias rejection. The wrapper also runs focused clippy with warnings denied.

## HPHYS0275/HPHYS0280 Typed Boundary Remediation

HPHYS0275 migrates the first high-risk runtime producer seams from
`BoundaryValue::scalar` to typed `BoundaryValue` variants:

- Daily climate runtime: `prcp`, `rad`, `tmax`, `tmin`, `tdpt`, `vwind`,
  `stmdur`, `stmstr`, `timem_####`, `mxint`, `avrint`, and `intsty_####`.
- SIMIMPL28 winter hourly runtime: `winter.hourly.rad_mj_m2_####`,
  `winter.hourly.air_temp_c_####`,
  `winter.hourly.cloud_fraction_####`, `snow.hourly.rain_m_####`, and
  `snow.hourly.snowfall_m_####`.

HPHYS0280 continues the same typed-boundary remediation:

- Daily climate runtime `wind` direction publishes as typed degrees.
- Watershed-prefixed climate aliases for HPHYS0275 typed families preserve
  non-scalar unit labels instead of scalarizing per-hillslope climate values.
- Snow runtime state and selected hourly trace families publish typed water
  depths, density, signed Celsius temperatures, hourly radiation, wind speed,
  and unit-interval branch/fraction values where registry authority is
  already explicit.

The registry keeps unported rows as `FollowUpRequired`. `snow.hourly.rain_m`
and `snow.hourly.rain_retained_m` are split so the migrated forcing input can
be `TypedRequired` while the retained-rain trace remains follow-up.

## HOLD Gaps

The following gaps are explicit continuation work, not silent omissions:

- Full repository symbol coverage is not complete; HPHYS0274 intentionally
  covers high-risk hydrology, snow/freeze, ET, climate, soil, percolation, and
  WAT publication surfaces first.
- Typed `BoundaryValue` variants remain continuation work after HPHYS0280 for
  output publication rows, integer/count wrappers such as
  `snow.runtime_settle_day_count`, soil/WB13 runtime geometry and storage rows,
  and non-selected production paths not migrated in the first two
  typed-boundary waves.
- Named conversion-helper enforcement remains follow-up work under HPHYS0276.
- High hourly radiation physical flux guards are implemented for first-wave
  SIMIMPL28 winter hourly radiation under HPHYS0277.
- Output writers still hard-code publication metadata until HPHYS0278 aligns
  schemas with this registry.
- Contract lint coverage for all `SC-*` unit sections remains follow-up work
  under HPHYS0279.
