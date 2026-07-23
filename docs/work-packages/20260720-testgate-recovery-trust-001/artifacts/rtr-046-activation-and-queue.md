# RTR-046 Activation And Provider Queue Evidence

Evidence class: Ran unless labeled Static.

## Failed Trusted Run

- Run: `29978778150`
- Exact HEAD: `ba6c1e1d3f0e55b6b3c076002dfb343810e8e3f2`
- Event: `push`
- Result: `failure`
- `Restore and verify newest durable attempt history`: FAIL.
- `Execute content-verifiable increment gates`: SKIPPED.
- Jobs, TESTGATE nodes, receipts, HEAVY, CRAP, and retries executed: zero.

## Runner Activation

Ran: canonical `tools/ci/omarchy-runner/manage.sh setup` recreated the idle
forest1 runner with the already-reviewed contract.

- Provider runner: exactly one `forest1-openwepp-01`, online and idle.
- Labels: `self-hosted`, `Linux`, `X64`, `openwepp`, `forest1`, `trusted`.
- Image:
  `sha256:034ce655da139123cd775317d590d04dec6377788e4d124dc0e674f8d021e7e8`.
- Root filesystem: read-only.
- `/runner-state`: named volume mounted read-only.
- `/testgate-history`: `openwepp-testgate-history`, mounted read-write.
- History ownership/mode: `10001:10001`, `0700`.
- Two independent write/remove probes passed and left no residue.
- Dual independent activation review: contract PASS.

## Provider-Orphan Queue

Three manual-dispatch records from 2026-07-19 remain `queued` with zero jobs:

| Run | HEAD | Created UTC |
| --- | --- | --- |
| `29673299308` | `850f7f6f10044c078299718d8e9c46b77d278a86` | `2026-07-19T04:25:36Z` |
| `29672334757` | `d4420b2431558dab0619c08a7bdcd7ac497ae229` | `2026-07-19T03:48:53Z` |
| `29672149962` | `4ee31784044694f856a2eef855b9864beac9f3cf` | `2026-07-19T03:41:41Z` |

Ran:

- Exact run inspection: each is `workflow_dispatch`, `queued`, and has zero
  jobs.
- Artifact inspection: zero artifacts for every run.
- Log inspection: GitHub returns HTTP 404 for every run.
- Normal cancellation: GitHub HTTP 500 for every exact run.
- REST force cancellation: GitHub HTTP 500 for every exact run.
- Explicitly authorized exact-run deletion on 2026-07-23: GitHub HTTP 403 for
  every run; no record was deleted. Request IDs:
  `16B3:38D69C:F42C53:F90EEE:6A61972C`,
  `5665:AFBD7:101C83E:106ACFA:6A61972C`, and
  `FBC8:C4BD1:FCC0A8:101A648:6A61972C`.
- Newer push run `29978778150` executed, so the records are provider-orphaned
  rather than scheduler-active.

Static: GitHub documents workflow-run deletion after completion or after more
than two weeks. The newest orphan reaches the documented age boundary after
`2026-08-02T04:25:36Z`. Earlier exact deletion requires an explicitly accepted
bounded provider-recovery exception or provider confirmation. Do not delete or
rerun `29978778150`.

Static: the bounded early-deletion exception was authorized, attempted against
only the three exact IDs, and refused by GitHub. Provider support intervention
or the documented age boundary is now required; no further local or repository
mutation can terminalize these provider records safely.
