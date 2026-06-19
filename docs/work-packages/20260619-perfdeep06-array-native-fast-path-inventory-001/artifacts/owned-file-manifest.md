# PERFDEEP06 Owned-File Manifest

Status: queued.
Evidence mode: not-run.

## Writable Files

- `docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/architecture/array-native-runtime-specification.md` only for small
  consistency corrections discovered during planning.

## Read-Only Source Inventory

Populate during execution with the Rust files inspected for the working-set,
publication, and no-hot-loop-map proofs.

## Scope Rule

No production Rust code is writable unless `package.md` is amended before
implementation with an explicit bounded write set, contract-first gates, and
review of the new acceptance criteria.
