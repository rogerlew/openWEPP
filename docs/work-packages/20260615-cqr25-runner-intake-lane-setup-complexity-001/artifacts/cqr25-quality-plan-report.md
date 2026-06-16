# CQR25 Quality Plan Report

Static: scoped quality target is CRAP/cyclomatic-complexity burn-down for the
current target function in
`crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`.

Static: protected boundaries are public API, manifest schema, sidecar behavior,
parser compatibility, runtime symbols, output schemas, units, formulas, typed
guards, and science-contract behavior.

Ran: baseline metric target was live-ranked as
`execute_hillslope_run` in
`crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
with CRAP `305.483748671`.

Ran: final metric target is the same public function with CRAP
`12.4198250729`. All target-file rows are `<= 30`.

Static: package is complete-with-warnings only because `cargo crap` reports
workspace LCOV source-map warnings for test/support sources that are outside
the CQR25 target closure.
