# Coverage Closure

Ran: science tier is used because the module aggregates and publishes
hydrologic volume/depth fields; ambiguous tier defaults to the stricter bar.

Production source ends before the pre-existing `#[cfg(test)]` module at line
`896`. Deduplicating identical source-region coordinates by maximum execution
count yields:

| Metric | Before | After | Science floor |
|---|---:|---:|---:|
| Lines | `424/629` (`67.409%`) | `607/629` (`96.502%`) | `90%` |
| Regions | `660/995` (`66.332%`) | `904/995` (`90.854%`) | `90%` |

Primary logical source functions clear the `75%` region floor after
coordinate-level deduplication; the lowest is `optional_int16_column_any` at
`11/14` (`78.571%`). The generic public builder is invoked consistently through
one `Vec<&PathBuf>` monomorph for absent, partial, nominal, and overflow cases,
eliminating the earlier separately executed low-coverage array-length
instantiation. Remaining zero-count compiler monomorph/error-closure instances
share already-covered source coordinates and are not separate obligations.

Obligation-to-test binding:

| Family | Applicability and binding |
|---|---|
| A — nominal valid input | `build_reads_wat_parquet_and_publishes_daily_rows` reads a real Parquet sibling and checks the published simulation-day index, area, selected depths, and runoff volume. The retained private batch-decoding test separately checks all parsed date components and representative optional fields. |
| B — boundaries/degenerate input | `build_handles_absent_and_partial_wat_sibling_inventories`, `build_rejects_non_finite_aggregated_area`, and the retained negative-Area test cover empty/partial inventories and invalid aggregate bounds. |
| C — state/order behavior | The retained private two-row batch-decoding test checks source row order and distinct day keys; the public same-day multi-OFE test checks grouping, outlet-only lateral aggregation, and optional-field carry. It does not claim a public two-day ordering assertion. |
| D — domain rejects | Aggregate overflow, negative Area, null scalar, wrong type, and missing-column tests assert typed rejection. |
| E — missing dependency/input | Absent/partial sibling inventory plus open and missing-schema cases cover required path/column absence. |
| F — non-finite values | `column_and_scalar_guards_cover_type_null_and_nonfinite_errors` covers NaN and both infinities; the aggregate overflow test covers derived non-finite Area. |
| G — conservation/continuity residual | The same-day multi-OFE test independently reconstructs weighted runoff depth and volume, separates Q/QOFE/Area aliases, and checks outlet-only lateral aggregation; the overflow test closes the invalid-magnitude boundary. |
| H — fail-closed posture | `display_formats_every_watershed_wat_error_variant`, `read_wat_file_reports_open_read_and_schema_failures`, and scalar guard tests cover every typed error family and public/file boundary without silent defaults. |
| Determinism (additional) | Fixed fixture values, ordered `BTreeMap` publication, atomic process-local fixture IDs, and RAII cleanup remove wall-clock/random inputs and preserve repeatable row assertions. |
| Output/aggregation schema | The target returns row seeds rather than serializing schema. The public test binds `sim_day_index`, area, runoff depth and volume, QOFE, lateral flow, profile porosity capacity, interception storage, and baseflow depth. Private decoding tests bind the parsed date components and representative optional inputs. Unasserted seed fields, including channel outflow/baseflow volumes, are not claimed as independently bound here; downstream writer schema is unchanged and out of scope. |
| Output closure/magnitude | Distinct Area, Q, QOFE, and Base values reject adjacent aliases; expected runoff volumes are independently reconstructed as `Q * Area / 1000`. Aggregate-area overflow and optional-zero cases cover degenerate magnitudes. |

No `COVERAGE-EXCLUDE` annotation or denominator shrink was added.
