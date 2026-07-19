# Align CRAP Adapter Digests

Package ID: `20260719-testgate-crap-adapter-digest-alignment-001`

Queue ID: `TESTGATE-CRAP-ADAPTER-DIGEST-01`

Status: `READY`

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
- [ ] Align both exact fields and prove semantic equality elsewhere.
- [ ] Run focused policy contracts, then the mechanical critical plan.
- [ ] Complete dual review and terminal verification.

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

- [ ] Both adapter digests equal direct script SHA; all other JSON is unchanged.
- [ ] Focused policy contracts pass.
- [ ] Mechanical critical receipt passes every selected node.
- [ ] Dual review and dual terminal verification have no open finding.
- [ ] No Rust change; line-count governance is `NOT_APPLICABLE`.

## Review And Delegation

Subagent authorization: two independent read-only reviewer/verifier roles are
required for exact binding, anti-evasion, receipt integrity, gate economy,
non-deferral, HOLD legitimacy, and terminal disposition.

## Security Impact

This restores cryptographic identity for the corrected confined adapter without
weakening any behavior or gate.

## Surprises And Discoveries

Pending execution.

## Outcomes And Retrospective

Pending execution.
