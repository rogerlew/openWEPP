# Terminal phase operand lineage

Status: `EXECUTED / CANDIDATE A REJECTED`.

| Operand | Units/sign | Owner/basis | Research rule |
|---|---|---|---|
| beginning pack ice | `kg m^-2`, nonnegative | snow owner, support start | immutable input |
| beginning liquid | `kg m^-2`, nonnegative | snow liquid owner, support start | immutable input |
| deposition | `kg m^-2`, nonnegative | signed vapor transfer, exact support | credited once to solid; Candidate A makes it available to same-support melt |
| sublimation | `kg m^-2`, nonnegative | signed vapor transfer, exact support | debited once from beginning exposed solid; cannot exceed it |
| external liquid | `kg m^-2`, nonnegative | precipitation/liquid owner, exact support | credited once to liquid before possible refreeze |
| vapor latent energy | `J m^-2`, signed | carrier energy component, exact support | appears once inside complete energy; never added again by allocator |
| non-vapor energy | `J m^-2`, signed | remaining carrier components | prototype uses `complete - vapor_latent`; not an independent reconstruction |
| complete energy | `J m^-2`, signed | complete carrier, exact support | satisfies cold content first, then phase allocation |
| refreeze | `kg m^-2`, nonnegative | liquid debit/solid credit | only under remaining cold deficit; minimal-circulation solution forbids simultaneous net melt/refreeze |
| melt | `kg m^-2`, nonnegative | solid debit/liquid credit | bounded by beginning ice plus deposition less sublimation under Candidate A |
| ending cold content | `J m^-2`, nonnegative | snow owner | cannot coexist with unrefrozen available liquid in accepted allocator state |
| unallocated energy | `J m^-2`, nonnegative | terminal energy ledger | Candidate A enforces material complementarity with ending solid |
| surface frost | `kg m^-2`, nonnegative | Candidate B distinct hypothetical owner | never aliases liquid; Candidate B is rejected for material partition sensitivity |

Ran: solid, liquid, and phase-energy self-consistency is computed in
`snow_terminal_phase_competition.rs`. Independent review rejected the vapor
claim because the prototype neither carries surface temperature/latent heat
nor reconstructs `Q_latent = m_v L_s(T_s)` from an independent receipt.
