# Objective and Observation Operator

Status: `FROZEN / NOT EXECUTED`

Evidence class: `Static: authority-preserving prospective specification`

The calibration model field is `/gsi/gsi21` from
`openwepp-canopy-research-daily-v1`, site `hubbard_brook`, arm `deciduous`,
member `hubbardbrook_deciduous_nh/p10`. For each calendar year, the modeled
spring date is the first daily pair satisfying `previous < 0.5 <= current`.
Equality belongs to the current day and no subdaily interpolation is used.

The independent holdout uses the same field, arm, and composite-member
semantics for site `harvard`, member `harvard_deciduous_ma/p6`. Its modeled
fall date is the first daily pair satisfying `previous > 0.5 >= current`.
Harvard remains sealed.

For observation interval `i=[lower_i, upper_i]` and modeled day `m_y`, signed
interval distance is zero when `m_y` is inside the closed interval, otherwise
the day distance to the nearest bound. For each eligible year `y`:

`annual_mse_y = mean_i(distance_i^2)`.

The scalar objective is:

`sqrt(mean_y(annual_mse_y))`.

Years receive equal weight. Species, sites, trees, subplots, records, and years
receive no fitted weights. A missing required annual crossing invalidates the
candidate with objective `+infinity`; failed records and years remain counted.
Diagnostics, not selection components, are species RMSE, observation- and
year-level median absolute distance, interval coverage fraction, and failed
counts.

Stage authority is immutable:

- GSI timing: 932 Hubbard P3 half-expansion intervals, calibration.
- Harvard timing: 319 50% leaf-fall intervals, independent holdout only.
- Biomass partition: `CAL03-OBS-HB-001` constrains the partition sum, not
  separate `Bf,max` and `Bs` values.
- Peak LAI: `CAL03-OBS-HB-005` admits a bounded mature leaf-on landscape LAI
  range, conditional on accepted upstream GSI and `fe`.
- Evergreen fraction and canopy cover: no quantitative selection component;
  qualitative/proxy screens cannot create scalar loss.
- Timing records cannot be reused as magnitude authority.
- Snow, interception, ET, runoff, frost, erosion, residue, and litter:
  downstream evaluation only after ensemble freeze.

Ran: the admission extractor rebuilt all 1,251 timing intervals byte-identical
to the admitted table, SHA-256
`890a0ff09ca707b097a15cb5de7964698a9b4d5af797ed6b81d5fccf7c141b61`;
the role counts are 932 calibration and 319 independent holdout.
