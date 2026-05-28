# WSHEDIMPL36 Implementation and Test Evidence

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Runtime projection update in
  `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`:
  - added conditional WS10 channel seed projection for
    `ws10_channel_{id}_{rccoef,rcexp,rcoset}` on `icntrl==4` lanes,
  - added explicit typed fail-closed payload-shape guards:
    `icntrl==4` requires rating-curve payload and `icntrl!=4` rejects
    rating-curve payload,
  - added explicit typed domain guards for projected rating-curve symbols:
    `rccoef>0`, `rcexp>0`, `rcoset>=0`.
- Parser contract surface was already authoritative for rating-curve domains;
  WSHEDIMPL36 adds parser vectors and runtime-seed symmetry.
- Test/fixture updates:
  - `tests/integration/infile_watershed_channel_parser_contract.rs`
  - `tests/fixtures/infile/watershed_channel/strict_rating_curve_rccoef_non_positive.chn`
  - `tests/fixtures/infile/watershed_channel/strict_rating_curve_rcoset_negative.chn`
  - runtime seam vectors in
    `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`

## Ran
- `cargo test --test infile_watershed_channel_parser_contract strict_mode_rejects_rating_curve_rccoef_non_positive` -> pass
- `cargo test --test infile_watershed_channel_parser_contract strict_mode_rejects_rating_curve_rcoset_negative` -> pass
- `cargo test -p openwepp-watershed-orchestrator watershed_channel_runtime_seed_` -> pass
