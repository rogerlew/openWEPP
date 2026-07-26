# Discrepancy Register

Evidence class: `Executed discrepancy audit`

| ID | Conflict | Required treatment |
| --- | --- | --- |
| CAL01-D01 | Hubbard report uses `dropfc=0.92`; delivered management has `0.95`. | Reproduce both branches. Never call either the unique final case. |
| CAL01-D02 | Yang `7.6 Mg/ha` is standing foliage biomass; report calls it leaf fall. | Exclude as annual-flux authority. Preserve as standing foliage context. |
| CAL01-D03 | Report converts Yang’s foliage pool to 4% of total biomass and adds another 4% for twigs/branches. | Classify both steps as Bill-derived assumptions; test leaf and woody fluxes separately. |
| CAL01-D04 | Coates `58/42` describes unmanaged hardwood/softwood basal area, not total-fuel composition. | Do not use the ratio to partition the 51.4 Mg/ha fuel stock. |
| CAL01-D05 | Gresham values are aggregate litterfall at nearby Hobcaw stands and include foliage plus small branches, bark, cones, and catkins. | Treat as regional flux context, not Santee leaf/needle-only validation. |
| CAL01-D06 | Report says residue decay was greater in warm Santee climate; delivered `oratea` and `orater` equal Hubbard at `0.0021`. | Reproduce exact operands and flag prose/operand inconsistency. |
| CAL01-D07 | Hubbard source synthesis reports 18.99 kg/m2 aboveground biomass; Yang’s different hardwood stand sums to about 30.4 kg/m2. | Retain site/stand/source identities; do not merge inventories. |
| CAL01-D08 | Hubbard hillslope sediment (2.3-5 kg/ha/yr) is compared with watershed background yield (25-50). | Keep boundaries separate; no goodness-of-fit claim. |
| CAL01-D09 | Santee hillslope surface runoff (103-211 mm/yr) is compared with 290 mm/yr total watershed discharge. | Keep surface/lateral/baseflow boundaries separate. |
| CAL01-D10 | Santee `100-300 kg/ha/yr` and Hubbard `900 mm/yr` are attributed to ChatGPT without a traceable source. | Exclude from calibration and validation. |
| CAL01-D11 | Report cites malformed Dun DOI `10.1016/jhydrol...`. | Use retained paper DOI `10.1016/j.jhydrol.2008.12.019`. |
| CAL01-D12 | Report misspells Gresham as “Greshman” and Gosz coauthor Bormann as “Borann.” | Preserve report bytes; use correct bibliographic identities in ledgers. |
| CAL01-D13 | Report says Santee forest-floor target is 25 Mg/ha without carrying the measured SE or managed-watershed alternative. | Use 25.0 ± 1.8 Mg/ha for unmanaged WS80 and retain 21.5 ± 2.5 separately. |
| CAL01-D14 | Report asserts LAI is constant at cap 6 despite prescribed senescence. | Make seasonal LAI and canopy phase a required CAL-02 diagnostic, not a successful phenology claim. |
