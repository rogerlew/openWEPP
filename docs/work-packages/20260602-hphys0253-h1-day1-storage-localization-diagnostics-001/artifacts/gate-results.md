# Gate Results

Status: complete

Evidence mode: ran

Ran:

| Gate | Result | Evidence |
|---|---|---|
| `cargo build -p openwepp-runner --bin openwepp-cli-hill` | pass | `/tmp/hphys0253_20260602T203448Z/reports/build_status.tsv` |
| H1 trace run with `OPENWEPP_HPHYS0245_TRACE_MAX_DAYS=1` | pass `rc=0` | `/tmp/hphys0253_20260602T203448Z/reports/h1_trace_status.tsv` |
| H1 day-1 localization report | complete | `/tmp/hphys0253_20260602T203448Z/reports/h1_day1_storage_localization.md` |
| H1 day-1 conservation audit | complete | `/tmp/hphys0253_20260602T203448Z/reports/h1_day1_conservation_audit.md` |
| Full `H1..H39` runtime batch | pass `39/39` | `/tmp/hphys0253_20260602T203448Z/reports/hillslope_batch_status.tsv` |
| Full `H1..H39` semantic reports | completed `39/39`, semantic pass `0/39` | `/tmp/hphys0253_20260602T203448Z/reports/semantic_status.tsv` |
