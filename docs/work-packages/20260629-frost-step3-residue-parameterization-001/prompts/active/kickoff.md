# FROST STEP 3 Kickoff

Execute the diagnostic package in `package.md`.

Use the Step 2 analyzer for Sleepers timing/sign-coherence classification.
Before any Sleepers re-score, prove with `OPENWEPP_R7G_FROST_TRACE_PATH` that an
existing seasonal `Dec_*` fixture drives a seasonal `residue_depth_m` trajectory
all the way to the frost solver. If the entry gate is flat or physically
unreasonable, close branch C and do not run the core re-score.

Preserve diagnostic-only scope: no frost-model code, snow-model code,
contract-physics, default, output-schema, selector, production fixture, or
harness-default changes.
