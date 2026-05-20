# ARCH-01 Disposition

Status: complete
Date: 2026-05-20 UTC
Outcome code: `GO_ARCHITECTURE_DISCOVERY_COMPLETE`
Evidence mode: `Static`
Ran evidence: none in ARCH-01 closeout

## Summary
ARCH-01 is closed as complete. The package produced subsystem boundaries,
state-surface ownership, invariant cataloging, comparator confidence-tier
policy, and a legacy `.run` + `.txt` sidecar compatibility bridge definition
aligned to ADR-0011.

## Exit Criteria Check
- [x] Required architecture artifacts are present and populated.
- [x] Subsystem boundaries and dependency map are explicit.
- [x] State-surface catalog includes ownership + tiered acceptance context.
- [x] Comparator tier policy is documented.
- [x] Follow-on implementation sequence is defined.

## Queued Follow-On Work Packages
1. `20260520-arch02-input-contract-and-typed-state-skeleton`
   - Scope: SS-01 + SS-02 scaffolding, including dual-mode `.run` bridge and
     typed state model baseline.
2. `20260520-arch03-routine-interface-and-kernel-lifecycle-skeleton`
   - Scope: SS-05 routine descriptor validation and lifecycle wiring.
3. `20260520-arch04-hillslope-orchestrator-and-hbp-skeleton`
   - Scope: SS-03 execution shell and HBP boundary wiring (no physics).
4. `20260520-arch05-tier-a-invariant-gate-slice`
   - Scope: SS-06 one-surface Tier-A daily water-balance invariant checks.
5. `20260520-arch06-replay-comparator-tier-routing-skeleton`
   - Scope: SS-07 tiered comparator disposition metadata path.

## Risks Carried Forward
- Chapter-level invariant extraction/citation normalization from
  `references/50201000` remains partially complete.
- ADR-0008 and ADR-0009 are still `Proposed`; implementations should track
  potential changes before relying on those semantics as final.
- No `Ran` evidence in ARCH-01; executable gates remain future work.
