# Worker Handoff

Status: local hold ready for commit.

First actionable follow-on: close defect `CHN-E006-EXTRA-RATING-ROW` in a
dedicated parser defect-closure package. Begin with a one-channel
`icntrl != 4` fixture followed immediately by a three-float rating triple and a
regression requiring `RatingCurveClosure` / `CHN-E-006`; cover two- and
four-token arity and analyze multi-channel ambiguity so numeric comment text is
not misclassified. Confirm or ratify the recognition rule because the owning
science contract is draft/in-review, then implement the precise typed-error
classification.

After this hold commit, continue nightly batch 01 with target 04,
`crates/openwepp-kernel-contract/src/lib_mod/writeback.rs`.
