# Candidate matrix

Status: `EXECUTED / FOUR CANDIDATES NO-GO`.

Ran: post-review synthetic trajectory matrix nextest
`11375817-3189-4900-999b-22b1df2595ab` passed 8/8. The executable
characterization covers zero vapor, sublimation, deposition, refreeze, rain,
positive/zero cold content, meltout/reappearance, sign-changing energy, exact
replay, vapor mass/latent poison, zero latent-coefficient poison, sublimation
overdraw, signed-zero rejection, mixed pack/frost rejection, tick overflow,
tagged-state schema poison, and forcing order.

## Candidate results

| Candidate | Executed result | Disposition |
|---|---|---|
| Released ordered trajectory | Closes its bookkeeping, but materially changes under a proportional split at meltout; a hypothetical constant-rate segment produces meltout followed by deposition reappearance and leaves positive solid with historical excess energy | **NO-GO**: partition-dependent at the phase boundary, and no post-event energy/liquid recipient is defined |
| Event-driven pack/frost hybrid | The attempted implementation reused the whole-segment complementarity allocator and endpoint-stamped events; it did not localize exhaustion or split deposition across the event | **NO-GO**: the named model was not scientifically instantiated; doing so requires an authoritative forcing path and frost thermodynamics/ownership |
| Time-resolved complementarity | One below-terminal coarse/two-half proportional comparison is equal; reordered physical paths differ. This is characterization, not a convergence proof | **NO-GO**: segment boundaries are model inputs, interior events are not localized, and no simultaneous-equilibrium authority exists |
| Existing-snow frost subtype | The test envelope preserves four ending scalars, but inherits the failed hybrid and cannot continue mixed pack/frost; distinct chronologies can serialize to the same bytes | **NO-GO**: incomplete restart identity plus missing frost thermodynamics and explicit snow-owner schema authority |

No outcome with material ending solid and material accumulated unallocated
energy is classified as acceptable. Such outcomes are explicitly classified
`UnresolvedSolidEnergyCoexistence`.

## Real fixture

Ran: post-review real complete-owner diagnostic nextest
`df2735e6-b5d8-4988-9970-e407a7b209c4` passed 1/1.

At `615737728343 ns`, the one-segment released-order interpretation produces a
**hypothetical constant-rate** transient meltout tick `615737728340 ns`,
reappearance at the support end, ending pack ice
`0.002815601898351902 kg m^-2`, liquid `0.6 kg m^-2`, and unallocated energy
`1.014879671856761e-6 J m^-2`. The hybrid attempt and time-resolved
complementarity interpretation instead retain pack ice
`0.002815601895309716 kg m^-2`, liquid `0.6000000000030422 kg m^-2`, and no
event. The integrated receipt does not authorize the uniform within-support
path used to calculate the diagnostic tick and cannot select a model.

The real vapor mass/latent-energy ratio matches the canonical 0 C latent heat
within `1e-9` relative tolerance. Nearby complete-energy perturbations of
`+/-1e-3` and `+/-1e-6 J m^-2` execute diagnostically.

## Boundary exposed by the matrix

No real receipt supplies a sealed sub-support ordering of complete energy and
vapor transfer. No frost candidate has authorized enthalpy, exchange,
receiver, publication, or complete restart semantics. Event records in this
prototype are diagnostic and trajectory-relative, not owner/receiver receipts.
These are model and authority blockers, not search-method defects.
