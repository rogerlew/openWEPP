# Work Packages

> **Canonical roadmap: [../ROADMAP.md](../ROADMAP.md)** — a **forward-only planning
> queue** (what is next and deferred). The section below is this roadmap's
> **execution log**: the home for **completed** work — package status, detail, and
> commits. When a queue item closes it is removed from `ROADMAP.md` and recorded
> here. If the two disagree on what is next, `ROADMAP.md` wins.

## Current roadmap execution log

State as of `2026-06-15`:

- CQR03 is complete for behavior-preserving CRAP/cyclomatic-complexity
  decomposition of
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`.
  The management runtime projection dispatcher and primary live-canopy
  assimilation helper are decomposed into private stage helpers, the obsolete
  target-file `too_many_lines` suppressions are removed, and every eligible
  target-module function has CRAP `<= 17.16724537037037` after the refactor.
  Required closure gates passed. Package:
  `20260615-cqr03-management-runtime-inputs-complexity-001/`.
- CQR02 is complete for behavior-preserving CRAP/cyclomatic-complexity
  decomposition of
  `crates/openwepp-input-contract/src/parsers/hbp/layout_parser.rs`.
  `parse_layout` is now a staged dispatcher over private parser helpers, public
  HBP parser APIs are unchanged, and every eligible target-module function has
  CRAP `<= 20.0` after the refactor. Required closure gates passed. Package:
  `20260615-cqr02-hbp-layout-parser-complexity-001/`.
- CQR01 is complete for behavior-preserving code-quality decomposition of
  `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs`.
  `compute_active_frost_coupling` no longer carries the
  `#[allow(clippy::too_many_lines)]` suppression, remains public-surface
  compatible, and its target CRAP row improved from `238.28646229402713` to
  `8.003859752282304`. Required closure gates passed. Package:
  `20260615-cqr01-frost-entry-complexity-001/`.
- REFACTOR024 is complete for a behavior-preserving line-count split of
  `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`. The root
  integration test is now an 11-line module harness with support and concern
  modules under `tests/integration/clim06_frost_frozen_soil_kernel_contract/`;
  all split files are below 1000 lines. The original 46 test functions remain
  present and the required closure gates passed. Package:
  `20260614-refactor024-clim06-frost-test-line-count-split-001/`.
- REFACTOR023 is complete for the 3000+ line
  `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs`
  mechanical split. Scope is behavior-preserving module extraction only:
  `coupling.rs` remains the snow/interval wiring surface while frost helpers
  moved under `support_helpers_mod/coupling/`. Final line counts:
  `coupling.rs=230`, `coupling/frost.rs=1838`,
  `coupling/frost_entry.rs=1000`. Required closure gates passed. Package:
  `20260614-refactor023-hillslope-coupling-line-count-split-001/`.
- HPHYS0320 **closed the SIMIMPL28 storm-start timing seam** (`wnttim < 1.0 -> 1.0`,
  `INV-CLIMATE-018`). This was the first real forcing correction of the entire
  HPHYS0298->0320 snow-comparator arc — and it was a **climate-forcing timing
  defect, not snow physics**. The snow surface was only where the symptom showed.
- The HPHYS0298->0320 snow/`RM` comparator route (the combined `57` carried rows)
  remains retired per ADR-0017 (comparator is a flag, not a target). **Do not open
  HPHYS0321 to continue that route.**
- The snow science review (`docs/backlog/20260605-snow-code-deferred-science-review.md`)
  is now **promoted and split into two stages** (static analysis of the J-95
  negative-SWE site, 2026-06-06): **Stage 1 = snow mass conservation /
  single-sourcing** — an architecture/conservation hard gate that sits on rung-1's
  closure gate, so it is **active now** (see the SNOWSCI Stage-1 package below);
  **Stage 2 = snow physics-magnitude** — the `snowd.for` equation adjudication,
  which **stays deferred behind the protected boundary.** Snow *conservation* is no
  longer suspended; snow *magnitude* still is.
- **WSHED01 closed the openWEPP-native totalwatsed3 CLI + closure** (2026-06-14,
  item 9) — the WBVAL06/6a end-to-end totalwatsed3 deferral is **resolved** on
  openWEPP-native output (`openwepp-cli-totalwatsed3`, ADR-0019/0020), closing
  ex-day-1 at `−0.41 mm/2191 d` with independent operands. The next active rung
  is the **MOFE >10-OFE far-point demonstration** (`MOFE-FARPOINT01`). Channel
  water-balance routed output (`chanwb`) is a **separate** follow-on
  (`WATERSHED-CHANWB-ROUTED-OUTPUT`), decoupled from the hillslope-only
  totalwatsed3 per ADR-0020.

Active work sequence (each rung adds one mechanism on an already-closed
foundation; boundaries are closure gates, not calendar phases).

[kernel refactor follow-on package-complete-with-hold] complete `lib_mod/kernel.rs` decomposition
from `kernel_core.rs` into bounded modules before any bounded surface migration.
 WBVAL02 and
WBVAL03 are Defect-Closure ExecPlan unblockers created from WBVAL01 evidence;
they are bounded defect closures, not a return to diagnostic relay packages.
WBVAL04 is the right-sized post-climate-fix redo of WBVAL01, gated first by a
publication-safe Daymet CLI audit:

1. **WBVAL01** *(executed-hold)* — validation/characterization of single-OFE
   water-balance **conservation closure** on a real CLIGEN daily (non-breakpoint)
   Rocky Mountain run (`/wc1/runs/in/indispensable-presenter`, DRIGGS ID).
   Execution discovered `22` single-OFE hillslopes plus `pw0` as a 9-OFE
   observed-only surface. `12/22` single-OFE hillslopes emitted complete WAT
   ledgers and all `12` are `conservation-break` for years `2..6`; `10/22`
   failed closed before WAT publication (`CLIM-RUNTIME-E-017` or
   `HKERNEL-WB11-PERC-E-003`). This grounds frost targets for emitted ledgers
   while preserving a required follow-on unblocker for the domain-guarded
   hillslopes and the missing year-1 initial-storage surface.
2. **WBVAL02-SIMIMPL28-RADBOUND** *(complete: validated invalid upstream input)* — closed defect
   `WBVAL02-CLIM-RUNTIME-E-017-RADBOUND` for the six WBVAL01 radiation-bound
   fail-closed single-OFE hillslopes (`p2`, `p4`, `p6`, `p9`, `p14`, `p17`).
   The shared DRIGGS daily climate record is invalid at the active SIMIMPL28
   source seam: on `1990-02-18`, `radly=486 Ly d^-1` exceeds baseline `sunmap`
   horizontal potential `r3=453.068716 Ly d^-1`. WBVAL02 amended
   `SC-CLIMATE-001`, added contract tests, and moved the fail-closed evidence
   to typed source symbol `radly`; no radiation guard was loosened and no
   snow/percolation compensation was authorized.
3. **WBVAL03-SNOWMELT-WB-CLOSURE** *(executed-hold)* — close the four
   WBVAL01 J-95 `HKERNEL-WB11-PERC-E-003` fail-closed hillslopes (`p7`, `p11`,
   `p18`, `p20`) and attribute the emitted-ledger conservation residual using a
   complete water-balance identity. Authority/write-set is
   snowmelt/storage/percolation/WAT publication. The closure leak is
   diagnostic-first only inside the package; it is not a diagnostic-only
   package. Current execution is legitimately held behind the upstream DRIGGS
   `radly` source-bound defect (`WBVAL04`): after WBVAL02, all four J-95
   targets and all 12 prior WAT-emitting hillslopes fail earlier at
   `CLIM-RUNTIME-E-017`, `radly=486`.
4. **WBVAL04-WBVAL01-REDO** *(executed-hold)* — reran the whole WBVAL01 Rocky
   Mountain single-OFE validation population after the observed-Daymet producer
   emitted CLI-safe radiation. The climate precondition now passes with zero
   `rad > baseline sunmap.r3` rows. The release validation batch ran all `22`
   single-OFE hillslopes: `18` emitted WAT and all `18` are
   conservation-break for years `2..6`; `p7`, `p11`, `p18`, and `p20` still
   fail closed at J-95 with `HKERNEL-WB11-PERC-E-003`. WBVAL04 routes two
   defect-shaped follow-ons: `WBVAL05-J95-HKERNEL-WB11-PERC-E-003` and
   `WBVAL06-SINGLE-OFE-WAT-CONSERVATION-RESIDUAL`.
5. **WBVAL05-J95-PERCOLATION** *(executed, hold-boundary)* — landed a
   contract-first WB18 fix (`SC-PERC-001` v29: WB18 consumes a published
   `wb12_infiltration` instead of recomputing the WB14/WB12 snow-liquid partition
   and re-validating snow state it does not own); no guard loosening. This cleared
   `HKERNEL-WB11-PERC-E-003` but relocated the fail-closed to
   `HKERNEL-WB14-RUNOFF-E-003`, exposing the true root cause: **negative
   `snow.runtime_swe = -0.006171`**. Legitimately held at the snow boundary; its
   negative-SWE follow-on is folded into SNOWSCI Stage 1.
6. **SNOWSCI Stage 1 — snow mass conservation / single-sourcing**
   *(closed-with-follow-up-postreview)* — closed
   `SNOWSCI-S1-SNOW-MASS-NONCONSERVATION` for the observed J-95 negative-SWE
   fail-closed mechanism by single-sourcing routed snowpack melt to the
   authoritative post-hourly depth/density store. The fix removed the WBVAL05
   publication blocker for `p7`, `p11`, `p18`, and `p20` without a snow
   physics-magnitude change or silent clamp. Post-review gates ran
   `cargo test --workspace`, workspace clippy, `cargo deny check`, fresh
   H1..H39 release/semantic validation, and WBVAL06 before/after residual
   measurement. WBVAL06 annual residual attribution was closed by
   `20260606-wbval06-single-ofe-wat-conservation-residual-defect-closure-001/`.
   Package:
   `20260606-snowsci-stage1-snow-mass-conservation-closure-001/`.
6a. **totalwatsed3 interception-flux audit companion** *(DONE — wepppy
   `aeef2cc6c`)* — WBVAL06 published the daily interception flux as
   `H.wat.Interception`, but openWEPP closure was shown only under its own
   identity audit. The acceptance surface is the **totalwatsed3** WB audit, which
   closes `P - (Runoff + Lateral + ET + Percolation) - ΔStorage`. This WP added
   `Interception` as an optional first-class outflow in
   `wepppy/wepp/interchange/totalwatsed3.py` and
   `tools/totalwatsed3_daily_closure_audit.py` (default 0 when absent, so legacy
   runs close unchanged; **`ET` untouched**). On openWEPP post-WBVAL06 output the
   totalwatsed3 closure identity now closes to ~`2e-7 mm/yr` for years `2..6`
   (vs ~15-19 mm without interception). WP:
   `wepppy/docs/work-packages/20260607_totalwatsed3_interception_flux_closure/`.
   Note: acceptance used a WAT-aggregated totalwatsed3-like surface; a full
   end-to-end totalwatsed3 run awaits openWEPP watershed outputs (MOFE rung).

   **RUNG-1 (single-OFE water-balance closure) is COMPLETE:** SNOWSCI-S1 (snow
   conservation) + WBVAL06 (interception publication) + 6a (totalwatsed3 audit
   consumes interception) → single-OFE WB closes and is auditable on the real
   surface. Next rung: **frost** (item 7).
7. **frost** *(rung-2 — FROSTVAL01 complete after follow-ons)* —
   infiltration/percolation gate (`ksflag`/`ksatadj`) on the closed single-OFE
   vertical balance, with no routing to alias it. **FROSTVAL01** originally ran
   the standard-WEPP `ksflag = 1` frost validation on
   `/wc1/runs/al/algebraic-radium` (43 single-OFE; all lanuse=1→ksflag=1; gridmet
   daily; comparator `wepp_260606`) and held. Findings from that first run (per
   Claude review): 37/43 blocked by `HS-RUNTIME-E-062` (soil-coverage); the
   frost-closure ledger was broken (its ~10 mm inputs were a tool-aggregation bug
   — openWEPP WAT `P` was verified correct/complete at 911 mm/yr — so the
   `frost-break` verdict was withdrawn); and openWEPP's own output showed real
   zero-term anomalies on the runnable cohort (`Q`/`Ep`/`Er`/`Interception` = 0)
   plus likely frost non-activation (`frozwt`=0 at a freezing site with real
   water). The ordered follow-ons closed those blockers: **FQ-1** soil-coverage
   unblock; **FQ-2** ledger fix folded into FQ-4; **FQ-3** ET/runoff zero-term
   characterization/closures; **FQ-4** frost-activation closure. A 2026-06-11
   FROSTVAL01 rerun over all 43 single-OFE prefixes now satisfies activation and
   closure-under-frost: `43/43` `frsoil.active=true`, `43/43` nonzero `frozwt`,
   paired frost-off runs change hydrology on all prefixes, and annual closure
   over 258 rows has max abs residual `3.22e-11 mm`. This exercises the standard
   `ksflag` gate, not the forest `ksatadj` model (separate concern). Package:
   `20260608-frostval01-ksflag-frost-single-ofe-closure-validation-001/`.
7a. **FQ-1 soil corrected-layer coverage** *(executed-hold-boundary)* — closed
   the population-scale `HS-RUNTIME-E-062` soil coverage blocker from
   FROSTVAL01. `SC-SOIL-001` v23 now requires valid parser-layer corrected
   diagnostics to extend the deepest normalized corrected interval to parser
   profile bottom while preserving normalized WB11/WB18/WB19 seed-grid authority.
   Post-fix algebraic-radium validation has zero `HS-RUNTIME-E-062` failures:
   `42/43` prefixes emit `H.wat.parquet` + `H.hbp`; `p11` now fails later at the
   protected percolation boundary with `HKERNEL-WB11-PERC-E-003` on `1990-162`.
   Handoff: `FQ1-P11-HKERNEL-WB11-PERC-E-003-J162`. Package:
   `20260608-fq1-soil-corrected-layer-coverage-closure-001/`.
7b. **FQ-3 runoff `Q/QOFE` underproduction** *(complete)* — closed
   `FQ3-DC-RUNOFFPART-QQOFE-001` for the post-FQ1 algebraic-radium single-OFE
   population. `SC-RUNOFFPART-001` v39 now requires WB12/WB14 to apply the
   top-two-layer storage limit before same-pass infiltration publication and to
   consume the WB18/percolation-produced infiltration value when it already
   owns the same-pass storage update. Post-fix validation produced nonzero
   `Q/QOFE` on all `42/42` runnable prefixes while preserving annual WAT closure
   at numerical noise (`max_abs=2.81e-11 mm`). Package:
   `20260608-fq3dc-runoffpart-q-qofe-closure-001/`.
7c. **FQ-3 Corn annual ET/canopy engagement** *(complete)* — closed
   `FQ3-DC-ET-CORN-ENGAGEMENT-001` for the post-FQ1 algebraic-radium Corn
   population. The annual PL activation sentinel was being deleted on pre-plant
   days and the scheduler calendar `day` symbol was day-of-month instead of
   Julian day, so annual Corn never reached its `jdplt` activation path.
   `SC-PLANT-001` v18 and `SC-EVAP-001` v26 now require annual pre-plant skips
   to be day-local and preserve PL schedule sentinels. Validation over all
   `36/36` Corn prefixes produced nonzero `Ep` and `Interception` with annual
   closure at numerical noise (`max_abs=3.16e-11 mm`). Upstream FQ-3 evidence
   classified `Er=0` as expected-config-zero (`legacy=0`), so this package
   closes the Corn engagement defect for `Ep`/canopy interception and records
   the original `Er` wording as an overclaim, not an unresolved defect. Package:
   `20260608-fq3dc-et-corn-engagement-closure-001/`.
7d. **FQ-4 ksflag frost activation + closure** *(complete)* — closed
   `FQ4-FROST-KSFLAG-ACTIVATION-001`. The root cause was an overbroad activation
   gate: openWEPP treated `frost.options.frost_file_present=0` as disabling
   frozen-soil coupling even when parsed missing-file defaults supplied valid
   standard frost controls with `wintRed=1`. `SC-SNOWFREEZE-001` v53 now makes
   frost file presence provenance-only for activation; `wintRed=1` plus active
   thermal/runtime triggers activates `frsoil`. Post-fix validation ran all `43`
   single-OFE prefixes: all emitted WAT, all had `frsoil.active=true`, all had
   nonzero `frozwt`, and annual closure with `SoilWaterTotal` held at numerical
   noise (`max_abs=3.22e-11 mm`). The old FROSTVAL01 `frost-break` verdict is
   withdrawn as a defective ledger artifact. Package:
   `20260608-fq4-ksflag-frost-activation-closure-001/`.
7e. **FDMC01 frost-depth comparator characterization** *(complete)* — sized the
   frost depth-model gap left open by FQ-4: openWEPP's freeze-index proxy
   (`frdp = 0.20·clamp(−mean_temp/6)`, capped 0.20 m) vs legacy heat-flow.
   Verdict **materially off** — depth capped 200 mm vs legacy 240–503 mm
   (43/43 exceed the cap), depth-series median correlation 0.13, frozen
   duration +258 days (ratchet over-persistence). This verdict + the
   settle-vertical-before-routing principle promoted frost-depth heat-flow
   parity to ROADMAP queue item 1 ahead of MOFE (2026-06-07). Package:
   `20260608-fdmc01-frost-depth-comparator-characterization-001/`.
7f. **FDHP01 frost-depth heat-flow parity** *(complete)* — replaced the
   freeze-index proxy with the single-OFE fine-sublayer heat-flow frost state
   machine (`INV-SNOWFREEZE-006`/`-012`, legacy `frostn` lineage, CRM Ch. 3.8,
   Dun et al. 2010), added WAT `frdp` publication, restored WAT
   `SoilWaterTotal` as the unfrozen `Total-Soil` alias, and bound WAT `frozwt`
   to the layered `Σ soilf(i)` store. The D3 staged arc landed daily
   `frwatc` handoffs, fine-layer freeze/thaw arms, capacity/overflow
   ownership, in-hour resistance feedback, seasonal lower-front heat,
   residue/shallow-front resistance, fixed frozen-path conductivity authority,
   and legacy `hr_tmp`/`tmpadj` surface-temperature synthesis. Dk certified the
   package at the declared ADR-0017 boundary: the Dj/Dk forced-snow cohort is
   `43/43` clean, years 2-6 independent `Total-Soil + frozwt` closure is
   `5.09e-7 mm`, profile-bound pinning is gone (`0/43`), mean/median max depth
   are `501.36/492.36 mm`, median depth correlation rose from the FDMC01
   `0.13` baseline to `0.764`, and frozen-duration residual collapsed from
   `+258` to `+61` days. Residual items are handoffs, not blockers: F4 snow
   density/depth-split magnitude, `p2` individual attribution, dynamic
   residue/decomposition `resdep` lifecycle exposure, and characterized
   upper-envelope subgroup deltas. `SC-SNOWFREEZE-001` v69 closes/re-states
   `GAP-SNOWFREEZE-002`; MOFE is now the next ROADMAP item. Package:
   `20260608-fdhp01-frost-depth-heat-flow-parity-closure-001/`.
8. **MOFE** *(rung 3 — MOFE01 hillslope water-routing closure complete)* —
   closed inter-OFE run-on/run-off routing on the frost-settled per-element
   balance using the `/wc1/runs/ar/arboreal-dendrite/wepp` graded 1–5-OFE
   ladder. M-H ran all 36 hillslopes with fresh openWEPP outputs: 36/36 exited
   zero, row cardinality matched exactly (`271808/271808` rows), transfer
   residual max was `0.0 mm`, per-element residual max was
   `5.968558980384842e-13 mm`, aggregate cancellation residual max was
   `0.0 mm`, downstream `QOFE == Q` alias rows were zero, hydrology clone
   active days were zero, and the 7 single-OFE anchors were 28/28
   byte-identical to the M-F-REDO2 anchor. M-I added the independent in-runner
   hillslope-total identity and closed it at `3.306423012547295e-13 mm`
   against `1e-9 mm`, with all multi-OFE cases nonzero-at-noise; it also
   source-guards the mutually exclusive multi-OFE persistent and single-OFE
   aggregate scheduler lifecycles. Local `owcmp` was run directly without the
   comparator subagent: row keys align for all 36 hillslopes, while semantic
   value-family comparison remains an ADR-0017 investigation signal, not an
   acceptance target. M-G deliberately left sediment-coupled erosion `qin/qout`
   plus particle-fraction handoff as a named follow-on. Package:
   `20260612-mofe01-inter-ofe-routing-closure-001/`. **Closure (2026-06-14):** MOFE01 water-routing closure is done-done on the 36-run 1–5-OFE ladder. Named follow-ons: `MOFE-FARPOINT01` (>10-OFE exceed-the-ceiling demonstration), `MOFE-MAGPARITY01` (±10–25% per-OFE runoff magnitude adjudication), `REFACTOR022` (line-count split), plus watershed/totalwatsed3 (queue item 1) and `MOFE-EROSION-QIN-QOUT-PARTICLE-HANDOFF` (sediment coupling).

   **Next rung — WSHED01 (openWEPP-native totalwatsed3 CLI + closure)** *(complete 2026-06-14)*: closed the end-to-end totalwatsed3 water-balance audit on openWEPP-native output (the WBVAL06/6a deferral). See item 9 below for the W-arc→T-arc pivot (ADR-0019/0020), the three-iteration runvol fix, and the closure evidence. Package: `20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/`.
9. **openWEPP-native totalwatsed3 CLI + closure** *(WSHED01 complete
   2026-06-14 — the WBVAL06/6a deferral resolved)* — consume the closed MOFE
   hillslope outputs and close the end-to-end `totalwatsed3` water-balance audit
   on **openWEPP-native** output. The package began as a watershed-CLI route
   (W-A→W-D): W-A/W-B/W-C cleared the `jpond=0` no-impoundment parser defect
   (`IMP-E-004`/`CLIWAT-E-010`, the `IMP-E-007` count-mismatch split) and the
   over-strict WS10 zero-sediment/`nchnum=0` channel guards, reaching watershed
   output; W-D's audit then exposed that `totalwatsed3` is **hillslope-only**
   (no channel terms) and that the producer was filling `runvol` from WAT `Q`
   (a self-consistency check, not conservation). **Pivot (operator-directed):**
   two ADRs — [ADR-0019](../decisions/0019-openwepp-owns-its-output-surface-wepppyo3-legacy-only.md)
   (openWEPP owns its output surface; `wepppyo3 wepp_interchange` frozen
   legacy-only) and [ADR-0020](../decisions/0020-totalwatsed3-dedicated-output-aggregation-cli.md)
   (totalwatsed3 is a dedicated `openwepp-cli-totalwatsed3`, an
   output-aggregation tier separate from the simulation binaries) — redirected
   the close to a native T-arc (T-A scope → T-B CLI → T-B2 native PASS `runvol`
   → T-C closure). The native `runvol` is sourced from the MOFE outlet-OFE
   routed runoff (the same surface the M-I hillslope-total identity closes on),
   genuinely independent of WAT `Q`. **Runvol took three iterations** —
   `QOFE·A_hillslope` over-scaled 2.5× (runoff > precip; caught by the closure),
   `Q·A_outlet` under-scaled ~4× (a crossed pairing that passed the one-sided
   `≤precip` bound and a self-restating test; caught by reconstructing the
   export from independent operands), and finally **`QOFE_outlet·A_outlet`**
   (≡ `Q·A_hillslope`). **Closure (`openwepp-cli-totalwatsed3` on the native
   arboreal-dendrite PASS/WAT):** `Σ runvol = 27.691 Mm³` (coeff 0.554), runoff
   < precip every year, independent of the WAT-`Q` column (18.895 Mm³); the
   `P − (Runoff + Lateral + ET + Perc + Interception) − ΔStorage` identity
   closes ex-day-1 at `−0.41 mm` over 2191 days with nonzero-at-noise daily
   residuals `[−0.248, +0.005] mm` (day-1 `+30.95 mm` is the storage-prepend
   init, present for any correct producer). Anchors byte-identical
   (`anchor_mismatches=0`); MOFE physics untouched (output-surface-only). The
   forensic record of the runvol arc is `artifacts/review-tb2-runvol-area-defect.md`;
   the durable geometry fact is agent memory `reference-qofe-q-area-duality`.
   Package: `20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/`.
   **Named follow-ons:** `WATERSHED-CHANWB-ROUTED-OUTPUT` (the decoupled
   channel-routing / `chanwb`/`chnwb` watershed output, distinct from the
   hillslope-only totalwatsed3 per ADR-0020 — the W-B/W-C watershed-CLI work
   landed there but channel water-balance routed output remains its own rung)
   and `MOFE-EROSION-QIN-QOUT-PARTICLE-HANDOFF` (sediment-coupled routing).
10. **snow physics-magnitude (Stage 2, deferred)** — the `snowd.for`
   melt/settling/density/partition equation adjudication against external authority
   (CRM Ch. 3.7, WEPP User Doc), behind the protected boundary. Distinct from snow
   *conservation* (Stage 1, item 6, done now); judged last against a fully closed,
   routed balance.

Acceptance target at each rung is **closure** (does it conserve), not **magnitude**
(is the forcing physically right) and not comparator-match. See memory
`project-work-sequencing-wb-frost-mofe-snow` for the rationale and the two
ladder invariants (single-before-MOFE hard dependency; frost is per-column so
single-OFE fully settles it).

## Series index

Per-package execution logs are split by work-package series (newest first within
each). The narrative above is the live cross-cutting state; the docs below are the
archival per-package detail.

| Series | Head package | State | Log |
|---|---|---|---|
| HPHYS | `hphys0320` (2026-06-06) | snow/`RM` comparator arc **retired** per ADR-0017 — do not continue | [series/hphys.md](series/hphys.md) |
| WBVAL | `wbval06` (2026-06-06) | rung-1 single-OFE WB closure **complete** | [series/wbval.md](series/wbval.md) |
| SNOWSCI | `snowsci-stage1` (2026-06-06) | Stage 1 (conservation) **closed**; Stage 2 (magnitude) deferred | [series/snowsci.md](series/snowsci.md) |
| Governance / ADR | `adr0017` (2026-06-05) | comparator-distrust ratified | [series/governance.md](series/governance.md) |

**Frost (FROSTVAL / FQ / FDMC / FDHP):** the recent rung-2 frost packages are logged
inline in the active-work-sequence narrative above (items 7, 7a–7f), not in a
separate series doc.

**Other / historical series** (`auth`, `soilauth`, `infile`, `inspec`, `sci`,
`simimpl`, `wshedimpl`, `inimpl`, `arch`, `pl`, `clim`, `erod`, `wb`, `mofe`,
`refactor`, …): these predate this curated log or were never carried in it. Their
detail lives in each package's dated directory (`package.md` + `artifacts/`). They
are not summarized here; the canonical forward queue is
[../ROADMAP.md](../ROADMAP.md).

Initiative tracking convention inherited from wepp-palimpsest. Each work package lives in a dated directory under this tree.

## Directory naming
`YYYYMMDD-<short-slug>/`

## Required files
- `package.md` — scope, deliverables, dependencies, exit criteria
- `prompts/` — agent prompts (active and archived)
- `artifacts/` — produced docs, contracts, evidence

## Autonomous execution intent (required)
- A work package is an execution-ready plan, not a lightweight task note.
- Planning must be front-loaded into the package so execution can proceed
  autonomously from kickoff through disposition without user intervention.
- `package.md` and kickoff prompts must define concrete sequencing, explicit
  file targets, gate commands, and expected evidence updates.
- Kickoff prompts must include an explicit `Autonomy:` line requiring
  end-to-end execution for the declared scope without additional user
  intervention unless hard-blocked.
- Kickoff prompts default to `Execution mode: package-end-to-end` and should
  direct execution across all package phases through disposition.
- Single-phase kickoff prompts are exception-only and must declare
  `Execution mode: phase-only (exception)` plus explicit rationale and
  follow-on trigger.
- Kickoff prompts must include a `Required reading` list with explicit path
  references to orientation and authority documents so agents do not need to
  independently search onboarding context.
- Kickoff prompts must tier required-reading as `Core`, `Conditional`, and
  `On-demand` to preserve authority while minimizing unnecessary pre-read load.
- `Core` should remain small and stable (global governance + package-local
  authority). Put large mechanism-specific authorities in `On-demand` unless
  package scope requires them before edits.
- Each package should include `artifacts/required-reading-map.md` documenting:
  path, tier, rationale, applicability trigger, and when it was read.
- Kickoff prompts should record required-reading budget metrics for local-repo
  files, using canonical thresholds defined in
  `docs/standards/kernel-work-package-preparation.md`.
- When `REQUIRES-JUSTIFICATION` is reached, author must explain why each heavy
  pre-read is mandatory and cannot be deferred to `On-demand`.
- Work-package authoring must reference and follow:
  `docs/codex_exec_plans.md`.
- Mechanical refactor packages should additionally follow:
  `docs/standards/mechanical-refactor-authoring-guide.md`.

## Dual review and disposition (required)

- Every work package must include two independent review artifacts:
  `artifacts/review_agent_a.md` and `artifacts/review_agent_b.md`.
- Every review finding must be dispositioned as `accepted`, `rejected`,
  `deferred`, or `follow-up` before package closure.
- Accepted findings must be fixed and verified; rejected findings must include
  rationale; deferred/follow-up findings must be linked from
  `artifacts/disposition.md` and `artifacts/worker-handoff.md`.
- Dual verification artifacts must verify both technical gates and that no
  review findings remain undispositioned.

## Phase shape (inherited from wepp-palimpsest)
- **Phase 0**: docs-only audit / inventory
- **Phase 1**: architecture decision with operator-signed acceptance
- **Phase 2**: single-mechanism implementation, replay-and-checkpoint between mechanisms
- **Phase 3**: closeout disposition

## Conventions
- Dates are UTC.
- Evidence classification per claim: `[DIRECT]` (read source / contract / output) vs `[INFERENCE]` (reasoned from evidence).
- Evidence mode per assessment: **Static** (read and reasoned) vs **Ran** (commands actually invoked).
- Single-mechanism rule: one landed change per replay checkpoint.
- Correctness over completion: unresolved contract/invariant correctness gaps keep package disposition in `HOLD` until explicitly resolved or risk-accepted.
- Kernel-affecting packages (including runtime projection controlling kernel branches) must list:
  - `docs/specifications/science-contract-authoring-procedure.md`
  - `docs/specifications/science-contracts/kernel-process-contract-profile.md`
  as dependencies, and must include a kernel-profile compliance checklist artifact.
- Code-authoring work packages should use contract-first sequencing when applicable:
  1. implement/ratify canonical contract amendments,
  2. implement contract-derived tests,
  3. record a pre-implementation contract gate, then
  4. modify production code.
- `package.md` dependencies for authored packages should include:
  - `/workdir/openWEPP/docs/codex_exec_plans.md`
- Missing kernel-profile/procedure compliance keeps disposition in `HOLD`.
