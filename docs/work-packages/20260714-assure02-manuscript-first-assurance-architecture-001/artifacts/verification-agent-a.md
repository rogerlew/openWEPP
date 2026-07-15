# Verification A — Scientific Communication And Pilot Integrity

Verification class: internal coding-agent verification; not external domain
peer review

Evidence class: Static + Ran

## Round 1

Verdict: **HOLD**

All original Review A findings passed verification:

- `RA-001`: exact path comparison, 7/7 focused tests, arithmetic
  reconstruction, and all seven SHA-256 identities independently matched.
- `RA-002`: formulation domain, timestep convention, admissibility, and the
  negative-`ks` exclusion are explicit.
- `RA-003`: tolerances, provenance, observed residuals, and PASS decisions
  independently recomputed correctly.
- `RA-004`: public research objects have a required owner, surface, version
  binding, and fail-closed publication rule.
- `RA-005`: terminology, table titles, and Priest River limitations are
  scientifically appropriate.

### VA-001 — Review-state metadata contradiction

The prototype incorrectly said coding-agent review was pending, while the
review disposition incorrectly said the catalog already recorded completed
verification and the catalog correctly said verification was pending. This
truthfulness mismatch required remediation before Verification A could pass.

The parent replaced the prototype's transient statement with a stable pointer
to retained package review/verification records and corrected the disposition
to describe the catalog's current posture rather than predeclare completion.

### Round 1 Ran

- Independently reran the exact twelve-path comparison: exit `0`, empty output.
- Independently reran the focused nextest selector: run
  `1e137510-efa5-4362-9341-90ceb930e6ab`, 7/7 passed, 1930 skipped.
- Independently checked all seven SHA-256 identities: matched.
- Independently recomputed both H2637 residuals/allowances and the two-day
  maximum residual: matched and passed.
- Scoped `markdown-doc lint`, `markdown-doc validate`, and
  `git diff --check`: passed.
- Custom local Markdown links: 37 checked, zero missing.

`wctl doc-lint` could not start because its Python environment lacks `typer`;
the verifier used the direct canonical `markdown-doc` fallback. The verifier
made no workspace edits.

## Round 2

Verdict: **PASS**

`VA-001` is closed. The prototype now uses stable retained-record wording and
disclaims external peer review; the B-006 disposition describes rather than
predeclares catalog state; the catalog truthfully recorded completed reviews
and pending verification at the time checked; and Round 1 accurately records
its HOLD, remediation, commands, and limitations.

No original Review A closure regressed:

- the twelve-path comparison remained empty;
- all seven hashes matched;
- focused nextest run `cfad7c42-c364-464a-9ce2-39cb4361fc06` passed 7/7;
- both residuals and allowances independently recomputed and passed;
- formulation/domain and public-research-object rules remained intact; and
- terminology, descriptive tables, and Priest River limits remained intact.

Current-tree gates at Round 2: no changed or untracked Rust files;
`markdown-doc lint` passed 26 files with zero errors/warnings;
`markdown-doc validate` passed 26 files with zero errors; and
`git diff --check` passed. No new findings. The verifier made no workspace
edits.
