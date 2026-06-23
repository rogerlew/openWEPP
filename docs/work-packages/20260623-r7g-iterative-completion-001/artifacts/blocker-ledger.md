# Blocker Ledger

Status: HOLD-R7G-FROST-STATEFUL-SUBSOLVER-REQUIRED.

## R7G-001: Sidecar Presence Treated As Active Snow

State: closed.

Evidence:

- Prior R7G full H2637 direct default and explicit direct failed closed at
  `HOLD-R7G-SURFACE-FREE-ACTIVE-SNOW-PARTITION-AUTHORITY-ABSENT`.
- Static inspection showed production direct treated snow sidecar presence as
  active snow authority.
- `SC-SNOWFREEZE-001` makes `snow.options.snow_file_present` provenance only.

Correction:

- Production direct now keeps sidecar-only snow inactive unless runtime SWE is
  present or the day is thermally active with projected controls.
- Added focused sidecar-only and runtime-SWE activation tests.

## R7G-002: Real Active Snow Partition Authority

State: closed.

Evidence:

- H2637 direct default advanced to real active snow at lane 1 day 13:
  `rainfall_m=0.0112`, `runtime_swe_m=0`, `tmax_c=0`, `tmin_c=-3.9`,
  controls projected.

Correction:

- Added typed winter hourly forcing, persistent `DirectSnowRuntimeCarry`,
  typed active snow partition inputs, direct snow compute, downstream operands,
  hydrology projection, and snow carry mutation.

## R7G-003: Same-Day Upstream EROD14 Qout Handoff

State: closed.

Evidence:

- H2637 direct default failed at lane 2 day 65:
  `R7D8 prior-lane erosion qout for EROD14 qin must execute before this span`.

Correction:

- Split erosion publication authority from typed `qout` handoff authority.
- Added a two-lane regression proving downstream EROD14 accepts committed
  upstream zero `qout` without compatibility fallback.

## R7G-004: Snow Liquid Hyetograph Mismatch

State: closed.

Evidence:

- H2637 direct default failed at lane 1 day 1097 with negative runoff:
  `liquid_input_m=0.003685864156251032` and
  `cumulative_infiltration_m=0.003816234031263418`.

Reduced mechanism:

- WB14/WB16 consumed a raw-rainfall hyetograph scaled only by canopy
  interception computed from post-winter rain. When snow retained rain, this
  inflated the infiltration event above direct liquid input.

Correction:

- Project raw hyetograph to post-winter rain first, apply canopy interception
  second, then add daily routed melt uniformly over the event duration.

## R7G-005: Active Frost Typed Direct Producer Absent

State: closed for endpoint/performance; superseded by R7G-006 for residual
parity.

Active marker:

- `R7G-005-ACTIVE-FROST-TYPED-DIRECT-PRODUCER-ABSENT`.

Evidence:

- Full H2637 direct default reaches endpoint with zero compatibility edges and
  `13.16-13.21 s` runtime.
- Protected output parity still fails: HBP/WAT/PASS differ between
  compatibility and direct.
- Prior WAT reduced deltas included active frost surfaces:
  `frozwt` differs on `34318` rows with max `46.704064939847704 mm`, and
  `frdp` differs on `34318` rows with max `288.0580378929632 mm`.
- At that point direct production remained frost-free for those rows because no
  active frost producer fed R4A/R4PQZ in the production direct hot loop.

Correction:

- Added production active-frost day context from direct lane state, typed
  winter hourly forcing, snow carry, frost runtime carry, and direct
  `DirectFrostRunoffSurface` authority.
- Fed frost infiltration cap and R4A frost surface projection without invoking
  compatibility edges.
- Fixed tiny-runoff peak-rate domain handling exposed by full H2637 frost
  execution.
- Removed redundant R4A frost solves for no-material partitions, shrank the
  retained frost surface template, and added a conservative zero-prior
  no-freeze fast path using the authoritative branch equations.

Evidence:

- `r7g_cont_direct_default_frost5`: endpoint `163.88 s`, zero compatibility
  edges, performance red.
- `r7g_cont_direct_default_frost6`: endpoint `122.43 s` after no-material R4A
  skip, still performance red.
- `r7g_cont_direct_default_frost7`: endpoint `94.08 s` after frost template
  shrink, still narrowly red.
- `r7g_cont_direct_default_frost10`: endpoint `87.11 s` after zero-prior
  no-freeze fast path, performance green.
- `r7g_cont_direct_default_frost11`: endpoint `89.88 s`, zero compatibility
  edges, retained source-state benchmark.

Disposition:

- The absent-producer endpoint/performance blocker is closed.
- The path is still not a full no-compatibility architecture proof because
  active frost compute remains symbol-map backed.
- Protected output parity remains red and is tracked as R7G-006.

## R7G-006: Frost/Snow Projection Parity Residuals

State: held; superseded by frost architecture migration.

Active marker:

`R7G-006-FROST-SNOW-PROJECTION-PARITY-RESIDUALS`.

Evidence:

- Latest retained direct capture:
  `/tmp/r7g-cont-h2637/capture/direct-frost11/`.
- Compatibility capture: `/tmp/r7g-cont-h2637/capture/compat/`.
- Direct manifest:
  `/tmp/r7g-cont-h2637/manifests/direct-default-frost11.json`.
- WAT schemas and row counts match, but protected WAT parity is red:
  `frozwt` differs on `34363` rows with max `11.12017732034371 mm`;
  `frdp` differs on `34363` rows with max `264.39519767438975 mm`.
- First frost mismatch occurs at year 1, simulation day 5, Julian day 5:
  compatibility OFE 1 has `frozwt=0.005660437443662737` and
  `frdp=0.11700610732602637`, direct has both zero.
- Largest frost-depth residual occurs at year 13, simulation day 4388,
  Julian day 5, OFE 6: compatibility `frdp=264.39519767438975`, direct
  `frdp=0.0`.
- Snow projection is also material: `Snow-Water` differs on `21305` rows with
  max `183.04425009202413 mm`, and `RM` differs on `14234` rows with max
  `39.94882220799281 mm`.
- PASS remains red: `runvol`, `sbrunv`, and `peakro` differ.
- Loss JSON and plot output remain byte-identical.

Attempted corrections:

- Preserving seed fine-layer symbols when no frost carry existed produced no
  output checksum change.
- Explicit production fine-layer seeding from lane layers produced no output
  checksum or delta improvement and regressed runtime to `101.16 s`; it was
  reverted.

Current reduced mechanism:

- Active frost is no longer absent, but the symbol-map frost projection still
  under-projects first-freeze and later seasonal frozen depth/water versus
  compatibility.
- Later snowpack/runoff divergence is also material and must be reduced after
  or alongside frost, because the largest `Snow-Water`/`RM` residuals are not
  explained by WAT frost columns alone.

Additional reduction during continuation:

- `direct-default-frost28` completed H2637 direct default in `107.96 s` with
  `compatibility_edge_invocations=0`; day-5 frost trace had `921` rows.
- `compatibility-runtime` with the same day-5 trace produced the required
  early rows before failing later at
  `HKERNEL-WB12-STORAGE-E-003 [sim_day_index=4551, calendar_year=1999,
  julian_day=168]`; the day-5 trace had `456` rows and was sufficient for
  reduction.
- The first WAT frost residual at simulation day `5`, OFE `1` was reduced to
  fine-layer carry loss rather than water closure failure:
  compatibility `Total-Soil=541.5228096330922`,
  `frozwt=0.005660437443662737`, `frdp=0.11700610732602637`; direct
  `Total-Soil=541.5280763089495`, `frozwt=0.0003937615864288717`,
  `frdp=0.002320648691535853`.
- Trace comparison showed direct entered that solve with homogeneous
  top-layer fine liquid (`slsw_theta ~= 0.169677...` in the earlier trace),
  while compatibility carried heterogeneous prior winter/frost fine state
  (`0.048377...`, then `0.321523...`, `0.361662...`). Total liquid plus
  frozen storage closed; the partition/front state did not.

Attempted in-envelope corrections after this reduction:

- Removed material-state gating from direct frost handoff so active frost
  partitions are forwarded even when current-day frozen material is zero.
- Made R4A preserve `DirectFrostRuntimeCarry` for every active frost partition,
  not only material-frozen partitions.
- Changed the no-freeze fast path to echo fine/shadow carry instead of
  clearing it.
- Added source/regression guards:
  `r7g_direct_production_hands_active_frost_context_to_r4a_without_material_gate`,
  `r7g_active_zero_material_frost_partition_preserves_fine_runtime_carry`, and
  `r7g_active_no_freeze_frost_partition_carries_fine_state_without_coarse_projection`.
- Found and corrected an over-projection failure mode where no-freeze/no-material
  frost carry emitted or consumed coarse active-water-only layer projection and
  stripped residual water from WAT `Total-Soil`.

Latest measured endpoint evidence before the final no-material consumer
safeguard:

- `direct-default-frost29`: exit `0`, `188.57 s`, `941936 KiB`, zero
  compatibility edges. This preserved fine carry but was performance-red.
- `direct-default-frost30`: exit `0`, `195.27 s`, `942324 KiB`, zero
  compatibility edges. WAT day 1 OFEs `4-7` still showed residual-stripped
  `Total-Soil=484.948003721413` versus compatibility
  `~527.86-527.89 mm`, proving full-solve no-material projection had the same
  coarse authority bug as the no-freeze path.
- After `direct-default-frost30`, R4A's no-material consumer branch was changed
  to ignore coarse layer projection and use only
  `frwatc_net_liquid_delta_m`; focused tests pass, but H2637 has not been
  rerun after that last consumer safeguard.

Current disposition:

- HOLD: `HOLD-R7G-FROST-STATEFUL-SUBSOLVER-REQUIRED`.
- The present architecture uses one-day symbol/request surfaces and retrofitted
  direct carries to emulate a stateful frost process. That has now produced
  repeated projection and performance failures: preserving the true fine-layer
  state increases hot-loop work, while treating fine/shadow state as coarse
  layer projection corrupts storage.
- The next package must migrate frost to a coupled stateful sub-solver with
  rich persistent lane state, then rerun R7G's no-compatibility, parity, and
  performance gates.
