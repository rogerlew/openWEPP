# Implementation and Test Evidence

Status: complete
Evidence mode: Static + Ran

## Implemented Files

- `tools/owcmp/owcmp`
  - Repo-local Python CLI dispatcher for `wat semantic`, `pl14s run`,
    `summarize`, and `manifest run`.
  - `manifest run` is limited in OWCMP01 to a PL14S lane plus explicit `args`
    pass-through to `pl14s run`; full manifest schema/identity validation is
    deferred.
  - `observe` fails closed with the package boundary:
    `owcmp observe normalize is deferred to a separate observability package`.
- `tools/owcmp/semantic_wat.py`
  - Port of `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`.
  - Preserves schema `pl14s-semantic-wat-v2`, row-key duplicate rejection,
    candidate parquet partition filtering, candidate year offset, alias sources,
    row-width diagnostics, tolerance checks, and top divergent-row reporting.
- `tools/owcmp/pl14s_suite.py`
  - Port of `tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`.
  - Preserves schema `pl14s-legacy-suite-v2`, baseline replay, strict `.dat`
    comparator invocation with `--abs-tol 0 --rel-tol 0`, candidate source
    classes, baseline-year policy, expected common-row-count checks,
    strict-equivalent blockers, conversion-derived row-consistency blockers,
    and provenance/hash capture.
  - Namespace changes are limited to the default tolerance path and semantic
    comparator script path.
- `tools/owcmp/summary.py`
  - Emits compact `summary.json` and `summary.md` for semantic reports and
    PL14S provenance manifests.
  - Reports policy-skipped commands as `SKIPPED` instead of `FAIL`.
- `tools/owcmp/configs/pl14s_wat_tolerances.json`
  - Byte-identical copy of the legacy PL14S WAT tolerance profile.
- `tools/owcmp/requirements.in`
  - Byte-identical copy of the legacy Python dependency input file.
- `tools/owcmp/requirements.lock.txt`
  - Byte-identical copy of the legacy Python dependency lock.
- `tools/owcmp/README.md`
  - Local command overview and OWCMP01 compatibility notes.
- `tests/integration/owcmp_cli_contract.rs`
  - Focused CLI contract/regression tests for the new path.
- `Cargo.toml`
  - Adds the `owcmp_cli_contract` integration test target.

## Preserved PL14S Behavior

- Semantic report schema remains `pl14s-semantic-wat-v2`.
- Suite provenance schema remains `pl14s-legacy-suite-v2`.
- Duplicate `(OFE, J, Y)` row keys hard-fail before comparison.
- Strict raw comparison remains required for `.dat` candidate inputs.
- Parquet and conversion-derived paths retain strict-equivalent and
  row-consistency policy metadata.
- Default strict comparator authority remains
  `/workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py`.
- The existing legacy contract test remains unchanged and passing.
- `tools/legacy_comparison_suite` remains present and runnable for OWCMP02.

## Focused Test Coverage

`tests/integration/owcmp_cli_contract.rs` covers:

- Static CLI/spec markers for `wat semantic`, `pl14s run`, `summarize`,
  `manifest run`, and deferred `observe normalize`.
- Semantic duplicate row-key hard failure through `tools/owcmp/owcmp`.
- End-to-end semantic compare plus `summarize` smoke on small `.dat` fixtures.
- Dynamic `pl14s run` smoke using a fake baseline replay script and fake strict
  comparator to assert provenance, strict-lane, semantic-lane, and tolerance path
  behavior without expensive external fixtures.
- PL14S provenance summary behavior for policy-skipped strict comparisons.
- PL14S provenance summary behavior for failed recorded commands.
- Deferred observe command failure.

## Sample Smoke Output

Command shape:

```bash
tmpdir=$(mktemp -d)
printf '1 1 2008 ...\n1 2 2008 ...\n' > "$tmpdir/baseline.wat.dat"
cp "$tmpdir/baseline.wat.dat" "$tmpdir/candidate.wat.dat"
python3 tools/owcmp/owcmp wat semantic \
  --baseline-wat "$tmpdir/baseline.wat.dat" \
  --candidate-wat "$tmpdir/candidate.wat.dat" \
  --report-json "$tmpdir/report.json"
python3 tools/owcmp/owcmp summarize \
  --input "$tmpdir/report.json" \
  --output-root "$tmpdir/summary"
```

Observed compact output:

```text
{"summary_json": "/tmp/tmp.dtitUFxjs1/summary/summary.json", "summary_md": "/tmp/tmp.dtitUFxjs1/summary/summary.md", "verdict": "PASS"}
/tmp/tmp.dtitUFxjs1/baseline.wat.dat
/tmp/tmp.dtitUFxjs1/candidate.wat.dat
/tmp/tmp.dtitUFxjs1/report.json
/tmp/tmp.dtitUFxjs1/summary/summary.json
/tmp/tmp.dtitUFxjs1/summary/summary.md
```
