# Independent Verification A

Evidence class: Ran + Static.

Disposition: `PASS`.

Verification A's initial terminal pass ran the 24-test focused suite, both
scoped documentation lints,
diff hygiene, exact line count, all three native modes, JSON misuse, schema and
exit semantics, exact write-set reconciliation, and audit exclusion.

The native repository correctly returned one `partial` JSON envelope and exit
3 in each mode because the frozen contract refuses its Git LFS declarations.
The implementation remained advisory and the manual route remained available.
All package-owned paths were declared; the unrelated audit remained untracked
and excluded. A final exact-state rerun after Verification B's nested-attribute
finding and correction ran the expanded 25-test suite.
