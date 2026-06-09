# owcmp

`owcmp` is the openWEPP comparison CLI. OWCMP01 ports the active PL14S WAT
semantic comparator and replay suite into this namespace while leaving
`tools/legacy_comparison_suite` intact for the follow-on cutover package.

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
