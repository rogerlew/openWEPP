# Implementation and Test Evidence

Status: `EXECUTED-CURRENT`

Evidence mode: `Static + Ran`

Static: the existing W11B real-CLI fixture was generalized without production
edits. It now accepts `ipeak`, `dtchr`, scalar-event descriptors, EVENT/NOEVENT
form, and an optional test-only release binary path. It reparses `chan.inp`
before launch, validates the requested timestep and canonical channel IDs, and
reparses both serialized HBP inputs. After launch it reads the EBE/CHANWB
Parquet publications. The water residual is a serialized-input routed ledger;
because production derives published storage from routed inflow and outflow,
exact closure is algebraic diagnostic evidence rather than an independent
conservation proof.

Ran:

- Initial compile found and fixed one harness lifetime annotation.
- Fail-fast runs localized invalid zero-EVENT use, material negative storage,
  event-scalar publication semantics, and the `chan.inp` default alias.
- Corrected final three-test file run ID
  `ace36dab-5980-499c-b510-de33836bed64`: `3/3 PASS` in 15.618 seconds.
- Final diagnostic selector run ID
  `f695f3db-0627-4c28-8d97-8e5c5d023158`: `1/1 PASS`, all 35 rows emitted.

The characterization test passes when the matrix executes and its hard
finite/zero/algebraic-closure checks hold. Structured findings are separately
classified against the package's physical sanity gates; a passing test process
does not relabel `SANITY-FAIL` findings as acceptable physics.

Review response removed a defect-locking legacy element-ID assertion, corrected
the channel-ID sidecar authority, and added serialized HBP reparsing. Ran on
the final response tree: focused clippy PASS, three-test run
`ace36dab-5980-499c-b510-de33836bed64`, and diagnostic run
`f695f3db-0627-4c28-8d97-8e5c5d023158`.
