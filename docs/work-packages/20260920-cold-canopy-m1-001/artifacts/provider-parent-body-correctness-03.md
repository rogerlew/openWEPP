# COLD-CANOPY-M1 provider/admission body — correctness fix verification 03

**Verdict: RELEASED for the bounded nonphysical provider controls.** This release covers the reviewed provider/admission controls only; it is not complete-parent, physical-support, or M1 acceptance.

**Evidence class: Static.** Same reviewer `/root/provider_correctness`; sole-fix verification of frozen 759-entry source aggregate `67b684300cb32e8a317559d6282bd2e6bd3f91139df667e12c96611b1936c971`. I inspected the supplied successful test-inclusive compile evidence. I did not run compilation, tests, or physics and made no source/configuration edit.

## Findings

No blocking correctness finding remains in the reviewed provider/admission control scope.

## Sole-fix verification

The body02 exact-owner oracle defect is resolved at `m1_fixed_sequence_provider_controls.rs:717-733`. Both retained `mass_kg_m2_tile_ground` and `enthalpy_j_m2_tile_ground` values are now decoded from the actual numeric JSON representation with `as_f64()`. These `f64` values retain the exact binary64 bits that the production payload encodes as lowercase hexadecimal and that `actual_m1_caller_beginning_for_provider_cycle` decodes back with `f64::from_bits`. The independent caller construction at `:734-822` therefore receives the exact source M/H values for both cycles, and the comparison at `:1433-1446` can reach all seven exact owner-byte assertions instead of panicking on `.as_str()`.

Independent static reconstruction confirmed the source values map to cycle-zero mass bits `3f926e978d4fdf3b` with zero enthalpy for both occupancies, and cycle-one pairs `3f989374bc6a7efa/c0c022c28f5c28f6` and `3f9eb851eb851eb8/c0b38d8000000000`. Those are the values carried through the production phase encoding and caller construction.

All production hashes from body02 are unchanged: named/invoked source accessors and selected GSI, shared canonical surface and typed thermal owner encoders, shared validated-reservoir caller initialization, cached immutable payload identity, and the ten-path manifest remain accepted without re-review. The supplied compile artifact reports success with 43 warnings and no test execution.

## Residual risk and missing validation

The released bounded provider controls still must run on this exact frozen source. Complete-parent stage/commit and physical progression remain separately pending and outside this disposition. Their absence does not narrow the provider controls, and successful provider controls will not establish either milestone.

## Reviewed identities

- source recorder: `provider-parent-body-review-03-source.json` SHA-256 `c6663c1fc40b1b4b05b6b3128f021d806b71a4b12d5439ba3cbb29aab1baf396`
- observer-relative patch: `provider-parent-body-review-03-from-observer-cut02.patch` SHA-256 `eefdda5b4b0a8da69f7706be28f9580d030eb7a50cc0dd6fb81ca8f0185b6f09`
- final compile stderr: `provider-parent-body-compile-05.stderr` SHA-256 `d70b7b1f9f8ae326546d3b0541f0589032ba0ae049b2b151d893722a4adc990d`
- corrected controls: `m1_fixed_sequence_provider_controls.rs` SHA-256 `d4dca435ed2b688928a70c51d088068bd6326c7e1ad36c760e6a7d650c7c3346`
- unchanged provider: `m1_fixed_sequence_provider.rs` SHA-256 `6671ab7418a0719c2d395a5dfbed02e6ef1b52e4545a7ccaae11d4b9655d582d`
- unchanged caller: `m1_receiver.rs` SHA-256 `0984384504186b11a92b43e6e6256dfd5187f4f0e13ae3694e85547428491b19`
- unchanged canonical owner bytes: `canonical_owner_bytes.rs` SHA-256 `ec236a76791db25c6c2b191a6e1bd3d33df99d6f5a132bf1cd08e534638aa676`
- unchanged typed thermal bridge: `v8_input_projection.rs` SHA-256 `ab0daaa67d8d8c188fc445cdef152639bbef0a91acb199aaea989e40f5aee852`
- unchanged V9 re-export: `v9_real_consumer_shadow.rs` SHA-256 `85d03a50f50bb2ccd6fa943caf1a560ad4df49a282cc4469b56de982c7033324`
- unchanged adapter manifest: SHA-256 `2a46507c5c232e08fd88365469e0d3bb84e21aeabad1ce76b8fc1ef23ad8fab3`
