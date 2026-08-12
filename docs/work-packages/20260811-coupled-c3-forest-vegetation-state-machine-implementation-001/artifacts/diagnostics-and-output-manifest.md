# Diagnostics And Output Manifest

Status: `PARTIAL / internal column diagnostics only; public outputs pending`

Increment 2A carries occupancy solver iteration count and normalized residuals
without using them as fallback inputs. Its routed result exposes distinct
throughfall, initial drainage, second drainage, stemflow, local layer water,
and occupancy/tile closure operands. A producer-supplied closure scalar is
ignored and replaced with independent reconstruction.

The full nested solver diagnostics, public request/authorization/final-use
triples, energy owner operands, five-ledger public candidate, and atomic commit
remain pending. Historical helper structs do not establish those public claims.
