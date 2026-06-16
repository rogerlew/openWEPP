# CQR28 Disposition

Status: complete-with-warnings.

Disposition:

- Accepted the behavior-preserving helper extraction.
- CQR28 target `Wb11HydrologyKernel::run_percolation` is closed at CRAP
  `17.19373252009578`.
- All newly extracted helpers are CRAP `<= 30`.
- Required Rust gates passed.

Warnings carried forward:

- `cargo crap` emits 126 LCOV source-map warnings.
- Same-file out-of-scope rows above CRAP `30` remain:
  `resolve_effective_wb18_frozen_depth` and `run_plant_root_uptake`.

Next action:

- Commit and push package write set.
- After push succeeds, update `docs/work-packages/cqr-burndown-execplan.md`
  CQR28 row with package path, pushed commit SHA, branch, date, and final CRAP.
