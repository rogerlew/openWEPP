# QA review — publication observer boundary

Reviewer: `/root/observer_qa` (independent secondary QA review). Reviewed
terminal source tree `85b8314efcd53ccb8111aa97af1f752ed49a99f46b805e754db5a1b2c7059963`
and frozen test binary
`9cf75b7337097df3454e2b301e2734565a36a3bdedbb5039b5df50d37abf0a49`.

**Static:** reviewed the adopted authorization, terminal four-file delta,
source custody, membership reconciliation, primary receipts, and reconstruction
reconciliation. **Ran:** this reviewer ran no model or test workload; findings
are based on the recorded terminal executions.

## Findings

No severity-bearing QA finding requiring a source correction.

The observer is cohesive: the serialization was removed from
`accepted_publication_support_capability.rs::install_validated_support`, and
the test-only observation occurs after the actual successful append through the
shared direct-consumer wrapper. The common write/flush `Result` flow classifies
the injected output error without changing the successful local append. This is
covered by the positive, N2 refusal, N12 late-owner rollback, and write-error
controls. The record correspondence receipt preserves byte-identical output
(`ffd2498b…`) for the positive and N12 paths, with zero records for N2 and the
failed measurement.

The unchanged capability guard/detection-control file is byte-identical. Final
mode listings retain all 14 promotion names and every pre-existing test's
metadata; the sole inventory addition is the write-error control, present in
all four modes. Final selected mode controls, builds, format, and parent,
credit, donor, install, and corruption reconstruction receipts pass on the
frozen source. Custody reconstruction applies the retained patch from the
authorized starting tree and reproduces all 747 source entries.

## Non-blocking debt / follow-ups

- **Low — `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs`:**
  strict Clippy remains a recorded failure (27 inherited diagnostics), and
  `--no-deps` remains a recorded failure (2,849 inherited diagnostics). The
  final strict comparison has no changed diagnostics. The no-deps comparison
  has a shifted pre-existing `clippy::struct_field_names` span after the three
  added audit counters; it does not increase the diagnostic count. This
  continuation does not close the repository-wide lint debt.

- **Gate classification:** `cargo deny` is **NOT_APPLICABLE** under
  testing-and-gate strategy §6.2.7: the terminal diff leaves manifests,
  lockfile, toolchain, dependency resolution, and policy bindings unchanged.
  Broad campaign/full-suite closure is **NOT_RUN** because it is outside the
  adopted bounded observer scope; neither is a missing required observer gate.
  The frozen-source named nextest controls, four-mode listings/builds, and
  unchanged reconstruction tools are the applicable evidence here.

## QA pass

QA accepts the bounded publication-observer continuation at the terminal frozen
source. The tests are non-vacuous around the external serialization boundary,
preserve the inherited local-append record meaning through outer rollback, and
keep failed measurement distinct from successful evidence.
