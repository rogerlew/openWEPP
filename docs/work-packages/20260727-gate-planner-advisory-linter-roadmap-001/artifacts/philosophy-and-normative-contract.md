# Philosophy And Normative Contract

Status: `RATIFIED BY ADR-0043`

## Product Definition

`workplan-lint` is a deterministic, read-only advisory tool. It observes a
declared work boundary and reports cited findings that may help an agent plan
validation. It is not a gate, planner of record, executor, verifier, receipt
issuer, lifecycle controller, or evidence authority.

## Normative Rules

1. A repository rule, science contract, or authorized package creates a
   requirement. Linter output never does.
2. Only a command that actually ran can produce execution evidence. A suggested
   command and a finding are not evidence.
3. The agent owns validation judgment, command execution, truthful recording,
   and final disposition. Agent judgment cannot waive an applicable governing
   requirement.
4. The linter reads; it never writes repository, queue, receipt, ledger,
   campaign, custody, or remote state.
5. The linter suggests; it never launches validation, suggested,
   package-declared, workflow, remote, or user-controlled commands or creates
   execution authorization. Only the frozen literal read-only Git
   inspection allowlist may run.
6. Findings and linter availability have no lifecycle semantics. They cannot
   create or clear `HOLD`, `BLOCKED`, `READY`, `PASS`, or equivalent states.
7. `partial` and `unavailable` mean assistance was incomplete or absent. The
   agent immediately uses the manual route; no repair prerequisite is opened.
8. The linter has no CI, runner, identity, attestation, publication, recovery,
   calibration, or protected-data role.
9. Ambiguity is reported with its exact source. If the governing requirement
   itself is ambiguous, the agent resolves that authority question; the tool
   does not guess.
10. A feature survives only while representative use demonstrates net friction
    reduction within the accepted complexity and performance budgets.

## Independent Binding Boundaries

The following remain binding without the linter: package scope and intent;
nearest `AGENTS.md` instructions; canonical testing policy; science contracts;
A0/A1/A3 correctness authority; applicable direct-consumer, reconstruction,
conservation, and anti-evasion checks; explicit review and verification
requirements; exact-diff reconciliation; calibration-readiness rules; and
Harvard freeze/open-once custody.

## Nonblocking

“Nonblocking” means no linter result, failure, absence, defect, or stale version
can alter the originating work's authority or lifecycle. It does not mean an
agent may ignore an independently applicable rule or falsely claim closure.

## Prohibited Interpretations

The target is not a smaller TESTGATE, a local CI replacement, a receipt
generator, a compatibility facade over the transaction engine, or a mechanism
for silently reducing test scope. The words `gate`, `admit`, `authorize`,
`certify`, and `pass` may appear only when explaining that the linter does not
own those actions or when citing an independent underlying requirement.
