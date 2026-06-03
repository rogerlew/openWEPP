# Spring Snowpack Lineage Diagnosis

Status: completed/HOLD
Evidence mode: Static + Ran

Static:

- HPHYS0267 correctly redirected work away from local WB17/WB19 threshold seams and toward snowpack/SWE/`RM`.
- Baseline non-ag parity keeps frost disabled, but snow remains active.
- Baseline `melt.for` permits negative hourly melt before `winter.for` daily redistribution; openWEPP currently exports non-negative hourly melt without that daily redistribution.

Ran:

- Final run root: `/tmp/hphys0268_final_20260603T174015Z`.
- H1 first material `Ep` divergence: Julian 99, `Ep diff=-1.027612 mm`.
- H7 first material `Ep` divergence: Julian 99, `Ep diff=-1.010480 mm`.
- H39 first material `Ep` divergence: Julian 115, `Ep diff=-1.139851 mm`.
- H1/H7/H39 trace closure is internally consistent after stale inactive-day snow hourly surfaces are cleared.
- Semantic divergence remains dominated by snowpack magnitude/timing: candidate SWE near zero while baseline still stores large snowpack in spring.

Conclusion:

- The next package should port baseline daily melt redistribution/early-melt timing, not patch WB17 `Ep`.
