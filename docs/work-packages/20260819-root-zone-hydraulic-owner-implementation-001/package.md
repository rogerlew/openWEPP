# Implement Root-Zone Hydraulic Owner V1

Status: `executing / released authority implementation active`

Date: `2026-08-19`

Package ID: `20260819-root-zone-hydraulic-owner-implementation-001`

Plan class: `Critical default-off kernel owner implementation`

## Objective

Implement the released `OPENWEPP_ROOT_ZONE_HYDRAULIC_OWNER_V1` as an opaque,
orchestrator-owned configuration and interval receipt, then remove caller-owned
root hydraulic physics from the V10 scheduler seam.

## Authority release

Authority package `20260819-root-zone-hydraulic-owner-authority-001` released
`OPENWEPP_ROOT_ZONE_HYDRAULIC_OWNER_V1` at local commit `de2b078fa`. The
required non-defaulted stratum path, live-saturation Brooks--Corey operator,
typed guards, exact vectors, three reviews, and two terminal verifiers passed.
Production implementation is now authorized within this package's bounded
write set.

## Intended future write set

Bounded root-zone owner module, orchestrator projection, non-wire restart
static context, contract-derived tests, Child-4 scheduler closure and package
artifacts. No selector/default/output/activation/cutover change.

## Delegation

Subagent authorization: after activation this package explicitly authorizes
read-only soil/plant hydraulic and Rust reviews plus two terminal verifiers;
the Child-4 package retains its separately authorized comparator and reviewers.
