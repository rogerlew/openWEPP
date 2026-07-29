# CAL-07 Independent Verification B

Evidence class: `Ran + Static`

Verdict: `PASS`

## Scope

Independently verify source and ensemble custody, the fail-closed VPD
diagnosis, absence of partial model products, figure accessibility and source
bindings, Markdown sidecars, the package/roadmap/catalog disposition, and
documentation lint. This verifier did not execute or modify the analysis and
edited only this artifact.

## Source and ensemble custody

Ran independent CSV parsing, `sha256sum`, byte counts, and direct comparison
with the CAL-04B accepted ledger and candidate table.

- `artifacts/source-manifest.csv`: 13 source objects; 13/13 SHA-256 digests
  and byte counts match the frozen files. Manifest SHA-256:
  `ccbd14ba27813435ed7d87b2e200f76e9e085c67dfa37d6db2ef16631e77c346`.
- CAL-04B accepted-ledger SHA-256:
  `83e749a3961604e4592f2a2217db30965c8bbb59f4752d0ff6d85fbac61fd986`.
- CAL-04B candidate-table SHA-256:
  `a56a07ef1df713a1555afb4098bd4f5e0fbe4b9f324ded0bd370c785fea3190e`.
- `inputs/ensemble.csv`: SHA-256
  `1c676cbd18e9157743c8957c9356ab1281c0bbef0f0049bcfcb47387a753fcd8`;
  37 rows, 37 unique accepted identifiers, exact accepted-ledger order, and
  zero parameter mismatches against the CAL-04B candidate table.
- `artifacts/ensemble-custody.csv`: 37 rows, 37 unique identifiers, and zero
  parameter mismatches against `inputs/ensemble.csv`. SHA-256:
  `23173591850b3eaddfd41fdfd628759138bc9c52c53b2f176703f2bc99720ebd`.

Result: `PASS`.

## Independent negative-VPD reconstruction

Ran `jq` directly over both frozen NASA POWER JSON objects and reconstructed
each row with the production equation:

```text
es(T) = 0.6108 * exp(17.27 * T / (T + 237.3))
VPD Pa = 1000 * (0.5 * (es(Tmax) + es(Tmin)) - es(Tdew))
```

The independent reconstruction examined 3,332 source rows and found exactly
three negative values, all in `SH-EN-ALERCE`:

| Date | Tmax (C) | Tmin (C) | Tdew (C) | VPD (Pa) |
|---|---:|---:|---:|---:|
| 2022-07-22 | 11.10 | 5.86 | 9.44 | -58.860502313193393 |
| 2022-09-15 | 10.97 | 5.32 | 9.30 | -70.492437340680112 |
| 2025-09-09 | 12.29 | 9.71 | 11.06 | -1.002242143992804 |

There is exactly one negative row in 2025, a prespecified scoring year.
`artifacts/negative-vpd-days.csv` contains exactly these three rows and has
SHA-256
`a31a5d078922580a920f469d2cfd0d3d1c911f1016c6f7b75a61b642d060eb17`.

Result: `PASS`.

## No partial result publication

Ran exact existence checks in `artifacts/`. All six prohibited result names
are absent:

```text
daily-kernel-output.csv
gate-results.csv
ensemble-daily.csv
shape-scores.csv
transition-residuals.csv
verdict-matrix.csv
```

The package publishes source/forcing diagnostics only. It contains no daily
canopy output, model-observation score, transition residual, result verdict
matrix, or focused-test receipt.

Result: `PASS`.

## Figures and Markdown sidecars

Parsed all four SVGs independently as XML and checked their paired sidecars.

- Four SVGs and four same-stem Markdown sidecars are present.
- Every SVG has `role="img"`, `aria-labelledby="title desc"`, a matching
  `<title>`, a `<desc>`, and `<metadata>`.
- Every embedded source binding resolves to an existing package file whose
  independently calculated SHA-256 equals the embedded digest.
- Every sidecar repeats its SVG's exact bound digest and contains `Caption`,
  `How to read it`, `Plain-language takeaway`, `Methods and source binding`,
  `Limitations`, and `Accessibility`.
- `artifacts/diagnostic-manifest.csv` contains 11 rows; 11/11 digest and byte
  bindings match. Manifest SHA-256:
  `3d63e2ccc1cee2919d4caa514d13b6f35a98f906469abd53da8821ad6aad6b54`.

Result: `PASS`.

## HOLD wording and validator

Static checks found consistent current disposition:

- `package.md`: `hold / forcing authority incompatible`.
- `artifacts/final-disposition.md`:
  `HOLD / FORCING AUTHORITY INCOMPATIBLE / NO CANOPY RESULT`.
- Roadmap Order 7: not passed, remains open, with no partial ensemble result.
- Work-package catalog: same HOLD/no-result wording and contract-admissible
  forcing prerequisite.

Ran:

```text
.venv/bin/python docs/work-packages/20260728-canopy-cal-07-southern-hemisphere-robustness-001/tools/validate_hold.py
```

Output:

```text
CAL-07 HOLD validation PASS: 3 negative VPD days; no partial canopy result
```

Result: `PASS`.

## Documentation lint

Before adding this verifier artifact, ran:

```text
markdown-doc lint --path docs/work-packages/20260728-canopy-cal-07-southern-hemisphere-robustness-001
markdown-doc lint --path docs/planning/canopy-phenology-assurance-roadmap.md
markdown-doc lint --path docs/work-packages/README.md
```

Results were 17 package files, one roadmap file, and one catalog file
validated with zero errors and zero warnings. The artifact-inclusive package
rerun then validated 21 Markdown files with zero errors and zero warnings.

## Final determination

`PASS`. Source and ensemble custody are intact; the exact three negative VPD
rows reproduce independently; one is in 2025; no partial model products
exist; all four figure/sidecar pairs satisfy their accessibility, provenance,
and explanatory-section obligations; and package, roadmap, and catalog all
retain the scientifically correct HOLD.
