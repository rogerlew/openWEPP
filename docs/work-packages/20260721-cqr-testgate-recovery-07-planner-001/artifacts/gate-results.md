# Gate Results

Status: HOLD-CORRECTION-DEPENDENCY.

Ran: `/tmp/cqr-planner-baseline-hrNVaR` completed one instrumented traversal at
`71b89668d03c7824bddc40b6cb6d58978221ddac`: 137 pass, one fail, two ignored;
test time 230.69 seconds and wall time 254.66 seconds. The verifier READY-audit
test failed because the ambient prompt-only head contained zero changed
`package.md` authorities. RTR-029 owns the correction. No retry ran.

Ran: after implementation commit `d967d9d6`, a delegated matching-module
traversal began before implementation review B returned. At 3 minutes 3
seconds, review B found the graph characterization non-discriminating. The
coverage process group was terminated before it produced LCOV or CRAP results;
no metric claim is made. Its retained root is
`/tmp/cqr-planner-rank7-4Piluk7u`; log SHA-256 is
`c25be3ace3ec314f1feaeaecfb714f9377e13082dfbd0517581ff36438e8bf32`.
The exact 633 MB disposable target was removed after path validation. This
aborted obsolete-head traversal is not retried; a corrected changed head owns
the one final metric attempt.

Ran: the corrected graph-sensitive characterization passed on both the
pre-extraction implementation (`7c10da5e`) and the current extraction. No broad
or HEAVY gate ran.

Ran: the first complete corrected-head matching-module traversal ran once at
exact clean head `505638e7`. It retained evidence under
`/tmp/cqr-planner-final-l4D0CV8P`: 144 pass, zero fail, two ignored; 86.5694%
production line coverage; 83.1609% deduplicated region coverage; and no CRAP
row above 30. The extracted target was CRAP 20.2263 and `load_source_graph`
was CRAP 12. The result failed the 85% aggregate region gate and the 75%
function floor for `manifest_roots` and `require_node_semantics`; no unchanged
rerun was launched. LCOV SHA-256 is
`67efe235df6105382e38c8f3e174057f0fe2a7ca90b980ad6966e9172f3f008d`.

Ran: changed-head coverage characterization then passed 21/21 planner tests at
`36c4f126` in 79.782 seconds. Review B found that the first dirty-manifest
assertions combined independent mutations; the test was corrected into causal
staged snapshots and its exact focused test passed 1/1 at `d1f4e772`. Renewed
review A and B both pass that clean corrected head. No broad or HEAVY gate ran.
