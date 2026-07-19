# Provider Queue Evidence

Ran: 2026-07-19 UTC.

## Intake

Runs `29673299308`, `29672334757`, and `29672149962` were the complete queued
Actions inventory. Each was a manual TESTGATE dispatch against a pre-pivot
commit, had zero jobs and zero artifacts, and requested exact retired
`omarchy` labels. No current forest1-labeled run was queued or active; provider
ID 23 was online and idle.

The first two records used concurrency group
`openwepp-omarchy-testgate-v2`; the third used
`openwepp-omarchy-testgate`. GitHub's current concurrency-groups API reported
zero active groups, proving the records were orphaned from live concurrency
leases rather than cumulative executable work.

## Provider Cleanup Attempts

- Normal cancel returned HTTP 500 for all three IDs.
- Force cancel returned HTTP 500.
- DELETE was prospectively authorized after proving zero jobs/artifacts, then
  returned HTTP 403 because the records were still queued.

## Bounded Drain

The exact pinned runner image
`sha256:034ce655da139123cd775317d590d04dec6377788e4d124dc0e674f8d021e7e8`
was checked locally before a no-network derivative added only the immutable
pre-job rejection hook. Derived image ID:
`sha256:32d89708fd30c14ac76e3c86771d4b84902ee98aa22fb624041660b8a9f5c41d`.
The hook probe exited 1 as required.

Temporary provider ID 24, `openwepp-queue-drain-01`, came online with exact
labels `self-hosted`, `Linux`, `X64`, `openwepp`, `omarchy`, and `trusted`.
Container `openwepp-queue-drain` was limited to one CPU, 1 GiB memory, 512
PIDs, read-only root/state, all capabilities dropped, `no-new-privileges`, no
binds, and bounded tmpfs. Its only purpose is to make GitHub assign the three
orphaned records; the official pre-job hook rejects candidate execution before
steps.

The runner listened from `2026-07-19T17:14:26Z` through the hard deadline at
`17:19:22Z`. It remained idle and all three records remained queued with zero
jobs and unchanged timestamps. At `17:19:47Z`, provider registration 24,
container `openwepp-queue-drain`, volume `openwepp-queue-drain-state`, and the
rebuildable derived image were removed. Post-removal inspection found no drain
registration, container, volume, or derived image.

Disposition: `HOLD-PROVIDER-ORPHAN-QUEUE`. The three records cannot match the
active forest1 labels and have no live concurrency-group lease, so they cannot
execute or consume forest1. GitHub provider repair/support is required to clear
their displayed queued state; this package creates no timer or monitoring
handoff.
