# COLD-CANOPY-M1 provider controls — correctness fix verification 03

**Verdict: RELEASED for provider-body authoring; executable-control release remains HOLD pending one mechanical compile fix.**

**Identity and scope.** Same independent reviewer `/root/provider_correctness`; static, focused verification of controls SHA-256 `5065444f69bac4895f797f6093049fe1165dff93c9459170a834c4bc651c0a82`, unchanged `land_surface_energy_shadow/mod.rs` SHA-256 `240a2238531c817634ce58e20a2b15efa4d4222416e2c4ca534e448b24091aad`, dependency-manifest SHA-256 `cff0ffc0730f08f4e0ca00f7ee2f39651981d94f136156da75922f3c62223d39`, at canonical recorder snapshot `d22c7b3d70aa4b2c91b910e2a6b13b574c82c287b77410662a87d92ee99c9f23` (758 entries). This review rechecked only the three High findings in review02 and their changed control cuts. The planned provider body remains intentionally absent. I did not run compilation, tests, or physical commands; I inspected the supplied source-bound compile evidence in `provider-parent-controls-compile-03.stderr`, and independently recomputed the eight declared dependency hashes from retained bytes. No source or configuration file was edited.

## Findings

1. **Medium — the both-cycle phase assertion does not compile.** `m1_fixed_sequence_provider_controls.rs:766-769` compares a `serde_json::Value` on the left with `Vec<Value>` on the right, producing E0277 independently of the intentionally missing provider API. Wrap `expected_initial_phase_records(cycle)` in `Value::Array(...)` (or compare matching slices) before treating this suite as executable evidence. This is a mechanical control defect and does not weaken the independently correct phase-bit oracle or block provider-body authoring.

2. **Low — the canonical-string match contains unreachable alternatives.** At `:76`, `'/'..='~'` already covers the later printable-ASCII ranges, so rustc reports the additional arms unreachable. Collapse the printable path to a single non-overlapping range after the explicit quote and backslash arms. The warning does not change the current encoder's behavior or the body-authoring release.

## Fix verification

1. **Initial-phase binary64 representation — resolved.** `m1_fixed_sequence_provider_controls.rs:484-497` independently converts both source numeric fields with `f64::to_bits()` and formats exactly 16 lowercase hexadecimal digits. The expected full payload uses these rows at `:499-514`, and the both-cycle/all-record control compares them again at `:754-769`. The calendar, run, and parent-forcing receipt oracles therefore consume the required closed-schema phase bytes rather than decimal JSON values.

2. **RFC 6901 document-root locators — resolved.** The complete exposure, topology, and controller output joins use the empty string at `:475`, `:479`, and `:480`. The GSI and destination joins retain their valid member pointers `/gsi` and `/destination` at `:474` and `:476`.

3. **Real dependency authentication — resolved.** `ADAPTER_MANIFEST_PATH` and `ADAPTER_MANIFEST` at `:47-51` bind the dedicated frozen JSON manifest. `retained_dependency_bytes` and `independently_validate_adapter_manifest` at `:377-419` read and hash-check all eight declared retained dependencies; independent review recomputation matched every declared SHA-256. Each extraction-adapter locator binds the actual manifest-byte hash at `:422-431`. The source adapter receives those raw bytes at `:622-627`, while `:962-973` corrupts a real manifest entry, constructs the adapter through the actual planned API, and requires provider admission refusal. The manifest covers the selected endpoint/source projections, destination and vegetation support, controller accessor, and both frozen input artifacts. Together with the separately bound provider-source hash and the canonical whole-crates recorder snapshot, this supplies the required direct and transitive custody without requiring a redundant second source inventory.

## Residual risk and missing validation

This is a static expected-red control review, not implementation acceptance. The body review must verify that `FixedSequenceSourceAdapter` validates the supplied frozen manifest before parsing or minting output, that its accessors execute the retained constructors/accessors named in the joins, and that the real caller refusal snapshot covers all seven owner bytes plus clock, staging, and publication state. The starting parent/snow integration obligations recorded in review01 also remain for implementation review. The compile evidence has the expected missing-provider errors plus the independent E0277 above; no test executed. Execution evidence remains deferred until the body and mechanical control fix exist.

No science-contract blocker remains to authoring the private provider body against this frozen control cut. Executable-control release requires the E0277 correction and subsequent compilation.
