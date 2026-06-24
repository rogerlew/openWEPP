# Worker Handoff

Status: `HOLD-R7H-H2637-DIRECT-PERFORMANCE-AND-PROTECTED-PARITY`.

Current state:

- Direct default-candidate H2637 reaches endpoint after this package's
  no-material frost fixes.
- Direct manifest reports `compatibility_edge_invocations=0`,
  `scheduler_kernel_executed=false`, and
  `publication_source=direct-publication-frame`.
- Timing is red: `113.53 s` versus the `91.2 s` `<=10x` gate.
- Retained compatibility comparison is red for HBP/WAT/PASS/loss/plot; current
  compatibility was not rerun after the direct timing gate failed.
- Default activation remains disabled.

First follow-up objective:

Close `HOLD-R7H-H2637-DIRECT-PERFORMANCE-AND-PROTECTED-PARITY`.

Required first actions:

1. Profile current H2637 direct winter/frost execution after the no-material
   closure fixes.
2. Remediate active winter/frost hot-path cost without reintroducing
   compatibility/symbol surfaces or heuristic physics changes.
3. Rerun same-binary H2637 default compatibility, explicit rollback, direct
   default-candidate, and explicit direct.
4. Require direct endpoint `<=91.2 s`,
   `compatibility_edge_invocations=0`, and protected HBP/WAT/PASS/loss/plot/
   manifest parity before any R7H release-readiness or direct-default
   activation claim.

