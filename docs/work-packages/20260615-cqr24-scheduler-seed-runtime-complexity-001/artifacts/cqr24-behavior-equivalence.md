# CQR24 Behavior Equivalence

Status: complete.

Static: the production change is private helper extraction for WB16 ealpha
production. Public function signature, visibility, call sites, runtime symbol
names, output symbol names, units, typed error enum, and parser compatibility
are unchanged.

Static: the refactor preserves:

- `Ok(None)` behavior for missing required runtime inputs.
- `wb16_ealpha_producer` error surface.
- SIMPIPE guard message IDs and guard detail text shape.
- `ofe{n}_frcteq`, `ofe{n}_alpha`, first-OFE `alpha`, and `ealpha`
  publication symbols.
- WB16 roughness, canopy, `frcteq`, OFE `alpha`, and equivalent-plane `ealpha`
  formula order.

Ran: focused characterization before production refactor:

- `cargo test -p openwepp-runner hillstab08_wb16_producer`
- Result: pass, `2` tests.

Ran: focused characterization after production refactor and suppression cleanup:

- `cargo test -p openwepp-runner hillstab08_wb16_producer`
- Result: pass, `2` tests.
