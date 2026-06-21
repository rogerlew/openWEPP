# R6E Kickoff Agent Prompt

Scope: local repository R6 direct-publication cutover task; package-end-to-end.

In `/home/workdir/openWEPP`, execute
`docs/work-packages/20260621-r6e-direct-publication-cutover-iterative-defect-closure-001/`
end to end.

Close defect `R6E-DIRECT-PUBLICATION-CUTOVER-BLOCKER`. Current executed state
is `HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`; the prior
`HOLD-R6E-PRODUCTION-DIRECT-RUNTIME-INPUT-BINDING-ABSENT` blocker was resolved
by typed direct publication day inputs and retained direct executor capture.

Required terminal target:

- `COMPLETE-R6-DIRECT-PUBLICATION-CUTOVER`, with HBP byte identity, WAT/PASS
  Arrow row/schema/metadata parity, loss JSON identity, manifest
  provenance/checksum parity, anti-alias fixtures, independent reconstruction,
  no-compatibility proof, default-disabled isolation, successful direct output
  writes, full closure gates, and line-count governance.

Execution order:

1. Read required package, root, work-package, DC, architecture, and R6D
   handoff/gate artifacts.
2. Reproduce the current cutover failure and record the evidence.
3. Build the blocker ledger.
4. Iterate: identify blocker, prove authority, implement in-envelope
   correction, add anti-alias/reconstruction evidence, validate parity, and
   rerun cutover.
5. Continue until direct publication cutover closes or a legitimate DC boundary
   is proven.
6. Run full closure gates and complete review, verification, disposition,
   roadmap/index updates, and worker handoff.

Constraints:

- Do not use compatibility WB13 rows, compatibility runtime publication
  surfaces, writeback payloads, stale logical state, skeleton publication
  capture, or wrappers around those structures as direct authority.
- Do not close after a diagnostic-only step.
- Do not claim cutover while any required output family still reads the
  compatibility path.
- Do not defer anti-alias, independent reconstruction, parity, or line-count
  gates while claiming complete.

Subagent authorization: this package explicitly authorizes spawning/delegating
to read-only blocker-inventory, parity runner, anti-alias/reconstruction
reviewer, no-compatibility source-scan, line-count-governance reviewer, and
verification subagents for R6 cutover execution, output parity, independent
reconstruction, package artifact review, line-count governance, and
gate-legitimacy verification. Expected outputs are compact findings, command
logs, metrics, and artifact updates. Write access is limited to package
artifacts unless this package is explicitly amended.
