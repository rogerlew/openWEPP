# ADR-0020: totalwatsed3 is a dedicated output-aggregation CLI

**Status:** Accepted (amends ADR-0006)
**Date:** 2026-06-14
**Deciders:** Roger Lew, Claude Code

## Context

[ADR-0006](0006-three-binaries-incl-replay.md) set three simulation-tier
binaries (`openwepp-cli-hill`, `openwepp-cli-watershed`, `openwepp-replay`).
[ADR-0019](0019-openwepp-owns-its-output-surface-wepppyo3-legacy-only.md) made
openWEPP own its output surface, including a **native totalwatsed3** producer.

WSHED01 first built totalwatsed3 **inside** `openwepp-cli-watershed`. That
entangled two unrelated concerns: totalwatsed3 is a **hillslope-only,
area-weighted water-balance aggregation** (consumes per-hillslope
`H.pass`/`H.wat`/`H.soil`/`H.element`; `Runoff` from PASS `runvol`; MOFE-aware
per-OFE collapse; **no channel routing**), whereas `openwepp-cli-watershed` is
**channel routing** (channel network, impoundments, `chanwb`/`chnwb`).
Bolting the former into the latter meant the hillslope-only totalwatsed3
closure had to clear the watershed CLI's **impoundment and channel** blockers —
concerns it does not use.

## Decision

totalwatsed3 is its **own binary, `openwepp-cli-totalwatsed3`**. It consumes
the per-hillslope interchange outputs (`H.pass`/`H.wat`/`H.soil`/`H.element`)
plus an area lookup, performs the hillslope-only area-weighted aggregation
(MOFE per-OFE collapse: `Runoff` from PASS `runvol`, latqcc outlet-OFE-only,
QOFE summed), and emits the openWEPP-native totalwatsed3 parquet + closure
surface. It is **not** part of `openwepp-cli-watershed`.

This refines the ADR-0006 binary taxonomy into two tiers:

| Tier | Binaries | Drives from |
|---|---|---|
| Simulation | `openwepp-cli-hill`, `openwepp-cli-watershed`, `openwepp-replay` | inputs / HBP shards (forward or replay) |
| Output aggregation | `openwepp-cli-totalwatsed3` (first of class) | completed per-hillslope interchange parquet (read-only) |

## Consequences

- **Clean separation of concerns:** channel routing (`openwepp-cli-watershed`)
  and hillslope WB aggregation (`openwepp-cli-totalwatsed3`) evolve
  independently with distinct argument and I/O surfaces.
- **The totalwatsed3 closure is decoupled from watershed channel routing** — it
  reads hillslope outputs directly and needs neither impoundments nor channel
  routing to close. (The WSHED01 W-B/W-C impoundment/channel blockers were
  never on the totalwatsed3 path.)
- The output-aggregation tier reads completed interchange parquet **read-only**;
  it is not a simulation binary and must be distinguishable as a derived/audit
  product (file-naming + parquet metadata), consistent with the ADR-0006
  posture for replay outputs.
- `openwepp-cli-watershed` shrinks (totalwatsed3 build moves out), which also
  reduces its line-count-governance pressure.
- Future native aggregations (per ADR-0019) join the output-aggregation tier
  rather than the simulation binaries.

## Relationship to other ADRs

- **Amends** [ADR-0006](0006-three-binaries-incl-replay.md) (binary structure).
- **Rationale basis:** [ADR-0019](0019-openwepp-owns-its-output-surface-wepppyo3-legacy-only.md)
  (openWEPP owns its output surface; wepppyo3 stays legacy-only).
- Work surface: WSHED01
  (`docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/`).
