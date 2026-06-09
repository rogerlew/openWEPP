# owcmp

`owcmp` is the openWEPP comparison CLI. It owns the active PL14S WAT semantic
comparator and replay suite.

## Commands

```bash
tools/owcmp/owcmp wat semantic ...
tools/owcmp/owcmp pl14s run ...
tools/owcmp/owcmp batch h1-h39-semantic ...
tools/owcmp/owcmp summarize --input <report.json> --output-root <dir>
tools/owcmp/owcmp manifest run --manifest <manifest.json>
```

`owcmp observe normalize` is intentionally deferred to a separate observability
package.

Direct `tools/owcmp/owcmp` invocations re-exec through `.venv/bin/python` when
the repo-local environment exists. This keeps agent-run comparator commands on
the same dependency surface as local Python tooling, including optional parquet
support from `tools/owcmp/requirements.lock.txt`.

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

## Agent Runner Usage

`comparator_suite_runner` should use `owcmp` directly and return compact
artifacts rather than raw per-H reports. The standard H1-H39 semantic batch
surface is:

```bash
tools/owcmp/owcmp batch h1-h39-semantic \
  --baseline-dir <dir-with-baseline_H1.parquet...baseline_H39.parquet> \
  --candidate-dir <dir-with-H1.wat.parquet...H39.wat.parquet> \
  --candidate-year-offset 2012 \
  --output-root <package-artifacts>/runner-h1-h39
```

The command writes:

- `summary.json` and `summary.md` for parent-agent handoff.
- `command-log.json` for exact per-H command evidence.
- `reports/semantic/H*.semantic.json` for raw comparator reports.
- `logs/H*.stdout.txt` and `logs/H*.stderr.txt` for command logs.

Agents should report the execution verdict, semantic pass count, first divergent
key when present, focus-column metrics, and artifact paths. They should not paste
raw per-hillslope reports or logs into chat.
