# Gate Results

Status: complete

Evidence mode: ran

Static:

- Primary gate ledger:
  `artifacts/comparator-ratification-ledger.json`.

Ran:

- `python3 -m py_compile artifacts/hphys0303_adr0016_ratification.py`: pass.
- `cargo fmt`: pass.
- First runner attempt with system Python: `proposed-hold`, blocked on missing
  `pyarrow`/`fastparquet`.
- Runner rerun with `/workdir/wepppy/.venv/bin/python`: pass,
  `ratification_status=accepted-ready`.
- Fixed comparator source delta: pass; non-release source delta is only
  `src/winter.for`.
- Fixed H1..H39 baseline parquet regeneration: pass, 39/39.
- Fixed H1..H39 baseline parquet year/key validation: pass, years
  `2013..2016`, expected row counts `365/365/365/366`, zero duplicate keys.
- Fixed observe identity: pass for H1/H7/H39 release vs observe-off/on.
- SC unit/provenance lint: pass for `SC-SNOWFREEZE-001` and `SC-WATBAL-001`.
- Host smoke helper: not applicable to selected HPHYS fixture root; failed on
  missing helper-specific files before running simulations and is explicitly
  dispositioned in `artifacts/comparator-ratification-ledger.json`.
- `python3 -m py_compile artifacts/hphys0303_adr0016_ratification.py`: pass
  after review fixes.
- `cargo fmt --check`: pass after formatting.
- `cargo test --test hphys0303_adr0016_comparator_ratification_contract -- --nocapture`:
  pass, 3 tests.
- `cargo test --test hphys0302_comparator_surface_audit_contract -- --nocapture`:
  pass, 3 tests.
