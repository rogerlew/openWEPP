# Protected operand and output map

Static: prospective. Numerical/output parity has not yet executed.

| Field | Operand/source | Units/basis | Independent comparison |
|---|---|---|---|
| Lane-D source | WAT5 hourly accepted source depths, associated WAT Area per OFE | mm and m2; sum depth/1000*area | compare manifest total_source_m3 and reject mm-as-m poison |
| Outlet | HBP 24 hourly volume bins; PASS runvol | m3, terminal outlet only | sum HBP bins, compare PASS and manifest total_routed_outlet_m3 |
| End storage | manifest total_end_window_storage_m3, accepted routing book | m3, complete routed topology | source + clamp - independently reconstructed HBP outlet - storage |
| Clamp | manifest total_clamp_m3 | m3, diagnostic custody correction | exact parity, unchanged no artificial normalization |
| Peak | maximum HBP hourly volume /3600; PASS peakro | m3/s, hourly reporting basis | compare both output surfaces |
| Topology/area | authored exact CompleteOwner seed and WAT Area | ordered OFE identity, m2 | exact count/order and area association |
| Day/support/trials | real committed qualification snapshot and adaptive telemetry | counts, actual source semantics | exact A/F/R except identified lower-level mechanism counts |
| Scientific output files | each emitted nonmanifest HBP/Parquet/output file | original serialization | SHA256 per path, byte exact |
| Run manifest | full raw JSON retained | field-defined units | explicit volatile-field allowlist only, to freeze from baseline schema before comparative admission |

Authority: existing SC-OFEROUTE-001/SC-WATBAL-001 routing closure, SC-SNOWENERGY-001
INV088/C056, SC-LANDSURFACEENERGY-001 INV164/C020. Preserve all inner mass/energy,
phase, liquid, soil and receipt independent reconstruction and forced-reference
tests required by those contracts. Output equality alone does not satisfy inner
process closure. Calibration/identifiability NOT_APPLICABLE: no process parameter,
empirical calibration or observation operator changes.

Runner reuses assert_real_lane_d_public_closure with existing source tolerance
1e-12 m * summed area and publication relative comparison 1e-9. These tests are
inherited authority checks, not new physics tolerances. Raw operands are emitted
so independent verifier reconstructs without reading producer counters.
