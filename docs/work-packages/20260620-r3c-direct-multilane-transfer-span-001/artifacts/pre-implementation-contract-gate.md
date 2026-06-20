# Pre-Implementation Contract Gate

Status: pass.
Evidence mode: Static.

R3C is authorized as an architecture/runtime span only. It does not require a
canonical `SC-*` amendment because it does not migrate process physics,
publication semantics, unit metadata, or output meaning.

Current-scope contract checks:

| Check | Result | Evidence |
|---|---:|---|
| Span selected before Rust edits | PASS | `r3c-span-contract.md` records the selected phases and operands. |
| Hydrology-process migration excluded | PASS | `package.md` excludes WB11/WB12/WB14/WB17/WB18/WB19 equations. |
| Publication cutover excluded | PASS | `package.md` excludes output schema/manifest/publication edits. |
| Compatibility calls excluded | PASS | Exit criteria require source scan and runtime counters. |
| Default activation excluded | PASS | R3C remains opt-in through existing direct skeleton selection only. |
