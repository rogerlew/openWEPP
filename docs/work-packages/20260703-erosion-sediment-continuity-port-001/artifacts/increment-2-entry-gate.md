# Increment 2 — Multi-OFE Erosion Integration: Entry-Gate Design

Author: Claude Code, 2026-07-04. Evidence: **Static** (legacy `route`/`erod`/
`enrich`/`xinflo` source-intent, SC-SED-001, the current Wave-1/Wave-2 runtime,
SC-ROUTE-001). No execution.

Status: **entry-gate design — not an implementation authorization.** This resolves the
architecture and the open decisions so Increment 2 can be scoped as a staged,
conservation-gated port. Governing authority: ADR-0035, SC-SED-001, ADR-0011
(contract-first), ADR-0017 (comparator-as-flag).

## 1. Where Increment 1 left the runtime

The 1b-C flip (merged `9c54837b`) made the **single-OFE Wave-1** sediment-continuity
solve real: `route`/`erod`/`runge` RK4 march of `dG/dx = D_f + D_i` over the
normalized profile, with `xcrit`/`mshear` regime dispatch, conservation-gated (the
1e-9 mass-balance publication closure + the flux-discretization gate). It is enabled
for single-OFE, no-tillage hillslopes (the forest-masquerade scope). First runtime
sediment: p61 `tdet = 20.9 kg/m`; DFF-WS3 directional burn law (`2491 kg` burn ≫
`258 kg` unburned).

Two erosion paths currently coexist in the runtime:

| Path | What | Scope now |
|---|---|---|
| **Wave-1** (`route`/`erod`/`runge`) | Full point-by-point RK4 continuity over `x∈[0,1]` | single-OFE only (the 1b-C flip) |
| **Wave-2** (EROD14/EROD15) | Boundary/case-based OFE-level sediment router (`ldtop`/`ldbot`/`lddend`, `qin`/`qout`, `case_value` 1–4, per-class `frcflw`) | the *only* path for `ofe_count > 1` |

**The central Increment-2 question is the relationship between these two.** *(Inference,
to confirm against the kernel at kickoff: EROD14 is a coarser boundary/analytic
formulation, not the full continuity — its inputs are OFE-boundary loads and a
case selector, not a per-grid march.)*

## 2. What legacy WEPP does across OFEs (source-intent)

Legacy chains the **same** per-OFE continuity down the hillslope via three boundary
couplings (baseline `route.for:139-154`, `xinflo.for:130-151`, `erod.for`,
`enrich.for`, REF-SED-LEGACY-MOFE-QIN / REF-SED-LEGACY-ENRICH):

1. **Discharge accumulation** — `qout` of OFE *i* becomes `qin` of OFE *i+1*; more flow
   downslope ⇒ more shear ⇒ more transport capacity.
2. **Sediment-load handoff** — the load exiting OFE *i* (`G_out`) seeds the top-of-
   segment inflow load (`ldtop`) of OFE *i+1*; a downslope OFE therefore starts already
   loaded, biasing it toward deposition (`load > Tc`).
3. **Particle-class + enrichment routing** — deposition preferentially drops coarse
   classes, so the fractions handed downslope are finer (enrichment); `enrich.for`
   updates class fractions at every deposition transition and at each OFE exit (`iendfg`
   terminal call). The 5-class fractions are copied to the downstream OFE on flow entry.

So: **per-OFE continuity, chained by (discharge, load, particle-fractions), with
enrichment on deposition.**

## 3. The architecture decision: Wave-1 owns the per-OFE physics

**Recommendation:** make **Wave-1 the per-OFE continuity engine** run for every OFE,
and retire Wave-2 as a *separate physics arm*, keeping only its **inter-OFE routing/
handoff plumbing** (the `qin/qout` accumulation and the `particle_flow_fraction`
handoff already threaded in `r7d8`). Rationale:

- Wave-1 is the more faithful physics (point-by-point regime resolution vs. a
  boundary/case approximation). Running two different sediment physics for `ofe_count==1`
  vs `>1` is an inconsistency that will alias into every multi-OFE fidelity question.
- The routing plumbing Wave-2 established (upstream `qout → qin`, prior-OFE particle
  fractions) is correct and reusable — it is the *handoff*, not the *physics*.

**Concretely:** for each OFE in down-slope order, run the Wave-1 solve with

- inflow load `G_in = G_out` of the OFE above (Wave-1 already accepts `strldn`/inflow),
- inflow discharge `qin` from the upstream operand handoff,
- inflow particle-class fractions from the upstream operand handoff,

and publish `G_out`, `qout`, and the exit class fractions for the next OFE. The
hillslope-exit values are the HBP EVENT payload.

**Open sub-decision (kickoff):** whether EROD14 is deleted outright or retained
behind a comparator flag for one release to cross-check Wave-1's multi-OFE numbers.
Given ADR-0017 (legacy/older-arm is a flag, not a target), retain-as-flag then delete
is the low-risk path.

## 4. Hard prerequisite: single-OFE completeness (before any multi-OFE work)

Multi-OFE routing **transports deposition and per-class concentration**. The 1b-C
first-cut gaps here are therefore in-scope *before* Increment 2 proper — but they are
different in kind:

- **`tdep` (total deposition) is already published** (`total_deposition_kg` /
  HBP `tdep`) and the solver computes it; it read 0 on p61/DFF-WS3 because those
  profiles are detachment-dominated (a *value*, not a missing surface). The real gap is
  **nonzero-deposition validation coverage** (a depositing fixture) and the **per-OFE
  deposition semantics** the multi-OFE handoff needs.
- **The 5-class `sediment_concentration_kg_m3` array is the genuinely-zeroed surface** —
  the single-OFE Wave-1 carries a scalar concentration but not the per-class breakdown;
  computing it is the particle-class work (Increment 3), which the particle handoff and
  enrichment require.
- **`field_width_m`** — sourced from hillslope geometry (currently the unit-width
  default), so the denormalized total mass is right at the OFE and hillslope exits.

These are structural (closure), so they precede the multi-OFE mechanism per the
closure-before-magnitude ordering. Call this **Increment 1c-fidelity** (single-OFE
surface completion) — a short rung in front of Increment 2.

## 4a. Particle-size distributions must be per-OFE (legacy `partsize.dat` is a MOFE gap)

The 5-class particle-size distribution — the base `frac`/`dia`/`spg`/`frcly`/`frslt`/
`frsnd`/`frorg` surface that seeds `sedcon` and every deposition/enrichment transition —
**must be sourced per-OFE from that OFE's own soil**, not from a single hillslope-global
distribution. This is a hard input-authority requirement for the multi-OFE chain, and it
is a place where legacy must **not** be inherited faithfully.

**Legacy source-intent (verified 2026-07-04, operator-flagged; static read of
`wepp-forest`/DEP `wepp20240930`).** WEPP's *default* particle-size computation is already
per-OFE — `prtcmp.for` derives the class distribution from each element's clay/silt/sand/
organic-matter (`clay(1,ielmt)`, `silt(1,ielmt)`, …), indexed by `ielmt`. **That per-OFE
default is the correct, faithful basis to port.** But Dennis Flanagan's late user-override
`usr_partsize` (tagged `DCF - March 14, 2024`, one of his last additions before
retirement) reads a **single** particle-size block from `partsize.dat` (`main.for:106`
opens unit 9; `prtcmp.for:90-101` reads one `npart` block into the *current* element) —
there is no per-OFE keying, and the sequential single-file read cannot supply a distinct
distribution per OFE. On a multi-OFE hillslope where OFEs differ in soil, one global
`partsize.dat` cannot represent the per-OFE variation, and the read mechanics degrade for
`ofe_count > 1`. This is a concrete instance of the program-wide finding that **legacy
MOFE was never a maintained path** (Dennis was ag / single-field focused as he approached
retirement) — the same class as EROD14 emitting zero HBP sediment. Treat the single-global
`usr_partsize` semantics as a legacy MOFE gap (ADR-0017 flag); do not port them.

**openWEPP requirement.**
- **Base per-OFE distributions:** derive each OFE's 5-class distribution from *that OFE's*
  soil texture / organic matter (the `prtcmp` per-element default lineage), per-OFE by
  construction. The downslope particle-fraction handoff (§2c/§2d) and enrichment (§2d)
  are only meaningful on top of correct per-OFE *base* classes.
- **User override, if supported at all, MUST be keyed per-OFE.** A single hillslope-global
  override is **rejected fail-closed**, not silently broadcast — a broadcast global
  distribution would misattribute one soil's classes to every OFE and corrupt both the
  particle-fraction handoff and the enrichment ratio downslope.
- **Gate.** A fixture with **soil-contrasting OFEs** (e.g. a coarse OFE above a fine one)
  is the validation vector: the per-class `sedcon` and the exit fractions must reflect
  per-OFE sourcing + enrichment, not a single distribution; size-class mass conservation
  (SC-SED-001 §11.5 / `INV-SED-0(08–11)`) is only a real check if each OFE's base classes
  are its own.

Stated here (ahead of the §2d/Increment-3 particle-class work) because it is an
**input-authority** decision that must be settled *before* the particle handoff is wired —
inheriting the single-global `partsize.dat` would bake a MOFE defect into the foundation.

## 5. The hourly-flow substrate (coupled, structural)

Per the operator direction (2026-07-04) and
[`backlog/20260704-hydrograph-resolved-sediment-and-routing.md`](../../../backlog/20260704-hydrograph-resolved-sediment-and-routing.md):
the multi-OFE `qin/qout` accumulation and the reinfiltration/decreasing-flow deposition
are **inherently per-hour**. The current single-peak collapse cannot represent a falling
limb, so the reinfiltration case is the interim INV-030 clamp. Increment 2 should be
designed on the **modeled hourly flow** (`wb14_hourly_excess_m`), not a single peak:

- the down-OFE discharge balance becomes an hour-by-hour `qin/qout` (runon absorbed by a
  downslope OFE is an ordinary hour-resolved reduction ⇒ deposition, retiring the clamp);
- the HBP EVENT payload gains a versioned **hourly-flow surface** so the watershed routes
  the modeled hydrograph instead of a triangular reconstruction (SC-ROUTE-001
  `REF-ROUTE-CH13-PEAKIN`);
- the peak/volume summaries remain (diagnostics, legacy fallback). The conservation
  gate is `Σ hourly = volume` **only**; WB16 `peakro` is a *separate* analytical peak
  estimator (`vave·qpstar`), not the hourly max, so `max(hourly) = peakro` is **not** a
  gate (it would reject/distort a valid hydrograph) — any rescaling to reconcile them is
  an ADR policy choice.

**Entry-gate decision to resolve:** does the Wave-1 continuity solve per-hour (24
solves/OFE-day) or consume a hydrograph-integrated transport? The **acceptance driver**
is the recession/reinfiltration-deposition case — the chosen form must produce
deposition on a falling limb without a clamp, and must close mass per hour. Cost is
bounded (5 classes × ≤24 h × OFEs); profile if it matters, but correctness of the
deposition case governs the form.

## 6. Validation posture (contract-first)

- **Conservation gates (hard, magnitude-independent):** per-OFE mass closure
  (`ΔG = Σ detach − Σ depos` at each OFE), the OFE-boundary handoff identity
  (`G_out(i) = G_in(i+1)`), hillslope-exit closure, and **size-class mass conservation**
  through enrichment (SC-SED-001 §11.5 / `INV-SED-0(08–11)` family). These gate the
  structure regardless of magnitude.
- **Directional / ratio checks:** the DFF-WS3 burn law generalizes to multi-OFE (a
  burned matrix cell exports more than an unburned one); ratios are robust to a uniform
  water-magnitude inflation, so they are valid structural checks *now*.
- **Magnitude is judged last, and is gated on the water magnitude.** Multi-OFE erosion
  magnitude rides on multi-OFE runoff magnitude (rill detachment ∝ discharge). The
  forest-lateral absolute-magnitude authority was promoted (2026-07-02) to a four-tier
  observed envelope (`SC-SUBHYD-001#INV-SUBHYD-033`; WS10/Maimai/Panola/Weiler; judgment
  run MOFEFID-C03), so the water magnitude is now judged against **field data**, not a
  legacy-less gap — but erosion magnitude must still wait until that judgment closes,
  otherwise water error aliases into erosion error (the exact failure the ROADMAP
  ordering principle prevents). p61's ~5× is untangled here (water magnitude vs
  `field_width` vs first-cut operands), not before.

## 7. Staged plan (each stage shadow-first + conservation-hard-stop)

- **2a — single-OFE completeness (Increment 1c-fidelity):** compute the 5-class
  `sedcon` array (the zeroed surface), source `field_width_m` from geometry, and add
  **nonzero-deposition validation coverage** for the already-published `tdep`. Gate:
  single-OFE closure unchanged; p61/DFF-WS3 directional law intact; a depositing fixture
  produces conserving nonzero `tdep`; the now-nonzero `sedcon` conserves per class.
- **2b — hourly-flow substrate:** ADR + SC-SED-001/HBP/SC-ROUTE-001 amendments; add the
  hourly-flow EVENT surface; Wave-1 consumes the hourly discharge (form per §5). Gate:
  `Σ hourly = volume`; single-OFE recession-deposition case produces deposition without
  the INV-030 clamp; non-hourly consumers byte-stable via the peak/volume fallback.
- **2c — multi-OFE Wave-1 chaining:** run Wave-1 per OFE with the `G_out→ldtop`,
  `qout→qin`, particle-fraction handoff (reusing the Wave-2 plumbing). Gate: per-OFE +
  hillslope-exit conservation; EROD14 retained behind a comparator flag for cross-check.
- **2d — enrichment + particle routing (folds Increment 3):** per-OFE base particle-size
  distributions (§4a — `prtcmp` per-element lineage, **not** the single-global
  `partsize.dat`/`usr_partsize` which is MOFE-broken), then class-resolved deposition and
  `enrich.for`-lineage fraction update at deposition/OFE-exit. Gate: size-class mass
  conservation; enrichment ratio ER emerges as a diagnostic; soil-contrasting-OFE fixture
  (§4a) exercises per-OFE sourcing.
- **2e — retire Wave-2 / EROD14** as a separate physics arm (delete or flag-off) once
  Wave-1 multi-OFE is conservation-closed and cross-checked.

*(Increment 3 (particle classes/enrichment) is effectively 2d — deposition cannot be
faithful without the size classes enriching, so the "2 then 3" split in the WP should
merge their entry gates. Customizable class tables + the post-fire ash class remain a
standalone parallel item — see the HR backlog §E — not required for 2c/2d.)*

## 8. Dependencies & risks

- **Water routing must be trustworthy.** Multi-OFE erosion sits on the MOFE water
  routing; it conserves structurally (closed), but the absolute magnitude gap gates
  erosion magnitude. The reinfiltration (`qout < qin`) handling is the shared crux — the
  hourly-flow substrate (2b) is what makes it a clean deposition rather than a clamp.
- **HBP schema is designed once.** The hourly-flow surface must be versioned and stable
  before it serializes across the hillslope/watershed boundary — a premature commit is a
  costly migration (same discipline the HR deposited-layer state carries).
- **Two-physics inconsistency** if Wave-1 and Wave-2 both persist long-term; resolve the
  ownership (§3) early so multi-OFE fidelity is not aliased by which solver ran.
- **Legacy `partsize.dat` is a MOFE gap — do not inherit (§4a).** The per-OFE base
  particle-size distributions must come from each OFE's soil (`prtcmp` lineage); Dennis's
  single-global `usr_partsize` override (`DCF - March 14, 2024`) cannot key per-OFE and
  would corrupt the particle-fraction handoff/enrichment. A user override, if supported,
  is per-OFE-keyed or fail-closed. Foundational to §2d — settle before the handoff is wired.

**Downstream consumer this unblocks:** the watershed sediment hold **`WSHED-W7DC01`**
(`work-packages/20260702-wshedw7dc01-hillslope-sediment-production-hold-lift-001/`) held
because production multi-OFE hillslopes publish **zero HBP sediment on the EROD14 Wave-2
path** even when enabled (substrate `/wc1/runs/in/insensible-aliquot/wepp`). Verified
2026-07-04: the 1b-C single-OFE Wave-1 flip carries nonzero HBP sediment (p61 shard
`tdet=20.9`, `sedcon=[19.08]`) but does **not** touch EROD14, so the multi-OFE path is
still zero. **W7DC01 is a downstream consumer of this increment (E.3), not a
watershed-side defect** — its "before/after pass parquet on a real multi-OFE substrate"
evidence is exactly what E.3 produces. Hand W7DC01 back to W7 fixture adoption once E.3
closes.

## 9. Contract & doc deliverables for kickoff

1. ADR — hydrograph-resolved sediment transport + routing (supersede-vs-coexist,
   comparator tier).
2. SC-SED-001 amendment — erosion consumes hourly discharge; per-OFE + size-class
   conservation invariants for the multi-OFE chain.
3. HBP format + SC-INFILE-HBP-001 — versioned hourly-flow EVENT surface.
4. SC-ROUTE-001 — route the serialized modeled hydrograph; triangular fallback.
5. This entry gate → a kickoff handoff prompt (per the WP template) once the ADR lands.
