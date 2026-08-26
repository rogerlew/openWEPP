# Candidate matrix

Status: `EXECUTED / FOUR CANDIDATES DISPOSITIONED FOR REVIEW`.

Ran: final synthetic trajectory matrix nextest
`a1e5a66d-c0fa-4af2-a164-8a20dee172e5` passed 7/7. The matrix
covers zero vapor, sublimation, deposition, refreeze, rain, positive/zero cold
content, meltout/reappearance, sign-changing energy, exact replay, vapor
mass/latent poison, sublimation overdraw, rollback, tagged restart, and forcing
order.

## Candidate results

| Candidate | Numerical/closure result | Physical/ownership disposition |
|---|---|---|
| Released ordered trajectory | Exact ledgers and event history; materially fails proportional fixed-path refinement at meltout | REJECT: substep partition selects how much post-melt deposition survives; also lacks a recipient for energy after transient meltout |
| Event-driven pack/frost hybrid | Fixed-path proportional refinement converges for the tested constant-rate path; ordered paths retain meltout/reappearance | UNQUALIFIED: actual carrier exposes an integrated endpoint, not an authoritative intra-support path; any surviving frost needs owner/enthalpy/receiver authority |
| Time-resolved complementarity | Fixed-path refinement converges; reordered paths intentionally differ and retain distinct events | UNQUALIFIED: no independent simultaneous-equilibrium authority; segment boundaries remain model inputs |
| Existing-snow frost subtype | Same physical result as hybrid; tagged pack/frost bytes replay exactly and schema poison rejects | UNQUALIFIED: changes snow-owner state and restart semantics and needs explicit authorization; owner reuse does not supply missing frost thermodynamics |

## Real fixture

Ran: final real complete-owner diagnostic nextest
`b275ae2c-fad8-429f-b405-325c9020237d` passed 1/1.

At `615737728343 ns`, the one-segment released-order trajectory localizes a
transient meltout at `615737728340 ns`, reappearance at the support end, ending
pack ice `0.002815601898351902 kg m^-2`, liquid `0.6 kg m^-2`, and
unallocated energy `1.014879671856761e-6 J m^-2`. The hybrid and time-resolved
complementarity interpretations of the same integrated segment instead retain
pack ice `0.002815601895309716 kg m^-2`, liquid
`0.6000000000030422 kg m^-2`, and no event. The fixture therefore cannot
select among them without an authoritative within-support forcing trajectory.

The real vapor latent coefficient reconstructed from independently produced
mass and energy matches the canonical 0 C surface latent heat within `1e-9`
relative tolerance. Nearby complete-energy perturbations of `+/-1e-3` and
`+/-1e-6 J m^-2` execute without closure failure.

## Matrix gaps that block selection

No real receipt supplies a sealed sub-support ordering of complete energy and
vapor transfer. Neither frost candidate has authorized enthalpy, surface
exchange, receiver, or restart semantics. The time-resolved complementarity
path still inherits the rejected same-segment equilibrium assumption. These
are authority/model boundaries, not search-method defects.
