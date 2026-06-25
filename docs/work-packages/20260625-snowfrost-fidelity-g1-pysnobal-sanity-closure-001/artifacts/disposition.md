# Disposition

Status: complete.

Evidence mode: Static + Ran.

Final route: `PROCEED-SNOWFROST-FIDELITY-G1-SANE-SITE-LANES`.

Static: G1 did not change production snow, frost, hydrology, erosion, runtime
activation, observation tolerances, or science-contract thresholds.

Static: PySnobal remains diagnostic hypothesis evidence only. Observed snow
depth and `SC-SNOWFREEZE-001#INV-SNOWFREEZE-048` remain the correctness
authority for snow-depth fidelity; PySnobal agreement or disagreement cannot
classify openWEPP as defective by itself.

Ran: current openWEPP snow comparison rows are now available for all five
frost-depth pilot sites. The rows are extracted from compatibility WAT
`Snow-Water` and `Snow-Depth`, and G1 corrected the date mapping to use
external climate dates from `sim_day_index`.

Ran: the selected `Tg=0.0 degC` PySnobal lane produces sane SWE and physical
snow depth for all five pilot sites. The harness route is
`PROCEED-SNOWFROST-FIDELITY-G1-SANE-SITE-LANES`.

Ran: the original Morris `Tg=-0.5 degC` full-lane failure still reproduces the
same PySnobal C-layer `sati.c` abort. A January 1980 window on that lane passes,
so the failure is state/history/proxy-lane dependent rather than a malformed
forcing row at the export boundary. This failed sensitivity lane remains
visible under strict `all-lanes` policy and does not block the explicit
site-sane comparator route.

Assessment: PySnobal snow physics should remain on the table for diagnostic
comparison. It is capable of producing sane SWE and physical snow-depth
surfaces for all five pilot sites under the selected simple ground-temperature
lane, and the comparisons against current openWEPP are now metric-bearing. The
next decision should compare PySnobal snow-depth behavior against observed snow
controls and openWEPP/legacy residual direction before considering any
incorporation into openWEPP architecture.
