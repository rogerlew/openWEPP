# Iteration Log

Status: executed-held.

## Entries

- 2026-06-23: Loaded package context and R7 architecture authority. Confirmed
  default CLI/API selected compatibility before this package and production
  direct was explicit via `--direct-production-executor`.
- 2026-06-23: Implemented `HillslopeRuntimeSelectionPolicy` and
  `HillslopeRuntimeSelectionResolution`, added default-candidate activation and
  explicit compatibility rollback semantics, and exported the policy API.
- 2026-06-23: Updated `openwepp-cli-hill` to default to
  `default-candidate`, add `--direct-default-candidate`, and add
  `--compatibility-runtime`.
- 2026-06-23: Added manifest `runtime_selection` provenance and focused R7E
  tests for default-disabled, default-activated, explicit compatibility
  rollback, explicit direct, and shadow policy resolution.
- 2026-06-23: Ran `cargo test -p openwepp-runner r7e_ -- --nocapture`; passed.
- 2026-06-23: Investigated R7F. Found production direct bypasses the
  compatibility scheduler/kernel request path but still invokes
  `DirectPublicationDayInputBuilder`, which retains `HillslopeWritebackSurface`
  seed/context surfaces inside the interleaved day/OFE loop.
- 2026-06-23: Corrected the direct runtime audit by counting production direct
  day-input builder invocations as compatibility-edge invocations.
- 2026-06-23: Updated focused R7 tests so production direct manifest
  compatibility-edge count equals direct publication row count for the fixture.
- 2026-06-23: Ran `cargo fmt --check`; passed.
- 2026-06-23: Ran `cargo test -p openwepp-runner r7 -- --nocapture`; passed.
- 2026-06-23: Closed package as
  `HOLD-R7F-DIRECT-DAY-INPUT-BUILDER-COMPATIBILITY-SURFACE-HOT-EDGE`; R7G and
  R7H remain blocked behind R7F.
