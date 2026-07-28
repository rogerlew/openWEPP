# Friction Baseline And Success Metrics

## Baseline

The retained trajectory assessment records 122 scoped commits, 10,417 net
lines, 32,772 tracked gate-related lines, 22 workflow runs consuming 7.62
hours, and no successful observed end-to-end TESTGATE transaction. The CAL-04B
wall-time audit records about 18 active hours, eight spawned prerequisite
packages, and zero of 9,261 candidate configurations executed before the first
new synthetic-design failure. These are diagnostic baseline observations, not
a claim that every line or hour was valueless.

The qualification package must also measure a fresh manual baseline on the same
people/agents, package snapshots, and tasks used for the linter comparison.

## Qualification Cohort

Use at least six real packages: two documentation, two non-kernel Rust, and two
kernel/science packages, including one calibration package. Measure all three
observation modes without running suggested commands as linter time.

Freeze an evaluator-authored obligation key for every package/mode before
trials. Use at least 18 paired cases. Counterbalance order: half of cases run
manual then linter and half linter then manual, assigned by a recorded seeded
shuffle. Each arm starts from the same immutable snapshot and brief. Discard
one separately identified warm-up case per agent; warm-ups are not in the
cohort.

Definitions:

- `planning time` starts when the brief and snapshot are exposed and stops when
  the agent submits an exact proposed command/obligation plan; command runtime
  and execution are excluded;
- a `plan-construction interaction` is one user clarification or one outer tool
  invocation between those events; batched reads in one invocation count once;
- a `critical obligation` is a key item whose omission could invalidate
  correctness, science, security, protected-data custody, package scope, or
  truthful closure, confirmed before unblinding by two reviewers;
- a deterministic finding is `non-actionable` when both reviewers agree its
  cited observation is false, inapplicable, duplicate, or cannot change the
  plan; the rate is non-actionable deterministic findings divided by all
  deterministic findings, with zero findings reported separately rather than
  assigned a zero denominator; and
- a `work interruption` occurs when a linter issue stops the originating task
  for more than five minutes, triggers escalation, or causes a separate repair
  package.

Record transcripts, monotonic start/stop timestamps, invocation counts, tool
version, host, cold/warm state, order, and reviewer scores. Reviewers are blind
to arm labels while scoring obligation coverage and actionability.

## Required Outcomes

- zero reviewer-confirmed critical obligation omissions;
- no more than 10% non-actionable deterministic findings;
- at least 30% reduction in median planning time;
- at least 50% reduction in plan-construction interactions;
- at most 5 seconds warm and 15 seconds cold per invocation;
- at most 3,000 non-test production lines;
- no daemon, database, ledger, receipt, CI workflow, remote identity, recovery,
  publication, or lifecycle state;
- zero linter-originated holds or prerequisite packages.

Report paired per-case results, medians, and bootstrap confidence intervals.
All qualification-period investigation and maintenance minutes attributable to
false findings, unavailable analysis, or linter defects are divided by the
number of scored linter cases and added to every linter case's planning time.
The 30% threshold applies to this adjusted median. Interruption and noise
thresholds use the definitions above.

## Stop-Loss

Disable the linter path if it misses a critical obligation, writes or executes
outside the allowlist, creates a lifecycle effect, exceeds the noise threshold,
fails the median planning-time target, exceeds the production-line budget, or
causes two work interruptions. Originating work continues manually.

Expansion or repair after a stop-loss requires explicit user authorization.
No automatic linter-repair campaign, compatibility layer, or new prerequisite
package is allowed.
