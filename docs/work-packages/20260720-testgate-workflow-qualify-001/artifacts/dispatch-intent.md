# Forest1 Dispatch Intent

Base scaffold commit: `1b6922a21cef008ad2c60ba0bf7b6024900b2a94`.

Intent package:
`docs/work-packages/20260720-testgate-workflow-qualify-001/package.md`.

After this committed documentation increment reaches `main`, perform one
ordinary `workflow_dispatch` TESTGATE run using this base and package. The
controller must first confirm that no TESTGATE run is queued or active. Record
the resulting run ID and evidence without a duplicate dispatch.
