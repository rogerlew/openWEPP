# Implementation Evidence

Status: `PASS-CORRECTION`

Evidence class: **Ran + Static**. Focused interactive output is supporting and
unarchived; the exact release log is terminal consumer evidence.

The new guard rejects missing schema, ambiguous top-level source claims,
duplicate target items, wrong source paths, noncanonical repository/commit
metadata, and any disagreement among fixture, lock, provenance, or Git-object
SHA-256 values. Static comparison proves its red/green discrimination;
unarchived focused runs reported the expected red state and AUTH06 5/5 pass.

The production fixture JSON and lock remain byte-identical at
`a1c50a82cd1e497875cb034481c4b2ef710c319907480b0f584fde30f48fae5e`.
Only provenance metadata changed: schema version 1 was added, legacy ambiguous
top-level source claims were removed, and the fixture item now binds canonical
repository `/workdir/openWEPP` plus verified Git commit `9aa4c3d6…`.

The exact release independently accepted fixture integrity after full nextest
passed 1,946/1,946 and dependency policy passed. The touched Rust test file has
442 lines, below line-count governance thresholds.
