# Stage 3 Melt Authority Derivation

Status: complete

Evidence mode: Static

Outcome: independent energy and mass authority supports
`STAGE3_MELT_OWNER_ADMITTED` as a future target.

Pinned libsnobal `_e_bal.c` constructs the energy balance as net radiation,
sensible heat, latent heat, conduction/ground heat, and precipitation-advection
terms. `_snowmelt.c` first applies that energy to layer cold content and converts
only positive remaining energy to melt through latent heat of fusion.
`_mass_bal.c` orders precipitation, snowmelt/refreeze, evaporation, wet
compaction, and runoff; `_runoff.c` applies liquid holding capacity before
export. These sources provide equation-reference chronology, not permission to
copy code or silently import every branch.

The target substep uses the pinned mass ordering: precipitation -> energy
balance -> melt/refreeze -> vapor mass mutation -> wet compaction ->
retention/runoff. Specifically:

1. Validate finite forcing, state, units, layer partition, and flux lineage.
2. Construct current active/lower thermal state and surface temperature.
3. Evaluate and bound the signed vapor exchange, split it exactly into
   deposition and sublimation, reserve sublimation from post-precipitation ice,
   and compose complete net radiation, sensible, latent, ground/interlayer,
   and precipitation-advected energy exactly once.
4. Satisfy active/lower cold content before phase conversion.
5. Define
   `m_ice_available=max(m_ice_after_solid_precip-m_sublimation,0)`, then set
   `m_melt=min(Q_excess/L_f,m_ice_available)` and record equal solid debit and
   liquid credit. Later deposition cannot enlarge melt availability.
6. Apply same-substep refreeze. Credit refrozen mass back to ice, debit it from
   liquid, and expose its latent heat explicitly in the energy ledger.
7. Apply the already evaluated signed vapor mass exchange, then wet compaction,
   holding-capacity retention, and routed export.
8. Repartition the mutated state before reevaluating the next substep.
9. Publish independently reconstructable energy, solid-to-liquid, and liquid-
   disposition ledgers.

If available ice saturates the melt bound,
`Q_unallocated_after_exhaustion=Q_excess-L_f*m_melt`. A positive value has no
admitted physical recipient or next-state chronology in 21N and therefore is a
hard terminal-boundary cutover hold, not discarded energy.

Current Rust is not complete enough to implement this target: sensible heat is
zero in the Stage 3 carrier, precipitation advection is absent, positive excess
is reported rather than converted, and the target disposition below the exact
`1 kg m^-2` resolved-domain boundary is not adjudicated. Each item is a hard
implementation hold. No partial activation, proxy, coefficient fit, or default
change is authorized by 21N.
