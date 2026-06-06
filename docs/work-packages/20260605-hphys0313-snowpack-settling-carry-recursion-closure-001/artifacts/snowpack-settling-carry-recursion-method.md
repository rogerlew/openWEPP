# HPHYS0313 Method

Status: complete

Evidence mode: ran

Static:

- Input ledger: HPHYS0312 prior-year terminal snowpack lineage ledger.
- Settling route: temporary fixed-comparator instrumentation added high-precision `H313_*` observe tags in `snowd.for`, including post-settling, branch input, and final cold-branch depth.
- Carry recursion route: existing HPHYS0305 fixed observe and openWEPP traces were scanned across calendar year 2014.
- Material thresholds remained `0.0005 m` depth and `0.5 kg m^-3` density.
- Instrumented observe tags are diagnostic evidence only; canonical source authority remains `/workdir/wepp-forest_260430_baseline`.

Ran:

- Built and ran a temporary fixed comparator with observe-off and observe-on lanes for H1, H7, and H39.
- Verified observe-on/off WAT output identity for the temporary instrumentation lanes.
- Wrote split-route ledger, summary, method, source-lineage, instrumentation patch, identity, and command log artifacts.
