# Kernel-Profile Compliance Checklist

Status: completed

Evidence mode: mixed

- Ran: contract-first sequencing was followed: canonical contracts, red test,
  pre-implementation gate, then production diagnostics.
- Static: canonical `SC-*` authority is updated in `SC-SUBHYD-001` and
  `SC-WATBAL-001`.
- Static: baseline provenance is pinned to
  `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Static: no heuristic/proxy process-physics math was introduced.
- Static: publication diagnostics are additive observability surfaces; realized
  `q`/`Qd` equations remain baseline-authoritative.
- Ran: full Rust gates and authority guards passed.
