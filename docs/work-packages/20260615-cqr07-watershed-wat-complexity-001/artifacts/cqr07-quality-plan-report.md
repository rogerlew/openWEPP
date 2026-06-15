# CQR07 Quality Plan Report

Static: target file is `crates/openwepp-runner/src/watershed_wat.rs`.

Quality target:

- Remove the `read_batch_into` `#[allow(clippy::too_many_lines)]` suppression.
- Preserve WAT reader behavior, optional aliases/defaults, row-indexed errors,
  aggregation operands, and public API surfaces.
- Add focused reader characterization before decomposing production code.
- Keep new private helpers at or below the local CRAP threshold.

Execution plan followed:

1. Capture baseline line, coverage, and CRAP evidence.
2. Run existing focused tests.
3. Add direct `read_batch_into` characterization tests for alias/default/error
   behavior.
4. Extract private column and row helper structures/functions.
5. Remove the lint suppression.
6. Re-run focused checks, metrics, closure gates, reviews, and verification.

Ran: all required closure gates passed. Raw LCOV and CRAP outputs are stored as
`lcov_before.info`, `lcov_after.info`, `crap_before.json`, and
`crap_after.json`.

Disposition: complete-with-warnings because coverage and pre-existing
out-of-scope CRAP rows remain below the broader quality threshold.
