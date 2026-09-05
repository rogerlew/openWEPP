# Covered nonfinal physical-only verification B — cycle 1

Evidence mode: `Static + Ran`

Verdict: `FAIL`

The verifier independently recomputed scoped diff SHA-256
`5e1c303754aa7c4ef0ccab43560bebec867d779fad5113f6d10258a581ff440a`
and confirmed the static gates and deliberately expected-red posture. It closed
`CRB-CNFP-002`, `003`, `005`, and `007`, but retained `001`, `004`, and `006`
because the behavioral population lacked exact failure counters, native zero-
work counters, exact error class/precedence, independent multisecant trial
index, and no-fallback accounting. New finding `VB-CNFP-001` identified an
incorrect VegetationTransaction error remapping for complete-owner, atomic-
install, rollback, and restart/replay failures.

Disposition: every remaining and new finding accepted. See `disposition.md`
and the current repeat-verification manifest in `contract_ref.md`.
