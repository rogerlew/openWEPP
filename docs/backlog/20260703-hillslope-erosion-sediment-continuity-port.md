# Hillslope erosion: sediment-continuity direct-runtime port

State: `concept (scoped)` — foundational gap; **blocks the WS-3 sediment
ordering law** and hillslope sediment fidelity generally. Contract-exists /
implementation-missing (like WS-2 `ksatadj`). Author: Claude Code, 2026-07-03.

## The gap (git-confirmed, Static)
openWEPP's direct-runtime erosion is **not a sediment producer**. Confirmed by
source + git archaeology:

- **Wave-1 (EROD13)** — `direct_runtime/erosion.rs::compute_direct_erod13` is a
  **single per-OFE-day** evaluation of the continuity coefficients
  (`η`, `τcn`, `θ`, `φ`, `dc`, `tc`, `df`) that **validates a supplied `dgdx`
  against the local flux balance** (`net_detachment + Di`); it does **not**
  evolve `G` along the OFE. There is no spatial march, no detachment/deposition
  regime switching, no crossing-point geometry, no deposition-region solve. And
  it is **hard-disabled** in production (`00_builders_and_authority.rs:942`
  `wave1_enabled = false`, inputs zeroed).
- **Wave-2 (EROD14)** — the **multi-OFE profile router** only (`qin/qout`
  handoff, per-class flow fractions, `qostar`, enrichment ratio); enabled only
  for `ofe_count > 1` (`erod14_wave2_enabled = ofe_count > 1`).
- **Net:** single-OFE hillslope → **no erosion at all**; multi-OFE → routing of
  sediment the (disabled, non-spatial) source kernel never produced.

The legacy is far more (`REF-SED-LEGACY-*` in SC-SED-001): `route`→`erod`→
`runge` marches ~101 points per OFE integrating `dG/dx = Df + Di`, with
`xcrit` (`mshear` case 1..5) regime classification and `depc`/`depend`/`depos`
analytic deposition-region solves — **per OFE, single-OFE included**.

**The spatial solve never existed in openWEPP** (no `runge`/`kutta` anywhere in
history). The symbol-map lane (deleted `a381702b`) had a *fuller* set —
`erod13`(426) + `erod14`(1001) + **`erod19`(1141, the `xcrit`/`mshear`
crossing-point classifier)** + a Wave-1 core kernel contract(500) — but even it
had no RK integrator, and only `erod13`(reduced) + `erod14`(routing) were
re-ported to the direct lane. `erod19` (regime/crossing control) was dropped.
This matches the operator's recollection: the sediment routing was built but the
detachment/deposition continuity was never validated or fully implemented.

## Contract state — already authored (implement to it, don't re-derive)
`SC-SED-001` (Hillslope Erosion Process Contract, v41, `in_review`, **56
invariants**) is comprehensive: sediment continuity + `Di`/`Df`/`G` signs
(REF-SED-CH11-CONT), rill detachment/deposition branches (CH11-DET/DEP),
normalized `η/τcn/θ/φ` (CH11-NORM), size-class enrichment (CH11-ENRICH), and the
full legacy authority chain — `erod.for`, `runge.for`, `contin.for`/`route.for`,
`xcrit.for` (mshear 1..5), `depc.for` (deposition partial solution), `depend.for`
(where deposition ends, `xdend`), `depos.for` (segment deposition profile),
`sedia`/`sedist`. HBP boundary payloads are specified (REF-SED-HBP-FORMAT:
`total_detachment_kg`, `total_deposition_kg`, `sediment_concentration_kg_m3[npart]`,
`particle_flow_fraction[npart]`). So this is **contract-exists / implement**, like
WS-2 `ksatadj`.

## Scope of the port (large — a staged program, not one WP)
Implement the `SC-SED-001` spatial sediment continuity in `direct_runtime`:
1. **Wave-1 detachment, enabled + spatial.** Evolve `G` along each OFE with the
   detachment-region continuity and the `xcrit`/`mshear` regime classification
   (the dropped `erod19` logic). **Design decision to resolve first:** port the
   legacy per-segment *analytic* approach (`xcrit` case + `depc`/`depend`
   closed-form deposition) vs a numerical march — SC-SED-001 references both the
   `runge` continuity form and the analytic deposition solves; pick the one the
   contract makes authoritative and note it.
2. **Deposition-region solve.** `depc`/`depend`/`depos` — where deposition begins
   and ends within a segment, and the deposition profile (`detach`, `tc`,
   `load`) under increasing/decreasing flow.
3. **Particle size-classes + enrichment.** `sedia`/`sedist` — size-class mass
   conservation, differential settling/deposition, enrichment ratio (partly in
   the existing EROD14).
4. **Single-OFE erosion.** Remove the `ofe_count > 1` gate for the *source*
   physics; a single-OFE hillslope must produce detachment/deposition/sediment.
5. **Wiring + closure.** Feed the HBP `EVENT` sediment payload; assert mass
   conservation (`Σ detach − Σ deposition = exported sediment`) and the SC-SED-001
   invariants; publish per-class concentrations/fractions.

## Dependencies + relationships
- **Blocks WS-3's sediment ordering law** (burned > unburned sediment). WS-3
  runoff + peak laws are unaffected (hydrology / WS-2 `ksatadj`). Until this
  lands, WS-3 should either **scope sediment out** (validate runoff + peak, defer
  sediment) or take this port as an explicit prerequisite.
- Related backlog: `20260526-hairsine-rose-multiclass-sediment-model.md`
  (alternative multi-class model — decide relationship: port WEPP-native first,
  Hairsine-Rose is a separate science direction).
- `SC-ROUTE-001` owns channel/watershed sediment routing (out of scope here —
  this is the hillslope producer).

## Guardrails
- Contract-first: implement to `SC-SED-001` invariants; the legacy `.for` files
  are source-intent authority (ADR-0024), not a parity-magnitude oracle
  (ADR-0017). Validate the laws + conservation, not legacy sediment magnitudes.
- Fail-closed typed guards; no provisional/proxy sediment math in the production
  path (AGENTS.md).
- The prior `erod19`/xcrit and Wave-1 core-kernel implementations are
  git-recoverable at `a381702b^` as a **reference** (they were the fuller,
  though still RK-less, prior port), the way the deleted `ksatadj` kernel seeded
  WS-2.

## Promotion criteria (before opening a work-package)
- Resolve the analytic-vs-numerical continuity decision against SC-SED-001.
- Confirm a **runoff+sediment-generating** validation fixture exists (Hortonian
  runoff on unfrozen ground — McKenzie Bridge class, NOT MORAN-WY p313, which is
  erosion-inert). WS-3's `clay_loam` anchor is a candidate but is single-OFE, so
  it exercises Wave-1 only (fine — Wave-1 is the missing piece).
- Decide staging: single-OFE Wave-1 detachment/deposition first (closes the
  biggest gap + unblocks single-OFE sediment), then multi-OFE integration.
