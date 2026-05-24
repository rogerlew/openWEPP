# EROD11 Alias Ownership Authority Evidence

Status: `completed`
Evidence mode: `Static + Ran`

Static:
- Wave-0 alias ownership authority is now ratified in canonical contracts via
  explicit `EROD11 Alias Ownership Register` sections in:
  - `SC-SED-001`
  - `SC-HYDRAULICS-001`
  - `SC-ROUTE-001`
  - `SC-WATBAL-001`
  - `SC-RUNOFFPART-001`
- Alias-ambiguity gap posture is dispositioned in canonical gap registers:
  - `GAP-SED-002` -> `closed`
  - `GAP-HYD-002` -> `closed`
  - `GAP-ROUTE-002` -> `closed`
  - `GAP-WATBAL-003` -> `closed`
  - `GAP-RUNOFFPART-002` -> `closed`
- Registry notes in `docs/specifications/science-contracts/index.md` now
  reflect EROD11 alias-ownership closure posture.

Ran:
- Verified canonical edits with `rg`/`sed` line checks in the worktree.
- Verified typed runtime alias surfaces align with implemented symbol authority
  in `crates/openwepp-kernel-contract/src/lib.rs`.
