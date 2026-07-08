# Worker Handoff

Status: `READY-FOR-FOLLOW-ON`
Evidence mode: Static.

## Current State

`SC-OFEROUTE-001` rev 45 and code now make active production Lane D routing
default to target `dx = 5.0 m` with the retained 10-cell floor, 4096-cell cap,
and 300 s active max-substep cap.

The dx5 production default is proven by runtime no-env evidence and output
identity against explicit diagnostic `dx5` runs.

## Do Not Reopen Without New Evidence

- Do not widen routed-shape tolerance in place.
- Do not widen annual sediment thresholds in place.
- Do not use H2637 synthetic timing as fleet promotion evidence.
- Do not revive abandoned hybrid/implicit stepping without a new ADR and
  contract-first package.
- Do not silently change shadow mesh; it remains a separate diagnostic
  surface at fixed `10` cells.

## Recommended Follow-Ons

1. D15A watershed-facing publication closure.

   Close the remaining non-silent gates named in `docs/ROADMAP.md`:
   watershed-facing HBP outlet re-pointing and active-mode erosion
   water-magnitude coupling. Prove the downstream consumer reads the active
   path.

2. Tier-1-class local numerics/cost work on the fine active mesh.

   Runtime cost is no longer a promotion blocker, but it is material:
   selected real-cohort aggregate user-time ratio is
   `dx5/fixed10 = 4.877887788778878`. Optimize only with byte-preserving or
   contract-authorized numerics.

3. Optional direct orchestrator API cleanup.

   `DirectLanedActiveMeshPolicy::FixedCells` remains available as an internal
   enum variant, but the production runner default no longer constructs it.
   If this becomes confusing for downstream API users, handle retirement or
   quarantining in a separate contract-aware cleanup package.

## Reproduction Pointers

- Promotion matrix: `artifacts/rev44-promotion-matrix.json`
- Runtime evidence: `artifacts/default-dx5-evidence.json`
- Runtime harness: `artifacts/run_default_dx5_evidence.py`
- Analyzer: `artifacts/analyze_dx5_production_matrix.py`
- Gate ledger: `artifacts/gate-results.md`
