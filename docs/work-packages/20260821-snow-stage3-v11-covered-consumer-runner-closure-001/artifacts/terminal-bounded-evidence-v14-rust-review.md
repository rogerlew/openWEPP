# V14 Rust/API/private-compilation/noninterference review

Disposition: `HOLD`.

Both frozen hashes match. Blocking findings:

- the literal trait names nonexistent `crate::v11_covered`; the live type is
  re-exported at
  `crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1`;
- `Option<FloorAdmissionDraftV14>` cannot retain duplicate drafts for later
  validation while finalization remains infallible;
- fixed `[SelectedTrialV14; 3]` cannot represent the omission poison;
- several claimed canonical collection/soil digests have no defined framing
  primitive or exact source expression;
- no provider ordinal binds the final selected pair to
  `call_count_through_final_pair`;
- caller-visible after-state is observable, but attempted local clones dropped
  on error are not; the snapshot contract conflates them.

No source expansion is authorized.
