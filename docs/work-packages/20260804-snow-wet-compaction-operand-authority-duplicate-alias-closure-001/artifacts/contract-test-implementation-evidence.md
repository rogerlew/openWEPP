# Contract-Test Implementation Evidence

Status: complete

Evidence mode: Ran

The new `snow_wet_compaction_operand_authority` integration target binds
`INV-SNOWFREEZE-092`, `OBL-SNOWFREEZE-P-065`, and `TOL-SNOWFREEZE-017` to:

- contract-marker and source-handoff assertions;
- deterministic Snowbird derivative hashes and exact integer precipitation
  reconstruction;
- a deliberately non-aliasing scalar vector; and
- real bulk and multilayer consumer vectors with preexisting snow, generated
  melt, retained rain, and released rain;
- mixed-onset and inactive-path chronology; and
- fail-closed materiality acceptance enforcement.

The scalar vector freezes generated melt `0.011 m`, retained rain `0.003 m`,
released rain `0.005 m`, state loss `0.006 m`, routed handoff `0.011 m`, and
retained-store change `0.002 m`. The authoritative input is `0.019 m`; every
rejected candidate and every pair of candidates is numerically distinct.

Before the production edit, the exact command
`cargo nextest run --test snow_wet_compaction_operand_authority
production_source_rejects_the_retired_duplicate_alias` failed as expected with
Nextest run ID `ca636270-2fc5-4121-9f85-98d885eedcd7`. After implementation,
`cargo nextest run --test snow_wet_compaction_operand_authority
--no-fail-fast` passed all eight tests with run ID
`a895fdfc-1dce-4b4d-9d5f-79c9e0f5225a`.

Review finding B added a second red/green contract-test cycle for the evidence
consumer itself. The materiality-acceptance test failed before fail-closed
threshold enforcement under run ID `aaa24321-f2c8-438f-a47b-74f3c9cc7bb0`
and passed after implementation under
`3f8084ae-2d21-422a-843a-30d925a1515c`.

The real-consumer test independently reconstructs the density diagnostic from
hourly positive `coe_melt_applied_m`, retained rain, and released rain. It
rejects current-duplicate, routed-only, state-loss-plus-rain, retained-store
level, and retained-store-delta values while retaining SWE, compact-ledger,
and density-process closure checks. Separate cases prove the same scalar reaches
the multilayer consumer, rain on the hour that first creates a pack is not
retroactively classified as contact rain, and inactive coupling stays zero.
