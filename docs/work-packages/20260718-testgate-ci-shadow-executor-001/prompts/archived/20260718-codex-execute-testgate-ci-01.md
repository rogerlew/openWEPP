# Execute TESTGATE-CI-01

Scaffold and execute `TESTGATE-CI-01` from frozen base
`28daa0339cbc99ceb94b176a506ce0c213685d7c` using `package.md` as the binding
ExecPlan. Implement the fail-closed typed executor, affected-quality adapter,
nonblocking lifecycle workflows, benchmark, scorecard seed, rollback evidence,
review, and terminal validation without reducing the current conservative gate.

Read root and nearest `AGENTS.md` files, the canonical testing-and-gate
strategy, ADR-0039, the predecessor handoff, and the package required-reading
map before edits. Keep evidence labeled `Static:` or `Ran:` and do not represent
shadow output as cutover evidence.

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two independent reviewer roles and two independent
terminal-verifier roles for read-only exact-diff/tree inspection; expected
outputs are review and verification verdicts for package artifacts; write access
is read-only. It also explicitly authorizes one closure-runner role for the
single terminal full-workspace Nextest, cargo-deny, and global adjudicated-CRAP
sequence; expected output is exact command, status, timing, counts, and artifact
paths; write access is limited to generated build/coverage output.

Stop any cutover claim at `SHADOW-LAUNCHED / OBSERVING` until the canonical
14-day/20-increment and provider-side operands actually exist and pass.
