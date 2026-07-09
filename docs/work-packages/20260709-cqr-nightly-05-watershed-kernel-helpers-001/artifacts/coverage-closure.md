# Coverage Closure

Evidence label: Static/Ran.

Status: `COMPLETE`

ADR-0021 tier: `science`, because the target owns WS12 impoundment
stage-discharge, stage-area continuity, and adaptive integration behavior bound
to `SC-IMPOUND-001`.

Closure rule:

- If characterization tests are added or materially changed, record line and
  region coverage status for the target, per-function 75% region-floor status or
  disposition, and obligation-to-test binding before completion.
- LCOV does not provide region coverage; if no region-capable report is
  available, record the gap explicitly and route to hold when the ADR gate
  cannot be satisfied.

After status:

- Eligible production surface excludes the root-level test-only block beginning
  at line `589`.
- Production-only LCOV target line coverage: `LF:532`, `LH:487`,
  `91.54135338345864%` - passes the science-tier line threshold.
- Production-only JSON target region coverage: `517 / 558`,
  `92.65232974910394%` - passes the science-tier region threshold.
- Whole-file JSON target line coverage, including inline tests: `811 / 857`,
  `94.63243873978999%`.
- Whole-file JSON target region coverage, including inline tests: `922 / 964`,
  `95.64315352697096%`.
- Whole-file JSON function coverage, including inline tests: `39 / 39`,
  `100.0%`.

Per-function floor disposition:

- Full llvm-cov JSON export:
  `/tmp/openwepp-cqr-nightly-05-helpers-focused-full.json`.
- Production functions with the lowest source-level region coverage:

| Function | Covered regions | Total regions | Region coverage |
|---|---:|---:|---:|
| `integrate_impoundment_stage_with_adaptive_retry` | `79` | `94` | `84.04255319148936%` |
| `route_impoundment_stage_over_duration` | `46` | `53` | `86.79245283018868%` |
| `impoundment_rk4_step` | `47` | `54` | `87.03703703703704%` |
| `impoundment_rockfill_outflow` | `21` | `23` | `91.30434782608695%` |
| `impoundment_culvert_family_outflows` | `52` | `55` | `94.54545454545455%` |

- All other target production functions are `95.0%` to `100.0%`
  region-covered.
- Generic guard constructors have multiple monomorphizations where applicable;
  each observed monomorphization is `100.0%` region-covered.

Whole-file metrics, including inline tests, are not used for science-tier
closure. They are recorded only for reproducibility.

Superseded whole-file values:

- JSON target line coverage: `811 / 857`, `94.63243873978999%`.
- JSON target region coverage: `922 / 964`, `95.64315352697096%`.
- JSON function coverage: `39 / 39`, `100.0%`.

Obligation-to-test binding:

- `INV-IMPOUND-003`: outlet-structure branch assembly and controlling-flow
  minima are characterized by the full outlet-family aggregation test and
  helper-level denominator guard tests.
- `INV-IMPOUND-004`: RK4/adaptive integration and regime-transition retry are
  characterized by stable-step, regime-transition retry, error-control retry
  with accepted smaller timestep, and invalid-timestep tests.
- `OBL-IMPOUND-P-004`: typed fail-closed behavior is characterized for missing,
  non-finite, and domain-violation guard paths without changing the public
  error classes.

Closure caveat:

- No new science authority is claimed. Coverage evidence supports this CQR
  refactor only; it does not promote `SC-IMPOUND-001` maturity or close broader
  impoundment physics parity.
