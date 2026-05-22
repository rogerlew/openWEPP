# Wave 4 Parser Worktree Execution Plan

Date: 2026-05-22
Status: Draft (INIMPL23)
Evidence mode: `Static` (with explicit `Ran` checks where noted)

## 1. Purpose

Define execution governance for Wave 4 watershed-sidecar parser worktrees
(`INIMPL24..29`) and Wave 4 integration (`INIMPL30`) so concurrent
implementation can proceed without write-overlap conflicts, ambiguous
ownership, or non-deterministic integration order.

## 2. Scope

Included:
- Worktree/branch topology and naming policy for `INIMPL24..29`.
- Disjoint ownership boundaries and shared-file quarantine policy.
- Deterministic integration intake/ordering for `INIMPL30`.
- Wave 4 promotion gates and blocker criteria.
- Required strict/compatibility checks mapped from ratified `W4DR-001..012`.

Out of scope:
- Surface-specific parser implementation code for Wave 4 workers.
- Non-parser kernel/orchestrator implementation.

## 3. Authority Inputs

- [DIRECT] `/home/workdir/openWEPP/docs/planning/parser-implementation-order.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl01-prioritize-parser-implementation-order/artifacts/parser-implementation-wave-plan.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl22-wave3-core-parser-integration-001/artifacts/inimpl22_disposition.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260522-arch13-wave4-hold-ratification-checklist-001/artifacts/wave4-hold-ratification-checklist.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260522-arch13-wave4-hold-ratification-checklist-001/artifacts/wave4-kickoff-acceptance-criteria.md`
- [DIRECT] `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md`
- [DIRECT] `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md`
- [DIRECT] `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-GWCOEFF-001.md`
- [DIRECT] `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PHOSPHORUS-001.md`
- [DIRECT] `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TCR-001.md`
- [DIRECT] `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-LCWB-001.md`

## 4. Worktree Topology

| Package | Branch | Worktree path | Role | Provisioning status |
| --- | --- | --- | --- | --- |
| `INIMPL24` | `inimpl24/chaninp-parser` | `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp` | `chan.inp` worker | provisioned |
| `INIMPL25` | `inimpl25/tc-parser` | `/home/workdir/openWEPP/.worktrees/inimpl25-tc` | `tc.txt` worker | provisioned |
| `INIMPL26` | `inimpl26/gwcoeff-parser` | `/home/workdir/openWEPP/.worktrees/inimpl26-gwcoeff` | `gwcoeff.txt` worker | provisioned |
| `INIMPL27` | `inimpl27/tcr-parser` | `/home/workdir/openWEPP/.worktrees/inimpl27-tcr` | `tcr.txt` worker | provisioned |
| `INIMPL28` | `inimpl28/phosphorus-parser` | `/home/workdir/openWEPP/.worktrees/inimpl28-phosphorus` | `phosphorus.txt` worker | provisioned |
| `INIMPL29` | `inimpl29/lcwb-parser` | `/home/workdir/openWEPP/.worktrees/inimpl29-lcwb` | `lcwb.txt` worker | provisioned |
| `INIMPL30` | `main` integration target | `/home/workdir/openWEPP` | wave integration coordinator | active |

`Ran` checks:
- `git worktree list --porcelain`
- `git branch --list 'inimpl*'`
- `ls -d .worktrees/*`
- `git rev-parse HEAD`

Observed state:
- Existing Wave 1..3 branches/worktrees are present.
- Wave 4 branches/worktrees (`INIMPL24..29`) are provisioned from
  `e7f5cf2498aa434c43b0f3bfa2fc68f08e998f0f`.
- Repository `HEAD` observed during governance authoring:
  `e7f5cf2498aa434c43b0f3bfa2fc68f08e998f0f`.

## 5. Execution Phases

### Phase 0: Governance Freeze (INIMPL23)

1. Publish this plan and INIMPL23 artifact bundle.
2. Freeze Wave 4 ownership and shared-file quarantine rules.
3. Require all Wave 4 worker streams to treat this plan as authority.

### Phase 1: Shared Scaffold Baseline (Pre-worker coding gate)

[INFERENCE] All Wave 4 workers require shared parser registry/harness wiring,
which is a direct overlap surface (`parsers/mod.rs`, crate exports, root
integration wiring).

Required baseline outcomes before worker coding starts:
- parser module declarations/stubs for all Wave 4 sidecar parser files,
- integration test target declarations for all six Wave 4 surfaces,
- baseline commit SHA recorded in Wave 4 branch registry,
- strict/compat policy IDs agreed and pinned in worker kickoff prompts.

Owner:
- integration/scaffold coordinator stream (`INIMPL23` preflight + `INIMPL30`
  integration authority).

### Phase 2: Parallel Worker Execution (INIMPL24..29)

1. Each worker edits only its owned parser/test/fixture namespace.
2. Workers treat shared quarantine files as no-direct-edit surfaces.
3. Strict/compat behavior and typed error taxonomy are implemented per assigned
   `SC-INFILE-*` contract and ratified `W4DR` decisions.
4. Any needed shared-file mutation is recorded as a handoff request for
   integration intake.

### Phase 3: Integration Intake and Sequencing (INIMPL30)

1. Validate worker handoffs and ownership conformance.
2. Integrate/cherry-pick in canonical Wave 4 order.
3. Resolve conflicts under ownership policy with explicit logging.
4. Run Wave 4 gate suite only when intake prerequisites are satisfied.

## 6. Ownership and Conflict Policy

Normative ownership is defined in:
- `/home/workdir/openWEPP/docs/work-packages/20260522-inimpl23-wave4-worktree-orchestration-001/artifacts/worktree-ownership-manifest.md`

Rules:
1. Worker write streams are disjoint at parser file, integration test file, and
   fixture-root levels.
2. Shared scaffolding files are quarantine-owned by integration/scaffold
   coordinator.
3. No worker edits another worker's parser, tests, fixtures, or package
   artifacts.
4. Science contracts/specs are read-only within Wave 4 implementation streams
   unless explicitly re-scoped.
5. Conflict priority: contract correctness > ownership compliance > minimal diff
   > velocity.

## 7. Wave 4 Promotion Gates

Required `Ran` gates for `INIMPL30` promotion:
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`
5. Wave 4 parser acceptance checks for strict/compat behavior and typed errors
   across all six Wave 4 surfaces.
6. No unresolved high-severity findings in worker/integration
   review-disposition-verification artifacts.

## 8. Required W4DR Gate Mapping

All ratified decisions must be evidenced in Wave 4 implementation or
integration artifacts:

1. `W4DR-001`: source-authority policy is consistent across all six surfaces.
2. `W4DR-002`: strict hard-fail vs compat collapse-with-warning branches are
   fixture-backed where applicable.
3. `W4DR-003`: parser vs output/consumer ownership boundaries are preserved.
4. `W4DR-004`: `ichout` strict domain and compatibility normalization are
   explicitly tested.
5. `W4DR-005`: `dtchr` strict reject and compat normalization branches are
   explicitly tested.
6. `W4DR-006`: `cbase` guard semantics and consumer closure checks are tracked.
7. `W4DR-007`: `gwcoeff` optional-absence branch and malformed present-file
   typed failures are tested.
8. `W4DR-008`: namespace separation across `gwcoeff` and `chan.inp` is
   preserved with alias-guard evidence.
9. `W4DR-009`: `phosphorus` bounded-range and watershed-coupled applicability
   semantics are fixture-backed.
10. `W4DR-010`: `tcr` strict bounds and compatibility producer-edge handling
    are fixture-backed.
11. `W4DR-011`: `lcwbflg` authority handling aligns to current-source policy.
12. `W4DR-012`: `tc_out` row grammar ownership is not pulled into parser scope.

## 9. Hard Blocker Criteria

Wave 4 integration execution beyond intake is blocked when any condition holds:
1. Any of `INIMPL24..29` worktrees are not provisioned.
2. Shared scaffold baseline commit SHA is not recorded.
3. Any worker package is missing required handoff/disposition/verification
   artifacts.
4. Any worker package retains unresolved high-severity findings.
5. Branch/worktree registry diverges from actual local worktree topology.
6. Wave 3 integration closeout (`INIMPL22`) is not verified closed for shared
   parser surfaces.

## 10. Start Criteria for INIMPL24..29

`GO` criteria:
1. INIMPL23 artifact bundle exists and is verified.
2. Shared scaffold baseline owner is explicit.
3. Shared scaffold baseline commit SHA is recorded.
4. Worker confirms no unauthorized edits outside owned write set.
5. Worker package explicitly references the required `W4DR` gate subset.

## 11. Recommendation

Current recommendation: `GO`.

Rationale:
- Governance authority, ownership boundaries, sequencing policy, and W4DR gate
  mapping are defined.
- Wave 4 branches/worktrees are provisioned from one recorded baseline and are
  ready for concurrent worker dispatch.
