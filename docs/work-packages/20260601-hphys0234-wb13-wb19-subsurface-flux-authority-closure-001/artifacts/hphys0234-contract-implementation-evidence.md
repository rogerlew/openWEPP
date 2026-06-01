# HPHYS0234 Contract Implementation Evidence

Status: completed  
Evidence mode: Static

## Static

Updated canonical contract authority in:
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`

Amendments applied:
1. `SC-WATBAL-001` advanced to `contract_version: 65`.
2. `SC-SUBHYD-001` advanced to `contract_version: 20`.
3. WB13 invariants now require flux-authoritative subsurface publication and
   coupling for `D`, `q`, `Qdd`, and `Qd` when state/flux symbol conflicts
   exist.
4. WB13 lineage register writer-surface entries now point to
   `require_runtime_surface_scalar_prefer_flux(...)` for subsurface family
   publication (`D`, `q`).
5. New HPHYS0234 addenda in both contracts record anti-shadow obligations and
   require stale-state-vs-flux conflict probes.

Authority anchors cited in-contract:
- `/workdir/wepp-forest_260430_baseline` (pinned baseline authority per
  ADR-0012 and contract provenance model)
