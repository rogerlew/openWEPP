# Line Count Governance

Evidence class: Static.

Ran:

```bash
wc -l docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md \
  crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs \
  tools/snowfreeze_observed/harder_pomeroy_default_activation.py \
  tests/integration/snowdensity10_3_19_harder_pomeroy_default_activation.rs \
  tests/integration/snowdensity03_physics_bulk_offline_contract.rs \
  docs/work-packages/20260628-snowdensity-10-3-19-harder-pomeroy-default-activation-001/package.md \
  docs/work-packages/20260628-snowdensity-10-3-19-harder-pomeroy-default-activation-001/artifacts/harder-pomeroy-default-activation.md
```

Snapshot after closeout artifacts:

- `SC-SNOWFREEZE-001.md`: `2744` lines.
- `00_builders_and_authority.rs`: `2612` lines.
- `harder_pomeroy_default_activation.py`: `427` lines.
- `snowdensity10_3_19_harder_pomeroy_default_activation.rs`: `186` lines.
- `snowdensity03_physics_bulk_offline_contract.rs`: `129` lines.
- `package.md`: `80` lines.
- `harder-pomeroy-default-activation.md`: `33` lines.
- `disposition.md`: `23` lines.

The large contract and direct-publication helper were pre-existing large files;
the package kept edits scoped to authority, selector defaulting, internal trace
evidence, and focused guards rather than splitting unrelated refactors into the
activation package.
