# F quality gate plan

Static: source-only planning; no Clippy/build executed by implementation worker. Exact documentation paths passed to find-agents; root/work-package guidance applies. Source remains frozen during parent release build.

## Existing evidence

`raw/A-gates-02-clippy.log.json` records A checkout `2b56e6ebc9c8e8073fb68b89626d4d4e2b4602e3`, exit 101, 45.820087486 seconds, and log SHA256 `fe4a96fa81eccf4b7b89cb84b57f84bf2d6ec77f8c96c44c6dc527f0db955772`. Command selected land-surface-energy, hillslope-orchestrator and runner, all targets, `-D warnings`. The log ends with orchestrator lib-test compilation aborted after 1374 errors. This is a failed baseline quality gate, not 1374 F regressions and not proof all downstream targets were checked. Its `.json` file is execution metadata, NOT structured compiler diagnostics.

## Exact prospective execution

Parent executes serially from each frozen A and F root with identical environment (`RUST_MIN_STACK=67108864`, `CARGO_NET_OFFLINE=true`), recording toolchain, source manifest, command, exit status and complete output:

```sh
nix develop --offline -c cargo clippy --offline -p openwepp-land-surface-energy -p openwepp-hillslope-orchestrator -p openwepp-runner --all-targets --message-format=json -- -D warnings
```

This matches the failed baseline package/target/lint selection, adding structured output only. Do not compare a narrow F run against the broad A total. If dependency failure hides orchestrator diagnostics, a supplementary matched A/F `-p openwepp-hillslope-orchestrator --all-targets` run may localize, but cannot replace the broad gate. Keep stderr and stdout; Nix banners are non-JSON, so parse only valid Cargo `compiler-message` records. Do not suppress inherited lints or set `--cap-lints` to obtain green. Baseline rerun is needed for machine-exact matching; the retained text log remains historical evidence.

## Delta classification

Normalize absolute worktree prefixes to repository-relative paths, not diagnostic semantics. Match multisets by package, target kind/name, diagnostic code, message, and primary-span source text plus enclosing function. Preserve multiplicity and secondary spans. Ignore shifted line numbers only after `git diff --unified=0 A -- <file>` or unchanged-source context maps the occurrence. Record each F diagnostic as inherited unchanged, relocated unchanged body, new/changed F, common protocol-test delta, or unresolved. Moving an existing diagnostic to the extracted helper requires actual A code/message correspondence, not merely finding the same lint elsewhere. New file alone does not prove new lint; old file alone does not prove inheritance. Changed expression/new test with no matched A occurrence is new. Unmapped/compiler-truncated target coverage remains unresolved, never zero by subtraction. A raw lower total cannot establish no regressions.

Primary F mapping: new `terminal_carrier_provider.rs` to added request/enum/adapters; new `terminal_carrier_evaluation.rs` to baseline `stage3_solver/evaluation.rs` coupling/boundary/flux body plus new dispatch/reference helper; new lower `terminal_feed_forward_tests.rs` and attachment `snow_stage3_v11_terminal_feed_forward_tests.rs` to entirely new tests/oracle. Existing support.rs API factoring, actual execution callback, carrier typed signatures/private comparator, snow_boundary.rs signatures, adaptive oracle fixture and source guards map against their baseline functions. Common 24-line integration provenance-test correction is separately tagged common, not F physics. Reexports and cfg(test) changes require both lib and lib-test diagnostic review.

## Static watch list, not diagnosed failures

The crate enables Clippy all and pedantic. New physical-request value passing followed by clone may raise `needless_pass_by_value`; the cfg(test) feedback delegation's `*provider` may raise `explicit_auto_deref`; the oracle thread-local nested tuple may raise `type_complexity`; new oracle/lower test helpers may raise `too_many_lines`, `too_many_arguments`, or test-fixture scalar/style lints. Existing allowances cover some, not all, of these sites. Only executable diagnostics establish actual violations. Inherited old-request dead-field warning was deliberately preserved with derived Debug; do not alter audit representation to hide it. Fix confirmed new diagnostics only after parent unfreezes exact source and reviewers can inspect the delta; no blanket allows, physics changes, or inherited-warning cleanup under this treatment.

## Disposition

Report global Clippy FAIL if it fails. A defensible incremental statement is separately worded: no new F diagnostics among fully compared targets, with inherited counts, relocated matches and unresolved/aborted coverage listed. Full green cannot be claimed from successful compilation, five focused tests, static inspection, or baseline debt. Any new F diagnostic or unmatched target is an explicit pending gate until corrected/adjudicated.
