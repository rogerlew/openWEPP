# Verification Agent A (D10B) — closure of accepted findings

Evidence class: Static + Ran (read-only shell; no cargo).

Verdict: **PASS-WITH-NOTES**.

Disposition-table completeness: PASS — every finding from both reviews has
a row; the A-MAJOR-2 split into 2a/2b/2c is correct; no orphans.

Per-finding: ALL accepted contract/artifact/production findings verified
CLOSED at their named locations (contract frontmatter 26; INV-011 rev-26
acceptance wording + no contradicting text; test-vector row (b)
TV-transient; Algorithm items 3/4/6 + INV-007 rev-26 forms; ground-3
requalification in contract + artifact; BEI gate row; TV scope caveat;
B-M1 integer-index loops + regression at sample_dt=0.003; B-M2
exact-total non-negative redistribution + preserved fail-closed guard +
runon-only regression; B-M3 span plumbing end-to-end + partial-bin
regression; m5/m6/m7/m9/m11/m12 doc fixes; rev-26 changelog row).
A-MINOR-7 rejection substantiated; B-m8 deferral verified as-recorded.

Notes (all closed post-verification by the executor):
1. B-m10 PARTIAL — one stale "piecewise-linear series" phrase inside the
   handoff test + two module-doc phrases; FIXED post-verification
   (comment-only; focused suite re-run 64/64, Ran by executor).
2. Gate-record: the post-fix focused 64/64 and the full workspace re-run
   were not yet recorded in gate-results.md at verification time; the
   executor's post-fix runs are appended to gate-results with this note.
3. package.md S5 checkbox to be checked at final closure (accurate
   mid-verification).
