# TESTGATE Qualification Attempt 3

Evidence class: Ran.

- Provider run:
  [`30165527516`](https://github.com/rogerlew/openWEPP/actions/runs/30165527516)
- Base: `086244c889c20de823fd1fa5b02d3527ecffa236`
- Head: `955358449381ab38378d28dac93ba7b21b496d14`
- Result: `PASS / TERMINAL CANDIDATE`
- Receipt ID:
  `bd23f0956a2a851d224d73d72121c7e8f71ce5deb43a636422bab82065f50f68`
- Receipt SHA-256:
  `6d525e5b6ed29ce2e659b72ff0e3ae46ec5ced0f88ab20c7819c4453a5caf7f2`
- Terminal plan ID:
  `e67bdad2bdb24c5de6afa74fa1823362be892b3179102ddebc5e1f5cd9481d4a`
- Package authority chain:
  `d66752039c8d08179f6126f4978c94122b9ac603ff3762734fa18ef755c7b797`

The receipt, terminal plan, recovery predicate, and hosted native attestation
bind the exact repository, workflow, base, head, package path, and clean source
tree. Source mutation remained unchanged. The forest1 claim and final envelope
truthfully retain trust class `LOCAL_UNTRUSTED`.

## Selected Correctness Path

- LIGHT: 6/6 PASS.
- Canonical pre-heavy audit: 10/10 checks `PASS`, overall `READY`.
- HEAVY: 6/6 PASS.
- Overall blocking nodes: 12 passed, zero failed, blocked, retried, or skipped.
- Full-workspace Nextest ran from 16:46:04Z through 17:00:12Z.

The selected HEAVY nodes were three A1 native-canopy hard-invariant suites,
workspace Clippy, workspace doctest, and full-workspace Nextest.

## Quality Separation

Receipt, terminal plan, and pre-heavy audit all record:

- status `DEFERRED_TO_QUALITY_CI`;
- `closure_eligible: true`;
- observations `COVERAGE` and `CRAP`;
- owner `openwepp-quality-observatory`; and
- trigger `OPTIONAL_OPERATOR_DISPATCH`.

The 12-node plan has no prohibited coverage/CRAP gate definition. No coverage,
CRAP, QA, or CQR workflow executed.

## Recovery And Compatibility

The current receipt has 18 resume decisions: 12
`REJECTED_INCOMPATIBLE_RECEIPT` and six
`RERUN / NO_PRIOR_NODE_RECEIPT`. Eleven retained pre-split roots are freshly
rejected because `quality_disposition` is required and legacy quality-bearing
plans contain prohibited quality content. The prior narrow qualification run
is rejected separately for exact-checkout mismatch.

The retained archive contains 16 recovery roots and 460 indexed files. Its
76-entry ledger has zero hash-chain breaks and closes the current LIGHT, HEAVY,
and overall attempt as `PASS`. The attempt index and ledger are bound by the
authenticated recovery artifact.

Independent read-only evidence is retained at
`/home/workdir/openWEPP-quality-history/20260725-order6-run-30165527516`.
