# Test Design

Status: `PASS`

Evidence mode: `Static`

The existing `mt3_hbp_hourly_consumer_contract` binary supplies seven real-CLI
tests without a W11E code edit:

| Surface | Cases and rejected alias |
|---|---|
| protected hourly HBP consumer | one- and two-channel production dispatch remain active |
| zero-count `chan.inp` | parsed 600-second result matches positive-count 600-second control and differs from 60-second default candidate |
| CREAMS serial publication | channel 2, 7,200 m3, and 240 kg reject internal-throughflow sum/channel 1 aliases |
| inadmissible MC | 4 branch/grid combinations x 5 scenarios; 4 zero controls execute and 16 active cases preserve typed E003 |
| admitted MC | static and dynamic 60-second routes execute with finite passive peaks and closed balance |
| KW/CREAMS sanity matrix | 15 zero/spike/spread/uniform/late public observations across KW 3,600/600 seconds and event-scalar CREAMS |

The KW/CREAMS matrix reconstructs external HBP totals from serialized payloads
and reads terminal Parquet surfaces. Public `Balance` is supporting evidence;
W11D's independent spatial Manning storage, fresh-day operand, dry-carry, and
last-slot vectors provide the non-tautological correction proof. Debug and
exact-release executions must both pass all seven tests.
