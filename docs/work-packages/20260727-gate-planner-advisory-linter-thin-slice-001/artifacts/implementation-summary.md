# Implementation Summary

Evidence class: Ran.

Order 3 implements a standalone Python advisory linter at
`tools/validation/workplan-lint`. The product has no dependency on the legacy
gate-planner implementation and no lifecycle, receipt, ledger, CI, publication,
recovery, calibration, or custody role.

The thin slice provides:

- explicit `pre-edit`, `working-tree`, and `terminal` modes;
- deterministic human and canonical JSON results;
- cited findings and inert command suggestions;
- completed/partial/unavailable/misuse availability semantics;
- bounded no-follow package and policy-file reads;
- a structurally validated, narrowed read-only Git argv allowlist;
- a fixed Git executable, cleared child environment, closed stdin, timeout, and
  output bounds;
- pre-launch refusal for prohibited Git configuration and attributes; and
- a manual route that never makes linter repair a prerequisite.

The product implementation is 1,023 lines including the executable wrapper,
below the 3,000-line package ceiling. The test implementation is 715 lines.

The repository's existing Git LFS filter configuration is prohibited by the
frozen safety contract. Consequently, direct use against openWEPP reports
package and policy inputs but marks Git-backed analysis partial with exit 3.
Representative temporary repositories without prohibited configuration
complete all three modes. The operator documentation explains that this
limitation is a linter availability result, not a package hold.
