# PL08 Semantic-Parity Direction Assessment

Status: `complete`
Evidence mode: `Static + Ran`
Assessment: `PARTIAL-POSITIVE SIGNAL / HOLD`

Static:
- Semantic parity (not bitwise parity) remains the project target.
- Tier-A comparator surfaces are high-confidence acceptance-direction signals.

Ran:
- `H5.wat.dat` strict comparator: `structure_diff`.
- Shared keyed daily fields (`1..20`) are exactly aligned across `1095` `(OFE,J,Y)` rows.
- Plant/residue proxy fields `Ep`, `Es`, `Er` are exact matches; `H5.plot.dat` is strict-identical.

## Direction Verdict

`UNRESOLVED / HOLD`

Why:

1. Strict Tier-A comparator outcome for daily water-balance remains unresolved (`structure_diff`).
2. Shared-field parity evidence is useful but still surrogate and non-authoritative for final Tier-A closure.
3. Direct openWEPP-vs-legacy Tier-A comparator run is still unavailable in this workspace.

## Required Follow-up to Clear HOLD

1. Emit openWEPP comparator-ready Tier-A daily water-balance output for the same fixture.
2. Re-run strict Tier-A comparator vs pinned legacy baseline.
3. Classify remaining deltas with explicit invariant/contract impact and close blocker decision.
