# Implementation Intent

Evidence class: `Static`.

Intent: CAL integration/custody tooling migration.

This package changes no model or calibration design. It replaces the
planner-owned path with direct package commands, durable primary-failure
evidence, bounded publication, and the smallest protected holdout owner.

Selected validation is focused Python unit/contract testing, JSON and source
scans, documentation lint, exact-diff reconciliation, dual review, and dual
verification. No CAL population, Harvard access, planner, TESTGATE, CI, full
workspace, comparator, or publication run is selected.
