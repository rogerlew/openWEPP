# Equation Authority Ledger

Evidence class: `Static`; accepted review findings are closed by dual terminal
review. A row selects only the named equations/branch. Source parameter
defaults and unlisted snow, stability, shortwave, or transport alternatives
are rejected.

| Family | Selected equation/branch | Exact source locator | openWEPP selection/ownership |
|---|---|---|---|
| canopy shortwave | V7 exact Sellers/CLM two-stream with ground VIS/NIR lower boundary | `SC-VEGETATION-001@11`, E01--E03 and V2/V3/V4 amendments | unchanged V8 full-column solution; direction/band/component identity retained |
| canopy longwave recurrence | `tau_i=exp(-0.8*Omega_i*(LAI_i+SAI_i))`, unit-emissivity, no-reflection reciprocal downward/upward flux | R-157 §2.3, Eqs. (14), (25)--(34), pp. 3587--3588; package extinction selection | arbitrary rank, clumping once, current trial temperatures, no stale upward-ground forcing |
| arbitrary-rank longwave structure | multilevel transfer/source matrix and boundary identities | R-158 supplement §S2.7.1, Eqs. (S2.16)--(S2.24), pp. 6--8; §S4, Eqs. (S4.1)--(S4.12), pp. 21--23 | V8 no-reflection specialization; exact structural transmission and top/ground boundaries |
| longwave component ownership | incoming absorption by component area; emission by that component temperature | R-158 Eqs. (S2.16)--(S2.22); R-157 Eqs. (28), (31)--(34) | source-resolved sun/shade/wet/stem net; unequal-temperature components never receive a repartitioned aggregate residual |
| canopy/ground turbulent topology | canopy, canopy-air, ground and atmosphere heat/vapor network | R-155 §§2.2--2.6, Eqs. (4)--(28), Fig. 1 and Table 2 | one OFE-local tile canopy-air node; open tile exact neutral reduction |
| vegetation--canopy-air resistance | canopy conductance and resistance | R-155 §2.6.1, Eqs. (50)--(53), pp. 853--854 | V8 component conductances remain component-resolved; no snow-burial or free-convection branch |
| ground--canopy-air resistance | neutral eddy-diffusivity path | R-155 §2.6.2, Eqs. (54)--(63), pp. 853--854 | neutral `psi_H=1`; complete configured geometry; no stability correction/floor/default |
| soil vapor resistance | Sellers soil resistance | R-155 §2.6.3, Eq. (67), p. 855; R-153 Eqs. 2.5.72--2.5.81 | bare mineral source only; litter branch sets direct mineral evaporation unavailable rather than donating demand |
| joint canopy-air energy/vapor | implicit canopy-air and atmosphere elimination | R-155 Appendix I, Eqs. (I1)--(I20), pp. 865--867 | V8/LSE ordered joint residual; ground appears exactly once |
| forest-litter energy | `C_l*dT_l/dt=Rn-H-LE-G+Q_adv/dt` | R-156 Appendix A, Eq. (A1) | hydrology owns litter water mass; LSE owns thermal state |
| litter humidity | cosine relative-humidity response | R-156 Appendix A, Eqs. (A8)--(A9) | exact capacity domain and dry/wet branches |
| litter conduction | series litter/top-soil resistance | R-156 Appendix A, Eq. (A10) | outward `G_down`; surface alias `-G_down`, soil receipt `+G_down` |
| litter capacity/conductivity | dry-plus-liquid heat capacity and water-dependent conductivity | R-156 Appendix A, Eqs. (A13)--(A14) | accepted mass/enthalpy storage form; no duplicated litter mass |
| bare-soil humidity and evaporation | CLM surface humidity and dry-layer resistance | R-153 Eqs. 2.5.72--2.5.81 | exact configured top source layer; frozen branch unsupported |
| soil thermal column | 1-D heat conservation and Crank--Nicolson discretization | R-153 §§2.6.1--2.6.3, Eqs. 2.6.1--2.6.49 | actual openWEPP interfaces, harmonic conductivity, zero lower flux |
| soil thermal properties | configured mineral/organic heat capacity and conductivity | R-153 Eqs. 2.6.75--2.6.91 | evaluate from immutable beginning hydrology/soil state; no site defaults |
| latent heat | `L_v(T)=2.501e6-2369*(T-273.15)` `J kg^-1` | R-153 thermodynamic constants/latent-flux family; package canonical selection | finalized signed vapor amount only; no double latent conversion |
| liquid sensible enthalpy | `h_l=C_w*(T-273.15)` and mass-weighted enthalpy mixing | R-156 Eq. (A14) for `C_w`; first-law package selection | exact source carry and paired receiver custody |
| precipitation temperature | `hydrometeor_temperature_c+273.15` | `openwepp_meteorology::phase::hydrometeor_temperature_from_relative_humidity`; `SC-SNOWFREEZE-001` Harder--Pomeroy addendum | required forcing; no air/freezing/soil substitution |
| runon/infiltration/runoff advection | source temperature and exact enthalpy carry | first-law selection in `advected-energy-convention.md` | upstream/downstream and surface/soil receivers share one route identity |
| evaporation request | positive potential vapor amount by source key | `SC-VEGETATIONTRANSACTION-001@2` generalized owner protocol | immutable D/A/F identity; no complement-of-canopy demand |
| condensation | signed `E_out<0`; `C_cond=-f_tile*E_out*dt` | R-153 Eqs. 2.5.140--2.5.148 liquid branch; ownership selection | positive hydrology store credit, separate from withdrawal |
| fixed water caps | source-specific complementarity in final joint residual | `SC-VEGETATION-001` V5/V6 cap authority generalized to ground keys | cap plus independently reconstructed final inventory; no reauthorization |
| surface/soil storage | accepted beginning/end sensible enthalpy | R-153 §§2.6.1--2.6.3; R-156 A1/A14 | thermal owner only; variable water capacity uses accepted mass/enthalpy state |
| closure | component, control-volume, mass/latent, ground-pair and route advection | `SC-LANDSURFACEENERGY-001` conservation authority | independent receiver validators consume primitive operands, never producer residuals |

No row admits an executable parameter default. Complete caller configuration,
state, and forcing fields are frozen in
the six `lse_v1_*_schema.json` artifacts; any source equation requiring a value
absent from that schema remains a release blocker.
