# Model Stack Selection

Status: `selected`

Evidence mode: `Static`

## Decision

The single model version is `OPENWEPP_C3_WOODY_V1`. It is a coupled state
machine, not a set of independently selectable submodels:

| Family | Selected definition | Authority class | Rejected alternative |
|---|---|---|---|
| radiation | CLM5 two-stream direct/diffuse VIS/NIR transfer (3.1--3.30), composed top-to-bottom through explicit openWEPP topology tiles | `REFERENCE_MODEL_DEFINITION` plus `OPENWEPP_CANONICAL_SELECTION` for vertical composition | Beer-only, single-big-leaf, and RHESSys optical bypass cannot preserve direction, band, or overlap |
| canopy scaling | de Pury--Farquhar sunlit/shaded leaf classes; CLM5 leaf-to-canopy integrals | `PRIMARY_PROCESS_AUTHORITY`; executable details `REFERENCE_MODEL_DEFINITION` | one averaged canopy leaf is degenerate under mixed light |
| interception | CLM5 liquid finite store (7.1) and distinct wet fraction; exact store ledger; liquid-only version | `REFERENCE_MODEL_DEFINITION` plus conservation | Gash retained as event comparator; bucket-free throughfall and RHESSys implicit losses rejected |
| transfer/energy | CLM5 canopy-air conductance network and explicit vegetation energy residual (5.86--5.121), solved per topology column | `REFERENCE_MODEL_DEFINITION` | agricultural complementary PMET and RHESSys defective psychrometric/energy branches rejected |
| photosynthesis | bounded FvCB C3 Rubisco, electron-transport, and TPU limits (9.2--9.11), including quadratic electron transport and co-limitation | `PRIMARY_PROCESS_AUTHORITY` plus CLM5 exact definition | C4 and diagnostic-only assimilation rejected |
| stomata | Medlyn (2011) equation, separately for sunlit/shaded leaves; `g0` and `g1` required caller parameters | `PRIMARY_PROCESS_AUTHORITY` | Jarvis multiplicative stress, hidden minimum conductance, and fixed CLM parameter rows rejected |
| hydraulics | CLM5 interval-equilibrium soil-root-xylem-sunleaf/shadeleaf water potentials and vulnerability conductances (11.1--11.41), with layer authorization/finalized use owned by hydrology | `REFERENCE_MODEL_DEFINITION` plus openWEPP transaction selection | unadmitted capacitance/storage, single effective root depth, direct vegetation soil mutation, and scalar beta-only endpoint rejected |
| C/N | CLM5-BGC pool, maintenance/growth respiration, allocation, storage/transfer, phenology, retranslocation, turnover and mortality architecture (16--21) | `REFERENCE_MODEL_DEFINITION` | immutable leaf N, diagnostic GPP, or no-material-mutation endpoint rejected |
| biogeochemistry boundary | new `SC-BIOGEOCHEM-001`: mineral-N requests/receipts and litter/CWD C/N/dry-material custody | `OPENWEPP_CANONICAL_SELECTION` plus conservation | temporary unlimited N source or unowned litter sink rejected |

Fixed scientific constants are restricted to definition-level constants (gas
constant, molecular ratios, photon-energy conversion when used, and selected
FvCB coefficients). Site/PFT rows in CLM, BIOME-BGC, RHESSys, or GIS2RHESSys
are never openWEPP defaults. Every site value and the complete initial state is
caller supplied and digest bound.

## Coherence Argument

Radiation produces sunlit/shaded absorbed PAR and energy. FvCB converts those
operands and leaf N-dependent capacity into assimilation; Medlyn uses that same
assimilation to produce conductance. The leaf-energy/hydraulic solve couples
conductance, leaf temperature, vapor-pressure deficit and layer water demand.
Hydrology returns maximum authorized layer withdrawals; the receipt-constrained
solve returns finalized use no larger than authorization, after which hydrology
and vegetation form candidate states. Respiration and N-constrained
allocation update persistent pools; leaf C and caller SLA determine future LAI;
turnover debits donor pools and proposes exact C/N/dry-material receipts. No
member can be replaced or disabled while retaining this model-version identity.

## Explicit Non-selections

- LUNA is not selected for v1. Photosynthetic capacity is derived from current
  leaf N by the admitted caller-supplied Rubisco-N and electron-transport
  coefficients; this preserves mutable leaf N without importing LUNA's separate
  optimization and empirical parameter set.
- FUN acquisition economics is not selected. Mineral-N demand is the exact
  stoichiometric shortfall of admitted allocation, and the biogeochemistry owner
  arbitrates all same-interval demands pro rata by requested amount. A different
  priority policy requires admission of a new model version; callers cannot
  switch the v1 scientific operator.
- Canopy snow is deferred as `AUTH-RHEC-012`; its entry is a typed
  `UnsupportedProcess::CanopySnow`, never rainfall.
- RHESSys/GIS2RHESSys remain licensed format, source-deviation, and comparator
  provenance (`AUTH-RHEC-013`), not equation or parameter-value authority.
