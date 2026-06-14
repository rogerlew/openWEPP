# ADR-0006: Three production binaries including replay

**Status:** Accepted (amended by ADR-0020)
**Date:** 2026-05-11
**Deciders:** Roger Lew, Claude Code

## Context
The minimal CLI surface for openWEPP is two binaries: one for single-hillslope simulation, one for watershed orchestration. The wepp-palimpsest fuzzing / ablation program has surfaced repeated needs for a tool that drives from a captured HBP shard rather than from inputs: trajectory-granularity parity diff, kernel-isolation re-execution, ablation-window re-run, golden-vector regeneration, single-hillslope debugging.

## Decision
Ship three binaries:

| Binary | Drives from | Purpose |
|---|---|---|
| `openwepp-cli-hill` | WEPP-format inputs + `.run` | Single hillslope simulation; forward in time |
| `openwepp-cli-watershed` | watershed structure + HBP set | Watershed routing; consumes completed hillslope shards |
| `openwepp-replay` | HBP shard + replay spec | Trajectory diff, kernel isolation, ablation window re-execution |

All three share the same kernel crates. `openwepp-replay` is a thin CLI over the same library; it is not a separate model.

## Consequences
- Argument surfaces stay clean (replay's window / kernel selection and perturbation overrides do not pollute the main CLI).
- I/O contracts are distinct (HBP-shaped input vs run-config-shaped input) and stay distinct.
- Replay outputs must be distinguishable from production simulation outputs at the file-naming and parquet-metadata level so they are not confused with model results.
- Replay is the comparator-harness driver in addition to its debugging role.
