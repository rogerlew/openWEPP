# Baseline/current comparison

Status: `BASELINE COMPLETE — CURRENT TERMINAL RERUN PENDING`

Evidence mode: `Static + retained Ran`

The delegated comparator compared exact fixed-point package baseline
`792af753e7c936a66352ee69ef5c5c1a18447082` with package-start current source
`e2de23695e7b9219a039f13a3c2fb935513f5a20` under identical full-profile
configuration and external scratch topology.

Ran baseline command:

```text
env RUST_MIN_STACK=67108864 nix develop -c \
  cargo nextest run --workspace --profile full
```

Baseline elapsed `01:20:21` (`4821 s`), exit `100`: 3,625 attempted, 3,480
pass, 116 fail, 29 timeout, 0 skip. Retained log
`/tmp/workspace_gate_hold_lift/baseline/nextest_full.log`, SHA-256
`5f1f4f6205ad6afc87f6b78d055f6fc29380e5d460f11487a838b4b0b50dd0ab`;
invocation snapshot SHA-256
`d5f58ce9b8710910971f9d3bfa86d369476028adc9cb6b8a9930b9735c33d9fa`.

Current retained source (`e2de23695`) attempted 3,628 tests: 3,503 pass, 96
fail, and 29 timeout. The exact 125 adverse outcomes partition as:

- 82 generated Assurance V2 identity-chain failures, corrected through one
  typed source-adoption transaction;
- 3 stale accepted-endpoint source scans, already corrected at package start;
- 1 relative run-directory test-helper defect;
- 3 committed Stage-3 publication/routing defects;
- 7 solver, transition, fixture-authoring, or topology failures;
- 29 timeouts, whose semantic versus resource attribution must be reevaluated
  after the preceding causal corrections.

Direct execution of the four LSE failures against binaries built from
`792af753e` reproduces their current payloads, so those are baseline-historical
but not waived: two are the covered no-update-witness defect and two expose a
missing frozen-litter phase/enthalpy authority path.

The exact adverse-name comparison is 122 shared, 23 baseline-only, and 3
current-only. All three current-only outcomes are
`precomputed_terminal_accepted_executor_tests` in
`openwepp-hillslope-orchestrator`; they are the narrow terminal regression
surface for the replacement current profile. The 23 baseline-only outcomes are
all ordinary failures, dominated by assurance/quality/frozen-vector checks
already corrected or regenerated in later source.

Raw audit sets:

- `/tmp/workspace_gate_hold_lift/baseline/shared.outcomes`;
- `/tmp/workspace_gate_hold_lift/baseline/base_only.outcomes`;
- `/tmp/workspace_gate_hold_lift/baseline/current_only.outcomes`.

The isolated baseline is the authoritative full census/timing evidence; it is
not a waiver for any shared or baseline-historical correctness failure.
