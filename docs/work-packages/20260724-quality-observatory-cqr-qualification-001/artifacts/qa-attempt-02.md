# QA Attempt 2

Evidence class: Ran.

- Provider run:
  [`30179148269`](https://github.com/rogerlew/openWEPP/actions/runs/30179148269)
- Source/workflow head:
  `c17f49d9bda46f2f6ea4d64fc9db5e41dbd4093b`
- Qualification TESTGATE run: `30177394609`
- Result: `EXECUTION_FAILED`
- Child exit: `2`
- Full log SHA-256:
  `6a5c3409002ae9a75eb1d49fd617fb7f5f561d5509622e1f0a6bf0a1d324a44c`
- Retained tail SHA-256:
  `5b0bddb36fee17cb70bc9d3dd709474da341359e79c2d6abacdc62b2835aa4f6`

Exact preflight, qualification, source freeze, forest1 labels, and empty
TESTGATE occupancy passed. The corrected control envelope independently
validated, and the decoded 32 KiB tail matched its digest.

Three verifier tests failed because their nested `openwepp-runner` lib-test
link terminated with signal 7 (`Bus error`). Cargo exited 101 and repeated
`cargo-llvm-cov rustc -vV` probes failed afterward. This is an infrastructure
failure, not a test assertion, measurement verdict, science result, or source
identity failure.

The failure occurred in `full`; `science-manual`, inventories, JUnits,
snowbench reconstruction, merged LCOV, CRAP, publication, and evidence ID were
not reached.

Disposition: canonical one-time infrastructure retry is authorized for this
unchanged exact head. A second occurrence blocks further unchanged retry.

Read-only evidence is retained at
`/home/workdir/openWEPP-quality-history/20260725-order7-qa-attempt2-run-30179148269`.
