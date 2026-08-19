# Implement Root-Zone Hydraulic Owner V1

Status: `queued / blocked on root-zone owner authority release`

Date: `2026-08-19`

Package ID: `20260819-root-zone-hydraulic-owner-implementation-001`

Plan class: `Critical default-off kernel owner implementation`

## Objective

Implement the released `OPENWEPP_ROOT_ZONE_HYDRAULIC_OWNER_V1` as an opaque,
orchestrator-owned configuration and interval receipt, then remove caller-owned
root hydraulic physics from the V10 scheduler seam.

## Blocker

The authority package is on HOLD because explicit root-tissue path values
cannot be supplied for required rooted scenarios and current subsurface
conductivity has saturated/base semantics. Production implementation is not
authorized until both are resolved and authority terminal verification passes.

## Intended future write set

Bounded root-zone owner module, orchestrator projection, non-wire restart
static context, contract-derived tests, Child-4 scheduler closure and package
artifacts. No selector/default/output/activation/cutover change.

## Delegation

Subagent authorization: after activation this package explicitly authorizes
read-only soil/plant hydraulic and Rust reviews plus two terminal verifiers;
the Child-4 package retains its separately authorized comparator and reviewers.
