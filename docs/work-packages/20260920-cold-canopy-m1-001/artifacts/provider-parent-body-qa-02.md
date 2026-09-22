# Provider-parent body QA — 02

**Reviewer:** `/root/provider_qa` (secondary QA)  
**Evidence:** Static source/manifest inspection plus inspection of supplied compiler receipt `provider-parent-body-compile-04.stderr`. This reviewer did not run Rust, tests, or physical work.

## Findings

No blocking QA finding in the changed private provider body or the proposed nonphysical build/list gate.

The previously reported source-adapter concern is fixed in the reviewed cut. `FixedSequenceSourceAdapter` now invokes named accessors for endpoint GSI, original exposure/topology, and controller policy before emitting their source-join rows. The GSI row is derived from `endpoint_fixture().receipt.forcing().gsi`, rather than being built from a control literal. The payload joins retain their full adapter-output hash and pointer bindings.

The prior custom surface/thermal-byte concern is also fixed at the reviewed call paths. The real caller delegates complete beginning-owner construction through the shared canonical owner route; soil thermal reaches `v8_input_projection`'s canonical active-owner encoder, and the new independent control compares all seven actual owner byte values across both cycles with independently constructed canonical projections. This is a maintainable improvement over a local duplicate serializer.

## Source and build binding

- Detached source receipt: `provider-parent-body-review-02-source.json`, aggregate SHA-256 `01c97d8d06a422163c87052342521b295c7e0c0a7384a73eccff631173696519` (759 entries).
- Reviewed changed provider: `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/m1_fixed_sequence_provider.rs`, SHA-256 `6671ab7418a0719c2d395a5dfbed02e6ef1b52e4545a7ccaae11d4b9655d582d`.
- Reviewed controls: `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/m1_fixed_sequence_provider_controls.rs`, SHA-256 `f056ba1d812a2bd5678d8b828b5093f3e4b190f786358f216ee5800eb1051141`.
- Shared canonical owner route: `crates/openwepp-hillslope-orchestrator/src/canonical_owner_bytes.rs`, SHA-256 `ec236a76791db25c6c2b191a6e1bd3d33df99d6f5a132bf1cd08e534638aa676`; thermal encoder path `land_surface_energy_shadow/v8_input_projection.rs`, SHA-256 `ab0daaa67d8d8c188fc445cdef152639bbef0a91acb199aaea989e40f5aee852`.
- Adapter dependency manifest: `m1-fixed-sequence-adapter-dependencies-v1.json`, SHA-256 `2a46507c5c232e08fd88365469e0d3bb84e21aeabad1ce76b8fc1ef23ad8fab3`. It declares ten distinct retained inputs and provider admission checks every retained path/hash before payload construction.
- Proposed build/list manifest: `provider-parent-body-build-support-02.json`, SHA-256 `ce3c5b08c88ba17c7e65da693bafe206e3f2e07d7fee9b8d994b80489797b4e1`, binds the same detached aggregate, relevant external manifest/source pins, existing support links, Nix environment, `CARGO_TARGET_DIR=/tmp/openwepp-cold-canopy-m1-target`, and `CARGO_BUILD_JOBS=2`.

The proposed command is a 180-second, nonphysical list-only gate:

```text
nix develop /workdir/openWEPP --command env CARGO_TARGET_DIR=/tmp/openwepp-cold-canopy-m1-target CARGO_BUILD_JOBS=2 cargo nextest list -p openwepp-hillslope-orchestrator --lib --message-format json
```

Its declared scope is compilation and listing of the actual orchestrator lib-test binary only. Static inspection of the changed provider and controls finds no support advancement, constitutive solve, or physical execution call. The supplied final compiler receipt reports a successful `lib test` compile; it also reports 43 pre-existing crate warnings outside this changed module, so it is compile evidence and not lint-clean evidence.

## Non-blocking follow-up

This list-only build gate does not establish `cargo fmt`, Clippy with warnings denied, a passing selected control run, broader tests, deny, physical execution, or final parent staging/commit/resource acceptance. Those remain missing evidence and need their own scoped receipts.

## QA disposition

**PASS — release the exact source-bound nonphysical build/list gate.** This clearance is limited to the reviewed changed source and manifest; it is not retroactive approval of the previously held control run or a final package acceptance.
