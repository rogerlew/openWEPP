# Independent Review A

Evidence class: `Static` and narrow `Ran`

Role: adversarial governance, shell-confinement, and test-economy reviewer.

Initial verdict: `FAIL`, with two in-envelope findings.

- `TGGO-A-01` (`HIGH`): dirty terminal plan `5ba07fb1...` failed independent
  reconstruction with `GATE-TERMINAL-RECONSTRUCTION`. It is rejected and must
  not execute. The committed final plan must reconcile before execution.
- `TGGO-A-02` (`MEDIUM`): source-substring assertions did not behaviorally prove
  output resolution. The accepted patch adds seven isolated branch probes.

Re-review verdict for `TGGO-A-02`: `PASS`. Five accepted cases distinguish
standalone default, standalone relative/absolute overrides, executor default,
and executor relative override. Two rejected cases prove executor absolute and
traversal rejection. Every probe is confined to one unique scratch root and
stops before acquisition on the deliberately absent scratch Python prerequisite.

Ran: shell syntax, Rust formatting, and diff hygiene pass. No broad gate was run
by the reviewer. The changed integration test is 558 lines, below WARN.

## Integrated Environment Amendment

Verdict: `PASS`, no findings.

The policy-derived `BTreeSet` projection closes the ambient-variable mechanism,
excludes undeclared noise/secrets before value decoding, preserves declared
value sensitivity and typed non-UTF-8 failure, and retains separate compiler,
target, Cargo/Git configuration, and runner-image identities. Planner,
same-host executor, and verifier use the same function; `PolicyBundle::load`
does not recurse. Focused environment tests pass 2/2 and the 249/558-line files
remain below WARN. Committed determinism and terminal execution remain pending.
