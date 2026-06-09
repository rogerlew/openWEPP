# owcmp

`owcmp` is the openWEPP comparison CLI. It owns the active PL14S WAT semantic
comparator and replay suite.

## Commands

```bash
tools/owcmp/owcmp wat semantic ...
tools/owcmp/owcmp pl14s run ...
tools/owcmp/owcmp summarize --input <report.json> --output-root <dir>
tools/owcmp/owcmp manifest run --manifest <manifest.json>
```

`owcmp observe normalize` is intentionally deferred to a separate observability
package.

`manifest run` is intentionally minimal in OWCMP01: it accepts a PL14S lane and
an explicit `args` list, then dispatches to `owcmp pl14s run`. Full manifest
schema validation, identity-evidence validation, and promotability policy belong
to a later manifest package and must not be treated as complete for OWCMP02.

## PL14S Compatibility

- Semantic report schema: `pl14s-semantic-wat-v2`
- Suite provenance schema: `pl14s-legacy-suite-v2`
- Default tolerance config: `tools/owcmp/configs/pl14s_wat_tolerances.json`
- Optional parquet support uses `tools/owcmp/requirements.lock.txt`
- Strict comparator is required when candidate input is `.dat`; parquet runs
  are classified as `strict-equivalent-required` and must satisfy semantic-lane
  equivalence checks.
- `--candidate-surface-source-class` is required and must be one of
  `native-runtime-dat`, `conversion-derived-dat`, or
  `native-runtime-parquet`.
- `conversion-derived-dat` evidence is tagged as non-promotable for final
  Tier-A closeout claims and must satisfy row-consistency checks.
- Semantic comparator evidence includes row-presence deltas, per-column
  tolerance verdicts, top divergent rows, `Total-Soil` alias continuity,
  observed row-width diagnostics, and baseline/candidate column disclosure.
- Use `--candidate-partition-value` with the default `wepp_id` partition column
  for multi-hillslope parquet inputs.
- Use `--candidate-year-offset` when candidate rows use simulation-year keys and
  baseline rows use calendar-year keys.
