# Reproduction Evidence

Status: `PASS AT SOURCE LEVEL / CANONICAL V2 BUILD BLOCKED`

Evidence class: `Ran`

## Strict Result

From the repository root, `.venv/bin/python` executed the retained procedure
with the accepted ensemble, complete candidate configurations, Harvard
holdout, litter ridge, gradient summary, Bezà member summary, and Elliot
comparison. Fresh output compared byte for byte with
`results/canopy-synthesis.json`.

- procedure SHA-256:
  `80c9af19d4aac382ca8019726b2aef93caa4b9d14afd26cfd9ea3e7bbb35a110`
- strict-result SHA-256:
  `515344ded0cc73b344cc40f7972439a2036adef39401443c432f79f72d605dba`
- reconstructed values: 32
- exact value bindings: 32 of 32 used

The procedure independently counts all 9,261 searched configurations,
reconstructs accepted-ensemble boundary counts and six coefficient ranges,
checks each within-site winter canopy ordering from retained CAL-06 operands,
and reconstructs the remaining timing, litter, Bezà, and Elliot quantities.

## Figures

`build_candidate_figures.py` ran successfully twice in the final correction
loop. All eight SVG SHA-256 identities matched the frozen inventory in
`candidate-figure-build.md`; all SVGs passed `xmllint --noout`. The
repository-relative source manifest contains nine rows, and every retained
digest matched. All plotted source and derived tables are declared research
objects.

## Source Validation

Direct Draft 2020-12 validation passed for `report.yaml`,
`canopy-synthesis.json`, and `assurance/v2/catalog.yaml`. Custom semantic
closure passed for:

- 39 research objects, paths, and foreign keys;
- 12 references;
- 32 value bindings;
- one table and one schema-native figure; and
- exact-use closure for every declared public object and directive.

Markdown lint passed with zero errors or warnings for 40 package files and two
report Markdown files. American-English normalization was idempotent.

## Canonical Boundary

An attempted direct catalog registration caused canonical `validate` and
`plan` to stop before report evaluation with:

```text
ERROR: SHA-256 mismatch for identified source 'assurance/v2/catalog.yaml'
```

The invalid unadmitted catalog change was removed. Canonical `validate --all`
then passed for both existing admitted V2 reports. The active CLI exposes
mutation operations for existing reports but no typed new-report admission
operation, and direct canopy validation now truthfully returns `unknown v2
report ID`. The identity lock has no canopy-report members. Hand-editing that
lock is prohibited, so canopy-specific canonical staging, normalization
checking, build, and check are `BLOCKED`, not passed.
