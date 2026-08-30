# Contract-test implementation evidence

Status: superseded floor vectors retained / amended exact-60 contract vectors
PASS on current head

Static: `tests/integration/snow_stage3_adaptive_compositional_contract.rs`
contains independent phase, bounded-vapor, exact-grid, successor-version, and
historical-preservation vectors. Execution is recorded in the pre-implementation
gate.

Static: every historical vector that encoded a 600-ms quantum, sub-600-ms rejection,
floor decision, attempt count, event tick, or floor-dependent performance is
invalid for the 2026-08-27 owner amendment. Replacement vectors must use the
exact 60-second (`60_000_000_000 ns`) floor and must be executed afresh.

Ran: the amended runtime guards and exact-60 production fixtures listed in
`implementation-test-evidence.md` passed, including exact-floor admission,
one-tick-below rejection, larger-support admission, direct/composed ownership,
restart-grid poisons, and day-support omission/reorder/substitution poisons.
These focused runtime results do not retroactively validate the superseded
pre-amendment contract-vector run.

Ran on current head:

```text
nix develop --command cargo test -p openwepp --test snow_stage3_adaptive_compositional_contract -- --nocapture
```

PASS, 4/4 in 0.01 s after updating the binding to the authorized
`SC-SURFACELIQUID-001` version 12 inverse-basis amendment. The phase projection,
bounded-vapor pairing, odd exact-grid split, and successor-contract/history
guards all passed. Three temporary dead-code warnings came from the concurrent
receiver-capacity wiring, so this run is not a warning-clean claim.
