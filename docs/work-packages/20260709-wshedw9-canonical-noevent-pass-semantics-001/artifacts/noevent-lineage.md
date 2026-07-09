# NoEvent Lineage

Status: `EXECUTED-COMPLETE`
Evidence: `Static`

| State / field | Source | Runtime target | Policy |
|---|---|---|---|
| Latest day state | HBP day-directory order plus validated payload | `HbpLatestEventState` | Latest represented day overwrites prior event state. |
| Runoff `EVENT` | `event_kind = 2` payload | `HbpLatestEventState::EventPayload` | Existing runoff/sediment payload semantics. |
| `NO_EVENT` | `event_kind = 0` payload | `HbpLatestEventState::NoEvent` | Valid no-runoff/no-sediment state with required state snapshots. |
| `SUBEVENT` | `event_kind = 1` payload | `HbpLatestEventState::NoEvent` with source kind retained | No full runoff/sediment event; current watershed routing zero-fills surface runoff/sediment while preserving parsed non-runoff fields. |
| Surface runoff/sediment for valid no-event | typed no-event state | `HillslopeContribution` zero fields | Authorized typed consequence, not optional-payload fallback. |
| Baseflow/deep seepage | scaled HBP `gwbfv`/`gwdsv` fields | `HillslopeContribution.generated_baseflow_m3` / `groundwater_deep_seepage_m3` | Parsed non-negative volumes; never synthesized from absence. |
