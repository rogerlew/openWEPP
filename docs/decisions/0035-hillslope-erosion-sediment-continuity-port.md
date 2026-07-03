# ADR-0035: Hillslope Erosion Sediment-Continuity Direct-Runtime Port

Status: **Accepted** (ratified 2026-07-03 by operator direction; Roger Lew
directed promotion + execution). Executor exception: **Claude Code authors this
port's code** — an operator-authorized departure from the CLAUDE.md
Codex-authors-code default, on the pattern established for FARPOINT01
(operator: "let's try having you run the work-package"). The code remains
subject to the normal AGENTS.md validation gates and Codex review.

Deciders: Roger Lew, Claude Code

Ratification provenance:
`docs/backlog/20260703-hillslope-erosion-sediment-continuity-port.md` (scoped
gap analysis + git archaeology), `SC-SED-001` (the pre-authored erosion
contract), and the WS-3 hold (Codex closed WS-3 to `hold` 2026-07-03 because the
sediment ordering law is unimplementable on the current path).

Relates: `SC-SED-001` (Hillslope Erosion Process Contract — authority),
`SC-ROUTE-001` (channel/watershed sediment routing — downstream boundary),
`ADR-0011` (architecture-first, contract-first), `ADR-0017` (legacy-as-flag, not
a magnitude oracle), `ADR-0024` (reference-implementation source-intent
authority), `ADR-0025` (array-native direct runtime — the lane this ports into),
`SC-SUBHYD-001`/`INV-SUBHYD-032` (WS-2 `ksatadj`, the sibling soil-side port).

## Context

openWEPP's direct-runtime hillslope erosion is not a sediment producer. Source
inspection + git archaeology (recorded in the backlog scope) establish:

- **Wave-1 (EROD13)** — `direct_runtime/erosion.rs::compute_direct_erod13` is a
  single per-OFE-day evaluation of the continuity coefficients
  (`η`, `τcn`, `θ`, `φ`, `dc`, `tc`, `df`) that **validates** a supplied `dgdx`
  against the local flux balance; it does not evolve sediment load `G` along the
  OFE, and it is **hard-disabled** in production (`wave1_enabled = false`).
- **Wave-2 (EROD14)** — the multi-OFE profile router only (`qin/qout` handoff,
  per-class flow fractions, enrichment), enabled only for `ofe_count > 1`.
- **Net:** single-OFE hillslope → no erosion; multi-OFE → routing of sediment the
  (disabled, non-spatial) source kernel never produced.

The legacy is a spatial steady-state sediment-continuity solve per OFE
(`route`→`erod`→`runge`, with `xcrit`/`mshear` regime classification and
`depc`/`depend`/`depos` deposition-region solves), run for **every OFE including
single-OFE**. The spatial solve **never existed in openWEPP** (no `runge`/`kutta`
in any history). The symbol-map lane held a fuller set — `erod13`/`erod14`/**`erod19`
(the `xcrit` crossing-point classifier)** + a Wave-1 core contract — deleted at
`a381702b`; only the reduced `erod13` + routing `erod14` were re-ported. The
sediment *routing* was built; the sediment *source physics* never was — matching
the operator's recollection that the routings were built but never fully
validated.

Decisively: **`SC-SED-001` (v41, `in_review`, 56 invariants) already specifies
the full model** — continuity + `Di`/`Df`/`G` signs, detachment/deposition
branches, the `η/τcn/θ/φ` normalization, size-class enrichment, the HBP sediment
payload, and the complete legacy source-intent chain (`erod`, `runge`,
`contin`/`route`, `xcrit`, `depc`, `depend`, `depos`, `sedia`/`sedist`). This is
therefore a **contract-exists / implement** effort, structurally identical to the
WS-2 `ksatadj` port, not a contract derivation.

## Decision

Implement the `SC-SED-001` hillslope erosion sediment-continuity **source
physics** in the `direct_runtime` production lane, contract-first, with the
legacy `.for` routines as source-intent authority (ADR-0024) — not as a
magnitude oracle (ADR-0017). Specifically:

1. **Enable and spatialize Wave-1.** Evolve sediment load along each OFE with the
   detachment-region continuity and the `xcrit`/`mshear` regime dispatch (the
   dropped `erod19` logic), plus the `depc`/`depend`/`depos` deposition-region
   solves. The exact continuity mechanism (legacy analytic per-segment solution
   vs numerical march) is resolved **in Increment-1 against `SC-SED-001` and the
   source-intent**, and recorded; the program commits to source-intent fidelity
   + conservation closure, not to a specific numerical scheme a priori.
2. **Erosion runs for single-OFE.** Remove the `ofe_count > 1` gate for the
   source physics; a single-OFE hillslope must produce detachment/deposition/
   sediment (as legacy does).
3. **Particle size-classes + enrichment** (`sedia`/`sedist`) and the HBP `EVENT`
   sediment payload, with mass-conservation closure
   (`Σ detach − Σ deposition = exported sediment`) as a hard gate.

**Staging** (per the staged-increment port template): **single-OFE Wave-1
detachment/deposition first** (closes the largest gap, unblocks single-OFE
sediment), then multi-OFE integration reusing the existing EROD14 routing, then
particle-class/enrichment completeness. Each increment is **shadow-state-first**
and **conservation-hard-stopped** before it becomes an authority.

## Consequences

- **Unblocks** hillslope sediment fidelity generally and the WS-3 sediment
  ordering law specifically (WS-3 stays `hold` until at least Increment-1 lands;
  WS-3's runoff + peak laws are independent and may proceed separately).
- **Large, multi-increment program** touching the most intricate WEPP physics
  (regime-switching continuity, deposition geometry, particle settling). Risk is
  managed by contract-first invariants + per-increment conservation gates + the
  git-recoverable `a381702b^` `erod19`/Wave-1 kernels as a reference (as the
  deleted `ksatadj` kernel seeded WS-2).
- The direct runtime's erosion authority (`wave1_enabled`, the `ofe_count > 1`
  gate) changes semantics; the existing EROD14 routing and its guards are reused,
  not discarded.

## Alternatives considered

- **Keep erosion disabled / defer indefinitely** — rejected: sediment is a core
  WEPP output and the gap silently mis-represents erosion as zero.
- **Adopt the Hairsine-Rose multiclass model** (`docs/backlog/20260526-…`) —
  deferred: a distinct science direction; port the WEPP-native SC-SED-001 model
  first, evaluate alternatives separately.
- **Hand off to Codex** (the default) — the operator elected Claude Code as
  executor for this port.
