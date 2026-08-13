# Diagnostics And Output Manifest

Status: `PARTIAL / V5 capped diagnostic authority bound; implementation active`

No opaque solver failure was added. The audit also confirms that existing
numerical errors do not yet retain the required partial iterations, residuals,
steps, backtracking, and active-bound payload; this remains part of the
authority-lift/resumption boundary.

Increment 2A carries occupancy solver iteration count and normalized residuals
without using them as fallback inputs. Its routed result exposes distinct
throughfall, initial drainage, second drainage, stemflow, local layer water,
and occupancy/tile closure operands. A producer-supplied closure scalar is
ignored and replaced with independent reconstruction.

The full nested solver diagnostics, public request/authorization/final-use
triples, energy owner operands, five-ledger public candidate, and atomic commit
remain pending. Historical helper structs do not establish those public claims.

The V3 exact numerical-failure diagnostics and production potential evaluator
remain imported unchanged through V5. V5 additionally requires configured-
order cap identities, independently evaluated law/cap/selected operands,
tie-derived active lists, typed residual records from the failed iterate,
iteration/step/backtracking/pivot/matrix payloads, and null candidate/use on
every failure. The immutable fixture binds those diagnostics, but Rust
implementation and review are still active. The capped path cannot publish
diagnostics, finalized use, or an accepted candidate yet.
