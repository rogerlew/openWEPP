# Terminal batch temporal coordinated contract candidate manifest

Status: `IN_REVIEW / PRODUCTION RUST PROHIBITED`

Base source: `68897d9488d85430cbf2b11cf1a9839670a3c044`

The independent reviewers receive this exact candidate set. Any edit to a
listed file invalidates their recommendation and requires fresh reviews.

| Surface | Candidate | SHA-256 |
|---|---:|---|
| `SC-SNOWENERGY-001.md` | v20 | `2fa34e123971cfa7d4831e6c4d2066376c56c08233536be9d77b090bd8c21ed1` |
| `SC-LANDSURFACEENERGY-001.md` | v10 | `4d5200250d459e28071d3ea1a5c1da9de1d6524de8ee152d137736552cb02f11` |
| `SC-SNOWFREEZE-001.md` | v138 | `c8c5ee51b2f9d0650da4bdad6b3fe2e1dd87c480a8f8d682d58d057dd50825d5` |
| `SC-COUPLEDTIME-001.md` | v5 | `e3294902ee457d253aca09ee8f3e687af0fe876b2d779d10468a9fb50757cbfd` |
| residual/state inventory | V1 | `21ee2b24b05b9b54a45f9bc1e953ddf194fface7e52476c5ab97f59443209b5a` |
| contract-derived successor guard | V1 | `8fac485dc7faf80fcb70dcd81b891ee99485b08344c27e96176c2ea83f323514` |

Ran: existing v19 preservation and new coordinated successor guards passed
4/4 under nextest run `296cf84e-b8a9-4b0a-8ae4-4cb62f34a0ae`.

Known evidence boundary: the existing `BelowCarrierDomain` rejection does not
return the last admitted coarse/fine component receipt sets. The successor
normatively requires their read-only retention, but the exact 1.875-second
component values/digests remain unavailable without the contract-derived
characterization surface. The known total discrepancy is retained; no
component is inferred or fabricated.
