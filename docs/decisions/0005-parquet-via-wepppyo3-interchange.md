# ADR-0005: Parquet schemas inherit from wepppy / wepppyo3 interchange

**Status:** Accepted
**Date:** 2026-05-11
**Deciders:** Roger Lew, Claude Code

## Context
openWEPP emits Parquet outputs. Two schema postures were considered:

- openWEPP defines its own schemas; wepppy adapts.
- openWEPP inherits the existing wepppy / wepppyo3 interchange schemas; consumers already exist.

The wepppy `wepppyo3` interchange schema already covers hillslope-trajectory parquet emission and is consumed by wepppy's query_engine.

## Decision
openWEPP emits parquet using the existing wepppy / wepppyo3 interchange schemas. The wepppy-side interchange code is adapted to concatenate openWEPP-emitted per-hillslope parquet files in place of regenerating them from legacy WEPP output.

## Consequences
- No new schema authoring on openWEPP's side initially; consumer-side compatibility is preserved by construction.
- openWEPP and wepppyo3 schema versions are co-managed; schema evolution requires coordinated changes.
- openWEPP does not need to support legacy ASCII output formats; legacy compatibility remains wepppy's concern via its existing parsers.
- Per-hillslope parquet from openWEPP is concatenated into watershed-level outputs by wepppy, not by openWEPP itself.
