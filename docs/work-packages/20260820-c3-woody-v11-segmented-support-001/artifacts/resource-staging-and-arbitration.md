# Resource Staging And Arbitration

Status: authority candidate

Each accepted segment reads the current staged complete owner set. Water and
NH4/NO3 request/authorization/final use preserve parent/segment/slab plus all
existing source identity. Final use alone forms the next staged owner. Energy,
soil thermal, canopy release enthalpy, vegetation C/N, and material receipts
join on the same slab. Parent validation independently reconstructs ordered
cumulative debits and ending inventories. No segment commits or reuses the
parent beginning. Rejection retains byte-identical staged and live owners.

For each exact water, NH4, or NO3 key, authoritative custody is evaluated in
accepted order as `ending = current_staged_beginning - admitted_amount`, using
one binary64 subtraction per segment; that ending is the next beginning.
Separately, the parent repeats an ordinary `+0.0`-seeded cumulative amount
left-fold for receipt diagnostics. Nonassociativity is expected: regrouped
`parent_beginning - cumulative` bits need not equal the sequential ending and
must never replace it. Both independent chronicles are authenticated.

Material amounts are computed per accepted segment by imported V10 phenology,
turnover, and mortality, retained with support/source lineage, concatenated in
accepted order, and assigned parent-scoped proposal IDs once. Recomputing only
from the final state is prohibited.
