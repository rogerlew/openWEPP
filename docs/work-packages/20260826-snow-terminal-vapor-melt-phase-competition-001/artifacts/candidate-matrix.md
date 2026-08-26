# Terminal vapor/melt candidate matrix

Status: `EXECUTED / CANDIDATES A AND B REJECTED / TYPED UNSUPPORTED`.

## Attempted Candidate A research semantics

Static: Candidate A is a support-local endpoint complementarity allocation,
not a continuous-time or LTE model. Signed vapor mass is split exclusively into
deposition or sublimation. Vapor latent energy is already one component of
complete energy and is not added a second time. Complete energy first changes
cold content. A remaining cold deficit may refreeze available liquid; an energy
surplus may melt available solid. The minimum-circulation solution prohibits
simultaneous net refreeze and melt, avoiding a non-unique melt/refreeze cycle.

Static: deposited mass is available to same-support melt. Refrozen mass is not:
the minimum-circulation complementarity branch makes refreeze and melt mutually
exclusive for one support-local endpoint. This prospective decision conflicts
with released `SC-SNOWENERGY-001@18` and therefore has no production authority
unless independent science review accepts the derivation and successor
contracts are released.

The accepted endpoint identities are:

```text
I_end = I_begin + deposition + refreeze - sublimation - melt
L_end = L_begin + external_liquid + melt - refreeze
Q_complete - (C_begin-C_end) - L_f*melt + L_f*refreeze - Q_unallocated = 0
I_end >= 0; Q_unallocated >= 0; I_end*Q_unallocated = 0 materially
```

## Result-blind matrix

Ran: nextest run `4329acf0-6d70-4eb8-996a-65c393e97008` executed seven
Candidate A/B tests. All seven passed.

| Vector | A | B / disposition |
|---|---|---|
| zero vapor | PASS closures/complementarity | algebraically closes |
| sublimation | PASS bounded solid debit | algebraically closes |
| deposition below/at/above melt balance | PASS; persistent/endpoint/interior classes separate | deposits into lagged frost |
| refreeze | PASS liquid debit, solid credit, fusion energy | algebraically closes |
| deposition plus refreeze | PASS; no melt/refreeze circulation | algebraically closes but owner chronology remains hypothetical |
| rain-on-snow | PASS external liquid and refreeze/melt separation | algebraically closes |
| positive/zero cold content | PASS distinct branches | algebraically closes |
| event at start/interior/end | PASS distinct typed classes | incomplete physical chronology |
| persistent deposition after exhaustion | PASS typed reappearance | distinct frost reappearance represented |
| subsequent frost melt | N/A, no frost owner | PASS algebraic next-support melt |
| support partition sensitivity | FAIL materially at meltout; proportional split alone passes | FAIL materially for B's deposition lag |
| vapor latent custody | FAIL independent proof; split is producer-derived | same input guards |
| mass/energy/water closure | PASS independent identities | PASS identities but not partition acceptance |
| invalid domain/split | typed fail closed | typed fail closed |
| Candidate C | typed `Unsupported` | retained fallback |

## Real complete-owner fixture

Ran: nextest run `043c2c94-78a8-452c-904c-98062b92931b` passed the exact real
fixture after the source split and Candidate A complete-owner mode.

- Production call remains typed `BelowCarrierDomain`.
- Released allocation at `615737728343 ns`: ice/deposition
  `0.002815601898351902 kg m^-2`, liquid/melt `0.6 kg m^-2`, and
  `1.014879671856761e-6 J m^-2` unallocated energy.
- Candidate A post-carrier research endpoint at the same tick: ice
  `0.002815601895309716 kg m^-2`, liquid `0.6000000000030422 kg m^-2`, and
  exact zero unallocated energy. The algebra closes, but the carrier fixed
  point used the released ending-snow transition, so this is not a jointly
  converged seven-owner Candidate A proof.
- Candidate B at the same tick: pack ice zero, distinct frost
  `0.002815601898351902 kg m^-2`, liquid `0.6 kg m^-2`, and the unchanged
  `1.014879671856761e-6 J m^-2` excess. It is not accepted because the lagged
  frost rule is materially partition-dependent and lacks an authorized owner.
- Candidate A at fixed parent endpoint `900000000000 ns`: complete-owner
  execution has zero solid, terminal liquid `0.6041154562923016 kg m^-2`, and
  `76872.41212288724 J m^-2` excess. This is classified as an interior-event
  diagnostic, not an accepted endpoint event or chronology proof.
- Nearby Candidate A energy perturbations of `-1e-3`, `-1e-6`, `0`, `+1e-6`,
  and `+1e-3 J m^-2` all close without positive material ice plus positive
  material excess energy.

## Falsifying partition

Ran: beginning ice `0.6 kg m^-2`, zero cold/liquid, aggregate deposition
`0.002 kg m^-2`, and energy `0.602 L_f` produces `AtEnd` with zero ending ice.
The valid energy-first partition produces `Interior` meltout and
`667.2 J m^-2` excess; a subsequent deposition-only partition produces
`Reappeared` with `0.002 kg m^-2` ice. Reverse ordering ends with zero ice.
Candidate A therefore changes endpoint state and event chronology with support
partition/order at the exact disputed phase boundary.

## Claim boundary

Candidate A and the tested Candidate B chronology fail the required partition
invariance. Candidate C remains typed unsupported. No event search,
contract successor, production implementation, receiver, restart, runner,
selector/default, activation, CoE, Child 3/4, or cutover change is claimed here.
