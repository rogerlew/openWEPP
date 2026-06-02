# Kernel-Profile Compliance Checklist

Status: complete

Evidence mode: static + ran

Static:

- Contract-first sequence followed:
  1. Canonical `SC-EVAP-001` and `SC-WATBAL-001` amendments landed before
     production edits.
  2. Contract-derived WB17 tests were added before production edits.
  3. Pre-implementation contract gate was recorded.
  4. Production code was modified after the contract gate.
- Physics authority is pinned to
  `/workdir/wepp-forest_260430_baseline` at
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No heuristic/proxy process-physics substitution was added. The implemented
  equations trace to baseline `evap.for`, `swu.for`, and `watbal_hourly.for`.
- WB17 root uptake is now a distinct post-WB19 scheduler phase, so final
  aggregate storage is recomputed after drainage/lateral mutation.
- Typed guard posture is preserved with `HKERNEL-WB11-ET-E-*` and
  `HKERNEL-WB17-SWU-E-*` failures.

Ran:

- `cargo test --test wb17_et_physics_kernel_contract -- --nocapture` passed
  `9/9`.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- Authority anti-evasion gates passed.

Compliance disposition:

- Implementation profile obligations for the targeted WB17 layer-storage scope
  are satisfied.
- Package disposition remains `HOLD` because full hillslope semantic closure is
  not achieved.
