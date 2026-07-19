# Align CRAP Adapter Digests

Package ID: `20260719-testgate-crap-adapter-digest-alignment-001`

Queue ID: `TESTGATE-CRAP-ADAPTER-DIGEST-01`

Status: `EXECUTED / FAIL-GLOBAL-OUTPUT-RELOCATION / REVIEWED`

Authorization: fail-closed `GATE-ADAPTER-DIGEST` from the confined coverage
temp correction, under Roger Lew's 2026-07-19 adversarial acceptance direction.

This Defect-Closure ExecPlan is governed by `docs/defect_closure_execplans.md`.

## Objective

Replace only the two stale `run_adjudicated_crap_gate.sh` adapter digest
bindings with the SHA-256 of the corrected current script, then execute the
mechanically selected critical plan.

## Progress

- [x] (2026-07-19) Reproduced: current script is
  `b75a06fae6899a05aabb77805933b4466b072a71a58e815430eefcffa0db1a85`;
  both bindings retain
  `212282e3ddaa17dd8f22373598987dfc8f1efc1559d84a4880c0d65f683f55fa`.
- [x] (2026-07-19) Aligned both exact fields and changed no other JSON field.
- [x] (2026-07-19) Direct SHA reconstruction matches both bindings; focused
  policy contracts pass 10/10.
- [x] (2026-07-19) Executed the 12-node critical plan: 11 nodes pass, including
  full Nextest 2,165/2,165; global CRAP fails before acquisition because its
  default absolute output conflicts with executor-safe relative relocation.
- [x] (2026-07-19) `20260719-testgate-global-crap-output-relocation-001` owns
  that distinct gate-runner defect. No passing node is rerun in this package.
- [x] (2026-07-19) Dual review and terminal verification accept the truthful
  failed disposition and the sole READY successor; no executable gate was
  repeated during documentation reconciliation.

## Correction Authority Envelope

Allowed correction: update only `executor.adapter_sha256` for
`affected-adjudicated-crap-v1` and `adjudicated-crap-v1` to the direct current
script digest. No command, risk, prerequisite, scope, or other field may change.

## Declared Write Set

- `gate-policy/v1/gate-definitions.json`
- `docs/work-packages/README.md`
- `docs/work-packages/20260719-testgate-policy-digest-alignment-001/**`
- `docs/work-packages/20260719-testgate-adversarial-clippy-cleanup-001/**`
- `docs/work-packages/20260719-testgate-assurance-socket-path-portability-001/**`
- `docs/work-packages/20260719-testgate-coverage-tmpdir-confinement-001/**`
- `docs/work-packages/20260719-testgate-crap-adapter-digest-alignment-001/**`

## Gate Plan

Direct SHA/JSON reconstruction, focused TESTGATE authority contracts, Markdown
and diff hygiene, then one exact local planner-selected terminal plan. This is
a critical policy binding; no manual narrowing, GitHub dispatch, or forest1
action is authorized.

## Acceptance

- [x] Both adapter digests equal direct script SHA; all other JSON is unchanged.
- [x] Focused policy contracts pass.
- [ ] Mechanical critical receipt passes every selected node.
- [x] Dual review and dual terminal verification have no open finding about the
  failed disposition or successor readiness.
- [x] No Rust change; line-count governance is `NOT_APPLICABLE`.

## Review And Delegation

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only reviewer/verifier roles for
exact binding, anti-evasion, receipt integrity, gate economy, non-deferral,
HOLD legitimacy, and terminal disposition; expected outputs are compact finding
reports, evidence reconciliation, and PASS/HOLD/FAIL verdicts; write access is
read-only.

Subagent requirement: two independent reviewer/verifier roles are required.

## Security Impact

This restores cryptographic identity for the corrected confined adapter without
weakening any behavior or gate.

## Surprises And Discoveries

- The fail-closed admission error was fully explained by the two expected
  derived bindings; no third stale adapter reference exists.
- Final receipt
  `78f526eee1d0b8a9142afc9f3ff8f9434702d1a5409d917a1c2a22687aa7638c`
  is `LOCAL_UNTRUSTED` / `FAIL`: 11 passed, 1 failed, 0 blocked. Global CRAP
  exits 2 with `executor artifact relocation requires an absolute root and
  safe relative output path`; it never begins coverage acquisition.

## Outcomes And Retrospective

The binding correction passes focused policy evidence and 11/12 critical
nodes. The package closes as a reviewed, truthful failed execution and assigns
the distinct global-output relocation defect to the named READY successor. The
failed global CRAP gate remains failed; no passing evidence was relabeled and no
passing executable node was rerun for documentation closure.
