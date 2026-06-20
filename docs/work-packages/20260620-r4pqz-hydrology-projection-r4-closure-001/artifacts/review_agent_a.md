# Review Agent A

Status: passed.

Static: local review, not delegated subagent work.

Findings:

- None requiring code changes.

Checks:

- Projection span includes inputs, direct compute, state mutation, downstream
  operands, and shadow projection.
- Required upstreams fail closed with typed `DirectRuntimeError` variants.
- Projection recomputes aggregate storage from final direct layer state rather
  than publication rows or stale scalar storage.
- Publication comparison fields are anti-aliased from direct operands and
  `public_output_cutover` remains `false`.
- No scheduler, output writer, output schema, or compatibility runtime cutover
  was introduced.
- Gate Evidence Non-Deferral Rule checked: package has concrete focused tests,
  full Rust gates, no-compat scan, scheduler no-diff, H2637 timing, and PASS
  equivalence evidence.

Residual risk:

- R4P/Q/Z is projection-only. Public WB13/WAT/PASS/loss authority remains
  compatibility-owned until R6.
