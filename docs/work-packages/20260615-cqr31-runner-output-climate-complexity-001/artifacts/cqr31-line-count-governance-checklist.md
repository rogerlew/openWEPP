# CQR31 Line-Count Governance Checklist

Before:

- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`:
  `1875`
- `docs/work-packages/README.md`: `669`
- `docs/work-packages/cqr-burndown-execplan.md`: `754`

After:

- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`:
  `2095`
- `docs/work-packages/README.md`: `673`
- `docs/work-packages/cqr-burndown-execplan.md`: `754`

Line-count disposition:

- No touched non-exempt Rust file is at or above `3000` lines.
- The target file grew because the high-CRAP function was split into explicit
  private helpers and grouped value structs.
- The growth is accepted for this package because it closes the scoped CRAP
  target and removes `too_many_lines` suppressions from the WB13 row builders.
