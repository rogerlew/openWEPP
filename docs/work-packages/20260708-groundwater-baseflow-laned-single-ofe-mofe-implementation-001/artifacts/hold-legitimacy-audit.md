# Hold Legitimacy Audit

Status: `EXECUTED-HOLD-GWDSV-CHANNEL-CONSUMER`

Evidence: `Static + ran`

## Blocker

Generated groundwater baseflow (`gwbfv`) now has a real downstream consumer via
direct WAT `Base` and the watershed WAT reader. Generated groundwater-reservoir
deep seepage (`gwdsv`) and the `bftharea` watershed/channel threshold branch do
not yet have a real downstream consumer in the implemented Lane D hillslope
surface.

## Evidence

- `DirectGroundwaterRunState::run_day` computes `deep_seepage_m3`.
- `DirectLanedActiveRunSummary` records
  `total_groundwater_deep_seepage_m3`.
- Direct WAT publishes `Base` only; no generated deep-seepage WAT/HBP/watershed
  column or reader was implemented.
- `DirectGroundwaterAuthority::LinearReservoir` carries
  `baseflow_threshold_area_ha`, but no watershed/channel threshold branch reads
  it.

## In-Envelope Correction Considered

Adding a new WAT/HBP deep-seepage output column without a real watershed
consumer would be producer-only evidence and would violate the package and root
closure rules. Moving watershed/channel `bftharea` routing into this hillslope
package would broaden the write set into watershed/channel branch behavior and
requires separate contract/test authority.

## Follow-On

Author a narrow follow-on package for generated groundwater deep-seepage and
`bftharea` channel consumer closure. Minimum scope:

- decide the public consumer surface for generated `gwdsv`;
- implement the real reader/aggregator/output path;
- implement or explicitly defer the `bftharea` threshold branch with topology
  area lineage; and
- prove namespace separation from current soil `Dp`, `latqcc`, and channel
  `cbase`.
