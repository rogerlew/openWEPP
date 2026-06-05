# Implementation/Test Evidence

Status: complete

Evidence mode: ran

Static:

- Runner: `artifacts/hphys0303_adr0016_ratification.py`.
- Ledger: `artifacts/comparator-ratification-ledger.json`.
- Fixed comparator commit:
  `47ac4c32faeea81bb99081f955a14c38b815ef4d`.
- Fixed comparator tag:
  `wepp_260430_negmeltfix_comparator_47ac4c32faee`.

Ran:

- `/workdir/wepppy/.venv/bin/python .../hphys0303_adr0016_ratification.py`:
  pass, `ratification_status=accepted-ready`.
- Fixed `release/wepp_260430` SHA256:
  `cd56985bf1575d8d82d4b4f943ca29a4fc2865448d7c308c0220c565a8955e87`.
- Fixed `release/wepp_260430_hill` SHA256:
  `b9337f1db714ef3d4ae45633b88249ccdc5416fbb7f5614a45fb688126eb45cd`.
- H1..H39 fixed baseline parquets regenerated under
  `/tmp/hphys0303_adr0016_1780691036/reports/hillslope/fixed_baseline_partitions`.
- H1..H39 fixed baseline parquet year/key validation passed for years
  `2013..2016`, 39/39 partitions, and zero duplicate `(ofe, year, julian)`
  keys.
- H1/H7/H39 fixed release vs observe-off/observe-on identity passed.
- SC unit/provenance lint passed after registry-backed Variables/Units and
  Symbol Alias Map amendments in `SC-SNOWFREEZE-001` and `SC-WATBAL-001`.
- Host smoke helper failed because the selected HPHYS fixture root lacks the
  helper's expected `p*.run`, `chntyp.txt`, and `gwcoeff.txt` files; the
  package relies on the stronger generated H1..H39 replay and observe identity
  evidence instead.
