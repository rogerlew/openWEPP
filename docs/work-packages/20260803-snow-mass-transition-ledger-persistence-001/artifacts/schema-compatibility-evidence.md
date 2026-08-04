# Schema Compatibility Evidence

Status: `PASS / byte identity`

Evidence mode: `Ran`

The scaffold and candidate real release CLIs each emitted `14245` schema-v4
rows and `659499507` bytes. Both files have SHA
`84a64c1b4031584842c4d20023acac92fcffbea946fcce04953fb5a0a339fb5f`.
Byte identity proves the ordered keys, numeric text, strings, nulls, row order,
and newline behavior are unchanged; no value-level fallback was required.

WAT is byte-identical at SHA
`e74b8df25485f6e1dd1430a9332c4aab3bafb8498a228f5455611d8081521b75`.
HBP/PASS is byte-identical at SHA
`d5d3468d361510df069475423f785e2be036e0b353c281d0c32d0f82b583c149`.
Loss JSON differs only in the deliberately distinct harness `run_name`;
removing that provenance field yields identical documents. No application
output field differs.

Both scaffold and candidate empty-path and filtered-out selector cases preserve
WAT/HBP/PASS outputs; the candidate creates no trace file in either case. Thus
capture is resolved before the solve and disabled selection has no real-writer
side effect.

Evidence: `target/snow_mass_transition_ledger_persistence/reports/comparison.json`.
