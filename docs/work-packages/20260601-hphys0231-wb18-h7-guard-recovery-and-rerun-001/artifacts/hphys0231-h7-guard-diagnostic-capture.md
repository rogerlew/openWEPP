# HPHYS0231 H7 Guard Diagnostic Capture

Status: completed  
Evidence mode: Ran

## Reproduction

Command:
- `target/debug/openwepp-cli-hill --run-dir /tmp/hphys0230_20260601T183925Z/parity/runs --run-file /tmp/hphys0230_20260601T183925Z/parity/runs/p7_openwepp.run --output-dir /tmp/hphys0230_20260601T183925Z/parity/hillslope_output`

Observed failure:
- `HKERNEL-WB11-PERC-E-003`
- `sim_day_index=1`
- `calendar_year=2013`
- `julian_day=1`

Captured WB18 diagnostic terms for failing layer:
- `L0007(fc=0.0000000000, ul=0.2436690111, theta=0.0401282821, ratio=0.0000000000, stz=0.1646835676, dynamic_branch_active=true, thetfc=0.0100000000, thetdr=0.0100000000, dg=0.5600000000, por=0.4451232341, cpm=0.2445993687)`

## Triage Conclusion

1. `fc=0` is not a seeding bug; it is the authoritative projection
   `fc = (thetfc - thetdr) * dg` with `thetfc == thetdr`.
2. Baseline lineage (`watbal.for`) explicitly maps non-positive `FC/UL` ratio
   to `hk=0` rather than hard-fail.
3. Corrective action is branch-guard placement and explicit legacy-degenerate
   branch handling (`Bi=0`), not runtime seed normalization.

## Post-Fix Verification

Command (same `H7` runfile) exits `rc=0` after WB18 guard-placement correction:
- `target/debug/openwepp-cli-hill --run-dir /tmp/hphys0230_20260601T183925Z/parity/runs --run-file /tmp/hphys0230_20260601T183925Z/parity/runs/p7_openwepp.run --output-dir /tmp/hphys0230_20260601T183925Z/parity/hillslope_output`

Measure mapping:
- `MEASURE-HP231-001`: satisfied.
