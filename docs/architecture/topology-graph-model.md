# Topology Graph Model

Status: Draft (ARCH04)
Evidence: Static
Ran evidence: none

## Purpose

Define the typed topology graph substrate used by openWEPP before any timestep
execution.

Implementation path:
`/home/workdir/openWEPP/crates/openwepp-topology/src/lib.rs`.

## Ownership Boundary

- Topology graph ownership: `simulation::topology`
- Upstream dependencies: typed parser outputs (`.str`, `.chn`, `.imp`) and
  topology context (`nhill`)
- Downstream consumers: hillslope/watershed schedulers and orchestrators

This graph is pre-execution authority. Kernel execution must not start before
validation passes.

## Typed Model

### Node kinds
- `hillslope`
- `channel`
- `impoundment`

### Downstream nodes
Graph rows are represented as typed downstream nodes (`channel` and
`impoundment`) with explicit contributor triplets:
- hillslope contributors: `left`, `right`, `top`
- channel contributors: `left`, `right`, `top`
- impoundment contributors: `left`, `right`, `top`

### Directed edges
Each non-zero contributor becomes a typed directed edge:
- `from`: contributor node key
- `to`: downstream node key
- `slot`: `left | right | top`

## Closure Semantics

Topology closure is deterministic and validated against declared counts and
reference domains:
- declared channel count == observed channel nodes
- declared impoundment count == observed impoundment nodes
- each downstream node has at least one non-zero contributor
- contributor references are in-domain and resolvable
- channel/impoundment subgraph is acyclic

## ARCH04 Fixture Contract

Integration fixtures (`tests/fixtures/topology/*.topo`) use a minimal,
explicit format:
- headers:
  - `HILLSLOPES <count>`
  - `CHANNELS <count>`
  - `IMPOUNDMENTS <count>`
- node rows:
  - `NODE <CHANNEL|IMPOUNDMENT> <id> H <l> <r> <t> C <l> <r> <t> I <l> <r> <t>`

## Execution Gate Placement

Topology validation is a pre-execution hard gate. On failure, gate status is
`Failure` with `boundary_class=TOPOLOGY_INVALID`, and violations are surfaced as
explicit typed diagnostics.
