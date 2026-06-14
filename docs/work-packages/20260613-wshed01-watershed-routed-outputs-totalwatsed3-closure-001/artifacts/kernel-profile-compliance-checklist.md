# Kernel Profile Compliance Checklist

Status: T-B executed

Evidence mode: Static

W-A did not edit kernel/runtime production code.

Checklist for subsequent implementation:

- W-B touches an input parser feeding watershed runtime state; use typed errors
  and fail closed on genuine malformed input.
- W-C touches watershed routing/output publication; use independent
  conservation operands and do not accept writer defaults as closure evidence.
- W-D corrected publication defects but did not claim closure because the
  independent audit still reports `2950.498418 mm` residual.
- T-A scoped a dedicated totalwatsed3 CLI; no production code was edited.
- T-B bound independent PASS `runvol` lineage for `Runoff`; WAT `Q` remains
  diagnostic.
- T-B fails closed on missing required PASS/WAT inputs with typed CLI errors.
- T-B did not silently substitute WAT `Q` for PASS `runvol`.
- T-B did not edit hillslope kernel process physics; changes are at the
  publication/output boundary and dedicated CLI consumer.
- T-C must localize and close the remaining `57.409871 mm` audit residual
  without weakening independent-operand acceptance.
- No `.unwrap()`/`.expect()` may be introduced in production paths.
- No silent canonicalization of invalid impoundment counts; zero is explicit
  empty-set semantics, not a fallback.
- Any future closure claim must include current-run evidence for
  `cargo fmt --check`, clippy, workspace tests, `cargo deny check`, and the
  package-specific totalwatsed3 audit gates.
