# Progress

Static: package scaffolded for row #4 runtime input CQR execution.

Ran:

- `cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path lcov.info && cargo crap --workspace --lcov lcov.info --min 0 --format json --output /tmp/openwepp-crap-row4-before.json`

Result:

- Coverage/test phase passed.
- LCOV written to `lcov.info`.
- CRAP JSON written to `/tmp/openwepp-crap-row4-before.json`.
- `cargo crap` warned about 132 test/artifact files with no LCOV entry; row #4
  extraction uses production runtime-input source files only.

## Work Log

- Row #4 CRAP-before extraction found 24 unique production offender entries
  above 30. `cargo crap` reports each row twice in this workspace build, giving
  the execplan's row count of 48.
- Added row #4 typed assertions for all `HillslopeRuntimeInputError` stable
  codes/display branches, annual-extension variant naming, perennial grazing
  cycle projection success/fail-closed paths, SIMIMPL28 hourly winter forcing
  suppression/diagnostic/breakpoint paths, sunmap domain guards, and legacy plus
  Harder-Pomeroy hourly phase partition branches.
- Split SIMIMPL28 sunmap slope-radiation wrapping and winter precipitation
  window normalization into private helpers to reduce production CRAP without
  changing physics authority or public behavior.
- Final CRAP-after extraction found 0 row #4 entries above CRAP 30.
- H2637 protected outputs remained byte-identical against the retained baseline.
