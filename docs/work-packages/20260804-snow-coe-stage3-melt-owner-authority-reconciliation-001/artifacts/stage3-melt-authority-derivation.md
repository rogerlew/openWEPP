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

The target substep is:

1. Validate finite forcing, state, units, layer partition, and flux lineage.
2. Construct current active/lower thermal state and surface temperature.
3. Compose complete net radiation, sensible, latent, ground/interlayer, and
   precipitation-advected energy exactly once.
4. Satisfy active/lower cold content before phase conversion.
5. Set `m_melt=min(Q_excess/L_f,m_ice,available)` and record equal solid debit
   and liquid credit.
6. Apply same-substep refreeze, holding-capacity retention, and routed export.
7. Repartition the mutated state before reevaluating the next substep.
8. Publish independently reconstructable energy, solid-to-liquid, and liquid-
   disposition ledgers.

Current Rust is not complete enough to implement this target: sensible heat is
zero in the Stage 3 carrier, precipitation advection is absent, positive excess
is reported rather than converted, and the target disposition below the exact
`1 kg m^-2` resolved-domain boundary is not adjudicated. Each item is a hard
implementation hold. No partial activation, proxy, coefficient fit, or default
change is authorized by 21N.
