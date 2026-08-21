# Stage-3 Terminal Handoff Live-Owner Closure

Status: `EXECUTED COMPLETE — affected closure passed; repository-wide exact-head assurance drift remains separately recorded`

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

| Caller field / old DTO surface | Actual owner used by this package | Class | Binding rule |
|---|---|---|---|
| wind, transfer height, roughness | sealed Stage-3 exposure receipt | sealed forcing | exact `5 m`, `0.005 m`, provider/source identity |
| air temperature, humidity, longwave | sealed half-hour forcing receipt | sealed forcing | exact provider receipt and interval identity |
| canopy temperature/humidity/conductance | staged V11 canopy owner and static configuration | staged/static | derived from current live V11 owner; no raw conductance input |
| snow temperature/state and mass ledgers | persistent Stage-3 shadow owner | staged | current interval beginning/ending state and accepted receipts |
| active participants/supports | coupled-time support receipts | sealed/staged | reconstructed from current segment and event boundary |
| event proposal/ticks/tolerances | Stage-3 terminal solver and coupled clock | static/staged | scheduler wall cursor and contract tolerances; no caller event DTO |
| terminal liquid mass | Stage-3 event receipt plus snow retained-liquid owner | staged | retained liquid + support rain + terminal melt - refreeze |
| terminal liquid temperature/enthalpy | SC-SURFACELIQUID-001 identity | static | `273.15 K`, zero sensible enthalpy relative to `T_ref` |
| WB14 parameters and continuation | surface-liquid configuration and live hydrology/WB14 owner | static/staged | actual configured lane, soil layer, cumulative state, exact remainder |
| V11/LSE/BGC/soil-thermal endings | ordinary scheduler's persistent shadow owner set | staged | one sequential candidate from live owners; no caller payload |
| rows/publication | immutable scheduler publication batch | staged | rows are consumed only after owner transaction returns successfully |

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
- [x] Persistent attachment and ordinary scheduler closure.
- [x] Full restart/replay/publication/failure/dual-terminal closure.
- [x] Line-count split, heavy gates, dual review/verification, and disposition.
