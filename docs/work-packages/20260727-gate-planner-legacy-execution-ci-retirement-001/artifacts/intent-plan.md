# Implementation Intent And Validation Plan

Evidence class: Static.

Intent: control-plane retirement with behavior-preserving direct-authority
migration. This is non-kernel but security-sensitive and repository-wide.

The exact terminal diff selects focused authority/workflow tests, anti-evasion
guards, workspace metadata, schema and documentation checks, plus
campaign-strength full-workspace Nextest and strict Clippy. The broad runs are
delegated to the required comparator-suite runner.

No linter, planner, TESTGATE, CAL, model, or Harvard command is an authority for
this package. Requirements are selected and executed directly.
