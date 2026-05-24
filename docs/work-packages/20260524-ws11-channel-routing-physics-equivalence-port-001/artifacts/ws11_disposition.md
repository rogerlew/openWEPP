# WS11 Disposition

Status: `hold`
Evidence mode: `Static + Ran`
Disposition: `HOLD`

## Static
- Disposition code: `WS11_HOLD_CLOSEOUT`
- Package state: `hold`
- Scope outcome: implemented-with-open-blockers
- Holds are explicit and evidence-backed in:
  - `gate-results.md`
  - `worker-handoff.md`
  - `ws11-routing-vectors-and-parity-traces.md`

## Ran
- Exit criteria check
  - [x] WS11 queue objective is evidence-backed.
  - [x] Surrogate-only channel routing posture is replaced with explicit
    `ipeak` branch routing behavior in production code.
  - [x] Canonical WS11 contract authority amendments are implemented.
  - [x] Contract-derived WS11 tests are implemented and executed.
  - [x] Pre-implementation contract gate evidence exists.
  - [x] WS11 implementation/test evidence is recorded.
  - [ ] Dedicated WS11 worktree branch requirement satisfied
    (`main` execution context recorded in `worker-handoff.md`).
  - [x] Required gates executed:
    - `cargo fmt --check`
    - `cargo clippy --workspace --all-targets -- -D warnings`
    - `cargo test --workspace`
    - `cargo deny check`
  - [x] Required gates all passing.
  - [ ] Routing vectors and baseline parity traces are fully produced
    (baseline comparator trace lane is blocked; see
    `ws11-routing-vectors-and-parity-traces.md`).
- Hold-lift deltas
  - `cargo deny check` is green (`advisories ok, bans ok, licenses ok,
    sources ok`); prior WS11 dependency-policy blocker is closed.
  - `cargo test --workspace` is green after release-sidecar race fix in
    `openwepp-runner`.

- HOLD reasons
  1. WS11 worktree-governance requirement was not met in this execution context
     (`main` branch used instead of dedicated `ws11-*` worktree branch).
  2. Legacy baseline routed branch parity lane is blocked by baseline runtime
     failure: persisted logs show `ipeak=3` mode-3 watershed run aborts with
     `SIGFPE` at `/workdir/wepp-forest/src/wshchr.for:342`, so full branch
     parity closure cannot be completed in this package closeout.
