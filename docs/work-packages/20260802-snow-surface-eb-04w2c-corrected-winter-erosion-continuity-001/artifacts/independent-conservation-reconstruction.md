# Independent Conservation Reconstruction

Status: pass for accepted real-fixture solves

Evidence mode: **Ran + Static**

The corrected EROD16 integration test independently rebuilds the signed cell
ledger for every accepted concave-profile storm from the published normalized
`load` trajectory and the input denormalization operands. It does not consume
the producer's aggregate detachment or deposition fields.

For each adjacent load pair it independently classifies the denormalized delta
as detachment or deposition, reconstructs the boundary delta from the first and
last load, and checks:

`boundary_delta - (sum_positive_cell_delta - sum_negative_cell_delta)`.

It requires
`abs(residual) <= 1e-9 * max(abs(exported), abs(detachment), 1e-9)` and also
checks that both producer aggregates match the separately accumulated cell
ledger and separately projects the first/last normalized loads to verify the
published inflow and export operands. These checks do not consume the
producer's flux-diagnostic residual.
Every depositing storm also proves that a detachment-only alias differs
materially from toe export.

Ran result: all 227 accepted storms pass; all 227 produce nonzero deposition,
with aggregate detachment `978601.7 kg` and deposition `124192.6 kg`. The four
refused storms remain explicit and contribute no fabricated sediment.

This establishes independent per-cell accounting from produced operands. It
does not claim an external observed sediment measurement or independent process
model; those are not the role of this conservation gate.
