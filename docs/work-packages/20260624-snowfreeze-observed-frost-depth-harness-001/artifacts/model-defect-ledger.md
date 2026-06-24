# Model Defect Ledger

Evidence class: Static/Ran.

Status: queued.

Do not fill this with raw observation disagreements. A row may become
`OPENWEPP-DEFECTIVE` only when `INV-SNOWFREEZE-047` and ADR-0017 criteria are
met.

| Defect ID | Site | Dates/window | Method | Observed issue | Snow-control status | Censoring status | Independent authority | Verdict | Follow-up |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |

Allowed verdicts:

- `PASS`
- `HARNESS-SURFACE-MISMATCH`
- `OPENWEPP-DEFECTIVE`
- `UNRESOLVED`
- `SOURCE-BLOCKED`
