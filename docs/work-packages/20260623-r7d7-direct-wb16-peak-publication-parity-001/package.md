# R7D7 Direct WB16 Peak Publication Parity

Status: executed-held.

Package type: Array-native runtime defect-closure implementation package.

Objective: close
`HOLD-R7D6-PASS-HBP-PEAKRO-COMPATIBILITY-ZERO-RESIDUAL` by adjudicating and
resolving the remaining H2637 PASS/HBP `peakro` residual after R7D6 direct
sediment producer closure.

Rationale: R7D6 proved H2637 direct production exits `0`, WAT is
byte-identical, and PASS sediment fields are parity-clean. The remaining
consumer-path mismatch is `peakro`: compatibility PASS/HBP rows publish `0.0`,
while direct publication emits typed WB16 peak-duration values. Suppressing
direct WB16 peaks to zero would discard producer authority; leaving the
residual unowned blocks byte/consumer-path closure.

Included scope:

- Re-read `SC-HYDRAULICS-001`, `SC-SED-001`, HBP/PASS serialization code, and
  R7D6 artifacts.
- Determine whether compatibility `peakro = 0.0` is a missing WB16 publication
  defect or direct publication requires a contract-authorized serialization
  policy that preserves typed WB16 state while matching required bytes.
- Implement the smallest contract-backed fix.
- Re-run focused H2637 direct/compatibility comparison until PASS/HBP peak
  residual is closed or a narrower contract blocker is proven.
- Preserve R7D6 sediment parity and `compatibility_edge_invocations = 0`.

Excluded scope:

- Reverting direct WB16 peak-duration state.
- Fabricating zero direct peaks solely to match compatibility output.
- Broad erosion/MOFE sediment-coupled `qin` migration beyond the peak
  publication residual.

Intended write set:

- `docs/work-packages/20260623-r7d7-direct-wb16-peak-publication-parity-001/**`
- `docs/work-packages/README.md`
- `docs/architecture/array-native-runtime-specification.md`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-runner/src/hillslope/scheduler_trace/per_ofe_internal_wb13.rs`
- focused tests under touched crates.

Acceptance gates:

- H2637 direct production exits `0` with direct compatibility-edge counters at
  `0`.
- PASS `peakro` residual is resolved with explicit authority; sediment parity
  remains clean.
- HBP residual is resolved or held at a narrower byte-layout/serialization
  blocker with exact byte/field evidence.
- WAT byte identity remains intact.
- Focused direct-production tests pass.
- Review, verification, parity, line-count, and worker-handoff artifacts are
  updated with `Static:` and `Ran:` evidence.

Final disposition:
`HOLD-R7D7-HBP-EROD15-SEDIMENT-EXPORT-ALIASES-DIRECT-PRODUCER-GAP`.

R7D7 closed the R7D6 PASS `peakro` residual by making compatibility PASS
serialization consume the same runtime `peakro` scalar that compatibility HBP
already consumed, and by making direct PASS use producer-authoritative
`runoff.peak_runoff_m3_s` before the erosion copy. Fresh H2637 5-day evidence
has WAT and PASS byte identity with direct
`compatibility_edge_invocations = 0`.

R7D7 remains held only on HBP sediment export aliases. HBP peak/duration are
now identical, but compatibility HBP publishes
`total_detachment_kg = 0.6` and
`sediment_concentration_kg_m3 = 6.816136920064195` for the latest event while
direct HBP publishes `0.0` for both. This is not a WB16 peak blocker and must
not be resolved by suppressing direct peaks or by wrapping compatibility
runtime surfaces. The next hold-lift package is R7D8, focused on direct
EROD15/HBP sediment-export alias authority and the MOFE sediment-coupled
handoff identified by the existing warning.
