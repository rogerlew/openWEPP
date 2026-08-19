# Admit Direct-Hydrology Persisted Restart Authority

Status: `COMPLETE / authority released; production restart implementation not started`

Date: `2026-08-17`

Package ID: `20260817-direct-hydrology-persisted-restart-authority-001`

Plan class: `Critical contract-first restart authority`

## Objective

Define versioned, deterministic, layout-independent
`DirectHydrologyRestartV1` and `DirectV10RealConsumerCheckpointV1` authority,
including complete field classification, identities, cursor, owner state,
canonical serialization, and fail-closed restoration.

The forcing adapter is held on the same cursor/checkpoint transaction, so this
authority executes concurrently with its remaining ownership closure rather
than waiting behind it. Canonical persisted owners are V10 vegetation,
LSE-V2, CP-GSI01 state/receipt, provider cursor, direct hydrology, surface
liquid, soil thermal, BGC, and scheduler position. Transient V9/LSE-V1
projections are reconstructed and identity-checked after restoration.

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

Disposition: all exit criteria passed on the frozen authority bytes. Three
independent authority reviews and two terminal verifiers passed; the authority
prompt is archived. Any production restart implementation or downstream
forcing/Child-4 campaign closure is separate future work.

## Delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to independent read-only state/restart and hydrology reviewers and two read-only
terminal verifiers, with package-local outputs.
