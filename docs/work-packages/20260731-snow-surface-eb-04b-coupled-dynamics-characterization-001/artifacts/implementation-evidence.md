# Implementation Evidence

Evidence: `Ran + Static`

`tools/run_analysis.py` is a deterministic, read-only consumer of retained
EB-04 and EB-04A evidence. It:

- validates the frozen factorial result, EB-04A report, executable,
  executable-source diff, and every trace hash;
- streams all 24 traces without modifying or rerunning a fixture and retains
  all 83,232 successful-day plus typed-terminal rows in a deterministic
  compressed chronology;
- independently reconstructs rejected temperatures and layer aggregates;
- checks paired B-cell reach and hashes as contrary evidence;
- audits production source ordering without changing it;
- emits machine JSON, four summary/window CSV tables, the deterministic
  complete-chronology CSV/GZIP, and five SVG plots with five Markdown sidecars;
- fails nonzero unless all 24 cases and all prospective signatures are
  dispositioned.

Final run returned `acceptance.passes=true`.
