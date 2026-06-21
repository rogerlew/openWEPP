# R6H Worker Handoff

Status: queued.

## Stable Hold

Queued.

## First Actionable Item If Held

If R6H cannot complete, close defect `R6H-DIRECT-PMET-DAY-STATE-CARRY-BUILDER`
only at a new exact boundary with:

- reduced fields;
- attempted or ruled-out in-scope corrections;
- no compatibility alias;
- dual review and verification;
- a scaffolded follow-on package naming the first implementation step.

## Rejected Shortcut

Do not fill PMET `Es`, storage totals, WAT id, or lane-specific operands from
WB13 rows, compatibility runtime surfaces, writeback payloads, writer rows, or
output rows. Those values are parity comparators only after direct artifacts
are built.
