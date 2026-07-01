# Verification

Status: `EXECUTED-HOLD`

Dual verification is required before closure. Label evidence class as
`Static:` or `Ran:` and verify gate legitimacy, not just artifact presence.

## Parent Verification

Evidence class: `Ran:` plus `Static:`

Ran:

```text
cargo check -p openwepp-watershed-orchestrator -p openwepp-runner --bins --tests
```

Result: `PASS`.

```text
cargo fmt --check
```

Result: `PASS`.

```text
cargo clippy -p openwepp-watershed-orchestrator -p openwepp-runner --all-targets -- -D warnings
```

Result: `PASS`.

```text
cargo test -p openwepp-runner --test watershed_cli_behavior_contract wshedw4 -- --nocapture
```

Result: `PASS`, `1` test passed.

```text
cargo test -p openwepp-runner --test watershed_cli_behavior_contract wshedw3 -- --nocapture
```

Result: `PASS`, `3` tests passed.

```text
cargo test -p openwepp-runner --test watershed_cli_behavior_contract -- --nocapture
```

Result: `PASS`, `24` tests passed in `74.01s`.

Gate legitimacy:

- Typed publication gate: satisfied for the public CLI path.
- Typed network frame existence/handoff gate: satisfied.
- Typed routing production-consumer gate: **blocked**. The public CLI still
  routes through `compatibility_writeback_surface` and
  `execute_watershed_dispatch_with_kernel`.
- Final W4 committed-fixture conservation/output closure: **blocked** by typed
  routing consumer gate.
- Final workspace closure gates (`cargo nextest run --workspace --profile full`
  and `cargo deny check`) were not run because W4 cannot close complete.

## Independent Verification

Read-only `rust_code_reviewer` and `rust_qa_reviewer` completed.

`rust_code_reviewer` evidence:

- Static review plus `git diff --check`.
- Accepted primary finding: W4 must remain held because the public CLI still
  routes through `compatibility_writeback_surface` into
  `execute_watershed_dispatch_with_kernel`.
- Accepted follow-up findings: typed-builder guard parity is not yet proven;
  compatibility publication harvest defaults missing routed symbols to zero;
  the current source guard proves only the partial CLI handoff.

`rust_qa_reviewer` evidence:

- Static review plus `git diff --check`, `cargo fmt --check`, and focused W4
  source-marker test.
- Accepted primary finding: W4 is not a production typed-routing cutover.
- Accepted QA findings: source guard coverage is insufficient for complete W4,
  conservation/publication final acceptance evidence is missing, and package
  truthfulness must record an executed hold.

Current tool roles do not expose `science_contract_reviewer`; science-contract
disposition was handled locally against
`docs/specifications/science-contracts/AGENTS.md` and the pre-edit
operand-lineage authority map. No `SC-*` contract amendment was made because
the landed partial implementation is intended to preserve current compatibility
semantics and does not claim final typed routing/publication closure.
