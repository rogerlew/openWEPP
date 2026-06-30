# Disposition

Evidence class: Ran + Static

Result: `EXECUTED-COMPLETE-STREAMING-RSS-REDUCTION`.

The package closes successfully. Direct production now streams publication rows
and requested WAT/PASS parquet output, eliminating the residual whole-run
publication-row retention that held
`20260630-direct-publication-rss-reduction-001`.

RSS is materially reduced and the retained-row slope is flattened:

- H2637 full: `316212 KiB` prior held -> `112652 KiB`.
- H2637 required-only: `184644 KiB` prior held -> `52228 KiB`.
- W9 longer-day single-OFE fixture: `47856 KiB` for `16437` days.

Identity is preserved for measured data outputs. H2637 full HBP/loss/plot/WAT/PASS
and cli01 HBP/loss/plot/WAT are byte-identical to the retained-row baseline.

No physics, output-schema columns, snow/frost behavior, default policy, or
compatibility replay behavior changed. The typed setup/symbol-map carrier
deletion package can resume on top of this lower-RSS direct publication path.
