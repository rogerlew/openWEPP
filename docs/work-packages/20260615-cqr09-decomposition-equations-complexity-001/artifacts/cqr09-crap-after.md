# CQR09 CRAP After

Ran: `cargo crap --workspace --lcov
docs/work-packages/20260615-cqr09-decomposition-equations-complexity-001/artifacts/lcov_after.info
--min 0 --format json --output
docs/work-packages/20260615-cqr09-decomposition-equations-complexity-001/artifacts/crap_after.json`
exited `0`.

Static: scoped target after refactor:

- `build_annual_decomposition_control`: line `741`, CC `9.0`,
  coverage `86.95652173913044`, CRAP `9.179748500041095`.

Static: newly extracted helpers after refactor:

- `resolve_annual_decomposition_action`: CRAP `8.02332361516035`.
- `require_annual_decomposition_control_inputs`: CRAP
  `11.109663992480952`.
- `annual_herbicide_decomposition_action`: CRAP `13.183513157488953`.
- `annual_burn_decomposition_action`: CRAP `9.209444331365765`.
- `annual_silage_decomposition_action`: CRAP `13.619043003599419`.
- `annual_cut_decomposition_action`: CRAP `11.921806669096211`.
- `annual_remove_decomposition_action`: CRAP `12.332361516034986`.
- `annual_noop_decomposition_action`: CRAP `10.489665965864438`.

Static: scoped closure threshold is satisfied because the target function and
all newly extracted helpers are `<= 30`.

Static: out-of-scope rows still above `30` after this package:

- `build_perennial_decomposition_control`: line `1318`, CRAP
  `174.48063613125004`.
- `compute_equation_decomposition_seed_surface`: line `12`, CRAP
  `46.796993926087495`.
