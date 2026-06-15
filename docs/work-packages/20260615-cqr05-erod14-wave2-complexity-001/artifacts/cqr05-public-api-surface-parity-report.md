# CQR05 Public API Surface Parity Report

Evidence: Static.

Before refactor:

- `pub(crate) fn run_erod14_wave2(...)`

After refactor:

- `pub(crate) fn run_erod14_wave2(...)`

Public-surface disposition:

- No new `pub` or `pub(crate)` item was added.
- New helper structs and functions are private to
  `hydrology_phase_erod14.rs`.
- Existing caller remains
  `hydrology_phase_peak_runoff.rs`, which still invokes
  `Self::run_erod14_wave2(request, &erod13_state_updates)?`.
