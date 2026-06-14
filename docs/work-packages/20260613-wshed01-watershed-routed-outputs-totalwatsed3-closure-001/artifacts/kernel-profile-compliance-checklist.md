# Kernel Profile Compliance Checklist

Status: W-D executed-hold

Evidence mode: Static

W-A did not edit kernel/runtime production code.

Checklist for subsequent implementation:

- W-B touches an input parser feeding watershed runtime state; use typed errors
  and fail closed on genuine malformed input.
- W-C touches watershed routing/output publication; use independent
  conservation operands and do not accept writer defaults as closure evidence.
- W-D corrected publication defects but did not claim closure because the
  independent audit still reports `2950.498418 mm` residual.
- W-D-REDO must bind independent daily PASS `runvol` lineage before closure.
- No `.unwrap()`/`.expect()` may be introduced in production paths.
- No silent canonicalization of invalid impoundment counts; zero is explicit
  empty-set semantics, not a fallback.
- Any future closure claim must include current-run evidence for
  `cargo fmt --check`, clippy, workspace tests, `cargo deny check`, and the
  package-specific totalwatsed3 audit gates.
