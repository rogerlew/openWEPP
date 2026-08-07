# Rejected Execution V1

Status: `INVALID_EXECUTION / no evidence read / retained`.

Ran at clean `eacb1137836c5857f997328d9b274104a3171fad` through the required
comparator runner. Before invoking the analyzer, the runner created
`attempt-001/` for its own stdout/stderr/timing logs. The analyzer then exited
in `0.245 s` with status `1` at the immutable-output precondition:
`RuntimeError: refusing to overwrite immutable output`.

No trace hash, tuple, scientific metric, water-year result, or decision class
was read or produced. The failed namespace is retained and will not be deleted
or reused. Logs:

- `attempt-001/stdout.log`: `0` bytes,
  SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`;
- `attempt-001/stderr.log`: `774` bytes,
  SHA-256 `95298b9ab5f58c1c4ce4dde2f1c9faa11e492c5853d7ba469ae369ee7d5c4da5`;
- `attempt-001/command-timing.log`: `72` bytes,
  SHA-256 `1f1423d16cb13ee8f32feb4013fa83a857926f399b76b3bf1071e70b48d39447`.

Prospective recovery: use immutable `attempt-002` while placing runner logs in
a distinct sibling namespace. This changes no cohort, estimator, predicate,
authority, or scientific rule.
