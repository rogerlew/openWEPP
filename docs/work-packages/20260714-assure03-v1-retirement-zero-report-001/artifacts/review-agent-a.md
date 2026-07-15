# ASSURE-03 Review A — Scientific Communication And Preservation

Review class: internal coding-agent review; not external scientific peer review

Evidence class: Static + Ran

Verdict: **HOLD**

## Findings

### A-001 — High — Preservation manifest misstates eight removals

The recovery record defines the TSV `action` column as the ASSURE-03 action,
but `v1-content-manifest.tsv` labeled `authoring.rs`, `graph.rs`, `model.rs`,
`path.rs`, `publication.rs`, `render.rs`, `review.rs`, and `snapshot.rs` as
`preserve-or-revise`. All eight are absent in the amended tree and recorded as
removed in `migration-inventory-disposition.md`. The recovery hashes remain
valid, but the action inventory is not truthful, and the exact-history test
checked only that `remove` rows were absent.

Required correction: mark the eight rows `remove` and enforce both action
directions while retaining the 51-row, size, and hash checks.

### A-002 — High — Living records contradict completed gate evidence

`gate-results.md` still said queued/not run; the search/link and implementation
artifacts said terminal work was pending; and package progress left completed
preservation, implementation, migration, and focused/full work unchecked.
Those statements contradicted the concrete initial PASS record in
`heavy-gate-runner.md` and violated the living ExecPlan requirement.

Required correction: give every package gate an explicit disposition with
commands/evidence, separately disposition skipped stability and spelling
preview, reconcile stale statuses, and check only actually completed progress.

## Ran And Positive Assessment

- All 51 frozen Git blobs independently matched recorded size and SHA-256.
- The eight action/existence mismatches above were the only such mismatches.
- Retained SNOTEL fixture, diagnostic, and activation identities matched.
- Narrative counts of 70,999 source rows, 13,590 paired rows, 159,986 selector
  rows, 53,711 precipitation rows, and the `5.551115123125783e-17 m`
  residual agreed with retained sources.
- Public assurance contained only `usersum/assurance/README.md`; no retired v1
  route or headline remained on the active surface.
- Direct `markdown-doc lint`/`validate` passed 36 files, `git diff --check`
  passed, and 53 local paths resolved. `wctl doc-lint` could not start because
  its environment lacks `typer`; the direct canonical fallback was used.

The neutral wording, snow/frost science, quantitative findings and limitations,
acceptance documents, cross-links, and prospective roadmap produced no other
finding.
