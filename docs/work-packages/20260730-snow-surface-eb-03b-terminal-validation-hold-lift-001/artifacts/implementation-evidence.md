# Implementation Evidence

Status: `complete`

Evidence mode: `Ran`

## EB03B-CQR-001

Reproduction:

- `.venv/bin/python tools/local_ci/cqr_quality_evidence.py self-test`
- Result before correction: exit 2 after 115.77 seconds with
  `valid exact-head fixture was not CURRENT`.
- Retained diagnostic receipt:
  `disposition=INVALID`, reason
  `canonical CRAP registry has invalid adjudications`.

Mechanism: the synthetic handoff self-test loaded the ambient canonical CRAP
registry. The inherited production edits legitimately invalidate adjudication
hashes, so optional QA debt contaminated a protocol-mechanics fixture.

Correction: `intake(args)` continues to load canonical quality/workflow modules.
A private `_intake(args, quality, workflow)` seam lets only `self_test()` pass a
controlled CRAP loader returning no adjudications for its synthetic report.
The loader is restored in `finally`. Real CLI inspection and all non-self-test
calls retain canonical registry validation and fail-closed behavior.

After correction:

- Python self-test: PASS in 118.88 seconds.
- `cargo nextest run --test cqr_quality_evidence_handoff_contract --profile full
  --no-fail-fast`: PASS 4/4 in 116.696 seconds, run
  `1056ac61-bedb-4967-8c70-275ad270db3f`.
- CURRENT, corrupt-artifact INVALID, and stale-identity outcomes all remain
  asserted.

## EB03B-ASSURE-001

Mechanism: two individual Rust tests serially constructed multiple complete
approved assurance trees. The matrix-level runtime, rather than any publication
operation, reached the unchanged 720-second nextest timeout.

Correction: split the two tests into 14 named independent tests. Every original
fixture mutation and assertion remains byte-for-byte equivalent within its own
test: unowned README, empty directory, narrative drift, staging symlink,
aliased usersum root, staging FIFO, wrong principal kind/role/domain, missing
competence/independence, withdrawn/superseded report, and missing release
transfer.

After correction:

- selected 14-case set: PASS 14/14 in 557.596 seconds, run
  `0c91b1f7-b15d-4cc1-afeb-d039d02ff342`;
- complete assurance publication binary: PASS 37/37 in 1811.491 seconds, run
  `d316acd6-71e2-4b3a-914d-90c9100fa5fa`;
- configured timeout and two-way assurance-publication concurrency unchanged;
- no production assurance code or authority changed.
