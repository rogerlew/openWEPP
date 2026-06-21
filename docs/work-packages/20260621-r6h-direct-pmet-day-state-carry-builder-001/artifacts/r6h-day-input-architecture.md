# R6H Day-Input Architecture

Status: queued.

Record the design before production edits.

## Required Shape

- Day 0 may seed from parsed static inputs, daily climate request, and the
  private direct seed surface as R6G does.
- Day `n` must execute in the direct runtime and commit direct lane/day state
  before day `n+1` PMET operands are constructed.
- PMET operands for day `n+1` must read direct-carried layer/state, not WB13
  rows, post-scheduler compatibility runtime symbols, writeback payloads, or
  writer rows.
- Inputs must be lane-dimensional wherever process state or OFE identity can
  differ by lane.

## Planned Evidence

| Concern | Evidence required | Status |
|---|---|---|
| Interleaving | Direct test proving day `n+1` input construction observes day `n` committed state. | Queued |
| Lane dimensionality | Fixture or focused test where two lanes diverge and cannot alias a day-global input. | Queued |
| Fail closed | Missing required direct state produces typed error or stable hold, not a default. | Queued |
| Consumer path | Runner cutover consumes the interleaved direct frame for WAT output. | Queued |
