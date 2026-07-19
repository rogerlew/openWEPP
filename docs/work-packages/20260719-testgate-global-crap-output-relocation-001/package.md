# Correct Global CRAP Output Relocation

Package ID: `20260719-testgate-global-crap-output-relocation-001`

Queue ID: `TESTGATE-GLOBAL-CRAP-OUTPUT-01`

Status: `ACTIVE / ENV-PROJECTION-REVIEW-PASS / COMMITTED-PLAN-PENDING`

Authorization: verified critical receipt
`78f526eee1d0b8a9142afc9f3ff8f9434702d1a5409d917a1c2a22687aa7638c`
under Roger Lew's 2026-07-19 adversarial acceptance direction.

This is a Defect-Closure ExecPlan governed by
`docs/defect_closure_execplans.md` and `docs/codex_exec_plans.md`.

## Objective

Make the global CRAP runner's default output relocatable beneath the executor
artifact root: retain a safe relative default until relocation, then resolve it
against the repository only for standalone runs. Rebind both adapter digests.
Also close the integrated planner defect that makes an otherwise exact terminal
plan unreconstructable across invokers by hashing undeclared ambient variables.

## Progress

- [x] (2026-07-19) Scaffolded the package from the verified sole failure in
  critical receipt
  `78f526eee1d0b8a9142afc9f3ff8f9434702d1a5409d917a1c2a22687aa7638c`.
- [x] (2026-07-19) Completed the bounded authority, phase, security, HOLD,
  delegation, and active-handoff surfaces required for READY status.
- [x] (2026-07-19) Implemented the bounded default-relocation correction,
  exact contract assertions, and two derived adapter digest updates.
- [x] (2026-07-19) Shell syntax, Rust formatting, direct digest reconstruction,
  and 13 focused TESTGATE contract cases pass, including isolated behavioral
  probes for executor/standalone default and explicit output branches.
- [x] (2026-07-19) Dual independent review completed; both findings were
  accepted, the behavioral-evidence finding is patched and reverified, and the
  stale-plan finding becomes the committed-plan pre-execution gate.
- [x] (2026-07-19) Exact committed planning reproduced `TGGO-A-01`: independent
  reconstruction differs only in `environment_manifest_sha256`, plan ID, and
  execution key. The environment projection currently hashes every ambient
  variable, including invoker-controlled `_` outside all gate allowlists.
- [x] (2026-07-19) Prospectively amended this package before planner edits to
  own `TESTGATE-ENV-PROJECTION-DETERMINISM-01` as the integrated blocker.
- [x] (2026-07-19) Projected only the union of policy-declared gate environment
  allowlists. Focused unit evidence proves undeclared ambient noise is neutral,
  declared-value changes break identity, and all four current declared keys are
  discovered from validated policy.
- [x] (2026-07-19) Dual independent re-review accepts the environment projection
  with no finding; committed double-plan identity and reconciliation remain the
  next gate.
- [x] (2026-07-19) Corrected the live package status token to `ACTIVE` before
  final focused evidence so the canonical helper can admit the already
  prospective amended write set mechanically.
- [ ] Complete dual review, one mechanical terminal execution, dual terminal
  verification, prompt archival, and final disposition.

## Rationale And Dependencies

Critical receipt
`78f526eee1d0b8a9142afc9f3ff8f9434702d1a5409d917a1c2a22687aa7638c`
is the reproducer and predecessor evidence. Eleven selected nodes pass. The
sole failure is global CRAP exiting before coverage acquisition because the
script creates an absolute default output path before executor relocation,
while relocation correctly accepts only a safe relative output path.

No external service, GitHub workflow, forest1 runner, production deployment,
science contract, or Rust kernel change is a dependency.

## Included And Excluded Scope

Included: the runner's default-output initialization and standalone resolution,
the exact executor contract assertions for both branches, the two adapter
digests mechanically derived from the corrected script, and the planner's
environment-variable identity projection.

Excluded: coverage/CRAP algorithms, thresholds, exception registries, command
selection, prerequisites, policy risk, workflow or runner configuration,
kernel/science behavior, and any unrelated test cleanup.

## Correction Authority Envelope

Defect `TESTGATE-GLOBAL-CRAP-OUTPUT-01` is the valid executor invocation failing
with `executor artifact relocation requires an absolute root and safe relative
output path` before global CRAP acquisition. The in-scope source, contract test,
derived policy bindings, and documentation are exactly the declared write set.

Allowed production edit: retain the default `target/adjudicated-crap` as a safe
relative value through executor relocation; when no executor artifact root is
present, resolve that same relative default against the repository before use.
Explicit user-supplied output behavior, validated confinement, coverage state,
CRAP adjudication, fail-closed checks, and command semantics are protected.

Acceptance is observable when focused contract evidence proves both executor
and standalone branches, both adapter bindings equal the direct corrected
script SHA, and the repository planner's exact critical plan passes. Any cause
outside this envelope, changed policy semantics, or need to alter CRAP behavior
is a branch-out boundary requiring prospective package amendment.

Defect `TESTGATE-ENV-PROJECTION-DETERMINISM-01` is the exact committed terminal
plan failing independent reconstruction because
`environment_manifest_sha256` changes across invokers while every source,
policy, tool, fixture, configuration, node, and inventory field is identical.
The in-scope correction is limited to
`crates/openwepp-gate-planner/src/execution_context.rs`: replace the unbounded
ambient-variable projection with the union of `environment_allowlist` keys in
the validated gate registry. Compiler, platform, target, features, runner image,
Cargo configuration, and Git-local configuration identities remain protected.
Unknown policy-declared environment keys remain bound automatically; secrets or
shell bookkeeping outside the declared projection must not affect identity.

Acceptance for the integrated defect is observable when focused unit evidence
proves undeclared-variable changes are identity-neutral and declared-variable
changes are identity-breaking, then two separately invoked exact committed
plans are byte-identical and independent reconciliation passes.

## Declared Write Set

- `tools/release/run_adjudicated_crap_gate.sh`
- `gate-policy/v1/gate-definitions.json`
- `tests/integration/testgate_ci_executor_contract.rs`
- `crates/openwepp-gate-planner/src/execution_context.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260719-testgate-global-crap-output-relocation-001/**`

## Deliverables

- A minimal default-output relocation correction with unchanged explicit-path
  behavior.
- Exact focused regression evidence for executor and standalone modes.
- Reconciled adapter identities and one mechanically selected terminal receipt.
- Dual independent review, finding disposition, verification, and final status.

## Correction And Gates

Preserve every coverage, CRAP, confinement, output, and fail-closed semantic.
Change only default path resolution, its exact contract assertions, and the two
derived adapter SHA fields. Scaffold is committed before implementation.

Run shell syntax and focused TESTGATE contracts, prove executor and standalone
path branches, then execute the mechanically selected critical plan. No manual
broad suite, GitHub dispatch, or forest1 action is authorized.

## Phase Plan

1. Intake: reconstruct the receipt failure, direct script SHA, applicable
   instructions, exact diff baseline, and protected semantics.
2. Correct: change only default path resolution, update its exact contract
   assertions, and rebind the two mechanically derived adapter SHA fields.
   If exact committed reconstruction fails solely on the environment projection,
   project only policy-declared environment keys and add focused unit evidence.
3. Focused validation: run shell syntax, formatting when invalidated, the exact
   TESTGATE executor contract, direct digest reconstruction, and diff hygiene.
4. Review: obtain two independent read-only reviews, disposition every finding,
   and patch accepted findings inside the declared write set.
5. Terminal validation: execute the planner-selected critical plan once. Do not
   rerun passing nodes separately or manually add broad gates.
6. Closure: complete two independent terminal verifications, archive the active
   prompt, update the catalog, and record PASS/FAIL/BLOCKED/NOT RUN truthfully.

## Conversion Rule And HOLD Legitimacy

The reproduced mechanism, expected path behavior, correction surface, and
focused regression all lie inside the envelope, so this package must implement
and validate the correction. Effort, runtime, or the existence of additional
inspection is not a legitimate HOLD.

HOLD is legitimate only if the failure is proven to require an excluded change,
the protected executor confinement contract is contradictory, the selected
evidence is unavailable, or the current repository invalidates the receipt
reproducer. A HOLD must name that boundary, record the attempted in-envelope
route, and assign a concrete next defect owner.

## Acceptance

- [x] Executor global CRAP output is relative then confined; standalone default
  remains repository `target/adjudicated-crap`.
- [x] Both adapter identities match direct script SHA.
- [x] Focused contract evidence passes.
- [ ] Mechanical critical receipt passes every node.
- [ ] Dual review and dual terminal verification have no open finding.
- [x] Changed Rust files are 249 and 558 lines, below the 2,000-line warning
  threshold; no kernel/process Rust changes.
- [x] Undeclared ambient variables do not alter plan identity; every
  policy-declared gate environment key remains bound.
- [ ] Two separate exact committed plan invocations are byte-identical and
  independent reconciliation passes before execution.

## Review And Delegation

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent read-only reviewer/verifier roles for
confinement, exact diff, anti-evasion, receipt integrity, non-deferral, HOLD
legitimacy, security impact, and terminal disposition; expected outputs are
compact finding reports, dispositions, commands, counts, timings, and
PASS/HOLD/FAIL verdicts; write access is read-only.

Subagent requirement: two independent reviewer/verifier roles are required.
The parent owns all package writes and finding patches.

## Security Impact

Executor relocation is a path-confinement boundary and adapter SHA values are
fail-closed executable identities. Review must prove the correction neither
permits traversal/absolute child paths under executor mode nor weakens digest
admission, while preserving the standalone repository-local default.
The environment projection is also an execution-identity boundary: it must bind
every policy-permitted gate variable without hashing undeclared ambient values
or secret-bearing process state.

## Surprises And Discoveries

- The predecessor's global CRAP node failed before coverage acquisition. This
  keeps the correction mechanism narrow and avoids treating expensive work as
  evidence when it never ran.
- The same runner behaves correctly when the executor supplies an explicit safe
  relative output; only default initialization precedes relocation incorrectly.
- The local TESTGATE helper correctly rejected a pre-edit zero-work observation;
  the committed package remains the prospective authority, and the intent plus
  terminal plans are generated from the bounded implementation diff.
- Review finding `TGGO-A-02` correctly rejected source-substring checks as
  behavioral branch proof. The accepted patch executes seven isolated path
  cases and stops before coverage acquisition because the scratch repository
  intentionally lacks the Python prerequisite.
- Review finding `TGGO-A-01` correctly rejected the first non-reconciling dirty
  terminal plan. It is retained as non-executable review evidence; the final
  committed plan must independently reconcile before execution.
- Repeating the plan on the exact committed head reproduced `TGGO-A-01`. A
  canonical JSON diff showed only `environment_manifest_sha256`, `plan_id`, and
  `execution_key` changed. `environment_record` hashes every process variable,
  although policy permits only `PATH`, `CARGO_HOME`, `RUSTUP_HOME`, and
  `RUSTUP_TOOLCHAIN`; invoker bookkeeping such as `_` therefore poisons
  deterministic reconstruction without representing a permitted gate input.
- The direct correction derives its key union from the validated gate registry
  instead of hard-coding the four current names. A future declared key therefore
  enters identity automatically, while undeclared secrets and shell bookkeeping
  remain outside the permitted projection.

## Decision Log

- Decision: correct default resolution in the existing runner rather than
  weaken executor relocation validation.
  Rationale: relocation is the security boundary; the script default is the
  in-envelope conflicting input.
  Date/author: 2026-07-19, parent agent.
- Decision: prospectively widen the same tooling package to the integrated
  environment-projection defect rather than execute a known-invalid plan or
  create another diagnostic relay.
  Rationale: the user's package objective is integrated tooling closure, the
  mechanism is reproduced and in repository-owned planner code, and the direct
  fail-closed correction is bounded and testable.
  Date/author: 2026-07-19, parent agent.

## Outcomes And Retrospective

Output relocation implementation, focused evidence, and its dual review pass.
Both dirty and committed terminal plans are explicitly rejected and must not
execute because independent reconstruction exposes the integrated environment-
projection defect. The prospectively amended package owns its direct correction;
terminal outcome and final retrospective remain pending.
