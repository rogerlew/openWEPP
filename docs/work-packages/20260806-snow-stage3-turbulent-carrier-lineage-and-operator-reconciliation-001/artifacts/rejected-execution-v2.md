# Rejected Execution V2

Status: `REJECTED BEFORE RESULTS`.

Evidence class: `Ran`.

Exact independently admitted execution commit:
`e591d89c219d69f619e68f9aa7194f88d20f9a1c`.

Command:

```text
.venv/bin/python docs/work-packages/20260806-snow-stage3-turbulent-carrier-lineage-and-operator-reconciliation-001/tools/run_operator_reconciliation.py --expected-head e591d89c219d69f619e68f9aa7194f88d20f9a1c
```

The command exited `1` after `404.70 s`. All 12 control/paired/sequential model
lanes completed, but independent reconciliation failed before any metric,
decision, receipt, manifest, or compact result was produced:

```text
RuntimeError: sequential state continuity mismatch: total_layer_state_fingerprint_before_fnv1a64
```

The first mismatch in every sequential site occurred between substeps 0 and 1.
The producer captured substep 0's after-state before the deterministic
alignment, control-volume temperature normalization, and fragment coalescence
performed at the start of substep 1. Consequently the serialized after-state
was not the exact next before-state; Paradise and Snowbird also exposed a
one-ULP total-cold summation difference. The frozen exact-continuity consumer
correctly rejected the evidence.

The ignored namespace
`target/snow_stage3_operator_reconciliation_v2/` is retained read-only and must
not be overwritten or reused. It contains `137` files, including all 12 run
manifests, `12` WAT, `12` HBP, `12` loss, and `8` schema-v6 snow-trace outputs.
No `results/`, `execution-receipt.json`, or
`retained-artifact-manifest.json` exists.

Retained custody hashes:

- execution binary:
  `61a18a1d91384b2df4ef31a9a198ba8f5b5d5a5f74bf91695ea3486ba328800b`;
- retained protocol at execution:
  `fd3b0b78452cb1f2a2251db4096007cb0145c23cc7f393228477e83d8a189c6a`;
- top-level stderr:
  `269697b46636362e0f977679a0c3f1e1ec807ebfa25f81344d73df9650e721fa`.

Disposition: preserve the exact-continuity requirement and move the producer's
deterministic next-control-volume preparation inside the serialized transition
boundary. Any corrected execution requires renewed result-blind admission and
uses the new v3 namespace.
