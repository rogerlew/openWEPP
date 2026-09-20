# Grid40 continuation QA review — baseline admission

Reviewer: `/root/grid40_qa`, independent QA/evidence scope.

Evidence class: **Ran** inspection and independent reconstruction of the
completed baseline receipt, frozen pair, trace, sidecar, JSONL event stream,
exact comparator result, and offline cost-reconstruction result.  No evaluator
was run in this review.

## Findings

No blocking QA finding.

The only baseline arm is durably claimed by receipt
`ac95a253af45e598f148657683f6740a307f26395cd07f41a2b7810f56f10d9b` under
unchanged pair freeze `a13048e5df6c41e034746c2d4d52b9468ab04a1aa03a6ce796491eceeaa2f901`.
It used the approved executable/selector and the bound T source, binary, and
original input identities.  It exited 0 in 0.556224868 seconds with no stop
reason or integrity error.  Receipt hashes match all actual retained child
files; the aggregate and each populated file remain inside their fixed bounds.

The strict baseline comparator passes all 95 ordinary records, schema, typed
values, binary64 bits, signed zeros, and record order, with no unreviewed or
diagnostic-probe record.  The trace's first `solver_input` is independently
typed/binary64 compared with the frozen capture; it retains R0, uncapped
Potential, V11SnowCovered, `ofe-1`/`forest`, and the original 29-coordinate
start.  The expected endpoint is **Rejected BacktrackingLimit/E034** at
iteration 6 and backtracking 65.  This is a matched baseline refusal, not
convergence.

The sidecar and JSONL stream hash to
`3ef900...30c91` and `0b34c6...70790`; their event arrays are identical.
The independent recount validates contiguous operation IDs, no error, denied,
or in-flight operation, and complete base/strict/jacobian/ordinary/complete
lifecycle joins.  Recounted costs are base 7, Jacobian assemblies 7, updates
6, signed probes 406 (98 identity shortcuts + 308 full evaluations), ordinary
13, complete 321, and domain predicates 1416, each below the frozen baseline
caps.  `ci_bracket_loop_iterations=0` describes that loop only; it is not used
to claim zero nested Ci work.

## Non-blocking debt and follow-up

- The inherited strict-Clippy FAIL remains unrelated to this matched baseline
  admission and remains visible from preparation QA.

## QA baseline admission GO

QA confirms `MATCHED_BASELINE_GO` for the exact hashes recorded in
`baseline-admission.json`.  The common pair freeze is unchanged and treatment
may consume its one remaining arm under that freeze.  This admission does not
provide a treatment outcome or scientific-success verdict; those require the
separate final result review.
