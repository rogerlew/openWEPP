# Implementation Gate Evidence

Evidence class: Ran.

## Focused Runs

| Invocation | Result | Disposition |
|---|---|---|
| `cargo nextest run -p openwepp-gate-planner --profile quick` (`275c6c8c-fce7-46c4-bb0e-bb5d57d93a34`) | 40 pass, 9 fail, 20 cancelled | One common cause: the scaffold's canonical strategy edit had not updated the impact-map digest. The digest and generation were corrected before continuing. |
| Same crate-focused command (`eff5624c-d034-4f23-9954-3959c3e113c2`) | 68 pass, 5 fail in 447.856 seconds | Three failures exposed an all-light compatibility receipt defect; two fixture verifiers correctly rejected the dirty committed-checkout precondition. |
| Exact three failed executor tests after `FINAL_LIGHT` correction | 3 pass, return code 0 | No broader rerun; only the production-path failures were repeated. |
| Exact two clean-checkout verifier tests | 2 fail in 991.474 seconds | Clean checkout closed the original precondition. Both then exposed the same fixture-only boundary: an `INTENT` plan was being treated as an executable terminal heavy receipt. |
| `heavy_audit_is_required_only_at_the_terminal_execution_boundary` | 1 pass, return code 0 | Fast seam proof: synthetic intent evidence remains usable for verifier fixtures, while a terminal heavy receipt without a `READY` audit is rejected. The two 16-minute fixture workflows were not repeated. |
| `heavy_handoff_accepts_only_checkpoint_bound_light_artifacts` | 1 pass, return code 0 | Exact handoff seam: the frozen light artifact is accepted and a byte mutation is rejected with `GATE-EXEC-CHECKPOINT-ARTIFACT-DRIFT`. |
| `cargo nextest run --test testgate_align_authority_contract --test testgate_ci_executor_contract --profile quick` | 15 pass, return code 0 | Final focused integration evidence before implementation commit. |
| `.venv/bin/python -m unittest tests/python/test_testgate.py` | 7 pass | Includes append-only history, attempt index, package authorization, and qualification subject-freeze cases. |
| Scoped `markdown-doc lint` | 30 files, 0 errors, 0 warnings | Package, standards, and local/release tooling documentation. |
| `git diff --check` and `cargo fmt --check` | return code 0 | Final focused static checkpoint. |

The two clean-checkout verifier cases were executed after the implementation
commit. Their first valid context exposed one shared boundary defect. A focused
seam test proves its correction; repeating two 16-minute reconstruction-heavy
fixtures would duplicate unchanged planner and artifact work and is deferred
to the authenticated terminal plan.

## Staged-Light Attempts

The first staged-light attempt stopped during reconstruction/preflight with
`GATE-EXEC-ENVIRONMENT-MISSING: CARGO_HOME`. It created no node attempt,
checkpoint, or selected-gate output. The cause is retained as
`TGCA-ENV-OPTIONAL-001` in the hash-chained ledger. Standard Cargo/rustup
default variables are now optional projections; `PATH` remains required.

The one replacement staged-light attempt passed all five selected light nodes
and wrote five per-node checkpoints. Its stage receipt is
`549c2d9efd90ae9e036155d933a1f95179b6708ab4ea55d47a7c17b09786ee5c`;
measured stage time was 76.754 seconds. That receipt became stale after the
subsequent audit-report, qualification-interface, and resume corrections, so
it is retained as evidence and is not eligible for final heavy admission.

## Repeat Accounting

- Full or campaign-strength heavy gates executed by the parent: **0**.
- Selected light nodes repeated after a completed result: **0**.
- Focused crate command repeated after an observable completed result: **2**.
  The first repeat followed the impact-map correction; the second obtained a
  durable return code after an orchestration capture lost the prior terminal
  status and its JUnit was overwritten. Subsequent correction reruns were
  restricted to the exact failing tests.
- Schema-contract reruns were restricted to the failed test or the two schema
  tests whose inputs changed.

No reassurance rerun is accepted as closure evidence.

The first exact-head audit reported `READY`, but no heavy dispatch occurred:
pre-dispatch inspection proved heavy preflight would reject the existing light
outputs. The corrected executor permits only audit-admitted light node IDs and
revalidates each checkpoint/node/artifact digest. Because this correction
changes execution identity, the earlier plan, light receipt, and audit are
retained but cannot authorize the final heavy stage.
