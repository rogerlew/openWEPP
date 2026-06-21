# R6F Worker Handoff

Status: complete.

## Handoff

close defect R6F-DIRECT-PUBLICATION-CUTOVER-BLOCKER: execute
`docs/work-packages/20260621-r6g-direct-wat-producer-authority-001/package.md`
to close `HOLD-R6F-WAT-DIRECT-PROCESS-PRODUCER-AUTHORITY-GAP`.

## Remaining Blockers

| Marker | Family | Exact boundary | Next package |
|---|---|---|---|
| `HOLD-R6F-WAT-DIRECT-PROCESS-PRODUCER-AUTHORITY-GAP` | WAT | Production runner still supplies only climate/calendar to retained direct publication. Direct runtime can accept typed ET/storage/profile inputs and carry layer state, but no canonical parsed-input producer binds those operands for cutover. | R6G |

## Do Not Do

- Do not copy `execution.wb13_rows` into direct publication rows.
- Do not read compatibility runtime symbols/surfaces as direct authority.
- Do not mark WAT green from direct self-consistency without Arrow row/schema/
  metadata parity and independent operand reconstruction.

## Start From

- `r6f-blocker-ledger.md`
- `r6f-operand-lineage.md`
- `r6f-no-compatibility-proof.md`
- `docs/work-packages/20260621-r6g-direct-wat-producer-authority-001/package.md`
