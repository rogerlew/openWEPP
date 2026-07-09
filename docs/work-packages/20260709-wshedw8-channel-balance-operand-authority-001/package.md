# WSHED-W8 Channel-Balance Operand Authority

Status: `EXECUTED-COMPLETE`
Package ID: `20260709-wshedw8-channel-balance-operand-authority-001`
Queue row: `WSHED-W8`
Evidence mode: `Static + ran`

## Objective

Implement contract-authorized `chanwb` channel-balance operands from typed
routed channel state: `Inflow`, `Outflow`, `Storage`, `Baseflow`, `Loss`, and
`Balance`. Existing null behavior must remain for unavailable operands; no
operand may be filled from an adjacent alias such as runoff or `cbase` unless
the typed channel state explicitly owns that value.

## Rationale

The `chanwb` parquet schema already publishes channel-balance columns, but the
typed watershed publication frame only carries generated channel baseflow, and
the writer computes `Balance` without the documented storage term. WSHED-W8
closes that publication boundary by adding a typed operand lineage and proving
the real writer consumes those typed fields.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`

## Included Scope

- Contract amendment in `SC-SYSTEM-001` for channel-balance publication
  operand lineage and null semantics.
- Typed routed channel state fields for authoritative channel-balance operands.
- `WatershedPublicationFrame` projection from routed channel state.
- `chanwb` writer balance formula including storage.
- Tests that distinguish inflow, outflow, storage, baseflow, loss, and balance
  values so aliasing cannot satisfy acceptance.
- Roadmap/work-package catalog disposition updates.

## Excluded Scope

- New channel process physics or transmission-loss model.
- New storage routing beyond exposing already authoritative routed state. The
  current direct lane carries explicit zero storage/loss operands until
  transmission-loss/storage physics owns nonzero terms.
- HBP hourly water/sediment watershed consumption; that remains M-T3.
- Changes to legacy/interchange seed writer behavior except formula consistency
  for `Balance`.

## Intended Write Set

- `docs/work-packages/20260709-wshedw8-channel-balance-operand-authority-001/**`
- `docs/ROADMAP.md`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs`
- `crates/openwepp-watershed-output/src/writers.rs`
- `tests/integration/wshedw5_typed_watershed_runtime_contract.rs`

## Phase Plan

1. Scaffold package and source/lineage artifacts.
2. Amend `SC-SYSTEM-001` before production code.
3. Add typed channel-balance operands to routed state and publication frame.
4. Project available operands through the real `chanwb` writer, retaining nulls
   for unavailable operands.
5. Add non-aliased tests for direct frame projection and parquet output.
6. Run focused and closure gates.
7. Update artifacts, roadmap, and package catalog disposition.

## Acceptance Criteria

- `SC-SYSTEM-001` explicitly authorizes `chanwb` channel-balance operands and
  their null semantics.
- `RoutedChannelState` carries inflow, outflow, storage, baseflow, and loss
  operands separately from runoff.
- `WatershedPublicationFrame` projects channel-balance operands from routed
  channel state.
- The `chanwb` writer computes `Balance = Inflow - Outflow - Loss - Storage`
  only when all required operands are present.
- Tests reject aliasing inflow as outflow and reject omitting storage from the
  balance.
- Existing null behavior remains for unavailable channel-balance operands.

## Execution Summary

- Added `INV-SYSTEM-033` to `SC-SYSTEM-001` and bound `chanwb` operands to typed
  routed channel state or explicit writer reconstruction.
- Added `channel_inflow_m3`, `channel_outflow_m3`, `channel_storage_m3`, and
  `channel_loss_m3` to `RoutedChannelState`; the direct lane populates inflow
  from `runvol_case`, outflow from `roff`, and explicit current-lane zeros for
  storage/loss.
- Added `WatershedPublicationFrame.channel_inflow_m3` and projected all channel
  balance operands from routed channel state into public publication.
- Updated the writer so `chanwb` `Inflow (m^3)` reads typed inflow, public
  `value` still reads watershed runoff, and `Balance (m^3)` is
  `Inflow - Outflow - Loss - Storage`.
- Added non-aliased writer/runtime tests proving the real `chanwb` path reads
  typed operands and keeps unavailable operands null.

## Required Gates

- Focused tests named in `artifacts/gate-results.md`.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile quick`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- `markdown-doc lint --path docs/work-packages/20260709-wshedw8-channel-balance-operand-authority-001 --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `git diff --check`

## Closure Statuses

`EXECUTED-COMPLETE`:

- All acceptance criteria pass with current evidence, and WSHED-W8 is no longer
  a channel-balance publication blocker.

`EXECUTED-HOLD-*`:

- A named operand lacks authoritative typed state after source audit. Nulls
  remain correct for that operand, and the hold artifact names the first
  actionable authority or implementation follow-on.
