# WS11 Contract Implementation Evidence

Status: `completed`
Evidence mode: `Static`

## Static
- Scope
  - Implemented WS11 Phase A canonical contract amendments for channel-routing
    physics equivalence.
  - Replaced WS10 gain-factor surrogate authority with explicit legacy-equivalent
    `ipeak` branch authority (Rational, CREAMS, kinematic-wave, Muskingum-Cunge)
    in canonical `SC-*` contracts.
- Contract files amended
  - `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
  - `docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
  - `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
  - `docs/specifications/science-contracts/index.md`
- WS11 contract changes
  - `SC-ROUTE-001`
  - Added pinned baseline provenance anchors for WS11 routing lineage:
    `wshcqi`, `wshdrv`, `wshpek`, `wshchr` from
    `/workdir/wepp-forest_260430_baseline` at
    `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
  - Expanded routing invariants to explicit `ipeak` branch exclusivity and
    routed closure semantics:
    - `INV-ROUTE-006`: method-branch exclusivity (`1` Rational, `2` CREAMS,
      `3` kinematic wave, `>=4` Muskingum-Cunge).
    - `INV-ROUTE-007`: threshold/routed closure behavior split across
      `ipeak <= 2` vs `ipeak >= 3`.
  - Added `WS11 Channel-Routing Physics Equivalence Addendum` with runtime
    symbols, legacy-equivalent routing steps, surrogate deauthorization,
    typed guard continuity (`WKERNEL-WS10-CHANNEL-E-001..003`), and
    contract-derived vectors.
  - Added `GAP-ROUTE-006` (`promotable-with-risk`) as a lineage-note row for
    companion documentation follow-up.
  - `SC-HYDRAULICS-001`
  - Added `REF-HYD-WS11-ROUTE` and `WS11 Channel-Routing Consumer Coupling
    Addendum`.
  - Required consumer preservation of explicit `ipeak` route-branch provenance.
  - Explicitly prohibited collapse to pre-WS11 gain-factor surrogate routing.
  - Preserved existing channel guard-family continuity:
    `WKERNEL-WS10-CHANNEL-E-001..003`.
  - `SC-SYSTEM-001`
  - Added baseline routing authority anchors:
    `REF-SYSTEM-WSHDRV-ORDER`, `REF-SYSTEM-WSHPEK-IPEAK`,
    `REF-SYSTEM-WSHCHR-WAVE`.
  - Expanded integration invariants for threshold/routed-gating and explicit
    `ipeak` branch exclusivity:
    `INV-SYSTEM-005`, `INV-SYSTEM-006`.
  - Added `WS11 Channel-Routing Physics-Equivalence Integration Addendum`
    and explicit surrogate deauthorization in system integration rules.
  - `science-contracts/index.md`
  - Updated registry summaries for `SC-ROUTE-001`, `SC-HYDRAULICS-001`, and
    `SC-SYSTEM-001` to reflect WS11 authority closure and preserved WS12
    context.
- Version bumps
  - `SC-ROUTE-001`: `9 -> 10`
  - `SC-HYDRAULICS-001`: `10 -> 11` (WS12 `10` baseline preserved)
  - `SC-SYSTEM-001`: `13 -> 14` (WS12 `13` baseline preserved)
- Sequencing compliance
  - WS11 output in this artifact is Phase A contract authority only.
  - No production kernel code edits were made as part of this update.

## Ran
- Not run (Phase A contract-authority update only; no runtime/test execution in
  this artifact).
