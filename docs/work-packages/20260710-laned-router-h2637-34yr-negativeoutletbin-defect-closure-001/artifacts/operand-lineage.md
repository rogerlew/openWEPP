# Operand Lineage

Status: `EXECUTED-PRE-IMPLEMENTATION`

Evidence mode: `Static + Ran`

This table is the conservation/publication intake required before the
LANED-NOB-001 production edit.

| Operand | Units | Normalization / basis | Source authority | Acceptance role |
| --- | --- | --- | --- | --- |
| upstream boundary rate `q_up` | `m²/s` per unit width | interval mean over the solver substep | upstream OFE conservative bin integral / `SC-OFEROUTE-001` Algorithm 5-6 | authoritative input |
| lateral source `v` | `m/s` | cell rate; integrated as `Σ v dx dt` to `m²` per unit width | Lane D seam / `INV-OFEROUTE-012` | authoritative input |
| raw predictor outlet extrapolation | `m²/s` per unit width | `2 q[n-1] - q[n-2]` | TVD-MacCormack donor stencil before boundary-domain enforcement | diagnostic only; may be negative |
| scheme-actual predictor/corrector outlet faces | `m²/s` per unit width | exact-zero lower bound, then rev-41 available-water upper cap | `SC-OFEROUTE-001` rev 51 | authoritative routed outflow |
| booked outflow | `m²` per unit width | `0.5 (F_pred + F_corr) dt`, summed over steps | solver mass ledger; same stage faces as update | authoritative output operand |
| outlet bins | `m²` per unit width per covered sample span | conservative pro-rata accumulation of booked step outflow | `SC-OFEROUTE-001` Algorithm 6 | authoritative handoff/publication substrate |
| initial/final mesh storage | `m²` per unit width | independent `Σ h_i dx` reconstruction | committed solver depth state | authoritative independent closure operand |
| positivity-clamp injection | `m²` per unit width | accumulated dry-floor cleanup only | rev-41 solver ledger | authoritative surfaced correction; expected roundoff |
| lane width | `m` | multiply `m²` solver operands to `m³` lane volumes | active lane geometry | authoritative conversion |
| soil-released runoff | `m³` | `q_runoff_m × A_lane_m²` | independent soil/day books | authoritative seam cross-ledger operand |
| active day end storage / outlet / source / clamp | `m³` | lane sums on a shared routing window | direct active day books | authoritative full-day closure operands |

## Independent Identities

The focused contract vector reconstructs storage change from committed depths,
not from `MassBalance.storage_change_m2`, then evaluates:

`inflow + source + clamp - outflow - (Σ h_final dx - Σ h_initial dx)`.

It separately requires `Σ outlet_bins == booked_outflow` and every bin to be
nonnegative. The active endpoint retains the rev-27 independent seam identity
`router_booked_injection == Σ(q_runoff × A_lane)` and the assembled hillslope
day identity.

## Rejected Aliases and Formulas

The regression fixture distinguishes and rejects:

- raw `2 q[n-1] - q[n-2]` as physical outflow when it is negative;
- committed outlet-cell `q[n-1]` as a substitute for the stage boundary flux;
- zeroing a negative published bin without changing the state-update and ledger
  face (publication-only masking);
- forward borrowing from later bins as a correction for a negative
  scheme-actual production flux;
- post-update negative-depth/storage clamping or clamp-mass injection;
- dropping the terminal deficit from the ledger;
- using router-produced storage-change bookkeeping as the only closure
  reconstruction.

On the captured last H2637 step, these aliases are numerically separated:
raw predictor face `-1.0522068720127724e-7 m²/s`, corrector face
`2.491288756274384e-9 m²/s`, committed outlet-cell discharge
`3.525168134310125e-9 m²/s`, and physically admissible predictor face
`0 m²/s`.
