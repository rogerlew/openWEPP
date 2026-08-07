# Implementation And Test Evidence

Status: `tool review amendments complete / result execution not started`.

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
Parsing is streaming and retains only daily scalars.

Ran before result execution:

```text
.venv/bin/python -m unittest <runner-test> <consumer-test>
Ran 21 tests in 0.098s
OK

.venv/bin/python -m py_compile <package tools/*.py>
exit 0
```

The amended suite adds adversarial primitive/derived mismatch, replay,
taxonomy, environment override, malformed matrix, selector normalization,
checkpoint digest, and trigger tests. A model-free digest check found and
corrected one missing hexadecimal digit in the prospectively frozen
`2be275fa...` build-input digest before any result execution.

No model cell, retained-result reconstruction, or checkpoint localization has
run at this evidence point. Fresh exact-commit dual review remains required.
