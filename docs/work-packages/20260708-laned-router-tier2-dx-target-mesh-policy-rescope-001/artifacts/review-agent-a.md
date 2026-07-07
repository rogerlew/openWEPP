# Review Agent A

Status: QUEUED
Evidence mode: not-run.

Reviewer instructions:
- Lead with severity-ordered findings and `file:line` references.
- Check gate legitimacy, not just artifact presence.
- Check that H2637 is not used as fleet-general proof.
- Check contract-first sequencing for any mesh-policy implementation.
- Check no hybrid code, selector, or `SC-OFEROUTE-002` path is revived.

Finding disposition template:

| ID | Severity | Finding | Required disposition |
|----|----------|---------|----------------------|
| | | | `accepted` / `rejected` / `deferred` / `follow-up` |
