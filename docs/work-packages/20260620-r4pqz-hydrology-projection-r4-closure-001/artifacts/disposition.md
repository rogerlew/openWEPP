# Disposition

Status: complete.

Verdict: COMPLETE-R4PQZ-HYDROLOGY-PROJECTION-R4-CLOSURE.

Disposition:

- Accepted. R4P/Q/Z adds a shadow-only direct hydrology projection span that
  recomputes aggregate storage from final direct layer state and assembles the
  direct-owned hydrology projection operands required for R4 closure.
- Accepted. The span includes typed inputs, direct compute, state mutation,
  downstream operands, and shadow projection.
- Accepted. Missing upstreams and invalid projection domains fail closed with
  typed errors.
- Accepted. Public output authority remains compatibility-owned; no
  WB13/WAT/PASS/loss/schema cutover occurred.
- Accepted. No-compatibility proof, focused tests, full Rust gates,
  default-disabled H2637 median, and protected PASS equivalence passed.

Follow-on:

- Record pushed commit SHA in `docs/work-packages/r4-burndown-execplan.md`
  after push.
- Plan the next stage as R5 full OFE-day direct path / endpoint activation
  readiness before R6 publication cutover.
