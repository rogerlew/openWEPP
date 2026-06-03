# Unit Registry Audit

Status: completed
Evidence mode: static-and-run

Static: Initial registry coverage was audited against high-risk source seams and
WAT publication metadata.

Ran: `cargo test --test sim_contract_boundary_unit_registry` passed before final
artifact disposition.

## Covered Families

- Hydrology and WAT publication: `P`, `RM`, `Q`, `UpStrmQ`, `SubRIn`, `QOFE`,
  `Tile`, `Irr`, `Area`, and aggregate storage columns.
- ET: `Ep`, `Es`, and `Er`.
- Percolation/deep seepage: `Dp` and `Pe`.
- Climate: `prcp`, `rad`, `tmax`, `tmin`, `tdpt`, `wind`, `vwind`, `stmdur`,
  `stmstr`, `timem_####`, `mxint`, `avrint`, and `intsty_####`.
- Snow/freeze: `snow.runtime_*`, `snow.hourly.*`, and `winter.hourly.*`
  high-risk aliases.
- Soil: `dg`, `solthk`, `thetdr`, `thetfc`, `por`, `ssc`, `nsl`, and `sat`
  primary/OFE/layer templates.
- WB13 profile runtime: `wb13_profile_depth_mm`,
  `wb13_profile_porosity_cap_mm`, `wb13_profile_fc_store_mm`,
  `wb13_profile_fc_tail_mm`, and `wb13_profile_wp_store_mm`.

## Important Unit Separations

- `prcp`: runtime climate precipitation in `m`.
- `P`: WAT publication precipitation in `mm`.
- `stmdur` and `timem_####`: runtime storm timing in `s`.
- `stmstr`: runtime storm-start marker in `h`.
- `rad`: daily climate parser seam radiation in `Ly d^-1`.
- `winter.hourly.rad_mj_m2_####`: hourly winter radiation in `MJ m^-2 h^-1`.
- `snow.runtime_swe`: runtime snow-water equivalent in `m`.
- `Snow-Water`: WAT publication snow water in `mm`.

## Audit Conclusion

The registry covers the intended high-risk scope and deliberately records
non-covered surfaces as continuation work rather than pretending comprehensive
repository coverage.
