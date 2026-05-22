# PL Runtime State Surface Map

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Baseline PL behavior is distributed across schedule controls, growth state, and decomposition/residue partitions.

Ran:
- Derived required runtime surfaces from audited baseline pathways and current parser output structures.

## Runtime Surface Families

| family | canonical symbols (minimum required set) | mutability | owning boundary |
|---|---|---|---|
| `pl_schedule` | `lanuse`, `nowcrp`, `itype`, `imngmt`, `tilseq`, `conseq`, `drseq`, `jdplt`, `jdharv`, `jdstop`, `resmgt`, `mgtopt`, `gday`, `gend`, `rw` | mutable (daily/seasonal controls) | parser-to-runtime PL adapter (`PL-MAN-SEAM-001`) |
| `pl_growth` | `vdmt`, `tlive`, `cancov`, `canhgt`, `lai`, `rtmass`, `rtd`, `sumgdd`, `hia`, `vdmx`, `isenes`, `ncount` | mutable (daily growth/senescence) | growth kernel boundary (`PL-GROW-SEAM-001`) |
| `pl_decomp` | `rmagt`, `rmogt(1..3)`, `rilrm(1..3)`, `rigrm(1..3)`, `smrm(1..3)`, `rtm(1..3)`, `iresd(1..3)`, `iroot(1..3)`, `senvin`, `fenvin(1..3)`, `benvin(1..3)` | mutable (daily decomposition + transitions) | decomposition/transition boundary (`PL-DECOMP-SEAM-001`, `PL-TRANSITION-SEAM-001`) |

## Ordering-Critical Couplings

1. `decomp` ordering must run ahead of same-day `soil/watbal` impacts on cropland pathways.
2. Growth transition events call residue transition logic (`resup`-equivalent) and therefore jointly mutate growth + decomposition surfaces.
3. `lanuse` and `imngmt` together determine legal runtime branch path and required symbol subset.

## Projection Notes for PL03

- Projection from `ManagementParseOutput` must produce typed structs that preserve section identity and slot indexing.
- Slot-indexed symbols should carry deterministic indexed aliases (for example `{idx4}` conventions) for parity/comparator instrumentation.
- Required symbol presence is strict; omitted mandatory fields must be typed failures.

## Evidence Links

- `/workdir/wepp-forest_260430_baseline/src/tilage.for:231`
- `/workdir/wepp-forest_260430_baseline/src/tilage.for:232`
- `/workdir/wepp-forest_260430_baseline/src/tilage.for:274`
- `/workdir/wepp-forest_260430_baseline/src/tilage.for:380`
- `/workdir/wepp-forest_260430_baseline/src/contin.for:811`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for:881`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for:913`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:464`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:509`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:559`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:696`
- `/workdir/wepp-forest_260430_baseline/src/decomp.for:579`
- `/workdir/wepp-forest_260430_baseline/src/decomp.for:605`
- `/workdir/wepp-forest_260430_baseline/src/decomp.for:633`
- `/workdir/wepp-forest_260430_baseline/src/resup.for:207`
- `/workdir/wepp-forest_260430_baseline/src/resup.for:221`
- `/workdir/wepp-forest_260430_baseline/src/resup.for:371`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs:253`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs:1082`
