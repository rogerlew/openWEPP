# CAL-07B Gate Evidence

Evidence class: `Ran + Static`

## Source custody

- Three frozen NASA POWER hourly LST JSON responses were retained for
  2022-07-22, 2022-09-15, and 2025-09-09 at
  `(-40.1726, -73.4439)` with parameters `T2M,T2MDEW`.
- Local copies of the POWER daily API, hourly API, temporal processing,
  meteorology, and time FAQ pages were retained.
- `artifacts/source-manifest.csv` records path, SHA-256, bytes, retrieval
  timestamp, and URL for every retained source object.
- The CAL-07 daily Alerce POWER response is referenced by retained path and
  SHA-256; it was not reacquired or altered.

## Reconstruction and attribution

- `tools/analyze.py` generated:
  - `artifacts/hourly-reconstruction.csv`;
  - `artifacts/daily-decomposition.csv`;
  - `artifacts/attribution.csv`; and
  - `artifacts/source-manifest.csv`.
- Every case contains exactly 24 unique LST hourly keys.
- All 72 hourly-product VPD reconstructions are positive.
- All three reconstructed contract-daily VPD values are negative.
- All three CAL-07 contract-daily VPD signs agree with the reconstructed
  contract-daily signs.
- All three cases are classified as
  `DAILY_SUMMARY_OPERATOR_MISMATCH`.

## Figure and sidecar evidence

- `tools/plot.py` generated:
  - `artifacts/figures/cal07b-hourly-operands-and-vpd.svg`;
  - `artifacts/figures/cal07b-additive-driver-decomposition.svg`;
  - `artifacts/figures/cal07b-source-reconstruction.svg`; and
  - one Markdown sidecar for each SVG.
- `artifacts/result-manifest.csv` binds the published tables, SVGs, and
  sidecars by SHA-256 and byte count.

## Independent validator

- `tools/validate.py` independently parses the retained raw JSON and
  reconstructs hourly rows, daily rows, decomposition closure, attribution,
  figure/sidecar presence, and result-manifest hashes without importing
  analyzer helpers.

Validator output:

```text
CAL-07B validation PASS: 72 positive hourly-product VPD rows; 3 DAILY_SUMMARY_OPERATOR_MISMATCH attributions
```

## Terminal gates

- `.venv/bin/python -m py_compile <CAL-07B tools>`: passed.
- `xmllint --noout <CAL-07B SVGs>`: passed for all three figures.
- `rsvg-convert <CAL-07B SVGs>`: passed for all three figures.
- `markdown-doc lint --path <CAL-07B package>`: 21 files, zero errors,
  zero warnings.
- `markdown-doc lint --path docs/planning/canopy-phenology-assurance-roadmap.md`:
  one file, zero errors, zero warnings.
- `markdown-doc lint --path docs/work-packages/README.md`: one file, zero
  errors, zero warnings.
- `git diff --check`: passed.

Terminal review and verification evidence is retained in the paired
`review-agent-*.md` and `verification-agent-*.md` artifacts. Both terminal
reviewers recorded static `GO` dispositions with no findings. The root agent
ran the terminal executable gates listed above.
