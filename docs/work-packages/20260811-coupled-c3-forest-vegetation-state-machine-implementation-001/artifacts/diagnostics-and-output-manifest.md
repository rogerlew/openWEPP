# Diagnostics And Output Manifest

Status: `implemented / focused pass`

Diagnostics carry transaction identity, nested solver iteration/evaluation behavior, normalized component residuals, final temperature/potential steps, active water caps, wet-store-cap activity, backtracking, authorization activity, gas/hydraulic transpiration mismatch, and typed convergence/pivot/domain failure. Candidate state carries model/config/state identity; typed requests carry stratum/layer/species/area-interval basis. Candidate output exposes request/maximum-authorization/final-use triples, C/N/DM transfer proposals, and authoritative five-ledger operands. The external validator reconstructs residuals; no producer residual scalar is accepted. Diagnostics are immutable evidence and never alter equations or supply fallback inputs.
