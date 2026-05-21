# Wave 3 Parser Worktree Execution Plan

Date: 2026-05-21
Status: Draft (INIMPL18)
Evidence mode: `Static` (with explicit `Ran` checks where noted)

## 1. Purpose

Define execution governance for Wave 3 watershed-core parser worktrees
(`INIMPL19..21`) and Wave 3 integration (`INIMPL22`) so concurrent
implementation can proceed without write-overlap conflicts, ambiguous ownership,
or non-deterministic integration order.

## 2. Scope

Included:
- Worktree/branch topology and naming policy for `INIMPL19..21`.
- Disjoint ownership boundaries and shared-file quarantine policy.
- Deterministic integration intake/ordering for `INIMPL22`.
- Wave 3 promotion gates and blocker criteria.

Out of scope:
- Surface-specific parser implementation code for Wave 3 workers.
- Wave 4 watershed sidecar parser implementation.

## 3. Authority Inputs

- [DIRECT] `/home/workdir/openWEPP/docs/planning/parser-implementation-order.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl01-prioritize-parser-implementation-order/artifacts/parser-implementation-wave-plan.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl01-prioritize-parser-implementation-order/artifacts/parser-implementation-priority-matrix.csv`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl17-wave2-sidecar-parser-integration-001/artifacts/inimpl17_disposition.md`
- [DIRECT] `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md`
- [DIRECT] `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md`
- [DIRECT] `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl19-implement-sc-infile-watershed-structure-parser-001/package.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl20-implement-sc-infile-watershed-channel-parser-001/package.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl21-implement-sc-infile-watershed-impoundment-parser-001/package.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl22-wave3-core-parser-integration-001/package.md`

## 4. Worktree Topology

| Package | Branch | Worktree path | Role | Provisioning status |
| --- | --- | --- | --- | --- |
| `INIMPL19` | `inimpl19/watershed-structure-parser` | `/home/workdir/openWEPP/.worktrees/inimpl19-watershed-structure` | watershed structure worker | provisioned |
| `INIMPL20` | `inimpl20/watershed-channel-parser` | `/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel` | watershed channel worker | provisioned |
| `INIMPL21` | `inimpl21/watershed-impoundment-parser` | `/home/workdir/openWEPP/.worktrees/inimpl21-watershed-impoundment` | watershed impoundment worker | provisioned |
| `INIMPL22` | `main` integration target | `/home/workdir/openWEPP` | wave integration coordinator | active |

`Ran` checks:
- `git worktree list --porcelain`
- `git branch --list 'inimpl1*' 'inimpl2*'`
- `ls -d .worktrees/*`

Observed state:
- `INIMPL19..21` branches/worktrees are provisioned from baseline commit
  `214f3f79837a51f393b38c5ebe1e84a5e1c08890`.
- Wave 2 integration is closed with `INIMPL17` recommendation `GO`.

## 5. Execution Phases

### Phase 0: Governance Freeze (INIMPL18)

1. Publish this plan and INIMPL18 artifact bundle.
2. Freeze Wave 3 ownership and shared-file quarantine rules.
3. Require all Wave 3 worker streams to treat this plan as authority.

### Phase 1: Shared Scaffold Baseline (Pre-worker coding gate)

[INFERENCE] All Wave 3 worker packages include
`crates/openwepp-input-contract/src/parsers/mod.rs` in intended write sets,
which is a direct overlap surface.

Required baseline outcomes before worker coding starts:
- shared parser module declarations/stubs for all Wave 3 watershed-core parsers,
- shared integration harness bootstrap for Wave 3 tests,
- baseline commit SHA recorded in Wave 3 branch registry.

Owner:
- integration/scaffold coordinator stream (`INIMPL18` preflight + `INIMPL22`
  integration authority).

### Phase 2: Parallel Worker Execution (INIMPL19..21)

1. Each worker edits only its owned parser/test/fixture namespace.
2. Workers treat shared quarantine files as no-direct-edit surfaces.
3. Strict/compat behavior and typed error taxonomy are implemented per assigned
   `SC-INFILE-*` contract.
4. Any needed shared-file mutation is recorded as a handoff request for
   integration intake.

### Phase 3: Integration Intake and Sequencing (INIMPL22)

1. Validate worker handoffs and ownership conformance.
2. Integrate/cherry-pick in canonical Wave 3 order.
3. Resolve conflicts under ownership policy with explicit logging.
4. Run Wave 3 gate suite only when intake prerequisites are satisfied.

## 6. Ownership and Conflict Policy

Normative ownership is defined in:
- `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl18-wave3-worktree-orchestration-001/artifacts/worktree-ownership-manifest.md`

Rules:
1. Worker write streams are disjoint at parser file, integration test file, and
   fixture-root levels.
2. Shared scaffolding files are quarantine-owned by integration/scaffold
   coordinator.
3. No worker edits another worker's parser, tests, fixtures, or package
   artifacts.
4. Science contracts/specs are read-only within Wave 3 implementation streams
   unless explicitly re-scoped.
5. Conflict priority: contract correctness > ownership compliance > minimal diff
   > velocity.

## 7. Wave 3 Promotion Gates

Required `Ran` gates for `INIMPL22` promotion:
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`
5. Watershed-core parser acceptance checks for strict/compat behavior and typed
   errors across all three Wave 3 surfaces.
6. No unresolved high-severity findings in worker/integration
   review-disposition-verification artifacts.

## 8. Hard Blocker Criteria

Wave 3 integration execution beyond intake is blocked when any condition holds:
1. Any of `INIMPL19..21` worktrees are not provisioned.
2. Shared scaffold baseline commit SHA is not recorded.
3. Any worker package is missing required handoff/disposition/verification
   artifacts.
4. Any worker package retains unresolved high-severity findings.
5. Branch/worktree registry diverges from actual local worktree topology.
6. Wave 2 integration closeout (`INIMPL17`) is not verified closed for shared
   parser surfaces.

## 9. Start Criteria for INIMPL19..21

`GO-WITH-AMENDMENTS` criteria:
1. INIMPL18 artifact bundle exists and is verified.
2. Shared scaffold baseline owner is explicit.
3. Shared scaffold baseline commit SHA is recorded.
4. Worker confirms no unauthorized edits outside owned write set.

## 10. Recommendation

Current recommendation: `GO`.

Rationale:
- Governance authority, ownership boundaries, and sequencing policy are
  defined.
- Worker streams are provisioned from a single recorded baseline and are ready
  for dispatch under Wave 3 governance controls.
