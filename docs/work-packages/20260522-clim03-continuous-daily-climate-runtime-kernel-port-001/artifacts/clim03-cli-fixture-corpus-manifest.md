# CLIM03 CLI Fixture Corpus Manifest

Evidence mode: `Ran`
Status: `complete`

## Source Corpus
- Source root: `/wc1/runs/**/wepp/runs/*.cli`
- Representative source file used for curation:
  - `/wc1/runs/an/analgesic-flannel/wepp/runs/p229.cli`

## Curated Fixtures
1. `tests/fixtures/infile/climate/wc1_canoga_day1.cli`
- extraction: source lines `1..16` (header + first wet-day daily row)
- purpose: validate wet-event runtime policy scaling and time-to-peak normalization path
- note: `datver` normalized `5.32300 -> 5.30` to satisfy current parser allowlist.

2. `tests/fixtures/infile/climate/wc1_canoga_stmdur_cap.cli`
- extraction: source lines `1..15` + source line `8651` (`stmdur=24.00` row)
- purpose: validate `stmdur<=23.999 h` cap and high-`ip` scaled disaggregation path
- note: `datver` normalized `5.32300 -> 5.30` to satisfy current parser allowlist.

## Coverage Intent
- real-world CLIGEN daily row shape from `/wc1/runs`
- positive precipitation wet-event branch
- zero/low `timep` normalization behavior
- duration-cap enforcement before unit conversion
