# Heavy Attempt 10: Runtime Consumer Probe

Evidence class: Ran.

Attempt:
`/home/workdir/openWEPP-quality-history/20260724-order3-attempt10-runtime-probe-eiCzXS`.

Executed head:
`924f82d00d530e8c8e7545ce6fdd92b60a423675`.

Admission ID:
`f7a45d0193ca4b022538f047a4506c076ad3a4665df86ded49581d3a69108bf6`.

## Result

The fresh admission independently enumerated 2,279 `full`, 36
`science-manual`, and 2,315 canonical workspace tests. It primed the declared
`openwepp-assurance` runtime binary before sealing the instrumented executable
manifest.

One admitted-snapshot Nextest invocation selected the three assurance
publication consumers that created late Cargo artifacts in attempt 7:

- `reconstructed_production_snapshot_passes_and_forged_roots_fail`;
- `release_driver_persists_verified_v2_artifacts_and_discovery_sidecar`;
- `synthetic_approved_fixture_publishes_idempotently_and_release_rejects_it`.

All three passed in `47.951s`; 22 tests in the same binary were skipped by the
exact expression. Ten external LLVM profile files were produced.

The repository helper reconstructed 291 executable rows before and after the
consumer run. Both manifests exactly equal admission, both working-tree
identities equal admission, and the canonical before/after evidence files are
byte-identical with SHA-256
`2b3487ea39b65a6003ed88e4c2a8b10ce8a7786e69d06ef158d1c890454cf312`.
Source and execution snapshot remained clean at the executed head.

## Retained Selector Diagnostics

Attempts 8 and 9 are retained, non-retried selector failures. Attempt 8
incorrectly used binary-qualified identities as positional libtest names.
Attempt 9 used exact `test(=...)` expressions but omitted the collector's
workspace context and still selected zero tests. A read-only Nextest inventory
probe then proved the final binary-and-anchored-test expression selects exactly
the three intended consumers before attempt 10 used it.

These selector failures did not execute a test, did not change the 291-row
manifest, and did not change the admitted checkout identity.

## Disposition

The runtime Cargo-artifact correction has a real downstream-consumer proof.
The full merged-coverage transition is authorized on a fresh committed head.
