# R2A Direct Executor Selection Proof

Status: queued.
Evidence mode: not run.

Execution must prove:

- direct skeleton selection occurs once at setup;
- default compatibility execution does not construct direct skeleton state;
- opt-in/test direct skeleton selection is explicit and fail-closed;
- direct skeleton execution does not enter compatibility scheduler/kernel
  request paths;
- no per-phase compatibility branch is added to hot loops.

Record the command(s), counters, static search results, and tests used to prove
each item.
