# Non-Quality DAG Parity

Evidence mode: Static + Ran.

The exact terminal plan contains 12 nodes: six light and six heavy. It retains
authority admission, authority anti-evasion, formatting, documentation lint,
gate-policy schema consistency, placeholder scanning, three native-canopy hard
invariant suites, workspace Clippy, workspace doctests, and full-workspace
nextest.

It contains none of the prohibited definitions:

- `affected-adjudicated-crap-v1`
- `adjudicated-crap-v1`
- `combined-workspace-quality-v1`

The planner no longer probes for coverage or CRAP tools and the ordinary
TESTGATE/release workflows no longer install, collect, or upload combined
quality evidence. Standalone quality tooling remains available for the later
optional Quality CI and CQR packages.

Plan and receipt carry exactly:

```text
status: DEFERRED_TO_QUALITY_CI
owner: openwepp-quality-observatory
trigger: OPTIONAL_OPERATOR_DISPATCH
observations: COVERAGE, CRAP
closure_eligible: true
```

The pre-heavy audit independently passed `QUALITY_DEFERRAL`. Schema and source
contract tests reject missing, unknown, `SKIPPED`, `NOT_APPLICABLE`, altered
owner/trigger, incomplete observations, false closure eligibility, retired
gate IDs/families, and the retired CRAP artifact contract.
