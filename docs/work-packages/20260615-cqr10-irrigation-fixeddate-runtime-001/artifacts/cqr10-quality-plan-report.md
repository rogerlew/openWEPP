# CQR10 Quality Plan Report

Static: scoped quality target is CRAP/cyclomatic-complexity burn-down for the
current target function in
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`.

Static: protected boundaries are public API, fixed-date irrigation runtime
symbols, parser compatibility, typed guards, error IDs, field names, allowed
strings, units, furrow duration and volume formulas, and kernel-facing
projection behavior.

Status: complete-with-warnings.

Static: baseline target identity:
`seed_hillslope_runtime_surface_from_irrigation_fixeddate`, line 342,
CC `38.0`, coverage `0.0%`, CRAP `1482.0`.

Static: after target identity:
`seed_hillslope_runtime_surface_from_irrigation_fixeddate`, line 341,
CC `4.0`, coverage `100.0%`, CRAP `4.0`.

Static: decomposition strategy:

- freeze behavior with focused fixed-date runtime projection tests before
  production refactor;
- extract validation, header, event, schedule, sprinkler, furrow, and
  next-record logic into private helpers;
- preserve public entrypoints and all protected symbols, guards, field names,
  thresholds, event order, and arithmetic expression order;
- stop at the scoped fixed-date target and leave depletion/frost rows to their
  own CQR packages.

Ran: raw before/after metric artifacts were captured in `lcov_before.info`,
`crap_before.json`, `lcov_after.info`, and `crap_after.json`.

Closure:

- PASS: target CRAP is `4.0`, below `30`.
- PASS: every newly extracted fixed-date helper is below CRAP `15`.
- PASS: target-file coverage improved from `194/686` to `423/747` lines.
- WARN: target-file coverage remains below science-tier closure threshold.
- WARN: pre-existing depletion row remains CRAP `1122.0` and out of scope.
