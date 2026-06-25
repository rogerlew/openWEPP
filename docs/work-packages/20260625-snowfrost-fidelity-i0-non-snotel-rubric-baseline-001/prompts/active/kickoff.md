# Kickoff

Execute SNOWFROST-FIDELITY-I0.

Use the existing non-SNOTEL frost fixtures and observation corpus under
`tests/fixtures/snowfreeze_observed/`. Run current openWEPP through
`observed_harness.py compare` for all five sites, then emit a v74
`SC-SNOWFREEZE-001#INV-SNOWFREEZE-050` snow/frost rubric profile. This package is
characterization only: do not change production physics or classify observation
disagreement alone as `OPENWEPP-DEFECTIVE`.

Close complete when all package acceptance criteria are satisfied, otherwise
close with the named HOLD boundary and first actionable handoff.
