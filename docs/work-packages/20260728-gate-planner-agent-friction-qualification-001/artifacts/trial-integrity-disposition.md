# Trial Integrity Disposition

Evidence class: Ran + Static.

Invalid setup and scoring rounds were retained under ignored `target/` data but
excluded from qualification:

1. Initial `git worktree` cases exposed a `.git` indirection and caused the
   linter to discover the parent repository.
2. A malformed standalone clone lacked its own Git configuration. Its checkout
   escaped to the root repository, detached the root at a historical commit,
   and temporarily removed the current linter path for other participants.
   The root was restored to `main` at exact head
   `3f3ace0deced1a1aee62eb21975f8c2fefd75ee5`; affected arms were rerun.
3. Placeholder plans, synthetic millisecond timings, generic templates,
   directory-valued package operands, and absent-tool probes were rejected.
4. The first complete blinded packet made arm provenance inferable in 12
   cases. The frozen rubric invalidated those cases. The same P1/P2 agents
   repeated them from fresh standalone clones without reading scorer results,
   the evaluator key, mappings, or prior plans.
5. The final V2 packet passed two independent inferability checks and blind
   adjudication. No labels were inferred before reconciliation closed.

The frozen arm-order file placed all nine manual-first cases before all nine
linter-first cases. Each participant therefore completed three manual-first
pairs followed by three linter-first pairs. This is counterbalanced by count
but not interleaved by sequence, so learning/fatigue is confounded with arm
order. Timing and interaction results are descriptive; the deletion
disposition rests independently on reviewer-confirmed linter-arm omissions.

These incidents were protocol/setup failures, not linter-originated holds,
maintenance time, or scored tool behavior. They did not run validation,
modeling, CAL, synthetic, population, freeze/open, or Harvard commands. The
user-owned untracked audit remained untouched.
