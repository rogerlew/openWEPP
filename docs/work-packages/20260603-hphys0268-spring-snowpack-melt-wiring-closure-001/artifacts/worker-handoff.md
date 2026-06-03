# Worker Handoff

Status: completed/HOLD
Evidence mode: Static + Ran

Static:

- Start HPHYS0269 from `SC-SNOWFREEZE-001#INV-SNOWFREEZE-014` and `SC-WATBAL-001#INV-WATBAL-054`.
- Do not return to WB17 `Ep` yet.
- Next focus is baseline `winter.for` daily melt redistribution plus `snowd.for`/`melt.for` early melt timing.
- Key baseline lines:
  - `/workdir/wepp-forest_260430_baseline/src/winter.for` daily positive/negative melt redistribution around lines 414-464.
  - `/workdir/wepp-forest_260430_baseline/src/melt.for` allows negative hourly melt before daily redistribution around lines 275-300.
  - `/workdir/wepp-forest_260430_baseline/src/snowd.for` uses daily mean temperature branch around lines 112-180 and density gate around lines 240-281.

Ran:

- Final evidence root: `/tmp/hphys0268_final_20260603T174015Z`.
- Candidate vs baseline examples:
  - H1 day 99: candidate `Snow-Water=2.768750 mm`, baseline `144.340000 mm`.
  - H7 day 99: candidate `Snow-Water=2.768750 mm`, baseline `159.590000 mm`.
  - H39 day 115: candidate `Snow-Water=0.000000 mm`, baseline `141.230000 mm`.
- Earlier series probe showed openWEPP accumulates snow in winter but releases melt much too early around Julian 72-90.
