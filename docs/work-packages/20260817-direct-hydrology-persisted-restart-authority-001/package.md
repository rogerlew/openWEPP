# Admit Direct-Hydrology Persisted Restart Authority

Status: `queued / follows forcing adapter`

Date: `2026-08-17`

Package ID: `20260817-direct-hydrology-persisted-restart-authority-001`

Plan class: `Critical contract-first restart authority`

## Objective

Define versioned, deterministic, layout-independent
`DirectHydrologyRestartV1` and `DirectV10RealConsumerCheckpointV1` authority,
including complete field classification, identities, cursor, owner state,
canonical serialization, and fail-closed restoration.

## Protected boundary and write set

Do not derive persistence from Rust memory layout and do not simply derive
`Serialize` on `DirectRunFrame`. This package owns restart contracts/schemas,
field ledgers, reconstruction obligations, independent vectors/tests, and its
evidence only; production Rust waits for authority release.

## Exit criteria

Every direct-frame field is classified; every continuation-affecting owner,
carry, identity, topology, configuration, interval/day cursor, and forcing
provider receipt is bound; deterministic binary64-safe serialization and typed
poisons are specified; independent state/restart and hydrology reviews plus
dual verification pass.

## Delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to independent read-only state/restart and hydrology reviewers and two read-only
terminal verifiers, with package-local outputs.
