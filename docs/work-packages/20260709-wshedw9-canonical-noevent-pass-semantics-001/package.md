# WSHED-W9 Canonical NoEvent Pass Semantics

Status: `EXECUTED-COMPLETE`
Package ID: `20260709-wshedw9-canonical-noevent-pass-semantics-001`
Queue row: `WSHED-W9`
Evidence mode: `Static + ran`

## Objective

Define and implement canonical watershed handling for HBP pass artifacts whose
latest represented day is not a runoff `EventPayload`. A valid no-event pass
state must be typed and contract-authorized; missing, malformed, stale, or
ambiguous payload state must still fail closed. No watershed routing input may
be synthesized from an absent optional payload.

## Rationale

The HBP parser currently exposes `Option<HbpLatestEventPayload>`. For
`NO_EVENT`/non-runoff records it returns `None`, and the watershed supervisor
rejects the inventory because no canonical `NoEvent` authority was previously
cited. The same optional shape can also retain an earlier runoff `EVENT` when a
later represented day is `NO_EVENT`, creating stale-event risk. WSHED-W9 closes
that boundary by exposing latest-day state as either runoff `EventPayload` or
typed valid no-event state.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/architecture/watershed-runtime-architecture-specification.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`

## Included Scope

- Contract amendment for HBP no-event/latest-day parser state.
- Contract amendment for watershed pass-inventory no-event consumption.
- Typed parser representation distinguishing runoff `EventPayload` from valid
  no-event/no-runoff state.
- Watershed supervisor inventory consumption of valid no-event state, including
  explicit zero runoff/sediment contribution surfaces and preserved parsed
  baseflow/deep-seepage fields.
- Regression tests for valid no-event, malformed no-event, stale prior-event
  rejection, and missing/invalid pass inventory failures.
- Roadmap/work-package catalog disposition updates.

## Excluded Scope

- New watershed routing physics.
- Full logical-day multi-shard HBP reader/index implementation beyond the
  current latest-day supervisor path.
- New lateral/tile-routing consumers for HBP `SUBEVENT` fields. The parser may
  preserve those fields, but production watershed routing may keep them outside
  current consumption.
- HBP hourly water/sediment watershed consumption; that remains M-T3.

## Intended Write Set

- `docs/work-packages/20260709-wshedw9-canonical-noevent-pass-semantics-001/**`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `crates/openwepp-input-contract/src/parsers/hbp/**`
- `crates/openwepp-runner/src/watershed_supervisor.rs`
- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
- `tests/integration/infile_hbp_parser_contract.rs`

## Phase Plan

1. Scaffold package and source/lineage artifacts.
2. Amend `SC-INFILE-HBP-001` and `SC-SYSTEM-001` before production code.
3. Add typed latest HBP event-state parser API.
4. Move pass inventory and routing-input construction to typed event state.
5. Add parser and supervisor/CLI tests for valid, malformed, stale, and missing
   cases.
6. Run focused and closure gates.
7. Update artifacts, roadmap, and package catalog disposition.

## Acceptance Criteria

- HBP contract names latest-day no-event/no-runoff parser state and preserves
  source event kind provenance.
- System contract requires `PassInventory` to consume only typed
  `EventPayload` or typed valid no-event state.
- A latest `NO_EVENT` record after an earlier runoff `EVENT` does not route the
  stale prior event.
- Valid no-event state produces explicit zero surface runoff/sediment
  contribution values and preserves parsed baseflow/deep-seepage values.
- Malformed no-event payloads fail closed at parse/inventory time.
- Missing or invalid pass files still fail closed; no compatibility fallback is
  added.

## Disposition

`EXECUTED-COMPLETE` on 2026-07-09. WSHED-W9 closed the latest-day HBP no-event
consumer boundary with typed parser state and real watershed CLI consumer
evidence. Valid `NO_EVENT`/`SUBEVENT` state is represented as
`HbpLatestEventState::NoEvent`; malformed no-event payloads still fail closed,
and older runoff `EVENT` payloads are not reused when a later represented day is
no-event.

## Required Gates

- Focused parser/supervisor tests named in `artifacts/gate-results.md`.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile quick`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- `markdown-doc lint --path docs/work-packages/20260709-wshedw9-canonical-noevent-pass-semantics-001 --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md --path docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `git diff --check`

## Closure Statuses

`EXECUTED-COMPLETE`:

- All acceptance criteria pass with current evidence, and WSHED-W9 is no longer
  a latest-event no-event blocker.

`EXECUTED-HOLD-*`:

- A named no-event/subevent field lacks authority for current-scope routing
  consumption, or the current latest-day supervisor path cannot distinguish
  valid no-event state from missing state without a broader reader/index change.
