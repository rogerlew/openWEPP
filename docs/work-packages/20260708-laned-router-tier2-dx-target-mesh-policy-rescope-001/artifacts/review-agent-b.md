# Review Agent B

Status: QUEUED
Evidence mode: not-run.

Reviewer instructions:
- Lead with severity-ordered findings and `file:line` references.
- Independently check fixture authority, timing provenance, comparator surfaces,
  and active production consumer claims.
- Check that every unmet gate is held with a named blocker and not deferred
  while marked complete.

Finding disposition template:

| ID | Severity | Finding | Required disposition |
|----|----------|---------|----------------------|
| | | | `accepted` / `rejected` / `deferred` / `follow-up` |
