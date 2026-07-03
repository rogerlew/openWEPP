# Increment-1b — Entry Gate + Design (Wave-1 operand production)

Evidence: **Static** — legacy baseline read (`dac3c950`,
`/workdir/wepp-forest_260430_baseline/src/`), the Increment-1
implementation record, and the current direct-runtime frame surfaces.
Author: Claude Code, 2026-07-03.

## Objective

Produce, in the production seed/runtime, every operand the Increment-1
Wave-1 continuity solver requires (`DirectWave1ContinuityInputs`), then
flip the production enable. Increment-1 proved the solver on real
McKenzie forcing with a **test-harness** operand chain; 1b replaces that
harness with source-intent production producers and activates single-OFE
sediment end-to-end. No solver changes are expected (the dry-day
activation contract was validated in Increment-1 review round 1).

## Staging decision — three gated stages, flip last

The operand chain divides cleanly by state requirements:

- **1b-A — event/transport operands (no new daily state).** Everything
  derivable from already-parsed inputs plus the runoff event:
  particle classes, fall velocities, transport coefficients, rill
  hydraulics/shear, effective-intensity surfaces, `detinr`.
  **Execution note (2026-07-03):** 1b-A split in practice into the
  **pure-producer subset** (particle/transport/hydraulics/delivery/detinr —
  landed) and the **runtime-surface subset** (`effint`/`effdrr` export
  from the WB14/WB16 excess machinery + the activation-flag wiring), which
  are runtime integrations rather than pure producers and are **held**.
  The `effint`/`effdrr` export is bounded but real (a new typed
  peak-runoff shadow surface carrying the `sumint`/`durre` integral); the
  `theta_suppressed` `frara` sub-branch shares the 1b-B winter block.
- **1b-B — daily erodibility adjustment chain (new daily state).**
  `kiadjf`/`kradjf`/`tcadjf` from `soil.for` — consolidation
  accumulators, cover/root subfactors, sealing, freeze-thaw. Shadow
  state first, its own gates.
- **1b-C — production activation.** Seed flip, pass-parquet writer
  unhardcode (Wave-1-sourced totals only), DFF-WS3 sediment HOLD flip,
  end-to-end fixture gate.

The flip cannot precede 1b-B: pinning the adjustment factors at the
`inidat.for:424` initialization value (1.0) permanently is NOT
legacy-faithful (legacy updates them daily from day 1) and would be
provisional math in the production path. INV-SED-007 requires
"consistent adjusted soil parameters from Chapter 7".

## Operand lineage map (producer → openWEPP status)

### 1b-A operands

| Operand(s) | Legacy producer | Inputs needed | openWEPP status |
|---|---|---|---|
| 5-class `frac/dia/spg` (+ per-class `fall`) | `prtcmp.for` (incl. the `jflag` large-aggregate clay correction re-entry, `dia` mm→m at `:333`) | layer-1 sand/clay/orgmat (silt = remainder) | texture parsed (`SoilLayer.sand_pct/clay_pct/orgmat_pct`); producer absent. Increment-1 test-harness port exists in `erod16_...rs` as the reference |
| `fall` / `veleff` | `falvel.for` + `cdre/cdre2` drag tables (`inidat.for:1017-1034`); effective `diaeff/spgeff` = 3-class log-means (`param.for:558-579`) | classes | absent (harness port exists) |
| `kt`, `kt2`, `ktrato`, `tcend` | `trcoef.for` → `yalin.for` (+ `shield.for` diagram; note the legacy mixed linear/log extrapolation above the table) | classes + shear | absent (harness port exists). `tcend = kt·shrsol^1.5` floor 1e-10 (`param.for:234`); the sandy `adjtc` floor lives INSIDE `yalin.for:141-145` |
| `qshear`, `qout` | `xinflo.for:150,186` (`qout = peakro·efflen`, `qshear = qout·rspace`) | peakro (WB16), efflen, rspace | peakro/efflen exist; rspace: cropland default 1.0 m (rangeland formula `xinflo.for:136-145` if native lanuse lands) |
| `frcsol`, `frctrl` (rill friction) | `frcfac.for:222-236` cropland: `frcsol = 1.11`; `frccov = 4.5·rilcov^1.5544`; `frlive = (canhgt/hmax)·flivmx`; `frctrl = frccov + frlive + frcsol` | rilcov, canhgt, hmax, flivmx | all on the PL projection (the WB16 ealpha producer already reads them). Closes `EROD-BND-002` — SC-HYDRAULICS/SC-SED boundary ownership must be recorded |
| `shrsol`, `shrend` | `shears.for` (Gilley width growth `1.13·q^0.303` capped at rspace, Chezy depth iteration tol 5e-6, `shear = wtdens·sin(atan(S))·Rh·frcsol/frctrl`); slopes: `cnslp = avgslp`, `slpend = (a_n + b_n)·avgslp` (`param.for:167-209`, 1e-6 floors) | qshear, slopes, width state, friction | absent. **Width is persistent state** (grown by Gilley per event, reset only at tillage — none in forest managements); `rwflag` gate |
| `effdrr`, `effint` | `irs.for`/`grna.for:607`: `effdrr = durre` (rainfall-excess duration), `effint = sumint/durre` (mean rainfall intensity over excess periods) | WB14 excess/hyetograph profile | WB16 already computes an excess duration internally (`runoff.rs:792`); needs a typed export + the `sumint` integral. `effdrn = runoff/peakro` already exported (`runoff_duration_s`) |
| `detinr` | `param.for:463-518`: `detinr = ki·kiadjf·effint·qi·intdr·rspace/width`, `qi = runoff/effdrr`; interrill delivery `rif = −23·rrc + 1.14` (clamped 0..1), per-class `drinti` from fall velocities, `intdr = Σ frac·drinti`; non-cropland: `intdr = 1` | ki (parsed), kiadjf (1b-B), effint, rrc, classes | absent. `rrc` (random roughness) is on the PL projection |
| `beta` | `param.for:586-590`: 0.5 when rain or sprinkler depth today, 1.0 otherwise | daily rain surface | frame has daily precipitation |
| `surface_frozen`, `theta_suppressed` | `param.for:396` (`frdp>0 && thdp<=0`), `param.for:530` (snow depth > 0; melt-only day with `frara<=0.8`; furrow-only) | frost/snow/melt surfaces | frame has frost depth/thaw and snow state; **`frara` lineage must be traced during 1b-A** (winter surface, Savabi-era) |
| `strldn`, `qin` | 0 for single-OFE OFE-1 (`xinflo`) | — | trivially available |
| `field_width_m` | slope `fwidth` | parsed | available |

### 1b-B operands (`kiadjf`/`kradjf`/`tcadjf`, `soil.for:820-1170`)

| Piece | Source | Inputs | Status |
|---|---|---|---|
| Consolidation baselines `kicrat` (0.1..1.0), `krcrat` (0.05..1.0), `tccrat`, `bconsd = 0.02` | `scon.for:630-760` (`kconsd` from sand/orgmat/**scon-corrected** `thetfc` — see the Profile-FC lineage note: legacy uses scon-corrected values) | texture + corrected thetfc | static, portable; openWEPP already carries the corrected-seed machinery |
| `rfcum` (rain since disturbance), `daydis` (days since disturbance; `+1` per day when `rfcum > 0.01`, `×(1−surdis)` at tillage) | `soil.for:833-846, 314-357` | daily precip/irrigation, tave, tillage events | two new daily accumulators; forest managements have no tillage → monotone |
| Consolidation `produc = bconsd·daydis`; `ckiasc/ckrasc/ctcasc` interpolate to the `*crat` baselines (`exp(−produc)` blend, `<10` underflow trap) | `soil.for:944-1100` | accumulators + baselines | equations in hand |
| Canopy `ckiacc` (Laflen), ground cover `ckiagc = exp(−2.5·inrcov)`, live root `ckialr = exp(−0.56·rtm15)`, dead root `ckiadr`, buried residue `ckrbgb = exp(−0.40·Σsmrm)`, dead/live root Kr (`ckradr`/`ckralr`), roughness `ctcarr = 1 + 8·(rrc − 0.006)` | `soil.for:925-1100` | cancov/canhgt/inrcov (PL projection), live/dead root mass + buried residue (growth/decomposition state) | frame surfaces exist; the root/residue mass symbol mapping must be audited (rtm15 = live root mass 15 cm; rtm 1..3 dead root pools; smrm buried residue pools) |
| Freeze-thaw `ckiaft/ckraft/tcaft` | `soil.for:843-920`: frozen-surface zero branch; thaw detection via layer-1 water vs field capacity; matric potential `tenkpa`; `fcycle` freeze-thaw cycle counter; `acyc` capped 1.31 | frost depth/thaw depth (frame has), layer-1 water/porosity/thetfc/thetdr (frame has), **`fcycle` counter (new state, winter-owned)** | equations in hand; `fcycle` producer must be traced (winter subsystem) |
| Interrill slope factor `ckiasa = 1.05 − 0.85·exp(−4·sin(slope))` | `soil.for:995-1018` | avgslp (rh/rspace ridge branch for tilled rows) | trivial |
| Floors: `kiadjf ≥ 0.03`, `kradjf ≥ 0.03`, `tcadjf ≤ 2.0` | `soil.for:1026,1096,1100` | — | note `tcadjf` has **no lower floor** in soil.for; the INV-SED-006 `≥ 0.30` gate is the yalin-side sandy floor — reconcile in the contract mapping before wiring (the solver's input validator currently enforces ≥ 0.30 on `tcadjf`; verify against SC-SED-001 whether that bound belongs on `tcadjf` or only on the transport adjustment inside `kt` — flag for the executor, contract-first) |

### 1b-C activation

1. Seed: `direct_production_typed_erosion_authority` populates
   `DirectWave1ContinuityInputs` (static geometry via
   `derive_wave1_slope_segments` + per-day operands via the 1b-A/1b-B
   producers through r7d8/day inputs) and enables for
   `contributor_ofe_count == 1`.
2. Pass-parquet writer: `build_hillslope_pass_row_from_direct_publication`
   currently hardcodes `tdet/tdep/sedcon = 0`; unhardcode **only for
   Wave-1-continuity-sourced rows** — the Wave-2 placeholder-seeded
   router totals must NOT publish (its seed uses `MOFE03_WAVE2_DEFAULT_*`
   placeholder values).
3. DFF-WS3: flip the sediment HOLD assertions to the live ordering law
   (high burn ≥ unburned detachment; direction only, ADR-0017).
4. Byte-stability: all non-sediment production surfaces must be
   unchanged (wat parquet, water balance) — shadow-first comparison run
   before the flip commit.

## Hard gates (per stage)

- **1b-A:** every producer unit-tested against hand-computed legacy
  equations; the `erod16` fixture test swaps its test-harness operand
  constructions for the production producers (the harness in
  `erod16_wave1_continuity_fixture_conservation.rs` then dies — it was
  explicitly temporary); operand-availability audit FIRST (Increment-1
  lesson: trace every producer before claiming it exists).
- **1b-B:** shadow adjustment-factor trajectories published as
  diagnostics before consumption; bounds (`0.03` floors, `2.0` cap,
  `*crat` clamps) as typed guards; no-tillage forest fixture sanity
  (daydis monotone, factors converge to the consolidation baselines).
- **1b-C:** the ADR-0035 Increment-1 gate, now end-to-end on the
  production path: conservation + INV-SED-001/002/003/006/007 +
  INV-SED-010 payload on the McKenzie clay-loam fixture; runoff/WB
  surfaces byte-stable; full AGENTS gates; **inert-day regression** (the
  review round-1 contract: dry/passby days publish zero-authority
  without touching routed operands).

## Known traps carried forward

1. Test inert paths with the operand shapes the production runtime
   actually supplies (review round-1 lesson).
2. `erosion_continuity.rs` is at ~1,950 lines — the 1b executor splitting
   or extending it must decompose first (2,000 WARN).
3. `peak_runoff_m3_s` frame fields behave as m/s depth-rate (the suffix
   is a misnomer); `effdrn ≡ runoff/peakro` = WB16 `runoff_duration_s`.
4. Fixtures run as cropland (`lanuse = 1` masquerade) — the CROPLAND
   branches are normative here; the DFF-WS1 native-forest lanuse mode
   (unmerged branch) uses the non-cropland `intdr = 1`/`fidel = frac`
   branches — keep both branch families, select by lanuse.
5. `scon.for` corrections: the consolidation baselines use
   scon-corrected `thetfc` (the Profile-FC lineage) — use the corrected
   seed values, not raw per-layer symbols.
6. `width` is persistent event-grown state; `fcycle` and
   `rfcum/daydis` are new daily accumulators — all shadow-first.
7. `frara` (theta-suppression melt branch) has no traced producer yet —
   resolve during 1b-A or carry the melt-only suppression as an explicit
   typed TODO with the snow-cover branch active.

## Entry-gate status: RESOLVED (design + lineage mapped)

Staging decided (A/B/C, flip last), every operand mapped to its legacy
producer with line references and its openWEPP input status, gates and
traps recorded. Execution may begin with 1b-A.
