# Line-Count Governance

The target changed from `1485` to `2393` physical lines (`+908`). This crosses
the repository's `2000`-line WARN threshold but remains below the `3000`-line
refactor blocker.

The increase is bounded to private cohesive parser helpers and target-local
characterization of all datver, typed-error, quote/token, ordering, restrictive
layer, and fail-closed parser branches. No adjacent module was moved or grown,
and no new lint suppression was introduced. The pre-existing module-level
`too_many_lines` allowance is unchanged.

Follow-on split intent: if this parser needs further behavior changes, evaluate
moving cohesive test fixtures or a contract-neutral parser submodule in a new
authorized package before it reaches the 3000-line blocker. This CQR package
does not make that broader boundary change.
