# Lane D Conditional Default Activation

Status: `EXECUTED-COMPLETE-CONDITIONAL-DEFAULT-ACTIVATION`
Evidence mode: Mixed.
Date: 2026-07-08

## Objective

Make Lane D active routing the default only when the run's scheduled lanes are
coefficient-complete. Preserve legacy/default behavior for runs with no
extended routing coefficients, and fail closed for mixed coefficient authority.

Binding policy:

- all scheduled lanes have native `routing_coefficients`: active Lane D runs by
  default;
- no scheduled lanes have native `routing_coefficients`: legacy/default path
  runs;
- a mix of coefficient-present and coefficient-absent lanes fails closed before
  streaming;
- explicit `OPENWEPP_LANED_ACTIVE=1` still requires complete coefficients;
- an explicit rollback selector may force legacy/off even when coefficients
  are complete.

## Scope

In scope:

- Scaffold package-local artifacts and prompt.
- Amend `SC-OFEROUTE-001` contract-first for conditional default activation.
- Implement the runner eligibility resolver.
- Preserve explicit active opt-in and shadow mutual-exclusion behavior.
- Add an explicit active-disable rollback selector.
- Add tests for all-extended default activation, all-legacy fallback, mixed
  fail-closed behavior, explicit active missing-coefficient fail-closed
  behavior, and explicit disable behavior.
- Prove protected scientific outputs are unchanged for no-coefficient fallback.
- Record active default closure/DC01/routed-consumer evidence on a
  coefficient-complete fixture.
- Record review, verification, gates, disposition, and handoff.

Out of scope:

- No mesh-policy change; rev 45 dx5 remains the active production default.
- No routed-shape or annual sediment tolerance change.
- No sediment process-physics change.
- No shadow mesh change.
- No watershed channel routing or watershed supervisor default change beyond
  consuming the hillslope default through the normal hillslope runner path.
- No revival of abandoned hybrid stepping.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260708-laned-router-dx5-production-mesh-policy-ratification-001/artifacts/final-disposition.md`
- `docs/work-packages/20260708-laned-router-dx5-production-mesh-policy-ratification-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-disturbed-route-coeff-source-acquisition-001/artifacts/openwepp-authority-lift.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-disturbed-route-coeff-source-acquisition-001/artifacts/active-consumer-proof.md`

Conditional:

- `crates/AGENTS.md` and `tests/AGENTS.md` before Rust/test edits.
- `SC-SED-001` only if this package changes erosion water-magnitude coupling;
  it should not.

## Phase Plan

### Phase A - Scaffold And Authority Map

- Create package-local `package.md`, `artifacts/`, `prompts/active/`,
  `prompts/archived/`, `.gitignore`, and catalog/roadmap pointers.
- Record required-reading and the operator's all/none/mixed eligibility rule.

### Phase B - Contract Amendment

- Amend `SC-OFEROUTE-001` to rev 46.
- Add conditional default activation authority:
  - all lanes coefficient-complete -> default active;
  - no lanes coefficient-complete -> legacy/off fallback;
  - mixed -> hard fail;
  - explicit active keeps complete-coefficient precondition;
  - explicit disable forces legacy/off.
- Update guard map, invariant text, BEI/test-vector obligations, and revision
  history.

### Phase C - Implementation And Tests

- Implement a runner eligibility resolver before `frame.laned_active` is
  attached.
- Keep `laned_active_config()` fail-closed for explicit active or default
  active execution.
- Add/adjust tests in the existing Lane D integration fixture.
- Keep protected HBP/pass/WAT/loss output behavior unchanged for all-legacy
  fallback.

### Phase D - Runtime Proof

- Build the exact release runner.
- Run all-extended default/no-env and explicit-active controls and prove both
  publish active evidence.
- Run all-legacy default/no-env and prove protected outputs match pre-flip
  fallback behavior.
- Run mixed authority and prove fail-closed before streaming.
- Run explicit disable on all-extended and prove active is absent.

### Phase E - Gates, Review, Disposition

- Run required gates.
- Complete review and verification.
- Disposition findings.
- Record final disposition and worker handoff.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to review, verification, and comparator/closure-gate
subagents for package-local review, verification, default/fallback comparator
work, and heavy Rust closure gates. Expected outputs are package-local
`artifacts/review-*.md`, `artifacts/verification-*.md`, and compact gate
metrics/log paths. Write access is bounded to this package's artifact
directory unless a subagent is explicitly assigned implementation fixes.

Subagent requirement: REQUIRED for heavy comparator/full closure gates when
available. Spawn `comparator_suite_runner` for selected default/fallback
comparator work or full closure/comparator runs; do not run those heavy gates
on the parent model unless the subagent is unavailable, in which case record
command-level evidence.

## Required Artifacts

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/contract-disposition.md`
- `artifacts/implementation.md`
- `artifacts/default-activation-evidence.md`
- `artifacts/default-activation-evidence.json`
- `artifacts/consumer-path-proof.md`
- `artifacts/gate-results.md`
- `artifacts/review-*.md`
- `artifacts/verification-*.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

Required before completion:

- Contract-first amendment to `SC-OFEROUTE-001`.
- Unit/contract-derived tests for the selector resolver.
- Focused Lane D integration tests.
- All-extended default/no-env active runtime proof.
- All-legacy default/no-env protected-output fallback proof.
- Mixed coefficient authority fail-closed proof.
- Explicit active missing-coefficient fail-closed proof.
- Explicit disable/rollback proof.
- Active closure and `INV-OFEROUTE-012` evidence on default-active run.
- DC01-disable / no-double-feed proof.
- Routed-hydrograph-to-erosion consumer proof.
- `git diff --check`.
- Markdown/doc lint for touched docs.
- Contract/profile/BEI checks required by touched contracts.
- `.rs` line-count disposition.
- `cargo fmt --check`.
- `cargo clippy --workspace --all-targets -- -D warnings`.
- `cargo nextest run --workspace --profile full`.
- `cargo deny check`.

Conditional:

- Authority anti-evasion guard only if required-case bindings, cohort fixture
  posture, or external-authority suite posture are touched.

## Exit Criteria

`EXECUTED-COMPLETE-CONDITIONAL-DEFAULT-ACTIVATION`:

- `SC-OFEROUTE-001` rev 46 authorizes conditional default activation.
- Default/no-env runs with complete coefficients activate Lane D.
- Default/no-env runs with no coefficients remain on the legacy/off path.
- Mixed coefficient authority fails closed before streaming.
- Explicit active remains complete-coefficient fail-closed.
- Explicit disable forces legacy/off on complete-coefficient runs.
- Protected fallback outputs are byte-identical.
- Active default closure, DC01/no-double-feed, and routed erosion consumer
  proof are recorded.
- Required gates, review, verification, and finding disposition are complete.

`EXECUTED-HOLD-*`:

- Any required activation/fallback/mixed proof gate fails.
- Hold audit names exact blocker, evidence, considered in-envelope correction
  route, and first actionable follow-on.
