# Review Findings

## Initial Review A

Static: HOLD. All findings were accepted and remediated.

- HIGH: schema-level `contains` rules treated historical open events as the
  current aggregate. Removed cross-event fold logic from JSON Schema and added
  a behavioral open-to-refresh-required history test.
- HIGH: the ledger folded changed paths rather than immutable change subjects.
  Added content-derived `impact_subject_id`, subject-keyed folding, and a
  repeated same-path/new-object contract.
- HIGH: terminal reconciliation could weaken campaign/request/lifecycle and
  authority bindings. Added monotonic request and lifecycle reconciliation and
  exact reconciliation of campaign, authority, state, and remaining axes.
- MEDIUM: embedded `**` was treated as recursive. Restricted globstar behavior
  to Git component-boundary forms, fail-closed malformed patterns, and added
  embedded-star negatives.

## Initial Review B

Static: HOLD. All findings were accepted and remediated.

- HIGH: self-declared supersession or withdrawal could erase an open impact.
  Both transitions now fail closed as unauthenticated until verified lifecycle
  capability exists; full-fold negative tests cover both states.
- HIGH: dirty impacts used the base commit as their target. They now bind the
  exact dirty-tree digest; campaign transfer is requested only by an exact
  committed terminal plan.
- HIGH: process-domain classification was inert and fallback coverage was too
  narrow. The governed impact map now binds groundwater and snow/frost domain
  watches, every unmatched report/object pair becomes `OPEN_UNKNOWN`, and the
  integration contract exercises domain, procedure, builder, and glob kinds.
- MEDIUM: registry authority was not tied to the report lifecycle selection.
  Policy loading now cross-checks the exact report-lead principal and role,
  with a substitution-negative test.

## Remediated Review

- Reviewer B: PASS with no actionable findings.
- Reviewer A: HOLD on one additional medium gap: null-principal draft reports
  could substitute an arbitrary unresolved role. Accepted and remediated by
  requiring `assurance_steward` for the null-lead state and extending the
  lifecycle-authority negative test to the groundwater draft.

Final static verdicts: Reviewer A PASS; Reviewer B PASS. Neither reviewer ran
heavy gates. No actionable findings remain.

After the first terminal Clippy attempt exposed only a 116-line test function,
both reviewers inspected the assertion-helper extraction and returned PASS:
all inputs, assertions, ordering, and deterministic two-plan comparison remain
unchanged. No behavior test was added or repeated for the mechanical split.

Both reviewers also PASS the narrow MIT-0 license-policy repair. They confirmed
the identical dependency/checksum was present at the frozen base, the change
does not weaken bans/advisories/sources, and the successful 2,154-test full run
should be retained because only `deny.toml` and package documentation changed.
