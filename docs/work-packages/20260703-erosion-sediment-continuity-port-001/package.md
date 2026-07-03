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
**INCREMENT-1b IN PROGRESS 2026-07-03 (branch
`erosion-inc1b-operand-production`, awaiting Codex review, NOT on main):
the 1b-A pure-producer subset is landed and gated (the `frcfac`/`shears`,
`prtcmp`/`falvel`/`yalin`/`trcoef`, and `detinr` producers now EXIST in
`direct_runtime/erosion_operands.rs` — superseding this header's original
"no producer exists" premise for those chains); the `effint`/`effdrr`
runtime export + activation-flag wiring (also 1b-A) are HELD, and 1b-B/1b-C
are blocked at the winter freeze-thaw coupling. See the staged status
below and [`artifacts/implementation-1b.md`](artifacts/implementation-1b.md).**
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
    block). Seed stays disabled; production outputs unchanged. See
    [`artifacts/implementation-1b.md`](artifacts/implementation-1b.md).
  - **1b-B — daily erodibility adjustments: BLOCKED at a confirmed
    stop-condition.** The `soil.for` chain is portable (consolidation
    via `scon` baselines + `rfcum`/`daydis` accumulators; cover/root/
    residue factors from the growth/decomposition state; slope factor)
    EXCEPT the freeze-thaw factors, whose `ifrost == 2` thawing branch
    needs the winter `fcycle` cycle counter — **absent from the direct
    runtime** (0 occurrences; produced only in the legacy winter
    subsystem `watdst.for`/`wshdrv.for`). Recommended: port the portable
    chain and fail-closed the thaw branch; surfacing `fcycle`/`frara` is
    a winter-subsystem WP (out of the erosion write set).
  - **1b-C — activation: gated behind 1b-B + the winter coupling.** Not
    started; the seed flip cannot be safe generally until the freeze-thaw
    branch is either produced or fail-closed. Seed remains disabled.
- **Increment 2 — multi-OFE integration.** Wire Wave-1 across OFEs and reconcile
  with the existing EROD14 routing (`qin/qout`, particle handoff).
- **Increment 3 — particle-class + enrichment completeness** (`sedia`/`sedist`)
  and per-class concentration/fraction publication.

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
