# CAL-07E Terminal Verification B

Evidence class: `Independently Ran + Static`

Verdict: `PASS`.

## Independent data verification

I parsed the retained eight-row PhenoCam standard-record subset independently
of the package validator and matched it to all 12 rows in
`phenocam-transition-product-audit.csv`.

Result:

```text
independent audit PASS: source_rows=8 comparisons=12 mismatches=0 material_abs_delta_ge_10=4
```

The check independently:

- selected each year/direction/product source row;
- matched all `gcc_mean` and `gcc_90` nominal dates;
- chronologically normalized provider lower/upper endpoints before matching
  each recorded confidence interval;
- recomputed signed `gcc_90 - gcc_mean` date differences; and
- counted four falling comparisons with absolute offsets of at least 10 days.

The retained CAL-07 simplified file also matches the archive Data Record 7
content, confirming that its transitions are the `gcc_mean` simplified
product. CAL-07D `observation-support.csv` independently confirms that its
ancillary context fields are `smooth_gcc_90` and raw GCC90 support.

## Package validator

Ran:

```text
.venv/bin/python docs/work-packages/20260729-canopy-cal-07e-literature-authority-review-001/tools/validate.py
```

Result:

```text
CAL-07E validation PASS: 15 sources, 15 claims, 12 transition comparisons, 1 figure
```

The validator checks source and claim identifiers, evidence vocabularies,
complete 2024/2025 direction/threshold coverage, signed date arithmetic,
checksum identity and exact date/CI binding to the retained source subset,
required artifacts, SVG parsing, and sidecar structure.

## Documentation and figure checks

Ran:

```text
markdown-doc lint \
  --path docs/work-packages/20260729-canopy-cal-07e-literature-authority-review-001 \
  --path docs/planning/canopy-phenology-assurance-roadmap.md \
  --path docs/work-packages/README.md \
  --format plain
```

Result:

```text
16 files validated, 0 errors, 0 warnings
```

Ran:

```text
rsvg-convert -o /tmp/cal07e-review-b-final.png \
  docs/work-packages/20260729-canopy-cal-07e-literature-authority-review-001/artifacts/figures/cal07e-evidence-authority-map.svg
```

Result: `PASS`; the 1200 by 720 render was visually inspected. Labels are
legible, remain within their colored marks, and agree with the sidecar.

## Write-set, hygiene, and size checks

Compared the terminal tree to starting commit
`156c16062a1e645f0a91182b3e31fd03cb880e45`.

Result:

```text
write-set reconciliation PASS: changed=20 out_of_scope=0
git diff --check: PASS
tools/validate.py: 177 lines
```

Every change is within the declared CAL-07E package, canopy roadmap, or
work-package catalog. No Rust, production runtime, science-contract, forcing,
parameter, or predecessor-package file changed. The only package-local
executable remains below the line-count warning threshold.

## Verification conclusion

All current-scope exit criteria are supported. The evidence validates a
literature-review closure and a bounded CAL-07F method audit only. It does not
validate a science correction, empirical calibration, independent biological
validation, or release of canopy assurance Order 7.
