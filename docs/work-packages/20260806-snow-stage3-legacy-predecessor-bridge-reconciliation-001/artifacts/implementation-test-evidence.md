# Implementation And Test Evidence

Status: `tool implementation PASS / retained execution PASS`.

Evidence mode: `model-free tests run; model results remain blinded`.

Implemented separate package-local execution and independent-consumer tools.
The execution tool owns source clones, package-local offline Cargo custody,
binary and sidecar identity, normalized semantic-input manifests, the four
endpoint cells and controls, explicit-selector equivalence arms, runtime
manifest validation, a fail-closed retained verifier, the conditional 14-source
checkpoint path, and protected HBP/WAT/loss identity. It performs no science
reduction. The consumer does not import it and independently adapts schema v4
and v6 before water-year reduction and factorial contrasts. Schema-v6
primitive equations use the prior reviewed independent operator consumer,
identified and hashed at execution, rather than producer or runner reductions.
Parsing is streaming and retains only daily scalars. Conditional checkpoint
execution is mandatory even when the independently derived trigger selects no
lanes. Triggered checkpoint reconstruction accepts the prospectively frozen
v4/v5 aggregate-only adapters and the v6 primitive adapter, replays the first
and last checkpoint against their forcing-matched endpoint cells before
localizing a transition, and fails closed on any execution, binary, semantic
input, chronology, protected-output, or retained-inventory mismatch.

Ran before result execution:

```text
.venv/bin/python -m unittest discover -s <package-tools> -p 'test_*.py'
Ran 42 tests in 0.142s
OK

.venv/bin/python -m py_compile <package tools/*.py>
exit 0
```

The amended suite adds adversarial primitive/derived mismatch, replay-failure
taxonomy, endpoint-anchor and ordered-transition helpers, checkpoint-only v5
custody, per-WY/median gates, malformed checkpoint receipts, environment and
HEAD drift, malformed matrices and arms, selector normalization, exact
protected-output keys, binary hash/size drift, semantic-input mutation,
complete inventory additions, overwrite refusal, checkpoint digests, and
trigger tests. Full synthetic conditional-path tests cover no trigger, one
triggered lane with no adjacent divergence, two lanes localizing the same or
different intervals, and endpoint-anchor rejection. Independent mutation tests
cover every frozen common fixture and actual protected-output bytes. A
model-free digest check found and corrected one missing
hexadecimal digit in the prospectively frozen `2be275fa...` build-input digest
before any result execution.

Final exact-commit result-blind review returned `PASS/PASS` at
`cb31e6f4d06fd66a3ef5b3a7711a095b3f3d84f4`. The required comparator then ran
the four endpoint cells and independent consumer at that exact clean SHA.
Execution, reconstruction, the explicit no-trigger checkpoint phase, runner
verification, and consumer verification all passed. The consumer verified
`110747` retained artifacts; exact timings and hashes are in
`gate-results.md` and `independent-reconstruction.md`.

Post-result candidate checks reran the package suite (`42/42`) and focused
contract selection (`12/12`). Canonical Markdown lint passed for 39 package,
catalog, and roadmap files. Assurance validation and planning pass with the
snow/frost report still `DRAFT`, generation `7d1a3ba1`, and zero public
reports. Temporary-root build/check, the governed review-draft renderer, and
the release/export guard pass. The renderer changed only its three expected
nonpublic v130-bound draft files; no review, approval, release, export, or
publication event was created.
