# Disposition

Status: `HOLD-INTEGRATED-VALIDATION`

Evidence class: **Ran + Static**

The frozen-source domain lanes pass, but the required default release candidate
command exits 101. Its threaded `cargo test --workspace` runs three explicitly
nextest-only H2637 selector tests in one process; shared environment makes all
three accept runs that must fail closed. The missing-coefficients case passed
alone under nextest at the same commit; the two mutual-exclusion attributions
are source-supported and were not separately rerun.

Defect `INTVAL-REL-001` is owned by
`20260713-dc-intval-release-nextest-isolation-001`. Integrated validation must
restart in full after that package lands and the default release gate passes.
No partial PASS is claimed.
