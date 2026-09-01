# V51 post-crossing contraction pre-implementation red

Evidence mode: `Static + Ran`

Status: `EXPECTED RED`

R130 failed at exact 60-second support `1920..1980 s`. The bounded r132
capture proved V40 `SideConstraint`, V41 `NonDescent`, budget `10/96`, reserve
9, constant exact water, exact static/cadence/side/rolling/raw-owner guards,
one adjacent canonical predicate crossing `0 -> 1`, and no recross. The exact
binary64 endpoint-derived enthalpy corrections are `+5069.96060020145`,
`+2965.9686717161603`, `-997.5636528494651`, and
`+340.4305842210815 J m^-2`; their magnitudes strictly
decrease and the corrections inside predicate 1 alternate.

The contract/source gate is authored before production. It must pass the V51
contract row and fail only because the bounded post-crossing contraction
helper and five positive/poison/dispatch behaviors are absent. This red
authorizes no tolerance, equation, physics, budget, receipt, closure, event,
topology, custody, rollback, publication, persistence, diagnostic, adaptive
floor, or exact 60-second floor change.

Retained evidence:

- r130 SHA-256 `43aee720db2758e47b166f96e726e307152c4fa14c82321564422062b9df728a`;
- r132 SHA-256 `db16c87e296f1a4756d9467e38fb1b36d7611df51b8275a483c3c33584600dbf`.
