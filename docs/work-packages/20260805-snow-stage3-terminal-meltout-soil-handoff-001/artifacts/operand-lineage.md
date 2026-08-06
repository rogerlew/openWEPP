# Operand Lineage

Status: complete / Phase-1 authority inventory / production gate blocked

Evidence class: Static contract and pinned-source reconciliation on 2026-08-06.
All energy quantities use an areal basis unless noted. `DIRECT` means the
current contract or pinned source defines the operand for the stated scope;
`INFERENCE` means conservation requires it but current authority does not
define the implementation; `MISSING` means the required producer, state, or
recipient is absent.

| Operand or state | Units/basis | Producer -> consumer | Authority | Phase-1 disposition |
| --- | --- | --- | --- | --- |
| Resolved-snow `Q_complete`, `Q_cold_required`, `Q_excess` | `J m^-2` per stability substep | Stage 3 carrier -> snow enthalpy/phase ledger | `DIRECT`: `SC-SNOWENERGY-001#INV-SNOWENERGY-029` | Admitted only while resolved snow remains. |
| Resolved-snow signed vapor and available ice | `kg m^-2` per stability substep | Stage 3 vapor exchange -> joint phase bound | `DIRECT`: `INV-SNOWENERGY-017/018/029` | Admitted; snow-surface flux expires at disappearance. |
| Outer `60/15/1 minute` cadence | seconds | mass-selected scheduler -> repeated flux evaluation | `DIRECT`: `INV-SNOWENERGY-023`; libsnobal `_divide_tstep.c` | Admitted outer cadence, not terminal convergence authority. |
| Terminal implicit/error-controlled enthalpy solve | state/error norm and seconds | shallow snow state -> localized event | `MISSING` | No algorithm, error tolerance, convergence rule, or failure contract. |
| Combined melt/sublimation exhaustion time | seconds within substep | snow mass/energy/vapor solve -> surface transition | `MISSING`; conservation need is `INFERENCE` | No admitted earliest-event localization or simultaneous deposition/precipitation chronology. |
| Residual solid conversion at `<=1 kg m^-2` | `kg m^-2` | libsnobal `m_s` -> `h2o_total` | `DIRECT` reference precedent in `_calc_layers.c` and `_adj_layers.c`; not openWEPP target authority | Threshold conversion is not an energy-conserving event algorithm. |
| Terminal retained-snow-liquid release | `kg m^-2` | snow liquid store -> surface-liquid supply | `DIRECT` reference precedent plus `INFERENCE` for openWEPP ownership | OpenWEPP has no named terminal release/energy ledger. |
| Actual cover, albedo, canopy, roughness, residue, frost inputs | fractions, `m`, state | runtime land state -> legacy `tmpadj` | `DIRECT` for the frost surface-temperature driver | Inputs exist; no canonical receiving-regime selector owns their post-meltout composition. |
| Snow-free shortwave, approximate longwave and sensible exchange | `W m^-2` | pinned `tmpadj` -> `surtmp` | `DIRECT` only for `SC-SNOWFREEZE-001#INV-SNOWFREEZE-006` frost forcing | Not an independently reconstructable complete land-surface energy balance. |
| Frozen-soil conductive/latent heat | `W m^-2`, `J m^-2` | `surtmp`/frost paths -> fine frozen-water/front state | `DIRECT`: `INV-SNOWFREEZE-006` | Admitted only for the frost-front formulation. |
| Unfrozen-soil sensible heat/enthalpy | `J m^-2` by layer | land surface -> prognostic soil thermal state | `MISSING` | `SC-SOIL-001` owns hydrologic/constitutive state, not soil thermal enthalpy. |
| Post-event evaporation/condensation water | `kg m^-2` over event remainder | receiving surface -> atmosphere | `MISSING` at event scale | `SC-EVAP-001` owns daily/final-hour ET, not an event-remainder vapor solve. |
| Post-event vapor latent energy | `J m^-2` | receiving-surface vapor mass -> energy ledger | `MISSING` | Snow-specific Stage 3 latent authority cannot survive the surface transition. |
| Snow-free precipitation-advected heat | `J m^-2` | rain/snow hydrometeor -> receiving surface | `MISSING` | Existing precipitation-heat authority is snow-carrier-specific. |
| Surface-water/ponding temperature and enthalpy | `K`/`degC`, `J m^-2` | terminal liquid/rain -> surface-water state | `MISSING` | Water contracts own depth/mass only. |
| Surface-liquid supply and snow drainage | `m` per event/day | terminal release/melt -> WB12/WB14 supply | `DIRECT` for existing `wmelt` lineage; terminal producer amendment missing | Must not alias hillslope runoff. |
| Infiltration | `m` and `m s^-1` | liquid supply -> WB18 layer storage | `DIRECT`: `INV-SNOWFREEZE-018`, `SC-RUNOFFPART-001`, pinned `grna.for` | Infiltration precedes runoff, but no energy follows the infiltrating water. |
| Ponding/depression storage and overflow | `m` | noninfiltrated supply -> storage/overflow | `DIRECT`: `SC-RUNOFFPART-001` | Mass recipient only. |
| Residual hillslope runoff | `m` | partition residual -> WB13/routing | `DIRECT`: `SC-RUNOFFPART-001` | Mass recipient only; runoff sensible-energy export is `MISSING`. |
| Persistent second snow mass/enthalpy state | `kg m^-2`, `J m^-2` across days | diagnostic shadow -> next-day shadow | `MISSING` and currently conflicting | `SC-SNOWFREEZE-001#INV-SNOWFREEZE-091` makes a second mass state closure-blocking. |
| Persistent parallel soil-water/frost/surface state | layer `m`, frozen `m`, energy | post-event shadow -> restart/next day | `MISSING` | Current contracts authorize one production state and exact handoffs, not a parallel coupled state. |
| Persistent vegetation/residue feedback | cover/mass/stress state across days | shadow soil water -> later cover/residue | `MISSING` | A seasonal divergence can feed plant/residue state; no bounded shadow ownership exists. |

## Anti-Alias Conclusions

- Snow-computed terminal excess is not recomputed land-surface energy.
- Snow drainage or terminal release is not residual hillslope runoff.
- Water-depth closure does not close water sensible energy.
- Frost-front latent heat does not provide unfrozen-soil sensible storage.
- Daily ET does not authorize an event-remainder vapor/latent solve.
- A diagnostic state reinitialized from CoE is not a coherent seasonal shadow.

The missing rows are current-scope blockers, not deferred evidence. No
production edit was made.
