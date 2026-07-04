# Erosion Sediment-Continuity Direct-Runtime Port (SC-SED-001, Wave-1 first)

Status: **INCREMENT-1 MERGED TO MAIN, 2026-07-03 (`12038179`,
operator-directed merge after Codex review: round-1 High
activation-ordering finding fixed and re-verified, round-2 merge-ready).
The normalized single-OFE Wave-1 continuity solver
(route/erod/runge/xcrit/depc/depeqs/depend/depos + param/xinflo/sloss),
shadow-first runtime wiring, publication projection, and the hard
conservation gates are live and validated (15 unit tests + the McKenzie
clay-loam fixture-forcing conservation test, real storm forcing,
nonzero detachment proven). SCOPE DEVIATION (declared boundary): the
production seed enable is HELD — the Increment-0 premise that the Wave-1
operand projection exists was struct-fields-only; no producer exists for
the `frcfac`/`shears`, `soil.for` daily-adjustment, `irs` `effint`, or
`prtcmp`/`yalin`/`trcoef` operand chains, and fabricating them would
violate the no-provisional-math rule. Operand production is
**Increment-1b** (queue in `artifacts/implementation.md`); flipping the
seed activates the solver unchanged — validated for the dry-day shape
after the Codex round-1 finding (activation gates now precede the
routed-operand validator, matching the legacy `contin.for` ordering;
regression-tested at solver and frame/r7d8 level). Production outputs
are unchanged; the DFF-WS3 sediment HOLD stands until the 1b activation.
**INCREMENT-1b-A (pure-producer subset) MERGED TO MAIN 2026-07-03
(`820377eb`, operator-directed merge after Codex review: round-1
truthfulness + fail-closed findings fixed, round-2 no blocking findings,
merge-ready). The `frcfac`/`shears`, `prtcmp`/`falvel`/`yalin`/`trcoef`,
and `detinr` producers now EXIST in
`direct_runtime/erosion_operands.rs` (superseding this header's original
"no producer exists" premise for those chains).**
**INCREMENT-1b-A-lift + 1b-B-portable MERGED TO MAIN 2026-07-03
(`bc4fd660`, operator-directed after Codex 2-round review: round-1 Medium
consolidation-baseline fail-closed hole fixed, round-2 stale-count Low
fixed). Landed: the faithful `effint`/`effdrr` producer
(`erosion_effective_intensity`) and the 1b-B portable adjustment
producers (`direct_runtime/erosion_adjustments.rs`: `scon` baselines +
`soil.for` subfactor chain → `kiadjf`/`kradjf`/`tcadjf`, `ifrost==2` thaw
branch fail-closed). Both pure/typed/fail-closed behind the disabled
seed. STILL OPEN: the 1b-C production flip (per-day operand threading +
`daydis`/`rfcum` accumulators + `effint`/`effdrr` WB14 surfacing + enable
+ pass-writer unhardcode + DFF-WS3 HOLD flip + full-run byte-stability),
held with a no-intervention spec in
[`artifacts/increment-1bc-flip-scope-and-hold.md`](artifacts/increment-1bc-flip-scope-and-hold.md);
the winter `fcycle` coupling inside 1b-B's thaw branch stays a
winter-subsystem prerequisite (out of the erosion write set). The
production seed stays disabled; production outputs are unchanged.**
Governing authority:
[`SC-SED-001`](../../specifications/science-contracts/contracts/SC-SED-001.md)
(Hillslope Erosion Process Contract, v41, 56 invariants);
[ADR-0035](../../decisions/0035-hillslope-erosion-sediment-continuity-port.md)
(program ratification); [ADR-0024](../../decisions/0024-reference-implementation-intent-authority.md)
(source-intent authority); [ADR-0017](../../decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md) legacy-as-flag.
Scope basis: [`docs/backlog/20260703-hillslope-erosion-sediment-continuity-port.md`](../../backlog/20260703-hillslope-erosion-sediment-continuity-port.md).
Owner/executor: **Claude Code** (operator-authorized, ADR-0035). This is a
**re-port** to source-intent, not a new derivation.

## Objective

Make the direct runtime a hillslope sediment producer by implementing the
`SC-SED-001` sediment-continuity **source physics** — currently absent (Wave-1
detachment is a disabled single-point check; only multi-OFE routing runs). Land
it staged, shadow-state-first, conservation-gated, single-OFE Wave-1 first.

## The record this rests on (git-confirmed)

- **Current state:** `direct_runtime/erosion.rs::compute_direct_erod13` is a
  single per-OFE-day coefficient evaluation (`η/τcn/θ/φ/dc/tc/df`) that validates
  a supplied `dgdx`; it does not evolve `G`, and `wave1_enabled = false`
  (`00_builders_and_authority.rs:942`). Wave-2 (EROD14) is the multi-OFE router,
  `ofe_count > 1` only.
- **Legacy source-intent** (`REF-SED-LEGACY-*`, baseline `dac3c950`):
  `route`→`erod`→`runge` spatial continuity per OFE; `xcrit`/`mshear` (1..5)
  regime classification; `depc`/`depend`/`depos` deposition-region solves;
  `sedia`/`sedist` particle classes.
- **Reference kernels** (git-recoverable, the fuller prior port, still RK-less):
  `a381702b^:…/hydrology_phase_erod13.rs`, `…/hydrology_phase_erod19.rs`
  (the `xcrit`/`mshear` classifier), `…/erod13_wave1_core_kernel_contract.rs`.

## Increments (staged; each shadow-first + conservation-hard-stop)

- **Increment 0 — entry gate + design.** Read the legacy `route`/`erod`/`runge`/
  `xcrit`/`depc`/`depend`/`depos` chain and the SC-SED-001 Wave-1 invariants;
  resolve the continuity mechanism (legacy analytic per-segment vs numerical
  march) against the contract + source-intent, and record it. Map the exact
  operand lineage the direct frame must supply. Record in `artifacts/entry-gate.md`.
- **Increment 1 — single-OFE Wave-1 detachment/deposition.** Implement the
  spatial continuity + `xcrit` regime dispatch + deposition-region solve for a
  single OFE; enable `wave1_enabled` for single-OFE; produce detachment,
  deposition, sediment load, and the HBP `EVENT` payload. Hard gate: mass
  conservation `Σ detach − Σ deposition = exported sediment` + the SC-SED-001
  invariants, on a runoff+sediment-generating fixture (McKenzie Bridge class,
  NOT MORAN-WY p313).
  **DONE 2026-07-03 except the production seed flip** (solver + wiring +
  publication + conservation gates landed and validated on real McKenzie
  clay-loam storm forcing; see `artifacts/implementation.md`).
- **Increment 1b — Wave-1 operand production + activation (SCAFFOLDED
  2026-07-03; declared boundary from Increment-1).** Design + full
  operand→producer lineage map in
  [`artifacts/increment-1b-entry-gate.md`](artifacts/increment-1b-entry-gate.md);
  executor prompt in
  [`artifacts/increment-1b-handoff-prompt.md`](artifacts/increment-1b-handoff-prompt.md).
  Three gated stages, production flip last:
  - **1b-A — event/transport operands (no new daily state): PURE-PRODUCER
    SUBSET DONE + GATED; remaining runtime surfaces HELD** (branch
    `erosion-inc1b-operand-production`, awaiting Codex review). Landed
    (production, typed, fail-closed, in `direct_runtime/erosion_operands.rs`,
    16 unit tests + the `erod16` fixture test swapped onto them):
    `prtcmp`/`falvel` particle classes + `veleff`; `shield`/`yalin`/
    `trcoef` → `kt/kt2/ktrato/tcend`; `frcfac`+`shears` rill hydraulics
    (`shrsol`/`shrend`, Gilley width); cropland/non-cropland interrill
    delivery; `detinr`. **HELD (in-scope-1b-A but not delivered — this is
    why the stage is a subset, not complete):** the `effint`/`effdrr`
    runtime export (a WB14/WB16 excess-surface integration, not a pure
    producer) and the activation-flag wiring (`beta`/`surface_frozen`
    trivial; `theta_suppressed`'s `frara` melt-branch shares the winter
    block). Seed stays disabled; production outputs unchanged. Merged to
    main `1887092b`. See
    [`artifacts/implementation-1b.md`](artifacts/implementation-1b.md).
  - **1b-A hold-lift — faithful `effint`/`effdrr` producer: DONE + GATED**
    (branch `erosion-inc1bc-activation`). `erosion_effective_intensity`
    (`reid.for`): `effdrr = durre`, `effint = sumint/durre` (mean
    **rainfall** intensity over excess periods, snowmelt excluded) — the
    faithful form of the operand the `erod16` test approximated as
    `runoff/effdrr` (which understated the interrill `Di = Ki·I·q`
    driver). 4 unit tests. The **export** (making it observable) is
    coupled to the flip — see 1b-C.
  - **1b-B — daily erodibility adjustments: PORTABLE PRODUCERS DONE +
    GATED; thaw branch fail-closed** (branch `erosion-inc1bc-activation`).
    `direct_runtime/erosion_adjustments.rs`: `scon.for` consolidation
    baselines (`kicrat`/`krcrat`/`tccrat`) + the `soil.for` cropland
    subfactor chain → `kiadjf`/`kradjf`/`tcadjf` (cover/root/residue,
    sealing via `produc`, slope factor, floors/cap), pure + typed +
    fail-closed, 9 unit tests. The **actively-thawing** (`ifrost == 2`)
    branch is **fail-closed** (typed error naming the missing winter
    `fcycle` counter — 0 occurrences in the direct runtime). The
    **stateful** parts (the `daydis`/`rfcum` accumulators + the
    prior-`ifrost` frost-regime resolution) are runtime wiring, part of
    the flip.
  - **1b-C — activation: IN EXECUTION 2026-07-03 (staged, branch
    `erosion-inc1c-flip`).** Operator-directed after the
    integration-surface audit (byte-stability risk LOW — pure consumer;
    most daily inputs frame-reachable). Stages: (1) static erosion
    operand seed; (2) `daydis`/`rfcum` accumulators + `wb14_hourly_rainfall`
    surface + frost-regime resolution; (3) per-day operand assembly in
    r7d8; (4) enable single-OFE + pass-writer unhardcode + DFF-WS3 HOLD
    flip + full-run byte-stability. Each stage gated. The seed flip
    remains a distinct stateful integration (per-day operand threading +
    new persistent daily accumulators + `effint`/`effdrr` WB14 surfacing +
    enable + pass-writer unhardcode + DFF-WS3 HOLD flip + full-run
    byte-stability), previously held rather than forced in a single pass:
    the winter `fcycle` coupling is a confirmed hard boundary inside it
    (bounded by the 1b-B fail-closed thaw guard, which makes the enable
    safe by construction), and the byte-stability gate needs a full
    shadow-run diff. Complete no-intervention spec in
    [`artifacts/increment-1bc-flip-scope-and-hold.md`](artifacts/increment-1bc-flip-scope-and-hold.md).
    Seed remains disabled; production outputs unchanged.
- **Increment 1c-fidelity — single-OFE surface completeness (structural, hard
  prerequisite for Increment 2).** Publish `tdep` deposition, the 5-class `sedcon`
  concentration (both 0 in the 1b-C first cut), and a real `field_width_m` from
  hillslope geometry — the surfaces multi-OFE routing must transport.
- **Increment 2 — multi-OFE integration.** Wire **Wave-1** across OFEs as the
  per-OFE continuity engine (`G_out→ldtop` load, `qout→qin` discharge, particle
  handoff), reusing the existing EROD14/Wave-2 routing plumbing, then retire Wave-2
  as a separate physics arm. Coupled to the **hourly-flow substrate** (the
  decreasing-flow/`qin<qout` deposition is per-hour) — carry the modeled hourly
  flow through erosion → HBP → routing instead of the single-peak collapse. Full
  architecture + open-decision resolution in
  [`artifacts/increment-2-entry-gate.md`](artifacts/increment-2-entry-gate.md);
  cross-cutting substrate concept in
  [`docs/backlog/20260704-hydrograph-resolved-sediment-and-routing.md`](../../backlog/20260704-hydrograph-resolved-sediment-and-routing.md).
- **Increment 3 — particle-class + enrichment completeness** (`sedia`/`sedist`)
  and per-class concentration/fraction publication. Folds into Increment 2d
  (deposition is not faithful without the size classes enriching) — merge the entry
  gates. Erosion **magnitude fidelity** is judged last, gated on the water-magnitude
  contract-gap (roadmap §E.5).

## Validation posture

Contract-first (SC-SED-001 invariants + conservation closure); legacy `.for` is
source-intent authority (ADR-0024), never a magnitude oracle (ADR-0017). Each
increment: shadow state validated to conservation before it becomes the
production authority.

## Non-negotiables

- Fail-closed typed guards; no provisional/proxy sediment math in the production
  path.
- Full AGENTS.md gates before an increment is merge-ready: `cargo fmt --check`,
  `clippy -D warnings`, `nextest --profile full`, `cargo deny`, `git diff --check`,
  the authority-suite anti-evasion check, SC-unit lint. Line-count governance
  (2000 WARN / 3000 BLOCK).
- Push branch for Codex review before merge; do not self-merge to main.

## Line-count governance dispositions

- **`crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
  crossed the 2000-line WARN threshold (1941 → ~2105) in Stage 3a**, from
  the `direct_production_wave1_operand_seed` static-seed builder. WARN, not
  BLOCK (< 3000). Disposition: **accepted for now, extraction deferred.**
  The seed builder is self-contained (its only in-module couplings are the
  PL/WB16 projection accessors and `SIMOUT_GUARD_ID`) and is a clean
  extraction target; this builders file is already in scope for the
  `openwepp-runner-hillslope` modularization family (refactor007-class).
  Splitting it now would be an unrelated mechanical refactor mid-flip; the
  erosion seed/assembly wiring is extracted as a follow-up once Stage 3
  lands, keeping the flip diff reviewable.
