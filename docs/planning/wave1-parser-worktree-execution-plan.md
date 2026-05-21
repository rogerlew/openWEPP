# Wave 1 Parser Worktree Execution Plan

Date: 2026-05-21
Status: Draft (INIMPL02)
Evidence mode: `Static` (with explicit `Ran` checks where noted)

## 1. Purpose

Define the execution governance for Wave 1 parser implementation worktrees (`INIMPL03..06`) and Wave 1 integration (`INIMPL07`) so parallel delivery can proceed without hidden path overlap or merge-order ambiguity.

## 2. Scope

Included:
- Worktree/branch topology and branch naming policy.
- Worker write ownership and shared-file quarantine policy.
- Integration order, intake gates, and conflict policy.
- Wave 1 promotion gate criteria.

Out of scope:
- Parser implementation code for any `SC-INFILE-*` surface.
- Watershed parser implementation.

## 3. Authority Inputs

- [DIRECT] `/home/workdir/openWEPP/docs/planning/parser-implementation-order.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl01-prioritize-parser-implementation-order/artifacts/parser-implementation-wave-plan.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl01-prioritize-parser-implementation-order/artifacts/follow-on-parser-implementation-wp-queue.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl03-implement-sc-infile-slope-parser-001/package.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl04-implement-sc-infile-soil-parser-001/package.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl05-implement-sc-infile-climate-parser-001/package.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl06-implement-sc-infile-management-parser-001/package.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl07-wave1-core-parser-integration-001/package.md`

## 4. Worktree Topology

| Package | Branch | Worktree path | Role |
| --- | --- | --- | --- |
| `INIMPL03` | `inimpl03/slope-parser` | `/home/workdir/openWEPP/.worktrees/inimpl03-slope` | slope parser worker |
| `INIMPL04` | `inimpl04/soil-parser` | `/home/workdir/openWEPP/.worktrees/inimpl04-soil` | soil parser worker |
| `INIMPL05` | `inimpl05/climate-parser` | `/home/workdir/openWEPP/.worktrees/inimpl05-climate` | climate parser worker |
| `INIMPL06` | `inimpl06/management-parser` | `/home/workdir/openWEPP/.worktrees/inimpl06-management` | management parser worker |
| `INIMPL07` | `main` integration target | `/home/workdir/openWEPP` | integration and gates |

`Ran` check (local): `git worktree list --porcelain` shows all four worker worktrees provisioned from the same baseline commit.

## 5. Execution Phases

### Phase 0: Governance Freeze (INIMPL02)

1. Publish this plan and the INIMPL02 artifact bundle.
2. Freeze Wave 1 ownership rules before code edits.
3. Require worker packages to reference this plan as execution authority.

### Phase 1: Shared Scaffold Baseline (Pre-worker coding gate)

[INFERENCE] Worker write sets intentionally avoid shared scaffolding files; therefore a single baseline scaffold commit is required before worker parser coding starts.

Required shared baseline outcomes:
- Workspace member and crate skeleton for parser contract implementation.
- Placeholder parser module declarations for all Wave 1 surfaces.
- Placeholder integration-test harness structure so each worker can stay in disjoint files.

Ownership:
- Only integration coordinator stream (or designated scaffold owner) may edit shared scaffolding files listed in `worktree-ownership-manifest.md`.

### Phase 2: Parallel Worker Execution (INIMPL03..06)

1. Each worker edits only owned files.
2. Each worker records owned-file manifest and worker handoff artifact.
3. Strict/compat behavior and typed error taxonomy are implemented per assigned `SC-INFILE-*` contract.
4. Any required shared-file change becomes a handoff request, not an in-worker direct edit.

### Phase 3: Integration (INIMPL07)

1. Intake handoffs from `INIMPL03..06`.
2. Integrate in canonical order: slope -> soil -> climate -> management.
3. Resolve conflicts under ownership policy.
4. Run Wave 1 gate suite and publish `Ran` evidence.

## 6. Ownership and Conflict Policy

Normative ownership is defined in:
- `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl02-wave1-worktree-orchestration-001/artifacts/worktree-ownership-manifest.md`

Rules:
1. Worker branches are disjoint write-set streams.
2. Shared scaffolding files are quarantine-owned by integration/scaffold owner.
3. No worker may directly edit another worker's parser/test/fixture namespace.
4. Contract/spec docs are read-only for worker packages unless explicitly re-scoped by a new governance package.
5. Conflict resolution priority: contract correctness > ownership convenience > implementation speed.

## 7. Wave 1 Integration and Gate Policy

Integration sequence authority:
- `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl02-wave1-worktree-orchestration-001/artifacts/wave1-integration-sequence.md`

Required promotion gates (Wave 1):
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`
5. Wave 1 parser acceptance: strict/compat branch coverage and typed error taxonomy coverage for all four surfaces.
6. No unresolved high-severity review findings in worker or integration dispositions.

## 8. Blocker Criteria

The following are hard blockers for starting parser coding in `INIMPL03..06`:

1. Shared scaffold baseline commit is not present and referenced.
2. Worker ownership manifest is missing or ambiguous.
3. Branch/worktree registry is not aligned with actual branch paths.
4. Integration sequence authority is not published.

[INFERENCE] These blockers are governance blockers, not parser-contract blockers.

## 9. Start Criteria for INIMPL03..06

`GO-WITH-AMENDMENTS` criteria:
1. INIMPL02 artifact bundle exists and is dispositioned/verified.
2. Shared scaffold baseline owner is explicitly assigned.
3. A scaffold baseline commit SHA is recorded in INIMPL02 artifacts.
4. Each worker confirms write-set acceptance before first code commit.

## 10. Recommendation for INIMPL02 Package

Current recommendation: `GO-WITH-AMENDMENTS`.

Rationale:
- Governance artifacts and sequencing authority are defined.
- A pre-worker scaffold baseline commit is still required before coding starts.
