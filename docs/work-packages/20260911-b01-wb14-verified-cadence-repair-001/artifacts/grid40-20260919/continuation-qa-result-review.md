# Grid40 continuation QA review — pair result evidence

Reviewer: `/root/grid40_qa`, independent QA/evidence scope.

Evidence class: **Ran** inspection and independent recount of both retained arm
receipts, traces, sidecars, event streams, the common freeze, baseline
comparison/admission, and treatment cost reconstruction.  No evaluator was run
in this review.

## Findings

No blocking QA/evidence finding.

Both original-start slots are consumed exactly once under the same unchanged
freeze `a13048e5df6c41e034746c2d4d52b9468ab04a1aa03a6ce796491eceeaa2f901`.
The baseline receipt is `ac95...10d9b`; the treatment receipt is
`b317...cee2`.  Each binds T `fae897...c4be`, binary `e26c...f272`, input
`527208...82dfd`, the same ignored selector, and zero automatic retries.
Treatment exited 0 in 4.115309141 seconds without a stop or integrity error;
all receipt-file hashes, per-file bounds, aggregate bounds, and final trace /
partial equality check out.

Baseline remains admitted by its 95-record exact typed/binary64/order match and
the expected iteration-6 E034 refusal.  Treatment's complete trace contains
359 records (one original solver input, 15 accepted bases, 15 linear systems,
15 full trials, 312 strict attempts, and one terminal failure), with no
diagnostic probe.  Its source input is independently typed/binary64 equal to
the original R0, uncapped Potential capture.

Treatment's sidecar and JSONL hashes are `bad1...adca` and `6aee...ae9b`; their
event arrays are identical.  The independent recount validates contiguous
operation IDs, lifecycle closure, no errors, denial, budget stop, or in-flight
operation, and these costs below the frozen treatment ceilings: 15 bases, 15
Jacobian assemblies, 14 updates, 863 signed probes (210 identity shortcuts and
653 full evaluations), 29 ordinary calls, 682 complete calls, and 3185 domain
predicates.  `ci_bracket_loop_iterations=0` is retained with its limited
meaning and does not erase other nested Ci work.

The complete original-start treatment outcome is **Rejected
BacktrackingLimit** at iteration 14, backtracking count 297.  This is a valid
fixed-policy negative result with retained evidence.  It is not convergence,
production activation, reader/E008/RQ1/A-001 qualification, or proof that no
root exists.

## Non-blocking debt and follow-up

- T's source files were writable during baseline and were switched to `0444`
  before treatment.  `continuation-readonly-custody.json` retains that
  limitation and the 749-file no-byte-drift proof; it must remain visible in
  final custody reporting.
- Strict Clippy remains inherited-debt FAIL and is not resolved by this pair.

## QA result evidence PASS

QA accepts the pair's execution integrity, custody, trace/event lifecycle, and
cost evidence.  The measured result is a valid treatment refusal under the
frozen policy.  The separate correctness review owns branch-aware primitive
diagnosis and any scientific interpretation; no additional evaluator call is
authorized by this QA verdict.
