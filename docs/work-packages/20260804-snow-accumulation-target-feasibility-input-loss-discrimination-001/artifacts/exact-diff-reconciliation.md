# Exact-Diff Reconciliation

Status: `pass / intended write set exact`

Evidence mode: `Ran`

The exact tracked candidate path set relative to intake commit `237ba40d`
contains 42 paths. Thirty-nine are under the package directory; the remaining
three are the explicitly authorized catalogs:

- `docs/ROADMAP.md`;
- `docs/planning/snow-surface-energy-balance-roadmap.md`; and
- `docs/work-packages/README.md`.

The newline-delimited sorted path-list SHA-256 is
`0a8d5849718c8f3f1ac8f9288438292db23492ea3a625b47bd64861bb053e038`.
No production source, Cargo manifest, science contract, test fixture,
observation, normalized data, or public output path is present.

Ignored generated evidence is confined to the accepted
`target/snow_accumulation_target_feasibility_input_loss_discrimination_v2/`
namespace and the retained `_rejected_v1` namespace. The rejected namespace
cannot support claims. The terminal diff therefore matches the package's
declared write set and characterization-only implementation intent.
