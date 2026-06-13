# worker handoff

Status: M-B handoff ready

Evidence mode: Ran + Static

## Summary

M-B retired the multi-OFE hydrology execution blocker:

- Current openWEPP completes H1-H36 with 36/36 exit code `0`.
- All 29 multi-OFE surfaces now complete the full 2192-day run.
- Single-OFE anchors H8/H15/H19/H20/H22/H23/H28 stayed byte-identical to M-A outputs.
- Local owcmp execution passed, but semantic comparison remains failed due WAT row-key/per-OFE publication structure. That is M-C scope.
- No comparator subagent was used; comparisons were run locally per operator direction.

## M-A summary

M-A established the current execution boundary and the legacy routing calibration:

- Current openWEPP passes all seven 1-OFE H surfaces.
- Current openWEPP fails all 29 multi-OFE H surfaces before publication.
- Legacy H1-H36 WAT outputs preserve downstream `UpStrmQ`/`SubRIn` handoff to printed precision.
- No 15-OFE `pw0.wat.dat` exists for WAT closure.

## Local evidence left on disk

Local-only temp lane:

- `/tmp/openwepp_mofe01_mb/current_after_fix/exit-codes.tsv`
- `/tmp/openwepp_mofe01_mb/current_after_fix/logs/H*.stderr.txt`
- `/tmp/openwepp_mofe01_mb/current_after_fix/manifests/H*.json`
- `/tmp/openwepp_mofe01_mb/output/H*.{hbp,loss.json,plot.parquet,wat.parquet}`
- `/tmp/openwepp_mofe01_mb/owcmp_after_fix/summary.json`
- `/tmp/openwepp_mofe01_mb/owcmp_after_fix/reports/semantic/H*.semantic.json`
- `/tmp/openwepp_mofe01_ma/current/exit-codes.tsv`
- `/tmp/openwepp_mofe01_ma/current/logs/H*.stderr.txt`
- `/tmp/openwepp_mofe01_ma/current/manifests/H*.json` for passing 1-OFE surfaces.
- `/tmp/openwepp_mofe01_ma/output/H*.wat.parquet` for passing 1-OFE surfaces.

These are not committed artifacts.

## Next worker focus

Start M-C with per-OFE WAT publication closure. Contract the row-key/publication semantics first, then make the local owcmp semantic row-key failures meaningful. Do not tune hydrology deltas while publication structure is still the dominant failure.
