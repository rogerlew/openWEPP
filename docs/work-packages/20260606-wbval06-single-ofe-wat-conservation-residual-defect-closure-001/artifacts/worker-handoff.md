# Worker Handoff

Status: queued

Evidence mode: not-run

Current handoff: none. Execute package phases before creating follow-ons.

If package closes `HOLD`, the handoff must name:

- Defect or boundary ID.
- Observable failure and failing fixture.
- Suspected mechanism.
- In-scope write set for the owning follow-on.
- Correction authority.
- Acceptance target.
- Legitimate `HOLD` conditions.

Forbidden relay: no handoff may name only a next diagnostic step.

Static:

- Pending execution.

Ran:

- Not run.
