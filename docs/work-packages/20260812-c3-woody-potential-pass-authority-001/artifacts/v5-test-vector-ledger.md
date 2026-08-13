# V5 Independent Test-Vector Ledger

Status: `frozen / independent regeneration and digest binding passed`

Evidence mode: `Static + Ran`

All expected values must come from the independent V5 generator and committed
fixture, never from the Rust implementation under test.

| Family | Required operands and outputs | Acceptance purpose |
| --- | --- | --- |
| Accepted capped system | at least two accessible layers; valid `F<=A<=D`; `q_law`, cap rate, selected `q`, active flags/order, coupled solution, normalized residuals, `F_W` | proves fixed caps are inside the coupled solve without requiring a positive law-active accepted layer that may be unreachable for the selected fixture |
| Controlled law branch | independently evaluated positive `q_law<cap_rate`, law derivative, and frozen generalized-Jacobian branch outside an accepted owner transaction | proves the required local piecewise branch without violating `A<=D` in acceptance evidence |
| Exact equality tie | bit-exact `cap_rate=q_law`, cap-active flag, zero branch derivative | freezes equality convention |
| Near-tie law side | representable `q_law<cap_rate`, law derivative, inactive flag | prevents tolerance from changing branch identity |
| Near-tie cap side | representable `cap_rate<q_law`, zero derivative, active flag | prevents strict-inequality or averaging substitution |
| Fully authorized value reduction | `A_W=D_W`, equality-active branches, and uncapped V3 physical result | proves exact value reduction to the imported coupled system without violating `A<=D` or the equality-active rule |
| Unavailable layers | zero-demand, dry, frozen, inaccessible, and zero-root cases | proves exact zero branches without borrowing |
| Capped failures | domain, singular pivot, and iteration limit | proves typed diagnostics, active-cap ordering, and no last iterate |
| Rollback | injected failure after cap preparation and coupled solve phases | proves byte-identical owner and transaction state |
| V4-to-V5 rebind | validated V4 source, bitwise-identical payload, distinct V5 model/configuration digest, recomputed state digest | proves identity-only migration without synthesis/remap |

## Required Poisons

The frozen fixture contains exactly the 27 V5 poison keys required by the
Version 9 amendment: amount/rate, area, interval, constitutive-law, tie,
generalized-branch, perturbation-pair, scalar-layer, gas/energy re-solve,
identity, ordering, closure, reauthorization, beginning-state, and atomic-
commit alternatives. The independent verifier requires exact set equality,
execution, and either typed rejection or numerical discrimination.

Each poison must yield a numerically distinct rejected value or typed
identity/domain failure. Presence-only labels and producer-self-comparison do
not close the gate.

Frozen identities:

- definition: `0ee6a50d5f72da0b9344d8bf1b77674e95a66ab196edc068851bb419eb7b36f3`;
- vectors: `6f5e9554fe7b91b6fcb76e777b027fbeafcf4c2873a6060bd158b6a578c37f6d`;
- generator: `4c3a1cfc18b2437dabd70e4aee03effa6af7aac893056c6248a896dd3a2b5775`.
