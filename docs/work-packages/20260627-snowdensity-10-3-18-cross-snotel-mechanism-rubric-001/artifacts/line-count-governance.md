# Line-Count Governance

Ran:

```text
wc -l tools/snowfreeze_observed/cross_snotel_mechanism_rubric.py \
  tests/integration/snowdensity10_3_18_cross_snotel_mechanism_rubric.rs \
  docs/work-packages/20260627-snowdensity-10-3-18-cross-snotel-mechanism-rubric-001/package.md \
  docs/work-packages/20260627-snowdensity-10-3-18-cross-snotel-mechanism-rubric-001/artifacts/cross-snotel-mechanism-rubric.md
```

Result:

```text
1167 tools/snowfreeze_observed/cross_snotel_mechanism_rubric.py
  55 tests/integration/snowdensity10_3_18_cross_snotel_mechanism_rubric.rs
  82 docs/work-packages/20260627-snowdensity-10-3-18-cross-snotel-mechanism-rubric-001/package.md
  78 docs/work-packages/20260627-snowdensity-10-3-18-cross-snotel-mechanism-rubric-001/artifacts/cross-snotel-mechanism-rubric.md
1382 total
```

Disposition: accepted for this package because the tool is a diagnostic report
assembler over heterogeneous SNOTEL, cancov, direct-runtime, archival, and flag
profiles. Future reuse should split site/model definitions, direct-run plumbing,
and report rendering before adding more corpora.
