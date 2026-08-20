# Owned File Manifest

Status: frozen before contract edits

Evidence mode: Static

| Path | Intended edit | Risk |
| --- | --- | --- |
| `docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md` | new authority | critical |
| `docs/specifications/science-contracts/index.md` | lifecycle row | governance |
| package `artifacts/**`, `tools/**`, `package.md`, prompt lifecycle | authority/evidence | critical |
| root `Cargo.toml`, `Cargo.lock` | new workspace crate/test target | integrated |
| `crates/openwepp-coupled-time/**` | new leaf subsystem | critical |
| `crates/openwepp-hillslope-orchestrator/Cargo.toml` and one bounded reference-consumer module/export | real reference consumer | integrated |
| `tests/integration/coupled_time_*` | contract/consumer/restart guards | critical |
| roadmap/catalog/campaign package | truthful terminal lifecycle only | governance |

Protected: all V10, snow, LSE, Lane D, Richards, soil, BGC production kernels;
existing DirectV10 restart V1 types/schema/vectors/manifest/bytes; selectors,
defaults, production output and publication. Reconcile exact terminal diff here
and in `exact-diff-reconciliation.md` before disposition.
