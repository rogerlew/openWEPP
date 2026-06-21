# R6F Kickoff Agent Prompt

Scope: local repository R6 direct-publication cutover task; package-end-to-end.

In `/home/workdir/openWEPP`, execute
`docs/work-packages/20260621-r6f-direct-publication-cutover-blocker-closure-001/`
end to end.

Close defect `R6F-DIRECT-PUBLICATION-CUTOVER-BLOCKER`. Start from
`HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`.

Required terminal target:

- `COMPLETE-R6-DIRECT-PUBLICATION-CUTOVER`, with HBP byte identity, WAT/PASS
  Arrow row/schema/metadata parity, loss JSON identity, manifest
  provenance/checksum parity, anti-alias fixtures, independent reconstruction,
  no-compatibility proof, default-disabled isolation, successful direct output
  writes, full closure gates, and line-count governance.

Premature-stop prohibition:

- Do not stop at "HBP byte identity failed."
- Do not stop at "direct process parity mismatch."
- Do not stop because the next correction is broad or complex.
- Do not stop after fixing one blocker when the rerun exposes another in-scope
  blocker.
- A `HOLD` is valid only after the package's `HOLD Legitimacy Checklist` is
  fully satisfied and dual review accepts the boundary.

Execution order:

1. Read required package, root, work-package, DC, architecture, R6E, crates,
   tests, and science-contract instructions.
2. Reproduce the current cutover failure and record command, marker, stderr,
   output file state, and direct runtime counters.
3. Decode/reduce the HBP byte mismatch to exact fields, rows, byte spans,
   direct operands, producers, and authority.
4. Implement every in-envelope correction required to close HBP byte identity.
5. Re-run cutover and continue through WAT, PASS, loss, and manifest blockers.
6. Add anti-alias fixtures and independent reconstruction before accepting any
   output family.
7. Continue until direct publication cutover writes public outputs and all
   gates pass, or a legitimate out-of-envelope `HOLD` is proven.
8. Run full closure gates and complete review, verification, disposition,
   roadmap/index updates, and worker handoff.

Constraints:

- Do not use compatibility WB13 rows, compatibility runtime publication
  surfaces, writeback payloads, stale logical state, skeleton publication
  capture, or wrappers around those structures as direct authority.
- Do not claim cutover while any required output family still reads the
  compatibility path.
- Do not defer anti-alias, independent reconstruction, parity, manifest, or
  line-count gates while claiming complete.

Subagent authorization: this package explicitly authorizes spawning/delegating
to read-only blocker-reduction, HBP parity-diff, WAT/PASS parity, manifest
parity, no-compatibility, review, and verification subagents for R6 cutover
execution. Expected outputs are compact findings, field/operand maps, command
logs, metrics, and package artifact updates. Write access is bounded to this
package's declared write set; review and verification agents should default to
read-only findings unless specifically asked to patch artifacts.
