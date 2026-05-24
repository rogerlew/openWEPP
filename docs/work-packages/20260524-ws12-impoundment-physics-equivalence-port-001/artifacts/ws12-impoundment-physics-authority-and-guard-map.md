# WS12 Impoundment Physics Authority and Guard Map

Status: `completed`
Evidence mode: `Static`

## Canonical Authority Surfaces
- `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
  - `contract_version: 5`
  - WS12 continuity/stage-discharge addendum, surrogate deauthorization, guard
    continuity, and baseline provenance anchors.
- `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
  - `contract_version: 10`
  - WS12 consumer-coupling addendum for impoundment payload handling.
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
  - `contract_version: 13`
  - WS12 system-integration addendum for impoundment node publish semantics.

## WS12 Surrogate Deauthorization
WS10 impoundment surrogate parity authority is explicitly removed for WS12:
- `headroom = max(hfull - h, 0)`
- `retention_scale = 1 + headroom`
- `qo = max((incoming_peak / retention_scale) - qinf, 0)`

WS12 parity authority is continuity + stage-discharge integration.

## Runtime Authority Mapping

| Authority component | Canonical WS12 source | Runtime/legacy mapping |
|---|---|---|
| Continuity differential | `SC-IMPOUND-001` WS12 addendum | `dH/dt = (Qi - Qo(H))/A(H)` |
| Stage-area relation | `SC-IMPOUND-001` variables/invariants + WS12 addendum | `A(H) = a0 + a1*H^a2` |
| Outflow composition | `SC-IMPOUND-001` WS12 addendum | `Qo(H)` as min-controller groups + additive structure terms |
| Stage step integration | `SC-IMPOUND-001` WS12 addendum | RK4 update form |
| Adaptive-step retry | `SC-IMPOUND-001` WS12 addendum | half-step/full-step retry with regime-safe step reset |
| System publish behavior | `SC-SYSTEM-001` WS12 addendum | impoundment node outputs remain deterministic + typed guarded |

## Legacy Provenance Anchors (Pinned Baseline)

| Legacy authority | Baseline source anchor | WS12 contract use |
|---|---|---|
| RK4 continuity stage update | `/workdir/wepp-forest_260430_baseline/src/imphnw.for:141-143,357-362` | continuity derivative + final stage update semantics |
| Outflow min-controller composition | `/workdir/wepp-forest_260430_baseline/src/imphnw.for:75-139`; `/workdir/wepp-forest_260430_baseline/src/impflo.for:220-286` | canonical outflow assembly across structure families |
| Adaptive-step error control | `/workdir/wepp-forest_260430_baseline/src/impflo.for:94-147` | retry and next-step proposal behavior |
| Regime-transition step reset | `/workdir/wepp-forest_260430_baseline/src/impflo.for:151-175`; `/workdir/wepp-forest_260430_baseline/src/impmai.for:322-418` | mandatory transition-safe retry semantics |
| Upstream/downstream coupling closure | `/workdir/wepp-forest_260430_baseline/src/wshiqi.for:74-179`; `/workdir/wepp-forest_260430_baseline/src/wshimp.for:207-218` | inflow merge and publish-duration/outflow closure |

## Guard Map (WS12 Continuity)
- Missing required symbol or parser-projected coefficient payload:
  `WKERNEL-WS10-IMPOUNDMENT-E-001`
- Non-finite symbol/intermediate/output:
  `WKERNEL-WS10-IMPOUNDMENT-E-002`
- Domain/regime-transition/continuity violation:
  `WKERNEL-WS10-IMPOUNDMENT-E-003`

## Parser-Coupling Surfaces
- Required impoundment coefficient families are canonicalized in
  `SC-INFILE-WATERSHED-IMPOUNDMENT-001` and consumed by WS12 authority as
  parse-to-runtime prerequisites:
  - structure/regime families: `a,b,c,d,e,ha,ht,hlm`
  - geometry families: `a0,a1,a2,l0,l1,l2`
  - misc controls: `h`, `hfull`, `deltat`, `qinf`
