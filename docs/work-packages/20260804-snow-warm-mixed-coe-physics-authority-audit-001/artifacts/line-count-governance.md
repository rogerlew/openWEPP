# Line-Count Governance

Status: pass

Evidence mode: Ran

Terminal package-local Python line counts:

- `tools/audit_coe_authority.py`: `477` lines;
- `tools/test_audit_coe_authority.py`: `142` lines; and
- total: `619` lines.

Both are below the `.rs` governance warning threshold by analogy and remain
single-purpose audit tools. Ran: exact diff from base
`ae3f49a3684b3da35a35a2250ee362e147259b09` contains zero `.rs` paths, so no
2000-line warning or 3000-line nonexempt Rust refactor obligation is exposed.
