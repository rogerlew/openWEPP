# CAL-07 Gate Evidence

Evidence class: `Ran`

## Result execution

- `tools/execute.py`: `FAIL CLOSED` at
  `SH-EN-ALERCE / 2022-07-22` with negative contract-defined VPD.
- No `daily-kernel-output.csv`, `gate-results.csv`, score table, or verdict
  matrix was published.
- The fixed-sequence producer-phase and consumer-ordering tests did not run
  after the executor failure. They are not claimed.

## Independent diagnosis and diagnostics

- `tools/diagnose_forcing.py`: passed; reproduced 3,332 forcing rows and
  exactly three negative Alerce VPD rows.
- `tools/plot_hold.py`: passed and reproduced four SVGs plus their diagnostic
  manifest.
- Two consecutive diagnostic/table/SVG generations had identical SHA-256
  output.
- `tools/validate_hold.py`: passed; source and custody hashes, negative-day
  inventory, camera counts, no-partial-result boundary, SVG accessibility,
  sidecar sections, prospective GO receipts, and final status all matched.

Validator output:

```text
CAL-07 HOLD validation PASS: 3 negative VPD days; no partial canopy result
```

## Package-local Rust tool

- `cargo fmt --manifest-path .../tools/executor/Cargo.toml -- --check`: passed.
- `cargo check --manifest-path .../tools/executor/Cargo.toml`: passed.

This validates the package-local executor as code; it does not override its
scientifically correct runtime rejection.

## Documentation and figures

- `markdown-doc lint --path <CAL-07 package>`: 15 files, zero errors/warnings
  before terminal review artifacts.
- `markdown-doc lint --path docs/planning/canopy-phenology-assurance-roadmap.md`:
  one file, zero errors/warnings.
- `markdown-doc lint --path docs/work-packages/README.md`: one file, zero
  errors/warnings.
- All SVGs parsed with `xmllint` and rendered successfully with
  `rsvg-convert`; visual inspection confirmed visible failure markers,
  unconnected camera gaps, legible axes, and status text independent of color.

Terminal lint, exact-diff, review, and verification evidence is appended only
after those gates run on the final tree.
