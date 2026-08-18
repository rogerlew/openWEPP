# Shortwave Method Comparison

Weiss--Norman is selected because it directly returns the required coherent
direct/diffuse visible/NIR partition from already-hourly global shortwave,
zenith, and pressure. Spitters-style daily-to-diurnal reconstruction is rejected
for V1 because SIMIMPL28 already owns daily-to-hourly disaggregation. Internal
SIMIMPL28 `sb/sd` and slope-adjusted `estrad` are rejected as exported component
forcing because their physical meanings and surface bases differ.

