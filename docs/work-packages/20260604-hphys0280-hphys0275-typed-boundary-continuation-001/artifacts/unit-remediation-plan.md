# Unit Remediation Plan

Status: completed
Evidence mode: static

Static:
- Follow-up 1: define a signed water-depth boundary wrapper if `snow.hourly.melt_raw_m_{idx4}` should become typed without rejecting corrected negative raw melt.
- Follow-up 2: migrate snow melt trace internals that publish inches/Fahrenheit/mph only after canonical conversion/alias requirements are specified.
- Follow-up 3: continue output Parquet metadata alignment under HPHYS0278 scope.
- Follow-up 4: address the pre-existing `pl14s` SIMIMPL18 workspace-test failure in a separate package; it is not caused by HPHYS0280 and blocks full-workspace GO.

Ran: not-run; remediation plan is static.
