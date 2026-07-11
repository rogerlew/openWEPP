# Implementation

Status: PASS
Evidence mode: Ran

After the science-tier safety net passed, the WAT reader was decomposed into
typed column groups (`WatIdentityColumns`, `WatRequiredColumns`, and
`WatStorageColumns`) plus row/value helpers. The refactor preserves the exact
column lookup sequence, validation/error order, path/batch/row iteration,
area-lookup insertion point, optional fallback behavior, and floating
accumulation order. No public type, accepted input, schema, unit, formula, or
writer mapping changed.

Production SHA-256 changed from
`1b9d8d124bf34a3d5f9189eb901a2ac87ff89d51076a58632c596ec878e47ac9`
to `c31512f697a5867ae089b599a9131de1247069fa50da03dfaa96248f748530e0`.
Focused format, 17/17 nextest, targeted Clippy, terminal coverage, and CRAP all
pass.
