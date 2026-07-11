# Line-count governance

Status: pass
Evidence mode: Static

`chaninp.rs` is 1,018 lines, the focused parser integration test is 931 lines,
and the touched WSHED-W5 integration test is 1,186 lines. Growth is attributable
to explicit raw/normalized ownership,
contract-derived A-H coverage, and named helper extraction. The source remains
one cohesive format parser; splitting it by record would obscure its ordered
whole-file invariants. No unrelated source was added.
