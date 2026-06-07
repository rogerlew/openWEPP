# Verification Agent B

Status: complete

Evidence mode: Static + Ran.

## Verification

Static:

- Confirmed `SC-PLANT-001`, `SC-EVAP-001`, `SC-WATBAL-001`, and
  `SC-RUNOFFPART-001` carry the new authority and revision entries.
- Confirmed the package artifacts disposition the `Er` overclaim.

Ran:

- 36-prefix Corn population run: zero missing/failed prefixes, zero Corn
  prefixes with zero `Ep`, zero Corn prefixes with zero `Interception`.
- Annual closure ledger: `216` annual rows, max absolute residual
  `3.1604940886609256e-11 mm`.
- Final gate:
  `cargo fmt --check && git diff --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test -p openwepp --test hphys0320_stmtim_start_time_source_line_contract && cargo test --workspace && cargo deny check`:
  passed.

Result: verified.
