# Handoff Prompt — Erosion Port Increment-1b (Wave-1 operand production + activation)

> **Task prompt for a fresh executing agent.** Self-contained; read the
> referenced artifacts, then implement. You are executing **Increment-1b**
> of the ADR-0035 erosion port in `/home/workdir/openWEPP` (Rust).
> ADR-0035 names Claude Code as executor (operator-authorized exception) —
> **author the Rust code.** Work in a fresh worktree off `main`
> (`ln -s /home/workdir/openWEPP/.venv .venv` for the full gate); push a
> branch for Codex review; **do NOT self-merge.**

## 0. Orient (read in order)
1. `docs/work-packages/20260703-erosion-sediment-continuity-port-001/artifacts/increment-1b-entry-gate.md`
   — **the design you are implementing**: staging (1b-A → 1b-B → 1b-C,
   flip last), the full operand→producer lineage table with legacy line
   references, gates, and traps. This handoff does not repeat the
   tables; the entry gate is the map.
2. `.../artifacts/implementation.md` — what Increment-1 landed, its
   review round-1 dry-day activation contract, and the 1b queue.
3. `.../package.md` + `docs/decisions/0035-...md` — program frame.
4. `docs/specifications/science-contracts/contracts/SC-SED-001.md` —
   INV-SED-004 (hydrologic inputs), -005 (shear partition), -006
   (transport + sandy floor), -007 (normalization/Chapter-7 adjusted
   operands), plus `EROD-BND-002` (the hydraulics intake boundary you
   are closing). Acceptance authority is the contract, never legacy
   magnitude (ADR-0017); legacy `.for` is source-intent (ADR-0024).
5. The Increment-1 solver: `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_continuity.rs`
   (its `DirectWave1ContinuityInputs` doc comments name each operand's
   legacy lineage) and the **temporary test-harness operand chain** in
   `tests/integration/erod16_wave1_continuity_fixture_conservation.rs`
   — a working Static port of `prtcmp`/`falvel`/`shield`/`yalin`/
   `trcoef`/`shears` you will promote to production form (do not copy
   its four labeled harness assumptions into production).
6. Legacy sources (baseline `dac3c950`,
   `/workdir/wepp-forest_260430_baseline/src/`): `prtcmp.for`,
   `falvel.for`, `sedia.for`, `shield.for`, `yalin.for`, `trcoef.for`,
   `frcfac.for`, `shears.for`, `xinflo.for`, `param.for`, `irs.for`,
   `grna.for`, `soil.for` (820–1170), `scon.for` (600–780),
   `inidat.for` (drag tables :1017, initializations :380/:424),
   `contin.for` (call order :1010–1230).

## 1. Non-negotiables
- **Operand-availability audit FIRST, per stage.** Increment-1's core
  lesson: the previous gate claimed "the projection exists" when only
  struct fields existed. Before writing a producer, trace each of its
  inputs to a concrete parsed/frame surface and record the mapping in
  the implementation artifact. Gaps become typed fail-closed
  requirements or explicit stop-conditions — never defaults.
- **No provisional math in production.** In particular: do NOT flip the
  seed with adjustment factors pinned at 1.0 (that is the day-zero
  initialization, not the daily chain). The flip happens in 1b-C only,
  after 1b-B's chain is live and shadow-validated.
- **Stage gates are hard stops.** If a stage's gate cannot close (e.g.
  the `frara` or `fcycle` lineage cannot be traced to a real producer),
  stop at the declared boundary, record it defect-shaped, and hand off —
  do not force.
- **Test inert paths with production-shaped operands** (review round-1
  lesson): dry/passby days supply zeroed routed operands; the solver's
  activation contract already handles this — keep it regression-covered
  through every new wiring layer.
- `erosion_continuity.rs` is ~1,950 lines (WARN at 2,000). Put the new
  producers in **new sibling modules** (e.g.
  `direct_runtime/erosion_operands.rs`, split by stage if large); if you
  must touch `erosion_continuity.rs` beyond small edits, decompose it
  first as a separate mechanical commit.
- Full AGENTS gates before merge-ready; record evidence classes
  (Static/Ran) in `artifacts/implementation-1b.md`; update `package.md`.

## 2. Stage 1b-A — event/transport operand producers
Implement production producers (typed, fail-closed, unit-tested against
hand-computed legacy equations — see the entry-gate table for sources):
1. Particle composition (`prtcmp` core incl. the `jflag` re-entry and
   mm→m conversion), fall velocities (`falvel` + drag tables), effective
   particle (`param.for:558-579` 3-class log-means) → `veleff`.
2. Transport: `shield`/`yalin`/`trcoef` → `kt`, `kt2`, `ktrato`,
   `tcend = kt·shrsol^1.5` (floor 1e-10). The sandy floor lives inside
   `yalin`; reconcile its relationship to the solver's `tcadjf ≥ 0.30`
   input guard against SC-SED-001 **before** wiring (contract-first; the
   entry gate flags this).
3. Rill hydraulics: `frcfac` cropland rill friction from the PL cover
   surfaces; persistent Gilley rill `width` state (event-grown, capped
   at `rspace`, tillage-reset hook even though forest managements never
   fire it); `qshear = peakro·efflen·rspace`; `shears` (Chezy depth
   iteration) at `cnslp = avgslp` and `slpend = (a_n+b_n)·avgslp` →
   `shrsol`, `shrend`. Record the `EROD-BND-002` ownership closure in
   the SC contract mapping.
4. Effective-intensity surfaces: export `effdrr` (rainfall-excess
   duration — WB16 computes it internally) and `effint = sumint/durre`
   from the WB14 excess machinery (`grna.for:607` semantics).
5. `detinr` assembly (`param.for:463-518`) including the interrill
   delivery ratio (`rif` from `rrc`, per-class `drinti`, `intdr`); keep
   both the cropland and non-cropland (`intdr = 1`) branch families,
   selected by lanuse.
6. Activation flags: `beta` (rain today → 0.5), `surface_frozen`
   (`frdp>0 && thdp≤0`), `theta_suppressed` (snow cover; melt-only —
   trace `frara` now or carry that sub-branch as an explicit typed TODO
   with the snow branch active; furrow branch is out of scope).

**Gate 1b-A:** producer unit tests green; the `erod16` fixture test
**swaps its test-harness chain for the production producers** (delete
the harness functions — they were explicitly temporary) and still
proves nonzero detachment + conservation on the McKenzie clay-loam
storm population; full suite green; no production output changes yet
(producers exist but the seed stays disabled).

## 3. Stage 1b-B — daily erodibility adjustment chain
1. Static baselines from `scon.for` (`kconsd` → `kicrat`/`krcrat`/
   `tccrat` with their clamps, `bconsd = 0.02`) using the
   **scon-corrected** `thetfc` seed (Profile-FC lineage — corrected
   values, not raw layer symbols).
2. New daily accumulators (shadow-first): `rfcum` (rain+irrigation since
   disturbance, `tave > 0` guard), `daydis` (+1 per day when
   `rfcum > 0.01`; tillage scaling hook), `fcycle` freeze-thaw cycle
   counter (trace the winter-side producer; stop-condition if absent).
3. The subfactor chain (`soil.for:843-1100`): freeze-thaw
   (`ckiaft/ckraft/tcaft` incl. the matric-potential thaw branch),
   canopy/ground-cover/live+dead-root/buried-residue factors from the
   frame's growth/decomposition/residue state (audit the symbol mapping:
   `rtm15`, `rtm(1..3)`, `smrm(1..3)`), sealing (`ckiasc` via
   `produc = bconsd·daydis`), slope factor `ckiasa`, and the composite
   `kiadjf`/`kradjf`/`tcadjf` with the 0.03 floors / 2.0 cap.

**Gate 1b-B:** factor trajectories published as shadow diagnostics and
sanity-checked on the no-tillage forest fixture (monotone `daydis`,
convergence to consolidation baselines, freeze-thaw excursions only in
winter); typed bound guards; full suite green; production outputs still
unchanged.

## 4. Stage 1b-C — activation
1. Seed flip: `direct_production_typed_erosion_authority` builds the
   populated `DirectWave1ContinuityInputs` (static geometry via
   `derive_wave1_slope_segments`; per-day operands via the 1b-A/1b-B
   producers through the day-input path) and enables for
   `contributor_ofe_count == 1`. Wave-2/EROD14 stays as-is.
2. Pass-parquet writer: unhardcode `tdet/tdep/sedcon` **only for
   Wave-1-continuity-sourced totals** (the Wave-2 placeholder-seeded
   router must not publish — its seed is `MOFE03_WAVE2_DEFAULT_*`).
3. Flip the DFF-WS3 sediment HOLD assertions to the live directional
   ordering law (high-severity burn ≥ unburned detachment; direction
   only, no magnitudes — ADR-0017).
4. **Hard gate (the ADR-0035 Increment-1 gate, end-to-end):**
   conservation `Σdetach − Σdeposition = exported` + INV-SED-001/002/
   003/006/007 + the INV-SED-010 totals payload on the McKenzie
   clay-loam fixture through the production path; dry/passby days
   publish zero-authority (inert-day regression); **all non-sediment
   surfaces byte-stable** vs the pre-flip baseline (wat parquet, water
   balance) — run the comparison before the flip commit; HBP EVENT
   payload carries the totals.

## 5. Deliverables
1. Production operand producers (1b-A) + daily adjustment chain (1b-B),
   typed, fail-closed, unit-tested, with the operand-availability audit
   recorded per stage.
2. Production activation (1b-C): seed flip, writer unhardcode, WS-3
   HOLD flip, end-to-end fixture gate green, non-sediment byte-stability
   evidence.
3. `artifacts/implementation-1b.md` (evidence classes matched to what
   ran) + `package.md` status; line-count governance notes for any file
   split.
4. Branch pushed for Codex review. **Not merged to main.**
