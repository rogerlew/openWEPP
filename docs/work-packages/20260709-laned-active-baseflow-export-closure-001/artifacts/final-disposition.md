# Final Disposition

Status: `EXECUTED-COMPLETE`

Verdict: `COMPLETE-MT2-ACTIVE-BASEFLOW-EXPORT-CLOSURE`

M-T2 closed the M-T2B export holds by making the existing HBP trailing payload
pair authoritative as generated groundwater baseflow (`gwbfv`) and deep seepage
(`gwdsv`), wiring those fields through the HBP parser into watershed
contributions, and adding the watershed/channel `lr_bf=1` branch that consumes
generated HBP baseflow instead of substituting `chan.inp` `cbase`.

The `bftharea` threshold is now evaluated in hectares for current-step
hillslope contributions. Below-threshold generated side baseflow is suppressed
at the channel branch; upstream dependency channel baseflow remains the already
routed result of its own branch.

M-T3 is unblocked on the groundwater/baseflow export leg. It still owns the
broader active HBP hourly water/sediment watershed consumption path.
