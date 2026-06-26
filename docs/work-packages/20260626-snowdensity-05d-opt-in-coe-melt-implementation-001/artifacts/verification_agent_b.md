# Verification Agent B

Evidence class: Ran + Static.

Verification mode: contract and anti-evasion pass.

## Commands

```sh
bash tools/release/check_authority_suite_antievasion.sh
cargo test --test auth11_required_suite_obligation_guards_contract
rg -n "dense_slow_melt_v1|snow_melt_model:|SnowMeltModel::|CoeShortwaveAlbedoV1|LegacyCoe|snow_melt_shortwave_absorbed_fraction|0\\.0607|qwet|frzftp" crates tests docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md -S
```

## Result

- Authority suite anti-evasion checks passed.
- Authority obligation guard test passed.
- Source scan confirmed the default production selector is `LegacyCoe` and the
  negative benchmark remains outside production melt physics.
