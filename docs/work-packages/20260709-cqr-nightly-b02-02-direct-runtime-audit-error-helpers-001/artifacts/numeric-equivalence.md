# Error and Numeric Equivalence

Static: `DirectRuntimeError` carries no process calculation. The only arithmetic
in its formatter is the pre-existing `lane_index + 1` and `day_index + 1` in
`DirectDayExecutionFailure`; those expressions remain unchanged in source order
and grouping.

The characterization suite compares the full rendered text for all 20 current
error variants. This proves exact typed-diagnostic identity for identity,
topology, publication, index, domain, kernel-guard, sink, closure, and day
execution failures. The source diff contains no changed diagnostic literal;
the strings moved only into private display representations.

The original formatter slice from scaffold commit `9a2bd314` is retained only
as untracked comparison material at `/tmp/openwepp-cqr-b02-t02-original-display.rs`
(SHA-256 `e530f71fe1cf0067a29eb59ab9466558a3ca1de2dd3a145fb3b6abde04124f9b`).

Ran: the following source-literal comparison exited `0` with no diff, comparing
the scaffold formatter literals to production lines before the current
`#[cfg(test)]` boundary:

```text
diff -u \
  <(git show 9a2bd314:crates/openwepp-hillslope-orchestrator/src/direct_runtime/04_audit_error_helpers.rs \
    | sed -n '287,430p' | rg -o '"direct (runtime|publication)[^"]+"' | sort -u) \
  <(sed -n '1,749p' crates/openwepp-hillslope-orchestrator/src/direct_runtime/04_audit_error_helpers.rs \
    | rg -o '"direct (runtime|publication)[^"]+"' | sort -u)
```
