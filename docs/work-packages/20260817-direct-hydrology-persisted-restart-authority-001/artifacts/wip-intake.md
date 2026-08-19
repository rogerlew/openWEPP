# WIP intake

Status: `executing / containment complete`

Ran: intake at superseding authorized baseline
`1cac432a4a5d2a0de87122bd68b69ab83cffe21a` on branch `main`.
`origin/main` matched and the worktree was clean. The operator confirmed
`/workdir/openWEPP` as the repository path and explicitly superseded the
original `093e172c5` starting-commit requirement with this commit.

Static: the only delta from `093e172c5008f8713f170445dff514e439961dda`
is the tracked research note
`references/deepresearch/richard-green-ampt-model-implementations.md`; it does
not alter the restart, forcing, V10/LSE-V2, or Child-4 implementation state.

The starting restart authority remains unreleased. The existing authority
target has only three draft documentation/shape tests. Those results are not
restart serialization, restoration, continuation, rollback, or poison-matrix
validation.

Implementation intent: critical contract-first restart authority followed by
critical persisted-state implementation. No selector/default/output change,
activation, deployment, calibration, external message, PR, branch, remote
branch, or push is authorized.
