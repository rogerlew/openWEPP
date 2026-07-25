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

Independent read-only inspection confirmed that the authenticated copies are
byte-identical to the unsigned originals. The pre-receipt failure SHA-256 is
`95f46413f506eaf96114afde4339f5a81dbc0f4b9a5829821c4b44e9246a7b02`; the
orchestration-error SHA-256 is
`690f9b497013d75a76642d0a1027923d4378ee91a8b2b155615cf1c9c29cdb87`.
The provider inventory contains 382 indexed entries and binds the repository,
workflow, run, attempt, ref, and exact head.

Disposition: repository-owned dispatch-binding defect. The intended activation
commit used the unrecognized composite status
`ACTIVE / QUALIFICATION / ORDER-6`; the package must first record the exact
planner-authoritative status `ACTIVE`. A later committed and pushed head may be
dispatched against corrected activation commit
`086244c889c20de823fd1fa5b02d3527ecffa236`. The failed head is not eligible
for an unchanged rerun.
