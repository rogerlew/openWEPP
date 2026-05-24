# WS11 Gate Results

Status: `hold`
Evidence mode: `Static + Ran`

## Static
- Pre-implementation contract gate exists and is recorded in:
  - `artifacts/ws11-preimplementation-contract-gate.md`
- Post-implementation gate objective:
  - prove WS11 vector closure and repository gate status after production edits.
- Dependency-policy note:
  - prior WS11 `cargo deny` blockers are resolved in the current dependency graph.

## Ran
- Pre-implementation contract gate
  - `cargo test --test ws11_channel_routing_physics_equivalence_contract`
  - result: **fail** (`2 passed; 4 failed`) before WS11 production edits.
- WS11 post-implementation targeted gates
  - `cargo test --test ws11_channel_routing_physics_equivalence_contract`
    - pass (`6 passed`).
  - `cargo test --test ws10_watershed_kernel_contract`
    - pass (`4 passed`).
  - `cargo test --test arch22_typed_state_surface_contract`
    - pass (`6 passed`).
- Required repository closeout gates (re-run 2026-05-24)
  - `cargo fmt --check`
    - pass.
  - `cargo clippy --workspace --all-targets -- -D warnings`
    - pass.
  - `cargo test --workspace`
    - pass.
  - `cargo deny check`
    - pass (`advisories ok, bans ok, licenses ok, sources ok`).
    - non-blocking warnings observed: `license-not-encountered` for unmatched
      allowlist entries in `deny.toml`.
- Baseline parity-trace lane evidence (persisted run logs)
  - `ws11_mode3_ipeak1.stdout.log`
    - pass (`WEPP COMPLETED WATERSHED SIMULATION SUCCESSFULLY`).
  - `ws11_mode3_ipeak2.stdout.log`
    - pass (`WEPP COMPLETED WATERSHED SIMULATION SUCCESSFULLY`).
  - `ws11_mode3_ipeak3.stderr.log`
    - fail (`SIGFPE`), backtrace anchored at
      `/workdir/wepp-forest/src/wshchr.for:342` in `wshchr_`.

## Hold Summary
- Repository validation gates are green, but WS11 remains `HOLD` because:
  1. dedicated WS11 branch/worktree governance requirement was not satisfied;
  2. full baseline parity trace closure is blocked by baseline `ipeak=3`
     runtime failure (`SIGFPE`).
