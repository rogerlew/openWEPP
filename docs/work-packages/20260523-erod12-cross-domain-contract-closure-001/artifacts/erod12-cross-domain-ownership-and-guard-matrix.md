# EROD12 Cross Domain Ownership and Guard Matrix

Status: `completed`
Evidence mode: `Static + Ran`

## Ratified Wave-0 Matrix

| lane_id | boundary surfaces | producer ownership | consumer guard ownership | closure posture |
|---|---|---|---|---|
| `EROD12-CD-001` | `Q`, `peakro`, `watdur`, `wb16_*` | `SC-RUNOFFPART-001` + `SC-WATBAL-001` | `SC-SED-001`, `SC-HYDRAULICS-001`, `SC-ROUTE-001` | Wave-0 hydrology forcing ownership and guard semantics are explicit in canonical contracts. |
| `EROD12-CD-002` | `fr`, `fi/fe`, `w`, `fs`, `ft`, `tau_f/tau_fe` | `SC-HYDRAULICS-001` | `SC-SED-001` | Hydraulics-to-erosion producer/consumer guard ownership is explicit. |
| `EROD12-CD-003` | `sed_det_total`, `sed_dep_total`, `sed_conc_i`, `sed_frac_i` | `SC-SED-001` | `SC-ROUTE-001` | Sediment handoff guard ownership and payload validation ownership are explicit. |
| `EROD12-CD-004` | `hs{ID}_peakro`, `hs{ID}_watdur`, `ws10_channel_{id}_qpo`, `ws10_channel_{id}_durrof`, `ws10_channel_{id}_roff` | `SC-ROUTE-001` (+ coupling surfaces in `SC-HYDRAULICS-001`) | `SC-SYSTEM-001` + WS10/impoundment companion lanes | Cross-domain publication ownership is explicit; non-Wave-0 watershed-release holds remain separate. |
| `EROD12-CD-005` | `D`, `Qd`, `ET`, `I` daily closure companion exports | `SC-WATBAL-001` | downstream hydrology/system consumers | Closure companion ownership is explicit; broader companion promotability remains governed by non-Wave-0 gaps. |

Static:
- Matrix rows are derived directly from canonical EROD12 addenda and gap
  register dispositions in companion `SC-*` files.

Ran:
- Matrix was cross-checked against updated contract text and row-scoped gap
  statuses using repository commands.
