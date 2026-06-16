# CQR30 CRAP After

Ran: `cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr30-erod13-wave1-complexity-001/artifacts/lcov_after.info`

Ran: `cargo crap --workspace --lcov docs/work-packages/20260615-cqr30-erod13-wave1-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr30-erod13-wave1-complexity-001/artifacts/crap_after.json`

After metrics for target and extracted helpers:

- `Wb11HydrologyKernel::run_erod13_wave1_core`: CRAP `8.0`,
  cyclomatic `8.0`, coverage `100.0`.
- `Wb11HydrologyKernel::erod13_process_inputs`: CRAP `29.0`,
  cyclomatic `29.0`, coverage `100.0`.
- `Wb11HydrologyKernel::erod13_event_inputs`: CRAP `23.0`,
  cyclomatic `23.0`, coverage `100.0`.
- `Wb11HydrologyKernel::erod13_derived_terms`: CRAP `7.0`,
  cyclomatic `7.0`, coverage `100.0`.
- `Wb11HydrologyKernel::erod13_deposition_fluxes`: CRAP
  `4.703439853450031`, cyclomatic `4.0`, coverage `64.70588235294117`.
- `Wb11HydrologyKernel::erod13_detachment_fluxes`: CRAP
  `4.5925925925925934`, cyclomatic `4.0`, coverage `66.66666666666666`.
- `Wb11HydrologyKernel::erod13_runoff_inputs`: CRAP
  `4.017712903358071`, cyclomatic `4.0`, coverage `89.65517241379311`.
- `Wb11HydrologyKernel::erod13_inputs`: CRAP `4.0`,
  cyclomatic `4.0`, coverage `100.0`.
- `Wb11HydrologyKernel::erod13_transport_fluxes`: CRAP `4.0`,
  cyclomatic `4.0`, coverage `100.0`.
- `Wb11HydrologyKernel::require_erod13_nonnegative_derived`: CRAP
  `3.7084548104956268`, cyclomatic `3.0`, coverage `57.14285714285714`.
- `Wb11HydrologyKernel::validate_erod13_runoff_continuity`: CRAP
  `2.2560000000000002`, cyclomatic `2.0`, coverage `60.0`.
- `Wb11HydrologyKernel::validate_erod13_dgdx_continuity`: CRAP `2.0`,
  cyclomatic `2.0`, coverage `100.0`.
- `Erod13Symbols::new`: CRAP `1.0`, cyclomatic `1.0`, coverage `100.0`.
- `Wb11HydrologyKernel::erod13_writebacks`: CRAP `1.0`,
  cyclomatic `1.0`, coverage `100.0`.

LCOV summary:

- `FNF: 14`
- `FNH: 14`
- `LF: 312`
- `LH: 285`

Warning: `cargo crap` reported `126` source files with no matching LCOV entry,
matching the before run and prior CQR package evidence.

Disposition: target and all extracted helpers are CRAP `<= 30`.
