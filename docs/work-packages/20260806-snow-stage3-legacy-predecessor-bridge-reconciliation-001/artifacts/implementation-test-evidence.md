# Implementation And Test Evidence

Status: `second tool review amendments complete / result execution not started`.

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
Ran 35 tests in 0.097s
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
trigger tests. A model-free digest check found and corrected one missing
hexadecimal digit in the prospectively frozen `2be275fa...` build-input digest
before any result execution.

No model cell, retained-result reconstruction, or checkpoint localization has
run at this evidence point. Fresh exact-commit dual review remains required.
