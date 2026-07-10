# Coverage Closure

Ran: ADR-0021 coverage closure recorded for characterization-test addition.

Tier: glue parser / binary payload validator.

Rationale: the baseline high-CRAP row,
`parse_non_runoff_event_payload`, had 0% line coverage in the fresh batch
baseline. This package adds a direct binary fixture that exercises the missing
non-runoff subevent payload path through the public HBP parser API.

Obligation-to-test map:

| Obligation | Evidence |
|---|---|
| Event kind `1` routes to non-runoff subevent parsing. | `latest_event_state_represents_non_runoff_subevent_payload` |
| Non-runoff payload state is exposed as `HbpLatestEventState::NoEvent`. | same test |
| `source_event_kind` records `HbpNoEventKind::Subevent`. | same test |
| Six scaled non-runoff payload fields decode to expected public values. | same test |
| Compatibility latest-event-payload API does not synthesize runoff payloads from non-runoff events. | same test |

Closure metrics:

| Function | Baseline coverage | Final coverage | Baseline CRAP | Final CRAP |
|---|---:|---:|---:|---:|
| `parse_non_runoff_event_payload` | 0.000% | 100.000% | 182.000 | 13.000 |
| `parse_runoff_event_payload` | 91.667% | 91.667% | 21.255 | 21.255 |

Per-function target disposition: all target-module production functions are
`<= 30` CRAP in the final full-workspace CRAP JSON.
