# Gate results

Status: `IN PROGRESS / HOLD`.

`Static:` The predecessor `EXECUTED HOLD` remains untouched. The new provider
bind rejects non-48 days, mismatched per-interval GSI receipt, incomplete or duplicate
destination sets, mismatched interval receipt, WB14 identity, parcel identity,
destination identity, or half-open support bounds. The parent support is exactly
1,800 seconds. Pre-execution validation joins the prepared beginning GSI state
and cursor to the committed owners and joins every lane's destination set to
its bound OFE. The V11/coupled-time
beginning owner must contain the canonical ordered Stage-3 snow envelope.

`Static:` The prepared capability now binds provider day-relative receipts to
run-relative supports. Parent 0 through 47 cover exactly one 86,400-second
day; the next day begins at 86,400 seconds. A per-support forcing digest is
derived from the sealed GSI, provider interval, lane/OFE/tile, exposure, WB14,
and precipitation identities and is used for ParentAuthority, StepConstraint,
and CoupledClockState construction. Parent construction does not pre-bind an
unknown future forcing receipt.

`Static:` The snow-present rejection in
`DirectSnowStage3V11Attachment`/`DirectV11RealConsumerStack` remains present.
Covered supports use the distinct `DirectV11SnowCoveredSegmentInput` and
`DirectV11SnowCoveredRealConsumerStack`; they do not enter the snow-free
adopter. The covered adopter derives carrier operands from committed V11 and
Stage-3 beginning state, emits one carrier receipt per lane, and supplies the
same exact Stage-3 boundary receipt to the Stage-3 evaluator. The V11 side
selects `CoveredColumnAuthority::V11SnowCovered`, and the LSE solver now has an
    explicit lower-boundary branch that suppresses covered ground water, sensible,
    soil-storage, and WB14-facing operators. The radiation projection now selects
    Stage-3 snow VIS/NIR albedos before the two-stream solve and emits a typed
    band/direction optical receipt. The transaction payload now uses an explicit
    Stage-3 lower-boundary variant instead of carrying a generic ground operand
    for V11, and weighted energy tolerance includes the Stage-3 boundary
    magnitude. Released precipitation-advection and soil-coupling custody,
    keyed heterogeneous physical consumption, and independent ledger closure
    remain blockers. This is an incremental custody lift, not a passed full
    lower-boundary claim.

`Static:` Persistent support acceptance checks the Stage-3 result against the
carrier sensible, latent, vapor, longwave, advected, and ending-ice values,
rejects terminal events on the persistent branch, and rejects a partial or
non-active Stage-3 result. Carrier receipts are retained per lane and an exact
`(OFE, tile)` receipt set is constructed for covered destinations. The V11
solve still consumes a parent-level aggregate carrier state; keyed
per-destination physical consumption and fixed-point iteration remain open.
Prepared supports contain only sealed covered forcing and do not expose live
carrier surfaces or carrier ledgers. The new support, configuration, V11, and
carrier forcing digests use explicit typed framing and fixed-width/f64-bit
fields.

`Ran:` focused Nix-provided `cargo check` for
`openwepp-land-surface-energy` and `openwepp-hillslope-orchestrator` passed;
land-surface-energy lib tests passed 64/64. The orchestrator lib suite passed
745/745 executed tests with 0 failures and 2 ignored (747 total). The newly
added persistent covered support/rollback test is one of the ignored tests
because released Stage-3 shortwave/soil boundary custody is not yet
available; an explicit `--ignored` run fails closed at
`ControlVolumeClosure("weighted_ofe_energy")`, and it is not counted as a
passing physical covered case.

`Ran:` warnings-denied library Clippy fails on pre-existing dead-code,
large-enum, precision, and scheduler/attachment findings in historical
direct-runtime paths. After the local refactor, no new diagnostic remains in
the land-surface-energy changes or covered carrier projection; the command is
still not a release-clean Clippy gate. Formatting and `git diff --check`
remain final checks.

The release gates remain open: covered V11 executor, runner-owned physical
support construction, terminal-liquid consumption, additive restart, scenario
matrix, and independent reviews are not yet dispositioned.

| Gate | Status | Evidence |
| --- | --- | --- |
| contract/operand mapping | `PASS` | `covered-consumer-contract-map.md` |
| covered executor | `IN PROGRESS / HOLD` | explicit covered lower-boundary seam and keyed destination receipts exist; released Stage-3 energy custody, keyed physical solve, fixed point, ledger, physical matrix, and terminal branch remain |
| runner 48-support capability | `IN PROGRESS` | provider bind implemented; support construction pending |
| terminal liquid exact once | `BLOCKED` | `event-and-terminal-liquid-lineage.md` |
| complete restart | `BLOCKED` | `restart-schema-and-equivalence.md` |
| physical scenarios | `NOT RUN` | `scenario-matrix.md` |
| reviews/verifiers/exact-head | `NOT RUN` | to be appended |
