# Terminal batch temporal corrected contract candidate manifest

Status: `IN_REVIEW / DIAGNOSTIC EVIDENCE PENDING / PRODUCTION RUST PROHIBITED`

Historical source checkpoint: `83fb00514e8932561bee5aff26ccdf7c130d470f`

The held v20/v10/v138/v5 candidate remains frozen at that checkpoint. This is
a distinct corrected candidate set. Any edit to a listed file invalidates the
corresponding recommendation and requires a fresh hash and review.

| Surface | Candidate | SHA-256 |
|---|---:|---|
| `SC-SNOWENERGY-001.md` | v21 | `7b125f383ae3dca7fb3fe52e40dfe9bf28347cf69c32bb4e13f7a031a9f3772e` |
| `SC-LANDSURFACEENERGY-001.md` | v11 | `067247c6c811bc1dcf472b84d6dd5422fb6ff4abb7587a4a2d957d00306b7dcf` |
| `SC-SNOWFREEZE-001.md` | v139 | `a84d8413855c540daa8ae1d0f9b74ef2f7ae49be0654b0463d2c818d7cb4f1ce` |
| `SC-COUPLEDTIME-001.md` | v6 | `bc192feb54a62ddc9085af904a14c113866d95201c935dff002d78e60bdb86ca` |
| SCC and forcing inventory | v2 | `f0875fb8260cd6543a8ed1cbd6f7fd55183b2138392ea62e11af762e1b4536e0` |
| corrected successor guard | v21 | `4fec89a94c68b4c6af766e7c67e47b0b41f168abd71a497f67a4fceda818ecd7` |
| rejected-trial diagnostic mini-gate | v1 | `af381f66ad624cdb90770bcb454fea3f41f65131c0ad7751146a708eb096b114` |

Ran: formatting and diff hygiene passed. The v19 preservation and corrected
v21 coordinated contract guards passed 5/5 in nextest run
`b608d819-22ac-4e0f-a45d-2a86c500cdd0`.

The diagnostic mini-gate is not implementation authority. Two independent
GO-to-evidence reviews on its exact hash are required before any test-only
observer may be considered. Until then the 1.875-second component receipts and
estimator effectivity matrix remain `NOT RUN`, and no final candidate review or
production implementation intent is permitted.
