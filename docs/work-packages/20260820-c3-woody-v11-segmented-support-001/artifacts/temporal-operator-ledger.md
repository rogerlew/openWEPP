# Temporal Operator Ledger

Status: authority candidate

| Class | Operations | Support source | State effect |
|---|---|---|---|
| Algebraic rate | radiation, FvCB/Medlyn, respiration rate, vapor, energy, hydraulic law | current forcing/state; exact slab duration only where rate/amount conversion is required | recomputed each accepted segment |
| Support integral | GPP/respiration C, transpiration, interception, turnover, mortality, N/material amounts | coupled-time `duration_s_bits` | staged amount/ledger |
| Sequential state | canopy liquid, warm starts, T10, C/N pools, NSC/XS, phase/timers/GSI | current staged beginning | ending becomes next beginning |
| Scheduled once | named daily/calendar/management receipts, parent proposal IDs, parent receipt, transaction increment | coupled scheduled-boundary/parent receipt | receipt prevents replay |
| Event | regime/participant/custody transition | zero duration | no rate integration |
| Reduction | accepted totals/maxima | accepted receipt operands only | diagnostic/buffered |

Phenology edge evaluation is sequential per physical segment, not globally
scheduled once; receipt consumption and calendar/management actions are once.
