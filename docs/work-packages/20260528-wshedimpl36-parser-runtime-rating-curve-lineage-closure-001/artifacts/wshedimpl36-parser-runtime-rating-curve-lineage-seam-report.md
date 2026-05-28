# WSHEDIMPL36 Parser/Runtime Rating-Curve Lineage Seam Report

Status: complete  
Evidence mode: static+ran  
Date: 2026-05-28

## Static
- Parser authority enforces rating-curve closure for `icntrl==4` lanes:
  - missing rating-curve line fails with `CHN-E-006`,
  - `rccoef>0`, `rcexp>0`, `rcoset>=0` enforced with `CHN-E-005` when
    violated.
- Runtime seed closure now enforces explicit payload-shape and domain symmetry:
  - projects `ws10_channel_{id}_{rccoef,rcexp,rcoset}` only when `icntrl==4`,
  - rejects `icntrl==4` rows missing rating-curve payload,
  - rejects `icntrl!=4` rows carrying rating-curve payload,
  - rejects out-of-domain projected rating-curve symbols with typed
    `WS-RUNTIME-E-010` failures.
- Lineage result:
  - watershed channel parser rating-curve authority and WS10 runtime seed
    authority now align on explicit payload-shape and domain semantics.

## Ran
- `cargo test --test infile_watershed_channel_parser_contract strict_mode_rejects_rating_curve_rccoef_non_positive` -> pass
- `cargo test --test infile_watershed_channel_parser_contract strict_mode_rejects_rating_curve_rcoset_negative` -> pass
- `cargo test -p openwepp-watershed-orchestrator watershed_channel_runtime_seed_` -> pass
