# Source And State Reconciliation

Status: `complete`.

Evidence class: Static.

## Primary-source outcome

| Question | Authority and result | Disposition |
|---|---|---|
| Atmospheric longwave | Flerchinger et al. (2009), corrected Table 1 and Tables 2/9, supplies one complete Dilley-O'Brien plus Unsworth-Monteith route and daily clearness bounds. Hourly temperature is retained for the nonlinear flux evaluation; daily vapor pressure and cloud fraction are held across the day. | bind with no-clamp derived-emissivity guard |
| Forest longwave composition | Essery et al. (2008), Rutter et al. (2023), and FSM2 support a complementary sky/canopy partition; canopy emission replaces obscured sky. | bind |
| Diffuse sky view | FSM2 Eq. 13-14 gives direct and diffuse exponential transmission with a `1.6` diffuse multiplier. | bind through analytical elimination |
| Canopy/snow emissivity | FSM2 and Rutter support effective-unity exchange for the selected stand-scale route. | bind exactly one |
| Canopy temperature | Rutter supports a stand approximation but shows temperature choice matters; open-air temperature is only an explicit approximation. | bind limitation, not a prognostic claim |
| Trunks/gaps | EB-01A found explicit trunk and gap physics useful locally but outside the initial homogeneous-stand candidate. | exclude |

## Current openWEPP state

| Surface | Static finding | EB-02 treatment |
|---|---|---|
| `canopy_cover_fraction` | Daily native canopy output combining a structural cover floor and biomass-derived foliar cover, capped below one. | canonical input `C` |
| `leaf_area_index` | Seasonal native canopy output derived from maximum LAI and foliar activity. | diagnostic provenance; do not add again |
| `structural_canopy_cover_fraction` | Parameter used as an effective cover floor, not stem-area index. | already represented in `C` |
| canopy height | Existing state with no admitted independent role in homogeneous Beer-law longwave transmission. | diagnostic only |
| daily solar forcing | Available for daily clearness inference. | bind units and daylight guard |
| legacy hourly cloud fraction | A repeated daily legacy scalar using a different mapping. | do not silently reuse |
| Stage-B snow temperature | Air-temperature-capped diagnostic surface. | nonequivalent candidate; no implicit selection |
| frost `tmpadj/surtmp` | Hourly adjusted surface temperature for frost heat flow. | nonequivalent candidate; no implicit selection |
| multilayer thermal state | Opt-in layer temperature and cold-content state. | nonequivalent candidate; EB-03 adjudicates |

## Reconciliation

The canopy mapping prerequisite is closed by deriving
`f_sky=(1-C)^1.6`. Atmospheric and exchange equations are closed at contract
level. Two runtime prerequisites remain deliberately open: a polar-night cloud
policy and one coherent snow/canopy temperature provider. They prevent runtime
activation but do not prevent canonical equation authority.

No production file is edited by EB-02.
