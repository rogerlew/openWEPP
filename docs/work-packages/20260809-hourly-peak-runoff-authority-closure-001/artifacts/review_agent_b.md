# Independent Hydrology/Science Review B

Status: `executed`

Evidence class: `Static: commit c7dbfefe7 contract, implementation, tests, real publication/HBP/pass consumers, and package evidence`

Verdict: `HOLD`

Reviewer independence: Reviewer A's output was not read before this verdict.

## Severity-Ranked Findings

### `SCI-B-001` — CRITICAL — positive runon can still receive synthetic uniform timing

The corrected contract hard-fails positive runoff without reconstructible
hourly timing and explicitly prohibits a synthetic uniform fallback
(`docs/specifications/science-contracts/contracts/SC-WATBAL-001.md:508-510`).
Production still manufactures a uniform surface-runon distribution when
`surface_total_m > 0` but upstream surface weights sum to zero, and likewise a
uniform lateral-runon distribution when lateral carry has no positive hourly
shape
(`crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:640-695`).
The behavior is directly ratified by
`crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_dc01.rs:27-44`.
That synthesized supply is then injected into WB14, so WB14 creates a positive
hourly excess series and the later positive-runoff/missing-shape guard sees an
apparently reconstructible source. The guard at `runoff.rs:1430-1458` therefore
cannot detect the original timing loss. The same shared shape helper also still
returns uniform weights for positive runoff with no raw source
(`runoff.rs:1377-1427`), and publication/executor comments continue to describe
that production fallback
(`direct_runtime/01_publication.rs:276-303`;
`direct_runtime/03_executor.rs:987-1005`).

This is proxy timing, not preservation of modeled timing, and it directly
violates the package correction envelope (`package.md:80-85`) and
`INV-WATBAL-102/103`.

Proposed disposition: `accepted / closure-blocking`. Make positive surface or
lateral runon with a missing/zero hourly source shape a typed failure before
WB14 admission. Remove the uniform branches from production helpers and replace
the uniform-fallback test with negative fail-closed vectors through the real
multi-OFE runon path. Retain exact zero vectors only for exact zero runoff.

### `SCI-B-002` — CRITICAL — routed melt is counted twice in the peak-shape operand mixture

The runner adds daily routed-melt handoff depth to the post-interception
hyetograph before WB14 infiltration
(`crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs:348-359,376-410`).
WB14's `hourly_excess_m` consequently already contains the non-infiltrated
portion of routed melt on the runoff side of the infiltration calculation.
The peak/shared-shape assembler then adds the full
`hourly_routed_melt_m` array again to WB14 hourly excess and saturation return
before normalizing
(`crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:1377-1415,1430-1458`).

This double counts melt as a shape operand and, more importantly, gives
infiltrated melt direct influence over runoff timing. Normalization back to
daily `q_runoff_m` preserves volume but does not repair the distorted temporal
distribution. The melt-only test is tautologically constructed with zero WB14
excess plus an arbitrary positive runoff scalar
(`direct_runtime_dc01.rs:46-76`); it does not execute the real runner path where
melt first enters WB14. The operand ledger's claim that WB14 excess and hourly
routed melt are two independent authoritative source limbs
(`artifacts/operand-lineage.md:7-10`) is therefore false for the implemented
lineage.

Proposed disposition: `accepted / closure-blocking`. Define and implement a
non-overlapping decomposition. The simplest authority-consistent route is for
WB14 hourly excess to carry the timing of all supply that actually becomes
infiltration-excess, with hourly saturation return added separately; do not add
gross routed melt again. If a distinct melt limb is retained, WB14 must expose
source-tagged excess components proving the melt contribution is excluded from
the generic excess limb. Add a real runner-path rain/melt test in which part of
melt infiltrates and independently reconstruct the hourly runoff series.

### `SCI-B-003` — MEDIUM — the public claim boundary is broader than the proved peak surface

`SC-WATBAL-001.md:127-130` and `artifacts/operand-lineage.md:10-13` call the
published quantity a hillslope maximum-hourly flow. Implementation computes a
per-lane depth-rate peak, rescales it to that lane's `runvol_basis_m`, and
multiplies by that lane's area
(`direct_runtime/01_publication.rs:452-488,582-614`). HBP selects the outlet
sediment row and reconstructs hourly volume from that row's runoff volume and
fractions (`crates/openwepp-runner/src/hillslope/04_direct_publication.rs:383-480,484-528`).
This is potentially a valid multi-OFE exit-flow construction because runon is
carried through the outlet lane, but the package supplies no independent
multi-OFE proof that outlet-row hourly volume equals the complete routed
hillslope exit hydrograph or that the per-lane basis adjustment preserves its
maximum. The Topanga claim explicitly excludes routed-watershed validation
(`package.md:111-125`) and the reported probe is only one baseline/mutation
limit (`artifacts/summary.md:7-25`).

Proposed disposition: `accepted`. Narrow closure claims to the demonstrated
single-OFE/per-lane maximum-hour surface unless an active multi-OFE fixture
independently reconstructs outlet hourly volumes from upstream carry plus local
runoff, verifies HBP/pass equality, and distinguishes this from watershed or
channel-routed peak flow. Continue to prohibit any instantaneous or routed-
watershed peak claim.

## Adversarial Checks Without Additional Findings

- Equal-volume concentrated versus spread shapes correctly produce a 24:1 peak
  ordering and rectangular-equivalent durations of 3,600 versus 86,400 seconds
  (`direct_runtime_dc01.rs:275-295`). The duration is consistently labeled as
  `Q / peak_depth_rate`, not rainfall duration, hydrograph duration, or time to
  peak (`SC-WATBAL-001.md:127-130`;
  `SC-INFILE-HBP-001.md:116-124,238-243`).
- Saturation-only timing is preserved in its produced hour and positive runoff
  with all three peak inputs exactly zero fails at the direct helper
  (`direct_runtime_dc01.rs:78-95,127-138`). Those focused checks are valid but
  do not cure the upstream proxy and melt-overlap defects above.
- Internal units are correctly depth rate (`m/s`), public conversion is one
  multiplication by a positive `area_m2`, and publication rescales the peak to
  the same run-volume depth basis before applying area
  (`direct_runtime/01_publication.rs:452-488,582-614`). No second area multiply
  occurs in HBP (`04_direct_publication.rs:401-408,436-480`).
- HBP minor-1 semantics and pass metadata correctly say maximum hourly mean
  volumetric flow, and HBP reconstructs `max(hourly_volume)/3600` on its event
  volume basis (`SC-INFILE-HBP-001.md:116-124,238-243`;
  `crates/openwepp-hillslope-output/src/hillslope_pass.rs:141-145`).
- The near-zero canonicalization is source-informed and bounded to
  `<=1e-12 m`, rather than a positive peak floor
  (`runoff.rs:1362-1375`; `SC-WATBAL-001.md:863-871`).

## Verdict Rationale

`HOLD` is mandatory. `SCI-B-001` retains explicitly prohibited synthetic
runon timing, while `SCI-B-002` distorts the hydrograph by overlapping gross
routed melt with WB14 excess that already consumed melt. Both can change the
maximum hour and peak magnitude while preserving daily volume, defeating the
core authority claim. They require production and contract/test reconciliation
before review can pass. `SCI-B-003` must be resolved by evidence or claim
narrowing before multi-OFE hillslope publication is claimed closed.
