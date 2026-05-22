# Plant/Landuse/Growth/Decomposition Follow-On WP Queue (PL01)

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Queue order is dependency-driven from PL01 boundary decision and architecture constraints.

Ran:
- Derived sequence from baseline coupling map and openWEPP seam/alias gaps.

| wp_id | title | objective | dependencies |
|---|---|---|---|
| `PL02` | PL Runtime Boundary Contract | Define typed runtime state contract for landuse-management schedule, plant growth state, and decomposition/residue state surfaces with canonical symbol tables. | PL01 decision |
| `PL03` | Management-to-Runtime Adapter | Implement strict orchestrator adapter from parsed management output to PL runtime surfaces (no silent defaults; typed errors only). | PL02 |
| `PL04` | PL Symbol Alias Completion | Extend `openwepp-sim-contract` alias registry for PL canonical symbols (`lanuse`, `itype`, `imngmt`, `jd*`, `vdmt`, `cancov`, `lai`, `rmagt/rmogt/smrm/rtm`, etc.). | PL02 |
| `PL05` | Growth Kernel Surface Scaffolding | Add kernel-facing typed interfaces and placeholder scheduler phases for annual/perennial growth state transitions. | PL02, PL03, PL04 |
| `PL06` | Decomposition/Resup Kernel Surface Scaffolding | Add typed interfaces and scheduler integration for decomposition + residue partition transitions preserving baseline phase order. | PL02, PL03, PL04 |
| `PL07` | PL Parser-to-Runtime Integration Tests | Add integration tests asserting full PL runtime surface projection from `.man` fixtures (including typed reject paths). | PL03, PL04 |
| `PL08` | Comparator Confidence-Tier Review (PL) | Run single-OFE daily water-balance + plant/residue parity investigation after PL05/PL06 integration. | PL05, PL06, PL07 |

## Queue Notes

- `PL03` and `PL04` can run in parallel if write sets remain disjoint.
- `PL05` and `PL06` should share one agreed scheduler ordering contract before implementation starts.
- `PL08` is an investigation/acceptance signal, not a standalone rejection gate for hourly/watershed divergences.
