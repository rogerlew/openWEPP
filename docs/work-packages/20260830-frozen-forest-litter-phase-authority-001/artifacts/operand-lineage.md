# Operand lineage

Status: `FROZEN BEFORE PRODUCTION`

Evidence mode: `Static`

All masses are destination areal `kg m^-2`; energies are destination areal
`J m^-2`; rates are derived only by division by the exact support seconds.

| Operand | Unit/basis | Source authority | Owner/role | Acceptance reconstruction |
|---|---|---|---|---|
| `W_l,b`, `W_i,b` | kg m^-2, beginning destination litter | surface owner V2 | authoritative immutable availability | exact nonnegative beginning pools and identities |
| `T*`, `U*` | K; J m^-2, post-solve/post-vapor/pre-phase litter | LSE V3 | authoritative phase input | `U*` reconstructed from dry + remaining phase masses and signed vapor enthalpy |
| liquid vapor mass | kg m^-2, signed | R-156 A7-A9 liquid component | authoritative liquid-only debit/credit | outbound <= `W_l,b`; inbound credits liquid only |
| ice vapor mass | kg m^-2, signed | R-156 A7-A9 frozen component | authoritative ice-only debit/credit | outbound <= `W_i,b`; inbound credits ice only |
| liquid vapor enthalpy | J kg^-1 | `C_w(T-T_ref)+L_v(T)` | authoritative energy operand | signed mass times exact liquid enthalpy |
| ice vapor enthalpy | J kg^-1 | `C_i(T-T_ref)+L_s(T)` | authoritative energy operand | signed mass times exact ice enthalpy; never liquid latent alias |
| `m_freeze`, `m_melt` | kg m^-2 | R-156/SURFEX bounded kinetic law | authoritative atomic transfer | equal liquid debit/ice credit or reverse; capacity uses `0.85*rho_w*dz` |
| fusion energy | J m^-2 | `L_f*(m_freeze-m_melt)` | authoritative LSE credit/debit | independently reconstruct exact sign and magnitude |
| `U_end`, `T_end` | J m^-2; K | selected conservative enthalpy realization | authoritative ending/warm-start state | `U_end=U*+L_f*m_freeze-L_f*m_melt`; divide by ending phase heat capacity |
| current ingress | kg m^-2 and J m^-2 | existing sealed ingress parcels | authoritative post-phase credit | exact parcel set, admitted after phase; cannot fund vapor/phase |
| WB14 supply | kg m^-2 liquid only | SC-WATBAL-001 chronology | authoritative hydrology debit | excludes all litter ice; current ingress only at existing chronology |
| frozen fraction | dimensionless | R-156 phase fraction | constitutive, not owner state | exact zero for empty; otherwise `W_i/(W_l+W_i)`; no `xwgmin` floor |
| closure residuals/diagnostics | native units | independent validator | diagnostic only | may report evidence but never substitute for operand reconstruction |

Required anti-alias mutants include `rho_i` in the capacity operand, `L_v` for
ice vapor, saturation over ice, total-pool outbound vapor caps, phase before
vapor, phase after ingress, old-capacity temperature, instantaneous equilibrium,
same-support re-solve, ice-to-WB14 donation, soil-`frozwt` donation, hidden
tiny-ice cleanup, and receipt/digest substitution. Every mutant must differ
numerically or fail a named structural guard on at least one canonical vector.
