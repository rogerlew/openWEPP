# Terminal phase operand lineage

Status: `EXECUTED / RESEARCH CANDIDATE A FROZEN FOR REVIEW`.

| Operand | Units/sign | Owner/basis | Research rule |
|---|---|---|---|
| beginning pack ice | `kg m^-2`, nonnegative | snow owner, support start | immutable input |
| beginning liquid | `kg m^-2`, nonnegative | snow liquid owner, support start | immutable input |
| deposition | `kg m^-2`, nonnegative | signed vapor transfer, exact support | credited once to solid; Candidate A makes it available to same-support melt |
| sublimation | `kg m^-2`, nonnegative | signed vapor transfer, exact support | debited once from beginning exposed solid; cannot exceed it |
| external liquid | `kg m^-2`, nonnegative | precipitation/liquid owner, exact support | credited once to liquid before possible refreeze |
| vapor latent energy | `J m^-2`, signed | carrier energy component, exact support | appears once inside complete energy; never added again by allocator |
| non-vapor energy | `J m^-2`, signed | remaining carrier components | `complete - vapor_latent` diagnostic reconstruction only |
| complete energy | `J m^-2`, signed | complete carrier, exact support | satisfies cold content first, then phase allocation |
| refreeze | `kg m^-2`, nonnegative | liquid debit/solid credit | only under remaining cold deficit; minimal-circulation solution forbids simultaneous net melt/refreeze |
| melt | `kg m^-2`, nonnegative | solid debit/liquid credit | bounded by beginning ice plus deposition less sublimation under Candidate A |
| ending cold content | `J m^-2`, nonnegative | snow owner | cannot coexist with unrefrozen available liquid in accepted allocator state |
| unallocated energy | `J m^-2`, nonnegative | terminal energy ledger | Candidate A enforces material complementarity with ending solid |
| surface frost | `kg m^-2`, nonnegative | Candidate B distinct hypothetical owner | never aliases liquid; Candidate B is rejected for material partition sensitivity |

Ran: independent solid, liquid, energy, and vapor-component reconstructions are
computed in `snow_terminal_phase_competition.rs`; exact self-consistency is
supplemented by vapor-double-count and sign/split poisons.
