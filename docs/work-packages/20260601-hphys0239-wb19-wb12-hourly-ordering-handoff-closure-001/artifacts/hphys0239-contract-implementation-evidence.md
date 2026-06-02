# HPHYS0239 Contract Implementation Evidence

Status: completed  
Evidence mode: Static

## Contract Amendments

1. `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
   - `contract_version: 68`
   - Added `INV-WATBAL-031` for canonical WB19->WB12->WB13 hydrology-tail
     ordering and WB13 `Q`/`Ep`/`Es`/`Er` flux-authoritative anti-shadow
     semantics.
   - Added guard-map linkage for `INV-WATBAL-031`.
   - Added `HPHYS0239 WB19->WB12->WB13 Ordering and Flux-Authority Handoff
     Addendum`.
   - Updated WB13 lineage register writer surfaces for `Ep`/`Es` and added `Er`
     to `require_runtime_surface_scalar_prefer_flux(...)`.

2. `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
   - `contract_version: 22`
   - Added `INV-SUBHYD-021` for deterministic WB19 `q`/`Qdd`/`Qd` sequencing
     and downstream WB12/WB13 anti-shadow handoff requirements.
   - Added guard-map linkage for `INV-SUBHYD-021`.
   - Extended WB13 coupling requirements with explicit post-WB19 same-pass
     ordering requirement.
   - Added `HPHYS0239 WB19->WB12/WB13 Handoff Ordering Addendum`.

## Measure Mapping

- `MEASURE-HP239-001`: satisfied (ordering + anti-shadow authority encoded in
  canonical `SC-*` contracts).
