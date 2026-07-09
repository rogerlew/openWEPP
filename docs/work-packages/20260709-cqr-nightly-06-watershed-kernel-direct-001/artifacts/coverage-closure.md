# Coverage Closure

Evidence label: Static/Ran.

Status: `ADR-0021-TARGETED-PASS`

ADR-0021 tier:

- `science`

Reason:

- Target code owns direct watershed channel routing, impoundment routing, and
  sediment/load-capacity publication helpers governed by `SC-ROUTE-001`,
  `SC-SED-001`, and `SC-IMPOUND-001`.

Coverage evidence:

- Targeted LCOV after: `LF:1888`, `LH:1782`, line coverage
  `94.385593220339%`.
- Targeted deduplicated source-span region coverage from llvm-cov JSON:
  `2123 / 2274`, region coverage `93.35971855760774%`.
- Targeted CRAP after: no target functions above CRAP `30`.
- Module threshold result: science-tier line and region thresholds are both
  above `90%`.
- Per-function floor result: all eligible production functions in `direct.rs`
  with llvm-cov source-span regions meet the ADR-0021 `75%` region floor.
  Lowest deduplicated per-function region rows from the final7 JSON are:

| Function | Covered regions | Total regions | Region coverage |
|---|---:|---:|---:|
| `compute_direct_channel_peak` | `32` | `41` | `78.048780%` |
| `accumulate_direct_hillslope_sediment` | `19` | `24` | `79.166667%` |
| `dependency_channel_deep_seepage_m3` | `13` | `16` | `81.250000%` |
| `assemble_direct_incoming_sediment_load_and_capacity` | `67` | `82` | `81.707317%` |
| `direct_ws12_impoundment_coefficients` | `83` | `99` | `83.838384%` |
| `route_direct_impoundment_outflow` | `47` | `54` | `87.037037%` |

No source-span denominator exclusions or `COVERAGE-EXCLUDE` annotations were
introduced.

Obligation-to-test binding:

- Row-level applicable obligation vectors are bound in
  `artifacts/obligation-to-test-map.md`.
- `SC-ROUTE-001`: direct channel runon/runoff, ipeak branches, hourly inlet
  superposition, peak-duration closure, WS20 sediment/channel profile surfaces,
  and coupling payload admission are bound to package-local helper tests plus
  `wshedw5_typed_watershed_runtime_contract`.
- `SC-SED-001`: direct sediment continuity, transport-capacity, and
  hillslope-to-routing payload obligations are bound to package-local sediment
  helper tests.
- `SC-IMPOUND-001`: direct impoundment continuity/outflow, stage/horizon
  guards, dependency payloads, and unit-governance preservation are bound to
  package-local helper tests plus `wshedw5_typed_watershed_runtime_contract`.

Disposition:

- ADR-0021 targeted closure is pass for the implemented CQR scope.
- Full-workspace llvm-cov was attempted but stopped in the known unrelated
  coverage-instrumented `laned_shadow_h2637` failure/hang. Final non-coverage
  workspace gates are recorded separately in `gate-results.md`.
