# Transaction Ordering

1. Validate complete V8/LSE/hydrology configuration, beginning state, forcing,
   topology, owner set, and transaction lineage.
2. Clone production inputs into a shadow envelope; freeze beginning owner
   bytes and the immutable **beginning hydrology-store** resource snapshot.
3. Validate source-resolved current precipitation and routed runon, including
   temperature/enthalpy lineage, but do not add them or any trial canopy release
   to the same-interval ET authorization inventory.
4. Solve the fully coupled potential canopy--ground system from immutable
   beginning thermal/vegetation state. Current precipitation, runon, and canopy
   release advection does not enter that interval's `H`, `LE`, `G`, surface
   temperature solve, or water availability.
5. Publish the complete typed vegetation-root and ground-source request set.
6. The real hydrology owner authorizes exactly once from immutable beginning
   stores, including all same-source competitors on that same beginning
   snapshot. Authorization is a maximum, not a debit. Current precipitation,
   runon, throughfall, drainage, and stemflow never enlarge it.
7. Rebuild the entire canopy--ground column from original beginning state under
   the fixed source caps. Recompute final canopy releases and their accepted
   wet-surface temperatures, but do not yet mix or partition current ingress.
8. Finalize each withdrawal under its fixed cap and validate exact `F<=A<=D`.
   Because `A` is backed only by immutable beginning stores, changed final
   canopy release cannot shrink its backing supply. There is no second
   final-inventory cap and no reauthorization.
9. Debit finalized withdrawals from beginning stores. Then apply current
   precipitation, runon and final canopy releases, separate condensation
   credits, capacity/overflow, infiltration and runoff to construct hydrology
   ending mass, LSE thermal state, soil-thermal receipts,
   C/N/material candidates, and all independent ledgers.
10. Validate exact D/A/F, source mass, latent/advection/ground-heat joins,
    identities, and the complete owner envelope.
11. Commit the shadow by one non-fallible whole-state replacement.

There is no stale potential result, potential-candidate continuation, demand
donation, request inflation, second authorization, or fallible action after the
commit boundary. Any failure preserves the full beginning shadow and
production bytes.
