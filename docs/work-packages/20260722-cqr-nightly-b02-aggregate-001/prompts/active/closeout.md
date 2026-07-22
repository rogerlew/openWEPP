# CQR Nightly B02 Aggregate Closeout Prompt

Execution mode: package-end-to-end.

Task: close the B02 aggregate by consuming both completed module packages,
closing any prospective TESTGATE tooling prerequisite, and executing exactly
one changed-head `INCREMENT` qualification through a comparator runner.

Subagent authorization: this prompt explicitly authorizes subagent
spawning/delegation to two independent read-only reviewers, two read-only
terminal verifiers, and one comparator runner. Expected outputs are
package-local review/verification artifacts and retained external comparator
evidence. Write access is read-only except for the comparator's ignored
artifact root.

Autonomy: continue through aggregate evidence, qualification, verification,
and disposition unless a declared boundary blocks.

Do not push, deploy, switch branches, manually dispatch TESTGATE, run HEAVY on
the parent, or rerun unchanged expensive gates.
