# Final disposition

Status: `EXECUTED-COMPLETE`
Evidence mode: Static and Ran

Defect `CHANINP-RAW-NCHNUM-CARDINALITY` is closed. Both modes now return exact
non-collapsible `CHN-E-002` when conditional record 4 does not match
nonnegative raw `nchnum`; diagnostic priority is cardinality before ID-token
parsing. A raw-count-closed 99-ID compatibility fixture preserves the source
count/list, derives normalized count 2 and the first-two-ID parser projection,
and proves the real network frame consumes normalized count.

Terminal parser coverage is 92.713% lines / 96.723% regions, the named-function
floor is 80%, and maximum CRAP is 16.352. The review-corrected monolith
reconstruction independently passes 36/36 at science tier and retains the
eligible CC 42 / CRAP 49.283 decomposition target. Terminal focused suites pass
36/36 and 19/19; the full workspace passes 1,747/1,747 with 3 configured skips.
Formatting, clippy, deny, Markdown, diff, A-H, line-count, security, dual
review/disposition, and dual verification gates all pass.

No current gate is deferred. Closure proves normalized-count consumption only;
it does not claim a downstream consumer or readiness for `ichnum_norm`. The
unrelated root `README.md` modification is excluded from the terminal commit.
