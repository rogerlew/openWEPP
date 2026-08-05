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

Required identities, in consistent area-mass units, are:

```text
Q_in - Q_cold - L_f * m_melt - Q_residual = 0
m_ice_before - m_ice_after - m_sublimation - m_melt = 0
m_liquid_in + m_melt - m_refrozen - delta_m_retained - m_routed = 0
```

The exact upstream liquid credit is the downstream Stage 3 handoff. It may not
be recomputed from pack state loss, routed melt, retained-store level, or CoE
formula terms. Refreeze, retention, and routing occur before the next substep's
thermal repartition.

Negative proof: if CoE and Stage 3 both generate melt, either both debit the
same ice (duplicate mass loss), only one debits ice (the other liquid is
unbacked), or their outputs are blended (neither energy nor phase ledger is
independently identifiable). Every case violates the exact-one authoritative
ledger. Therefore `CURRENT_DUAL_OWNER_ALLOWED` is prohibited by construction.
