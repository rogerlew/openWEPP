# Function Length After

Static: after refactor, `00_core_types.rs` has 1255 lines.

Static: after refactor, no
`#[allow(clippy::too_many_lines)]` remains in `00_core_types.rs`.

Static: `HillslopeRuntimeInputError::fmt` is now a dispatcher beginning at line
1183. Display formatting is routed to private family helpers beginning at lines
502, 570, 669, 723, 791, 865, 941, 959, 1037, and 1105.

Static: `HillslopeRuntimeInputError::code` remains public at line 319 and routes
to private code-family helpers beginning at lines 388, 406, 420, 431, 445, 458,
470, and 486.

Ran: `cargo clippy -p openwepp-hillslope-orchestrator --all-targets --
-D warnings` passed after the refactor.

Ran: `cargo clippy --workspace --all-targets -- -D warnings` passed after the
refactor.

Disposition: function-length lint debt closed without replacement suppression.
