# V5 Authorization-Capped Pass Selection

Status: `admitted / canonical digest binding and dual review passed`

Evidence mode: `Static`

## Imported System

V5 imports exact V4 digest
`8ace38d1148f95261306cd6b0bf6f22e23ac8ead4cb6897dbdb53061b78ee437`.
The V3 six-unknown coupled system for four hydraulic potentials and distinct
sun/shade beta factors remains unchanged. The capped pass begins from original
beginning occupancy state and adds only fixed owner-authorization caps; it does
not reuse or sequentially clamp the Stage-A candidate.

## Layer Operand Selection

For layer `i`, tile fraction `f_t>0`, and interval `dt>0`:

```text
A_tile_i   = A_W_i / f_t
cap_rate_i = A_W_i / (f_t * dt)
q_law_i    = independently evaluated hydraulic-law flux
q_i        = min(q_law_i, cap_rate_i)
F_W_i      = f_t * q_i * dt
```

`A_W_i` and `F_W_i` are stand-ground interval amounts. `A_tile_i` is a
tile-ground interval amount. `cap_rate_i`, `q_law_i`, and `q_i` are
tile-ground rates. Every conversion occurs once.

## Active Set, Derivative, and Residual

A layer is cap-active iff `cap_rate_i<=q_law_i`; exact equality belongs to the
cap branch. The generalized derivative of `q_i` with respect to coupled state
is zero on the cap/equality branch and the hydraulic-law derivative when
`q_law_i<cap_rate_i`.

Using strict `<` for cap activity, averaging derivatives at equality, or
evaluating the cap before the hydraulic law is rejected. Root continuity is
`q2-sum_i(q_i)`. The canonical normalization scale includes every `q_law_i`,
`cap_rate_i`, and `q_i` in addition to imported V3 terms. Active-cap identities
and diagnostics follow configured root-layer order, never map or lexical order.

## Identity Rebinding

V4-to-V5 migration validates the complete V4 source and copies its state
payload bitwise. It synthesizes or remaps no field. The caller supplies distinct
V5 model/configuration identity, after which the state digest is recomputed.
Stale V4 identity is rejected by the V5 parser/execution boundary.

## Transaction Rules

- Fixed authorizations retain exact transaction, owner, occupancy, soil-layer,
  unit, and stand-ground basis identity.
- No occupancy borrows another occupancy's or layer's unused authorization.
- No reauthorization or outer tile-column fixed point occurs.
- Only `F_W_i` is finalized use and owner debit; `A_W_i` is never substituted.
- Failure publishes no candidate, use, receipt, transaction advance, or warm
  start.
