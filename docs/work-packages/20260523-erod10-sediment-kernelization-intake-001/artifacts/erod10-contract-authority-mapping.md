# EROD10 Contract Authority Mapping

Status: `completed`
Evidence mode: `Static + Ran`

Static:
- Mapping synthesized from canonical contract files and their current
  promotability gaps.

Ran:
- Contract sections, invariants, and gap registers were inspected with `rg` and
  `sed` in the repository worktree.

## Canonical Contract Map for Erosion Lane

| contract_id | erosion-lane authority surface | key invariants/symbols | current gap posture | closure owner |
|---|---|---|---|---|
| `SC-SED-001` | Core Chapter-11 hillslope erosion authority and payload export surface. | `INV-SED-001..011`, `G`, `Di`, `Df`, `Tc`, `peakro`, `watdur`, `sed_*`. | `GAP-SED-002/003` non-promotable. | `EROD11`, `EROD12` |
| `SC-HYDRAULICS-001` | Shear/friction/rill-width producer authority consumed by erosion branches. | `INV-HYDRAULICS-001..012`, `tau`/`tau_f`, `fs/ft`, `w`, `peakro`, `watdur`. | `GAP-HYD-002/003` non-promotable. | `EROD11`, `EROD12` |
| `SC-RUNOFFPART-001` | Event runoff/infiltration/peak partition authority feeding erosion hydrologic forcing. | `INV-RUNOFFPART-001..010`, `Q`, `peakro`, `watdur`, event branch metadata. | `GAP-RUNOFFPART-002/003/004` non-promotable. | `EROD11`, `EROD12` |
| `SC-WATBAL-001` | Daily closure and WB16 peak diagnostics authority for erosion-coupled hydrology payloads. | `INV-WATBAL-001..013`, `Q`, `peakro`, `watdur`, closure surfaces. | `GAP-WATBAL-002/003` non-promotable. | `EROD11`, `EROD12` |
| `SC-ROUTE-001` | Downstream routing/channel consumer authority for erosion payload and watershed coupling. | `INV-ROUTE-001..013`, `qsed*`, `Tc`, `qpo`, `durrof`, handoff payload semantics. | `GAP-ROUTE-002/003/005` non-promotable. | `EROD11`, `EROD12`, `WS10` |

## Runtime Boundary Producer/Consumer Map

| boundary_id | producer authority | payload | consumer authority | owner package |
|---|---|---|---|---|
| `EROD-BND-001` | `SC-RUNOFFPART-001` + `SC-WATBAL-001` | `Q`, `peakro`, `watdur`, hyetograph-derived metadata | `SC-SED-001` (`INV-SED-004`) | `EROD13` |
| `EROD-BND-002` | `SC-HYDRAULICS-001` | `tau`/`tau_f`, `fs`, `ft`, `w`, friction terms | `SC-SED-001` (`INV-SED-005/006/007`) | `EROD13` |
| `EROD-BND-003` | `SC-SED-001` | `sed_det_total`, `sed_dep_total`, `sed_conc_i`, `sed_frac_i` | `SC-ROUTE-001` (`INV-ROUTE-011`) | `EROD15` + `WS10` |
| `EROD-BND-004` | `SC-ROUTE-001` | channel/watershed routing sediment continuity surfaces | downstream watershed accounting consumers | `WS10` |

## Ownership and Hold Rules

| rule_id | rule | gate impact |
|---|---|---|
| `EROD-AUTH-001` | Identity-only alias rows in companion contracts are not sufficient for production erosion coupling. | `HOLD` until `EROD11` publishes explicit canonical->runtime alias ownership. |
| `EROD-AUTH-002` | Cross-domain non-promotable companion gaps must be dispositioned before kernel code-authoring packages proceed. | `HOLD` until `EROD12` closes or authority-accepts `GAP-SED-003`, `GAP-HYD-003`, `GAP-ROUTE-003`, `GAP-RUNOFFPART-004`. |
| `EROD-AUTH-003` | Erosion export payload cannot claim production downstream coupling while WS10 remains queued. | `HOLD` on `EROD15` closeout until WS10 production path exists. |

## Contract-First Sequencing Requirement (For EROD11+)

Each kernel-authoring package in the erosion lane must execute this sequence:

1. Canonical contract amendments in `docs/specifications/science-contracts/contracts/SC-*.md`.
2. Contract-derived tests for amended invariants and guard codes.
3. Pre-implementation contract-gate evidence (expected fail or missing behavior evidence).
4. Production code changes.
5. Post-implementation verification + required repository gates.

This mapping is the contract-authority baseline for all EROD follow-on packages.
