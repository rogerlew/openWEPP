# PERFDEEP02 Disposition

Evidence class: Static + Ran.

Disposition: `NO-GO - performance blocked`.

## Summary

PERFDEEP02 implemented the Stage-1 dense-slot hydrology island mechanics over
`HillslopeDayFrame` and added the carried real-surface seed/flush diagnostic
hook. Functional tests pass, but the production opt-in H2637 endpoint missed the
required performance gate by more than 2x. The island is disabled by default and
requires `OPENWEPP_PERFDEEP02_FRAME_ISLAND=1`.

## Completion Criteria

- Real H2637 frame roundtrip: pre-final diagnostic passed, 235961 rows, zero
  mismatch rows.
- Focused frame roundtrip tests: passed.
- Dense-slot island scheduler test: passed.
- Full Rust gates: passed.
- H2637 production opt-in endpoint: failed. Attempts either completed at
  `2417.14 s` or were terminated after `23:36` and `25:27` elapsed because they
  had already exceeded 2x the PERFDEEP01 `669.97 s` reference endpoint.

## Decision

`NO-GO - performance blocked`.

PERFDEEP02 does not authorize default production dense-island activation. The
implemented path remains an opt-in base for follow-on work, and the default
runtime path is fail-closed to avoid a known endpoint regression.

Follow-on perf work must first remove the frame lifecycle blocker: carry dense
frame state persistently across per-day/per-OFE lane execution, or otherwise
avoid full frame re-seeding/copying before expanding the island or deleting
logical hot-path surfaces.
