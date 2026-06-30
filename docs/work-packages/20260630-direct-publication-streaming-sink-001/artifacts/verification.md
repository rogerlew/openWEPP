# Verification

Evidence class: Ran + Static

## Static

- Production direct execution now calls
  `run_publication_stream_with_interleaved_day_inputs` and
  `DirectPublicationStreamingSink`.
- Whole-run retained WAT/PASS projection helpers are test-only or compatibility
  helpers; production direct output validation uses streamed row counts.
- `DirectRunPublicationFrame.rows` remains in the orchestrator capture API for
  tests/diagnostics, not as the production direct publication sink.

## Ran

- H2637 full-output run emitted `235961` rows and selected
  `direct-production-executor` with `compatibility_edge_invocations=0`.
- H2637 required-only run emitted `235961` rows and selected
  `direct-production-executor` with `compatibility_edge_invocations=0`.
- W9 longer-day observed fixture emitted `16437` rows and selected
  `direct-production-executor` with `compatibility_edge_invocations=0`.
- cli01 emitted `2` rows and selected `direct-production-executor` with
  `compatibility_edge_invocations=0`.
- H2637 full and cli01 data outputs are byte-identical to the prior retained-row
  baseline outputs.
