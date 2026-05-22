# PL04 Alias Template Validation Notes

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Alias template policy remains bounded to two tokens: `{ofe}` and `{idx4}`.
- Deterministic reverse lookup requires exact or single-template-match resolution.

Ran:
- Added/validated integration tests for invalid token rejection, template-string ambiguity rejection, and overlapping-template reverse ambiguity reporting.

## Template Semantics

1. `{ofe}`
- Must parse as positive integer (`>=1`).

2. `{idx4}`
- Must parse as exactly four ASCII digits (`0000`..`9999`).

3. Unsupported tokens
- Any token other than `{ofe}` or `{idx4}` is rejected at registry construction via `InvalidBoundaryAliasTemplate`.

## Guard Behavior

1. Duplicate template string across different canonical symbols
- Rejected at construction as `AmbiguousBoundaryAlias`.

2. Overlapping but non-identical template strings
- Construction allowed.
- Reverse lookup reports typed `AmbiguousBoundaryAlias` when a concrete alias matches more than one canonical template.

3. Malformed indices
- Non-4-digit or truncated index aliases fail lookup with `BoundaryAliasNotFound`.

## Representative Validation Assertions

- Invalid token: `ofe{bad}_xinput_{idx4}` -> `InvalidBoundaryAliasTemplate`.
- Duplicate template alias string across canonicals: `ofe{ofe}_shared_{idx4}` -> `AmbiguousBoundaryAlias`.
- Overlapping templates at lookup time: `ofe1_shared_0001` -> `AmbiguousBoundaryAlias`.
- Bad width index: `ofe5_rmogt_002` -> `BoundaryAliasNotFound`.
