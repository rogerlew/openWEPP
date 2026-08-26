# Intake and pre-implementation intent

Status: `AUTHORIZED / EXECUTING`.

Static: starting owner handoff was exact clean pushed
`e3b9e20eebbf5ecd319c372c3d31b1a05a2479d7`. The terminal lifecycle
reconciliation then landed and pushed as clean stable documentation increment
`486a7b8a3`.

Static: the prior exact-clean workspace run at `9b1105d` contains 81 Assurance
V2 identity/source failures and nine retained guards. The prior comparator at
`97accd99` adds the separately classified V9 host-libcrypto binding failure.

Static: Assurance V2 expects released SnowEnergy hash
`b95ba52c9d3212f1248f836a552a48247e17eaa5b4c8489823fa8aae3dcde372`
and released SnowFreeze hash
`976f052f8c74366b1406b95dbe79968691dab326ee9e266c3d4b730cd5a08e6b`.
Those are exactly the canonical contract hashes at fully qualified physical
implementation `43cc9bbe`; current rejected v21/v139 candidate bytes hash to
`7b125f...` and `a84d84...`. Therefore the lock is not rebound. The candidate
canonical additions are removed by exact released-byte restoration while all
historical/research package trees and Git history remain intact.

Static: the nine retained guards are one Stage-0 anti-wiring guard, five
SnowFreeze v136 registry-wording guards, two SnowEnergy v18 registry-wording
guards, and one `terminal_parcels_from_event` attachment-token guard. Each is
executed and dispositioned individually before correction. Test deletion,
weakening, reclassification, or fixture rebinding is prohibited.

Ran: after released-byte restoration, the focused 159-test census passed all
Assurance V2 tests and seven of the original nine registry guards. It exposed
five additional stale exact-version assertions that still pin unreleased
SnowFreeze v137 even though every asserted invariant already exists in released
v136. Those assertions are changed to exact v136; their invariant/obligation
checks remain intact.

Ran: the remaining two original guards are stale path bindings. The Stage-0
anti-wiring scan still checks every forbidden token in every runtime source,
but its exact Stage-3 allowlist predates two Stage-3-only files. Add only those
two exact paths; no token or directory wildcard is removed or broadened. The
attachment guard predates source decomposition; make it inspect both the host
and its included terminal-execution module while retaining every rejected-
surface negative assertion and the exact current parcel-construction token.

Static: no production equation, forcing, selector, restart, receiver, runner,
CoE, terminal behavior, or cutover posture is in the intended diff. No
production Rust source edit is required.
