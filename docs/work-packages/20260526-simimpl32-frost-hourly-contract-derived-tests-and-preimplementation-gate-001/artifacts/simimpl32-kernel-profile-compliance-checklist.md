# SIMIMPL32 Kernel Profile Compliance Checklist

Status: complete-with-hold
Evidence mode: static+ran
Date: 2026-05-26

## Static
| requirement | result | notes |
|---|---|---|
| Contract-first sequence respected | pass | SIMIMPL32 executed steps 2 and 3 only (tests + gate). |
| Canonical `SC-*` authority preserved | pass | No contract-authority drift introduced in package execution. |
| Baseline provenance anchor retained | pass | SIMIMPL32 vectors and gate posture track SIMIMPL31 authority and baseline anchor. |
| No heuristic/proxy frost substitutions introduced in production path | pass | Test-only package; no production physics edits. |
| Typed guard failure posture preserved | pass | Missing seam symbols and lineage gaps fail explicitly in SIMIMPL32 vectors. |
| Pre-migration behavior captured as failing vectors | pass | Ignored-vector run reports 5/5 expected failures. |
| Runtime migration closure achieved | hold | Deferred to SIMIMPL33/SIMIMPL34 and SIMIMPL35 rerun/disposition. |

## Ran
- `cargo test -p openwepp --test clim06_frost_frozen_soil_kernel_contract -- --ignored --nocapture`
- `cargo test --workspace`
- `cargo deny check`
