# Heavy Attempt 11: Science Confinement

Evidence class: Ran / Static.

Attempt:
`/home/workdir/openWEPP-quality-history/20260724-order3-local-attempt11-TyJumg`.

Executed head:
`125819092583c3304fa1e173b4f6e82fcb81c7be`.

Admission ID:
`0d4ab87cc20864744eea88db73f713faded564ca53ece9bcb0fad92a94e1ad3c`.

## Result

Admission was `READY` with 2,279 `full`, 36 `science-manual`, and 2,315
canonical workspace tests.

The instrumented `full` profile passed all 2,279 tests with 31 configured
skips and 15 slow tests in `2299.240s`. The built-in post-full executable and
working-tree identity check passed.

The `science-manual` profile ran all 36 tests: 35 passed, one failed, one was
slow, and 2,286 nonselected tests were skipped in `480.503s`. The sole failure
was:

`snowdensity03_physics_bulk_offline_contract::physics_bulk_runtime_mentions_are_confined_to_authorized_opt_in_surfaces`

It reported `tests/integration/testgate_ci_executor_contract.rs` as an
unauthorized `physics_bulk` mention.

The terminal read-only comparison still found all 291 executable rows and the
working-tree identity equal to admission. Source and snapshot remained clean
at the executed head.

Merge, CRAP evaluation, snowbench disposition, publication, and terminal
verification did not run. Published files: 0. No quality evidence ID was
issued.

## Root Cause And Correction Intent

Order 2 added a source-contract assertion that names the
`snowdensity03_physics_bulk_offline_contract` binary to prove that the `full`
profile excludes it and `science-manual` selects it. The older science
confinement allowlist scans every Rust integration test containing the token
`physics_bulk`, but does not include this governance-only Nextest
configuration consumer.

The package write set is amended before implementation to include the exact
science confinement test. The correction will add only
`tests/integration/testgate_ci_executor_contract.rs` to its explicit allowed
test surfaces. It will not add a production, runtime, diagnostic, selector,
physics, or publication consumer and will preserve rejection of every
unlisted path.
