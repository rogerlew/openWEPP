# Workflow Validation Review

Evidence class: `Static`

Reviewer role: independent workflow-validation subagent

## Findings

1. `BLOCKER`: no QA workflow exists.
2. `BLOCKER`: CRAP is embedded across TESTGATE policy, publication,
   conservative fallback, release validation, and contract tests.
3. `BLOCKER`: the collector executes exactly one profile and therefore cannot
   observe moved snowbench tests.
4. `BLOCKER`: actionable CRAP currently exits nonzero; observational debt needs
   a successful execution-integrity result distinct from `debt_status=FAIL`.
5. `HIGH`: forest1 occupancy and retired-Omarchy filtering need a machine
   helper with fail-closed fixtures.
6. `HIGH`: separate concurrency groups alone do not prevent a dispatch race;
   QA must recheck before acquisition and use a bounded priority lease or
   equivalent mechanism.
7. `HIGH`: canonical standards conflict on canceling versus ignoring retired
   runner records.
8. `HIGH`: release validation automatically recollects CRAP today.
9. `MEDIUM`: pre-split retained receipts need an incompatible-recovery fixture.

Initial recommendation: separate changed-head TESTGATE qualification from
QA/CQR functional qualification.
