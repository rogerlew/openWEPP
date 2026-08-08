# Synthetic Soil-Water Coupling

Status: complete authority protocol; implementation missing

Evidence mode: Static

One immutable transaction identifies interval, area, beginning vegetation
state, beginning hydrology state, forcing, soil-layer identities, and all
candidate/receipt records.

Stage A freezes the beginning states. Vegetation returns potential response and
`D_s,l` requests plus reconstructible diagnostics. It mutates no soil/frozen
state and publishes no actual transpiration or realized carbon response.

Stage B evaluates all vegetation and competing requests against the same
hydrology snapshot. Hydrology alone constructs candidate layer mutations and
returns `U_s,l` with exactly one reason: `fully_supplied`, `zero_demand`,
`liquid_storage_limit`, `frozen_exclusion`, `rooting_exclusion`, or
`competing_demand`. Each allocation is finite, non-negative, no greater than its
request, and the layer sum cannot exceed same-snapshot admissible liquid.
On one immutable transaction and horizontal-area basis, the aggregate guard is
`sum_s U_s,l + W_comp,l <= A_l`: vegetation allocation plus every competing
withdrawal cannot exceed the layer liquid amount hydrology declares
admissible. Individually bounded requests cannot overbook that aggregate.
Missing priority/fairness authority, invalid state, or stale identity is a typed
failure rather than a limitation reason.

Stage C consumes that exact receipt and constructs candidate vegetation state.
For each stratum, actual transpiration equals the sum of accepted layer
withdrawals. A constitutive response that cannot accept the exact receipt
rejects the transaction; it cannot use less water after hydrology debit.
Land-surface energy independently joins the same mass to one latent-energy
debit. Both owners reconstruct all shared operands.

The orchestrator commits hydrology and vegetation candidates atomically only
after receiving owners and water/energy/element/material closure pass. Failure
or non-convergence leaves all states unchanged. Version 1 authorizes no
iteration; a successor must define variables, norm, dimensional tolerance,
maximum iterations, and rollback before iterative coupling exists.

No allocation policy or physiology is implemented by this artifact.
