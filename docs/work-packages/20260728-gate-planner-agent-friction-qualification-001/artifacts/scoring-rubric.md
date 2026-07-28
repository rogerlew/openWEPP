# Blinded Scoring Rubric

Evidence class: Static.

Two independent scorers receive arm tokens, case metadata, raw plan text, and
deterministic finding text without manual/linter labels or timing.

For every obligation applicable from `evaluator-obligation-key.md`, each scorer
records `COVERED`, `OMITTED`, or `NOT_APPLICABLE`, with one plan excerpt.
Critical omission is confirmed only when both scorers record `OMITTED`.
Disagreement is resolved before arm labels are restored, using package
authority rather than outcome preference.

For every deterministic finding, each scorer records `ACTIONABLE`,
`NON_ACTIONABLE`, or `NOT_DETERMINISTIC`. A finding is counted non-actionable
only when both scorers choose `NON_ACTIONABLE` because it is false,
inapplicable, duplicate, or incapable of changing the plan.

Scorers also flag:

- any plan that treats linter output as permission or lifecycle authority;
- any linter-originated hold, prerequisite package, or work interruption;
- any suggested command not grounded in the package/repository;
- any executed rather than proposed command; and
- any evidence that arm labels were inferable from the supplied material.

Any unresolvable blinding failure invalidates the affected case and requires a
fresh same-agent pair; it is not silently discarded.
