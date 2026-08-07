# Independent Verification A

Status: `GO`.

Evidence mode: `Static + Ran` at clean `fae6f8a18`.

The verifier independently reproduced all 154 eligible water years, site counts
`34/44/41/35`, zero truncation/active-state failures, Paradise WY2015 ratio
`0.06217301915749281`, both terminal classes, result/receipt hashes, the prompt
archive hash, DRAFT assurance posture, protected boundaries, and persistence
`HOLD`. Analyzer pytest passed `29/29`; `git diff --check` passed.

Two closure findings were accepted: the diff shortstat was stale and the first
quick profile was incomplete. The diff evidence is corrected against the exact
closure candidate, and the comparator runner subsequently completed the entire
quick profile `2,239/2,239 PASS` plus full `2,288/2,288 PASS`.

Re-verification at clean `2719512a1` confirmed the corrected diff, allowed
roadmap vocabulary, complete comparator evidence, clean worktree, and no
remaining closure blocker.
