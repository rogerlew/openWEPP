# Correct Global CRAP Output Relocation

Package ID: `20260719-testgate-global-crap-output-relocation-001`

Queue ID: `TESTGATE-GLOBAL-CRAP-OUTPUT-01`

Status: `READY`

Authorization: verified critical receipt
`78f526eee1d0b8a9142afc9f3ff8f9434702d1a5409d917a1c2a22687aa7638c`
under Roger Lew's 2026-07-19 adversarial acceptance direction.

This is a Defect-Closure ExecPlan governed by
`docs/defect_closure_execplans.md`.

## Objective

Make the global CRAP runner's default output relocatable beneath the executor
artifact root: retain a safe relative default until relocation, then resolve it
against the repository only for standalone runs. Rebind both adapter digests.

## Declared Write Set

- `tools/release/run_adjudicated_crap_gate.sh`
- `gate-policy/v1/gate-definitions.json`
- `tests/integration/testgate_ci_executor_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260719-testgate-global-crap-output-relocation-001/**`

## Correction And Gates

Preserve every coverage, CRAP, confinement, output, and fail-closed semantic.
Change only default path resolution, its exact contract assertions, and the two
derived adapter SHA fields. Scaffold is committed before implementation.

Run shell syntax and focused TESTGATE contracts, prove executor and standalone
path branches, then execute the mechanically selected critical plan. No manual
broad suite, GitHub dispatch, or forest1 action is authorized.

## Acceptance

- [ ] Executor global CRAP output is relative then confined; standalone default
  remains repository `target/adjudicated-crap`.
- [ ] Both adapter identities match direct script SHA.
- [ ] Focused contract evidence passes.
- [ ] Mechanical critical receipt passes every node.
- [ ] Dual review and dual terminal verification have no open finding.

## Review And Delegation

Two independent read-only reviewer/verifier roles are required for confinement,
exact diff, anti-evasion, receipt integrity, non-deferral, HOLD legitimacy, and
terminal disposition.

## Outcomes

Pending execution.
