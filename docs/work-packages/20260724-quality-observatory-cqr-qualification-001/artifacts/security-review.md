# Security Review

Evidence class: Ran + Static.

Disposition: `PASS`.

- TESTGATE authority verification binds repository, workflow, source ref,
  exact head, run, receipt, and terminal plan; both superseded-head checks
  passed.
- QA accepts only a successful exact-head TESTGATE run, freezes the source,
  owns an exclusive forest1 lease, and publishes only the indexed 11-file
  allowlist below the 100 MiB ceiling.
- Canonical JSON, file digests, payload-derived identity, envelope, COMPLETE
  control receipt, source/toolchain/control identities, and current repository
  state are independently checked by CQR.
- Selection-only intake cannot recollect. A byte-mutated publication failed its
  control binding with exit 2 and `collection_launched=false`.
- No secret, credential, raw LCOV, `.profraw`, target, cache, reconstruction,
  temporary tree, or compressed bypass was published.

No accepted security finding remains.
