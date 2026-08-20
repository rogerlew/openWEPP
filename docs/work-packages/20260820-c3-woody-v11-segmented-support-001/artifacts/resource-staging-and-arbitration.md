# Resource Staging And Arbitration

Status: authority candidate

Each accepted segment reads the current staged complete owner set. Water and
NH4/NO3 request/authorization/final use preserve parent/segment/slab plus all
existing source identity. Final use forms an occupancy debit receipt, not the
next shared owner. Energy,
soil thermal, canopy release enthalpy, vegetation C/N, and material receipts
join on the same slab. Parent validation independently reconstructs ordered
cumulative debits and ending inventories. No segment commits or reuses the
parent beginning. Rejection retains byte-identical staged and live owners.

For each exact occupancy water, NH4, or NO3 debit key, the parent repeats an
ordinary `+0.0`-seeded cumulative-use left-fold for receipt diagnostics. Shared
custody is independently carried by the complete owner transition; neither a
single occupancy post-use value nor a regrouped cumulative value may replace
that transition ending.

Occupancy-scoped debit receipts are not shared-owner inventory rows. They bind
request, authorization, final vegetation use, and occupancy/layer/source
identity. A separate shared transition keyed by owner/OFE/layer/source binds
the complete staged beginning and ending, ordered debit-receipt links, other
admitted flux links, and the canonical complete-owner candidate digest. Shared
transition endings—not occupancy post-use fields—form cross-segment
predecessors. Every debit is linked exactly once and authorized against the
current shared beginning.

Material amounts are computed per accepted segment by imported V10 phenology,
turnover, and mortality, retained with support/source lineage, concatenated in
accepted order, and assigned parent-scoped proposal IDs once. Recomputing only
from the final state is prohibited.
