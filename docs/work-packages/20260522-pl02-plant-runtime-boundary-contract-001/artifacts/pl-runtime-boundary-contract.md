# PL Runtime Boundary Contract

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Baseline plant/landuse/growth/decomposition behavior is a cross-phase mutable state contract, not parser-local configuration.
- openWEPP architecture requires one owner per mutable runtime surface and explicit canonical-symbol alias continuity.

Ran:
- Audited baseline ownership/ordering anchors in `tilage.for`, `contin.for`, `watbal.for`, `grow.for`, `decomp.for`, and `resup.for`.
- Audited openWEPP parser/runtime seam and alias-registry anchors in `management.rs`, `runtime_inputs.rs`, and `symbols.rs`.

## Boundary Contract

| seam_id | owner | inputs | outputs | hard requirements |
|---|---|---|---|---|
| `PL-MAN-SEAM-001` (management parser -> PL runtime state) | `openwepp-hillslope-orchestrator::runtime_inputs` (new PL adapter surface) | `ManagementParseOutput` registries + expanded schedule slots | typed PL runtime surfaces (`pl_schedule`, `pl_growth`, `pl_decomp`) | no silent defaults; typed rejects only; canonical symbol continuity preserved |
| `PL-GROW-SEAM-001` (scheduler -> growth kernel) | hillslope phase scheduler dispatch | `pl_schedule` + `pl_growth` + required soil/water stress symbols | updated growth state (`vdmt`, `cancov`, `canhgt`, `lai`, `rtmass`, `rtd`, senescence controls) | deterministic phase ordering and typed failure propagation |
| `PL-DECOMP-SEAM-001` (scheduler -> decomposition kernel) | hillslope phase scheduler dispatch | `pl_schedule` + `pl_decomp` + environmental index surfaces | updated residue/root partitions (`rmagt/rmogt/rilrm/rigrm/smrm/rtm`) | same-day management effect ordering preserved (`decomp` before `soil/watbal` path) |
| `PL-TRANSITION-SEAM-001` (growth/decomp -> residue transition) | transition primitive (`resup`-equivalent) | harvest/senescence/add-remove event flags + current residue/root state | residue slot shifts and root reset transitions | event-class semantics are explicit (`-2`, `-1`, `0`, `1`, `10..13`) and typed |

## Domain Rules

1. Landuse branch policy:
- Runtime PL execution profile remains cropland-only in current openWEPP executable path (`landuse=1`).
- `landuse=2` remains explicit typed reject in the parser/runtime contract boundary until rangeland execution is intentionally implemented.

2. Management class branch policy:
- `imngmt` controls annual/fallow (`1/3`) vs perennial (`2`) runtime path selection.
- `resmgt` and `mgtopt` controls must be preserved as runtime scheduling controls, not collapsed into parser-only metadata.

3. Mutable state ownership split:
- Schedule/control surfaces are owned by parser-to-runtime adaptation.
- Growth state surfaces are owned by growth kernels during daily execution windows.
- Decomposition/residue surfaces are owned by decomposition/transition kernels.

4. Failure policy:
- Missing required PL surfaces, invalid domains, or non-finite runtime-critical values are typed failures.
- Fallback/wrapper defaulting is prohibited for contract-required PL fields.

## Contract Outcome

`ACCEPT` for PL02 scope: the runtime boundary is explicit, single-owner responsibilities are defined, and implementation constraints for PL03+ are concrete.

## Evidence Links

- `/workdir/wepp-forest_260430_baseline/src/tilage.for:228`
- `/workdir/wepp-forest_260430_baseline/src/tilage.for:268`
- `/workdir/wepp-forest_260430_baseline/src/contin.for:811`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for:881`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for:890`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:464`
- `/workdir/wepp-forest_260430_baseline/src/grow.for:696`
- `/workdir/wepp-forest_260430_baseline/src/decomp.for:579`
- `/workdir/wepp-forest_260430_baseline/src/resup.for:174`
- `/workdir/wepp-forest_260430_baseline/src/resup.for:207`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md:23`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md:57`
- `/home/workdir/openWEPP/docs/architecture/simulation-subsystem-kernel-architecture.md:62`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs:292`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs:643`
- `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/management.rs:1082`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:504`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:771`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:891`
- `/home/workdir/openWEPP/crates/openwepp-sim-contract/src/symbols.rs:255`
- `/home/workdir/openWEPP/crates/openwepp-sim-contract/src/symbols.rs:302`
