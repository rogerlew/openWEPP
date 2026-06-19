# PERFDEEP06 Follow-On Package Sequence

Status: queued.
Evidence mode: not-run.

## Required Content

Define the next packages in the ADR-0025 sequence. The first follow-on must be
a direct-frame implementation package that ports a complete migrated hydrology
daily OFE chain over `&mut HillslopeDayFrame` with:

- no symbol maps or writeback payloads in the migrated success path;
- shadow bit identity for migrated phases;
- H2637 HBP/WAT/PASS identity;
- endpoint/RSS measurement;
- layout/type-size and allocation evidence;
- full Rust gates before implementation closure;
- no default activation unless the endpoint gate is met.

## Gate

This artifact is complete only when the next package objective, write set,
acceptance criteria, and stop criteria are concrete enough to scaffold without
another planning pass.
