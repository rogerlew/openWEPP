# Coverage Closure

Evidence label: Static.

Status: `QUEUED`

ADR-0021 tier: `science`, because the target owns WS20/WS21 channel sediment
routing math, typed guards, and contract-bound output/diagnostic behavior.

Closure rule:

- If characterization tests are added or materially changed, record line and
  region coverage status for the target, per-function 75% region-floor status or
  disposition, and obligation-to-test binding before completion.
- Because LCOV does not provide region coverage, record available line coverage
  plus a branch-sensitive CRAP surrogate and explicit per-function disposition.

Current scaffold status:

- Baseline LCOV: `LF:934`, `LH:0`, `0.0%`.
- Characterization not yet added.
- Coverage closure pending implementation.
