# Time Authority Intake

Status: complete

Evidence mode: Static + Ran

Ran: launch HEAD `f06be2a5c` on `main`, clean and two local scaffold commits
ahead of `origin/main`; `git diff --check` passed. Hardening anchor is
`46b0c679a`, itself a child of published scaffold `f48100538`. Package no-push
boundary remains binding.

Static: instruction discovery returned root plus science-contract, crate, test,
and work-package policies for the intended paths. Core reading was remeasured at
`501127` bytes (`WARN`).

## Current owner inventory

- `openwepp-kernel-contract::TransactionId(u128)` is the shared persistent
  parent transaction identity. Existing vegetation/LSE candidates finalize and
  increment at parent granularity; slabs must not invoke those finalizers.
- V10 `VegetationConfiguration.dt_s: f64` is configuration/state/receipt
  identity and is exact-bit joined throughout the real orchestrator. It is a
  protected boundary.
- LSE forcing/runtime identity also carries `f64 interval_s`; a future adopter
  must receive one centrally derived bit-identical slab duration.
- Half-hour forcing receipts use string transaction identity plus integer-second
  day/interval support. They remain immutable parent receipts projected into
  parent-relative nanosecond support; they are not `TransactionId` aliases.
- Current DirectV10 restart is interval-granular and byte-frozen. Its V1 DTOs
  use `deny_unknown_fields` plus canonical byte reserialization; fields or enum
  variants cannot be added safely.

## Selected boundary

Create leaf crate `openwepp-coupled-time`, depending only on
`openwepp-kernel-contract`, `serde`, `sha2`, and `thiserror`. Vegetation, LSE,
orchestrator, and restart may depend downward on it; the time crate must never
depend upward. The hillslope orchestrator receives only a bounded reference
consumer module. Coupled-time restart is owned by the leaf crate as an additive
V1 wire; `openwepp-persisted-restart-v1` and all DirectV10 V1 bytes remain
untouched.

## Frozen implementation intent

Implement validated u128 nanosecond identities, centralized duration-bit
derivation, fixed complete owner sets, segment participant sets, accepted slabs,
zero-duration event transitions, deterministic constraint arbitration,
diagnostic attempts, adopter policy identity, restart snapshots, atomic parent
commit, and buffered reductions. Use deterministic string-encoded u128 in JSON
artifacts. Reference policy is simple halving and has no Richards authority.

No current source evidence changes the central Richards disposition: later
`RichardsCoupledV1` must import this leaf authority and cannot combine with
legacy fixed scheduling or mutate legacy Lane D/R4L state outside the coupled
transaction.
