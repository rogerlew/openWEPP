# Implement Direct-Hydrology Persisted Restart

Status: `queued / restart authority released / execution not started`

Date: `2026-08-17`

Package ID: `20260817-direct-hydrology-persisted-restart-implementation-001`

Plan class: `Critical persisted-state implementation`

## Prerequisite disposition

The blocking authority prerequisite is satisfied by
`OPENWEPP_DIRECT_HYDROLOGY_RESTART_V1` and
`OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1`, released in commit
`f95395597fe434dbc853c1b947b16fd434f013c1`. This package is eligible for a
separate implementation kickoff; no production restart implementation has
started.

## Objective

Implement canonical `DirectHydrologyRestartV1` frame projection/restoration and
`DirectV10RealConsumerCheckpointV1` byte serialization/restoration, then prove
fresh-object continuation and exact failure rollback.

## Intended write set and exclusions

Direct-runtime persistent DTO/projection code, V10/LSE-V2 checkpoint integration,
tests, schemas, and package evidence. No raw-layout persistence, Debug strings,
activation, selector/default/output change, deployment, calibration, PR,
remote branch, or push.

## Exit criteria

Repeated bytes are deterministic; restore works after dropping the original;
half-day/day/multi-day continuation and production outputs match; all required
identity, order, corruption, cursor, surface-liquid, and V9/V10 poisons reject;
restore failure rolls back exactly; reviews, full package gates, and dual
verification pass.

## Delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to read-only restart/hydrology and Rust reviewers and two read-only terminal
verifiers, with compact package-local outputs.
