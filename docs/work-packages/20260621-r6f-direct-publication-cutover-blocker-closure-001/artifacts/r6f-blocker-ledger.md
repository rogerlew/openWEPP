# R6F Blocker Ledger

Status: scaffolded.

This ledger is append-only during execution. A row that says only "parity
mismatch" is invalid; reduce the blocker until it names fields, operands,
producers, and authority.

## Active Iteration Ledger

| # | First seen | Marker | Output family | Smallest reduced unit | Direct operand | Producer/consumer | Authority | In envelope? | Correction attempted | Validation result | State |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | R6E | `HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH` | HBP | Pending field/byte-span reduction | Pending | Pending | Pending | Presumed yes until disproven | Pending | Pending | Open |

## Blocker Reduction Notes

For each open row, record:

- exact command that exposed it;
- exact output file, row/key, field, byte span, or metadata item;
- direct value and compatibility value;
- direct operand lineage;
- expected authority and units;
- why the blocker is in-envelope or out-of-envelope;
- correction patch or reason correction is impossible within the package;
- rerun result.

## Closed Blockers

| # | Closed date | Closing commit | Evidence |
|---|---|---|---|
|  |  |  |  |
