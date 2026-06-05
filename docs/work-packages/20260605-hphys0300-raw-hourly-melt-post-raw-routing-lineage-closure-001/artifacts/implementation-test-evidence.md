# Implementation/Test Evidence

Status: complete

Evidence mode: ran

Ran:

- `.venv/bin/python docs/work-packages/20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001/artifacts/hphys0300_raw_post_raw_lineage.py --run-root /tmp/hphys0300_full_20260605T155527Z --artifact-dir docs/work-packages/20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001/artifacts`

Results:

- Full H1..H39 semantic suite ran under run root
  `/tmp/hphys0300_full_20260605T155527Z`.
- Targeted H1/H7/H39 openWEPP traces ran with return code `0` for H1, H7, and
  H39.
- Baseline observe identity was reused through the HPHYS0299 corrected runner
  and archived in `baseline-observe-identity.md`.
- HPHYS0300 route counts:
  - `raw-hourly-melt-term-state-hold`: `7`
  - `post-raw-routing-without-baseline-negative-melt-hold`: `1`
  - `corrected-depth-hourly-forcing-hold`: `1`

Decision:

- No production code edits are authorized. All raw/post-raw rows still have
  `term_state_evidence_status = aggregate-only`; additional paired
  `melt.for`/`snowd.for` term/state instrumentation is required before
  production snow kernel changes.
