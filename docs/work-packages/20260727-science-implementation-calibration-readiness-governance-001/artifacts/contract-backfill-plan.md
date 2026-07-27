# Calibration-Posture Contract Backfill Plan

Evidence class: `Ran inventory + Static migration disposition`

Inventory date: `2026-07-27`

The canonical contract tree contains 39 `SC-*` contracts. None had the new
`Calibration and Identifiability` heading or explicit
`CALIBRATION_NOT_APPLICABLE` token before ADR-0042.

Existing contracts remain conformant under the legacy schema baseline. The
new section and readiness-matrix bindings become mandatory for:

1. every new `SC-*` contract; and
2. an existing contract at its next material scientific amendment.

Owner: science-contract governance and the work package authoring the material
amendment.

Trigger: a change to equations, algorithms, process parameters, parameter
meaning/domain, observation mapping, calibration authority, or empirical
claim posture. Editorial-only, provenance-only, or mechanical formatting
changes do not trigger backfill.

Closure evidence at each trigger must record the three ADR-0042 status fields,
the applicable readiness obligations, evidence paths, skip rationales, and
data-role separation. Repository-wide eager backfill is not required by this
governance package.
