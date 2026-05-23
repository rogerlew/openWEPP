# PL13 Kickoff Agent Prompt

You are executing `20260523-pl13-growth-transition-kernel-001` for the
monolithic openWEPP scientific hydrology/erosion model.

Domain context (explicit):
- This package is the production growth-transition implementation lane for
  `PL09-GAP-006` (growth path), not an alias continuity package.
- `PL11` owns transition-control/runtime-projection payload semantics;
  `PL13` consumes those projected payloads in production growth transition
  execution.
- `PL13A` runs in parallel and owns alias continuity authority closure.

Objectives:
1. Implement production annual/perennial growth transition execution with typed
   transition signaling and invariant guards.
2. Preserve ARCH15/ARCH21 typed-seam non-regression posture.
3. Keep contract authority as source of truth for growth algorithm intent.
4. Respect parallel ownership boundary with `PL13A`.
5. Enforce state update/invariant behavior for `sumgdd`, `vdmt`, `cancov`,
   `lai`, `rtmass`, `rtd`, and `hia` surfaces in transition paths.
6. Author/update contract-derived conformance tests and execute documented
   pre-implementation contract gates before production growth-kernel code edits.

Mandatory sequencing constraints:
- Do not modify production growth-kernel code until:
  1. contract authority updates are drafted, and
  2. pre-implementation contract-gate evidence is recorded.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Do not introduce silent defaults/clamping for invalid state domains.
- Use typed guard/failure behavior for impossible transition/update states.
- Record evidence mode explicitly (`Static:` vs `Ran:`).

Parallel ownership boundary (strict):
- PL13-owned runtime surfaces:
  - `crates/openwepp-hillslope-orchestrator/src/lib.rs`
  - `crates/openwepp-kernel-contract/src/lib.rs`
  - `tests/integration/**` (growth transition lane)
- PL13A-owned alias continuity authority surfaces:
  - `docs/specifications/science-contracts/symbol-alias-registry.md`
  - `docs/specifications/science-contracts/contracts/SC-PLANT-001.md` alias-map
    sections
  - `crates/openwepp-sim-contract/src/symbols.rs`
- Do not modify PL13A-owned files from this package.

Required outputs:
- `artifacts/pl13-process-contract-authority.md`
- `artifacts/pl13-growth-kernel-algorithm.md`
- `artifacts/pl13-sc-contract-amendment-plan.md`
- `artifacts/pl13-preimplementation-contract-gate.md`
- `artifacts/pl13-implementation-and-test-evidence.md`
- `artifacts/pl13-typed-seam-non-regression-evidence.md`
- `artifacts/pl13-kernel-profile-compliance-checklist.md`
- `artifacts/pl13-parallel-ownership-boundary.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/pl13_disposition.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`

Required gates:
- Pre-implementation contract gate evidence must exist before production kernel
  edits are considered valid for disposition.
- Typed-seam non-regression evidence must explicitly cover ARCH15/ARCH21 posture.
- Kernel profile compliance checklist must trace growth transition assertions to
  canonical science-contract authority.
- If code is changed, run:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
