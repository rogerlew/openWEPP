# Artifacts

Status: executed 2026-06-19. Verdict:
`NO-GO - section 7 falsification / re-profile before expanding`.

Required deliverables:

- `perfdeep03-ownership-refactor.md` - implemented lane-owned persistent compact
  dense state, true-boundary materialization, dirty-slot tracking, and static
  line-count disposition.
- `perfdeep03-endpoint.md` - load-bearing H2637 endpoint gate. Opt-in
  PERFDEEP03 measured `1147.96 s`, `229580 KB` against the PERFDEEP01
  `669.97 s` reference, so the gate failed.
- `perfdeep03-identity.md` - H2637 identity evidence: HBP/WAT byte-identical,
  PASS Arrow-equivalent, diagnostic roundtrip zero-mismatch.
- `perfdeep03-gate-results.md` - workspace Rust gates, focused tests, package
  gate table, and markdown gate result.
- `perfdeep03_disposition.md` - final no-go disposition and follow-on guidance.
- `review-claude-independent.md` - independent review confirming no-go and
  surfacing the larger kernel-body rewrite concern.

Runfiles:

- `runfiles/perfdeep03-h2637.run` - opt-in H2637 endpoint and diagnostic run.
- `runfiles/perfdeep03-h2637-default.run` - default-disabled H2637 endpoint and
  identity check.
