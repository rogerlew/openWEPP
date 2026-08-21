# Implement Child 2C Shared Carrier and Terminal Snow Handoff

Status: `EXECUTED HOLD / ordinary-runner typed owner-input authority and custody closure remain open`

Date: `2026-08-20`

Package ID: `20260821-snow-stage3-shared-carrier-terminal-handoff-implementation-001`

Plan class: `Critical contract-first default-off kernel implementation and real-consumer handoff`

## Purpose / Big Picture

Implement the released Child 2C shared V11-canopy/Stage 3-snow carrier and the
terminal snow-to-real-owner handoff as one parent transaction. A successful
implementation will let the actual hillslope scheduler evolve a snow-covered
segment, localize a terminal event on an admissible integer tick, transfer
retained and newly generated liquid exactly once, and continue only with the
post-event V11, snow-free LSE, surface-liquid, hydrology, soil-thermal, and BGC
owners. It must be observable through the actual scheduler consumer and its
restart, rollback, and conservation evidence, not through a reference-only or
shadow-only path.

This is a fresh successor to the historical
`20260819-snow-stage3-terminal-meltout-lse-handoff-implementation-001` package.
That package remains an executed HOLD and is historical evidence. The checkpoint
`83cf6eb8e` is consumed as contract/evidence provenance; it is not a reset point
or branch base. The implementation base is current `main` after Child 2C
lifecycle terminalization (`71e6a2651`).

## Authority and release boundary

The exact released contracts are:

- `SC-COUPLEDTIME-001@3` — approved / active;
- `SC-LANDSURFACEENERGY-001@7` — approved / active;
- `SC-SNOWENERGY-001@14` — approved / active;
- `SC-VEGETATION-001@26` — approved / active; and
- `SC-VEGETATIONTRANSACTION-001@15` — approved / active.

Child 2B remains protected at release `1d0239f4aab78966537c465bdfd4d1efc69f5ef1`.
The implementation must preserve V10 behavior, DirectV10 Restart V1,
coupled-time Restart V2, V11 Restart V3, Child 2B receipts, defaults, and CoE
production ownership.

The package may implement a default-off candidate and prove its real consumer.
It must not authorize production activation, selector/default change, CoE
retirement, seasonal efficacy, calibration, deployment, or publication. The
terminal release boundary is:

`COMPLETE / default-off Stage 3 shared-carrier and terminal snow-to-real-owner handoff implemented / actual scheduler consumer / CoE production authority unchanged`

## Parent transaction authority

The implementation is one indivisible parent transaction:

```text
Stage 3 snow-covered beginning state
-> shared carrier and V11 pre-event slabs
-> terminal event proposal
-> deterministic admissible event-boundary selection
-> Stage 3 terminal snow/liquid/energy closure
-> zero-duration custody transition
-> post-event V11 + snow-free LSE + surface liquid + hydrology
-> one complete-owner atomic commit
```

Every fallible validation occurs before one non-fallible complete-owner
replacement. A failure at any point preserves byte-identical committed owners
and publishes nothing.

The complete parent owner set must include at least the coupled clock, Stage 3
snow, V11 vegetation, shared-carrier state and receipts, LSE, surface liquid,
direct hydrology, soil thermal, BGC, forcing/GSI cursor state, and the
publication/reduction buffer.

## Included scope

- Implement the shared canopy-air node for V11 canopy surfaces and Stage 3
  ground snow using the released sealed exposure and transfer geometry.
- Implement reciprocal canopy/snow/sky longwave and separate sensible, vapor,
  snow-mass, liquid, and energy ledgers with exact owner lineage.
- Integrate deterministic event-boundary candidate enumeration, support-domain
  checks, tolerance checks, tie-breaking, `ERR-CT-021` retry, and zero-duration
  custody transition.
- Integrate the post-event real-owner stack only for a nonzero,
  support-admissible remainder; no snow operands may enter that solve.
- Prove the actual scheduler consumer, complete-owner custody, restart before,
  at, and after the event, replay protection, rollback, publication ordering,
  and independent conservation reconstruction.
- Add contract-derived unit/integration tests, mandatory scenario fixtures,
  and package evidence for the actual production path while keeping the
  candidate default-off.

## Excluded scope

- Production activation, selector/default changes, CoE retirement, deployment,
  public publication, or seasonal qualification.
- Canopy-intercepted snow.
- Raw 10 m wind as subcanopy wind, fixed attenuation multipliers, separate
  V11/snow canopy-air nodes, or surrogate/provisional/heuristic physics.
- Compensated or sub-ULP LSE storage arithmetic, hidden duration floors,
  remainder dropping, result scaling, or below-domain LSE execution.
- Rewriting, rebasing, or silently relabeling the historical Child 1 HOLD.

## Intended write set

The exact source write set is frozen during intake before production edits. The
candidate surfaces are limited to the actual scheduler and its owned
collaborators under:

- `crates/openwepp-hillslope-orchestrator/src/`;
- `crates/openwepp-coupled-time/src/`;
- `crates/openwepp-land-surface-energy/src/`;
- `crates/openwepp-vegetation/src/`;
- `crates/openwepp-persisted-restart-v1/src/`; and
- package-owned integration tests under `tests/` or an explicitly justified
  crate test target.

Documentation, schemas, fixtures, evidence, and package-local prompts belong
under this package. `Cargo.toml` may change only for explicitly named test
registration or required owned-module wiring. No unrelated source path may be
added after intake; any expansion requires a package amendment before editing.

## Required implementation chronology

For every accepted pre-event slab, consume sealed atmospheric forcing, V11 and
Stage 3 beginning states, the coupled-time support receipt, and the exposure /
geometry configuration. Solve the shared canopy-air temperature and humidity,
then stage V11 canopy and Stage 3 snow responses, reciprocal longwave, and all
ending ledgers.

Given proposed `t*`, enumerate integer candidates and accept only when both
neighboring supports are zero or at least their active-participant common
minimum, event-time error passes, snow-mass error passes, liquid-mass error
passes, and energy error passes. Preserve proposed and accepted ticks, support
receipts, candidate digest/evaluations, tolerance identity, tie rank, terminal
snow/liquid/energy state, and owner/replay identity in the receipt.

At the accepted tick, close Stage 3 solid-snow ownership, transfer retained and
newly generated liquid exactly once, change the active participant set, advance
the event ordinal exactly once, and advance no physical time. A restart at that
boundary must not replay the transfer or custody transition.

Only a nonzero, support-admissible remainder may execute the post-event stack:
V11, snow-free LSE, surface liquid, direct hydrology, soil thermal, and BGC.
Snow albedo, snow temperature, snow roughness, snow turbulent flux, snow
longwave, and snow surface state are forbidden in that solve.

## Mandatory scenarios

The implementation and evidence must cover no meltout; events at parent start
and parent end; admissible interior events; sub-minimum pre- and post-event
proposals; no admissible candidate with `ERR-CT-021` retry; two equal-rank tie
rules; sublimation; deposition; rain during terminal melt; retained snow liquid
transfer; new meltwater transfer; infiltration; ponding/overflow; routed
runon/runoff; cross-midnight event; restart before, at, and after the event;
new snowfall after a snow-free period; and failures in the pre-event carrier,
event reconstruction, post-transfer candidate construction, post-event
LSE/hydrology, and immediately before parent commit.

For every failure scenario, require byte-identical rollback of every committed
owner and no publication.

## Phase plan

1. Intake current `main`, required reading, actual consumer path, owner set,
   operand lineage, source provenance, and exact write set.
2. Freeze implementation intent and the pre-implementation contract gate before
   production edits. Confirm that the released contracts are sufficient and
   identify any required contract amendment as a separate hold decision.
3. Implement the shared carrier and snow-covered segment in the actual
   scheduler, with typed state, receipts, ledgers, and fail-closed guards.
4. Implement terminal event localization, zero-duration custody, post-event
   real-owner continuation, and atomic parent commit.
5. Add restart/replay/rollback and independent conservation/output evidence;
   prove the actual scheduler consumer and negative old-path proof.
6. Run focused, quick, domain, and critical full-workspace validation selected
   by the canonical testing strategy; record exact outputs and line counts.
7. Obtain two independent reviews and two independent verifications, explicitly
   disposition every finding, reconcile the exact terminal diff, and close only
   at the default-off/no-activation boundary.

## Exit criteria

- The actual hillslope scheduler consumes the shared carrier and terminal
  handoff; no reference, skeleton, wrapper, adapter, or shadow path carries the
  closure claim.
- All required chronology, custody, support, receipt, ledger, replay,
  rollback, and publication invariants are exercised by tests and fixtures.
- The complete-owner commit is atomic, failures are byte-identical no-ops, and
  no post-event solve reads snow operands.
- Independent snow, liquid, signed vapor, energy, longwave, event-time, and
  output reconstructions pass with alias-separating fixtures and real closure/
  magnitude evidence.
- Restart before/at/after event and cross-midnight chronology pass without
  replay or duplicate transfer.
- Dual reviews, finding disposition, dual verification, exact diff, consumer
  path, security, line-count, and required documentation gates pass.
- Production activation, selector/default change, CoE retirement, seasonal
  efficacy, and calibration remain explicitly unauthorized or unclaimed.

## Required subagent authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to two independent implementation/code-review agents and two independent
verification agents for the shared-carrier, terminal-handoff, custody,
consumer-path, and rollback scope. Expected outputs are compact findings,
command/count summaries, and named package artifacts. Implementation agents may
write only the bounded source/test paths frozen by intake; reviewers and
verifiers are read-only except their named artifacts. Any required critical
full-workspace or heavy batch run must be delegated to the authorized
`comparator_suite_runner` role when available; if unavailable, record the
tool-policy block before running locally.

## Security and data impact

This package is local repository engineering. It must preserve fail-closed
validation, owner locking/identity, receipt replay protection, rollback, and
publication ordering. No credentials, external messages, deployment, release,
or protected data mutation is authorized. Reassess security impact if any
runtime endpoint, persistence format, or external boundary enters the frozen
write set.

## Calibration and efficacy boundary

This package implements contract-authorized physics but does not calibrate it or
claim seasonal physical efficacy. `science_implementation_status` may become
`IMPLEMENTED`; `calibration_evidence_status` remains `NOT_CALIBRATION_READY` or
`NOT_APPLICABLE` until a separately authorized calibration/validation package;
`identifiability_status` remains `NOT_ASSESSED` or `NOT_APPLICABLE` with
rationale. Canopy-intercepted snow remains `NOT_IMPLEMENTED`.

## Progress

- [x] (2026-08-20) Scaffolded as a fresh successor on current `main` after
  Child 2C lifecycle terminalization.
- [x] (2026-08-20) Preserved the historical Child 1 HOLD and recorded
  `83cf6eb8e` as consumed evidence, not an implementation base.
- [x] (2026-08-21) Complete actual-consumer intake and freeze the exact
  source/test write set.
- [x] (2026-08-21) Complete the pre-implementation contract and owner-set
  gate; endpoint consumer proof remains held.
- [x] (2026-08-21) Implement the shared carrier, event locator, staged
  terminal transaction, restart wire, and focused test source.
- [x] (2026-08-21 follow-on) Wire the concrete V11/LSE/BGC/soil-thermal owner
  executor to the opt-in scheduler and prove its staged owner commit.
- [ ] Execute mandatory scenarios and full restart/rollback/consumer closure.
- [x] (2026-08-21 follow-on) Complete dual independent review, finding
  disposition, and dual independent verification; all four dispositions are
  `HOLD`.
- [x] (2026-08-21 hold-lift) Complete the bounded hold-lift audit, identity /
  restart hardening, exact-diff reconciliation, and final HOLD disposition.
- [ ] Complete the final release boundary; this requires a separately
  authorized typed owner-input/receiver authority and the remaining custody,
  publication, and critical-profile gates.

## Decision log

### Follow-on amendment — 2026-08-21

The user-directed follow-on authorizes the remaining concrete-owner wiring and
the package's independent review/verification gates. The amended source write
set is limited to the existing orchestrator owner surfaces and their tests:

- `crates/openwepp-hillslope-orchestrator/src/snow_stage3_terminal_handoff.rs`;
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`;
- `crates/openwepp-hillslope-orchestrator/src/v11_vegetation_consumer.rs`;
- `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs`; and
- package-owned integration/unit tests needed to prove the typed endpoint.

The follow-on may not activate the normal selector, retire CoE ownership, or
edit released science-contract authority. The endpoint claim requires an
actual invocation of the typed V11 stack that produces LSE, BGC, and
soil-thermal owner candidates before the terminal handoff commit; a callback
that merely rewraps bytes is insufficient.

The follow-on endpoint now satisfies that typed invocation at the opt-in
scheduler boundary. The ordinary hillslope runner remains outside the amended
write set and still needs a separately authorized typed receiver/transaction
integration; no production-path closure is claimed here.

### Hold-lift amendment — 2026-08-21

The user explicitly authorized completion of the remaining HOLD work. This
amendment reopens the package's already-declared ordinary-runner, owner-custody,
restart, rollback, publication, and mandatory-scenario scope. The bounded source
write set now includes the normal hillslope runner and the typed transaction
surfaces it must consume:

- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs` and
  package-owned runner tests;
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/` owner/frame and
  publication surfaces;
- `crates/openwepp-hillslope-orchestrator/src/snow_stage3_terminal_handoff.rs`
  and `v11_vegetation_consumer.rs`;
- `crates/openwepp-persisted-restart-v1/src/` for the complete typed restart
  transaction; and
- package-owned integration/unit tests and evidence.

The hold-lift must move the real downstream consumer, not relabel the existing
shadow path. It may not add surrogate/proxy process physics, silently activate
the normal selector, edit released science-contract authority without a
separate contract-first amendment, or weaken fail-closed custody/restart
guards. If the existing runner lacks authoritative typed owner state, the
implementation must expose that missing input as a typed boundary rather than
fabricate it.

- Decision: create a fresh successor rather than edit the historical Child 1
  package. Rationale: the historical package records the pre-V11/shared-carrier
  HOLD and must remain immutable evidence. Date/Author: 2026-08-20 / Codex.
- Decision: implement the carrier and terminal handoff in one package and one
  parent transaction. Rationale: splitting them would create an unreviewed
  interface between pre-event shared-air state, terminal closure, and the
  post-event receiving owners. Date/Author: 2026-08-20 / Codex.
- Decision: use current `main` after lifecycle terminalization rather than
  resetting to `83cf6eb8e`. Rationale: the checkpoint identifies consumed
  evidence; current `main` contains the released contracts and tests needed by
  the successor. Date/Author: 2026-08-20 / Codex.
- Decision: retain `HOLD` after the initial implementation increment. Rationale:
  at that checkpoint the direct scheduler seam still accepted opaque ending
  owner bytes. This historical reason is superseded by the follow-on typed
  endpoint, but the package remains held for ordinary-runner integration,
  contract closure, authority-document guards, and independent review/
  verification. Date/Author: 2026-08-21 / Codex.
- Decision: retain `HOLD` after the typed-owner follow-on. Rationale: the
  owner-aware opt-in endpoint reaches the real V11/LSE/BGC/soil-thermal stack,
  but the normal hillslope runner does not construct it; carrier/event joins,
  terminal liquid custody, durable publication/restart identity, mandatory
  scenarios, and authority-document guards remain open. Date/Author:
  2026-08-21 / Codex.
- Decision: execute the hold-lift amendment in this package. Rationale: the
  user explicitly requested completion of the remaining HOLD work, and the
  original package already declared ordinary-runner consumer, owner custody,
  restart, rollback, publication, and mandatory-scenario closure as included
  scope. Date/Author: 2026-08-21 / Codex.
- Decision: close the hold-lift increment as `EXECUTED HOLD`. Rationale: the
  implementable identity/restart and participant-join protections are complete,
  while the ordinary runner still lacks authoritative typed V11/LSE/BGC/soil-
  thermal input state and the current receiver cannot legally transfer terminal
  liquid into the real surface-liquid owner. Fabricating those owners would
  violate the science-contract and kernel governance boundary. Date/Author:
  2026-08-21 / Codex.

## Surprises & Discoveries

- The openWEPP repository does not provide the WEPPcloud `wctl doc-*` wrapper;
  lifecycle prompt relocation was therefore performed with `git mv` and
  byte-identity was checked directly. This is a tooling boundary, not a
  documentation correctness exception.
- The required Core reading set is `678907` bytes, which is `WARN` but below
  the `REQUIRES-JUSTIFICATION` threshold. Conditional and on-demand sources
  remain deferred until actual-consumer intake identifies the touched paths.

## Outcomes & Retrospective

The increment produced contract-bound carrier/event/runtime/restart code and a
real direct-publication opt-in seam. The follow-on now proves that the typed
V11/LSE/BGC/soil-thermal owner stack executes and commits before the handoff
runtime advances. The ordinary runner and the broader carrier/event/custody,
publication/restart, mandatory-scenario, authority, and review/verification
closures remain open. Nix-backed focused Rust validation passes, while
unchanged authority-document guards fail in the combined and frost profiles.
The package remains HOLD.

## Surprises & Discoveries

- The historical Child 1 package is an executed HOLD and is not a safe base for
  the successor. The campaign record explicitly requires a fresh successor.
- The openWEPP checkout does not provide the WEPPcloud `wctl doc-mv` runtime
  prerequisite (`docker/defaults.env`); the Child 2C prompt was therefore moved
  with a tracked-file move and byte-compared before lifecycle closure.

## Outcomes and retrospective

This is an executed implementation increment under HOLD. The normal
selector/default, publication authority, and CoE owner remain unchanged. The
typed owner-aware opt-in endpoint is wired and hardened with event identity,
participant joins, contiguous ordinals, and persisted receipt bodies. The
ordinary runner still lacks the authoritative typed owner-input boundary needed
for a valid receiver binding; terminal-liquid custody, durable publication,
critical-profile, and authority-document closure therefore remain explicitly
open.
