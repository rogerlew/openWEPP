# First quality02 failure

The first focused controls run exited `101` before execution because the
extracted initial-trace helper used an untyped `[1.0, 0.25]` radius array and
Rust could not resolve `to_bits`. The oracle literals now use `1.0_f64`; the
final focused controls rerun exits `0` with all 16 controls passing.

The first focused lint run then reported the remaining transition length and
two helper-by-value diagnostics. The final lint rerun has no diagnostics for
either touched trust-region source file; its exit `101` is inherited elsewhere
in the crate.
