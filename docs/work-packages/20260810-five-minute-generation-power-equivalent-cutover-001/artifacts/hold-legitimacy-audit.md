# Hold Legitimacy Audit

Status: `historical PASS — HOLD was legitimate and is now lifted`

Evidence mode: `Static + Ran`

## Boundary

The declared prerequisite boundary is unavailable: package
`20260809-hourly-peak-runoff-authority-closure-001` is explicitly reopened,
not terminally closed. This package states that no shared contract or
production source edit may begin in that state.

## Direct evidence

Static: predecessor `package.md` and `artifacts/disposition.md` retain the
executing/reopened statuses and one unchecked lifecycle-closure item. Ran: the
current peak-authority integration test passes 4/4, proving implementation
agreement but not satisfying the separate terminal-package-status predicate.
See `prerequisite-authority-gate.md` for exact evidence.

## In-envelope route considered

The executor checked whether the current head already contained the missing
ADR authority reconciliation. It does: commits `669269ee4` and `a8a96498e`
align ADR-0036 and record review/gate receipts. However, closing the predecessor
would require editing its historical package and disposition artifacts. This
package expressly says not to modify historical artifacts in that prior
package and does not own them in its write set.

Package-local feasibility tooling cannot lift this boundary. After Review A
correctly found that the initial intake HOLD had stopped before exhausting
that lawful route, the executor implemented and ran the prospective shape and
constitutive study. It rejected every fixed exponent before opening Topanga
outcomes. See `feasibility-protocol.md`, `constitutive-response-study.md`, and
`exponent-authority.md`. There is now no package-local candidate arm to
execute, and package-local diagnostics still cannot authorize the blocked
contract-first water-output work.

Read-only Topanga intake additionally confirmed that all 1,408 frozen snapshot
files and all 1,088 mutation directories are locally present with no missing
required primary inputs. This rules out missing cohort inputs as the blocker.
No result-bearing mutation outcome was opened. Compact identities are retained
in `topanga-plan-identity.json`.

## Why this package cannot close now

Proceeding would violate an explicit pre-production gate and the declared
write-set boundary. The remaining blocker is external to this package's
authority, not implementation effort, diagnostic uncertainty, unavailable
source reading, or unexecuted package-local exponent analysis.

## Next defect-shaped action

Close the already-open predecessor lifecycle at the reconciled ADR/source
identity: update its final package/disposition/verification evidence to a
truthful terminal result after resolving its remaining review receipt. Then
resume this package from Milestone 1 and regenerate all baseline identities at
the new exact base. Do not reuse the stale local release runner binary.

## Resolution

Ran/Static: the predecessor's reconciled exact source passed the fresh complete
workspace suite (2,346/2,346) and the required focused authority, anti-evasion,
dependency, formatting, and documentation gates. Two fresh independent
terminal verifiers returned `PASS`, and the predecessor package and disposition
are now terminal. The condition that justified this HOLD no longer exists.

The HOLD is lifted. This package resumes at Milestone 1 baseline freezing. Its
historical review and verification receipts remain evidence for the executed
HOLD increment only; they do not verify the resumed implementation increment.
