# SC-SURFACELIQUID-001@8 review disposition

Disposition date: 2026-08-23

All findings from the two independent candidate reviews are accepted. The
contract remains `in_review / draft` until both reviewers independently verify
these amendments and the implementation evidence below.

| Finding | Disposition | Resolution |
|---|---|---|
| `A-001`, `B-004` | Accepted / resolved by scope guard | Replaced the dimensionally invalid cross-OFE scalar Stage-3 parent ledger with a lane-keyed ledger and retained a typed production guard against more than one active covered Stage-3 lane. The complete-owner two-OFE/two-child fixture verifies the separately admitted snow-free multi-OFE domain; genuine multi-lane covered execution remains unauthorized. |
| `A-002`, `B-007` | Accepted / resolved | Restored `last_reviewed: pending`, candidate lifecycle language, and implementation-awaiting-verification gap states. Promotion is deferred until dual verification passes. |
| `A-003` | Accepted / resolved | Added exact coupled-time and Stage-3 cadence invariant anchors. |
| `A-004` | Accepted / resolved | Added the complete ADR-0042 readiness matrix, canonical `science_implementation_status=IMPLEMENTED`, and explicit non-calibratable custody scope. Lifecycle/gap fields separately retain pending promotion. |
| `A-005` | Accepted / resolved | Reordered invariants before producer/consumer obligations, placed mapped schema elaborations before the Binding Exposure Index, and made the Gap Register immediately precede the Change Log. |
| `A-006`, `B-006` | Accepted / resolved | Added runtime aliases, units, provenance, invariant evidence tags, and strict Binding Exposure coverage. |
| `B-001` | Accepted / resolved | Defined proposal-bounded pre-event child, zero-time event, and fresh post-event child chronology with final-only parent cursor publication. |
| `B-002` | Accepted / resolved | Defined the runtime-aligned `DirectWb14ImmutableIdentityV1` and exact tagged `framed_sha256` parent preimage, including field encodings, digest derivations, immutable parameter bits, and per-child reconstruction. |
| `B-003` | Accepted / resolved | Defined the seven-owner manifest, auxiliary cursor/receipt lineage, inactive-owner carries, restart wire, and atomic replacement order. Runtime now installs the parent-local surface candidate before snapshot/request/debit/credit/ingress physics. |
| `B-005` | Accepted / resolved | Child receipts store topology-rank ordered routed-queue before/after digests inside the scalar receipt seal; replay validates adjacency between consecutive OFE ranks. |

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

Promotion decision: approved after independent verification A and B both
returned PASS at exact head `a35bf816858ea754bf6f000468377c4acbaca659`.
