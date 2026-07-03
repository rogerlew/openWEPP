# Erosion Sediment-Continuity Direct-Runtime Port (SC-SED-001, Wave-1 first)

Status: **SCAFFOLDED, 2026-07-03 — foundation ratified (ADR-0035); Increment-1
(single-OFE Wave-1) in execution by Claude Code.**
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
