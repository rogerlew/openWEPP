# Implementation Log

Evidence class: Ran + Static

## Code Changes

- Added `DirectStreamingPublicationExecution` and
  `run_publication_stream_with_interleaved_day_inputs` in the direct runtime.
  The existing retained `DirectRunPublicationFrame` capture API now delegates to
  the streaming path and clones rows only for tests/diagnostic callers.
- Added `DirectPublicationStreamingSink` in the runner. Production direct
  execution now observes each `DirectPublicationDayRow`, updates a compact
  summary, writes requested WAT/PASS chunks, and drops the row.
- Replaced production output construction from whole-run `publication.rows` with
  summary-derived HBP/loss/manifest construction plus streamed WAT/PASS row
  counts.
- Added `HillslopeWatParquetRowGroupWriter` and
  `HillslopePassParquetRowGroupWriter`; the legacy slice-writing helpers now
  wrap these writers for compatibility.
- Preserved output-schema columns and direct-runtime physics. No snow, frost,
  hydrology, erosion, default-policy, or compatibility replay physics changed.

## Notes

- The streaming summary accumulates MOFE carry in millimeters and divides once,
  matching the retained-frame manifest arithmetic.
- Retained direct-publication helpers remain under test-only use for identity
  fixtures and source guards.
