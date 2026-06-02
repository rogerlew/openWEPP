# HPHYS0244 Gate Results

Ran: diagnostics commands and documentation validation gates.

## Diagnostic Gates
- `Ran:` targeted H1/H7/H39 parquet comparison script with
  `/workdir/wepppy/.venv/bin/python`.
  - Result: generated `/tmp/hphys0244_20260602T045926Z/storage_dp_summary.tsv`.
  - Result: generated
    `/tmp/hphys0244_20260602T045926Z/first_30_storage_dp_timeseries.tsv`.
  - Result: generated `/tmp/hphys0244_20260602T045926Z/availability.json`.
- `Ran:` targeted source/prior-artifact `rg` lineage search.
  - Result: generated `/tmp/hphys0244_20260602T045926Z/static_lineage_search.txt`.
  - Result: generated `/tmp/hphys0244_20260602T045926Z/source_line_evidence.txt`.
- `Ran:` HBP string audit for `H1`, `H7`, and `H39`.
  - Result: only generic `state_N` labels found; no layer `theta`, `st`,
    `Pe`, or `pei` telemetry labels.

## Documentation Gates
- `Ran:` `wctl doc-lint --path docs/work-packages/README.md`
  - Result: pass; `1` file validated, `0` errors, `0` warnings.
- `Ran:`
  `wctl doc-lint --path docs/work-packages/20260602-hphys0244-h1-h7-h39-storage-wb18-lineage-diagnostics-001`
  - Result: pass; `0` configured files validated, `0` errors, `0` warnings.
- `Ran:` `git diff --check`
  - Result: pass.

## Non-Run Gates
- `cargo fmt --check`: not run; HPHYS0244 made no Rust changes.
- `cargo clippy --workspace --all-targets -- -D warnings`: not run; HPHYS0244
  made no Rust changes.
- `cargo test --workspace`: not run; HPHYS0244 made no Rust changes.
- `cargo deny check`: not run; HPHYS0244 made no dependency or Rust changes.
