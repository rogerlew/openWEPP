# Independent contract review A — final corrected-manifest rereview

Evidence mode: `Static + Ran + Artifact-reported`

Reviewer: `rust_code_reviewer`

Ordered manifest reviewed:
`a8a6677672c7f8f34acdaca30f3b94388902d022fe5886f28e4532762ffce804`

Verdict: `PASS`

Promotion recommendation: `APPROVE`

## Findings

No findings.

## Disposition verification

| Finding | Final result | Evidence |
|---|---|---|
| `FFTC-A-001` | `CLOSED` | `INV-SNOWENERGY-088` scopes exactly one call to one typed evaluator invocation, includes enclosing execution mode, forbids equal-payload reuse, separates Full/Retry/Half1/Half2/Root and discovery/exact invocations, preserves the independent batch path, and makes `FinalAccepted` same-map completion rather than a replay (`SC-SNOWENERGY-001.md:3403-3440`). This preserves `INV-SNOWENERGY-034` step doubling. |
| `FFTC-A-002` | `CLOSED` | The named expected red is structural preimplementation classification only. Postimplementation acceptance is assigned to independently executed package-owned in-crate behavioral tests and compile-time negative-capability evidence (`contract_ref.md:40-51`; `readiness-matrix.md:24-26,34-41`). |
| `FFTC-A-003` | `CLOSED` | Current-revision assertions in `snow_terminal_enthalpy_event_numerics_contract.rs` are reconciled to revision 61. The recorded whole-target result is 39 passed, exactly the named structural expected red failed, and 22 historical tests ignored (`contract_ref.md:68-78`). |
| `FFTC-A-004` | `CLOSED` | The core invariant now distinguishes provider-wrapper `TerminalCustody` from evaluator boundary/result `Kernel` (`SC-SNOWENERGY-001.md:1338`). The Guard Map gives the source-real order: provider validation and sole call; evaluator result join as `Kernel` with `snow.terminal_trial_boundary_support_join`; `stage3_hourly_surface_energy` plus required diagnostics as `Kernel`/`TurbulentTransfer`; then terminal transition as `TerminalNumerics`/`Kernel` (`:1452`). This matches `evaluation.rs:478-553` and preserves adjacent competing-poison authority without hoisting validation or inventing variants. |
| `FF-B-001` | `CLOSED` | `OBL-SNOWENERGY-C-056` treats 400 calls as the only direct exact-release observation, labels 200 groups as a pre-change inference, and reserves the exact 200-invocation multiset claim for the postimplementation exact-head run (`SC-SNOWENERGY-001.md:3453-3465`). |
| `FF-B-002` | `CLOSED` | The structural red reads package-owned `cqr_row5_tests.rs`; the contract artifacts expressly deny it behavioral acceptance authority and require executable authentic-consumer tests after implementation. |
| `FF-B-003` | `CLOSED` | The prospective baseline and keep/revert arithmetic are exact: `2,049,833 - 750,000 = 1,299,833 us` and `4,984,488 - 750,000 = 4,234,488 us`. Three unchanged-binary CPU-0 runs must satisfy both median ceilings, exact science/count/multiset identity, and per-run RSS `<=65,536 KiB`; any failure requires full revision-61 production reversion (`SC-SNOWENERGY-001.md:3489-3497`). |

## Residual risk and missing tests

- Production is intentionally absent. The structural expected red may be
  satisfied by source text alone, but the contract correctly prevents that
  assertion from closing behavioral acceptance. The required one-call,
  forced-two-call oracle, role/path separation, negative-capability,
  competing-poison, rollback, exact 200-invocation multiset, and numeric
  keep/revert evidence remains mandatory postimplementation work.
- Static source inspection supports the feed-forward premise: the real common
  carrier does not consume the generic evaluator's preceding ending hint or
  coupling-iteration ordinal. Complete transition and custody equality must
  still be proved at the authentic consumer after implementation.
- Revision-58 pins remain in other snow-energy integration targets outside the
  ordered manifest. They do not contradict the narrowly reported target result,
  but no full snow-energy integration-suite green claim should be made until
  exact-head implementation validation reconciles or justifies them.
- The reported 39-pass / one named expected-red / 22-ignored integration result
  was reviewed from the package evidence and was not rerun in this narrow final
  rereview.

## Checks run

Ran the exact ordered manifest recipe. Result: `PASS`; the digest matched
`a8a6677672c7f8f34acdaca30f3b94388902d022fe5886f28e4532762ffce804`.

Ran:

```text
.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md
bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md
```

Result: `PASS`; strict BEI reported 51 fully consolidated rows and unit
compliance reported no findings.

Revision 61 now provides unambiguous contract-first authority for the
invocation-local one-call scheduling change, preserves all numerical and path
independence required by `INV-SNOWENERGY-034/086`, and supplies sufficient
postimplementation behavioral, rollback, closure, workload, and numeric
keep/revert gates. No correctness blocker remains in the reviewed manifest.
