# Independent review A

Status: `COMPLETE — HOLD; CORRECTNESS/SOURCE APPROVED`

Evidence mode: `Static + reported Ran`

The independent correctness reviewer found no production correctness defect in
the retained exact-source increments. V29 reuse is limited to bit-exact beta
one or the two branches that complete before beta is read; V30 probe reuse is
absent; caps/request validation preserves error precedence; the carrier soil
proof remains seal- and pointer-bound; and the current Jacobian-base production
call site binds the evaluated and stored trial correctly.

Initial finding: HIGH — C-018 lacked complete evaluation/full-solve coverage
for the `Inactive` branch. Resolved at `solver_tests.rs`: the final fixture
rebuilds bound shortwave authority, proves both leaves select `Inactive`,
compares ordinary and forced-exhaustive complete evaluations with exact `2`
versus `4` calls, and compares full-solve outcomes while proving call
elimination. Focused and full LSE tests passed.

Initial documentation findings: MEDIUM — intermediate HOLD wording conflicted
with the terminal owner-choice boundary, and an old `12,436,785 us` section was
labeled terminal. Resolved: the intermediate narrative now states that
in-envelope work continued until exhausted, the old source/binary section is
historical, and `gate-results.md` owns exact terminal identities and outcomes.

Ran independently: `git diff --check` PASS. Heavy gate results were reviewed
from the recorded exact-source evidence.

Verdict: `HOLD — correctness/source approved; qualification remains
legitimately held on performance/RSS and orchestrator Clippy failures`.
