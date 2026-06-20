# R3A Disposition

Status: complete.
Evidence mode: Static + Ran.

Verdict: `COMPLETE-R3A-PHASE-SPAN`.

R3A implemented the first complete direct-runtime phase span:
direct transfer-input accounting over
`DirectPhaseKind::Normalization -> DirectPhaseKind::LateralTransfer`.

Closure evidence:

- selected span includes typed inputs, direct compute, state mutation,
  downstream operands, and shadow projection;
- phase-span identity passed with exact binary-fraction fixture evidence;
- no-compatibility proof passed by source scan, scheduler no-diff, and runtime
  counters;
- runtime counters are non-tautological, including one production opt-in
  compatibility-edge handoff and zero direct-span edge invocations;
- default-disabled H2637 median was `632.08 s`, under the `<= 676.67 s` gate;
- protected H2637 identity passed;
- full Rust closure gates, scoped markdown lint, and `git diff --check` passed;
- line-count governance passed with the established runner setup WARN-band file
  explicitly dispositioned;
- dual review and dual verification are complete.

Limits:

- no R4 hydrology-path migration claim;
- no R6 publication cutover;
- no endpoint-improvement claim;
- no default activation readiness claim.

Recommended follow-on:

- Scaffold R3B or R4A as a new package for the next direct phase span with
  canonical process authority. R3A only proves the direct span execution
  harness and evidence pattern.
