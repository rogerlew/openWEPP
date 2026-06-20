# Pre-Implementation Contract Gate

Status: pass.
Evidence mode: Static.

R4A is authorized as a narrow direct runtime migration under existing
`SC-RUNOFFPART-001` authority.

| Check | Result | Evidence |
|---|---:|---|
| Process span selected before Rust edits | PASS | `r4a-process-span-contract.md` records phases, operands, and formula. |
| Canonical authority exists | PASS | `SC-RUNOFFPART-001` owns event partition closure, runoff handoff, saturation addback, local-liquid handoff, and storage-cap domains. |
| Contract amendment required | PASS | No amendment required; R4A migrates an existing closure slice and does not change science authority. |
| Operand lineage recorded | PASS | `operand-lineage.md` records units, basis, authority, and status. |
| Publication cutover excluded | PASS | `package.md` excludes output schema/manifest/publication edits. |
| Compatibility calls excluded | PASS | Exit criteria require source scan and runtime counters. |
| Default activation excluded | PASS | R4A remains opt-in through existing direct skeleton selection only. |
