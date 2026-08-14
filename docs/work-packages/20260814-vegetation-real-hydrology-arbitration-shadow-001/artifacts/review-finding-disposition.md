# Review Finding Disposition

Status: `all accepted findings corrected / both independent re-reviews GO`

Evidence class: `Static + Ran`

## Hydrology review

| Finding | Disposition | Correction |
| --- | --- | --- |
| `HYD-REV-001` | accepted | Added the production-owned `authorize_direct_layer_withdrawals()` endpoint over seeded production day frames and the shared production `apply_direct_finalized_layer_liquid_debit()` mutation primitive. The owner endpoint reuses the canonical dependency-neutral proportional primitive; no copied arithmetic remains. |
| `HYD-REV-002` | accepted | `try_from_day_start()` now calls the same `DirectRunFrame::seed_day_frame()` constructor used by production and derives arbitration facts from its percolation layers and water state. |
| `HYD-REV-003` | accepted | The public bridge rejects unequal interval, candidate transaction, ordered layer set, beginning liquid, or frozen facts. Root accessibility is derived from the same forcing. |
| `HYD-REV-004` | accepted | Exactly unfrozen and exactly fully frozen layers are represented. Partial frost returns a typed unsupported operand error; no whole-layer heuristic remains. |
| `HYD-REV-005` | accepted | A single positive request limited by nonzero storage receives `LiquidStorageLimit`; `CompetingDemand` requires multiple positive eligible requests for the source. |
| `HYD-REV-006` | accepted | Artifact claims now distinguish the bounded canonical arbitration projection from complete whole-frame structural equality. Transaction, owner and interval were added to the projection bytes. |
| `HYD-REV-007` | accepted with bounded scope | The executable Child-2 bridge now explicitly rejects routed multi-OFE use. Multi-lane source identity and candidate debit remain tested below the public bridge; coordinated routed execution belongs to the Child-4 scheduler consumer and is not claimed here. |

## Rust review intake

The initial Rust review identified zero-demand transaction mismatch, interval
nonbinding, a full-depletion mass/depth round-trip edge, weak boundary error
mapping, missing water-authority impact coverage, incomplete snapshot language
and incomplete arbitration correspondence. All were accepted. The changed
transaction surfaces are atomically bound to admitted
`SC-VEGETATIONTRANSACTION-001`; an attempted direct `SC-WATBAL-001` impact-map
binding was rejected by the admission gate because that contract remains
`draft/in_review`, so the package records it as architectural ownership context
rather than falsely admitting it. The final exact-byte Rust review returned GO
with no correctness blocker for the declared default-off single-OFE boundary.

| Finding | Disposition | Correction |
| --- | --- | --- |
| `RUST-REV-001` | accepted | The one shared proportional primitive now uses canonical ordering, typed derived-overflow rejection, per-request bounds, a remainder correction reconstructed by the shared compensated sum, and a nonprogress guard. Owner validators and debit reconstruction use the same canonical sum. Exact binary64 witnesses are frozen. |
| `RUST-REV-002` | accepted | Reason precedence now distinguishes exact-zero supply, nonzero single-request scarcity, actual eligible competition and excluded requests. Positive and negative zero share one zero class. |
| `RUST-REV-003` | accepted | Soil-water aggregation is centralized in the production direct-runtime owner and reused by ET, subsurface and the shadow adapter. |
| `RUST-REV-004` | accepted | All current surfaces are described as a V7 root transaction in a V8-precursor root/OFE envelope; complete V8 surface-class/resource identity and the joint ground batch remain explicit Child-4 obligations. |

The numerical follow-up also accepted and corrected signed-zero, finite-input
derived-overflow, canonical reconstruction and exact impact-binding findings.

No finding is silently rejected. The single-OFE boundary is a declared child
scope, not evidence for the later routed real-consumer endpoint.
