# Line-Count Governance

Evidence class: `Static + Ran`

Current affected-file counts after remediation:

| File | Lines | Disposition |
|---|---:|---|
| `direct_runtime/00_core_frames.rs` | 2,823 | WARN: only the optional-owner attachment method remains here; canonical attachment error completion is extracted. |
| `direct_runtime/surface_liquid_attachment.rs` | 573 | PASS; cohesive complete raw-attempt/snapshot framing and canonical attachment error-context completion. |
| `direct_runtime/surface_liquid_owner.rs` | 2,862 | WARN: cohesive owner schema, canonical persistence, checked proportional D/A/F, contextual failure payloads, and candidate validation; resource preflight is extracted, and a future campaign split will separate persistence/schema from arbitration validation. |
| `direct_runtime/surface_liquid_owner/resource_validation.rs` | 396 | PASS; cohesive public resource identity, arithmetic, cardinality and bound preflight. |
| `direct_runtime/surface_liquid_owner_tests.rs` | 1,889 | PASS; crate-local owner tests include symmetric joint-supply, canonical finalized-use ordering, direct taxonomy and raw attempted-byte failures. |
| `vegetation_real_hydrology_shadow.rs` | 2,157 | WARN: only canonical snapshot and production-lane accessors changed; future campaign split will move snapshot/lane-map projection into a dedicated adapter module. |
| `direct_runtime/surface_liquid_ingress.rs` | 2,279 | WARN: canonical parcel identity/order, public precedence preflight and ingress construction remain cohesive; future campaign split will move dependency-neutral parcel identity into a dedicated boundary module. |
| `direct_runtime/surface_liquid_ingress_tests.rs` | 2,979 | WARN: focused independent partition, persistent-endpoint, taxonomy, context-matrix and bit-frozen mixed-kind routing tests; cadence and public context poisons are extracted, and no further cases may be added without another cohesive split. |
| `direct_runtime/surface_liquid_ingress_context_tests.rs` | 246 | PASS; extracted cadence, public exact-context/precedence and full-infiltration round-trip poison tests. |
| `direct_runtime/subsurface.rs` | 2,088 | WARN: this package changes only the shared WB14 transition call and attached custody fields; a future direct-runtime decomposition owns the broader production subsurface split. |
| `land_surface_energy_shadow/mod.rs` | 2,880 | WARN: cohesive default-off arbitration and canonical public precedence; independent validation, preflight, error completion and framed hashing are extracted. |
| `land_surface_energy_shadow/receiver_validation.rs` | 2,083 | WARN: self-contained expected topology, canonical identity/domain/cardinality/bound preflight, exact later-row context, E010/E011 closure taxonomy and complete framed digest encoding; derived arithmetic preflight is extracted. |
| `land_surface_energy_shadow/receiver_preflight.rs` | 160 | PASS; independent derived receiver arithmetic and receipt-conversion preflight. |
| `tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs` | 2,958 | WARN: complete unified public-boundary acceptance and receiver taxonomy poison matrix; raw-attempt hash and precedence cases are extracted, and no further cases may be added without another cohesive split. |
| `tests/integration/land_surface_energy_real_hydrology_shadow_contract/raw_hash_tests.rs` | 100 | PASS; extracted raw beginning/attempted-input hash poisons. |
| `tests/integration/land_surface_energy_real_hydrology_shadow_contract/precedence_tests.rs` | 183 | PASS; extracted mixed public E002 through E008 precedence matrix. |
| `direct_runtime/surface_liquid_closure.rs` | 2,892 | WARN: independent chronological WB14 partition, raw source/OFE mass join, routed projection and persistent endpoint join; future campaign split will separate frozen operand projection from comparison/diagnostics. |
| `direct_runtime/surface_liquid_wb14.rs` | 303 | PASS; dedicated shared production transition. |
| `direct_runtime/runoff.rs` | 2,852 | WARN: this package adds only the shared WB14 transition call; a future direct-runtime decomposition package will separate WB14 legacy/profile mechanics from unrelated runoff kernels. |

Strict affected-crate Clippy passes with `-D warnings`; no line-count lint is
suppressed. The WARN files remain coherent existing integration surfaces and
require no exception under `crates/AGENTS.md`.
