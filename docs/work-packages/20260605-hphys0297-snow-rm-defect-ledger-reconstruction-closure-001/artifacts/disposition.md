# Disposition

Status: executed-hold
Evidence mode: Static + Ran

Ran:
- Contract-derived HPHYS0297 test passed.
- Full H1..H39 suite ran.
- H1/H7/H39 target traces ran.
- Per-window reconstruction ledger ran.

Decision:
- Do not re-tier any HPHYS0297 target window.
- Do not patch WB17, WB18, WB19, WB13, WB12, or WB14 in HPHYS0297.
- Carry all nine target windows as `UNRESOLVED`.

Hold reason:
- The six corrected-negative-melt candidate windows do not reconstruct
  `/workdir/wepp-forest_260430_baseline` baseline `RM` to the named `2.000 mm`
  tolerance.
- The three spring-2016 windows have immaterial negative raw melt and remain
  snow/winter producer magnitude/timing holds.
- Full semantic parity remains `0/39`.
- Dual independent review and verification artifacts are not complete.

Recommended continuation:
- Scaffold HPHYS0298 to localize the missing winter producer term/timing
  lineage that accounts for the reconstruction residual not explained by the
  pinned negative-melt branch alone.
- Focus first on first-2013 and spring-2014 windows where reconstruction misses
  baseline by `4.997207` to `45.281781 mm`, then continue spring-2016
  accumulation/carry-state timing.
- Keep WB17/WB18/WB19/WB13 downstream compensation prohibited.
