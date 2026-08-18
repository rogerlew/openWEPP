# Implement Direct-Hydrology Persisted Restart

Status: `queued / blocked on restart authority release`

Date: `2026-08-17`

Package ID: `20260817-direct-hydrology-persisted-restart-implementation-001`

Plan class: `Critical persisted-state implementation`

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
