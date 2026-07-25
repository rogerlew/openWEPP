# TESTGATE Qualification Attempt 1

Evidence class: Ran.

- Provider run:
  [`30164861346`](https://github.com/rogerlew/openWEPP/actions/runs/30164861346)
- Head: `37eeee9a045ad15e3afe2c534ec132551dfbc81c`
- Dispatched base: `907222635e281a2e135b7f83bdf41eef9656a2d6`
- Result: `FAIL`
- Typed cause:
  `GATE-PACKAGE-CHAIN-ANCHOR-INACTIVE:
  docs/work-packages/20260724-testgate-quality-observatory-qualification-001/package.md`

The execution job passed checkout, trusted-main admission, pinned-toolchain
verification, dependency bootstrap, executor build, durable-history
restoration, and superseded-head rejection. The content-verifiable gate step
then rejected the package anchor before planning because the package was still
`QUEUED` at the dispatched base.

The verifier authenticated and published recovery evidence, then failed closed
because execution had not succeeded. The authority job likewise failed closed.
Artifacts `testgate-unsigned-30164861346-1` and
`testgate-recovery-verified-30164861346-1` are retained by the provider. The
unsigned orchestration error and pre-receipt failure carry the same typed
cause; the recovery artifact contains authenticated recovery predicate,
attestation, and verification records.

Disposition: repository-owned dispatch-binding defect. The first active
authority commit is
`f01ab161b3ba135a04cb82b1934141577e21b641`; a new committed and pushed head may
be dispatched against that corrected base. The failed head is not eligible for
an unchanged rerun.
