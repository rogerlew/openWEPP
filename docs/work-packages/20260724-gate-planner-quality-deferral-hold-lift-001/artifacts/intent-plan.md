# Pre-Implementation Intent Plan

Evidence class: Static / Ran.

Base scaffold head:
`81599ed1`.

Risk: `CRITICAL`.

Reason: the package changes TESTGATE planning/reconciliation fixtures and one
terminal trust invariant, then authorizes a full-workspace correctness run and
the previously blocked global quality observation.

## Intended Diff

- replace two synthetic HEAVY definitions that incorrectly reuse prohibited
  `adjudicated-crap-v1` with an ordinary fixture-only HEAVY identity;
- order the source-mutation fixture deterministically so the intended
  out-of-manifest mutator runs first and the later independent node is blocked;
- make intent/terminal reconciliation reject any quality-disposition drift with
  a typed error;
- preserve the canonical schema prohibition on all retired quality nodes and
  every exact-checkout/source-mutation verifier guard;
- update package evidence and, only after current passing receipts, lift and
  complete the Order-3 package.

## Selected Gates

- the exact seven retained test identities under `--cfg coverage`;
- the complete `openwepp-gate-planner` crate under the coverage configuration;
- ordinary owning-crate Nextest and warnings-denied Clippy;
- Rustfmt, schemas/source guards, docs lint, diff/write-set reconciliation, and
  line-count governance;
- two independent read-only implementation/security reviews with zero
  undispositioned findings;
- delegated full-workspace correctness;
- delegated fresh Order-3 merged-coverage collection;
- two independent read-only terminal verifications of compact publication.

No coverage/CRAP debt verdict is a prerequisite-package closure gate. Order-3
execution and evidence integrity remain binding because this package explicitly
owns its hold lift.
