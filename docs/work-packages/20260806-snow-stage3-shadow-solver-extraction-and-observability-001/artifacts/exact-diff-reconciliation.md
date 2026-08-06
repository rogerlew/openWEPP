# Exact Diff Reconciliation

Status: closure candidate; final exact-head comparison queued.

Evidence class: `Static + Ran`.

The declared write set and review-driven amendment account for every path in
the 94-path baseline-to-candidate diff. No fixture, observation, calibration,
reference corpus, public schema, default, selector activation, production melt
owner, assurance lifecycle event, approval, release, publication, or unrelated
crate changed.

The prompt moved byte-preservingly from `prompts/active/` to
`prompts/archived/`; the mis-rooted `wctl doc-mv` wrapper rejected the relative
and absolute openWEPP paths, so an explicit patch move completed the operation
after both required helper attempts. No inbound link needed rewriting.

The final comparison will bind the exact clean commit after heavy validation
and both terminal verifiers. It must show no missing or extra manifest path,
`git diff --check` must pass, and the worktree must remain clean.
