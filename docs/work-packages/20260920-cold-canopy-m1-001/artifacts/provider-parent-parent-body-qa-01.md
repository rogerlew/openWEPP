# Provider-parent partial body QA 01

**Static review.** No Rust command, test, or physical command was run by this
reviewer.

## Identity and scope

Reviewed detached source receipt
`provider-parent-parent-body-review-01-source.json` (receipt SHA-256
`8492b08e280ead5edacd7eca6d0f562af3d082eb7c0c0c7f46bdab937c6b8531`),
whose 760-entry tree is
`067138e680d3b0953b1f9a181ba779fec18aeced3db0f54f30182d63cc170fdf`, and
its patch `023ed26e10944796ef19f7c51e2eee7e2eab391b6850da85377bd7a8578f36bd`.
The reviewed provider file is
`crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/m1_fixed_sequence_provider.rs`
(current SHA-256 `056db9943adbf60daaf34e49235976b269ecccbb08cdd27a9dd78f49739f3c41`).

The recovery receipt `provider-parent-parent-body-recovery-01.json` reproduces
the same tree and patch and explicitly limits itself to source-byte recovery;
it supplies no executable qualification. `provider-parent-authority-checks-01.json`
is likewise correctly scoped to canonical-document BEI/unit/A0 structure and
does not claim detached runtime admission, A1, or A3 evidence.

## Findings

### BLOCKER — native receiver context remains bootstrap state

[`m1_fixed_sequence_provider.rs:815`](../../../../crates/openwepp-experiments/cold-canopy-m1-20260920/crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/m1_fixed_sequence_provider.rs)
creates `endpoint_fixture()` immediately before native receiver advancement.
That is still bootstrap receiver context, rather than the actual caller's
retained native forcing/context, so runtime admission cannot yet be qualified.

The earlier ledger finding is retracted: `LedgerEntryV1::new` requires equal
debit and credit digests, and the established V11 custody pattern deliberately
uses the beginning owner digest twice while the accepted slab separately
authenticates ending owners. Likewise,
`provider_covered_input`/`provider_conditions`/`provider_topology_binding`
read the retained original input and prescribed geometry/scalars; their
expected-red module location alone is not a provenance defect.

### BLOCKER — parent execution remains intentionally incomplete

The known compile-13 expected-red state (56 absent APIs) is consistent with the
source: caller finalization, resource debit/receipt observation, raw outcome
publication, and replay/refusal paths are absent. This prevents a body/runtime
or physical-gate release. In addition,
[`m1_fixed_sequence_provider.rs:787`](../../../../crates/openwepp-experiments/cold-canopy-m1-20260920/crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/m1_fixed_sequence_provider.rs)
maps canonical solver failure into an undifferentiated `ProviderError`; the
eventual public parent error should retain the typed solver failure and context.

## Fix verification and bounded command disposition

The clone-then-install structure in `advance_fixed_sequence_support` leaves the
live caller unchanged until both native receiver staging and `accept_slab`
succeed. That is a sound rollback shape. The adapter dependency manifest is
**not** cited as stale: its ten
declared members were independently rehashed by the executor and match.

The current parent-control change is limited to the required `BTreeMap` import;
it does not invalidate the accepted test06 control assertions. Provider04
evidence remains source-specific and is not promoted to this source cut.

`provider-parent-parent-build-support-01.json` (SHA-256
`d15f968551d34afcf7ca0d9fb90e9159842d43786e027d7346f09156ed9bc322`) binds
the reviewed 760-entry source tree, recorder SHA
`6781d00a9e42d45418993c522b5a7796ce55e6261834964b7452149140faf042`, 619
recorded inputs, detached execution context, and the exact 180-second,
`physical=false` `nix develop … cargo nextest list -p
openwepp-hillslope-orchestrator --lib --message-format json` argv. **QA clears
that command only as the declared expected-red compile/list diagnostic.** It
does not run tests or physics, does not establish a usable binary, and must
preserve the expected compile failure as evidence of the incomplete APIs.

## Verdict

**HOLD — parent body/runtime, selected parent controls, and all physical/full
gates.** **PASS — the exact nonphysical 180-second expected-red build/list
diagnostic only.**
