# Coverage Before

Evidence label: Static.

Status: `EXECUTED`

Source: `/tmp/openwepp-cqr-nightly.lcov`

Target module:
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/01_ws22_ws23_ws26_detachment.rs`

Baseline target coverage:

- Lines: `193/749` (`25.767690253672%`)
- Branches: `0/0`
- Functions: `7/52`

Existing focused tests observed before scaffold:

- `wshedimpl26_dcap_flagm2_caps_detachment_rate_at_maxe`
- `wshedimpl27_enddet_helper_exercises_xdbig_and_midpoint_rebracketing`

Initial interpretation:

Existing tests cover some `ws26_dcap` and `ws27` behavior, but the four
highest-CRAP rows are currently zero-covered. Characterization should cover
branch outcomes directly before decomposition.
