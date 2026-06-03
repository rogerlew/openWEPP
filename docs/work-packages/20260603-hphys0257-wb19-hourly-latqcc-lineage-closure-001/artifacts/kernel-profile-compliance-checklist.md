# Kernel-Profile Compliance Checklist

Status: completed

Evidence mode: mixed

- Static: contract-first sequencing was followed: canonical contracts, then
  contract-derived tests, then red-gate evidence, then production edits.
- Static: canonical `SC-*` authority was amended in `SC-SUBHYD-001` and
  `SC-WATBAL-001`.
- Static: pinned baseline provenance is cited for `input.for`, `tilage.for`,
  and `watbal_hourly.for`.
- Static: production implementation uses typed missing-input/range guards and
  does not add silent defaults, clamps, or heuristic lateral-flow scaling.
- Ran: authority anti-evasion guards passed.
- Ran: Rust format, clippy, workspace tests, and dependency/license gates
  passed.
- Static: overall semantic parity remains `HOLD` because the full H1..H39
  comparator remains `0/39`.
