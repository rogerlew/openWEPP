# Semantic-Parity Direction Assessment (SR07)

Status: `complete`
Evidence mode: `Static + Ran`
Assessment: `HOLD`

Static:
- Semantic parity is the target (not bitwise parity), but Tier-A comparator outcomes remain high-confidence acceptance signals.
- SR06 implementation closure and tests are complete, but SR07 requires Tier-A comparator review for direction validation.

Ran:
- A reproducible Tier-A daily water-balance comparator lane (`H5.wat.dat`) was executed and produced `structure_diff` with early row-level schema divergence.

## Direction Verdict

`UNRESOLVED / HOLD`

Why:

1. The executed comparator lane demonstrates substantial Tier-A divergence (`structure_diff`) on daily water-balance output.
2. The executed lane is legacy-vs-legacy surrogate evidence; it does not provide direct openWEPP-vs-legacy semantic-parity direction evidence.
3. There is no current openWEPP executable/output surface in this workspace that emits comparator-ready Tier-A daily water-balance files.

## Required Follow-up to Clear HOLD

1. Produce an openWEPP candidate Tier-A output surface for the same single-OFE fixture and rerun comparator against pinned baseline.
2. Attach first-divergence decomposition (surface, timestep, contract invariant impact) and classify as resolved vs accepted risk.
3. Re-run SR07 disposition with explicit Tier-A unblock decision.
