# Line-Count Governance

Evidence mode: Ran.

Command:

```bash
wc -l tools/snowfreeze_observed/non_snotel_rubric_baseline.py \
  tools/snowfreeze_observed/README.md \
  docs/work-packages/20260625-snowfrost-fidelity-i0-non-snotel-rubric-baseline-001/package.md
```

Result:

```text
  730 tools/snowfreeze_observed/non_snotel_rubric_baseline.py
  182 tools/snowfreeze_observed/README.md
   78 docs/work-packages/20260625-snowfrost-fidelity-i0-non-snotel-rubric-baseline-001/package.md
  990 total
```

No touched `.rs` files. No file crosses a work-package line-count threshold.
