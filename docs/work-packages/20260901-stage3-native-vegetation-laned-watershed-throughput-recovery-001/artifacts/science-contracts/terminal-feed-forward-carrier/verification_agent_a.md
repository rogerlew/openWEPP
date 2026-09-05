# Independent contract verification A

Evidence mode: `Static + Ran + Expected-red`

Verifier: `rust_code_reviewer`

Ordered four-file manifest verified:
`a8a6677672c7f8f34acdaca30f3b94388902d022fe5886f28e4532762ffce804`

Verdict: `PASS-WITH-NOTES`

Implementation recommendation: `GO`

## Findings

No blocking findings.

Low-severity recordkeeping note: the `FFTC-A-003` action in `disposition.md`
says that 25 current-contract revision pins were reconciled. Independent
recount finds 26 `contract_version: 61` assertions in the integration target
and zero `contract_version: 58` assertions. All current pins are correct; the
stale count does not weaken the revision gate or block implementation.

## Accepted-finding verification

| Finding | Result | Independent verification |
|---|---|---|
| `FFTC-A-001` | `CLOSED` | The core and detailed `INV-SNOWENERGY-088` text makes the one-call rule local to one typed evaluator invocation, includes `CoveredTerminalExecutionMode`, forbids equal-payload reuse across invocations, and preserves distinct Full/Retry/Half1/Half2/Root, discovery/exact-endpoint, terminal-batch, and canonical final-map paths. `FinalAccepted` remains same-map pending-prefix completion rather than a provider replay. |
| `FFTC-A-002` | `CLOSED` | `FEED_FORWARD_TESTS` resolves to the package-owned `runoff_reconciliation/cqr_row5_tests.rs`. The expected-red assertion is expressly structural and the contract artifacts reserve acceptance for independently executed in-crate authentic-consumer behavior tests plus compile-time negative-capability evidence. Symbol presence cannot close `C-056`. |
| `FFTC-A-003` | `CLOSED` | Independent static recount finds 26 revision-61 assertions and no revision-58 assertion in the target. The complete target has 62 tests, 22 ignored, and 40 active; rerun produced 39 passes and exactly the named structural expected red. Historical process-version prose is not treated as a current metadata pin. |
| `FFTC-A-004` | `CLOSED` | The core Guard Map matches the source-real order: outer/provider validation; sole provider call and provider-wrapper `TerminalCustody`; evaluator boundary/result join as `Kernel` with `snow.terminal_trial_boundary_support_join`; `stage3_hourly_surface_energy` and required diagnostics as `Kernel` or `TurbulentTransfer`; terminal transition as `TerminalNumerics` or `Kernel`; then independent exact/final completion. Inspection of `evaluation.rs:478-553`, `02_guard_errors.rs:585-631`, and the provider wrapper around `snow_stage3_v11_terminal_execution.rs:680-695` confirms the named variants and order. No invented variant, hoisted validation, second-call repair, or fallback is authorized. |
| `FF-B-001` | `CLOSED` | Revision 61 retains 400 complete calls as the only direct exact-release pre-change observation. It explicitly labels 200 pre-change groups as an inference from static call structure and focused real capture. Exact 200-invocation role/path multiset evidence is mandatory only on postimplementation exact head, alongside unchanged `20/32/4` topology and `48/56/20/32/4` workload identity. |
| `FF-B-002` | `CLOSED` | The structural red reads the declared CQR source and currently fails first on the absent production request type. The package, contract reference, readiness matrix, and `C-056` separately require executable count/reference/path/poison/rollback tests, preventing the source-text gate from serving as behavioral acceptance. |
| `FF-B-003` | `CLOSED` | The retained baseline artifact exists and records Rust manifest `c300ea39...`, binary `1ba20247...`, `provider_carrier=2,049,833 us`, `run_wall_us=4,984,488 us`, and RSS `62,560 KiB` under the exact CPU-0 release command. The prospective ceilings are arithmetically exact baseline reductions of `750,000 us`: `1,299,833 us` and `4,234,488 us`. Three runs of one unchanged postimplementation binary must meet both median ceilings, every-run exact science/count/multiset identity, and every-run RSS `<=65,536 KiB`; any failure requires full revision-61 production reversion. |

Both final review artifacts were inspected. `review_agent_a.md` and the final
superseding sections of `review_agent_b.md` report `PASS` / `GO` on this exact
manifest; their finding dispositions agree with the canonical seven-column
record in `disposition.md`.

## Checks run

Ran the documented ordered manifest recipe. Result: `PASS`; exact digest
`a8a6677672c7f8f34acdaca30f3b94388902d022fe5886f28e4532762ffce804`.

Ran:

```text
nix develop --command cargo test --test snow_terminal_enthalpy_event_numerics_contract revision_61_binds_one_typed_feed_forward_call_per_logical_terminal_group -- --exact --nocapture
```

Result: `PASS`, 1 passed.

Ran the named structural seam test. Result: `EXPECTED_RED`, failing first and
only on absent `FeedForwardTerminalCarrierRequestV1`, as required before
production implementation.

Ran the complete integration target. Result: `EXPECTED_RED`, 39 passed,
exactly the named structural seam failed, and 22 historical tests were ignored.

Ran:

```text
.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md
bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md
nix develop --command cargo fmt --all -- --check
git diff --check -- <the ordered four manifest paths>
```

Result: `PASS`; 51 binding-exposure rows are consolidated, unit compliance has
no findings, formatting is clean, and manifest-file whitespace validation is
clean.

## Residual risk and missing tests

Production is intentionally absent at this contract-first checkpoint. The
postimplementation authentic-consumer behavior matrix, compile-time request
negative capability, adjacent competing-poison precedence, rollback and
publication checks, exact 200-invocation release multiset, closure identity,
and three-run numeric keep/revert evidence remain mandatory. This verification
authorizes proceeding to tests-first implementation; it does not approve
production retention or package closure.
