# simimpl03 contract amendment matrix

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Static
- Matrix maps SIMIMPL03 gap families to canonical contract authority updates.
- Scope is contract/index/governance authoring only; no production code mutation.

## Ran
- Verified amended sections via direct contract probes (`rg`/`sed`) after edits.

## Amendment matrix
| Gap ID | Authority target(s) | Amendment closure authored | Result |
|---|---|---|---|
| `GAP-SIMPIPE-001` | `SC-WATBAL-001`, `SC-SYSTEM-001` | Added production execution-ownership invariants and guard mapping: `INV-WATBAL-018`, `INV-SYSTEM-018`; typed guard families `HS-SIMPIPE-E-001`, `WS-SIMPIPE-E-001`; addendum rules prohibiting projection-only publication when execution-owned claims are made. | closed-at-contract-layer |
| `GAP-SIMMODE-001` | `SC-WATBAL-001`, `SC-SYSTEM-001`, `SC-INFILE-WEPPUI-001` | Added deterministic requested/effective mode to lane closure authority: `INV-WATBAL-019`, `INV-SYSTEM-019`, `D-WUI-005`, `G-WUI-008`, `WUI-E-005`; required `selected_lane` provenance mapping. | closed-at-contract-layer |
| `GAP-SIMOUT-001` | `SC-WATBAL-001`, `SC-SYSTEM-001`, `SC-INFILE-WEPPUI-001` | Added simulation-owned WB13/replay-surface provenance authority and projection/synthesis prohibition for required candidate outputs: `INV-WATBAL-020`, `INV-SYSTEM-020`, `G-WUI-009`. | closed-at-contract-layer |
| `GAP-SIMCONS-001` | `SC-WATBAL-001`, `SC-SYSTEM-001` | Added selective consolidated-intake triage governance (`adopt`/`defer`/`reject`) and no-wholesale-intake guardrails: `INV-WATBAL-021`, `INV-SYSTEM-021` with `HS-SIMCONS-E-001`, `WS-SIMCONS-E-001`. | closed-at-contract-layer |

## Registry alignment
- Updated `science-contracts/index.md` notes for `SC-WATBAL-001` and `SC-SYSTEM-001` to reflect SIMIMPL03 authority additions.
