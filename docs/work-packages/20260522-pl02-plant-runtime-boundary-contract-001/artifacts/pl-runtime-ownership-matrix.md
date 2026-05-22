# PL Runtime Ownership Matrix

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- PL runtime closure requires single-writer ownership per mutable state family.

Ran:
- Mapped writer/reader boundaries from baseline process flow and openWEPP architecture constraints.

| state family | primary writer | secondary writers | primary readers | owner decision |
|---|---|---|---|---|
| Landuse + management schedule controls (`lanuse`, `itype`, `imngmt`, dates/options) | PL parser-to-runtime adapter | none | scheduler dispatch, growth/decomp kernels | adapter-owned (`PL-MAN-SEAM-001`) |
| Growth biomass/canopy/root state (`vdmt`, `cancov`, `canhgt`, `lai`, `rtmass`, `rtd`) | growth kernel | transition primitive (event-time residue transfer coupling) | ET/water-balance/erosion consumers | growth-owned (`PL-GROW-SEAM-001`) |
| Decomposition residue/root partitions (`rmagt`, `rmogt`, `smrm`, `rtm`, `rilrm`, `rigrm`) | decomposition kernel | transition primitive (`resup`-equivalent) | runoff/cover/soil and reporting surfaces | decomposition-owned (`PL-DECOMP-SEAM-001`) |
| Residue slot identity/environment indices (`iresd`, `iroot`, `fenvin`, `benvin`, `senvin`) | transition primitive + decomposition kernel | none | decomposition and downstream cover/soil consumers | transition/decomp co-owned with explicit phase boundaries |
| Crop-slot transition flags (`nowcrp`, `isenes`, `ncount`) | scheduler + growth transition path | none | growth/decomp kernels | scheduler-owned control surface |

## Ownership Rules

1. Parser and runtime owners are different surfaces:
- parser owns immutable parse output;
- PL adapter owns runtime projection + control-surface initialization.

2. Kernel ownership is phase-bounded:
- growth mutates growth state only;
- decomposition mutates decomposition state only;
- transition primitive mutates slot-shift and handoff surfaces on event boundaries.

3. No ownership ambiguity:
- no fallback writes from readers into writer-owned state;
- missing writer prerequisites are typed errors, not implicit initialization.

## Evidence Links

- `/workdir/wepp-forest_260430_baseline/src/tilage.for:228`
- `/workdir/wepp-forest_260430_baseline/src/tilage.for:376`
- `/workdir/wepp-forest_260430_baseline/src/contin.for:811`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for:881`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:464`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:696`
- `/workdir/wepp-forest_260430_baseline/src/decomp.for:579`
- `/workdir/wepp-forest_260430_baseline/src/resup.for:207`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md:20`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md:78`
