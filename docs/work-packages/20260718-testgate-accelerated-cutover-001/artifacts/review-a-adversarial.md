# Review A: Adversarial Security And Authority

Evidence class: static review plus live provider/host inspection.

The adversarial review repeatedly returned HOLD until accepted findings were
patched. Its scope covered public-repository event admission, ref ancestry,
token scope, runner persistence, cache/workspace contamination, receipt
fabrication, output escape, rollback, and redundant-test pressure.

## Accepted Findings And Resolution

- Local unsigned receipts could not authorize normal gates. Execution now has
  contents-read permission only; an independent hosted verifier reconstructs
  the envelope, and a minimal hosted aggregate owns OIDC attestation.
- Intent could be authorized post hoc and rollback was co-located with the
  candidate runner. Prospective package lineage is required, and conservative
  rollback is an independent GitHub-hosted manual workflow.
- The runner control plane and executable caches were job-writable/persistent.
  Registration state and root are read-only; bounded tmpfs surfaces are purged
  after completion; self-update is disabled.
- A forest1 Docker-container Buildx probe was privileged and used a mutable
  builder image. The builder, volume, image, and layers were removed. Image
  construction moved to the controller's non-privileged default driver with an
  8-CPU/24-GiB hard envelope and exact image rebind before transfer.
- The final review found that static routing could not close the forest1
  consumer path, the rollback lane had zero provider runs, and its reuse
  predicate rejected valid fully adjudicated global CRAP. These findings are
  accepted: cutover remains disabled pending a docs-only normal consumer run,
  and the rollback workflow receives a non-heavy hosted smoke plus corrected
  canonical predicate.

No finding was waived. Provider activation remains contingent on executable
consumer evidence and final terminal re-review.
