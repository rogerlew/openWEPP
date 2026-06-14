# Kernel Profile Compliance Checklist

Status: T-B2-REDO executed

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
- T-B2 did not edit hillslope kernel process physics; it added an optional
  output-surface publication from existing MOFE outlet transfer state.
- T-B2 preserves existing HBP/WAT output anchors byte-for-byte in the real
  arboreal-dendrite rerun.
- T-B2 was reviewed defective after execution because its MOFE PASS `runvol`
  used `QOFE * publication area`, which over-scaled runoff and made the
  acceptance audit self-consistent with the bug.
- T-B2-REDO did not edit hillslope kernel process physics; it corrected the
  output-surface volume dual to published `Q * Area / 1000`.
- T-B2-REDO preserves existing HBP/WAT output anchors byte-for-byte in the
  real arboreal-dendrite rerun (`anchor_mismatches=0`).
- T-B2-REDO proves the corrected annual water-year precipitation bound before
  handing the corrected native output to T-C.
- T-C must localize and close the remaining `6948.564523 mm` corrected-output
  audit residual without weakening independent-operand acceptance.
- No `.unwrap()`/`.expect()` may be introduced in production paths.
- No silent canonicalization of invalid impoundment counts; zero is explicit
  empty-set semantics, not a fallback.
- Any future closure claim must include current-run evidence for
  `cargo fmt --check`, clippy, workspace tests, `cargo deny check`, and the
  package-specific totalwatsed3 audit gates.
