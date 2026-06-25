# Worker Handoff

Status: complete

The frost-depth research corpus is now annotated and classified. Future work
should not start by enabling `Qwet` directly. Start with the observation
harness, snow-insulation attribution, and heat-flow column benchmarks.

Recommended follow-up package:

- Scaffold a contract-first `GAP-SNOWFREEZE-002` frost-depth physics
  adjudication package.
- Current-scope gates should include:
  - paired modeled/observed snow-depth diagnostics;
  - no-`Qwet` heat-flow baseline metrics;
  - at least one analytical thaw/freezing benchmark from Kurylyk et al. (2014);
  - source-level check of the current official WEPP branch for `frzftp`/`Qwet`;
  - an explicit decision whether `Qwet` remains off, enters research mode, or
    gets a bounded candidate implementation.

Do not:

- Treat compatibility frost output as the target.
- Promote `Qwet` without frozen hydraulic-conductivity/impedance authority.
- Commit ignored copyrighted PDFs.
