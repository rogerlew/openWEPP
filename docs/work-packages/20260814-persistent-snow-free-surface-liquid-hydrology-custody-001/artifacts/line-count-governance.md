# Line-Count Governance

Evidence class: `Static + Ran`

Current affected-file counts after remediation:

| File | Lines | Disposition |
|---|---:|---|
| `direct_runtime/00_core_frames.rs` | 2,780 | WARN: optional-owner attachment delegates complete identity preflight and canonical error completion to the extracted attachment module. |
| `direct_runtime/surface_liquid_attachment.rs` | 685 | PASS; cohesive complete raw-attempt/snapshot framing, frame/configuration identity and numeric-domain preflight, and canonical attachment error-context completion. |
| `direct_runtime/surface_liquid_owner.rs` | 2,915 | WARN: cohesive owner schema, canonical persistence, checked proportional D/A/F, contextual condensation/other failure payloads, and candidate validation; resource and whole-set identity preflights are extracted, and a future campaign split will separate persistence/schema from arbitration validation. |
| `direct_runtime/surface_liquid_owner/resource_validation.rs` | 396 | PASS; cohesive public resource identity, arithmetic, cardinality and bound preflight. |
| `direct_runtime/surface_liquid_owner/identity_validation.rs` | 243 | PASS; cohesive whole-configuration and restart identity-set preflight. |
| `direct_runtime/surface_liquid_owner_tests.rs` | 2,232 | WARN: crate-local owner tests include symmetric joint-supply, canonical finalized-use ordering, direct taxonomy, contextual condensation, raw attempted-byte and cross-row precedence failures; future additions require cohesive extraction. |
| `vegetation_real_hydrology_shadow.rs` | 2,157 | WARN: only canonical snapshot and production-lane accessors changed; future campaign split will move snapshot/lane-map projection into a dedicated adapter module. |
| `direct_runtime/surface_liquid_ingress.rs` | 2,421 | WARN: canonical parcel identity/order, exact authoritative-parent mass/Q splitting, public precedence preflight and ingress construction remain cohesive; future campaign split will move dependency-neutral parcel identity into a dedicated boundary module. |
| `direct_runtime/surface_liquid_ingress_tests.rs` | 2,979 | WARN: focused independent partition, persistent-endpoint, taxonomy, context-matrix and bit-frozen mixed-kind routing tests; cadence and public context poisons are extracted, and no further cases may be added without another cohesive split. |
| `direct_runtime/surface_liquid_ingress_context_tests.rs` | 461 | PASS; extracted cadence, public exact-context/precedence, full-infiltration and exact mass/Q round-trip poison tests. |
| `direct_runtime/subsurface.rs` | 2,088 | WARN: this package changes only the shared WB14 transition call and attached custody fields; a future direct-runtime decomposition owns the broader production subsurface split. |
| `land_surface_energy_shadow/mod.rs` | 2,930 | WARN: cohesive default-off arbitration and category-wide cross-input public precedence; independent validation, checked receiver aggregation, error completion and framed hashing are extracted. No further additions are allowed without cohesive extraction. |
| `land_surface_energy_shadow/receiver_validation.rs` | 2,290 | WARN: self-contained expected topology, canonical identity/domain/cardinality/bound preflight, exact configured context, E010/E011 closure taxonomy and complete framed digest encoding; derived arithmetic preflight is extracted. |
| `land_surface_energy_shadow/receiver_preflight.rs` | 340 | PASS; one shared checked receipt fold supplies precedence preflight and final receiver operand freezing without duplicating arithmetic. |
| `tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs` | 2,903 | WARN: complete unified public-boundary acceptance and receiver taxonomy poison matrix; raw-attempt hash, precedence and sealed-receiver context cases are extracted, and no further cases may be added without another cohesive split. |
| `tests/integration/land_surface_energy_real_hydrology_shadow_contract/raw_hash_tests.rs` | 100 | PASS; extracted raw beginning/attempted-input hash poisons. |
| `tests/integration/land_surface_energy_real_hydrology_shadow_contract/precedence_tests.rs` | 441 | PASS; extracted mixed public E002 through E008, cross-input envelope, nonfinite lane and condensation-context precedence matrix. |
| `tests/integration/land_surface_energy_real_hydrology_shadow_contract/sealed_receiver_context_tests.rs` | 180 | PASS; extracted sealed LSE/thermal exact-owner, configured-context, topology and rollback-hash poisons. |
| `direct_runtime/surface_liquid_closure.rs` | 2,979 | WARN: independent chronological WB14 partition, raw source/OFE mass and exact mass/Q join, routed projection and persistent endpoint join; exact-Q and frozen raw-parent mass reconstruction are extracted, and no further additions are allowed without cohesive split. |
| `direct_runtime/surface_liquid_enthalpy_closure.rs` | 79 | PASS; closure-only bit-exact parent/child Q comparison independent of producer helpers. |
| `direct_runtime/surface_liquid_raw_parent_closure.rs` | 49 | PASS; closure-only raw OFE/source mass reconstruction from frozen parent parcels, independent of temporal child receipts. |
| `direct_runtime/surface_liquid_wb14.rs` | 303 | PASS; dedicated shared production transition. |
| `direct_runtime/runoff.rs` | 2,852 | WARN: this package adds only the shared WB14 transition call; a future direct-runtime decomposition package will separate WB14 legacy/profile mechanics from unrelated runoff kernels. |

Strict affected-crate Clippy passes with `-D warnings`. One cohesive 102-line
parcel-comparison function has a documented local `too_many_lines` allowance
because its contract-significant comparison order must remain contiguous. The
WARN files remain below the 3,000-line blocking threshold and require no
file-size exception under `crates/AGENTS.md`.
