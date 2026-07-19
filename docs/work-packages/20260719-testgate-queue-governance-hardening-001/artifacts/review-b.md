# Review B: Schema And Authority Ordering

Evidence class: static, focused execution, provider, and image/runtime
inspection.

Verdict: implementation PASS; provider closure pending.

The reviewer independently confirmed GitHub's current `queue: single` schema,
the permanent concurrency name, mutually exclusive forest1/release labels,
exact-head checks, and appropriate focused-only validation. The derived drain
image has the pinned image's exact 17-layer prefix plus only two immutable hook
layers; its hook probe exits 1 and its runtime matches the declared confinement.

The initial authority-race finding was accepted and patched. The aggregate now
checks current main before attestation and again after native verification and
authenticated evidence upload, as the final workflow step. Contract ordering
assertions prevent either guard from drifting behind the work it protects.

No implementation finding remains. Final closure depends only on terminal
provider cleanup and removal of the bounded drain resources.
