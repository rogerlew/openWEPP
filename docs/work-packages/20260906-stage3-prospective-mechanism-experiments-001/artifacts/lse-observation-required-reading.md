# Common LSE observation intake

Static: observation-only implementation; no mechanism or numerical behavior change.
Instruction discovery ran before edits and returned AGENTS.md and crates/AGENTS.md
for every source path, plus docs/work-packages/AGENTS.md for this artifact.

Exact prospective source writes (all under crates/openwepp-land-surface-energy/src/):
solver_mechanism_audit.rs (new), solver.rs (only if import needed),
solver_covered_solve.rs, solver_covered_evaluation.rs, transaction.rs,
transaction_v3_bridge.rs, lib.rs. This artifact is the only delegated document write.

Read before implementation: root AGENTS.md (160 lines/12599 bytes),
crates/AGENTS.md (75/5436), docs/work-packages/AGENTS.md (388/26781),
docs/codex_exec_plans.md (244/20921), docs/standards/AGENTS.md (58/4052),
docs/standards/prompt-wording-guidance.md (180/10508),
docs/standards/testing-and-gate-strategy.md (459/22200),
docs/specifications/science-contracts/AGENTS.md (77/5942),
docs/standards/numerical-solver-architecture.md (152/6742),
docs/standards/kernel-work-package-preparation.md (255/15309).
Governance full-file byte inventory: 130490. Long combined command output was
truncated; focused subsequent reads recovered instructions used here.

On-demand actual source ranges read: solver.rs 1–115; solver_covered_solve.rs
125–145, 225–325, 477–612; solver_covered_evaluation.rs 1876–1965;
transaction.rs 287–310, 515–535, 665–690, 2726–2775;
transaction_v3_bridge.rs 135–157; lib.rs 1–95. Searches additionally located
functions, lifecycle constructors and existing forced-complete hooks. Full source
file byte inventory is 326522; only the named ranges were required reads.
Exact selected source byte counts, reconstructed from pre-edit HEAD ranges:
solver.rs 4015; solver_covered_solve.rs 9673;
solver_covered_evaluation.rs 3751; transaction.rs 4779;
transaction_v3_bridge.rs 833; lib.rs 3308; total 26359.

Accepted predecessor evidence read: September 4 final-disposition,
coverage-findings, memory-attribution (lines 1–145 each) and complete historical
GDB take-return summary; September 1 performance-budget 1–100,
tolerance-authority 1–110 and component-temperature-dependency-replay/contract_ref
1–130. Kickoff 1–240 and current package.md 1–160 read.
These are context only; no historical source custody recovery attempted.

Budget disposition: OK (selected reads below 400000 bytes). Parent combined map
owns exact aggregate accounting across workers. Common hooks are explicit
test-support observation sessions; compact mode allocates no event payloads.
Detailed events have a configured bound and checked loss accounting. Transaction
map identity must originate before potential solve and rejoin the same token in
final; solve/iteration/sweep identities originate at their actual lifecycle sites.
No replay selector, physics copy, altered guard, or numerical tolerance is allowed.
Validation: observer lifecycle/error/overflow/bound tests, owning-crate tests,
runner parity and package-selected full correctness before disposition. No heavy
build is dispatched by this worker.

Ran: plain rustfmt was unavailable (exit 127); scoped
`nix develop -c rustfmt --edition 2024 --config skip_children=true <six edited source files>`
passed, as did `git diff --check`. This included formatter-only normalization of
existing layout in both included solver source files. No numerical expression
changed. New source module 529 lines; solver_covered_solve 1423;
solver_covered_evaluation 2651 (WARN; inherited canonical body retained; replay
arm's shared node extraction is the prospective decomposition seam);
transaction 2952 (WARN, below 3000; observer adds eight lines only; future
transaction decomposition remains outside this observation scope).

API: `solver_mechanism_audit::begin_mechanism_audit(bool, usize)` returns a
thread-bound `Session`, available only under test/test-support. Consumed
`Session::finish` returns serializable `MechanismAudit` or typed nested/open-scope/
overflow errors; dropping the session clears the observer. Per-scope errors
include unwind/early exit; numerical solve rejection remains a successfully
completed `Result::Ok` solve and must not be relabeled an evaluator error.
Detailed traces expose ordered start/end events, independently minted iteration
IDs, sweep base bits/topology, stencil sides and actual classified probe starts.
Probe coordinate bits are optional solely for malformed out-of-range calls.
Loss is counted explicitly, and compact mode never allocates detailed payloads.
Map metadata is private, nonsemantic equality metadata in the potential phase;
its session identity is globally unique and final rejoins only that session.
The synthetic V3 bridge does not invent a map ID.
