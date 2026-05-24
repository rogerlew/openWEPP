# EROD11 Cross-Contract Boundary Ownership Register

Status: `completed`
Evidence mode: `Static + Ran`

## Boundary Register

| boundary_id | producer authority | consumer authority | alias closure posture | wave gate impact |
|---|---|---|---|---|
| `EROD-BND-001` (`Q`, `peakro`, `watdur`, `wb16_*`) | `SC-RUNOFFPART-001` + `SC-WATBAL-001` | `SC-SED-001`, `SC-HYDRAULICS-001`, `SC-ROUTE-001` | Explicit typed runtime aliases ratified in canonical `SC-*` addenda. | Closes `EROD10-AH-001` for hydrology-to-erosion/routing forcing surfaces. |
| `EROD-BND-002` (hydraulics shear/friction -> erosion) | `SC-HYDRAULICS-001` | `SC-SED-001` | Canonical identity aliases retained; runtime projection ownership is deferred under erosion-physics `HOLD`. | Removes ownership ambiguity while preserving implementation HOLD posture. |
| `EROD-BND-003` (erosion sediment payload -> routing) | `SC-SED-001` | `SC-ROUTE-001` | Canonical identity aliases retained; runtime projection ownership is deferred under erosion-physics `HOLD`. | Removes ownership ambiguity while preserving downstream implementation HOLD posture. |
| `EROD-BND-004` (`ws10_channel_*`, `ws10_impoundment_*`, contributor peaks) | `SC-ROUTE-001` (+ `SC-HYDRAULICS-001` coupling) | WS10/impoundment/watershed consumers | Explicit typed runtime alias families ratified. | Confirms WS10-coupled boundary ownership is explicit for erosion-lane planning. |

Static:
- Ownership rows are canonicalized in `SC-SED-001`, `SC-HYDRAULICS-001`,
  `SC-ROUTE-001`, `SC-WATBAL-001`, and `SC-RUNOFFPART-001`.

Ran:
- Ownership rows were verified against updated contract text and symbol
  projections in `openwepp-kernel-contract`.
