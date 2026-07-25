# Selection Parity

Evidence class: Ran.

The fixture retained one exact raw/actionable row for
`crates/openwepp-sim-contract/src/lib.rs`. Intake independently:

- validated the declared six-field canonical deduplication key;
- rejected the same key repeated under a different crate alias;
- reconstructed the current registry partition;
- grouped the actionable row by production module;
- computed excess CRAP, function count, and maximum CRAP; and
- returned the same module and exact row in `candidate_selection`.

The receipt records `selection_review_status=REQUIRED`. It does not finalize
the operator's requested `N` before the ExecPlan's two selection reviews.
