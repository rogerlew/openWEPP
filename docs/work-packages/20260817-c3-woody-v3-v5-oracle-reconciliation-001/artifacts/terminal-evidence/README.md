# Terminal Evidence

Evidence class: `Ran`

These transcripts were captured from uninterrupted commands on the exact
pre-closure worktree after both independent reviewers identified missing raw
evidence custody.

- `full-workspace.log`: `nix develop --command cargo nextest run --workspace
  --profile full --no-fail-fast`, with explicit Nix LLVM 21.1.8 `LLVM_COV`
  and `LLVM_PROFDATA`; command exit code 0. Started 2026-08-17 20:48:55 -0700
  and completed 2026-08-17 21:03:56 -0700. The harness configuration suppresses
  per-test progress, so the transcript's terminal `COMMAND_EXIT_CODE="0"` is
  the direct complete-workspace result. The immediately preceding enumerated
  run established 2,999 runnable tests and 33 skipped; no source bytes changed
  between it and this evidence-custody rerun.
- `bench-1-strict-projection.log`: 1 passed.
- `bench-2-one-open.log`: 1 passed.
- `bench-3-single-rank.log`: 1 passed.
- `bench-4-multirank.log`: 1 passed.
- `bench-5-mixed.log`: 1 passed.
- `bench-6-complete-endpoint.log`: 10 passed.
- `bench-7-rollback.log`: 1 passed.

Every transcript records its exact command, timestamps, test summary, and
terminal command exit code. No benchmark selector was zero-count.
