# Test Vector Ledger

## Executed positive families

The final fixture executes rather than merely names these families:

- open bare soil day and night;
- dry and wet litter-covered forest floor;
- canopy-covered, open-plus-covered, and two heterogeneous canopy columns;
- zero shortwave and longwave cooling;
- reciprocal ground-to-canopy longwave and ground H/E feedback;
- surface condensation with a signed hydrology mass credit;
- full, partial surface, and partial top-layer water authorization;
- simultaneous root and ground competition against one immutable store;
- precipitation/runon followed by infiltration/runoff and source-carried
  enthalpy receipts;
- positive and negative ground heat, zero and nonzero storage changes;
- alternate valid initial guesses;
- singular Jacobian, backtracking limit, iteration limit, domain rejection, and
  exact rollback.

Each accepted record contains primitive state/configuration/forcing, current
trial and accepted component states, source-resolved radiation, source-keyed
`D/A/F`, finalized hydrology owner reconstruction, post-solve liquid partition,
advected-energy receivers, equal/opposite ground-heat receiver, dimensional
component residuals, normalized nonlinear residuals, and local/OFE closure.

The exact-core mandatory matrix contains 22 named entries. The added
`frozen_ground_cap_centered_probe` binds fixed branch and fixed cap value under
the review-reproduced centered temperature perturbation. Open bare-soil
reductions execute the full arbitrary-layer Crank--Nicolson surface system;
covered entries execute the full multirank V8 radiation, gas, energy,
hydraulic, shared-canopy-air, ground-surface, and soil-thermal system. The
equilibrium-zero branch executes with `C_dry=W=U_s=0` and uses the current
algebraic trial surface temperature at both top-interface endpoints. Alternate
temperature warm starts do not enter its physical Crank--Nicolson operands.
The additional albedo vector executes the complete V7/V8 two-stream lower
boundary twice with distinct ground albedos and proves changed ground
reflection, canopy absorption, terminal shortwave, surface temperature, and
root/ground water demand. No canopy absorption is independently hard-coded.

The complete owner matrix additionally executes:

- a single immutable arbitration over five competitors for one shared soil
  layer, including two heterogeneous covered columns and an open-ground
  request;
- a routed upstream/downstream multi-OFE parcel with source-carried enthalpy;
- retained-ingress LSE enthalpy and a soil-thermal infiltration receipt;
- independently constructed vegetation, hydrology, LSE, soil-thermal, and BGC
  candidate bodies and independently reconstructed typed receipts;
- complete `D/A/F` and ending-store ledgers for all 19 shared-source requests;
- a positive condensation credit joined to hydrology mass and LSE energy;
- exact rollback for every domain and numerical rejection.

The soil-thermal candidate independently consumes the accepted infiltration
enthalpy in the identified first node. Its ending temperature is reconstructed
as beginning temperature plus the sum of ground-heat and infiltration receipts,
converted from stand-ground to tile-ground exactly once and divided by the
node's areal heat capacity. The frozen result is
`292.28354996106884 K`. Four executable poisons reject omission, duplication,
wrong-node application, and wrong-area-basis application of that receipt.

Post-ingress surface enthalpy is reconstructed as the complete dry-body plus
retained-liquid node state. The routed multi-OFE vector derives its downstream
parcel from the accepted upstream runoff crossing and converts per-area amounts
through distinct upstream and downstream extensive areas.

## Executed poison families

Every poison record is passed through an executable typed validator and retains
the attempted and authoritative digests, typed failure, null candidate, and
rejection result. There are no label-only poisons. Required families include
PMET donation, stale ground longwave, omitted/duplicated canopy-ground
exchange, reference-air bypass under canopy, direct/diffuse and VIS/NIR swaps,
leaf/stem/ground repartition, missing/double tile fraction, tile/stand alias,
authorization-as-use, latent duplication, condensation sign reversal, omitted
precipitation heat, swapped runoff/infiltration advection, duplicate `G`, legacy
`surtmp`, hidden wind floor, negative-flux clipping, producer-supplied
residuals, request inflation, second authorization, wrong layer, and wrong
occupancy. Natural singular, iteration-limit, and backtracking-limit failures
retain full model/configuration/state/transaction/solve identity, ordered
residuals, active branches, and an exact five-owner-plus-transaction-envelope
rollback rather than using synthetic failure hooks.

The frozen failure vectors bind semantic error identity, not merely schema
membership: numerical failures use `LSEB-E-034` with their exact singular,
iteration-limit, or backtracking-limit kind, while unsupported-domain failures
use `LSEB-E-030` with the exact rejected-domain kind and typed failure. The
authority test asserts these pairings directly.

## Independence

Expected values are generated only by the Python authority stack. The inherited
V3 calculator and exact joint core are loaded only after exact SHA-256
verification. The final joint calculator does not load or execute Rust. All six
positive DTO instances are validated against the committed Draft 2020-12
schemas before fixture serialization.
