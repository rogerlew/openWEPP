# Production noninterference

Status: `STRUCTURAL PASS / FULL CONSUMER PROOF BLOCKED`.

`Static:` The default remains off. The ordinary production scheduler's new
attachment field is `None` unless explicitly configured; the old persisted
restart field and compatibility types remain available, and the legacy
caller-built handoff bridge is test-only. No selector/default, CoE authority,
production output, or publication-batch API was changed.

`Ran:` source-boundary integration tests passed (`3/3`). This proves the
negative wiring boundary only. It does not prove real-run noninterference or
endpoint closure because the runner has no new attachment construction path.
