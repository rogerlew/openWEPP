# Line-Count Governance

Evidence class: `Static + Ran`

Current affected-file counts after remediation:

| File | Lines | Disposition |
|---|---:|---|
| `direct_runtime/00_core_frames.rs` | 2,824 | WARN: only the optional-owner attachment method remains here; canonical attachment error completion is extracted. |
| `direct_runtime/surface_liquid_attachment.rs` | 104 | PASS; cohesive canonical attachment error-context completion. |
| `direct_runtime/surface_liquid_owner.rs` | 2,612 | WARN: cohesive owner schema, canonical persistence, checked proportional D/A/F, contextual failure payloads, and candidate validation; crate-local tests are separate, and a future campaign split will separate persistence/schema from arbitration validation. |
| `direct_runtime/surface_liquid_owner_tests.rs` | 1,141 | PASS; crate-local owner and checked-arithmetic tests include exact three-way proportional allocation, tiny-positive and contextual serialized-byte failures. |
| `vegetation_real_hydrology_shadow.rs` | 2,157 | WARN: only canonical snapshot and production-lane accessors changed; future campaign split will move snapshot/lane-map projection into a dedicated adapter module. |
| `direct_runtime/surface_liquid_ingress.rs` | 2,014 | WARN: canonical parcel identity/order plus ingress construction remain cohesive; future campaign split will move dependency-neutral parcel identity into a dedicated boundary module. |
| `direct_runtime/surface_liquid_ingress_tests.rs` | 2,998 | WARN: focused independent partition, persistent-endpoint, taxonomy, context-matrix and bit-frozen mixed-kind routing tests; the resumed Child-3 campaign must split fixture builders and exact-vector tests before adding further cases. |
| `direct_runtime/subsurface.rs` | 2,088 | WARN: this package changes only the shared WB14 transition call and attached custody fields; a future direct-runtime decomposition owns the broader production subsurface split. |
| `land_surface_energy_shadow/mod.rs` | 2,815 | WARN: cohesive default-off arbitration and contextual receiver construction; independent validation, preflight, error completion and framed hashing are extracted. |
| `land_surface_energy_shadow/receiver_validation.rs` | 1,557 | PASS; self-contained expected topology, global arithmetic preflight, canonical error completion, independent closure and complete framed digest encoding. |
| `tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs` | 2,446 | WARN: complete unified public-boundary acceptance and poison matrix; the resumed Child-3 campaign must split fixtures from transaction tests before adding new scenarios. |
| `direct_runtime/surface_liquid_closure.rs` | 2,678 | WARN: independent chronological WB14 partition, routed projection and persistent endpoint join; future campaign split will separate frozen operand projection from comparison/diagnostics. |
| `direct_runtime/surface_liquid_wb14.rs` | 303 | PASS; dedicated shared production transition. |
| `direct_runtime/runoff.rs` | 2,852 | WARN: this package adds only the shared WB14 transition call; a future direct-runtime decomposition package will separate WB14 legacy/profile mechanics from unrelated runoff kernels. |

Strict affected-crate Clippy passes with `-D warnings`; no line-count lint is
suppressed. The WARN files remain coherent existing integration surfaces and
require no exception under `crates/AGENTS.md`.
