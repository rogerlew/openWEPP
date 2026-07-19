# Reproduction And Correction

Evidence class: `Ran` and `Static`

Three retained receipts show the same mechanism across
`assurance_dossier_build_contract` and `assurance_v2_source_contract`: fresh
coverage overrides `TMPDIR` with the deep
`${OPENWEPP_GATE_ARTIFACT_ROOT}/target/affected-crap/tmp` path, causing Unix
socket bind to fail before contract behavior executes.

The correction changes only fresh-acquisition temp selection:

- executor-confined runs: `${OPENWEPP_GATE_ARTIFACT_ROOT}/tmp`;
- standalone fallback: `${OUTPUT_DIR}/tmp`, unchanged.

Output, Cargo target, Nextest store, LCOV, CRAP report, package scope, and
evidence paths are unchanged. The TESTGATE integration contract adds exact
source-string assertions for both branches.
