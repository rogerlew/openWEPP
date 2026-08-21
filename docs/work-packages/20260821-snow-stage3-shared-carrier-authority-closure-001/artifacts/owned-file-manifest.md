# Owned-file manifest

Status: reconciled / terminal dual verification PASS

Evidence mode: Static

Initial intended write set:

- This package tree.
- `docs/work-packages/README.md` and `docs/ROADMAP.md`.
- Child 2B `artifacts/worker-handoff.md` and Child 2 campaign lifecycle files.
- The specifically amended canonical `SC-*` contract files and registry.
- Contract-derived tests and independent vectors only.
- Receipt schemas, valid/invalid fixtures, and restart/rollback vectors under
  the package artifact directory.
- `Cargo.toml` only for one explicit `[[test]]` registration; no dependency,
  feature, workspace, or production target changes.

Production Rust is excluded. Any path expansion requires a prospective package
amendment before edits.
