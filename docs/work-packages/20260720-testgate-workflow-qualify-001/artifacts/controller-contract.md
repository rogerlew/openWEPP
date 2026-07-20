# Qualification Controller Contract

Status: scaffold contract.

The parent controller, not the black-box executor, owns admission and mutation
of the disposable scenario environments. Before execution it records:

- exact implementation commit and all frozen subject-root digests;
- qualification interface and matrix schema versions;
- ordered case IDs, injections, expected statuses, spawn counts, artifacts, and
  reuse/rejection sets;
- fresh case and provider artifact roots;
- existing queued/active provider run IDs; and
- the one permitted provider run or reusable run ID.

The executor receives the frozen implementation, case input paths, and output
root. It cannot edit repository source, policy, tests, workflows, or expected
results. The controller stops after the first unexpected result and preserves
all output. It does not repair, retry, reorder, or skip cases.

Provider execution is serial and ordinary. Before a manual dispatch, the
controller must prove no TESTGATE run is queued or active. It dispatches at most
one run and records its ID immediately. It never creates overlapping runs to
test concurrency; that behavior is exercised through the frozen hermetic
workflow harness and corroborated with retained provider records.
