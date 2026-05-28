# WSHEDIMPL32 Contract Test Implementation Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Updated parser contract vectors in
  `tests/integration/infile_watershed_channel_parser_contract.rs`:
  - `compatibility_mode_normalizes_legacy_ishape` now asserts compatibility
    normalization to naturally eroded class (`ishape=3`),
  - `strict_mode_accepts_naturally_eroded_ishape_class` verifies strict parser
    acceptance of class `3`.
- Added runtime seam vectors in
  `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs` tests:
  - `watershed_channel_runtime_seed_projects_naturally_eroded_ishape`,
  - `watershed_channel_runtime_seed_rejects_out_of_domain_ishape`.
- Added/updated parser fixtures:
  - `tests/fixtures/infile/watershed_channel/compat_ishape_normalized.chn`
    (`ishape=9` to exercise compatibility normalization path),
  - `tests/fixtures/infile/watershed_channel/strict_ishape_naturally_eroded.chn`
    (`ishape=3` strict acceptance vector).

## Ran
- `cargo test --test infile_watershed_channel_parser_contract` -> pass
- `cargo test -p openwepp-watershed-orchestrator watershed_channel_runtime_seed_projects_naturally_eroded_ishape` -> pass
- `cargo test -p openwepp-watershed-orchestrator watershed_channel_runtime_seed_rejects_out_of_domain_ishape` -> pass
