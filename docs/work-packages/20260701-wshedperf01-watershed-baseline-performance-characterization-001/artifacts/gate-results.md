# Gate Results

Status: `UPDATED`

| Gate | Result | Evidence |
| --- | --- | --- |
| Required reading map complete | `PASS` | `artifacts/required-reading-map.md` |
| Environment/input inventory complete | `PASS` | `artifacts/environment-and-input-inventory.md` |
| Release openWEPP watershed binary built | `PASS` | `target/release/openwepp-cli-watershed`; `artifacts/baseline-command-log.md` entry #1 |
| Pinned legacy result or blocker recorded | `PASS` | `timing/legacy_pw0_timed_1.time` (Ran, exit `0`) |
| openWEPP routed-stage timing recorded | `PASS` | `timing/openwepp_watershed_from_hbp_timed_1.time`; `timing/repeat_openwepp_routed/openwepp_routed_{1,2,3}.time` |
| openWEPP practical full end-to-end timing recorded | `PASS` | `timing/openwepp_watershed_end2end_full_validated.time` |
| Scope labels and repeat counts recorded | `PASS` | `artifacts/watershed-baseline-timing.md` |
| Profiling/coarse attribution recorded | `PASS` | `artifacts/watershed-profile-attribution.md`, `timing/openwepp_watershed_perf_routed.time` |
| Architecture handoff complete | `PASS` | `artifacts/watershed-perf-architecture-handoff.md` |
| No production files edited | `PASS` | `git status --short` shows only package artifact + `docs/work-packages/README.md` updates |
| Final disposition written | `PASS` | `artifacts/disposition.md` |
