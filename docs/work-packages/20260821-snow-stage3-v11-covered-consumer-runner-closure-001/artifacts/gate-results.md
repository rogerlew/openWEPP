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
still reaches the shared covered-column endpoint after selecting
`CoveredColumnAuthority::V11SnowCovered`; that endpoint has not yet replaced
all snow-free ground/litter/mineral surface operators. This is an explicit
closure blocker, not a passed lower-boundary claim.

`Static:` Persistent support acceptance checks the Stage-3 result against the
carrier sensible, latent, vapor, longwave, advected, and ending-ice values,
rejects terminal events on the persistent branch, and rejects a partial or
non-active Stage-3 result. Carrier receipts are retained per lane and
aggregated using ordered topology tile fractions; no first-lane shared-air
shortcut is used. Prepared supports contain only sealed covered forcing and
do not expose live carrier surfaces or carrier ledgers. The new support,
configuration, V11, and carrier forcing digests use explicit typed framing
and fixed-width/f64-bit fields.

`Ran:` the Nix-provided six-package `cargo check` passed in this increment;
the orchestrator lib test suite passed 746/746 executed tests with 0 failures
and 1 ignored (747 total), including the persistent covered support and
rollback test; the focused attachment tests passed 5/5;
the two-day provider-bound regression passed 1/1; climate adapter passed 8/8;
coupled-time passed 13/13; V11 vegetation passed 9/9; test compilation and
formatting checks passed; and `git diff --check` passed. Warnings-denied Clippy
remains a baseline failure: library targets report 27 findings from
historical shadow, attachment, scheduler, and evaluator lint debt, with
additional existing test-target line-count/float-comparison findings under
`--all-targets`.

`Ran:` the final focused orchestrator library Clippy rerun still fails with
17 pre-existing direct-runtime shadow/scheduler findings; no new carrier,
Stage-3 boundary, or canonical-forcing diagnostic remains in that output.

The release gates remain open: covered V11 executor, runner-owned physical
support construction, terminal-liquid consumption, additive restart, scenario
matrix, and independent reviews are not yet dispositioned.

| Gate | Status | Evidence |
| --- | --- | --- |
| contract/operand mapping | `PASS` | `covered-consumer-contract-map.md` |
| covered executor | `IN PROGRESS` | owner-derived carrier/Stage-3 boundary and rollback pass for the stable support; lower-boundary operator, physical matrix, and terminal branch remain |
| runner 48-support capability | `IN PROGRESS` | provider bind implemented; support construction pending |
| terminal liquid exact once | `BLOCKED` | `event-and-terminal-liquid-lineage.md` |
| complete restart | `BLOCKED` | `restart-schema-and-equivalence.md` |
| physical scenarios | `NOT RUN` | `scenario-matrix.md` |
| reviews/verifiers/exact-head | `NOT RUN` | to be appended |
