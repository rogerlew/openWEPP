# Grid40 continuation QA review — concrete pair-freeze candidate

Reviewer: `/root/grid40_qa`, independent QA/evidence scope.

Evidence class: **Static** candidate-freeze and write-boundary inspection, plus
**Ran** byte comparison and ignored-test listing recorded in
`continuation-binary-binding.json`.  No original arm, evaluator, or test body
was run for this review.

## Findings

No blocking QA finding.

`continuation-pair-freeze-candidate.json` binds T
`fae897fe90f6bbadcaf298607c9d23fd986cd778f9dd56195f7a5510cbd5c4be`, the
immutable input `527208...482dfd`, all nineteen external inputs, five support
links, the updated recorder `e34d...297ab`, the snapshot tool, baseline
comparator/input-bit/ledger/custody/recovery evidence, and the adopted
`2026-09-20T02:42:19.481118Z` deadline.  All thirteen currently bound local
files were independently rehashed against the candidate values.

The executable copy
`/home/roger/openwepp-experiments/b01-wb14-grid40-run-evidence-20260919/grid40-tests.launch`
is mode `0555`, byte-identical to the retained read-only binary, and hashes to
`e26cc780e7065211714a37653e55f9038cee3a5a32a6964de7122cd80cf0f272`.
Its isolated `--list` receipt finds exactly one ignored test:
`grid40_experimental_tests::grid40_original_start_replay_is_explicit_and_single_arm_only`.
The candidate uses that full selector with `--ignored --exact --test-threads=1`
and records the default-feature build identity.

The candidate has the reviewed exact eight slots and writer map:
`stdout`, `stderr`, `trace.json`, `trace.partial`, `trace.partial.next`,
`sidecar.json`, `events.jsonl`, and `reserved-2`; the three output environment
variables resolve only to `sidecar.json`, `events.jsonl`, and `trace.json` in
the arm directory.  It preserves `no_subprocess=true`,
`no_other_destinations=true`, the 120-second/16-GiB/64-MiB/1-GiB arm limits,
and the fixed 50/51/50/2900/20000 cost limits.  The trace input and arm selector
remain explicit environment values, not output writers.

## Permitted mechanical finalization

The candidate's only pending fields are its correctness and QA prerequisite
values and `write_boundary.review_sha256`.  QA approves replacing those exact
values with the completed independent correctness continuity GO, QA GO, and
the SHA-256 of `continuation-qa-review.md`; bind the corresponding immutable
review artifacts in `bound_files`.  No other candidate field, including source,
binary, input, selector, arguments, environment, slot list, limits, recorder,
deadline, or write-boundary predicates may change.  The resulting
`pair-freeze.json` must be byte-frozen before baseline and retained unchanged
for treatment.

## Non-blocking debt and follow-up

- Strict Clippy remains inherited-debt FAIL as stated in the preparation
  review; this candidate neither hides nor resolves that quality debt.

## QA pre-arm GO

QA approves this candidate and the permitted mechanical finalization above.
Once those named review bindings are filled exactly, it is a complete
source/binary/write-boundary pair freeze with no remaining QA preparation
condition before the single baseline arm.  This GO does not assess a baseline
or treatment result; the later result review remains required.
