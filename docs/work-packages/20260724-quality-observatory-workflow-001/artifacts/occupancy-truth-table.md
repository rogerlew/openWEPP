# Occupancy Truth Table

Evidence class: Static and deterministic fixture tests.

| Provider observation | Classification | Workflow effect |
| --- | --- | --- |
| No nonterminal TESTGATE run | `CLEAR` | Continue to the next admission check |
| Current TESTGATE `requested`, `waiting`, `pending`, `queued`, or `in_progress` | `LIVE_TESTGATE` | `DEFERRED_TESTGATE_PRIORITY` |
| Exact retired Omarchy ID/head/workflow/event/conclusion with zero jobs and artifacts | `CLEAR` with ignored ID | Continue without cancellation or waiting |
| Any Omarchy field drift | `UNKNOWN` | `DEFERRED_OCCUPANCY_UNKNOWN` |
| Repository, workflow, labels, pagination, count, JSON, API, or authentication ambiguity | `UNKNOWN` | `DEFERRED_OCCUPANCY_UNKNOWN` |
| Provider snapshot exceeds its shared five-second deadline | `UNKNOWN` | Stop or defer fail closed |
| Lease already held | N/A | `DEFERRED_FOREST1_LEASE` |

The controller rechecks occupancy before and after the nonblocking lease,
while supervising the child, around independent publication verification, and
immediately before upload staging.
