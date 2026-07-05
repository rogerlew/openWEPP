# Line-Count Governance Checklist

Status: passed
Evidence mode: Static

For every touched `.rs` file, record line count after edits:

- `<2000`: OK.
- `>=2000`: WARN with decomposition rationale and follow-on.
- `>=3000`: BLOCK unless generated/fixture exception is approved with owner
  and sunset plan.

Static:

- No `.rs` files were edited in D9.
- Line-count governance has no touched Rust file to disposition.
