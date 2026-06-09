# Manifest Preflight Evidence

Evidence mode: Ran

Commands run from `/workdir/openWEPP`:

| Command | Result |
|---|---|
| `tools/owcmp/owcmp manifest list --json` | pass; returned `owcmp-suite-list-v1` with `3` suites |
| `tools/owcmp/owcmp env --manifest tools/owcmp/suites/n-idaho-single-ofe-ksflag0.json --json` | pass; pyarrow `24.0.0`, `39/39` plot outputs present, interchange WAT parquet present |
| `tools/owcmp/owcmp env --manifest tools/owcmp/suites/minnesota-corn-ksflag1.json --json` | pass; pyarrow `24.0.0`, `43/43` WAT outputs and `43/43` plot outputs present |
| `tools/owcmp/owcmp env --manifest tools/owcmp/suites/wa-cascades-mofe-ksflag0.json --json` | pass; pyarrow `24.0.0`, `36/36` WAT outputs, `36/36` plot outputs, and watershed structure present |

The manifests are preflight declarations. They do not run comparisons or mutate
the `/wc1` run roots.

