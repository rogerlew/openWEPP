# Coverage Closure

Evidence label: Static/Ran.

Status: `PASS`.

ADR-0021 tier: `glue` because the target is a diagnostic CLI argument
parsing/dispatch entrypoint, not a kernel math module.

Closure rule:

- If characterization tests are added or materially changed, record line and
  region coverage status for the target, per-function 75% region-floor status or
  disposition, and obligation-to-test binding before completion.
- If only module-local CLI parse tests are added, the obligation map is
  CLI/API behavior, with no new `SC-*` obligation unless contract-derived tests
  are edited.

Current status:

- Glue-tier line coverage: `426 / 487 = 87.47433264887063%` - `PASS`.
- Delegated full-workspace LCOV/CRAP metrics completed; see
  `artifacts/gate-results.md`. The full LCOV command used
  `--ignore-run-fail`, so masked cargo-test failures are recorded separately
  and are not used as pass evidence.
- Region coverage: `NOT AVAILABLE` from LCOV; see
  `artifacts/coverage-after.md` for surrogate rationale.
- CRAP closure: `PASS`, max focused target CRAP
  `13.001854595336077`.
- Obligation-to-test binding: no new `SC-*` obligation was introduced. Tests
  bind CLI behavior to parser/dispatch oracles and preserve existing
  snowbench diagnostic-surface confinement integration tests.

Per-function floor disposition:

- Parser/decomposition helpers are covered at `97.77777777777777%` to `100%`
  by module-local tests.
- `run_export_pysnobal`, `run_physics_bulk`, and `run_coe_melt` have low
  focused line coverage only on heavy diagnostic success paths. Their
  command-specific CLI guard branches are covered here; the underlying library
  snowbench success paths are covered by existing integration tests. No
  formula, output schema, or success-path request construction changed outside
  direct field forwarding.
- `main` and `run` are binary entrypoint/env-args wrappers and are excluded from
  the per-function floor by ADR-0021 eligible-surface policy for binary
  entrypoints.
