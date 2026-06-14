# Kernel Profile Compliance Checklist

Status: W-A executed

Evidence mode: Static

W-A did not edit kernel/runtime production code.

Checklist for subsequent implementation:

- W-B touches an input parser feeding watershed runtime state; use typed errors
  and fail closed on genuine malformed input.
- W-C touches watershed routing/output publication; use independent
  conservation operands and do not accept writer defaults as closure evidence.
- No `.unwrap()`/`.expect()` may be introduced in production paths.
- No silent canonicalization of invalid impoundment counts; zero is explicit
  empty-set semantics, not a fallback.
- Any future closure claim must include current-run evidence for
  `cargo fmt --check`, clippy, workspace tests, `cargo deny check`, and the
  package-specific totalwatsed3 audit gates.
