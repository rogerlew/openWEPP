# Gate results

Status: `intake only / no restart validation yet`

Ran at the original draft checkpoint: the existing
`direct_hydrology_restart_authority_contract` reported 3/3 PASS. These are
draft authority documentation/schema-shape checks only. They do not execute
canonical checkpoint serialization, restoration, continuation, rollback, real
vectors, or the typed poison matrix and therefore are not restart validation.

Ran at superseding intake: branch, clean-tree, exact `HEAD`, and `origin/main`
checks PASS at `1cac432a4a5d2a0de87122bd68b69ab83cffe21a`.

All implementation, focused, authority-release, and terminal gates remain
`NOT RUN` for this resumed execution until their corresponding changes exist.

## HOLD-remediation intake and primitive reference increment

Ran at starting commit `bb3cc3a0ed...`:

- branch/ancestry/origin inventory: PASS;
- clean worktree and `git diff --check`: PASS;
- instruction discovery over every declared path: PASS;
- package-local reference crate unit tests: PASS, 6/6;
- package-local reference crate all-target Clippy with warnings denied: PASS;
- strict canonical parser tests cover reordered bytes, whitespace, duplicate
  members, and unknown members;
- primitive tests cover signed zero, exact lowercase widths, u32 overflow,
  negative day indices, and interval 48 rejection;
- the first actual runtime mapping (`DirectWaterState`) exhaustively
  destructures all six fields and round-trips their bits exactly.

Disposition remains `HOLD / REMEDIATION IN PROGRESS`. The remaining runtime
owners, artifact regeneration, complete poison matrix, and reviews are not yet
passed; production restart implementation remains forbidden.
