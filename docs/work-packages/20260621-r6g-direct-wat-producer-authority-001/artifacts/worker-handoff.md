# R6G Worker Handoff

Status: executed-held.

## Stable Hold

`HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT`

## First Actionable Item

Close defect `R6G-DIRECT-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT` by replacing
the current precomputed `DirectPublicationDayInput` vector for PMET WAT
publication with an interleaved direct day-input builder:

1. Seed day 0 from parsed static inputs and climate request as R6G does now.
2. Execute the direct day.
3. Commit direct lane/day state.
4. Build day `n+1` PMET component inputs from the committed direct layer/state
   surface, not from WB13 rows or post-scheduler compatibility runtime symbols.
5. Repeat through the climate span, then project and publish direct WAT rows.

## Scope for Follow-On

- Direct runtime day execution orchestration for publication cutover.
- Direct publication input construction for PMET branch operands.
- Lane-dimensional direct day inputs for non-trivial OFE/lane cases.
- Canonical WAT id semantics beyond the inherited single-WAT fixture.
- Allowlisted direct symbol lineage for private seed-surface inputs.
- WAT parity gates for `Es`, `Total-Soil`, and `SoilWaterTotal`.
- HBP identity regression for the inherited near-zero runoff fixture.
- No compatibility WB13 row, compatibility runtime surface, writeback payload,
  or writer-row authority.

## Known Good Evidence to Preserve

- HBP byte identity is green for the inherited current fixture.
- First WAT row direct equals compatibility for `Es`, `Total-Soil`,
  `SoilWaterTotal`, `Dp`, `latqcc`, and `Tile`.
- R6F identity/profile fields are no longer in the reduced WAT mismatch set.
- Direct projection storage includes residual liquid water in unfrozen layer
  depth.
- The stable R6G hold marker is reserved for exactly
  `Es`, `Total-Soil`, `SoilWaterTotal`.

## Rejected Shortcut

Do not read WB13 rows, compatibility runtime surfaces, writeback payloads, or
writer rows to populate day-2 PMET `Es` or storage. Those values are allowed as
parity comparators only after direct artifacts are built.
