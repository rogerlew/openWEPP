# R7G Kickoff Agent Prompt

Execute `docs/work-packages/20260623-r7g-performance-closure-fixture-hardening-001/package.md`
end to end.

Required posture:

- Read root `AGENTS.md`, `docs/work-packages/AGENTS.md`,
  `docs/specifications/science-contracts/AGENTS.md`, and the package before
  edits.
- Treat same-binary H2637 performance, output identity, no-compatibility
  proof, fixture matrix, and independent reconstruction as current-scope
  gates.
- Do not stop after a benchmark, a changed blocker marker, or a partial
  fixture matrix. Iterate until `COMPLETE-R7G-PERFORMANCE-CLOSURE-FIXTURE-HARDENING`
  or a legitimate `HOLD-R7G-<SPECIFIC-ARCHITECTURE-BLOCKER>`.
- If direct default misses `<=10x`, profile the direct path and remediate every
  in-envelope measured blocker before holding.
- Do not claim release/default activation readiness. R7H owns release cutover.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only reviewer and verifier subagents for R7G
performance evidence review, fixture-matrix review, no-compatibility profile
review, operand-reconstruction review, and line-count governance review.
Expected outputs are compact Markdown findings summarized into
`artifacts/review-disposition.md` and `artifacts/verification.md`; subagents
may not edit files.
