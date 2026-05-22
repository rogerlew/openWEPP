# CLIM04 CLI Fixture Corpus Manifest

Evidence mode: `Static`
Status: `curated`

## Discovery
Breakpoint source corpus discovered from:
- `/wc1/runs/**/wepp/runs/*.cli`
- discovery command: `rg -l --glob '**/wepp/runs/*.cli' '^\s*1\s+1\s+[01]\s*$' /wc1/runs`

## Curated Fixtures Added
1. `tests/fixtures/infile/climate/wc1_major_restlessness_breakpoint_stmstr_nonzero.cli`
- source: `/wc1/runs/ma/major-restlessness/wepp/runs/p4.cli`
- extracted day: `4 1 2007` (`nbrkpt=5`)
- purpose: verify `stmstr` capture from non-zero first breakpoint hour and elapsed-time normalization.

2. `tests/fixtures/infile/climate/wc1_major_restlessness_breakpoint_nbrkpt_42.cli`
- source: `/wc1/runs/ma/major-restlessness/wepp/runs/p4.cli`
- extracted day: `12 9 2013` (`nbrkpt=42`)
- purpose: verify larger real-world breakpoint event-shape projection and symbol fanout.

3. `tests/fixtures/infile/climate/breakpoint_duplicate_timem.cli`
- synthetic compatibility-control fixture (constructed from canonical climate header layout)
- purpose: strict duplicate-time rejection plus explicit legacy compatibility-mode acceptance path coverage.

## Existing Fixture Reuse
- `tests/fixtures/infile/climate/breakpoint_overflow_51.cli`
- retained to validate post-policy behavior (`51` is valid under strict `<=1500`).

## Provenance Notes
- Curated WC1 fixtures preserve canonical symbol/units (`timem` hours, `pptcum` mm).
- `datver` for curated test fixtures is normalized to `5.30` to satisfy strict parser allowlist policy.
