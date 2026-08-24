# SC-SURFACELIQUID-001@8 review disposition

Disposition date: 2026-08-23

All findings from the two independent candidate reviews are accepted. The
contract remains `in_review / draft` until both reviewers independently verify
these amendments and the implementation evidence below.

| Finding | Disposition | Resolution |
|---|---|---|
| `A-001`, `B-004` | Accepted / resolved | Replaced the dimensionally invalid cross-OFE scalar Stage-3 parent ledger with a lane-keyed ledger. The complete-owner two-OFE/two-child fixture exercises unequal OFE areas. |
| `A-002`, `B-007` | Accepted / resolved | Restored `last_reviewed: pending`, candidate lifecycle language, and implementation-awaiting-verification gap states. Promotion is deferred until dual verification passes. |
| `A-003` | Accepted / resolved | Added exact coupled-time and Stage-3 cadence invariant anchors. |
| `A-004` | Accepted / resolved | Added the complete ADR-0042 readiness matrix with `PASS` and `NOT_APPLICABLE` dispositions and explicit non-calibratable custody scope. |
| `A-005` | Accepted / resolved | Reordered invariants before producer/consumer obligations and consolidated v8 binding rules into the canonical algorithm, guards, obligations, invariant map, Binding Exposure Index, vectors, and gaps. Later schema sections elaborate those same invariant IDs and do not create a parallel authority. |
| `A-006`, `B-006` | Accepted / resolved | Added runtime aliases, units, provenance, invariant evidence tags, and strict Binding Exposure coverage. |
| `B-001` | Accepted / resolved | Defined proposal-bounded pre-event child, zero-time event, and fresh post-event child chronology with final-only parent cursor publication. |
| `B-002` | Accepted / resolved | Defined `DirectWb14ConfigurationIdentityV1`, canonical preimage ordering, digest ownership, immutable parameter bits, and per-child reconstruction. |
| `B-003` | Accepted / resolved | Defined the seven-owner manifest, auxiliary cursor/receipt lineage, inactive-owner carries, restart wire, and atomic replacement order. Runtime now installs the parent-local surface candidate before snapshot/request/debit/credit/ingress physics. |
| `B-005` | Accepted / resolved | Child receipts bind topology-rank ordered routed-queue before/after digests and replay validates queue eligibility. |

Implementation evidence prepared for verification:

- `Stage3ParentIntegratedBoundaryLedgerV1` is keyed by production lane.
- The V11 complete-owner fixture executes two unequal-area OFEs through two
  accepted 900-second children, routes upper excess into lower runon during
  each child, retains parcel mass/enthalpy lineage, binds ordered per-OFE WB14
  replay, finalizes one parent receipt set, returns seven ending owners, and
  advances the persistent interval once.
- Independent closure replay consumes the sealed scalar authority's exact
  beginning cumulatives and accepted child duration rather than reconstructing
  short children from a stale persistent cursor or a fixed 1,800-second span.
- Focused strict Binding Exposure lint passed (2 rows), the contract-derived
  suite passed 11/11, and the orchestrator suite passed 787/787 with one
  configured skip.

Promotion decision: pending two independent verification passes.
