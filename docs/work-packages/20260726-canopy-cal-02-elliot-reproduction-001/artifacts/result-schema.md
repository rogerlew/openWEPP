# Result Schema And Operand Lineage

Evidence class: `Static pre-run schema`

The harness retains arm-local raw outputs and publishes normalized CSVs. Every
mass is per modeled OFE surface area; no watershed or channel quantity is
silently substituted.

| Field | Unit / basis | Producer | Acceptance role | Rejected aliases |
| --- | --- | --- | --- | --- |
| `live_biomass_kg_m2` | kg/m2 OFE, daily | WEPP plant output `vdmt` | standing live stock | standing residue, root mass |
| `current_flat_residue_kg_m2` | kg/m2 OFE, daily | plant output flat pool `#1` | current forest-floor cohort | crop-index token preceding the mass |
| `previous_flat_residue_kg_m2` | kg/m2 OFE, daily | plant output flat pool `#2` | previous cohort | buried pool `#2` |
| `old_flat_residue_kg_m2` | kg/m2 OFE, daily | plant output flat pool `#3` | old cohort | dead-root pool `#3` |
| `total_flat_residue_kg_m2` | kg/m2 OFE, daily | independent normalized sum of three flat pools | forest-floor aggregate | standing + buried + root mass; WEPPcloud `DeadBio` |
| `canopy_height_m` | m OFE, daily; nullable | WEPP plant output canopy height | canopy state | fixed-width overflow token |
| `canopy_height_overflow` | boolean, daily | normalized from an all-asterisk height field | publication-limit evidence | invented numeric replacement |
| `lai_m2_m2` | m2/m2 OFE, daily; nullable | WEPP plant output LAI | canopy state | canopy-cover fraction |
| `lai_overflow` | boolean, daily | normalized from an all-asterisk LAI field | publication-limit evidence | invented numeric replacement |
| `canopy_cover_fraction` | fraction OFE, daily | WEPP plant output | canopy state | percent-formatted element output |
| `runoff_mm` | mm over hillslope effective length, event | `.ebe.dat` | hillslope surface-runoff context | WEPPcloud hill streamflow or watershed discharge |
| `sediment_delivery_kg_m` | kg per hillslope width, event | `.ebe.dat` | hillslope sediment context | kg/m2 detachment, channel export |
| `peak_hillslope_runoff_rate` | mm/h, element event/interval | `.element.dat` `PeakRO` | hillslope peak-rate return levels | rainfall intensity, watershed peak discharge |
| `annual_live_decline_sum_kg_m2` | kg/m2 OFE/year | sum of positive daily live-stock decreases | independent transfer check | year-boundary stock difference |
| `annual_litter_transfer_kg_m2` | kg/m2 OFE/year | perennial `grow.for` reconstruction: maximum published daily `vdmt * (1 - dropfc)` | output-precision-bounded gross live-to-current-residue transfer | exact internal `vdmx`; foliage-only litterfall; residue stock change after decomposition |

`total_flat_residue_kg_m2` is reconstructed from three separately parsed
columns. Fixtures and tests keep the three values distinct so an adjacent
buried/root/standing alias cannot reproduce the expected sum accidentally.
Annual summaries use raw daily rows; equilibrium is the mean and range of
year-end stocks over years 91--100. Return periods, when published, must be
reconstructed from the retained event series under the declared plotting
position and may not be copied from report prose. The package's empirical
100-year recurrence convention selects descending rank `100 / T`; it is not
labeled as an interpolated Weibull estimator.

The stock binary has no litter-specific Observe tag. Pinned `grow.for`
establishes that active growth and perennial senescence are mutually exclusive,
that daily `delvd` is constant within the senescence period, and that the same
`vdmy-vdmt` mass is added to current residue. The formula reconstruction
therefore defines gross transfer for these perennial arms. Crop output rounds
`vdmt` to `0.001 kg/m2`, so its annual maximum is a bounded estimate of
internal `vdmx`; independently summed daily live decline checks it at that
publication precision.

Top-level normalized files describe the Windows/WEPPpy-2006 lane.
`linux-9002-260725/` contains the separately named Linux/source-native-9002
lane. Same-named files from the two locations must not be interchanged.

The admitted executable prints `****` when canopy height or LAI exceeds the
field width. Normalized CSVs preserve those fields as null and set the
corresponding overflow flag. The harness does not infer or invent a numeric
value from an overflow token.

The source-native constant arm lacks changing cohort transfers by design. Its
fixed initial residue is an analytical comparator, not evidence that perennial
pool parsing succeeded.
