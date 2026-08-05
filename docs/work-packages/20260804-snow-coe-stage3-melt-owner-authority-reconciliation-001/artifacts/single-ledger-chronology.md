# Single-Ledger Chronology

Status: complete

Evidence mode: Static

The target has one phase-change event and one linked handoff:

```text
complete Stage3 energy
  -> cold-content debit
  -> bounded ice debit == generated-liquid credit
  -> same-substep refreeze + retained-liquid change + routed liquid
  -> repartitioned state for the next substep
```

Define `m_ice_start` before same-substep precipitation. Define
`m_liquid_external_in` as rain/upstream liquid entering during the substep,
excluding the retained store already present at its start, and
`delta_m_retained=m_liquid_phase_end-m_liquid_phase_start`. Required identities,
in consistent area-mass and energy units, are:

```text
Q_complete + Q_refreeze - delta_E_cold - L_f*m_melt - Q_unallocated_after_exhaustion = 0
m_ice_start + m_solid_precip + m_deposition - m_ice_end - m_sublimation - m_melt + m_refrozen = 0
m_liquid_external_in + m_melt - m_refrozen - delta_m_retained - m_routed = 0
```

`delta_E_cold` is the signed increase in below-phase-threshold snow internal
energy for the declared active/lower control volumes; it does not hide
refreeze latent heat, and `Q_refreeze=L_f*m_refrozen`. The exact mass sequence
is precipitation -> melt/refreeze -> vapor mass mutation -> wet compaction ->
retention/runoff.

The energy evaluation supplies one bounded signed vapor exchange before melt:
`m_deposition=max(m_v,0)` and `m_sublimation=max(-m_v,0)`. Sublimation is
reserved from post-precipitation ice, so
`m_ice_available=max(m_ice_after_solid_precip-m_sublimation,0)` bounds melt.
The reserved vapor mutation is applied after melt/refreeze in the pinned mass
order; later deposition never enlarges melt availability.

`Q_unallocated_after_exhaustion=Q_excess-L_f*m_melt`. It must be zero in every
currently admitted resolved substep. A positive value is not discarded or
carried: it marks the unresolved terminal meltout/remaining-energy boundary and
blocks cutover until canonical authority defines the physical recipient and
next-state chronology.
The exact upstream liquid credit is the downstream Stage 3 handoff. It may not
be recomputed from pack state loss, routed melt, retained-store level, or CoE
formula terms. Refreeze, retention, and routing occur before the next substep's
thermal repartition.

Negative proof: if CoE and Stage 3 both generate melt, either both debit the
same ice (duplicate mass loss), only one debits ice (the other liquid is
unbacked), or their outputs are blended (neither energy nor phase ledger is
independently identifiable). Every case violates the exact-one authoritative
ledger. Therefore `CURRENT_DUAL_OWNER_ALLOWED` is prohibited by construction.
