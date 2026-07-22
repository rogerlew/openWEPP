# Implementation

Static: scaffold commit `3e9a1427` predates both Rust edits. No production file
changed.

Static: the planner graph-selection test retains all fixture inputs, Git/action
ordering, and assertions. Cohesive private helpers now own Git operations,
dependent package/workspace creation, lockfile generation, request construction,
graph assertions, and invalid-revision precedence. The `#[test]` body is 42
lines, down from 157.

Static: the verifier guard test retains all inputs and assertions. Its retry
attempt fixture/assertion block moved unchanged into
`assert_retry_attempt_guards`; the `#[test]` body is 75 lines, down from 101.
