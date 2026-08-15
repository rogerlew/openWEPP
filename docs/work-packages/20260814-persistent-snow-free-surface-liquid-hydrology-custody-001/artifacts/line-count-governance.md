# Line-Count Governance

Evidence class: `Static + Ran`

Current affected-file counts after remediation:

| File | Lines | Disposition |
|---|---:|---|
| `direct_runtime/runoff.rs` | 2,852 | Below the mandatory 3,000-line threshold after extracting the shared WB14 transition. |
| `direct_runtime/00_core_frames.rs` | 2,783 | WARN: the increment adds only the bounded optional-owner attachment and validation seam; a broader split is outside this custody package. |
| `direct_runtime/surface_liquid_owner.rs` | 2,347 | WARN: cohesive owner schema, canonical persistence, D/A/F reconstruction, failure payloads, and candidate validation; crate-local tests were extracted below the mandatory threshold. |
| `direct_runtime/surface_liquid_owner_tests.rs` | 876 | PASS; crate-local owner and checked-arithmetic tests extracted mechanically from the production module. |
| `vegetation_real_hydrology_shadow.rs` | 2,157 | WARN: only canonical snapshot and production-lane accessors changed; no new constitutive owner was added. |
| `direct_runtime/surface_liquid_ingress.rs` | 1,941 | PASS: ingress construction, canonical chronological mixing and source-support exposure; crate-local tests are separate. |
| `direct_runtime/surface_liquid_ingress_tests.rs` | 1,858 | PASS; focused ingress, support-window/disposition, multi-hop and domain tests. |
| `land_surface_energy_shadow/mod.rs` | 2,881 | WARN: cohesive default-off arbitration plus complete receiver reconstruction; strict helpers are decomposed and a future campaign split may separate receiver closure DTOs. |
| `direct_runtime/surface_liquid_closure.rs` | 2,003 | WARN: chronological window-keyed raw/post-mix and independently routed projection; below the mandatory threshold. |
| `direct_runtime/surface_liquid_wb14.rs` | 303 | PASS; dedicated shared production transition. |

Strict affected-crate Clippy passes with `-D warnings`; no line-count lint is
suppressed. The WARN files remain coherent existing integration surfaces and
require no exception under `crates/AGENTS.md`.
