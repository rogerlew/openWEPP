# Verification Agent A

Status: complete

Evidence mode: Static + Ran

Final verdict: `PASS`.

The initial verification returned `PASS_WITH_FINDINGS` with one
closure-blocking low finding: the nearby Stage-3 closure note cited 21L's
tracked summary instead of its receipt-bound target result and rounded the
energy maximum incorrectly. The orchestrator accepted the finding, changed
the citation to
`target/snow_warm_mixed_prepeak_loss_energy_attribution_v2/results.json`, and
recorded exact independent maxima `2.6329172132452672e-17 m` and
`2.4985638447105885e-08 J m^-2`. Final recheck reproduced the target result's
receipt-bound SHA-256 `8ae39e3c...13e5` and both maxima. No blocker remains.

Independent verification also reproduced:

- all 18 frozen filesystem identities plus the pinned Git blob;
- the analyzer hash and four focused tests;
- `394705` eligible hours, `17431` site-days, and maximum residual
  `9.941202185450096e-18 m`;
- byte identity between the tracked quantitative result and a fresh terminal
  rerun;
- raw-versus-routed melt separation at every site;
- handbook, pinned legacy, Rust, and contract chronology;
- the narrowed validation/transferability rationale and
  `NOT_AUTHORIZED` production-correction boundary; and
- Markdown lint/validation, prompt identity, and diff hygiene.

The verifier agreed that non-selected Rust, workspace, comparator, release,
and anti-evasion gates are inapplicable to the exact documentation and
package-local analysis diff.
