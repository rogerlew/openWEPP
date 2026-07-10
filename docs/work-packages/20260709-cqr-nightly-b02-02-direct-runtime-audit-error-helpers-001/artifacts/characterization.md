# Characterization Plan

Ran before decomposition: a detached worktree at scaffold commit `9a2bd314`
received the 20-case display characterization only (no production formatter
change). Command:

```text
CARGO_TARGET_DIR=/tmp/openwepp-cqr-b02-t02-predecomposition-target \
  cargo nextest run -p openwepp-hillslope-orchestrator --profile quick \
  -E 'test(cqr_predecomposition_display_characterization)'
```

Exit `0`: `1` passed, `333` skipped. The test rendered every
`DirectRuntimeError` variant and compared the complete diagnostic before the
formatter was decomposed. This is the direct downstream `Display` consumer and
includes `DirectDayExecutionFailure`'s one-based lane/day formatting.

The production target now carries the same 20 exact-output cases. The existing
serialized direct-runtime test seam additionally characterizes the two public
audit APIs; the target-local counter test characterizes the isolated specialized
counter paths. No test asserts a reformatted, normalized, or partial diagnostic.
