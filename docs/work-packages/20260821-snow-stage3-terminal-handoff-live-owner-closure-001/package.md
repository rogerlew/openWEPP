# Stage-3 Terminal Handoff Live-Owner Closure

Status: `EXECUTED HOLD / ordinary-scheduler attachment and publication batching implemented / actual Stage-3/V11 coupled terminal chronology and typed owner custody remain unimplemented`

Date: `2026-08-21`

Package ID: `20260821-snow-stage3-terminal-handoff-live-owner-closure-001`

Plan class: `critical contract-first default-off kernel closure`

## Objective

Resume from `56f39487109022f7b01cb18978fa2c5076a285dd` as a fresh Child-1
successor. Preserve
`20260821-snow-stage3-shared-carrier-terminal-handoff-implementation-001` as
an immutable `EXECUTED HOLD`. Replace its caller-built request and owner
executor seam with one persistent, default-off Stage-3/V11 shadow attachment
that the ordinary scheduler advances internally.

The closure must bind every caller field to an actual sealed, static, or staged
owner; consume terminal events; derive carrier inputs from sealed forcing and
live owners; construct the exact INV-010 liquid parcel; execute the INV-011
remaining WB14 continuation and sequential V11/LSE/hydrology owners; build one
complete candidate; install it once; round-trip additive restart before, at,
and after the event; and build immutable publication batches without a row
callback inside the owner transaction.

## Protected boundaries

This package does not reopen shared-carrier, V11, coupled-time, LSE-support,
root-hydraulic, or persisted-restart science authority. It does not change
selectors, defaults, CoE ownership, production outputs, or deployment. The
shadow remains default-off and publication remains outside production owner
activation.

The stale `SC-SNOWENERGY-001` version guard is corrected in the contract test
to the released contract version; the released contract body is not changed.

## Prospective owner binding

| Caller field / old DTO surface | Intended owner / required source | Class | Binding rule | Implementation audit |
|---|---|---|---|---|
| wind, transfer height, roughness | sealed Stage-3 exposure receipt | sealed forcing | exact `5 m`, `0.005 m`, provider/source identity | scaffold receipt exists; live owner custody not independently proven |
| air temperature, humidity, longwave | sealed half-hour forcing receipt | sealed forcing | exact provider receipt and interval identity | scaffold forcing exists; coupled consumer path not proven |
| canopy temperature/humidity/conductance | staged V11 canopy owner and static configuration | staged/static | derived from current live V11 owner; no raw conductance input | FAIL: configured receipts are still accepted; actual V11 stack is not invoked |
| snow temperature/state and mass ledgers | persistent Stage-3 shadow owner | staged | current interval beginning/ending state and accepted receipts | FAIL: configured event/state projection is accepted rather than solved from Stage 3 |
| active participants/supports | coupled-time support receipts | sealed/staged | reconstructed from current segment and event boundary | partial: support receipt plumbing exists; actual multi-owner chronology is absent |
| event proposal/ticks/tolerances | Stage-3 terminal solver and coupled clock | static/staged | scheduler wall cursor and contract tolerances; no caller event DTO | FAIL: configured event tick is accepted; persistent terminal solve is not called |
| terminal liquid mass | Stage-3 event receipt plus snow retained-liquid owner | staged | retained liquid + support rain + terminal melt - refreeze | partial: INV-010 parcel path is exercised by scaffold scenarios only |
| terminal liquid temperature/enthalpy | SC-SURFACELIQUID-001 identity | static | `273.15 K`, zero sensible enthalpy relative to `T_ref` | PASS for the admitted parcel identity |
| WB14 parameters and continuation | surface-liquid configuration and live hydrology/WB14 owner | static/staged | actual configured lane, soil layer, cumulative state, exact remainder | PASS for bounded surface-liquid continuation; not a complete coupled continuation |
| V11/LSE/BGC/soil-thermal endings | ordinary scheduler's persistent shadow owner set | staged | one sequential candidate from live owners; no caller payload | FAIL: debug-derived payloads are hashed into a synthetic chain |
| rows/publication | immutable scheduler publication batch | staged | rows are consumed only after owner transaction returns successfully | PASS for the immutable post-commit batch boundary |

## Included scope

- persistent default-off attachment on the ordinary direct scheduler;
- internal carrier/event/parcel/continuation construction and fail-closed joins;
- complete owner candidate staging and one consuming installation;
- additive restart and replay protection at all event positions;
- immutable publication batch construction and late failure rollback;
- source-level negative guards proving the old caller-built APIs are not the
  ordinary closure path;
- contract-derived tests for mandatory event, continuation, restart, replay,
  failure, publication, and dual-terminal scenarios;
- correction of stale version assertions and required Rust line-count split.

## Excluded scope

- released science-contract amendments or new science authority;
- selector/default changes, CoE retirement, production output changes,
  deployment, calibration, efficacy, or cutover;
- reopening any protected science surface named above;
- canopy-intercepted snow or unsupported exposure providers.

## Intended write set

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/`
- `crates/openwepp-hillslope-orchestrator/src/snow_stage3_terminal_handoff.rs`
- `crates/openwepp-hillslope-orchestrator/src/v11_vegetation_consumer.rs`
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs` and
  its split test/module files as required by line-count governance
- `crates/openwepp-persisted-restart-v1/src/`
- `crates/openwepp-runner/src/hillslope/`
- package-owned integration tests under `tests/`
- stale contract-version test guard and package documentation/evidence only

No source path outside this write set may be added without a package amendment.

## Phase plan

1. Freeze owner bindings, exact write set, contract gate, and conservation
   operand lineage before implementation.
2. Implement the persistent attachment and ordinary scheduler internal path;
   remove banned DTOs from the ordinary closure API.
3. Implement terminal parcel, actual WB14 continuation, sequential owner
   execution, complete candidate installation, restart/replay, and immutable
   publication batching.
4. Add and run mandatory scenarios, late-failure injection, dual-terminal and
   independent closure/reconstruction tests.
5. Split every nonexempt Rust file above the repository ceiling, run focused,
   quick, domain, heavy, and exact-head checks, perform dual review and dual
   verification, reconcile the terminal diff, and disposition the package.

## Exit criteria

- ordinary scheduler consumes one persistent default-off attachment internally;
  no ordinary API accepts `SharedCarrierInput`, `SnowCarrierLedgerInput`,
  `TerminalStateRates`, `SnowStage3TerminalHandoffRequest`,
  `DirectV11SnowStage3OwnerExecutor`, raw conductances, raw carrier ledgers,
  or raw owner payloads;
- all caller fields have prospective owner bindings and actual sealed/static/
  staged construction evidence;
- terminal event, exact INV-010 parcel, INV-011 WB14 continuation, sequential
  V11/LSE/hydrology execution, complete candidate, and one installation are
  real and fail closed;
- restart before/at/after, replay, late failure, publication rollback, and
  dual-terminal tests prove byte-identical rollback and no duplicate transfer;
- publication rows are immutable and no row callback executes inside the owner
  transaction;
- stale contract guard is corrected; all nonexempt Rust line-count violations
  are split before terminal review;
- exact diff, security, consumer-path, dual review, dual verification, heavy
  gate, and exact-head evidence are recorded truthfully.

## Security and data impact

Local repository engineering only. Preserve typed fail-closed guards, owner
identity, replay protection, additive restart validation, rollback, immutable
publication ordering, and no-secret policy. No external write, deployment,
release, or production activation is authorized.

## Validation and review authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to two independent review agents and two independent verification agents for
the bounded source, restart, publication, consumer-path, and line-count scope;
expected outputs are compact findings and command/count summaries; reviewers
and verifiers are read-only except named package artifacts. If delegation is
unavailable, equivalent local checks must be run and the limitation recorded.

## Progress

- [x] Resume commit and prior EXECUTED HOLD identified and preserved.
- [x] Prospective owner-binding table and protected boundaries authored.
- [x] Contract-derived tests and pre-implementation gate.
- [x] Persistent attachment and ordinary scheduler hook.
- [x] Surface-liquid/WB14 continuation plumbing, restart wrapper, replay guard,
  failure rollback, and immutable publication batch.
- [x] Line-count split and package-scoped compile/test gates.
- [ ] Actual persistent Stage-3 terminal solve and event-boundary coalescing.
- [ ] Actual shared V11/Stage-3 carrier and sequential typed owner execution.
- [ ] Parent-interval chronology before complete-day finalization.
- [ ] Complete coupled owner-set and cursor restart.
- [ ] Repository-backed positive physical scenarios and exact-head assurance
  reconciliation.

Status: EXECUTED HOLD
Evidence mode: Ran/Static plus external source review

The ordinary-scheduler attachment, bounded surface-liquid continuation,
restart/replay wrapper, late-failure rollback, and immutable publication batch
are retained as a useful increment. The central Child-1 physical closure is
not complete: the current attachment observes a completed ordinary day and
packages configured/debug-derived representations rather than executing the
Stage-3/V11 coupled snow-covered segment, terminal solve, snow-free remainder,
and complete typed owner transaction.

The repository-wide exact-head assurance drift remains separately recorded,
but it is secondary to these package-local implementation blockers. Child 3
must not start.

## Disposition correction — 2026-08-21

Required disposition:

`EXECUTED HOLD / ordinary scheduler attachment, surface-liquid continuation,
restart wrapper, replay guard, and immutable batch implemented / actual
Stage-3 event solve, shared V11 carrier, sequential typed owner execution,
complete restart, and positive physical scenarios missing`

The correction is documentation-only. It does not alter the landed Rust,
contracts, selectors, defaults, CoE ownership, production outputs, deployment,
or the preserved prior package. The next implementation increment must close
the blockers recorded in `artifacts/review-disposition-correction.md` before
any Child-3 qualification work is authorized.
