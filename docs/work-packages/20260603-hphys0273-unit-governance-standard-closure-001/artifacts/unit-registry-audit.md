# Unit Registry Audit

Status: completed
Evidence mode: static

Static: HPHYS0273 did not implement the machine-readable registry. It made the
registry mandatory and assigned first implementation to HPHYS0274.

## Registry Status

- No repository-wide machine-readable boundary-symbol unit registry exists yet.
- Existing unit authority is distributed across `SC-*` Variables/Units tables,
  runtime symbol names, `BoundaryValue` labels, unit wrapper type names, and
  output writer metadata.
- `docs/specifications/unit-governance.md:63` through
  `docs/specifications/unit-governance.md:81` define the required registry
  fields and `HOLD` posture until registry gaps are explicit.

## Required HPHYS0274 Seed Scope

HPHYS0274 should start with high-risk hydrology/climate/snow symbols:

- `P`, `RM`, `Q`, `Ep`, `Es`, `Er`, `Dp`, `Pe`
- `Total-Soil`, `SoilWaterTotal`, `Snow-Water`
- `radly`, `radmj`, `winter.hourly.rad_mj_m2_####`
- `dg_####`, `thetfc_####`, `thetdr_####`, `st_####`, `theta_####`
- watershed flow/volume symbols currently published with `m^3` or `m^3/s`
  metadata.

Ran: not-run; this is a static registry gap audit.
