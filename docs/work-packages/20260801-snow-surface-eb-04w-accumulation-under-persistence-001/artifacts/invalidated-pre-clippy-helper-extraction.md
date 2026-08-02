# Invalidated Pre-Clippy-Helper Exact Cohort

The exact 16-cell cohort built from binary SHA-256
`20fe640f7e176e3624ecbe83c6f734f06784c5bbfe03a97a92d4619f74a28623`
passed its science ledgers and retained-output comparison. It was superseded
after exact-head Clippy identified a `too_many_lines` defect in
`validate_active_snow_hour`. The phase validation was extracted without changing
its order, thresholds, symbols, or arithmetic.

Because that refactor changes production source identity and the release
binary, the old receipt, cohort analysis, and neutrality result are not terminal
evidence. The raw target tree was moved intact to
`target/snow_surface_eb04w_accumulation_diagnostics_pre_clippy_helper` before a
fresh release build and cohort execution.
