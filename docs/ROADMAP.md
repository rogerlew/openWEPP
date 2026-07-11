# openWEPP Engine Roadmap

Status: living — **canonical**, **forward-only planning queue**
Last updated: 2026-07-11
Audience: all contributors
Owner: maintainers (Claude Code maintains this document)

This is the single authoritative roadmap for the openWEPP simulation engine — a
**forward-looking planning queue only.** Completed work is **not** recorded here; it
lives in the [work-packages execution log](work-packages/README.md). When a rung
closes, it is **removed** from this queue and its detail moves to that log. Other
locations (ADRs, backlog, agent memory) point here; when they disagree, this document
wins. See [§ Keeping this current](#keeping-this-current).

---

## How to read this queue

openWEPP is built **architecture-first** with **top-down science contracts** as the
correctness authority
([ADR-0011](decisions/0011-architecture-first-top-down-science-contracts.md)). Legacy
WEPP is a **flag, not a target**
([ADR-0017](decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md)).
Defects are closed with **Defect-Closure ExecPlans**
([ADR-0018](decisions/0018-defect-closure-execplans-conversion-rule.md),
[defect_closure_execplans.md](defect_closure_execplans.md)).

**The ordering principle — closure, not magnitude, not comparator-match.** Each item's
acceptance target is **closure** (does it *conserve* — water/mass balance, bounds),
**not** whether forcing magnitude is physically perfect and **not** whether it matches
a legacy binary. Conservation is independent of magnitude, so the structure is closed
first; **magnitude/physics fidelity is judged last**, against an already-closed and
routed system, so magnitude error is never aliased with structural error. Every item
adds **one mechanism** on an already-closed foundation. Boundaries are **closure gates,
not calendar phases**.

**Current position:** single-OFE water-balance closure, frost
activation/conservation, single-OFE frost-depth heat-flow parity,
MOFE hillslope inter-OFE water-routing closure, the **openWEPP-native
totalwatsed3 CLI + closure** (WSHED01, the WBVAL06/6a deferral, closed
2026-06-14), and the **MOFE >10-OFE far-point demonstration** (FARPOINT01,
closed 2026-06-16 — openWEPP's three identities close at 19 OFEs past the legacy
ceiling; the frost `watbtm` double-count it surfaced was closed contract-first)
are closed. **MAGPARITY01** completed the per-OFE runoff magnitude adjudication:
the H2637 71% `runvol` is not an `INV-RUNOFFPART-028`, area-scaling, or export
defect. **STAGE2-LATQCC-H2637-MAGNITUDE** then traced WB19 per-substep operands
and found no equation, withdrawal, conductivity-override, active-depth, or
`drfc` formula defect; REFINTENT001 landed the ratified `ksatadj`
source-intent saturation-fraction algorithm (a correct defect closure) — **but it
is byte-inert on H2637 (`ksatadj = 0` there), so it does NOT close the FARPOINT01
71% flag.** STAGE2-BASE-CONDUCTIVITY-H2637-MAGNITUDE then proved base `ksat` is
byte-live and found a source-intent defect in vertical `wb18_perc_ssc` 200 mm
normalization. BASECOND01 closed that defect: vertical `ssc` is now harmonic
below the top 200 mm interval while hourly H2637 `wb19_lateral_ssh` remains
arithmetic from `ksat*anisotropy`. The H2637 no-UI rerun was aggregate-inert
(`runvol_pct_precip` remained `71.0036550031206`). POST-BASECOND01 then
resolved the remaining FARPOINT01 71% magnitude flag as
`CORRECT-BY-CONSTRUCTION` / `NO DEFECT` for the verified openWEPP lateral
lineage. The only residual question is absolute physical magnitude, now tracked
as a deferred external-authority `CONTRACT-GAP` in
[backlog/20260618-forest-lateral-flow-absolute-magnitude-authority.md](backlog/20260618-forest-lateral-flow-absolute-magnitude-authority.md).
**Perf track — CLOSED 2026-07-01: both budgets met (H2637 32.77 s = 3.52x legacy, quiet 3-rep; `20260701-hillperf-*` packages).** The read-side
id-table work (PERFOPT01, PERFIDX03B, PERFIDX04) took H2637 978->666 s (-31.9%, bit-identical),
ending at the PERFIDX06 endpoint of **73.12x**; PERFARRAY01/02 then proved that an input-only
array request/accessor seam was still a half-measure. PERFARCH03 closed the missing fully
array-native WB11 runoff branch floor: the branch hot loop measured **0.959423 us/OFE-day** with
boundary materialization measured separately. PERFMIG01 then landed the first production rung:
WB11 warm-rain writeback now emits a 543+8 dense `SymbolId` payload, identity-clean, but H2637
measured **669.97s** (`+0.47%` vs PERFIDX06) because the single-phase compatibility boundary still
costs about **108 us/payload**. PERFMIG02 then migrated hot scalar helpers to dense-first reads and
retired six internal logical materializations, preserving identity, but the final-code H2637 endpoint
was flat/negative at **672.14s / 675.00s**. Its strict apply-boundary attribution also failed:
skip-six apply cost was **105.461 us/payload** vs **104.752 us/payload** for materialize-all because
stale-logical removal outweighed six avoided inserts. PERFDEEP02 then proved the
temporary full-registry dense mirror was worse (`2417 s`, 3.6x), and PERFDEEP03
fixed ownership with a lane-owned compact dense state but still missed the real
endpoint gate (`1147.96 s` vs `669.97 s`). PERFDEEP04 profiled that no-go:
`sync_from_writeback_surface` is the dominant opt-in-only hotspot (`33.49%`
inclusive), with hot-symbol rebuild, symbol-id lookup, and boundary BTreeMap
flush still live. PERFDEEP05 removed that full-sync hotspot and preserved final
H2637 identity, but the opt-in endpoint remained a no-go (`911.11 s` vs the
`669.97 s` activation reference; default-disabled `701.95 s`). The remaining
profile is now cleaner: daily cached-slot refresh, dense logical writeback
apply, `SymbolRegistry::id_of`, and dirty flush dominate. Deep migration remains
fail-closed; PERFDEEP06 completed the fast-path inventory/API planning gate and
also made the default-disabled regression load-bearing: PERFDEEP05
default-disabled measured `701.95 s` versus the `669.97 s` reference, while
PERFDEEP03 default-disabled sat in the `697-708 s` band. PERFDEEP07 is
executed and held: retained cleanup improved the disabled path to `685.85 s`,
but did not meet the P0 `<= 676.67 s` timing gate, so direct-frame hydrology
implementation did not start. The R0/R1 array-native schema and frame planning
package is now complete for planning-only scope. PERFDEEP08 executed the next
narrow hard-isolation attempt and also held: the diagnostic-hook cache
candidate measured `691.93 s`, slower than PERFDEEP07's `685.85 s`, and was
reverted. PERFDEEP09 then closed the R2 blocker: no-edit control reproduced the
failure at `682.65 s`, and the retained one-pass perennial decomposition
indexed-overflow guard measured `634.61/635.65/636.58 s` with median
`635.65 s`, protected identity, and full closure gates. R2A then introduced the
distinct direct-runtime namespace and explicit no-op/shadow direct executor
skeleton, preserving the default-disabled H2637 gate at
`634.06/636.01/640.93 s` (median `636.01 s`). R3A then implemented the first
complete direct phase span, direct transfer-input accounting, with typed inputs,
direct compute, state mutation, downstream operands, and shadow projection.
Its default-disabled H2637 gate passed at `630.31/640.85/632.08 s` (median
`632.08 s`). R3B then added a second direct-runtime span, direct water-ledger
accounting, with richer direct-state dependencies and a signed diagnostic
residual while preserving no-publication/no-R4 boundaries. Its default-disabled
H2637 gate passed at `640.67/643.05/639.21 s` (median `640.67 s`). R3C then
added run-level multi-lane transfer/topology propagation, with diagnostic
transfer ledger state, downstream operands, shadow projection, reciprocal
topology validation, and the same no-publication/no-R4 boundary. Its
default-disabled H2637 gate passed at `640.85/643.41/644.07 s` (median
`643.41 s`). R4A then implemented the first direct hydrology-process span: a
narrow SC-RUNOFFPART-authoritative runoff-partition closure slice with direct
inputs, direct compute, state mutation, downstream operands, and shadow
projection. It preserved the no-publication/no-default/no-scheduler boundary and
passed the default-disabled H2637 gate at `644.01/646.84/643.66 s` (median
`644.01 s`). R4B then added the downstream direct WB12 storage-reconciliation
consumer of R4A direct runoff, with explicit direct operands for the remaining
unmigrated storage inputs, direct state mutation, downstream operands, and
shadow projection. It preserved the same boundary and passed the
default-disabled H2637 gate at `637.34/641.14/646.88 s` (median `641.14 s`).
R4C then migrated the storage-input producer feeding R4B: it consumes R3A direct
precipitation and current direct storage, mutates R4B `storage_initial_m` and
`precip_input_m`, produces downstream storage-input operands, shadow-projects
the result, and split storage-related direct-runtime code into a narrow module.
It preserved no-publication/no-default/no-scheduler boundaries and passed the
default-disabled H2637 gate at `637.63/640.25/639.19 s` (median `639.19 s`).
R4D then migrated the direct WB18/WB12 deep-seepage handoff producer feeding
R4B `deep_seepage_m`: it consumes a dedicated direct `D` handoff, mutates
direct deep-seepage state plus the R4B input, produces downstream operands, and
shadow-projects the result. R4B now requires R4C storage input, R4D deep
seepage, and R4A runoff before reconciliation. It preserved the same boundary
and passed the default-disabled H2637 gate at `635.94/650.91/645.47 s`
(median `645.47 s`). R4E-H then completed the grouped storage-budget handoff
surface for R4B `subsurface_loss_m` / `Qd`, aggregate
`evapotranspiration_m`, and signed `snow_coupling_m`, with direct inputs,
handoff compute, state mutation, downstream operands, and shadow projection for
each producer. R4B now requires those producers before storage reconciliation.
It preserved the same boundary and passed the default-disabled H2637 gate at
`648.48/652.43/642.26 s` (median `648.48 s`). R4I-L then completed the
grouped runoff-path input handoff surface for R4A `liquid_input_m`,
`runon_input_m`, `cumulative_infiltration_m`,
`depression_storage_delta_m`, and `surface_saturation_runoff_m`, with direct
inputs, handoff compute, state mutation, downstream operands, and shadow
projection for each producer. R4A now requires those producers before runoff
partition. It preserved the same boundary, split runoff direct-runtime code
into `direct_runtime/runoff.rs`, and passed the default-disabled H2637 gate at
`646.47/642.52/640.20 s` (median `642.52 s`).
R4M/O then promoted the subsurface handoff surface into request-free direct
WB18/WB19 compute from typed layer vectors. R4M now computes direct `D`, `Pe`,
and per-layer percolation fluxes; R4O now computes direct lateral `q`, tile
drainage `Qdd`, final `Qd`, carry arrays, diagnostics, and layer withdrawals;
and R4B now requires R4M/R4O shadows before storage reconciliation. It
preserved no-publication/no-default/no-scheduler boundaries and passed the
default-disabled H2637 gate at `643.70/646.33/639.62 s` (median `643.70 s`).
R4N then promoted the aggregate ET handoff into request-free direct WB17
evapotranspiration and post-WB19 root-uptake compute. R4N now computes
surface/residue ET, soil-evaporation layer mutation, SWU/root-uptake vectors,
water stress, and final aggregate ET; R4O consumes the R4N ET-mutated layer
vector when present, and R4B requires final R4N ET before storage
reconciliation. It preserved the same no-publication/no-default/no-scheduler
boundary and passed the default-disabled H2637 gate at
`643.84/650.42/649.22 s` (median `649.22 s`).
R4P/Q/Z then closed R4 by adding a shadow-only direct hydrology projection span.
It requires R4A/R4B/R4G/R4J/R4M/R4O/R4N upstream shadows, recomputes aggregate
storage from the final R4N layer vector, separates frozen storage, assembles
typed direct hydrology projection operands, and preserves
`public_output_cutover = false`. It preserved no-publication/no-default/
no-scheduler boundaries and passed the default-disabled H2637 gate at
`645.54/644.74/640.28 s` (median `644.74 s`).
R5A-D then completed full-day direct executor lifecycle, direct normalization
and storage-bounds phases, direct decomposition/residue transitions, and direct
annual/perennial growth transitions. R5E closed R5 endpoint readiness by
proving exactly one canonical 14-phase entry per OFE-day, preserving
sub-operation accounting for folded R4/R5 spans, and passing H2637
default-disabled timing at `641.37/642.02/635.47 s` (median `641.37 s`) plus
opt-in direct-skeleton endpoint evidence at `638.33 s`. Protected output
comparison passed with HBP/WAT/loss/plot byte identity and PASS DuckDB row
equivalence.
R6 direct publication cutover is complete at
[20260621-r6-direct-publication-cutover-001](work-packages/20260621-r6-direct-publication-cutover-001/package.md),
and its initial `HOLD-R6-R5E-PREREQUISITE` state is cleared by R5E pushed
commit `d8f6bbea`. Resumed R6 promoted the PERFDEEP06 publication operand
ledger into canonical architecture authority and originally held with
`HOLD-R6-DIRECT-PUBLICATION-FRAME-ABSENT`.
R6A executed at
[20260621-r6a-run-bound-direct-publication-frame-001](work-packages/20260621-r6a-run-bound-direct-publication-frame-001/package.md)
to build that direct publication frame from typed direct state and add direct
HBP/WAT/PASS/loss/manifest projection consumers before output-family cutover.
Current R6 execution added a guarded `DirectPublicationFrameCutover` candidate,
but it fails closed at `R6-DIRECT-PUBLICATION-PARITY` because HBP byte identity
differs (`1654` direct bytes vs `1654` compatibility bytes) and the production
manifest writer still uses compatibility provenance. R6B executed-held at
[20260621-r6b-direct-publication-parity-manifest-cutover-001](work-packages/20260621-r6b-direct-publication-parity-manifest-cutover-001/package.md)
with `HOLD-R6B-DIRECT-PUBLICATION-TYPED-OPERAND-BRIDGE-ABSENT`: the production
candidate still creates a skeleton direct run frame, seeds only geometry and
calendar metadata, and captures publication rows from zero/default direct
state. The next R6 hold-lift must implement that typed operand bridge before
anti-alias fixtures, independent reconstruction, manifest provenance/checksum
cutover, output-family parity, or benchmarks can close. R6C executed-held at
[20260621-r6c-direct-publication-typed-operand-bridge-001](work-packages/20260621-r6c-direct-publication-typed-operand-bridge-001/package.md)
with `HOLD-R6C-DIRECT-PHASE-PUBLICATION-PRODUCER-ABSENT`: it corrected the
cutover path to fail before skeleton direct-frame construction, proving the
remaining blocker is the absent retained production direct publication producer
surface in the climate lifecycle. R6D executed-held at
[20260621-r6d-production-direct-publication-producer-retention-001](work-packages/20260621-r6d-production-direct-publication-producer-retention-001/package.md)
with `HOLD-R6D-PARITY-GRADE-PUBLICATION-PRODUCERS-ABSENT`: it added a
cutover-only retained `DirectRunPublicationFrame` to the production climate-day
loop, sourced from parsed climate/calendar and slope geometry, and cutover now
consumes that retained frame without skeleton capture. The next blocker is
parity-grade retained producers for hydrology/storage/subsurface/evaporation,
PASS, loss, manifest, and erosion plus anti-alias/reconstruction evidence and a
direct-publication helper split out of the monolithic runner module. R6E
executed-held at
[20260621-r6e-direct-publication-cutover-iterative-defect-closure-001](work-packages/20260621-r6e-direct-publication-cutover-iterative-defect-closure-001/package.md);
it resolved the direct-publication helper split, added typed direct
publication day inputs from parsed climate, changed retained cutover
publication to direct executor capture, and preserved fail-closed no-output
behavior. R6F executed-held at
[20260621-r6f-direct-publication-cutover-blocker-closure-001](work-packages/20260621-r6f-direct-publication-cutover-blocker-closure-001/package.md)
with `HOLD-R6F-WAT-DIRECT-PROCESS-PRODUCER-AUTHORITY-GAP`: it closed the HBP
byte mismatch on the current near-zero runoff fixture, preserved fail-closed
cutover behavior, and reduced WAT to identity/year/profile/ET/storage producer
authority. R6G executed-held at
[20260621-r6g-direct-wat-producer-authority-001](work-packages/20260621-r6g-direct-wat-producer-authority-001/package.md)
with `HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT`: it bound parsed
typed direct WAT producers for the inherited current fixture, fixed residual
storage projection, proved first WAT row parity, and reduced the remaining WAT
delta to day-2 `Es`, `Total-Soil`, and `SoilWaterTotal`. R6H executed-held at
[20260621-r6h-direct-pmet-day-state-carry-builder-001](work-packages/20260621-r6h-direct-pmet-day-state-carry-builder-001/package.md)
with `HOLD-R6H-WAT-PMET-LAYER-CARRY-ULP-PARITY`: it replaced the precomputed
PMET day-input vector with an interleaved direct day/lane builder, preserved
current-fixture HBP byte identity, made WAT storage totals bit-identical, and
reduced the remaining current-fixture WAT residual to day-2 `Es` only. R6I
completed at
[20260621-r6i-direct-pmet-layer-ulp-parity-001](work-packages/20260621-r6i-direct-pmet-layer-ulp-parity-001/package.md)
with `COMPLETE-R6I-DIRECT-PMET-LAYER-ULP-PARITY`: it corrected direct
active-frost fine-layer carry projection, proved current-fixture HBP and WAT
identity, and moved the cutover blocker to manifest writer wiring. R6J
completed at
[20260621-r6j-direct-publication-cutover-blocker-closure-001](work-packages/20260621-r6j-direct-publication-cutover-blocker-closure-001/package.md)
with `COMPLETE-R6-DIRECT-PUBLICATION-CUTOVER`: it wired direct manifest
provenance/checksums/counters, removed compatibility output oracles from the
production cutover writer, closed current-fixture and H2637 HBP/WAT/PASS/loss
parity, stabilized PASS Parquet metadata, and proved fresh same-binary H2637
default/direct byte identity for HBP/WAT/PASS/loss/plot with
`compatibility_edge_invocations=0`.
(Completed-rung detail and commits: [work-packages execution log](work-packages/README.md).)

---

## Watershed Runtime Performance Queue

The watershed runtime performance direction is queued by
[watershed-runtime-architecture-specification.md](architecture/watershed-runtime-architecture-specification.md).
ADR-0032 ratifies the entrypoint, `--jobs` default, and canonical benchmark
mode. WSHED-FIXTURE01 adopted the committed 32-hillslope carnivorous-adobo
development fixture under `tests/fixtures/watershed/carnivorous-adobo/`.
WSHED-W2, WSHED-W3, WSHED-W4, WSHED-W4DC01, WSHED-W5, and WSHED-W6 are
complete. WSHED-W6 adopted the full `1305`-hillslope
`onshore-xenophobia` fixture, cut public publication to the typed frame path,
and recorded strict committed-fixture scaling plus pinned legacy completion
evidence. The post-W6 queue is follow-on hardening and authority completion, not
another watershed runtime rewrite. Physics-bearing work must implement actual
authoritative operands and retain null/fail-closed behavior when authority is
absent.

This section is the authoritative queue for WSHED-family work and
watershed-facing Lane D consumer work. With WSHED-W7R closed on 2026-07-08,
current binding order: openWEPP management YAML canonical authorization ->
landuse migration CLI/spec implementation -> completed Lane D single-OFE/MOFE
groundwater/baseflow implementation and export closure -> completed WSHED-W8
channel-balance operand authority -> completed WSHED-W9 canonical no-event pass
semantics -> completed WSHED-W10 `chan.inp` absent-default authority -> Lane D
watershed HBP hourly water/sediment consumption -> queued WSHED-W11
channel-network hourly water/sediment routing -> remaining WSHED hardening.
M-T2P closed as a projection-authority hold, and M-T2Q locked
`ow-lanuse-1` or later native management semantics as the canonical new-physics
production datver, so this queue keeps pre-native legacy datvers on protected
legacy/off paths unless they are explicitly migrated into canonical native YAML.

| # | Item | Mechanism | Acceptance target | State |
|---|---|---|---|---|
| **WSHED-W7** | **Sediment-active watershed fixture and publication closure** | Adopt or generate a committed full watershed fixture with actual nonzero sediment response from hillslope pass artifacts, then run the full watershed supervisor through typed dispatch and typed publication. No subsets, no synthetic sediment fills, no surrogate physics. | Full committed fixture evidence; `--jobs 1` vs parallel parquet schema/row identity across all required outputs; nonzero detachment/deposition/sediment-yield rows; independent reconstruction of sediment/publication operands where conservation-sensitive; source guards proving public publication remains on the typed frame path. | ✅ **resumed/complete 2026-07-08 via WSHED-W7R** — package: `work-packages/20260702-wshedw7-sediment-active-watershed-fixture-publication-closure-001/`. W7R adopted `tests/fixtures/watershed/p102-sediment-active/`, proved current-main nonzero p102 HBP production, release `--jobs 1`/`--jobs 4` decoded parquet identity, nonzero public `totalwatsed3` sediment, and HBP/public reconstruction. |
| **WSHED-W7DC01** | **Hillslope sediment production hold lift** | Historical hold-lift for `WSHED-W7-HOLD-001`: diagnose/correct all-zero production HBP sediment on real multi-OFE source substrates. | Historical evidence retained; do not execute as written unless a fresh producer-side zero-sediment regression reappears. | **superseded / historical** — package: `work-packages/20260702-wshedw7dc01-hillslope-sediment-production-hold-lift-001/`. The diagnosed producer-side cause was the multi-OFE EROD14/Wave-2 path. Later erosion E.3/E.4 chained Wave-1 per OFE, produced real multi-OFE sediment on a W7DC01 substrate, and deleted EROD14/Wave-2. WSHED-W7R verified the producer on current main and closed W7 fixture/publication adoption. |
| **WSHED-W7R** | **Resume W7 sediment-active watershed fixture/publication closure** | Resume W7 on current main after active Tier 1 local numerics closes. First prove the W7DC01 producer-side blocker is gone on current binaries, then adopt/generate the sediment-active watershed fixture and run the full typed watershed supervisor/publication path. | Current-main nonzero multi-OFE HBP producer proof (`tdet`/`tdep`/`sedcon_*`); committed sediment-active watershed fixture or generated fixture with provenance; full watershed typed dispatch/publication; `--jobs 1` vs parallel schema/row identity; nonzero sediment public outputs; independent sediment reconstruction; source guards proving publication remains on `WatershedPublicationFrame`. | ✅ **complete 2026-07-08** — accepted fixture `tests/fixtures/watershed/p102-sediment-active/`; focused guard `wshedw7r_p102_sediment_active_fixture_publishes_nonzero_sediment_and_jobs_identity`; release evidence in W7 package artifacts. |
| **M-T2A** | **Srivastava groundwater/baseflow authority** | Contract-first authority package for WEPP groundwater/baseflow before any new implementation. Establish Srivastava (2013) as the primary process authority, with Srivastava et al. (2013/2017) and Dun et al. (2009) as companion lineage, and define the terminology boundary between lateral subsurface export (`latqcc`), groundwater-reservoir baseflow, deep seepage, and `chan.inp` `cbase`. | New or amended science contract for groundwater/baseflow process behavior; equations, symbols, units, state timing, coefficient provenance, and runtime ownership boundaries; explicit linkage to `SC-INFILE-GWCOEFF-001`; single-OFE and Lane D MOFE obligations; fail-closed behavior for missing/mixed authority; no production physics code changes. | ✅ **complete 2026-07-08** — created `SC-GWBASEFLOW-001`; package: [work-packages/20260708-groundwater-baseflow-srivastava-authority-001/package.md](work-packages/20260708-groundwater-baseflow-srivastava-authority-001/package.md). |
| **M-T2P** | **Canonical hourly Lane D routing-coefficient projection authority** | Contract-first authority package for making hourly water balance plus Lane D active routing the canonical production path for single-OFE and MOFE by expanding routing-coefficient authority beyond explicit native `routing_coefficients` where defensible. Legacy cropland projection may be accepted only if source-mapped, bounded, deterministic, and fidelity-adequate; non-hourly/DC01-only/non-Lane-D paths remain validation/reference/rollback surfaces, not new production consumers. | Current-authority audit; legacy cropland source audit for `k_o`, `form_C_d`, `D_r_m`, `lambda`, and `vegetation_C_d`; projection rule or hold; canonical-path policy; predeclared fidelity envelope; contract/spec disposition; implementation handoff. | executed-hold 2026-07-08 — `SC-OFEROUTE-001` rev 48 rejects implicit legacy-cropland projection from row/rill/random-roughness/cover/friction/erosion diagnostic fields. Complete explicit `routing_coefficients` or another ratified explicit producer remains required for Lane D active/default coefficient authority; package: [work-packages/20260708-laned-router-canonical-hourly-laned-routing-coeff-projection-authority-001/package.md](work-packages/20260708-laned-router-canonical-hourly-laned-routing-coeff-projection-authority-001/package.md). |
| **M-T2Q** | **`ow-lanuse-1` canonical production datver authority** | Contract-first lock-in of the M-T2P follow-on decision: do not project coefficients and do not add optional sidecar authority. `ow-lanuse-1` or later native management files with embedded `routing_coefficients` are the canonical new-physics production surface; pre-native legacy datvers remain deprecated compatibility/validation/rollback inputs on legacy/off paths. | `SC-OFEROUTE-001` rev 49; plant-file spec and management-lanuse interface contract updated; implementation gap and worker handoff for runtime eligibility guards plus WEPPpy producer materialization. | ✅ **complete 2026-07-08** — package: [work-packages/20260708-laned-router-ow-lanuse-canonical-production-datver-authority-001/package.md](work-packages/20260708-laned-router-ow-lanuse-canonical-production-datver-authority-001/package.md). |
| **M-T2S** | **openWEPP management YAML canonical authorization** | Authorize typed producer-side openWEPP management YAML as the canonical native format for `ow-lanuse-1+`. Freeze flat `.man` as an ingest-only source/compatibility reader; do not require a native flat writer; prove the real runtime consumer reads YAML directly. | Draft/public YAML spec; `infile-management-yaml` registry row; `SC-INFILE-MANAGEMENT-YAML-001` or explicit contract amendment; recommended publishable `openwepp-management-schema` crate; producer lowercase `.yaml` and informal `.man.yaml` naming; consumer `.yaml`/`.YAML`/`.yml`/`.YML`; runtime YAML intake; PL route-coefficient projection from YAML; negative proof that `.man`, reports, and sidecars do not carry the production claim. | ✅ **complete 2026-07-08** — added `SC-INFILE-MANAGEMENT-YAML-001`, amended `SC-OFEROUTE-001` rev 50 and management-lanuse authority, added publishable `crates/openwepp-management-schema`, wired YAML dispatch through `parse_management_document_from_path`, and proved PL route-coefficient projection from YAML. Package: [work-packages/20260708-openwepp-management-yaml-canonical-authorization-001/package.md](work-packages/20260708-openwepp-management-yaml-canonical-authorization-001/package.md). |
| **M-T2R** | **Landuse migration CLI specification and implementation** | Implement a Rust library/CLI for crates.io distribution that migrates frozen legacy management flat files and flat `ow-lanuse-1` sources to canonical typed management YAML. Initial target: legacy cropland to coefficient-complete `ow-lanuse-1` YAML requiring disturbed-class authority, plus flat-native-to-YAML and native-YAML-to-latest migrator scaffolding. | Draft/public CLI spec; new `openwepp-landuse-migrate` crate and binary; `--args-for-migration-to` discovery; `--validate`; default `.man.yaml` naming for flat `.man` inputs; lowercase `.yaml` producer output; embedded/versioned Disturbed route-coefficient table; fail-closed no-sidecar/no-compat-only behavior; YAML schema validation and runtime-consumer proof; crates.io readiness. | ✅ **complete 2026-07-09 UTC** — added publishable `crates/openwepp-landuse-migrate`, embedded the Disturbed route-coefficient table, implemented discovery/validation/migration/dry-run/report behavior, proved migrated YAML is consumed by the real PL runtime projection path, and finalized `SPEC-TOOL-LANDUSE-MIGRATION-CLI-001`. Package: [work-packages/20260708-landuse-migration-cli-spec-implementation-001/package.md](work-packages/20260708-landuse-migration-cli-spec-implementation-001/package.md). |
| **M-T2B** | **Groundwater/baseflow for Lane D single-OFE and MOFE implementation** | Implement the contract-authorized groundwater/baseflow process on the canonical Lane D path, with `lane_count = 1` as the production single-OFE case and `lane_count > 1` as MOFE. Consume `gwcoeff` coefficients, update daily groundwater storage from deep-percolation recharge, compute generated groundwater baseflow and groundwater-reservoir deep seepage, and integrate Lane D aggregation/export without feeding generated baseflow or reservoir deep seepage into active surface routing. | Contract-derived tests; Lane D single-OFE daily reservoir vector; Lane D MOFE recharge/storage/baseflow/deep-seepage accounting vector; H2637-class subsurface-dominated closure vector; protected coefficient-absent/default behavior; malformed/mixed groundwater authority fail-closed; HBP/watershed handoff fields explicitly populated or intentionally null by contract. | ✅ **complete 2026-07-09 UTC via M-T2** — initial package implemented `gwcoeff` parser handoff, direct groundwater carry/recurrence, Lane D MOFE recharge aggregation, WAT `Base` publication, unit registry metadata, and real watershed WAT `Base` consumer proof. M-T2 lifted the generated `gwdsv` consumer and `bftharea` watershed/channel threshold holds. Package: [work-packages/20260708-groundwater-baseflow-laned-single-ofe-mofe-implementation-001/package.md](work-packages/20260708-groundwater-baseflow-laned-single-ofe-mofe-implementation-001/package.md); closure: [work-packages/20260709-laned-active-baseflow-export-closure-001/package.md](work-packages/20260709-laned-active-baseflow-export-closure-001/package.md). |
| **M-T2** | **Lane D active baseflow export closure** | Close the `INV-OFEROUTE-012` baseflow leg before watershed-facing HBP consumption: prove and, if needed, implement conservation/export of non-exfiltrated subsurface outflow (`latqcc`/baseflow bypass) when active routing owns the surface-water path. This is separate from the already-landed `ui_SCrunf` exfiltration source term and must reconcile any stale over-strong active-owner wording before downstream consumer claims. | Contract-first authority/current-state audit; active-mode operand-lineage table for exfiltration, routed surface outflow, `latqcc`/baseflow export, and full hillslope water-balance terms; H2637-class subsurface-dominated closure vector; fixture separating exfiltration source from bypassed baseflow; typed hard-fail on missing or double-counted baseflow; protected off/default behavior unchanged. | ✅ **complete 2026-07-09 UTC** — amended HBP/baseflow contracts, serialized and parsed HBP `gwbfv`/`gwdsv`, threaded those fields into watershed contributions, made `lr_bf=1` consume generated HBP baseflow instead of `cbase`, implemented `bftharea` threshold handling, and added real consumer-path tests. Package: [work-packages/20260709-laned-active-baseflow-export-closure-001/package.md](work-packages/20260709-laned-active-baseflow-export-closure-001/package.md). |
| **WSHED-W8** | **Channel-balance operand authority** | Contract-first implementation of real channel-balance publication operands (`Inflow`, `Outflow`, `Storage`, `Baseflow`, `Loss`, `Balance`) from authoritative routed channel state. Existing nulls remain correct until the operands are actually available. | Science-contract or architecture authority for each volume operand; typed routing/publication state carries the operands directly; `chanwb` closure evidence uses independent operand reconstruction; tests reject aliasing inflow as outflow or otherwise filling unavailable fields. | ✅ **complete 2026-07-09 UTC** — added `SC-SYSTEM-001` rev 86 / `INV-SYSTEM-033`, typed routed channel-balance operands, publication-frame projection, storage-aware `chanwb` balance reconstruction, non-aliased runtime/writer tests, and full workspace closure gates. Package: [work-packages/20260709-wshedw8-channel-balance-operand-authority-001/package.md](work-packages/20260709-wshedw8-channel-balance-operand-authority-001/package.md). |
| **WSHED-W9** | **Canonical HBP no-event pass semantics** | Make latest-day HBP `NO_EVENT`/`SUBEVENT` state explicit at parser and watershed intake boundaries, without stale prior-event reuse or synthesized optional runoff payloads. | Typed latest-event state; zero surface runoff/sediment only as a validated no-event consequence; preserved baseflow/deep-seepage; malformed/missing/stale state fails closed. | ✅ **complete 2026-07-09 UTC** — amended `SC-INFILE-HBP-001` v0.2.3 and `SC-SYSTEM-001` rev 87, implemented typed latest-event state and watershed intake/guard behavior, and passed focused/full closure gates. Package: [work-packages/20260709-wshedw9-canonical-noevent-pass-semantics-001/package.md](work-packages/20260709-wshedw9-canonical-noevent-pass-semantics-001/package.md). |
| **WSHED-W10** | **`chan.inp` absence/default authority** | Adjudicate the current deterministic `chan.inp`-absent defaults and either ratify them as explicit runfile/contract defaults or replace them with fail-closed required input. | Documented authority for `dtchr`, `ntchr`, `nchnum`, and `cbase` behavior; CLI tests prove warning/default or required-input behavior intentionally; no hidden modeling defaults remain. | ✅ **complete 2026-07-09 UTC** — amended `SC-INFILE-CHANINP-001` v0.1.2 and `SC-SYSTEM-001` rev 88, updated the end-user `chaninp` spec, made the parser default branch legacy no-output (`ichout=0`) with deterministic `dtchr=60`/`ntchr=1440`, routed unconfigured/missing `chan.inp` through the typed parser default state, and proved parser/frame/CLI behavior. Package: [work-packages/20260709-wshedw10-chaninp-absent-default-authority-001/package.md](work-packages/20260709-wshedw10-chaninp-absent-default-authority-001/package.md). |
| **CQR-NIGHTLY** | **Rolling CQR Nightly ExecPlan Burndowns** | Operator-triggered repo-wide CQR maintenance. A request such as `execute cqr nightly for 8 modules` measures the live workspace CRAP/LCOV surface, selects the top eligible production modules by current CRAP burden, scaffolds one behavior-preserving CQR work package per module, commits each scaffold, executes each package, and commits either completion or hold evidence before continuing. This replaces the narrower WSHED-CQR queue item. | Standing ExecPlan and templates installed at `work-packages/cqr-nightly-burndown-execplan.md`, `work-packages/templates/cqr-nightly-package.md`, and `work-packages/templates/cqr-nightly-kickoff-prompt.md`; target selection excludes active dirty/science work unless explicitly authorized; each package records before/after CRAP, coverage/ADR-0021 closure when tests change, focused characterization, behavior identity, line-count governance, gates, review, verification, and disposition. Local target holds roll back implementation edits, commit hold evidence, and continue; global/process holds stop the batch. | queued; invoke by operator request after active feature/science packages release overlapping write sets. |
| **CQR-FOLLOWUP-20260711** | **Burn down nightly batch-01 follow-up queue** | Execute four serialized single-module tracks created by the 2026-07-11 nightly holds: fixed-date finite-value defect closure, CHAN.INP raw-cardinality defect closure, watershed-channel extra-rating typed-error closure, and totalwatsed3 science-tier cover-then-decompose closure. Preserve the PMETPARA formatter hold as reviewed no-action. | Follow `work-packages/cqr-nightly-followup-burndown-execplan.md`; each parser DC closes contract authority, tests, production correction, science-tier coverage/obligations, and CRAP in one non-deferring package; totalwatsed3 independently reconstructs conservation-sensitive output; every package has scaffold/terminal commits, dual review/verification, and full gates; final fresh workspace rerank recorded. | ✅ **complete 2026-07-11** — FQ-01 through FQ-04 completed with terminal commits and dual verification; PMETPARA remains reviewed no-action; the clean-commit final rerank retained science-tier coverage and zero target rows above CRAP 30. ExecPlan: [work-packages/cqr-nightly-followup-burndown-execplan.md](work-packages/cqr-nightly-followup-burndown-execplan.md). |
| **CQR-PREINT-20260711** | **Comprehensive pre-integration CQR campaign** | Burn down the clean-commit census of 45 production modules with raw CRAP above 30 before broader integrated testing: two serialized high-risk tranches of 10 modules, one medium-risk tranche of 13, then classification/disposition and eligible cleanup for 12 provisional low-priority modules. Risk priority never substitutes for exact ADR-0021 symbol eligibility. | Follow the [campaign assessment](work-packages/cqr-pre-integration-campaign-assessment.md), [durable 67-row baseline](work-packages/cqr-pre-integration-campaign-baseline.md), [binding execution contract](work-packages/cqr-pre-integration-campaign-execution-contract.md), [authoring review](work-packages/cqr-pre-integration-campaign-authoring-review.md), [High A](work-packages/cqr-high-risk-a-execplan.md), [High B](work-packages/cqr-high-risk-b-execplan.md), [Medium](work-packages/cqr-medium-risk-execplan.md), and [Low/Assessment](work-packages/cqr-low-priority-assessment-execplan.md) plans. One child plan and one module package are active at a time; every eligible row closes at CRAP at most 30 or enters authority-bounded defect closure; proposed no-action dispositions require two independent reviews. Final fresh raw/actionable rerank and full gates produce exactly `GO-INTEGRATED-VALIDATION` or `HOLD-CQR-FOLLOWUP`; HOLD installs a finite authority/write-set follow-up queue here. | **GO — AUTHORING READY 2026-07-11** — three independent reviewers passed the remediated plan; baseline 45 module paths / 67 deduplicated raw rows, provisionally 20 high / 13 medium / 12 low. High A is `QUEUED-READY`; activate it from a clean source-bound worktree in a separate dispatch commit before execution. |
| **M** | **MOFEFID — MOFE fidelity campaign** | Four lanes on the closed MOFE routing foundation: (A) proactive MOFE defect-review sweep; (B) adjudication-first audit against the eleven defect classes in the wepp-forest stakeholder water-balance brief; (C) lateral-flow observed-authority rubric from `tests/fixtures/forest_lateral_flow_authority/`; (D) Papanicolaou 2018 OFE-by-OFE routing (space/time-variant resistance, TVD-MacCormack KWE). A-C are complete; Lane D is merged through D1-D8, subsurface seam design, solver/identity seam implementation, opt-in runtime SHADOW + real-H2637 executed vector, D9-D13 disposition/authority work, the rev-21 opt-in-shadow friction operand closure, rev-22 melt-limb source-shape closure, rev-23/rev-53 routed-hydrograph erosion-shape closure, D14 runtime profiling/optimization, D15 opt-in activation, D16 selected-cohort materialization/active-suite blockers, ADR-0037 hybrid abandonment/removal, the Tier-2 target-`dx` mesh-policy rescope, the rev-41 WA active-router positivity-preserving solver correction, the rev-41 Tier-2 re-adjudication, the `dx0p625` fine-reference hold-lift, the `mn_corn_h4` day-792 shape attribution hold, the day-792 raw-hydrograph CFL timestep-transition hold, the active-router timestep-policy adjudication (`SC-OFEROUTE-001` rev 43), the coupled space-time `dx5` ratification hold, the WA annual pass-sediment fine-reference attribution hold, annual pass-sediment metric authority (`SC-OFEROUTE-001` rev 44), active `dx5` production mesh-policy promotion (`SC-OFEROUTE-001` rev 45), conditional default activation (`SC-OFEROUTE-001` rev 46), Tier 1 local numerics (`SC-OFEROUTE-001` rev 47), route-coefficient projection hold (`SC-OFEROUTE-001` rev 48), and canonical `ow-lanuse-1` production datver authority (`SC-OFEROUTE-001` rev 49). | Strategy: [planning/mofe-fidelity-campaign-strategy.md](planning/mofe-fidelity-campaign-strategy.md). D15A ([work-packages/20260706-mofefid-d15-active-owner-optimization-001/package.md](work-packages/20260706-mofefid-d15-active-owner-optimization-001/package.md), executed 2026-07-06) landed the opt-in ACTIVE OWNER at `SC-OFEROUTE-001` rev 27. ADR-0037 ([decisions/0037-abandon-hybrid-implicit-stepping.md](decisions/0037-abandon-hybrid-implicit-stepping.md)) abandoned the hybrid implicit stepper and retained plain Lane D active routing as the surviving production path. The coupled `dx5` ratification package ([work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001/package.md](work-packages/20260708-laned-router-tier2-dx5-coupled-spacetime-ratification-001/package.md)) held before production promotion: `dx5` candidate comparisons pass, but the fine-reference basis remained open on `mn_corn_h4` shape and WA refined-75 annual sediment. The WA attribution package ([work-packages/20260708-laned-router-wa-sediment-reference-adequacy-attribution-001/package.md](work-packages/20260708-laned-router-wa-sediment-reference-adequacy-attribution-001/package.md)) localized the annual sediment miss to one low-mass erosion day (`1126`) and classified it as annual sediment response to a sub-threshold routed-hydrograph shape perturbation, not active-router numerics or daily water-magnitude drift. The annual metric authority package ([work-packages/20260708-laned-router-annual-sediment-adequacy-metric-authority-001/package.md](work-packages/20260708-laned-router-annual-sediment-adequacy-metric-authority-001/package.md)) amended `SC-OFEROUTE-001` rev 44 and replayed 21 selected-cohort annual sediment comparisons with zero rev-44 blockers. The production mesh-policy package ([work-packages/20260708-laned-router-dx5-production-mesh-policy-ratification-001/package.md](work-packages/20260708-laned-router-dx5-production-mesh-policy-ratification-001/package.md)) amended rev 45 and promoted active production default to `target_dx_m = 5.0` with selected-cohort no-env/default runtime proof and protected off-path identity. The conditional default activation package ([work-packages/20260708-laned-router-conditional-default-activation-001/package.md](work-packages/20260708-laned-router-conditional-default-activation-001/package.md)) amended rev 46: coefficient-complete scheduled lanes route active by default, no-coefficient runs stay legacy/off, and mixed authority fails closed. Tier 1 local numerics ([work-packages/20260708-laned-router-tier1-local-numerics-001/package.md](work-packages/20260708-laned-router-tier1-local-numerics-001/package.md)) amended rev 47 and met the H2637 timing target while holding only the unratified Hirsch `Re^0.45` approximation. M-T2P ([work-packages/20260708-laned-router-canonical-hourly-laned-routing-coeff-projection-authority-001/package.md](work-packages/20260708-laned-router-canonical-hourly-laned-routing-coeff-projection-authority-001/package.md)) amended rev 48 and rejected implicit legacy-cropland route-coefficient projection. M-T2Q ([work-packages/20260708-laned-router-ow-lanuse-canonical-production-datver-authority-001/package.md](work-packages/20260708-laned-router-ow-lanuse-canonical-production-datver-authority-001/package.md)) amended rev 49 and locked `ow-lanuse-1` or later native management files with embedded `routing_coefficients` as the canonical new-physics production surface. M-T2B ([work-packages/20260708-groundwater-baseflow-laned-single-ofe-mofe-implementation-001/package.md](work-packages/20260708-groundwater-baseflow-laned-single-ofe-mofe-implementation-001/package.md)) and M-T2 ([work-packages/20260709-laned-active-baseflow-export-closure-001/package.md](work-packages/20260709-laned-active-baseflow-export-closure-001/package.md)) closed the Lane D groundwater/baseflow implementation/export chain on 2026-07-09 UTC. WSHED-W7R ([work-packages/20260702-wshedw7-sediment-active-watershed-fixture-publication-closure-001/package.md](work-packages/20260702-wshedw7-sediment-active-watershed-fixture-publication-closure-001/package.md)) closed the sediment-active watershed fixture/publication gap on 2026-07-08. | ✅ **active `dx5` production mesh default, conditional Lane D default activation, Tier 1 local numerics, route-coefficient projection hold, canonical native datver authority, WSHED-W7R, and the Lane D groundwater/baseflow export chain complete/held as applicable; M-T3 complete** — execute WSHED-W11 channel-network hourly water/sediment routing. |
| **M-T1** | **Lane D Tier 1 local numerics** | Execute the backlog Tier 1 active-router local numerics package: analytic celerity, bounded Newton alpha solve, hot-path `h * h.sqrt()` for `h^1.5`, and bounded `Re^0.45` optimization only if approximation evidence supports it. This is intentionally not bit-identity work. | `SC-OFEROUTE-001` amended before code; contract-derived tests; D10B/Iwagaki oracle ladder within rev-25/rev-26 tolerances; exact booked-ledger conservation; TV(q) transient bound; 19-OFE class-fixture conservation sweep; H2637 active endpoint and rev-27 day closure; named fidelity deltas; D14/D15A persistent-slot plus `perf` timing evidence. | executed-hold 2026-07-08 — package: [work-packages/20260708-laned-router-tier1-local-numerics-001/package.md](work-packages/20260708-laned-router-tier1-local-numerics-001/package.md). Landed rev-47 analytic local numerics and met the H2637 timing target (`3.15x`); hold is limited to the unratified Hirsch `Re^0.45` approximation envelope. |
| **M-T3** | **Lane D watershed HBP hourly water/sediment consumption** | After M-T1, WSHED-W7R, the groundwater/baseflow export chain, and WSHED-W8/W9/W10 complete, close the watershed-facing consumer path for active Lane D HBP minor-1 hourly runoff and sediment surfaces. This is broader than HBP outlet re-pointing: the production watershed/channel consumer must read hourly water and sediment timing, not only validate or inventory it. | Contract-first authority if needed; active HBP producer proof including the post-baseflow active water ledger; watershed supervisor intake proof; `WatershedNetworkFrame` handoff proof; production channel/watershed consumer proof; equal-daily-total/different-hourly-distribution sensitivity evidence; active missing/malformed/mixed hourly authority fail-closed; protected legacy/off fallback evidence. | ✅ **complete 2026-07-09 UTC** — amended `SC-ROUTE-001` rev 49/50, implemented all-hourly/no-hourly WS10 inlet authority, added production CLI HBP hourly consumer proof, and passed final workspace gates. Package: [work-packages/20260708-laned-router-watershed-hbp-hourly-water-sediment-consumption-001/package.md](work-packages/20260708-laned-router-watershed-hbp-hourly-water-sediment-consumption-001/package.md). |
| **WSHED-W11** | **Channel-network hourly water/sediment routing** | Close M-T3's explicit leaf-channel scope limit by adding contract-authorized typed channel-hourly water and particle-class sediment state, stateful channel routing, and downstream dependency consumption. Preserve no-hourly behavior and fail closed at unsupported branches/topologies; do not reconstruct channel hourly output from scalar summaries. | Canonical time-grid/projection and particle-class authority; typed routed channel hourly outputs; at least two dependency-ordered channels consuming HBP `V_h`/`S_h`; equal-daily-total/different-shape downstream sensitivity; per-channel/network water and sediment closure; production CLI consumer proof; protected minor-0 identity; full review/verification/gates. | **executed hold 2026-07-10; hold lifted and successor completed same day** — Phase A remains the source/hold record. WSHED-W11A ratified `SC-ROUTE-001` v53 authority; WSHED-W11B completed implementation, `GAP-ROUTE-014` correction, real-consumer/conservation proof, dual verification, and full gates. Packages: [held W11](work-packages/20260710-wshedw11-channel-network-hourly-water-sediment-routing-001/package.md), [completed W11B](work-packages/20260710-wshedw11b-channel-interval-sediment-implementation-001/package.md). |
| **WSHED-W11A** | **Channel-hourly sediment sequencing authority** | Lift `WSHED-W11-HOLD-001` by acquiring and ratifying, or explicitly rejecting, canonical authority for time-resolved WS18-WS26 channel sediment routing coupled to routed channel water series. | Temporal quantum; water/sediment coupling; channel geometry/profile/bed state carry; per-class ingress/egress and detachment/deposition/storage closure; guards/tolerances/test vectors; dual review/verification; exact W11 implementation handoff. | ✅ **complete 2026-07-10 UTC (`EXECUTED-COMPLETE-AUTHORITY`; Codex RATIFIED)** — amended `SC-ROUTE-001` v51-v53: per-`dtchr`-interval quasi-steady sequencing (`INV-ROUTE-015..020`, biconditional lane activation, unique hydraulic-profile operand map with the distinct `qlat(it)` wave-total / derived `qlat_eff(it)` per-unit-length symbols, monotonic geometry carry, pinned-`dcap.for` widening realization with `t_exp`/`t_norm` operands, per-interval/day class mass closure, eleven contract-derived vectors) on convergent WEPP/CREAMS + KINEROS ARS-77 + HEC-RAS quasi-unsteady authority (references vendored); dual review (21 findings) + dual verification passed; two Codex post-hoc REOPEN cycles dispositioned same day (v52: five closure amendments incl. `GAP-ROUTE-014` — the two migrated-lane terminals W11 must correct before interval reuse; v53: the total-vs-per-length lateral operand fix); cycle-2 re-confirmation RATIFIED — `WSHED-W11-HOLD-001` stands lifted, W11 resumes at Phase B. Package: [work-packages/20260710-wshedw11a-channel-hourly-sediment-authority-001/package.md](work-packages/20260710-wshedw11a-channel-hourly-sediment-authority-001/package.md). |
| **WSHED-W11B** | **Channel-interval sediment implementation and defect closure** | Implement the ratified W11A interval lane on the real production network and close `GAP-ROUTE-014` without reopening settled science authority. | All eleven W11A vectors before production edits; capped-widening and post-contact/subcritical terminal corrections; typed interval water/class state; distinct total `qlat` and per-length `qlat_eff`; two dependency-ordered channels; real release CLI consumer proof; independent per-class/channel/network closure; protected non-activated identity; dual review/verification and full gates. | **executed complete 2026-07-10** — release consumer, independent closure, dual verification, full 1,677-test gate, erosion, deny, clippy, and comparator passed. Package: [work-packages/20260710-wshedw11b-channel-interval-sediment-implementation-001/package.md](work-packages/20260710-wshedw11b-channel-interval-sediment-implementation-001/package.md). |

---

## Queue

| # | Item | Mechanism | Acceptance target | State |
|---|---|---|---|---|
| 1 | **Per-OFE runoff magnitude adjudication** | Decide if per-OFE runoff vs legacy (FARPOINT01: openWEPP 71% vs legacy 55.5% of precip on H2637) is expected Stage-2 divergence or a defect | A per-term verdict (expected vs defect-shaped follow-on) | ✅ **`MOFE-MAGPARITY01` complete 2026-06-18** — no `INV-RUNOFFPART-028`, area-scaling, closure, or export defect; 71% `runvol` decomposes to routed lateral/subsurface magnitude. Follow-on is Stage-2 `latqcc`/WB19 magnitude, not a fix. |
| 2 | **Monolith line-count split** | Behavior-preserving split by domain responsibility of the WARN-band files. Historical REFACTOR022 measurement on 2026-06-18 found **10 files >2000, 0 over 3000**, but PERFDEEP06 remeasured current `main` and found `scheduler.rs` at 3177 lines; packages touching it need a fresh exception/sunset plan or split. | Target tier (4 files >2500) under 2000 WARN; bit-identical outputs | ✅ **`REFACTOR022` complete 2026-06-18** — the four target-tier files were split under 2000 lines, true pre-refactor HEAD identity passed with `anchor_mismatches = 0`, and the 2000-2500 tier remains deferred advisory WARN work. Current PERFDEEP07 planning must carry the `scheduler.rs` >3000 disposition. |
| 3 | **Stage-2 physics-magnitude** | Fidelity of deferred magnitudes vs external authority | Magnitude correctness, judged against the closed + routed balance with comparator as flag | ✅ **`STAGE2-LATQCC-H2637-MAGNITUDE` complete 2026-06-18** — WB19 `latqcc` equation and operand-bound checks passed on selected H2637 high-magnitude rows; no openWEPP defect or defect-closure handoff. Verdict: `CONTRACT-GAP`; closing it routes to item 4 (reference-implementation-intent authority), not an external benchmark. |
| 4 | **Reference-implementation-intent authority + `ksatadj`/SC-SUBHYD-001** | Establish **ADR-0024** that for empirical forest models with no external physical authority, the legacy reference-implementation **intent** (algorithm) is a valid `SC-*` A0 anchor — **distinct from** legacy binary *behavior* (A6 flag, ADR-0017) — then apply it: extract the `ksatadj` intent from `wepp-forest_260430_baseline/src/{infpar,input}.for`, anchor it in `SC-SUBHYD-001`, and re-adjudicate openWEPP vs the *intent* | ADR-0024 ratified; `SC-SUBHYD-001` `ksatadj` anchor + invariant; `CORRECT` (close the FARPOINT01 71% flag) or `OPENWEPP-DEFECTIVE` (defect-closure ExecPlan) | ✅ **complete 2026-06-18** — **ADR-0024 ratified**; `SC-SUBHYD-001` v33 `INV-SUBHYD-032` + `REF-SUBHYD-KSATADJ-INTENT` authored and Claude-reviewed (both sides of the `sat_frac` divergence verified against source). Verdict `OPENWEPP-DEFECTIVE`: openWEPP forms `sat_frac = Σθ/Σul` vs source-intent `avsat/(avpor·avcpm)`. Fix routes to item 5; FARPOINT01 stays open until it lands. |
| 5 | **`REFINTENT001-KSATADJ-SATFRAC` defect closure** | Rebuild the WB14 `ksatadj` operand lineage so `sat_frac` is formed per `SC-SUBHYD-001#INV-SUBHYD-032` source intent: rock-corrected `avpor*avcpm` denominator, total-water + `avsm15` residual numerator, the two `avsat` caps, top-two-tillage weighted averaging, not `sum(theta)/sum(ul)` | `INV-SUBHYD-032` satisfied; non-aliased tests where surrogate differs from intended formula; determinism preserved; re-run H2637 + close the FARPOINT01 71% flag by source-intent conformance | ✅ **complete-with-correction 2026-06-18** (`REFINTENT001-KSATADJ-SATFRAC`) — source-intent `sat_frac` fix landed (correct, gate-clean, non-aliased-tested; valuable for `ksatadj=1` soils). **But Claude review found it byte-inert on H2637** (`ksatadj = 0`; WAT SHA identical pre/post), so it does **not** close FARPOINT01 — flag re-opens. The 71% is base-conductivity-driven → item 6. |
| 6 | **H2637 base lateral/percolation conductivity adjudication** | The H2637 71% lateral magnitude is driven by the **base soil conductivity** (`Ke`/`ssc`, soil-file `ksat` + the 200 mm runtime-layer normalization), **not** `ksatadj` (which is off for H2637). Adjudicate that conductivity lineage under `SC-SUBHYD-001` / `SC-INFILE-SOIL-001`, same intent-vs-behavior discipline | Per-term verdict on the base-conductivity lineage (`CORRECT`/`OPENWEPP-DEFECTIVE`/`CONTRACT-GAP`); resolve or re-route the FARPOINT01 71% flag | ✅ **`STAGE2-BASE-CONDUCTIVITY-H2637-MAGNITUDE` complete 2026-06-18** — base `ksat` is byte-live (`ksat_x0.9` changed WAT/PASS checksums and magnitude outputs). Verdict `OPENWEPP-DEFECTIVE`: vertical `wb18_perc_ssc` split-layer normalization is arithmetic but source intent is inverse-conductivity/harmonic (`117.955408` vs `270.8259 mm/h` on H2637 layer 3). Hourly `wb19_lateral_ssh` remains arithmetic and must be preserved. |
| **P** | **PERF - comprehensive array-native re-architecture (direct runtime)** | Comprehensive array-native migration remains the perf direction, but PERFDEEP03 and PERFDEEP05 falsified the current partial hydrology-island shape as a production endpoint win. PERFDEEP05 removed the PERFDEEP04 full-sync hotspot, while the accumulated plumbing also taxed the default-disabled path (`701.95 s` vs `669.97 s`). PERFDEEP06 converted that into a direct-frame plan with a zero-cost-disabled P0 gate. PERFDEEP07 improved but did not close that gate (`685.85 s` retained vs `<= 676.67 s`); PERFDEEP08 tested disabled diagnostic-hook caching and was slower (`691.93 s`). PERFDEEP09 closed the disabled-path blocker, R2A created the separate direct-runtime skeleton without phase math or publication cutover, R3A proved the first complete direct phase span, R3B proved a richer direct water-ledger span, R3C proved run-level multi-lane transfer/topology propagation, R4A through R4P/Q/Z closed the direct hydrology path through shadow-only projection, R5A through R5E completed full OFE-day direct endpoint readiness while preserving no-publication/no-default/no-scheduler boundaries, and R6A added the missing run-bound direct publication frame plus direct HBP/WAT/PASS/loss/manifest projection consumers. R6 promoted the publication operand ledger into architecture authority and added a fail-closed cutover candidate. R6B proved the candidate still lacked a production typed operand bridge; R6C corrected that failure mode so cutover failed before skeleton capture; R6D added a cutover-only retained production `DirectRunPublicationFrame` in the climate lifecycle; R6E resolved direct-runtime input binding for parsed climate; R6F closed the current-fixture HBP byte blocker; R6G reduced WAT to `HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT`; R6H cleared that blocker by adding an interleaved PMET day/lane input builder and reducing WAT to `HOLD-R6H-WAT-PMET-LAYER-CARRY-ULP-PARITY`; R6I closed the PMET layer-carry ULP blocker by carrying active-frost fine-layer projection through direct lane commit; and R6J completed direct publication cutover, including manifest/direct-counter provenance, direct-only public writes, PASS byte-stable metadata, H2637 default `640.41 s / 227396 KiB`, H2637 direct cutover `637.53 s / 349400 KiB`, HBP/WAT/PASS/loss/plot byte identity, and `compatibility_edge_invocations=0`. | Next: scope post-R6 work separately. Immediate technical debt before more publication growth: split `00_runner_intake_and_lane_setup.rs` (`2997` lines) and `direct_runtime.rs` publication-row construction (`2922` lines). Default activation, broader nonzero erosion authority, and performance tuning require separate packages and gates. | **≤10× MET; direct production is the no-env default.** After R6, the frost validation arc ratified `INV-SNOWFREEZE-047/048/050` and **activated direct production as the no-env default for all supported surfaces** (frost ratification + cutover correction, ADR-0030). H2637 direct = **61.40 s ≈ 6.7× legacy** (within ≤10×). The direct-publication **RSS arc is complete and run-length-flat** (H2637 1.13 GiB → 110 MiB full / 51 MiB required-only, byte-identical; merged `5b139058`). **Remaining: none — item closed.** Compatibility deletion completed (kernel-boundary terminal typing); the **≤5× ideal was met 2026-07-01** at 32.77 s = 3.52× via the hillperf packages (the lever was winter-subsystem overhead — duplicate frost solve + success-path guard formatting — not RSS/cache residency). |
| **W** | **R7 terminal cleanup: compatibility runtime deletion** | The ADR-0026 winter-column sub-solver, frost observed-data ratification/default activation, and the direct cutover correction are complete. ADR-0030 amends the R7 terminal contract: compatibility frost bit-parity is no longer the acceptance target, direct production is the no-env hillslope default, and silent compatibility fallback is a defect. ADR-0031 supersedes the retained-seam clause and authorizes full deletion. | Delete obsolete skeleton/shadow/cutover transition modes, remove the public `--compatibility-runtime` selector, and delete production hot-loop symbol-map machinery under no-regression/static-proof gates. | ✅ **complete 2026-06-30** — `20260630-compatibility-runtime-deletion-001` removed obsolete skeleton/shadow/cutover transition modes; the direct-publication RSS packages made the endpoint run-length-flat (1.13 GiB → 110/51 MiB, byte-identical); `20260630-typed-day-zero-seed-computation-001` cut production seeding to typed `DirectProductionSeedAuthority`; `20260630-compatibility-runtime-full-deletion-001` ratified ADR-0031 and removed the public `--compatibility-runtime` selector; and `20260630-kernel-boundary-terminal-typing-001` deleted the compiled scheduler/day-frame/carrier support surface. Production hillslope execution is now direct-only with `compatibility_edge_invocations=0`. |
| **K** | **Kernel-boundary typing program (symbol-map-free terminal state)** | Eliminate the symbol-map carrier types (`HillslopeKernelRequest`/`KernelWritebackPayload`/`HillslopeWritebackSurface`/`SymbolRegistry`) — the kernel-invocation boundary — by **typing the kernel request/writeback interface**: typed phase context, typed phase result/mutation APIs, typed diagnostic/trace sinks, and typed audit/publication event streams (authority-bearing typed state, never symbol-map wrappers). Migrate consumers first (diagnostics/trace → tests → phase return paths → event streams), **delete `scheduler.rs`/`day_frame.rs`/carriers last**. The binding 8-step sequence is in [array-native spec §8.2](architecture/array-native-runtime-specification.md). | Every live consumer reads typed boundaries; carrier types deleted or retired as obsolete runtime surfaces; H2637 identity; `compatibility_edge_invocations=0`; static source-scan proof; full workspace gates; CQR burndown scopes CRAP-clean. | ✅ **complete 2026-07-01** — runtime deletion completed on 2026-06-30 (`a381702b`/`20260630-kernel-boundary-terminal-typing-001`), deleting the compiled symbol-map scheduler/day-frame runtime, carrier exports, scheduler trace/publication/seed support, and scheduler-only tests. The nine-row [kernel-boundary CQR burndown](work-packages/kernel-boundary-cqr-burndown-execplan.md) then closed on 2026-07-01: all owned row scopes re-measured at `0` production functions above CRAP 30, H2637 stayed byte-identical in every row with `compatibility_edge_invocations=0`, and final evidence is recorded in the row packages and execplan. Obsolete symbol-map-runtime tests stayed retired with justification; retained ratified-contract assertions were restored at typed runtime/parser surfaces. Full-workspace CRAP still reports `266` functions above 30 outside the burndown scopes, left to the broader CQR backlog/dispositions rather than item K. |

Current direct-runtime note: R7D8 through R7F closed the direct publication,
default-candidate, rollback, and typed day-input hot-loop blockers. R7G/R7H
then moved snow/frost into the ADR-0026 winter-column architecture and reached
zero compatibility-edge counters. The later frost validation arc ratified
`INV-SNOWFREEZE-047/048/050`, bounded `GAP-SNOWFREEZE-002`, activated direct
production as the no-env default, and corrected the temporary multi-OFE/Wave-2
and legacy sidecar-discovery compatibility fallback. ADR-0030 compatibility-runtime deletion then removed the obsolete
skeleton/shadow/cutover transition modes (merged `5b139058`), and the
direct-publication RSS arc made the direct endpoint run-length-flat
(1.13 GiB → 110/51 MiB, byte-identical). ADR-0031 then removed the public
`--compatibility-runtime` selector from API/CLI/harness entrypoints.
`20260630-kernel-boundary-terminal-typing-001` then completed the symbol-surface
support-boundary deletion: `scheduler.rs`, `day_frame.rs`, carrier exports,
scheduler trace/publication/seed support, and scheduler-only tests are gone.
This completed the single-authority re-architecture after the day-zero seed
authority had been cut over through a shadow-first typed-seed-carrier package
(build typed carrier → shadow-prove seed identity → cut over all reads at once →
delete), not an incremental burn-down (which held twice). The ≤5× ideal was
subsequently **met** (2026-07-01) by the hillperf pair: the mechanical
winter-overhead sweep (byte-identical, 71.4 → 46.7 s) and the frost
single-solve rewire (rubric-accepted, → 32.77 s = 3.52×); see
`work-packages/20260701-hillperf-mechanical-winter-overhead-001/` and
`20260701-hillperf-frost-single-solve-001/`.

### W. Winter-column snow/frost implementation sequence

This sequence is retained as historical context for the R7G/R7H winter-column
cutover that is now complete. It deliberately stopped patching the old direct
snow/frost retrofit and removed it after typed winter-column authority was
present.

1. **Ordering gate under ADR-0026.** Apply the ratified stateful sub-solver
   exception and complete the source trace for whether frost liquid partition
   needs post-ET state. Record the final API shape before building beyond
   skeleton types: one `advance_winter_column_day(...)` call if the trace allows
   it, or pre/post winter calls over one mutable `DirectWinterColumnState` if
   runoff reconciliation depends on ET-mutated state.
2. **Mechanical containment before growth.** Create the winter-column module
   boundary outside `direct_runtime` phase modules, with `DirectWinterColumnState`,
   `DirectSnowLaneState`, `DirectFrostLaneState`, `DirectWinterDayForcing`, and
   `DirectWinterDayOutcome`. Direct runtime may own the lane state and call the
   producer, but snow/frost solver math and request/symbol adapters must not live
   in `direct_runtime/runoff.rs`, `direct_runtime/storage.rs`, or publication
   day-input plumbing.
3. **Snow lane migration.** Move the R7G typed snow partition into the winter
   column as a distinct snow sub-state/sub-solver. Preserve prior-vs-post snow
   views: frost thermal forcing reads prior SWE/depth/density, while downstream
   liquid forcing and publication read post-partition snow. Cut R4B/R4PQZ/direct
   publication consumers to `DirectWinterDayOutcome`, then delete the old
   `DirectSnowCoupling*` state/downstream/shadow plumbing and R4G snow span from
   direct-runtime modules. The intermediate `DirectSnowRuntimeCarry` bridge is
   not a retained architecture surface; it is removed by the cutover/deletion
   package once no direct-runtime consumer requires the compatibility mirror.
4. **Frost state skeleton and comparator seam.** Promote R7G frost carry evidence
   into canonical `DirectFrostLaneState`: front/thaw scalars, fine layers, layer
   shadows, no-material carry, liquid/frozen exchange ledger, `watpdg/watbtm`,
   and publication diagnostics. Keep any `HillslopeKernelRequest` or
   `DirectFrostRunoffSurface` bridge only in named test/comparator adapters.
   Production direct code must not build or seed symbol surfaces for frost.
5. **Typed frost solver extraction.** Extract the existing frost hourly solver
   into typed winter-column inputs and in-place lane-state mutation. Validate it
   against the compatibility adapter under `SC-SNOWFREEZE-001` tolerances for
   internal diagnostics, while reserving byte/Arrow identity for public outputs.
   Enforce the hard invariant that persistent fine/shadow carry is not coarse
   layer projection; coarse layer mutation is emitted only from an explicit,
   closed liquid/frozen storage exchange.
6. **Consumer cutover and deletion.** Cut R4A runoff, R4B storage
   reconciliation, R4P/Q/Z hydrology projection, and direct HBP/WAT/PASS/loss/
   manifest publication to winter-column operands. Then remove the current
   direct-runtime snow/frost retrofit: `DirectFrostRunoffSurface`,
   `DirectFrostLiquidPartition`, `frost_runoff_surface`, `frost_liquid_partition`,
   `frost_layer_carry_projection`, `DirectSnowRuntimeCarry`, every
   `snow_runtime_carry` field/constructor input/validation path, optional frost
   runtime carry fields,
   `reconcile_r4a_frost_runtime`, frost surface seeding helpers, and the direct
   publication day-input fields that only exist to shuttle those bridges.
7. **Closure and activation gates.** Rerun the R7G matrix from the new
   architecture. Closure requires no winter hot-path compatibility/symbol-surface
   references, `compatibility_edge_invocations=0`, H2637 direct default `<=10x`
   legacy, protected output identity for HBP/WAT/PASS/loss/plot/manifest,
   snow/frost anti-alias fixtures, and independent operand reconstruction. Only
   after those gates pass can R7H release readiness or direct-default activation
   proceed.

   Current status (2026-06-30): complete and superseded by ADR-0030 runtime
   cleanup. The frost ratification/default-activation package and direct
   cutover correction moved production default execution to direct mode for all
   supported surfaces. Compatibility deletion is no longer blocked by frost
   bit-parity; only the explicit replay seam remains intentionally retained.

### W.1 Snow and Frost Fidelity Adjudication

This is the successor path after R7H opt-in closure. It is an observation- and
benchmark-driven physics adjudication, not a compatibility bit-parity grind.
Authority starts from `SC-SNOWFREEZE-001#GAP-SNOWFREEZE-002`,
`INV-SNOWFREEZE-047`, the observed frost-depth fixture harness, and the
expanded literature set in
`references/annotated_bibliography.md` R-24 and R-26 through R-34.

Static source constraint: `/workdir/wepp-forest_260430_baseline/src/frzng.for`
and current `/workdir/wepp-forest/src/frzng.for` both carry the migration-heat
block with the debug/commented `frzftp = -50` line immediately followed by the
operative `frzftp = 0.0` assignment. The following gate then requires
`frzftp < wtpm` and active frost depth before computing `qwet`; with the shipped
`0.0` front potential this migration heat path is effectively disabled. The
operator additionally confirmed the same `frzftp = 0.0` assignment still ships
in official WEPP 2024-09-30. Therefore `Qwet` is a source-conflict research
candidate only: do not re-enable or port the dead legacy block as production
authority without the adjudication gates below.

Recommended work-package sequence:

1. **SNOWFROST-FIDELITY-A: observation residual classification and
   snow-control gate.** Run the observed frost-depth harness across the pilot
   sites and classify residuals before changing physics: snow-confounded,
   heat-flow/thermal-property shaped, lower-boundary/`Qdry` shaped,
   frozen-hydraulic-conductivity/infiltration shaped, migration/fringe shaped,
   or inconclusive. A frost-depth defect verdict requires paired modeled snow
   depth agreement within `TOL-SNOWFREEZE-009`; otherwise the verdict is a snow
   insulation or harness-input problem, not a frost physics change.
2. **SNOWFROST-FIDELITY-B: no-migration heat-flow benchmark closure.** Add
   analytic or source-level benchmark gates before field calibration, including
   Kurylyk-style one-dimensional freeze/thaw front cases and Dall'Amico-style
   energy-conservation checks. Validate the current no-`Qwet` column through
   snow/residue insulation, surface temperature forcing, thermal conductivity,
   heat capacity, latent heat, lower-boundary heat flow, and fine-layer
   front-state mutation.
3. **SNOWFROST-FIDELITY-C: SFCC and frozen-conductivity diagnostics.** Add
   diagnostic-only implementations for unfrozen-water/SFCC and frozen
   hydraulic-conductivity formulations drawn from Watanabe and Flury,
   Kurylyk and Watanabe, Azmatch, Ming, Cheng, Amankwah, and Devoie. These are
   research/comparison surfaces until a contract ratifies a selected model,
   parameter source, and texture-class default.
4. **SNOWFROST-FIDELITY-D: snow-depth publication and A rerun.** Publish
   modeled snow depth from the existing `snow.runtime_depth_m` runtime state as
   a diagnostic WAT surface (`Snow-Depth`, `mm`), consume it in the observed
   frost-depth harness, and rerun A classification across all pilot sites. This
   is a diagnostic publication package only: `Snow-Water` remains SWE, no
   physics constants or process equations change, and no residual tuning is
   authorized.
5. **SNOWFROST-FIDELITY-E: snow-depth correspondence and direction audit.**
   After D supplies the paired snow-depth control, prove the modeled/observed
   snow-depth comparison is like-for-like before routing residuals: source
   field semantics, units, daily timing/stage, signed residual direction, and
   depth-vs-SWE anti-alias evidence. If correspondence passes and
   `TOL-SNOWFREEZE-009` still fails, frost attribution remains blocked and the
   next route is snow-depth fidelity, not heat-flow or frozen-K tuning.
6. **SNOWFROST-FIDELITY-F: legacy snow-depth output capture and comparator
   assessment.** Before production snow-depth correction, capture pinned
   legacy physical snow depth through an explicit dated output surface and
   compare legacy snow depth/SWE with current openWEPP and observations. Legacy
   remains a flagging comparator under ADR-0017, not a correctness target.
   WAT `Snow-Water` is SWE; physical legacy snow depth must come from
   daily-winter hour-24 rows, with large graphics retained only as sparse
   `snodpy`/`densg` operand provenance.
7. **SNOWFROST-FIDELITY-G: snow-depth producer/carry/input/settlement DC.**
   Close the snow-depth fidelity issue exposed by E. Start from signed residual
   direction, source-line lineage, and F's legacy/openWEPP/observation
   comparator evidence before production edits: snowpack initial state/carry,
   snowfall depth input, density/settlement, rain-on-snow storage, melt
   depletion, and publication lineage. Any correction requires
   contract-first authority, benchmark/source-line evidence, and rerun
   observation snow-control gates.
8. **SNOWFROST-FIDELITY-H: SNOTEL density three-way rubric profile.** Acquire
   paired SNOTEL SWE/depth/soil-temperature rows for the five SNOTEL fixtures,
   derive observed-density SSD arms before residual comparison, and score
   openWEPP, pinned legacy WEPP, and PySnobal as profile overlays under
   `SC-SNOWFREEZE-001#INV-SNOWFREEZE-050`. Output is a
   per-model/per-site/per-cell rubric profile, not a scalar tolerance verdict.
9. **SNOWFROST-FIDELITY-I: conditional heat-flow/frozen-K adjudication.** After
   snow-depth/density control passes or is bounded by a contract-approved snow
   correction and H's PySnobal-unavailable profile cells remain diagnostic flag
   evidence only, choose the smallest
   frost mechanism package still supported by the residuals: no-change/pass,
   heat-flow thermal property correction, lower-boundary heat correction, or
   frozen-K/SFCC parameter/model adjudication. Any production candidate requires
   contract-first authority, benchmark conservation evidence, and observation
   validation.
10. **SNOWFROST-FIDELITY-J: conditional migration/fringe candidate.** Consider a
   `Qwet` or frozen-fringe term only if A-C show residuals that cannot be
   explained by snow insulation, no-migration heat flow, lower-boundary heat,
   or frozen conductivity. Any candidate must use a frozen/fringe-limited
   conductivity, explicit source-water caps, mass and latent-heat closure, and
   observation validation. The old unfrozen-`kunsat` maximum-gradient block is
   not a production shortcut.
11. **SNOWFROST-FIDELITY-K: promotion and activation gate.** Promote the smallest
   physics model that passes field observations, benchmark conservation,
   independent operand reconstruction, snow/frost anti-alias fixtures, public
   output parity where parity remains contractual, and direct-publication
   provenance. Direct stays opt-in until this package explicitly clears
   `GAP-SNOWFREEZE-002`; compatibility deletion and frost-influenced default
   activation remain blocked until then.

Current status (2026-06-25): SNOWFROST-FIDELITY-A is complete as
characterization, SNOWFROST-FIDELITY-B is complete as benchmark-only
no-migration heat-flow closure, and SNOWFROST-FIDELITY-C is complete as
diagnostic-only SFCC/frozen-K candidate tooling. A found all five pilot sites
metric-bearing `UNRESOLVED`, but zero sites are eligible for frost-model defect
attribution because modeled snow depth is absent and `TOL-SNOWFREEZE-009`
cannot be evaluated. B added CLIM06 gates for independent surface resistance
reconstruction, a Kurylyk/Stefan-style one-dimensional freezing bound,
snow/residue insulation, lower-front dry heat, and latent-energy-bounded
fine-layer mutation; production `crates/` still contain no `qwet`, `Qwet`, or
`frzftp` implementation. C added an offline diagnostic JSON/Markdown surface
for Clapeyron/SFCC liquid-water screening, SFCC-Mualem frozen conductivity,
Watanabe/Flury-style capillary-bundle screening, Cheng-style impedance scaling,
and Amankwah-style salinity sensitivity; it explicitly remains non-production,
not a texture-default source, and not field-calibration authority. The next
field-validation package must expose modeled snow depth and rerun A
classification before field residuals are used to choose heat-flow,
frozen-conductivity, SFCC/impedance, or migration/fringe changes.
SNOWFROST-FIDELITY-D completed that diagnostic publication/rerun step:
modeled snow depth is now published as WAT `Snow-Depth`, but the five-site
rerun produced `0` defect-attribution eligible sites because three sites fail
paired snow-depth control and two lack paired observed snow-depth rows.
SNOWFROST-FIDELITY-E then added `INV-SNOWFREEZE-048`, signed residual and
anti-alias audit tooling, and reran the all-site evidence. E found all three
paired-snow sites are snow-depth fidelity issues, not correspondence blockers:
the dominant signed direction is modeled snow deeper than observed snow, with
only `4`, `5`, and `2` adjacent-day timing rescues on Sites 1, 2, and 4. Sites
3 and 5 remain insufficient for snow-control because they lack paired observed
snow depth. Heat-flow, frozen-K/SFCC, impedance, and migration/fringe work
remain unauthorized until a snow-depth producer/carry/input/settlement package
passes or bounds snow control. SNOWFROST-FIDELITY-F then added a pinned-legacy
snow comparator and proved legacy physical snow depth must be captured from
dated daily-winter hour-24 rows; legacy WAT exposes SWE (`Snow-Water`) only,
and large graphics `treal(73)=snodpy*1000` is sparse operand provenance. Across
the three paired-snow sites, both current openWEPP and pinned legacy WEPP still
fail snow-depth control. Legacy is closer by mean absolute observed-depth
residual on Sleepers South and Morris, while current openWEPP is closer on
Sleepers W9. Current openWEPP SWE is already close to legacy SWE on common
model dates (mean absolute deltas about `0.0007-0.0134 m` across the five
sites), so the next package remains the snow-depth producer/carry/input/
settlement DC, using legacy as source-line guide and flag evidence rather than
as a target to bit-match. SNOWFROST-FIDELITY-H then added the SNOTEL
SWE/depth/density corpus and v74 rubric profile report. It derived
observed-density SSD arms from peak-SWE-period SNOTEL density for all five
sites and emitted `snotel-density-three-way-comparison-v2` profiles under
`INV-SNOWFREEZE-050`: all five sites route `STRUCTURAL` in the auxiliary density
fork, while the rubric remains profile-not-scalar. H is complete-with-
disposition: water-year segmented PySnobal passes four sites, while CSS Lab
WY2017 fails inside PySnobal's C core despite finite exported forcing and is
dispositioned as a known upstream PySnobal/SNOBAL thin-snow numerical
instability. Affected PySnobal cells are unavailable diagnostic flag evidence,
not openWEPP failures. The next package is the snow-depth
producer/carry/input/settlement structural remediation DC before any heat-flow,
frozen-K, or migration/fringe production physics work. SNOWFROST-FIDELITY-I0
then established the non-SNOTEL v74 rubric baseline across the original five
frost sites: three paired-snow sites still fail snow control, two isotherm sites
lack paired observed snow depth, rubric totals are `fail=19`, `marginal=8`,
`pass=5`, `strong=20`, `unavailable=63`, and `openwepp_defective_cells=0`.

**Terminal status (2026-06-29): this fidelity arc is complete.** The
SNOWDENSITY-10.3.x snow program reached its forcing-limited floor (the
Harder–Pomeroy psychrometric phase partition is the activated default,
cross-SNOTEL forcing-robust score `15/179` vs legacy `16/176`; holding-capacity
melt and bulk-density compaction are activated; the climate-class density and
shortwave-albedo levers were tested and held). The frost arc then ran the
observed frost-depth harness through the `INV-SNOWFREEZE-050` forcing-robust
tiering, traced the dominant residual to fixture/management residue
parameterization (closed by the dynamic forest-litter coupling, `SC-RESIDUE-001`
`INV-RESIDUE-019`) and to forcing-limited snow magnitude — **not** the frost
solver. It **ratified `INV-SNOWFREEZE-047/048/050`, accepted `TOL-SNOWFREEZE-007..011`,
added `INV-SNOWFREEZE-084` (frost direct-production default activation), and
re-dispositioned `GAP-SNOWFREEZE-002` as open-but-attributed/bounded** under the
Policy-B analog (bounded residuals + full no-regression, not zero-residual). Full
detail in `docs/planning/snow-frost-fidelity-strategy.md` and the
`20260629-frost-*` work-packages. Remaining bounded/deferred items: two snow-free
`Qwet`/wet-heat candidate cells (no legacy authority — legacy `frzftp=0`), and the
litter-drop daylength re-anchor (`docs/backlog/20260626-frost-daylength-canopy-decline-hemisphere-robust.md`).
The end-user summary is `usersum/snow-frost-modeling-and-validation.md`.

(MOFE01 + FARPOINT01 closed hillslope water-routing closure through 19 OFEs; the
H2637 magnitude arc is no longer an active queue item. Absolute forest lateral-flow
magnitude authority is deferred in backlog, not a blocker.)

---

### 1. Per-OFE runoff magnitude adjudication ✅ (complete 2026-06-18)

FARPOINT01 closed the >10-OFE routing **conservation** but surfaced a
**magnitude** divergence: on H2637 openWEPP routes 71 % of precip to the outlet
vs legacy's 55.5 % (both bounded; legacy with_ui is the q-cap-broken 127.7 %).
Adjudicate whether the per-OFE runoff magnitude is expected Stage-2 divergence or
a defect-shaped follow-on, judged against the already-closed routed balance
(comparator a flag, ADR-0017). Package: `MOFE-MAGPARITY01`.

MAGPARITY01 verdict: no transfer, closure, area-scaling, or export defect.
The 71% `runvol` decomposes to a small local surface residual plus routed
upstream lateral/subsurface flow. STAGE2-LATQCC-H2637-MAGNITUDE verdict: the
WB19 `latqcc` equation and operand-bound checks pass, with no openWEPP defect;
the remaining bounded delta is an absolute lateral-flow magnitude authority gap.

### 2. Indexed runtime-surface — hot-path redesign ▶️

PERFHO01 *(complete 2026-06-16)* characterized the ~80–110× H2637 wall-clock gap:
CPU-bound (`977.99/978.55` user s), **not** I/O or parquet, scaling
roughly linear-to-modestly-superlinear in OFE count (`b≈1.12`) — a large
**constant per-OFE-day cost**, not an explosive exponent. GDB-sampled dominant
cost: per-OFE-day symbol-keyed `BTreeMap<BoundarySymbol, BoundaryValue>`
runtime-surface clone/insert/remove/lookup + success-path writeback validation
(`11/15` samples); the WB13-string lead was tested and found **not** dominant.
Verdict: **not acceptable as-is**. **PERFOPT01** *(complete 2026-06-16)* landed
the first optimization — removed the per-OFE-day report-to-persistent-state +
climate-overlay surface clones and made writeback validation detail lazy — for
**~1.15×** on H2637 (`978.55→849.86 s`), **bit-identical** (`anchor_mismatches = 0`,
independently re-confirmed against a separate pre-opt baseline) and
determinism-preserving. The residual now sits in hydrology/transfer guards +
remaining lane-surface clone/drop → **`PERFHO02`** (next characterization). The
band's upper reaches (the `BTreeMap` data-structure replacement) were deliberately
not attempted under the strict bit-identity gate. Packages:
`work-packages/20260616-perf-high-ofe-hillslope-characterization-001/`,
`work-packages/20260616-perfopt01-runtime-surface-map-churn-001/`.

**PERFHO02** *(complete 2026-06-16)* sampled the optimized H2637 path and found
the residual in hydrology typed-symbol lookup/dynamic symbol construction/frost,
decomposition, and PL guard work (`13/20` GDB samples), with secondary
`apply_kernel_writeback` sort/allocation/insertion (`4/20`). After
`kernel.perf_event_paranoid=0` became visible, `perf record` confirmed the same
direction (`execute_persistent_scheduler_kernel_lifecycle` `96.24 %` children,
`apply_kernel_writeback` `12.46 %`, `compute_active_frost_coupling` `12.35 %`).
Output writers again had no sampled dominance.

**Decision (operator, 2026-06-16):** target **≤10× (≤5×)** vs legacy. Incremental
PERFOPT passes are Amdahl-capped well above 10× (PERFOPT01 = 1.15×; the cost is
distributed across every per-OFE-day `String`-keyed access, not a few excisable
functions). **PERFARCH01** completed the design and feasibility work:
`work-packages/20260616-perfarch01-indexed-runtime-surface-design-001/`.
It chose a frozen run-scoped `SymbolRegistry`, sorted-order `SymbolId`, and
dense indexed state/flux storage, with proposed
[ADR-0022](decisions/0022-indexed-runtime-surface-representation.md). Prototype
storage operations were 109.85× faster for clone, 219.16× faster for pre-resolved
lookup, and 115.77× faster for update batches. <=10× is plausible if staged
implementation migrates about 89-90% of current elapsed time out of string-keyed
surface mechanics; <=5× remains aspirational.

**PERFIDX01** *(complete 2026-06-16)* landed the frozen registry + invariants
(sorted-id, equality, completeness with 0 post-freeze unknowns, bit-identical) —
but its completeness audit surfaced that the *bounded* symbol universe is
**~1.7M for H2637** (vs the ~6K the dense-store premise assumed; ~3.6K actually
used), RSS nearly doubling. Lookups stay O(1) at any size, but a dense
`Vec<Option>` indexed by the global 1.7M-`SymbolId` would make the per-OFE clone
(the *dominant* cost) **larger and slower** than the BTreeMap it replaces. **ADR-0022 Amendment 1** (2026-06-16) refined this: the store is sized to the
working set (sparse `Vec<(SymbolId, value)>` primary; dense global-`SymbolId` `Vec`
rejected), with the global registry/sorted-id unchanged. **PERFIDX02** *(complete 2026-06-16)* **cleared the gate**: on real H2637 surfaces
(4,087 present entries) the sparse `Vec<(SymbolId, value)>` clone is **54–70× faster**
than `BTreeMap::clone` (Codex caught + fixed an LLVM clone-elision bench artifact),
the registry tightened **1.7M→44,746** (reachable `ncut`/`ncycle` bound, 0 unknowns),
the shadow equals the BTreeMap (mismatch 0), and outputs are bit-identical (shadow
dormant in production). The dominant lever (clone) is proven; the *total* ≤10× still
awaits the Stage-6 re-measure. **PERFIDX03** *(executed-hold 2026-06-17)* attempted
the flip: the diverse-management registry gate passed (0 unknowns), but the flip
**regressed OFE5 +41.9%** (27.01→38.34 s) — the compat seam clones the sparse store
then **exports it back to a full `BTreeMap` per lane/day** for the kernel, and that
export dwarfs the clone win. Codex held (disabled the flip; no-flip 26.80 s); the
uncommitted code was **discarded** and the record kept. This proves the flip and the
read-side migration are *coupled*: the flip can't win while the seam reads via a full
export. **PERFIDX03B** *(complete 2026-06-17)* closed the blocker differently than a
re-flip: instead of making the indexed store authoritative for reads, it eliminated
the per-lane/day **clone** with `std::mem::take` **move semantics** — the logical
surface is moved into execution, refilled from the report, and the indexed mirror is
refreshed afterward as Stage-4 groundwork. OFE5 **38.34→25.45 s** (−5.1% vs the 26.82
baseline); the full anchor ran and passed (H2637 both UI + 1–5 ladder); outputs are
bit-identical. (The `pass.parquet` byte difference was disproved as a regression:
the same baseline binary emits 3 distinct `pass.parquet` hashes across identical runs
— pre-existing parquet-*container* non-determinism — and the decoded rows are
identical including order.) The win is honestly modest because the **read** seam still
resolves `BoundarySymbol` and the mirror is maintained but not yet consumed.
**PERFIDX04** *(complete 2026-06-17)* closed that second lever: resolve-once
`HotSymbolTables` (built once from the frozen registry) plus an indexed read-mirror
**carried beside** the logical surface and **dual-applied** on each accepted writeback
and same-day OFE transfer (in-place mutation — *no* full-map export, so the PERFIDX03
trap stays closed), migrating the hot read families (climate, frost, WB18/19, PL,
MOFE hourly; **irrigation excluded** — deferred/inert). The mirror is a
**non-authoritative read shadow**; the logical `BTreeMap` remains the commit authority,
so bit-identity is the proof the by-`SymbolId` reads equal the by-`BoundarySymbol` reads.
Result: **H2637 −24.3% / −25.2%** (888.92→673.29 / 894.98→669.75 s), OFE5 −14.3%
(OFE1 −4.4%, setup unamortized on a trivial single-OFE run); full anchor bit-identical
(Claude independently reproduced OFE2, which exercises the transfer-sync path); the
profiler shows hot `format!` collapsed to 0.01% self with id-table helpers in its place.
**PERFIDX05** *(HELD 2026-06-18)* attempted the **write/guard** side (apply-by-`SymbolId`,
consumer-boundary id-sets, failure-path tests) and came back **bit-identical but
−5.3–5.8% on H2637**. Root cause is **structural**, not incompleteness: every
writeback/transfer/guard must **dual-write** the authoritative logical `BTreeMap` *and*
the indexed mirror, and that cost exceeds the id-lookup saving on the write side (reads
won in PERFIDX04 precisely because a read touches only the mirror). The one scan whose
removal could have paid — the decomposition overflow scan — is blocked by the
prefix→range interloper-proof the package correctly declined to force. So the
write/guard-side migration is net-negative under the read-mirror design, and **PERFIDX04
appears to have captured most of the win available under it**. Per operator decision the
PERFIDX05 code was **discarded** (record kept). **PERFIDX06** *(complete 2026-06-18)*
re-measured the PERFIDX04 endpoint and legacy on the same H2637 fixture: openWEPP no-UI
`666.82s`, legacy no-UI median `9.12s`, primary ratio **73.12×**; with-UI ratio
**57.84×**. Verdict: **≤10× is not closed and is not reachable by more narrow id-table
work under the current read-mirror design**; ≤5× is not plausible without a deeper
hot-path state redesign. The next perf mechanism is a deliberate array-authoritative or
fixed-index hot-path state scoping package that avoids both the PERFIDX03 export seam and
the PERFIDX05 dual-write ceiling.
*(Irrigation stays deferred — `backlog/20260617-irrigation-management-gated-activation.md`;
it runs only when the management declares it and is out of scope for the perf migration.)*

### 4. Stage-2 Physics-Magnitude ⏸️ (deferred, judged last)

Fidelity questions deferred by the closure-not-magnitude principle until the structure is
closed and routed so the comparator can attribute error cleanly. They do **not** block
items 1–2.

| Item | What | Provenance | Backlog |
|---|---|---|---|
| Snow magnitude | `snowd.for` melt/settling/density/partition equation fidelity (CRM Ch. 3.7) | legacy physics adjudication | [backlog/20260605-snow-code-deferred-science-review.md](backlog/20260605-snow-code-deferred-science-review.md) (Stage 2) |
| WB16 peak trace-events | The analytical peak estimator (`vave·qpstar`) undershoots legacy on trace runoff events (1–4 mm): p61 yr-10, openWEPP `peakro` 1.1–1.4e-6 vs the legacy passby threshold 2.78e-6 m/s — the legacy-exact erosion day gate then under-routes events legacy routes (`Sed.Del ~0.1 kg/m` class). Bounded; erosion-side adjudicated faithful (cover-closure WP §4). | E.5 adjudication + the ground-cover closure WP (2026-07-05) | fold into the next WB16/peak fidelity investigation — the instruments (p61 + `H61.ebe.dat`) are staged |

(Frost depth was a Stage-2 candidate; the FDMC01 verdict + the settle-vertical-before-routing
principle promoted it to active queue item 1 on 2026-06-07.)

---

## E. Hillslope erosion sediment sequence

**Current position:** **the erosion system is closed, routed, and
class-enriched** — E.4 (merged `55163073`) ported `enrich.for` end-to-end
(per-quantum `frcflw` composition through every deposition region and OFE
exit, per-OFE class operands, the SSA enrichment-ratio diagnostic;
GAP-SED-007 closed), on top of the E.3 Wave-1 chain — E.3 (merged `07856444`, stage-2e deletion merged
`5628f5a0`) chained Wave-1 per OFE on the hourly substrate (per-OFE seeds via
lane slicing, the EROSION-lineage hourly handoff with the `param.for`
Eq. [11.4.x] coefficient continuity, the EXIT-scoped chain EVENT with
`Σ S_h(exit) = Σ_lanes(tdet − tdep)`), produced the **first multi-OFE
sediment in openWEPP's history** (real W7DC01-substrate H102; the deleted
EROD14 path published zero — the `WSHED-W7DC01` producer-side cause is
gone), recorded the INV-RUNOFFPART-030/INV-SED-012 dispositions
(`multi_ofe_wave1_chained` requires the actual seed enable — no-tillage
scope), and DELETED the EROD14/Wave-2 arm outright (SC-SED-001 rev 45 +
companion registry dispositions). Before that, single-OFE Wave-1 went live on
the hydrograph-resolved substrate — E.2 (ADR-0036, merged `7abf2e86`) made the
per-hydraulically-active-hour solve the production form on the shared DC01
shape authority, serialized the paired hourly `V_h`/`S_h` EVENT surfaces (HBP
minor 1, npart = 5, true-volumetric peak), and made the watershed route the
pair (hour-resolved inlet superposition; `S_h` is the sediment mass/timing
authority). The full-reinfiltration and falling-limb quanta deposit with no
clamp. Before that, E.1 completed the single-OFE publication surfaces — the erosion 1b-C flip (SC-SED-001, ADR-0035)
made the direct runtime a sediment producer for single-OFE no-tillage
(forest/disturbed) hillslopes (`route`/`erod`/`runge` RK4 continuity,
conservation-gated; DFF-WS3 directional burn law holds), and E.1 (Increment
1c-fidelity, branch `erosion-e1-inc1c-fidelity`, awaiting Codex review) completed
the single-OFE surfaces: geometry-sourced `field_width_m` (total-kg payload
semantics, p61 `tdet = 15,148 kg = 20.91 kg/m × 724.3 m`), the 5-class `sedcon`
composition split (pre-enrichment basis labeled `GAP-SED-007`), and conserving
nonzero-deposition coverage (190 depositing storm solves on the concave
validation instrument). Detail in
[`work-packages/20260703-erosion-sediment-continuity-port-001/`](work-packages/20260703-erosion-sediment-continuity-port-001/).
The forward queue below is **structure-first, magnitude-last** (ADR-0011): each rung
adds one mechanism on a closed foundation; erosion **magnitude** is judged last and is
**gated on the water-magnitude** item (rill detachment ∝ discharge, so multi-OFE
runoff magnitude must be trustworthy before erosion magnitude is — otherwise water
error aliases into erosion error). Note for E.5: the p61 "~5×" is per-width and a
detachment-vs-delivery cut-point mismatch (compare `exported_kg_m` to legacy
`Sed.Del`) — it is NOT a `field_width` artifact.

Full architecture + open-decision resolution:
[`increment-2-entry-gate.md`](work-packages/20260703-erosion-sediment-continuity-port-001/artifacts/increment-2-entry-gate.md).

### E.5 Erosion magnitude fidelity — ADJUDICATED 2026-07-05 (GAP-SED-009); forward item = the cover-pathway defect-closure
**Verdict (Investigation tier, ADR-0017):** p61 over-detaches ~6× at
MATCHED per-event, per-width delivery cut-points; p102 corroborates the
same ~4–6× class at outlet-aggregate scope (order-of-magnitude evidence
— no per-day chain-export series exists yet). **Attributed to the
erosion ground-cover pathway** (the management IC's
declared `inrcov`/`rilcov` never reach the erosion cover operands;
mass-derived residue cover is ~0 on forest no-decomp ICs — legacy holds
the declared 0.85 covers). **NOT water-driven** — p102's 50-year runoff
volume matches legacy within ~1% — and **not structural** (every
E.1–E.4 closure holds). Record:
WP `20260703-…/artifacts/increment-4-magnitude-adjudication.md`;
`SC-SED-001` `GAP-SED-009`. **The cover-pathway defect-closure
COMPLETE (WP `20260705-erosion-ground-cover-authority-defect-closure-001`,
merged `f7b82dcd`): `GAP-SED-009` CLOSED — p61 dominant event 3.97 vs
legacy 4.2 kg/m, p102 17.4 vs ~19.4 kg/m/yr; the small-event day gate
adjudicated legacy-exact.**

**The remaining erosion ledger is slotted into the existing sequence
(2026-07-05), not queued as work:** the `strcov` standing-mat term and
the other labeled cropland deferrals collect in
[backlog/20260705-cropland-tillage-erosion-enable.md](backlog/20260705-cropland-tillage-erosion-enable.md)
(promoted by a cropland validation driver); the WB16 `peakro`
trace-event flag rides the Stage-2 Physics-Magnitude table (fold into
the next WB16/peak fidelity investigation); the per-class-hourly
interchange channel stays in
[backlog/20260704-hydrograph-resolved-sediment-and-routing.md](backlog/20260704-hydrograph-resolved-sediment-and-routing.md)
"Remaining scope" as the consumer-pulled companion of channel-hourly
routing.

Original scope:
Absolute-magnitude fidelity (p61's ~5×), judged on the closed+routed erosion system
**after** the water-magnitude judgment closes — the forest-lateral magnitude authority
is now the four-tier observed envelope `SC-SUBHYD-001#INV-SUBHYD-033`
(WS10/Maimai/Panola/Weiler; judgment run MOFEFID-C03), so erosion magnitude waits on
that field-data judgment rather than a legacy comparator. Untangles water-magnitude vs
`field_width` vs first-cut-operand contributions.

**Parallel arms (not in the critical path):**
- **Customizable sediment class table + post-fire ash class** — standalone value for
  the disturbed-forest target (the fixed 5-class scheme cannot represent ash); HBP
  `npart` is already variable. Independent of Hairsine-Rose. Promote as its own small
  backlog item.
- **Hairsine-Rose multi-class model** — long-horizon alternative authority arm (explicit
  deposited-layer + settling-velocity deposition `d_i=v_s·c` — the cleaner deposition
  physics for exactly the reinfiltration/footslope/post-fire regimes). Concept-stage,
  contract-first (references-intake HRREF01 active), disabled-by-default. Benefits from
  E.2 but does not gate it.
  [`backlog/20260526-hairsine-rose-multiclass-sediment-model.md`](backlog/20260526-hairsine-rose-multiclass-sediment-model.md).

---

## Keeping this current

This is a forward-only queue; it only works if it stays true.

1. **This file is canonical, and forward-only.** It contains what is *next* and
   *deferred* — never completed work. The [work-packages execution log](work-packages/README.md),
   ADRs, backlog, and agent memory reference it; they do not redefine the queue.
2. **When an item closes, remove it from the queue.** Move its detail/commits to the
   execution log and update the "Current position" line and `Last updated`. This is
   part of the closing package's documentation, not a separate chore. Do not let
   completed items accumulate here.
3. **A strategic decision is not bound until it lands here and in the active handoff.**
   A roadmap living only in an ADR or memory has no hook into the execution chain — to
   redirect work you must change both this file *and* what the active work-package
   handoff names as its next item. (Lesson from the HPHYS0314–0320 arc, where the
   snow-comparator relay continued for seven packages after the strategy had already
   changed elsewhere.)
4. **Deferred ≠ forgotten.** Stage-2 items carry a backlog pointer; promote a
   defect-shaped backlog entry when an item is deferred.

## Authority pointers

- [ADR-0011](decisions/0011-architecture-first-top-down-science-contracts.md) — architecture-first, top-down contracts, comparator-tier policy
- [ADR-0017](decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md) — comparator is a flag, not a target
- [ADR-0018](decisions/0018-defect-closure-execplans-conversion-rule.md) — Defect-Closure ExecPlan conversion rule
- [defect_closure_execplans.md](defect_closure_execplans.md) — DC-ExecPlan authoring
- [specifications/science-contracts/README.md](specifications/science-contracts/README.md) — `SC-*` contract authority
- [work-packages/README.md](work-packages/README.md) — completed-work execution log
