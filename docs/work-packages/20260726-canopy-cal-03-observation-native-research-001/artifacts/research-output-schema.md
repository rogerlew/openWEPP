# Campaign Research Output Schema

Evidence class: `Static implementation contract`

Selector: `OPENWEPP_CANOPY_RESEARCH_TRACE_PATH`

Schema identity: `openwepp-canopy-research-daily-v1`

Format: UTF-8 JSON Lines, one native-forest lane/day record in chronological
execution order. `OPENWEPP_CANOPY_RESEARCH_SITE_ID` and
`OPENWEPP_CANOPY_RESEARCH_ARM_ID` provide required nonempty campaign
identities; enabling a path without either identity fails closed. The caller
must use a fresh file for each run. The surface is default-off,
campaign-confined, and not a public output API.

Each record includes:

- calendar date/year/ordinal and zero-based day/lane indices;
- the three bounded GSI indicators, photoperiod hours, instantaneous GSI,
  GSI21, and real-window sample count;
- structural, evergreen, deciduous, total foliar, and total aboveground live
  biomass (`kg/m2`); LAI (`m2/m2`); cover fraction; and daily leaf-on/off
  transfers (`kg/m2`);
- exact growth, snow, interception, ET, runoff, erosion, and frost consumer
  operands/results;
- leaf, needle, fine-woody, and total litter fields; surface residue before and
  after (`kg/m2`); decay-only decomposition loss (`kg/m2`); surface decay
  factor; and residue depth (`m`).

`needle_litter_input_kg_m2` and `fine_woody_litter_input_kg_m2` are explicitly
null because current ratified native CP2 has no such source. Null means
inapplicable or unavailable, never zero. All required numerical fields are
finite; the simulation and serializer fail closed otherwise. The analysis tool
rejects missing fields, out-of-range GSI values, duplicate/nonchronological
records, producer/consumer mismatches, and aggregate mass nonclosure.

The decomposition loss is reconstructed independently as
`(surface_before + litter) * (1 - surface_decay_factor)` and therefore excludes
management-action removal. CAL-03 native forest runs admit no residue-removal
action; any future action-bearing campaign requires a schema revision that
publishes action loss separately.
