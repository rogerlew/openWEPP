# DFF WS-3A Wave-1/Wave-2 Sediment Production

Status: `queued`

Date opened: `2026-07-03`

Package type: defect-closure, science-contract, direct-runtime sediment
production, and WS-3 hold-lift package.

## Objective

Close `HOLD-DFF-WS3-SEDIMENT-PRODUCTION` by implementing proper
contract-backed Wave-1 and Wave-2 production in the direct runtime, then return
to WS-3 to assert disturbed-burn sediment ordering.

This package exists because WS-3 proved that a Wave-2 activation switch alone is
not sound: current production direct execution disables Wave-1 and seeds
`DirectErod13Inputs::zero()`, so downstream pass parquet sediment remains zero.

## Correction Authority Envelope

In scope:

- `SC-SED-001` contract-first amendments or clarifications required for
  production Wave-1/Wave-2 direct-runtime execution.
- Production direct-runtime seed authority for real EROD13 Wave-1 operands.
- Production EROD14 Wave-2 and downstream sediment publication paths needed to
  populate `tdet`, `tdep`, and `sedcon_*`.
- Focused tests proving real downstream HBP parquet consumers read the new
  path.
- WS-3 representative p1/p4 re-run, followed by enough matrix coverage to assert
  sediment ordering where physics produces sediment.
- Coordination with
  `../20260702-wshedw7dc01-hillslope-sediment-production-hold-lift-001/package.md`
  so the hillslope sediment-production hold is not solved twice in divergent
  ways.

Out of scope:

- Surrogate, proxy, empirical stand-in, or fixture-only sediment values.
- Runfile-only selectors that enable Wave-2 while Wave-1 production operands
  remain missing.
- Watershed routing/channel sediment changes unless required to prove the
  hillslope HBP consumer path.
- Re-litigating WS-2 `ksatadj` conductivity or WS-1 lanuse authority.

## First Actionable Item

Close the direct-runtime defect: identify the authoritative source for each
EROD13 Wave-1 input needed by `DirectErod13Inputs`, publish those inputs from
typed production state, and prove that `H4.pass.parquet` for the WS-3 p4
representative cell no longer has all-zero sediment solely because Wave-1 was
disabled.

## Required Evidence

- Contract lineage table for every Wave-1/Wave-2 operand added to production.
- Consumer-path proof from source operand to direct authority to HBP parquet
  `tdet`, `tdep`, and `sedcon_*`.
- Negative proof that no compatibility surface, zero seed, or runfile-only
  selector is carrying the acceptance claim.
- Focused unit/integration coverage for Wave-1 presence, Wave-2 ordering, and
  fail-closed missing operands.
- WS-3 representative p1/p4 before/after sediment output evidence.
- Full Rust closure loop before complete disposition:
  `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo nextest run --workspace --profile full`, and `cargo deny check`.

## Protected Boundaries

- No provisional sediment physics in production paths.
- No fabricated nonzero sediment to satisfy an ordering test.
- No fixture-specific branches.
- No package closure until the real downstream HBP parquet output proves the
  direct production path is live.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to `science_contract_reviewer`, `rust_code_reviewer`, `rust_qa_reviewer`, and
`comparator_suite_runner` subagents for contract review, implementation review,
verification, and comparator/matrix evidence. Expected outputs are compact
findings and artifact paths. Review and verification roles are read-only;
implementation remains in the parent unless a bounded write set is explicitly
assigned.
