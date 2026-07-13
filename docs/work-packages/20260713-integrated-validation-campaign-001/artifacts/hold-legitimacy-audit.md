# HOLD Legitimacy Audit

Evidence class: **Ran + Static**

Status: `PASS`

The blocker is outside this validation-only package's protected write set. The
default release command exited 101 after 8:50.55 with exactly three H2637
selector failures. Source inspection proves those tests mutate process-global
Lane D variables and explicitly require nextest process isolation. The
missing-coefficients failure passed alone under nextest at the same frozen
source; source inspection supports the same collision mechanism for the two
mutual-exclusion failures, but they were not separately rerun.

The considered in-package route—serial rerun or environment override—cannot
close the required unmodified default release gate and would weaken the package
contract. Editing `tools/release/run_release_candidate_gates.sh` is explicitly
forbidden here. The smallest legitimate correction boundary is the release
harness plus a source guard, now owned by `INTVAL-REL-001`. This is a named,
testable defect rather than a diagnostic relay.
