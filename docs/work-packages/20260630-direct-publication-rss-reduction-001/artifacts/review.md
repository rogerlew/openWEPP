# Review

Evidence class: Static + Ran

## Review 1 - Correctness of Implemented Reduction

Finding: no issue with the implemented representation change.

The dominant allocation removed by this package was a typed setup vector that
the production direct executor did not need. Allowing empty constructor
`day_inputs` is safe only because the dynamic direct path already constructs
per-day inputs during execution; the validation change preserves the historical
full-vector form for tests and any caller that intentionally supplies prebuilt
day inputs.

The optional WAT/PASS projection change is also fail-closed: rows are built only
when requested, and a requested optional output errors if its rows are missing.
This avoids silently skipping output publication.

Identity evidence is strong for the measured H2637 surfaces: full-output
HBP/WAT/PASS/loss/plot bytes match the baseline, and minimized HBP/loss bytes
match the baseline.

## Review 2 - Closure Gate

Finding: package should not be marked complete.

The user-requested gate was not merely "reduce RSS"; it was to make direct
publication RSS roughly constant in run length. The package materially reduced
the slope by removing the `DirectDayConstructorInputs` setup vector, but it did
not eliminate whole-run retained publication rows. H2637 HBP/loss-only remains
`184644 KiB` while `cli01` is `19584 KiB`, and H2637 full-output remains
`316212 KiB` because requested WAT/PASS projection and parquet/Arrow buffers
still scale with row count.

The remaining route is a streaming direct publication sink that validates and
emits rows without retaining `DirectRunPublicationFrame.rows` for the whole run.
If parquet row-group chunking is needed to flatten full-output RSS, byte identity
may become the blocker because changing row groups commonly changes parquet file
bytes even when row contents are identical. That choice should be explicit in a
follow-on package.

## Review 3 - Gate Hygiene

Finding: full test gate is not green in this worktree.

Focused Rust tests, formatting, clippy, deny, authority anti-evasion, and
required-suite obligation guards passed. `cargo nextest run --workspace
--profile full` failed in `8` tests after `1848` passed. The failures were in
Python-backed harness/diagnostic tests and an environment check; the worktree
does not have `.venv/bin/python`. This is not evidence of an RSS regression, but
it is a blocking full-gate result for package closure.
