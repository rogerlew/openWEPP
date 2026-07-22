# Coverage Closure

Status: PASS pending dual terminal verification.

Ran: the final changed-head traversal meets both ADR-0021 aggregate coverage
floors and every compiled-function region floor. The two functions below the
floor in the retained predecessor measurement now pass:

- `manifest_roots`: 45/53 = 84.9057% regions;
- `require_node_semantics`: 12/14 = 85.7143% regions.

Static: characterization changes are test-only and independently reviewed for
causality, fail-closed behavior, fixture confinement, and production identity.
