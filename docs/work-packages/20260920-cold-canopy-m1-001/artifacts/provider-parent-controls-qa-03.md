# Provider-parent controls QA — 03

**Reviewer:** `/root/provider_qa` (same independent QA reviewer; focused review02-fix verification).  
**Evidence:** Static. No Rust compilation or test execution was performed.

## Findings

No blocking findings in the assigned correction scope.

## Fix verification

`expected_initial_phase_records` now derives the required 16-character lowercase binary64 strings from the frozen values' `to_bits()` at `m1_fixed_sequence_provider_controls.rs:484-497`. The expected full payload and both-cycle assertions use those records.

The adapter's root-output locators now use the RFC 6901 root pointer `""` for exposure, topology, and controller policy at lines 475, 479, and 480.

The new frozen dependency manifest `artifacts/m1-fixed-sequence-adapter-dependencies-v1.json` has SHA-256 `cff0ffc0730f08f4e0ca00f7ee2f39651981d94f136156da75922f3c62223d39`. Its eight listed path/hash pairs were independently rehashed against the detached source and linked authoritative artifacts. The oracle maps each closed path to direct retained bytes and verifies every listed digest before producing the payload. Its bytes are bound into each adapter locator. The planned source-adapter constructor receives those raw bytes, and the test poisons an actual manifest source hash through that constructor before asserting `VEG-E-144` and no caller mutation. This resolves the review02 synthetic-Cargo-label finding without inventing a second whole-crate inventory.

The retained endpoint fixture is still read through its real constructor and receipt forcing accessor; destination is selected from its serialized surface-record row with `ground_ingress_mode == "covered_canopy_release"` and the actual `key.ofe_id/key.tile_id` fields. This is a real-source boundary for the planned adapter, not a literal GSI or destination substitute.

Reviewed identities:

- controls SHA-256 `5065444f69bac4895f797f6093049fe1165dff93c9459170a834c4bc651c0a82`
- `mod.rs` SHA-256 `240a2238531c817634ce58e20a2b15efa4d4222416e2c4ca534e448b24091aad`
- canonical detached-source snapshot supplied for this cut `d22c7b3d70aa4b2c91b910e2a6b13b574c82c287b77410662a87d92ee99c9f23`

The private provider body remains intentionally absent, so its planned imports and `include_bytes!` are expected-red compilation failures until the body is authored. This is not an execution result.

## Compile-only manifest clearance

**PASS** for the selected nonphysical compile manifest `artifacts/provider-parent-controls-build-support-03.json`, SHA-256 `a15b47c39dba0dfce5587b321c8a2c1071e2000b8f919b71ced9f3640d64eb4b`: the detached source root, canonical snapshot, authority/input roots, all seven existing support links, material environment, new dependency-manifest pin, and the exact argv agree. The command is `nix develop /workdir/openWEPP --command env CARGO_TARGET_DIR=/tmp/openwepp-cold-canopy-m1-target CARGO_BUILD_JOBS=2 cargo check -p openwepp-hillslope-orchestrator --tests`, with a 180-second nonphysical bound and no binary requirement. The source snapshot is the recorder's custody mechanism for detached `crates/**`; the linked `src/lib.rs` and external support pins are separately present. The recorder must still make its fresh deadline/reserve and output-collision checks immediately before dispatch.

## Verdict

**QA PASS — release controls03 for the private provider body and the bounded expected-red compile.** This passes only the reviewed contract-derived control and compile-custody stage. Provider implementation, a passing compile/test run, real parent admission, and M1 acceptance remain unproved.
