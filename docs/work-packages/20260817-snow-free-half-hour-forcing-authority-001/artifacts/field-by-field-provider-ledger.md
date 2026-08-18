# Field-by-Field Provider Ledger

| Field family | Sole provider | Refinement |
|---|---|---|
| air temperature | SIMIMPL28 hourly parent | hold twice |
| horizontal global shortwave | daily `radmj` + horizontal `radcur/rpoth` | half energy, same flux |
| VIS/NIR direct/diffuse | Weiss--Norman on horizontal parent | same parent flux twice |
| cloud fraction | SIMIMPL28 effective daily cloud | hold |
| downward longwave | atmospheric Dilley--Unsworth | evaluate hourly, hold twice |
| pressure | FAO-56 climate-station elevation | static |
| vapor pressure/specific humidity/VPD | daily dew point + hourly air + pressure | evaluate hourly, hold twice |
| wind/dew point | existing daily climate | hold |
| precipitation | exact breakpoint overlap; parent-hour fallback only | integrate/split |
| CO2 | explicit run configuration | hold |
| reference height | digest-bound aerodynamic config | hold |
| GSI | admitted daily GSI operator | hold |
| WB14 | typed static OFE config | static |
| soil/surface/canopy/CN/BGC/runon | live scientific owners | never forcing-interpolated |

