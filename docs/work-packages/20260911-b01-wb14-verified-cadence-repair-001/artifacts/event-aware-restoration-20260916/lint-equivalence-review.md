# Lint equivalence review — retained reader functions

Reviewer: `/root/event_qa` (independent QA). Scope is limited to the three
pre-existing `snow_stage3_v11_current_context_capture.rs` functions and their
related changed dead-code diagnostics requested by the orchestrator. No Rust or
lint command was run by this reviewer.

Evidence class: **Static, inspected Ran evidence.** I inspected the structured
baseline/candidate records in `lint-quality-orchestrator-comparison.json`, the
complete-target receipt, and source bodies in Q
(`/workdir/openwepp-experiments/b01-wb14-cadence/snowfree-recorder-candidate-20260915`)
and R (the derived reader). The comparison reports baseline 2,954, candidate
2,892, matched 2,849, and 43 candidate-unmatched occurrences. The receipt's
`target_coverage` records complete lib/libtest and example target profiles, and
`build_finished` records success. This review assesses the preserved structured
diagnostics.

## Findings

### Not equivalent — `import_and_reconstruct_pre_child_context`

Q has `clippy::too_many_lines` at 156 lines (multiplicity 2); R has the same
lint at 169 lines (multiplicity 2). This is not a source relocation: R adds
snow-free versus covered phase selection, changes the constraint owner from the
fixed covered owner to a selected owner, and calls the new
`reconstruct_provisional_on_clone_for_constraint_owner`. The lint's measured
body and relevant dependency surface changed. Resolve/refactor it in the final
cut; it cannot be credited as an inherited equivalent.

### Not equivalent — `restore_captured_pre_child_context`

Q has `clippy::too_many_lines` at 117 lines (multiplicity 2); R has it at 133
lines (multiplicity 2). R adds phase-kind discrimination and routes the
provisional reconstruction through the new owner-aware helper. This is a
material reader behavior change, not an equivalent diagnostic relocation.

### Not equivalent — `restore_provider_bound_context_from_rows`

Q has `clippy::too_many_lines` at 380 lines (multiplicity 2); R has it at 384
lines (multiplicity 2). R expands the target selector to admit the snow-free
pre-child row and incorporates the new reader path. The changed source/content
and changed line count preclude an equivalent-relocation attribution.

### Not equivalent — changed `RestoredStage3PublicationDayEvidenceV1` dead-code field list

Q's unmatched diagnostic says fields `entry`, `canonical_record`,
`parent_receipt`, and `qualification_delta` are unread. R's says only
`canonical_record` and `qualification_delta`. The changed field list is direct
evidence that R consumes two fields that Q did not; it is not the same
diagnostic. Either remove/use the remaining fields or retain an explicit
final-source explanation; do not mark it inherited solely because the struct
predates the package.

### New at the compared R cut — old reader helper dead code

The R-only unmatched list contains the owner-aware provisional helpers,
`validate_archive_first_positive_join`, archive authenticator,
prefix-membership decoder/reader functions, and the pre-child/member-export
entrypoints. None appears in Q's structured baseline-unmatched set. They are
new diagnostics at this comparison cut, even where their underlying reader
logic existed earlier. Some may disappear after final call-path wiring, but that
requires a final complete-target structured comparison; no equivalent relocation
is established here.

## Verdict

**No requested diagnostic qualifies as an equivalent relocation.** The three
old overlong reader functions and changed field-list diagnostic are materially
changed and need final resolution. Final lint evidence must preserve complete
target coverage explicitly and classify every remaining unmatched diagnostic by
lint, source content, and dependency surface rather than count.

## Refactored accessor follow-up

Evidence class: **Static, inspected Ran evidence.** In
`lint-refactored-orchestrator-comparison.json`, the sole remaining diagnostic in
`v9_real_consumer_shadow_v10_accessors.rs:92–100` is
`clippy::unnecessary_lazy_evaluations`. Q and R are byte-identical for this file
(SHA-256 `06fe849d…`), and the primary span, lint code, message, suggested
replacement, target metadata, and source excerpt are identical.

It is unmatched solely because the candidate structured message adds the child
note ``-D clippy::unnecessary-lazy-evaluations implied by -D warnings``. That
child reflects the candidate command's warning-denial presentation, not an
accessor source, type, primary span, or dependency-surface change. This is an
**inherited equivalent diagnostic**. Do not edit the byte-identical production
accessor solely to eliminate it; preserve this attributable classification in
the final matched-lint disposition.

The final audited orchestrator comparison retains exactly two occurrences of
this same presentation variant and no other candidate-unmatched diagnostic.
Its complete target coverage and successful build-finished record confirm that
the classification remains applicable to the final audited quality cut.

## Exact-tree terminal comparison

**Evidence class: Ran (inspected capped comparator records) and Static.** The
terminal tree `7c04f52e…` / patch `6f9f1cf0…` retains the preceding accessor
adjudication. `terminal-lint-orchestrator-comparison.json` has 2,829 candidate
occurrences, 2,827 matched, and only the two multiplicity copies of this
byte-identical accessor presentation variant. It is inherited, not introduced;
every target coverage record is complete and build-finished succeeds.

`terminal-lint-runner-comparison.json` matches all 202 candidate occurrences;
its complete target coverage and build-finished record also succeed. Both use
`--cap-lints warn`, so they establish diagnostic attribution only and do not
replace a strict Clippy gate.
