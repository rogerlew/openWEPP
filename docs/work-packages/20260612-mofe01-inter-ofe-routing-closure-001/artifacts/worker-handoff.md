# worker handoff

Status: M-A handoff ready

Evidence mode: Ran + Static

## Summary

M-A established the current execution boundary and the legacy routing calibration:

- Current openWEPP passes all seven 1-OFE H surfaces.
- Current openWEPP fails all 29 multi-OFE H surfaces before publication.
- Legacy H1-H36 WAT outputs preserve downstream `UpStrmQ`/`SubRIn` handoff to printed precision.
- No 15-OFE `pw0.wat.dat` exists for WAT closure.

## Local evidence left on disk

Local-only temp lane:

- `/tmp/openwepp_mofe01_ma/current/exit-codes.tsv`
- `/tmp/openwepp_mofe01_ma/current/logs/H*.stderr.txt`
- `/tmp/openwepp_mofe01_ma/current/manifests/H*.json` for passing 1-OFE surfaces.
- `/tmp/openwepp_mofe01_ma/output/H*.wat.parquet` for passing 1-OFE surfaces.

These are not committed artifacts.

## Next worker focus

Start M-B with red tests for the day-2 MOFE `runoff_reconciliation` failure. Do not begin by tuning comparator deltas. The first useful proof is that representative 2, 3, 4, and 5 OFE hillslopes execute past day 2 and publish enough state to inspect downstream runon.
