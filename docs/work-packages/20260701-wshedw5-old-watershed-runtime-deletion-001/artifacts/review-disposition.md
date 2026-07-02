# Review Disposition

Status: `executed`

Evidence mode: `static + ran`

| Finding | Severity | Disposition | Evidence |
| --- | --- | --- | --- |
| Protected WS11/WS12 coverage not restored | High | Accepted, fixed | Expanded W5 typed contract to 8 tests; focused nextest passed. |
| Package artifacts still queued/not-run | High | Accepted, fixed | Evidence artifacts updated with W5 results. |
| Source guard too narrow for included kernel files | Medium | Accepted, fixed | Runner guard scans included kernel helper files; focused runner test passed. |
| WS12 projection errors collapsed to domain | Medium | Accepted, fixed | `direct.rs` maps WS12 runtime-input non-finite/domain separately; focused tests passed. |
| Stale chan.inp/channel runtime-input taxonomy retained | Medium | Accepted, fixed | `WatershedRuntimeInputError` trimmed to live WS12 variants. |
| No surrogate/provisional production physics found | Info | Recorded | Direct WS11/WS12/WS18/WS20 helper path retained; tests assert anti-surrogate behavior. |

No accepted review finding remains open.
