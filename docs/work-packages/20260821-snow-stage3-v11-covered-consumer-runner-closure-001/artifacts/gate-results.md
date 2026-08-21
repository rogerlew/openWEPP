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

`Static:` The snow-present rejection in
`DirectSnowStage3V11Attachment`/`DirectV11RealConsumerStack` remains present;
no covered interval is routed through snow-free LSE.

`Ran:` the Nix-provided six-package `cargo check` passed in this increment;
the orchestrator lib test suite passed 742/742 executed tests with 0 failures
and 1 ignored (743 total); the focused attachment tests passed 2/2;
the two-day provider-bound regression passed 1/1; climate adapter passed 8/8;
coupled-time passed 13/13; V11 vegetation passed 9/9; test compilation and
formatting checks passed; and `git diff --check` passed. Warnings-denied Clippy
remains a baseline failure with 27 documented findings from historical shadow,
attachment, scheduler, and evaluator lint debt.

The release gates remain open: covered V11 executor, runner-owned physical
support construction, terminal-liquid consumption, additive restart, scenario
matrix, and independent reviews are not yet dispositioned.

| Gate | Status | Evidence |
| --- | --- | --- |
| contract/operand mapping | `PASS` | `covered-consumer-contract-map.md` |
| covered executor | `BLOCKED` | implementation pending |
| runner 48-support capability | `IN PROGRESS` | provider bind implemented; support construction pending |
| terminal liquid exact once | `BLOCKED` | `event-and-terminal-liquid-lineage.md` |
| complete restart | `BLOCKED` | `restart-schema-and-equivalence.md` |
| physical scenarios | `NOT RUN` | `scenario-matrix.md` |
| reviews/verifiers/exact-head | `NOT RUN` | to be appended |
