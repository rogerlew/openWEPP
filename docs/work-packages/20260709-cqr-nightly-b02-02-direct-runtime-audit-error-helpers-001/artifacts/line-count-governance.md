# Line-Count Governance

The target changed from `538` to `916` physical lines (`+378`). It remains below
the repository's `2000`-line WARN threshold and the `3000`-line refactor
blocker. The increase is bounded to private display representation types and
20 exact-output characterization cases; no adjacent module moved or grew.

The old public formatter's local `clippy::too_many_lines` suppression was
removed. The new conversion and each private display formatter pass strict crate
clippy without new suppressions.
