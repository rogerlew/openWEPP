# Independent Review A

Evidence class: `Static`

Disposition: `PASS`

Review A initially held on writable-root aliasing, the stale executor validator,
incident-004 status remnants, missing command binding in the opening token, and
insufficient direct-plan structural validation. The accepted corrections:

- reject custody/output roots that overlap the repository, Harvard, the
  calibration attempt, execution objects, or each other;
- migrate `validate_executor.py` to the direct JSON plan;
- make incident 005 the consistent current science hold;
- bind the resolved sandbox invocation into the fsynced opening token; and
- require exact phases, command inventory, order, predecessor chain, field
  types, literal executable allowlist, and Harvard policy.

Negative tests cover plan drift and adversarial writable-root paths. A final
low observation that preflight-only consumed its output root was also corrected.
No closure blocker remains. The reviewer ran no CAL command, Harvard read, or
model execution.
