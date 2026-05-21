# Wave 2 Parser Worktree Execution Plan

Date: 2026-05-21
Status: Draft (INIMPL10)
Evidence mode: `Static` (with explicit `Ran` checks where noted)

## 1. Purpose

Define execution governance for Wave 2 sidecar parser worktrees (`INIMPL11..16`) and Wave 2 integration (`INIMPL17`) so concurrent implementation can proceed without write-overlap conflicts, ambiguous ownership, or non-deterministic integration order.

## 2. Scope

Included:
- Worktree/branch topology and naming policy for `INIMPL11..16`.
- Disjoint ownership boundaries and shared-file quarantine policy.
- Deterministic integration intake/ordering for `INIMPL17`.
- Wave 2 promotion gates and blocker criteria.

Out of scope:
- Surface-specific parser implementation code for Wave 2 sidecars.
- Wave 3 watershed parser implementation.

## 3. Authority Inputs

- [DIRECT] `/home/workdir/openWEPP/docs/planning/parser-implementation-order.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl01-prioritize-parser-implementation-order/artifacts/parser-implementation-wave-plan.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl01-prioritize-parser-implementation-order/artifacts/parser-implementation-priority-matrix.csv`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl09-management-full-typed-datamodel-001/`
- [DIRECT] `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md`
- [DIRECT] `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md`
- [DIRECT] `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md`
- [DIRECT] `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md`
- [DIRECT] `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md`
- [DIRECT] `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl11-implement-sc-infile-pmetpara-parser-001/package.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl12-implement-sc-infile-irrigation-depletion-parser-001/package.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl13-implement-sc-infile-irrigation-fixeddate-parser-001/package.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl14-implement-sc-infile-frost-parser-001/package.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl15-implement-sc-infile-snow-parser-001/package.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl16-implement-sc-infile-weppui-parser-001/package.md`
- [DIRECT] `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl17-wave2-sidecar-parser-integration-001/package.md`

## 4. Worktree Topology

| Package | Branch | Worktree path | Role | Provisioning status |
| --- | --- | --- | --- | --- |
| `INIMPL11` | `inimpl11/pmetpara-parser` | `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara` | pmetpara sidecar worker | provisioned |
| `INIMPL12` | `inimpl12/irrigation-depletion-parser` | `/home/workdir/openWEPP/.worktrees/inimpl12-irrigation-depletion` | depletion sidecar worker | provisioned |
| `INIMPL13` | `inimpl13/irrigation-fixeddate-parser` | `/home/workdir/openWEPP/.worktrees/inimpl13-irrigation-fixeddate` | fixed-date sidecar worker | provisioned |
| `INIMPL14` | `inimpl14/frost-parser` | `/home/workdir/openWEPP/.worktrees/inimpl14-frost` | frost sidecar worker | provisioned |
| `INIMPL15` | `inimpl15/snow-parser` | `/home/workdir/openWEPP/.worktrees/inimpl15-snow` | snow sidecar worker | pending provisioning |
| `INIMPL16` | `inimpl16/weppui-parser` | `/home/workdir/openWEPP/.worktrees/inimpl16-weppui` | wepp-ui sidecar worker | pending provisioning |
| `INIMPL17` | `main` integration target | `/home/workdir/openWEPP` | wave integration coordinator | active |

`Ran` checks:
- `git worktree list --porcelain`
- `git branch --list 'inimpl1*'`
- `ls -d .worktrees/inimpl1*`

Observed state: branches/worktrees for `INIMPL11..14` exist; `INIMPL15..16` must be provisioned before worker execution can start for those packages.

## 5. Execution Phases

### Phase 0: Governance Freeze (INIMPL10)

1. Publish this plan and INIMPL10 artifact bundle.
2. Freeze Wave 2 ownership and shared-file quarantine rules.
3. Require all Wave 2 worker streams to treat this plan as authority.

### Phase 1: Shared Scaffold Baseline (Pre-worker coding gate)

[INFERENCE] All Wave 2 worker packages include `crates/openwepp-input-contract/src/parsers/mod.rs` in intended write sets, which is a direct overlap surface.

Required baseline outcomes before worker coding starts:
- shared parser module declarations/stubs for all Wave 2 sidecars,
- shared integration harness bootstrap for sidecar tests,
- baseline commit SHA recorded in Wave 2 branch registry.

Owner:
- integration/scaffold coordinator stream (`INIMPL10` preflight + `INIMPL17` integration authority).

### Phase 2: Parallel Worker Execution (INIMPL11..16)

1. Each worker edits only its owned parser/test/fixture namespace.
2. Workers treat shared quarantine files as no-direct-edit surfaces.
3. Strict/compat behavior and typed error taxonomy are implemented per assigned `SC-INFILE-*` contract.
4. Any needed shared-file mutation is recorded as a handoff request for integration intake.

### Phase 3: Integration Intake and Sequencing (INIMPL17)

1. Validate worker handoffs and ownership conformance.
2. Integrate/cherry-pick in canonical Wave 2 order.
3. Resolve conflicts under ownership policy with explicit logging.
4. Run Wave 2 gate suite only when intake prerequisites are satisfied.

## 6. Ownership and Conflict Policy

Normative ownership is defined in:
- `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl10-wave2-worktree-orchestration-001/artifacts/worktree-ownership-manifest.md`

Rules:
1. Worker write streams are disjoint at parser file, integration test file, and fixture-root levels.
2. Shared scaffolding files are quarantine-owned by integration/scaffold coordinator.
3. No worker edits another worker's parser, tests, fixtures, or package artifacts.
4. Science contracts/specs are read-only within Wave 2 implementation streams unless explicitly re-scoped.
5. Conflict priority: contract correctness > ownership compliance > minimal diff > velocity.

## 7. Wave 2 Promotion Gates

Required `Ran` gates for `INIMPL17` promotion:
1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`
5. Sidecar parser acceptance checks for all six Wave 2 surfaces (strict/compat + typed-error coverage).
6. No unresolved high-severity findings in worker/integration review-disposition-verification artifacts.

## 8. Hard Blocker Criteria

Wave 2 integration execution beyond intake is blocked when any condition holds:
1. `INIMPL15`/`INIMPL16` worktrees are not provisioned.
2. Shared scaffold baseline commit SHA is not recorded.
3. Any worker package is missing required handoff/disposition/verification artifacts.
4. Any worker package retains unresolved high-severity findings.
5. Branch/worktree registry diverges from actual local worktree topology.
6. Upstream dependency packages required by worker contracts are not verified closed (`INIMPL09` for management-coupled sidecars; Wave 1 core parser baselines for climate/soil-coupled sidecars).

## 9. Start Criteria for INIMPL11..16

`GO-WITH-AMENDMENTS` criteria:
1. INIMPL10 artifact bundle exists and is verified.
2. Shared scaffold baseline owner is explicit.
3. Shared scaffold baseline commit SHA is recorded.
4. Worker confirms no unauthorized edits outside owned write set.

## 10. Recommendation

Current recommendation: `GO-WITH-AMENDMENTS`.

Rationale:
- Governance authority, ownership boundaries, and sequencing policy are defined.
- `INIMPL15..16` provisioning and shared scaffold baseline SHA capture remain required before full Wave 2 parallel execution.
