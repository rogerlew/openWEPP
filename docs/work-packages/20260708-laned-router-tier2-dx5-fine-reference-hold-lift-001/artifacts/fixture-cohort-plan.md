# Fixture Cohort Plan

Status: `DRAFT`
Evidence mode: Static.

## Narrow Rerun

Primary member: `mn_corn_h4`.

Rungs:

- fixed operational baseline (`baseline_fixed10`)
- `dx20`
- `dx10`
- `dx5`
- `dx2p5`
- `dx1p25`
- `dx0p625`

The package runs the complete `mn_corn_h4` rung set so the new `dx0p625`
reference can both close the strict adequacy gate and rebase `mn_corn_h4`
candidate comparisons against `dx1p25` if adequacy closes.

## Standing Evidence

The prior rev-41 selected-cohort evidence stands for `h2637`,
`n_idaho_forest_h1`, and `wa_cascades_forest_h1` unless the new `mn_corn_h4`
evidence implicates a judged surface. H2637 remains synthetic stress only.

## Raw Output Hygiene

`artifacts/fine-reference-runs/` is ignored. The committed evidence surface is
the package-local summary JSON/Markdown, release-binary provenance, output
hashes, and disposition artifacts.
