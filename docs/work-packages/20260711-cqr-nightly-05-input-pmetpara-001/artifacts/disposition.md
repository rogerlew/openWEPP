# Finding Disposition

Status: `EXECUTED-HOLD-CQR-NIGHTLY-LOCAL-FORMATTER-DISPOSITION`.

Accepted: both independent reviewers classify the only row above `30`,
`PmetparaParseError::fmt`, as an ADR-0021 observability-only formatting-arm
exclusion and the nightly ExecPlan's explicit local-hold case. No source or test
change was made. No findings remain undispositioned.

Follow-up: if maintainers want formatter contract coverage, prepare a dedicated
PMETPARA module-test-enhancement package that binds ADR-0021 obligations A-H and
meets the glue-tier `85%` line/region thresholds before remeasurement.
