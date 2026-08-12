# Conservation And Anti-tautology Plan

Status: `FROZEN BEFORE PRODUCTION EDITS`

Water reconstruction uses start canopy liquid + rain + condensation against end store + wet evaporation + throughfall + stemflow + drainage, all kg m-2 interval. Energy reconstructs absorbed short/longwave against sensible + latent + storage/conductive terms in W m-2 ground and separately checks surface partitions. Carbon reconstructs beginning pools + molar-converted GPP against respiration + ending pools + exported litter/CWD. Nitrogen reconstructs beginning vegetation N + finalized mineral use against ending pools + transfers. Dry material independently uses transferred C divided by explicit `drymatter_carbon_fraction`, never C itself.

Poison fixtures must make expected values differ from rate/amount, leaf/ground area, sign, adjacent diagnostic, authorization/final-use, C/DM, layer swap, mixed-profile average, big-leaf, omitted-stem, and PMET-donation aliases. Tests reconstruct from public candidate operands rather than calling producer closure helpers.
