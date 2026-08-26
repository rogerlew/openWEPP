# Discrete terminal support-root reviews

Status: `DUAL GO-TO-HOLD / NO PRODUCTION AUTHORITY`.

These reviews cover the test-only
`CHILD1-DISCRETE-SUPPORT-ROOT-001` exploration and its defect-shaped HOLD.
They do not authorize successor contracts, Batch V2, production installation,
receiver/restart/runner work, or cutover.

## Science and numerics

Static: the reviewer inspected the corrected discrete endpoint implementation
and HOLD artifact against the canonical SnowEnergy tolerances. The endpoint
classifier retains raw unallocated-energy bytes and compares them to the
unchanged `1.0e-6 J m^-2` tolerance; it does not clamp the physical value.

Ran: the reviewer reran the real fixture with nextest run ID
`eba7ddb4-be86-45b8-991d-5e95f3cc26bd`; 1/1 selected test passed.

Disposition: `GO for defect-shaped HOLD`. Tick `615737728342` is
`PreTerminal` at `7.418275345e-7 J m^-2`. Both immutable brackets select tick
`615737728343` as `Invalid` at `1.014879672e-6 J m^-2`; tick
`615737728344` remains `Invalid` at `1.287902705e-6 J m^-2`. At the boundary,
melt and liquid are exactly `0.6 kg m^-2`, ending ice equals positive
deposition, no event occurred, closures pass, and replay is byte-identical.
The complete endpoint map reaches a bracket-independent material-invalid
boundary before any `TerminalAtEndpoint`, so continuing beyond it cannot prove
an admissible earliest root.

Nonblocking note: the typed operator reports `InvalidEndpoint` partly because
the diagnostic `Invalid` evaluation retains a candidate. The physical HOLD
rests on the reproducible material-invalid boundary, not that particular error
variant.

## Ownership, Batch shape, and custody

Static: the reviewer verified fail-closed invalid-lane handling, typed-root use
by the real callback, exact binding of the selected hydrology joint to one
matching typed carrier candidate, revalidation of all six non-snow owner byte
representations, retained event/closure evidence, immutable real beginnings,
and production exclusion of the test-only mode.

Ran: the reviewer ran `git diff --check`, library `cargo check`, and the focused
discrete-root matrix. All passed; the matrix passed 9/9. The expensive real
fixture was not independently rerun in this review and is covered by the
science/numerics rerun above.

Disposition: `GO-to-HOLD`. The real evidence is truthfully limited to one lane
and establishes a typed invalid boundary rather than a successful terminal
root. It makes no real multi-lane, Batch V2, successor-contract, production,
or cutover claim.

Nonblocking follow-ups if this model is revisited:

- centralize the test-only endpoint result/closure assembly with the
  production assembly so it cannot drift;
- centralize the duplicated duration-to-WB14-ceiling mapping used by provider
  construction and evidence sealing.

## Combined disposition

The two required reviews agree on `GO-to-HOLD`. This is not the dual GO needed
for production implementation. Production remains `BelowCarrierDomain`, and
no v22/v12/v140/v7 contract candidate is authorized.
