# CQR17 Behavior Equivalence

Status: closed.

Static: the production refactor preserves the
`Wb11HydrologyKernel::erod19_xcrit_classification` signature:

```rust
pub(crate) fn erod19_xcrit_classification(
    a: f64,
    b: f64,
    c: f64,
    tauc: f64,
    xb: f64,
    xe: f64,
) -> (f64, f64, f64)
```

Static: arithmetic from the original classification branches was moved into
private helpers without changing the existing formulas:

- `tauc.powf(1.5) - c`, clamped to zero for the threshold offset.
- `Self::erod19_shear(a, b, c, xb)` and
  `Self::erod19_shear(a, b, c, xe)`.
- Linear, rising, all-above, curved-root, and two-root branch outputs.
- Final `xc1` and `xc2` clamping to `[xb, xe]`.

Static: no runtime symbols, `WritebackField` publications, typed guard errors,
phase dispatch, parser behavior, unit metadata, or science-contract semantics
were changed.

Ran: focused characterization was added before production refactor and passed
against the pre-refactor implementation:

```text
cargo test -p openwepp-hillslope-orchestrator \
  cqr17_erod19_xcrit_classification_preserves_branch_vectors -- --nocapture
```

Result: exit code `0`; one test passed with ten branch vectors.

Ran: the same focused characterization passed after decomposition with exit
code `0`.
