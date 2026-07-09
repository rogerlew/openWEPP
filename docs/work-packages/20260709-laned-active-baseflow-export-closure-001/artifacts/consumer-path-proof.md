# Consumer Path Proof

Status: `EXECUTED-COMPLETE`
Evidence: `Static + ran`

The implemented path is:

1. direct runtime computes `DirectGroundwaterDayOutput`;
2. direct publication row carries generated `groundwater_baseflow_m3` and
   `groundwater_deep_seepage_m3`;
3. HBP writer serializes those as the existing payload pair `gwbfv`/`gwdsv`;
4. HBP parser exposes them as typed `HbpLatestEventPayload` fields;
5. watershed pass inventory validates and hands them to `HillslopeContribution`;
6. watershed channel kernel consumes generated baseflow in the `lr_bf=1`
   branch and records deep seepage separately;
7. typed watershed publication exposes generated baseflow/deep-seepage
   diagnostics without using `cbase` as a fallback.

Ran:

- `r6a_direct_hbp_writer_serializes_groundwater_payload_operands` proves direct
  publication operands serialize into HBP and parse back as `gwbfv`/`gwdsv`.
- `hbp_latest_event_payload_exposes_groundwater_baseflow_and_deep_seepage`
  proves the HBP parser exposes nonzero generated groundwater fields.
- `gwbaseflow_lr_bf1_channel_branch_consumes_generated_hbp_not_cbase` proves
  `lr_bf=1` consumes generated HBP baseflow instead of `cbase`.
- `gwbaseflow_bftharea_suppresses_below_threshold_side_baseflow` proves the
  `bftharea` channel threshold branch.
- `gwbaseflow_generated_hbp_payload_without_gwcoeff_authority_fails_closed`
  proves generated HBP groundwater payloads without `gwcoeff` authority fail
  closed.
