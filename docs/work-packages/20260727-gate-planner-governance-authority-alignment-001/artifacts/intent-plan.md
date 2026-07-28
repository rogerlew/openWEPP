# Implementation Intent

Evidence class: `Static`.

Intent: governance-authority migration.

Base commit: `bac26bfa`.

The package removes prospective planner/TESTGATE authority from operative
guidance and source-coupled governance guards. It does not change executable
planner behavior, production code, science contracts, CAL, CI, or Harvard
state. Validation is direct and focused under ADR-0043.

Selected evidence:

- migrated governance integration guard;
- external-authority anti-evasion guard and required-suite contract;
- JSON parsing and historical-object reconstruction;
- scoped documentation lint;
- exact diff/write-set reconciliation;
- dual review and dual verification.

Not selected: planner, TESTGATE, full workspace, Clippy, coverage/CRAP, CAL,
population, comparator, publication, CI, or Harvard execution.
