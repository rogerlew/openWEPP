# TESTGATE Runner GitHub CLI Recovery

Package: `20260723-testgate-runner-gh-cli-recovery-001`
Status: `COMPLETE`
Defect: `RTR-046`
Cause: `GATE-TRUSTED-RUNNER-GH-CLI-MISSING`

## Objective

Install a checksum-pinned GitHub CLI in the trusted runner image and require
its exact version during preflight so durable history restore and native
attestation cannot fail with exit 127 after the image is admitted.

## Correction Authority Envelope

- Observed violation: push run `29979508839` passed trusted comparison,
  toolchain, bootstrap, and planner build, then failed before gate execution in
  durable history restore. Live runner inspection found no `gh` executable.
- In scope: pinned runner-image dependency, exact preflight version check,
  source-contract regression, runner documentation, image identity bindings,
  build/install/setup, and recovery evidence.
- Acceptance: the reviewed image contains exact GitHub CLI 2.96.0 from its
  official checksum-bound archive; preflight rejects drift; the rebuilt image
  is dual reviewed, installed, and proves `gh api` availability without
  launching TESTGATE.
- Protected boundaries: no threshold or test-selection change, no manual
  dispatch, no unchanged expensive rerun, and no unrelated runner expansion.

## Declared Write Set

- `tools/ci/omarchy-runner/Dockerfile`
- `tools/ci/omarchy-runner/manage.sh`
- `tools/ci/omarchy-runner/README.md`
- `.github/workflows/testgate-shadow.yml`
- `tests/integration/testgate_ci_executor_contract.rs`
- `docs/work-packages/20260718-testgate-accelerated-cutover-001/artifacts/host-capacity-security.md`
- `docs/work-packages/20260720-testgate-recovery-trust-001/**`
- `docs/work-packages/20260723-cqr-nightly-b03s-aggregate-001/**`
- `docs/work-packages/20260723-cqr-nightly-b03-execplan.md`
- `docs/work-packages/20260723-testgate-runner-gh-cli-recovery-001/**`
- `docs/work-packages/README.md`

## Required Reading

- `AGENTS.md`
- `tests/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/testing-and-gate-strategy.md`
- `tools/ci/omarchy-runner/README.md`
- `docs/work-packages/20260720-testgate-recovery-trust-001/package.md`

## Subagent Authorization

Subagent authorization: this package explicitly authorizes two independent
read-only implementation reviewers and two read-only terminal verifiers.
Expected outputs are package-local artifacts and live read-only runner
evidence. No reviewer may deploy, push, or dispatch TESTGATE.

## Progress

- [x] Retained the two zero-node trusted-run failures and reopened RTR-046.
- [x] Scaffolded prospective correction authority before image edits.
- [x] Installed and preflighted the exact checksum-pinned CLI in the candidate image.
- [x] Ran focused source-contract and image-build validation.
- [x] Obtained dual implementation review and activated the exact bound image.
- [x] Closed RTR-046 and obtained dual terminal verification.

## Exit Criteria

- Official release archive digest is pinned and verified before extraction.
- `gh version 2.96.0` is mandatory in trusted preflight.
- Focused integration, shell syntax, formatting, docs, and diff checks pass.
- Exact rebuilt image identity is bound everywhere and dual reviewed.
- Live runner proves the exact CLI and persistent history contract before the
  next changed-head push.
