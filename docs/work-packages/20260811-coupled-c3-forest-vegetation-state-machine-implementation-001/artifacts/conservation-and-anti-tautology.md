# Conservation And Anti-tautology Plan

Status: `PARTIAL / water and vegetation-owner reconstruction pass; BGC and energy owners pending`

Typed request/authorization identity and tolerance-bound comparison now have
independent protocol tests. Radiation and potential energy/water operands are
not emitted because their canonical construction is incomplete.

Water reconstruction uses start canopy liquid + rain + condensation against end store + wet evaporation + throughfall + stemflow + drainage, all kg m-2 interval, and separately reconstructs each soil-layer withdrawal. Energy reconstructs incident short/longwave against reflected/terminal shortwave, emitted longwave, sensible and latent interval energy in J m-2 ground. Carbon reconstructs beginning pools + molar-converted GPP against respiration + ending pools + exported litter/CWD. Nitrogen reconstructs beginning vegetation N plus exact layer/species finalized mineral use against ending pools + transfers. Dry material independently uses transferred C divided by explicit `drymatter_carbon_fraction`, never C itself. Candidate-to-ledger validation binds uses and transfers to these operands before residual reconstruction.

Poison fixtures must make expected values differ from rate/amount, leaf/ground area, sign, adjacent diagnostic, authorization/final-use, C/DM, layer swap, mixed-profile average, big-leaf, omitted-stem, and PMET-donation aliases. Tests reconstruct from public candidate operands rather than calling producer closure helpers.

Increment 2A exposes one `OccupancyLiquidLedger` per lane and one
`TileLiquidLedger` per column. The validator reconstructs
`S0 + P + condensation - S1 - evaporation - throughfall - stemflow - initial
drainage - second drainage`, then cancels only internal same-tile releases for
column and weighted-stand reconstruction. It overwrites the solver's residual
field with the independently reconstructed value; a supplied `9999` residual
cannot affect acceptance.

Poisons now distinguish stand-area from conditional tile-area LAI, a replicated
full canopy store, aggregate-first rainfall, missing tile rain, duplicate or
missing occupancy, wrong authorization identity, and omitted/double area
weighting. Independent hydrology/energy owner reconstruction and public
candidate operands remain pending and are not claimed.

Increment 4A adds independent vegetation-owner reconstruction outside the
phenology/allocation producers. Carbon keeps nonnegative physical vegetation C
(all six display/storage/transfer pools, NSC, and standing-dead C) separate
from the finite signed `XS_C` reserve; it closes both beginning operands plus
accepted capped GPP against the directly retained full-maintenance operand,
growth respiration, material exports, and both ending operands. The validator
uses `1e-14 + 64*epsilon*operand_sum`, binds the exact candidate transaction and
beginning/ending state digests, and rejects missing/duplicate strata or reused
proposal IDs. Nitrogen includes all tissue pools,
retranslocation, and standing-dead N; it closes finalized external use and
material exports. Dry material is independently recalculated from each exact
proposal C operand and the stratum's explicit configured carbon fraction, then
checks its C and N proposal aggregates against the independently constructed
element ledgers; carbon-as-dry-matter and forged aggregate poisons fail. No
producer residual is stored or accepted. BGC receiving-side and independent
energy-owner reconstruction remain pending.

Increment 4B removes the caller-provided BGC receipt alias. The BGC owner
independently groups finalized use by exact `(layer,species)`, reconstructs
`beginning-use=ending`, and constructs its own material receipts and receiver
`beginning+incoming=ending` operands for C, N, and dry material. Tests prove
unused authorization is not debited and wrong-species use, duplicate proposal
identity, missing receivers, and transformation requests reject without
changing beginning state. Vegetation-proposal/BGC-receipt reconciliation at the
sealed V7 transaction boundary and independent component-level energy closure
remain pending.
