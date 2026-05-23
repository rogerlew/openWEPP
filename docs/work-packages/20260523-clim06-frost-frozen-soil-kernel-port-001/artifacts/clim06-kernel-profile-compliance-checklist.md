# CLIM06 Kernel Profile Compliance Checklist

Status: `completed`
Evidence mode: `Static + Ran`

## Required Contract-First Sequence
- [x] Contract amendments implemented in canonical SC files before kernel edits.
- [x] Contract-derived tests implemented before kernel edits.
- [x] Pre-implementation contract gate executed and recorded before kernel edits.
- [x] Production CLIM06 kernel/runtime edits performed after gate evidence.

## Kernel Profile Requirements
- [x] No silent defaults/clamping for active-coupling missing/non-finite/domain-invalid frost symbols.
- [x] Typed guard family preserved at WB14 phase (`HKERNEL-WB14-RUNOFF-E-001..003`).
- [x] Runtime seam uses typed errors for frost projection (`HS-RUNTIME-E-054/055`).
- [x] CLIM06 runtime surfaces are explicit (`frost.runtime_dfrost/dthaw/nft/ws_frz/infcap_frz`).

## Required Repository Gates
- [x] `cargo fmt --check`
- [x] `cargo clippy --workspace --all-targets -- -D warnings`
- [x] `cargo test --workspace`
- [x] `cargo deny check`
