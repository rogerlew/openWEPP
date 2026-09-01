# Public API parity

Status: `VEGETATION VERIFIED; TERMINAL PENDING`

Evidence mode: `Static`

The vegetation refactor authorizes no public API delta. Terminal verification
must confirm the public `restore` and `restore_with_bgc_scope` signatures,
checkpoint types, restart bytes, errors, and consumers are unchanged. New
items, if any, must be private implementation details only.

Ran: the vegetation worker reconstructed 63/63 public items and methods with
zero differences. The new replay state/helper are private. Publication may add
only a crate-private sealed-ingress getter; fixed-point and LSE corrections
authorize no public API change. Terminal workspace API parity remains pending.

## WGHL-FULL-001E

Static: the publication correction changes no exported item, function
signature, error variant, serialized field, output schema, or crate boundary.
The accepted receipt projector and its result carrier are private; the
existing real accepted-child test independently reconstructs the same sealed
receipt source. The existing sealed-support WB14-parameter accessor is now
available to its crate-private committed-publication consumer outside
`cfg(test)`. The mechanical test extraction uses a private path module. No
public API delta was introduced; terminal workspace API parity remains
parent-owned.

## WGHL-FULL-001F

Static: the solver change adds one private predicate and private inline tests.
It changes no `pub` item, function signature, enum, struct, model-definition
identity, serialized form, error type, or crate boundary. The existing
`pub(crate)` covered diagnostic surface is unchanged. F-slice public API
parity is `PASS`; terminal workspace parity remains parent-owned.

## WGHL-FULL-001D open-snow structural split

Status: `PASS`

Static: the split authorizes no public or crate-private API delta. All moved
items retain their exact spelling, visibility, attributes, signatures, order,
and module scope because each destination is expanded by a same-position
`include!`. Pre-split inventory contains no exported item in either moved
block. Terminal source reconstruction and focused consumer tests remain to be
recorded.

Ran: both shards contain zero exported or crate-private items. Expanding the
two includes reproduces the exact pre-split source hash
`593efc8ceb7f54c3bf7b4bed965c9b5ea8c48966e41e192b29452ccdc375ef64`;
therefore every pre-split signature, visibility, attribute, implementation,
test, and item order is identical. All-target crate check, V35 source binding,
reuse/reconstruction tests, and the 5/5 terminal endpoint group passed.

## Terminal orchestrator structural splits

Status: `PASS`

Static: the two exact same-scope `include!` splits authorize no public or
crate-private API delta. All moved items must retain exact spelling,
visibility, attributes, signatures, order, and module scope. Terminal expanded
source reconstruction, exported-item comparison, and focused consumer tests
remain required.

Ran: the prepared-support shard retains one exported struct and its complete
inherent implementation; the carrier-humidity shard retains four crate-private
test-audit structs and two crate-private test-audit functions. Expanding each
same-position include reproduces the exact full pre-split source SHA-256
(`8d718bd4164f0725b8b5f5810f9f90ec838a50aedcb6025286bd8ead0ea8f70a`
and `18cc5eec03340f920fcdc8c17d84ef6b8ac3b087af92908c898543ec170aded8`),
so visibility, signatures, attributes, implementation bodies, tests, and item
order are byte-identical. All-target orchestrator check and focused consumers
passed; no API delta was introduced.
