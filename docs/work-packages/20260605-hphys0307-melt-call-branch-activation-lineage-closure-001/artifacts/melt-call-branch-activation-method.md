# HPHYS0307 Melt-Call Branch Activation Method

Ran:

- Loaded HPHYS0306 `branch-active-melt-term-ledger.json`.
- Preserved fixed comparator commit
  `47ac4c32faeea81bb99081f955a14c38b815ef4d`.
- Compared upstream `baseline_only_active_count` and
  `openwepp_only_active_count` for each H1/H7/H39 target window.
- Classified branch mask gaps as baseline-extra, openWEPP-extra, matched
  same-hour multi-source, or parser conflict lanes.
- Kept `production_edit_authorized=false` for every row because this package
  produced classification/source-lineage evidence only and did not prove an
  implementation target defect.

Static:

- Baseline branch predicate provenance is recorded in
  `melt-call-branch-activation-source-lineage.md`.
