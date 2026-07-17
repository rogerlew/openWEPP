# Renewed Terminal Finding Disposition

Evidence class: `Static`

Renewed terminal verifiers A and B independently returned `HOLD`. All findings
are accepted and remediated; no finding is waived or deferred.

## V2A-001 / V2-B-006 — Premature closure and incomplete discovery

**Disposition:** Accepted and remediated.

The package, README, catalog, and final disposition were reopened before
technical remediation. The artifact index now names both round-2 reviews,
disposition, gates, and renewed verifications. The package will not return to
completion until both renewed verifiers pass the final exact tree and then
reverify the closure-only bookkeeping bytes.

## V2-B-001 — Receipt/attestation identity cycle

**Disposition:** Accepted and remediated.

The authority now derives an unsigned immutable receipt first, then an
attestation envelope whose subject is the exact receipt ID/digest and artifacts,
then a separate envelope ID. Ledger/certificate consumption requires both and
rejects subject mismatch or recursion.

## V2-B-002 — Enforceable evidence writer boundary

**Disposition:** Accepted and remediated.

The unprotectable custom ref is replaced by GitHub branch/tag namespaces under
`openwepp-evidence/**`. Active branch/tag rulesets restrict creation, update,
and deletion and permit only a dedicated evidence-publisher GitHub App to bypass.
Atomic push,
force-with-lease, provider configuration evidence, failure behavior, and
primary Git/GitHub references are normative.

## V2-B-003 — Assurance supersession

**Disposition:** Accepted and remediated.

Supersession is one atomic event with a valid same-report/same-target replacement
included in the fold. Dangling, cross-report, cross-target, withdrawn, or
recursively invalid replacement leaves the original entry open/pending.

## V2-B-004 — Missing acceptance matrix

**Disposition:** Accepted and remediated.

Handoff scenarios 33–38 preserve the complete A0–A6 outcome matrix, stale and
superseded obligations, omitted-impact and supersession/withdrawal assurance
cases, node-ID field mutations, Git invalid/change-kind cases, and release reuse
rejections.

## V2-B-005 — Retry outcome inconsistency

**Disposition:** Accepted and remediated.

Section 5.1 defines accepted execution consistently as clean `PASS` or
policy-permitted infrastructure-only `PASS_WITH_RETRY`. Semantic failures can
never retry into acceptance; all attempts and mandatory debt remain visible,
and ledger/reuse acceptance uses the same policy generation.

## Result

Both renewed verifiers returned technical `PASS`, authorizing the closure-only
status/index transition. Their verification artifacts record the exact-byte
recheck of that transition. No authority or implementation-handoff byte changes
in the closure step.
