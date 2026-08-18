# Independent Rust review

Disposition: `CODE PASS / CAMPAIGN HOLD`.

Reviewed exact working-tree bytes based on
`d1f3094c22bbb2057bf1c8e0925574492be40d84`.

The final review found no remaining material Rust defect. It verified that:

- the positive-area exact-zero-PAR predicate is shared by solve and finalize;
- one-sided differences are limited to the uncapped active V10 branch;
- iteration-zero acceptance is callable only through sealed transaction
  FullSupply classification and complete fixed-final reevaluation;
- potential identity, accepted coordinates, and request batch are private;
- finalization checks batch digest plus transaction and beginning-state
  lineage, with resealed-lineage poisons;
- partial root authorization is typed unsupported while partial ground supply
  retains the ordinary final solve;
- V10 and LSE-V2 configuration receipts are exactly joined;
- the mixed V1/V10 projection API is absent;
- historical V8/V9 dispatch, clone-before-commit rollback, and default-off/no
  selector posture remain unchanged.

Ran evidence includes the focused lineage poison (1/1 PASS), strict affected
Clippy in the pinned Nix toolchain, and the parent exact-byte gate set recorded
in `gate-results.md`.

The review retains two separate campaign blockers: the provider-derived day
rejects at interval 15 in the uncapped potential solve, and V10/LSE-V2 lacks
persisted checkpoint/restore plus restart-equivalence evidence.
